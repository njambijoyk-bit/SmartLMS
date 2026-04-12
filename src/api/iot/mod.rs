//! IoT & Smart Campus API Module
//! Handles device management, telemetry ingestion, and smart campus automation

use actix_web::{web, HttpResponse, Error};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::db::DbPool;
use crate::models::user::User;

// ==================== Data Transfer Objects ====================

#[derive(Debug, Deserialize)]
pub struct RegisterDeviceDto {
    pub device_id: String,
    pub device_type_id: Uuid,
    pub name: Option<String>,
    pub location_id: Option<Uuid>,
    pub room_id: Option<Uuid>,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
pub struct TelemetryBatchDto {
    pub device_id: Uuid,
    pub readings: Vec<TelemetryReadingDto>,
}

#[derive(Debug, Deserialize)]
pub struct TelemetryReadingDto {
    pub metric_name: String,
    pub metric_value: Option<f64>,
    pub metric_string: Option<String>,
    pub timestamp: Option<String>,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
pub struct CreateRuleDto {
    pub room_id: Uuid,
    pub name: String,
    pub trigger_type: String,
    pub trigger_config: serde_json::Value,
    pub action_type: String,
    pub action_config: serde_json::Value,
}

#[derive(Debug, Deserialize)]
pub struct UpdateDeviceStatusDto {
    pub status: String,
    pub battery_level: Option<i32>,
    pub firmware_version: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct DeviceInfoDto {
    pub id: Uuid,
    pub device_id: String,
    pub name: Option<String>,
    pub device_type: String,
    pub status: String,
    pub location: Option<String>,
    pub room: Option<String>,
    pub last_seen: Option<String>,
    pub battery_level: Option<i32>,
}

#[derive(Debug, Serialize)]
pub struct TelemetrySummaryDto {
    pub device_id: Uuid,
    pub metric_name: String,
    pub latest_value: Option<f64>,
    pub avg_value_24h: Option<f64>,
    pub min_value_24h: Option<f64>,
    pub max_value_24h: Option<f64>,
    pub sample_count: i64,
}

#[derive(Debug, Serialize)]
pub struct SafetyIncidentDto {
    pub id: Uuid,
    pub incident_type: String,
    pub severity: String,
    pub description: String,
    pub detected_at: String,
    pub resolved: bool,
}

// ==================== API Handlers ====================

/// Register a new IoT device
pub async fn register_device(
    pool: web::Data<DbPool>,
    user: User,
    dto: web::Json<RegisterDeviceDto>,
) -> Result<HttpResponse, Error> {
    // Only admins or facility managers can register devices
    if !user.is_admin() && !user.has_role("facility_manager") {
        return Ok(HttpResponse::Forbidden().finish());
    }

    let mut conn = pool.get().map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    // Generate auth token for device
    let auth_token = format!("iot_{}", Uuid::new_v4());

    let result: (Uuid,) = sqlx::query_as(
        r#"
        INSERT INTO iot_devices (device_id, device_type_id, name, location_id, room_id, metadata)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING id
        "#
    )
    .bind(&dto.device_id)
    .bind(dto.device_type_id)
    .bind(&dto.name)
    .bind(dto.location_id)
    .bind(dto.room_id)
    .bind(&dto.metadata)
    .fetch_one(&mut *conn)
    .await
    .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    // Store auth token
    sqlx::query(
        r#"
        INSERT INTO iot_device_auth (device_id, auth_token)
        VALUES ($1, $2)
        "#
    )
    .bind(result.0)
    .bind(&auth_token)
    .execute(&mut *conn)
    .await
    .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Created().json(serde_json::json!({
        "device_id": result.0,
        "auth_token": auth_token,
        "message": "Device registered successfully"
    })))
}

/// Ingest telemetry data from devices
pub async fn ingest_telemetry(
    pool: web::Data<DbPool>,
    device_auth: web::ReqData<DeviceInfoDto>, // Validated via middleware
    dto: web::Json<TelemetryBatchDto>,
) -> Result<HttpResponse, Error> {
    let mut conn = pool.get().map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    // Update device last_seen
    sqlx::query(
        r#"UPDATE iot_devices SET last_seen = NOW() WHERE id = $1"#
    )
    .bind(device_auth.id)
    .execute(&mut *conn)
    .await
    .ok();

    // Batch insert telemetry
    let mut tx = conn.begin().await.map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    for reading in &dto.readings {
        sqlx::query(
            r#"
            INSERT INTO iot_telemetry (device_id, metric_name, metric_value, metric_string, metadata)
            VALUES ($1, $2, $3, $4, $5)
            "#
        )
        .bind(device_auth.id)
        .bind(&reading.metric_name)
        .bind(reading.metric_value)
        .bind(&reading.metric_string)
        .bind(&reading.metadata)
        .execute(&mut *tx)
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
    }

    tx.commit().await.map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "received": dto.readings.len(),
        "status": "processed"
    })))
}

/// Get device list with filters
pub async fn list_devices(
    pool: web::Data<DbPool>,
    user: User,
    query: web::Query<DeviceListQuery>,
) -> Result<HttpResponse, Error> {
    if !user.is_admin() && !user.has_role("facility_manager") {
        return Ok(HttpResponse::Forbidden().finish());
    }

    let mut conn = pool.get().map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    let mut where_clause = String::from("WHERE 1=1");
    let mut params: Vec<Box<dyn sqlx::Encode<sqlx::Postgres> + Send + Sync>> = Vec::new();

    if let Some(status) = &query.status {
        where_clause.push_str(&format!(" AND d.status = ${}", params.len() + 1));
        params.push(Box::new(status.clone()));
    }

    if let Some(location_id) = query.location_id {
        where_clause.push_str(&format!(" AND d.location_id = ${}", params.len() + 1));
        params.push(Box::new(location_id));
    }

    if let Some(device_type_id) = query.device_type_id {
        where_clause.push_str(&format!(" AND d.device_type_id = ${}", params.len() + 1));
        params.push(Box::new(device_type_id));
    }

    let sql = format!(
        r#"
        SELECT d.id, d.device_id, d.name, dt.name as device_type, d.status, 
               l.name as location, r.name as room, d.last_seen, d.battery_level
        FROM iot_devices d
        JOIN iot_device_types dt ON d.device_type_id = dt.id
        LEFT JOIN locations l ON d.location_id = l.id
        LEFT JOIN rooms r ON d.room_id = r.id
        {}
        ORDER BY d.last_seen DESC
        LIMIT ${} OFFSET ${}
        "#,
        where_clause,
        params.len() + 1,
        params.len() + 2
    );

    // Simplified for brevity - in production use sqlx::query_with
    let devices: Vec<(Uuid, String, Option<String>, String, String, Option<String>, Option<String>, Option<String>, Option<i32>)> = 
        sqlx::query_as(&sql)
        .bind(query.limit.unwrap_or(50))
        .bind(query.offset.unwrap_or(0))
        .fetch_all(&mut *conn)
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    let result: Vec<DeviceInfoDto> = devices
        .into_iter()
        .map(|d| DeviceInfoDto {
            id: d.0,
            device_id: d.1,
            name: d.2,
            device_type: d.3,
            status: d.4,
            location: d.5,
            room: d.6,
            last_seen: d.7,
            battery_level: d.8,
        })
        .collect();

    Ok(HttpResponse::Ok().json(result))
}

#[derive(Debug, Deserialize)]
pub struct DeviceListQuery {
    pub status: Option<String>,
    pub location_id: Option<Uuid>,
    pub device_type_id: Option<Uuid>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

/// Get telemetry summary for a device
pub async fn get_device_telemetry(
    pool: web::Data<DbPool>,
    user: User,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, Error> {
    let device_id = path.into_inner();

    if !user.is_admin() && !user.has_role("facility_manager") {
        return Ok(HttpResponse::Forbidden().finish());
    }

    let mut conn = pool.get().map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    let summaries: Vec<(String, Option<f64>, Option<f64>, Option<f64>, Option<f64>, i64)> = sqlx::query_as(
        r#"
        SELECT metric_name, 
               (SELECT metric_value FROM iot_telemetry 
                WHERE device_id = $1 AND metric_name = t.metric_name 
                ORDER BY timestamp DESC LIMIT 1) as latest_value,
               AVG(metric_value) as avg_value_24h,
               MIN(metric_value) as min_value_24h,
               MAX(metric_value) as max_value_24h,
               COUNT(*) as sample_count
        FROM iot_telemetry t
        WHERE device_id = $1 AND timestamp > NOW() - INTERVAL '24 hours'
        GROUP BY metric_name
        "#
    )
    .bind(device_id)
    .fetch_all(&mut *conn)
    .await
    .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    let result: Vec<TelemetrySummaryDto> = summaries
        .into_iter()
        .map(|s| TelemetrySummaryDto {
            device_id,
            metric_name: s.0,
            latest_value: s.1,
            avg_value_24h: s.2,
            min_value_24h: s.3,
            max_value_24h: s.4,
            sample_count: s.5,
        })
        .collect();

    Ok(HttpResponse::Ok().json(result))
}

/// Create smart classroom automation rule
pub async fn create_automation_rule(
    pool: web::Data<DbPool>,
    user: User,
    dto: web::Json<CreateRuleDto>,
) -> Result<HttpResponse, Error> {
    if !user.is_admin() && !user.has_role("facility_manager") {
        return Ok(HttpResponse::Forbidden().finish());
    }

    let mut conn = pool.get().map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    let result: (Uuid,) = sqlx::query_as(
        r#"
        INSERT INTO smart_classroom_rules (room_id, name, trigger_type, trigger_config, action_type, action_config)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING id
        "#
    )
    .bind(dto.room_id)
    .bind(&dto.name)
    .bind(&dto.trigger_type)
    .bind(&dto.trigger_config)
    .bind(&dto.action_type)
    .bind(&dto.action_config)
    .fetch_one(&mut *conn)
    .await
    .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Created().json(serde_json::json!({
        "id": result.0,
        "message": "Automation rule created"
    })))
}

/// Get active safety incidents
pub async fn get_safety_incidents(
    pool: web::Data<DbPool>,
    user: User,
    query: web::Query<SafetyIncidentQuery>,
) -> Result<HttpResponse, Error> {
    let mut conn = pool.get().map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    let mut where_clause = String::from("WHERE resolved_at IS NULL");
    
    if let Some(lab_id) = query.lab_id {
        where_clause.push_str(&format!(" AND lab_id = '{}'", lab_id));
    }

    let sql = format!(
        r#"
        SELECT id, incident_type, severity, description, detected_at, 
               CASE WHEN resolved_at IS NULL THEN false ELSE true END as resolved
        FROM lab_safety_incidents
        {}
        ORDER BY 
            CASE severity 
                WHEN 'critical' THEN 1 
                WHEN 'high' THEN 2 
                WHEN 'medium' THEN 3 
                ELSE 4 
            END,
            detected_at DESC
        "#,
        where_clause
    );

    let incidents: Vec<(Uuid, String, String, String, String, bool)> = sqlx::query_as(&sql)
        .fetch_all(&mut *conn)
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    let result: Vec<SafetyIncidentDto> = incidents
        .into_iter()
        .map(|i| SafetyIncidentDto {
            id: i.0,
            incident_type: i.1,
            severity: i.2,
            description: i.3,
            detected_at: i.4,
            resolved: i.5,
        })
        .collect();

    Ok(HttpResponse::Ok().json(result))
}

#[derive(Debug, Deserialize)]
pub struct SafetyIncidentQuery {
    pub lab_id: Option<Uuid>,
}

/// Resolve a safety incident
pub async fn resolve_safety_incident(
    pool: web::Data<DbPool>,
    user: User,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, Error> {
    let incident_id = path.into_inner();

    let mut conn = pool.get().map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    sqlx::query(
        r#"
        UPDATE lab_safety_incidents 
        SET resolved_at = NOW(), resolved_by = $1 
        WHERE id = $2 AND resolved_at IS NULL
        "#
    )
    .bind(user.id)
    .bind(incident_id)
    .execute(&mut *conn)
    .await
    .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "message": "Incident resolved"
    })))
}

/// Configure digital signage content
pub async fn update_signage_content(
    pool: web::Data<DbPool>,
    user: User,
    path: web::Path<Uuid>,
    dto: web::Json<SignageContentDto>,
) -> Result<HttpResponse, Error> {
    if !user.is_admin() && !user.has_role("communications_manager") {
        return Ok(HttpResponse::Forbidden().finish());
    }

    let screen_id = path.into_inner();
    let mut conn = pool.get().map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    // Create content
    let content_result: (Uuid,) = sqlx::query_as(
        r#"
        INSERT INTO digital_signage_content (title, content_type, content_data, schedule_start, schedule_end, priority, target_locations, created_by)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        RETURNING id
        "#
    )
    .bind(&dto.title)
    .bind(&dto.content_type)
    .bind(&dto.content_data)
    .bind(dto.schedule_start)
    .bind(dto.schedule_end)
    .bind(dto.priority.unwrap_or(1))
    .bind(&dto.target_locations)
    .bind(user.id)
    .fetch_one(&mut *conn)
    .await
    .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    // Update screen to show new content
    sqlx::query(
        r#"UPDATE digital_signage_screens SET current_content_id = $1 WHERE id = $2"#
    )
    .bind(content_result.0)
    .bind(screen_id)
    .execute(&mut *conn)
    .await
    .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "content_id": content_result.0,
        "message": "Signage content updated"
    })))
}

#[derive(Debug, Deserialize)]
pub struct SignageContentDto {
    pub title: String,
    pub content_type: String,
    pub content_data: String,
    pub schedule_start: Option<String>,
    pub schedule_end: Option<String>,
    pub priority: Option<i32>,
    pub target_locations: serde_json::Value,
}

// ==================== Routes Configuration ====================

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1/iot")
            // Device Management
            .route("/devices", web::post().to(register_device))
            .route("/devices", web::get().to(list_devices))
            .route("/devices/{device_id}/telemetry", web::get().to(get_device_telemetry))
            
            // Telemetry Ingestion (authenticated via device token)
            .route("/telemetry/ingest", web::post().to(ingest_telemetry))
            
            // Automation Rules
            .route("/rules", web::post().to(create_automation_rule))
            
            // Safety
            .route("/safety/incidents", web::get().to(get_safety_incidents))
            .route("/safety/incidents/{incident_id}/resolve", web::post().to(resolve_safety_incident))
            
            // Digital Signage
            .route("/signage/{screen_id}/content", web::post().to(update_signage_content))
    );
}
