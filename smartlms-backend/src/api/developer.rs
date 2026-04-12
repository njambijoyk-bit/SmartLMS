// Developer Platform API - REST endpoints for API management, webhooks, and integrations
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::utils::app_state::AppState;
use crate::services::developer::{Integration, IntegrationType, WebhookEndpoint, SdkConfig, RateLimitInfo};
use crate::services::developer::service;

// ============================================================================
// REQUEST/RESPONSE TYPES
// ============================================================================

#[derive(Debug, Deserialize)]
pub struct CreateApiKeyRequest {
    pub name: String,
    pub permissions: Vec<String>,
    pub rate_limit: Option<i32>,
    pub expires_in_days: Option<i32>,
}

#[derive(Debug, Serialize)]
pub struct ApiKeyResponse {
    pub id: Uuid,
    pub name: String,
    pub key: String, // Only shown once on creation
    pub key_prefix: String,
    pub permissions: Vec<String>,
    pub rate_limit: i32,
    pub expires_at: Option<chrono::DateTime<chrono::Utc>>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize)]
pub struct ApiKeySummary {
    pub id: Uuid,
    pub name: String,
    pub key_prefix: String,
    pub permissions: Vec<String>,
    pub rate_limit: i32,
    pub is_active: bool,
    pub last_used_at: Option<chrono::DateTime<chrono::Utc>>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateWebhookRequest {
    pub name: String,
    pub url: String,
    pub events: Vec<String>,
    pub headers: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Serialize)]
pub struct WebhookResponse {
    pub id: Uuid,
    pub name: String,
    pub url: String,
    pub events: Vec<String>,
    pub secret: String, // Only shown once on creation
    pub is_active: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateIntegrationRequest {
    pub name: String,
    pub integration_type: String,
    pub config: std::collections::HashMap<String, String>,
    pub credentials: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Deserialize)]
pub struct GetSdkConfigRequest {
    pub institution_id: Uuid,
}

#[derive(Debug, Serialize)]
pub struct UsageStats {
    pub total_requests: i64,
    pub successful_requests: i64,
    pub failed_requests: i64,
    pub avg_response_time_ms: f64,
    pub period_start: chrono::DateTime<chrono::Utc>,
    pub period_end: chrono::DateTime<chrono::Utc>,
}

// ============================================================================
// API KEY ENDPOINTS
// ============================================================================

pub async fn create_api_key(
    State(state): State<AppState>,
    user_id: Option<axum::extract::Extension<Uuid>>,
    Json(req): Json<CreateApiKeyRequest>,
) -> Result<Json<ApiKeyResponse>, (StatusCode, String)> {
    let user_id = user_id.ok_or((StatusCode::UNAUTHORIZED, "User not authenticated".to_string()))?;
    
    // Get user's institution (simplified - in production, fetch from DB)
    let institution_id = get_user_institution(&state.pool, user_id).await?;
    
    // Generate API key
    let api_key = generate_api_key();
    let key_prefix = api_key[..12].to_string();
    let key_hash = hash_api_key(&api_key);
    
    // Calculate expiry if specified
    let expires_at = req.expires_in_days.map(|days| {
        chrono::Utc::now() + chrono::Duration::days(days as i64)
    });
    
    // Insert into database
    let id = Uuid::new_v4();
    sqlx::query!(
        "INSERT INTO api_keys (id, institution_id, user_id, name, key_hash, key_prefix, 
         permissions, rate_limit, expires_at, created_at, updated_at)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)",
        id,
        institution_id,
        user_id,
        req.name,
        key_hash,
        key_prefix,
        serde_json::to_value(&req.permissions).unwrap(),
        req.rate_limit.unwrap_or(1000),
        expires_at,
        chrono::Utc::now(),
        chrono::Utc::now()
    )
    .execute(&state.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    Ok(Json(ApiKeyResponse {
        id,
        name: req.name,
        key: api_key,
        key_prefix,
        permissions: req.permissions,
        rate_limit: req.rate_limit.unwrap_or(1000),
        expires_at,
        created_at: chrono::Utc::now(),
    }))
}

pub async fn list_api_keys(
    State(state): State<AppState>,
    user_id: Option<axum::extract::Extension<Uuid>>,
) -> Result<Json<Vec<ApiKeySummary>>, (StatusCode, String)> {
    let user_id = user_id.ok_or((StatusCode::UNAUTHORIZED, "User not authenticated".to_string()))?;
    
    let rows = sqlx::query!(
        "SELECT id, name, key_prefix, permissions, rate_limit, is_active, 
         last_used_at, created_at
         FROM api_keys WHERE user_id = $1 ORDER BY created_at DESC",
        user_id
    )
    .fetch_all(&state.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    let keys = rows.into_iter().map(|r| ApiKeySummary {
        id: r.id,
        name: r.name,
        key_prefix: r.key_prefix,
        permissions: serde_json::from_value(r.permissions).unwrap_or_default(),
        rate_limit: r.rate_limit,
        is_active: true, // Would need to check expires_at in production
        last_used_at: r.last_used_at,
        created_at: r.created_at,
    }).collect();
    
    Ok(Json(keys))
}

pub async fn revoke_api_key(
    State(state): State<AppState>,
    Path(key_id): Path<Uuid>,
    user_id: Option<axum::extract::Extension<Uuid>>,
) -> Result<StatusCode, (StatusCode, String)> {
    let _user_id = user_id.ok_or((StatusCode::UNAUTHORIZED, "User not authenticated".to_string()))?;
    
    sqlx::query!(
        "UPDATE api_keys SET is_active = false, updated_at = $1 WHERE id = $2",
        chrono::Utc::now(),
        key_id
    )
    .execute(&state.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    Ok(StatusCode::NO_CONTENT)
}

// ============================================================================
// WEBHOOK ENDPOINTS
// ============================================================================

pub async fn create_webhook(
    State(state): State<AppState>,
    user_id: Option<axum::extract::Extension<Uuid>>,
    Json(req): Json<CreateWebhookRequest>,
) -> Result<Json<WebhookResponse>, (StatusCode, String)> {
    let user_id = user_id.ok_or((StatusCode::UNAUTHORIZED, "User not authenticated".to_string()))?;
    
    let institution_id = get_user_institution(&state.pool, user_id).await?;
    
    let (webhook, secret) = service::create_webhook(
        &state.pool,
        institution_id,
        &req.name,
        &req.url,
        req.events,
    )
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    Ok(Json(WebhookResponse {
        id: webhook.id,
        name: webhook.name,
        url: webhook.url,
        events: webhook.events,
        secret,
        is_active: webhook.is_active,
        created_at: webhook.created_at,
    }))
}

pub async fn list_webhooks(
    State(state): State<AppState>,
    user_id: Option<axum::extract::Extension<Uuid>>,
) -> Result<Json<Vec<WebhookEndpoint>>, (StatusCode, String)> {
    let user_id = user_id.ok_or((StatusCode::UNAUTHORIZED, "User not authenticated".to_string()))?;
    
    let institution_id = get_user_institution(&state.pool, user_id).await?;
    
    let rows = sqlx::query!(
        "SELECT id, institution_id, name, url, events, secret, is_active,
         failure_count, last_failure_at, last_success_at, created_at, updated_at
         FROM webhook_endpoints WHERE institution_id = $1",
        institution_id
    )
    .fetch_all(&state.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    let webhooks = rows.into_iter().map(|r| WebhookEndpoint {
        id: r.id,
        institution_id: r.institution_id,
        name: r.name,
        url: r.url,
        events: serde_json::from_value(r.events).unwrap_or_default(),
        secret: r.secret,
        is_active: r.is_active,
        created_at: r.created_at,
    }).collect();
    
    Ok(Json(webhooks))
}

pub async fn toggle_webhook(
    State(state): State<AppState>,
    Path(webhook_id): Path<Uuid>,
) -> Result<StatusCode, (StatusCode, String)> {
    sqlx::query!(
        "UPDATE webhook_endpoints SET is_active = NOT is_active, updated_at = $1 WHERE id = $2",
        chrono::Utc::now(),
        webhook_id
    )
    .execute(&state.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    Ok(StatusCode::NO_CONTENT)
}

pub async fn delete_webhook(
    State(state): State<AppState>,
    Path(webhook_id): Path<Uuid>,
) -> Result<StatusCode, (StatusCode, String)> {
    sqlx::query!("DELETE FROM webhook_endpoints WHERE id = $1", webhook_id)
        .execute(&state.pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    Ok(StatusCode::NO_CONTENT)
}

// ============================================================================
// INTEGRATION ENDPOINTS
// ============================================================================

pub async fn create_integration(
    State(state): State<AppState>,
    user_id: Option<axum::extract::Extension<Uuid>>,
    Json(req): Json<CreateIntegrationRequest>,
) -> Result<Json<Integration>, (StatusCode, String)> {
    let user_id = user_id.ok_or((StatusCode::UNAUTHORIZED, "User not authenticated".to_string()))?;
    
    let institution_id = get_user_institution(&state.pool, user_id).await?;
    
    let integration_type = parse_integration_type(&req.integration_type)
        .ok_or((StatusCode::BAD_REQUEST, "Invalid integration type".to_string()))?;
    
    let integration = service::register_integration(
        &state.pool,
        institution_id,
        &req.name,
        integration_type,
        req.config,
    )
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    Ok(Json(integration))
}

pub async fn list_integrations(
    State(state): State<AppState>,
    user_id: Option<axum::extract::Extension<Uuid>>,
) -> Result<Json<Vec<Integration>>, (StatusCode, String)> {
    let user_id = user_id.ok_or((StatusCode::UNAUTHORIZED, "User not authenticated".to_string()))?;
    
    let institution_id = get_user_institution(&state.pool, user_id).await?;
    
    let integrations = service::list_integrations(&state.pool, institution_id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    Ok(Json(integrations))
}

pub async fn delete_integration(
    State(state): State<AppState>,
    Path(integration_id): Path<Uuid>,
) -> Result<StatusCode, (StatusCode, String)> {
    sqlx::query!("DELETE FROM integrations WHERE id = $1", integration_id)
        .execute(&state.pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    Ok(StatusCode::NO_CONTENT)
}

// ============================================================================
// SDK ENDPOINTS
// ============================================================================

pub async fn get_sdk_config(
    State(state): State<AppState>,
    user_id: Option<axum::extract::Extension<Uuid>>,
) -> Result<Json<SdkConfig>, (StatusCode, String)> {
    let user_id = user_id.ok_or((StatusCode::UNAUTHORIZED, "User not authenticated".to_string()))?;
    
    let institution_id = get_user_institution(&state.pool, user_id).await?;
    
    let config = service::get_sdk_config(&state.pool, institution_id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    Ok(Json(config))
}

pub async fn check_rate_limit(
    State(state): State<AppState>,
    Query(params): Query<std::collections::HashMap<String, String>>,
) -> Result<Json<RateLimitInfo>, (StatusCode, String)> {
    let api_key = params.get("api_key")
        .ok_or((StatusCode::BAD_REQUEST, "api_key parameter required".to_string()))?;
    
    let endpoint = params.get("endpoint").map(|s| s.as_str()).unwrap_or("/api/v1");
    
    let info = service::check_rate_limit(&state.pool, api_key, endpoint)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    Ok(Json(info))
}

// ============================================================================
// USAGE ANALYTICS ENDPOINTS
// ============================================================================

pub async fn get_usage_stats(
    State(state): State<AppState>,
    user_id: Option<axum::extract::Extension<Uuid>>,
    Query(params): Query<std::collections::HashMap<String, String>>,
) -> Result<Json<UsageStats>, (StatusCode, String)> {
    let _user_id = user_id.ok_or((StatusCode::UNAUTHORIZED, "User not authenticated".to_string()))?;
    
    let days = params.get("days").and_then(|d| d.parse::<i32>().ok()).unwrap_or(30);
    
    let row = sqlx::query!(
        r#"SELECT 
            COUNT(*) as total,
            COUNT(*) FILTER (WHERE status_code < 400) as successful,
            COUNT(*) FILTER (WHERE status_code >= 400) as failed,
            AVG(response_time_ms) as avg_response_time,
            MIN(created_at) as period_start,
            MAX(created_at) as period_end
           FROM api_usage_logs 
           WHERE created_at >= NOW() - INTERVAL '1 day' * $1"#,
        days
    )
    .fetch_one(&state.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    Ok(Json(UsageStats {
        total_requests: row.total.unwrap_or(0),
        successful_requests: row.successful.unwrap_or(0),
        failed_requests: row.failed.unwrap_or(0),
        avg_response_time_ms: row.avg_response_time.unwrap_or(0.0),
        period_start: row.period_start.unwrap_or_else(|| chrono::Utc::now()),
        period_end: row.period_end.unwrap_or_else(|| chrono::Utc::now()),
    }))
}

// ============================================================================
// ROUTER
// ============================================================================

pub fn developer_router() -> Router<AppState> {
    axum::Router::new()
        // API Keys
        .route("/api-keys", axum::routing::post(create_api_key))
        .route("/api-keys", axum::routing::get(list_api_keys))
        .route("/api-keys/:key_id", axum::routing::delete(revoke_api_key))
        // Webhooks
        .route("/webhooks", axum::routing::post(create_webhook))
        .route("/webhooks", axum::routing::get(list_webhooks))
        .route("/webhooks/:webhook_id/toggle", axum::routing::post(toggle_webhook))
        .route("/webhooks/:webhook_id", axum::routing::delete(delete_webhook))
        // Integrations
        .route("/integrations", axum::routing::post(create_integration))
        .route("/integrations", axum::routing::get(list_integrations))
        .route("/integrations/:integration_id", axum::routing::delete(delete_integration))
        // SDK
        .route("/sdk/config", axum::routing::get(get_sdk_config))
        .route("/rate-limit", axum::routing::get(check_rate_limit))
        // Analytics
        .route("/usage/stats", axum::routing::get(get_usage_stats))
}

// ============================================================================
// HELPER FUNCTIONS
// ============================================================================

async fn get_user_institution(
    pool: &sqlx::PgPool,
    user_id: Uuid,
) -> Result<Uuid, (StatusCode, String)> {
    // Simplified - in production, fetch actual institution from users table
    let institution_id = sqlx::query_scalar!(
        "SELECT institution_id FROM users WHERE id = $1",
        user_id
    )
    .fetch_optional(pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
    .ok_or((StatusCode::NOT_FOUND, "User not found".to_string()))?;
    
    Ok(institution_id)
}

fn generate_api_key() -> String {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let key: String = (0..32)
        .map(|_| format!("{:02x}", rng.gen::<u8>()))
        .collect();
    format!("sk_live_{}", key)
}

fn hash_api_key(key: &str) -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(key.as_bytes());
    format!("{:x}", hasher.finalize())
}

fn parse_integration_type(s: &str) -> Option<IntegrationType> {
    match s.to_lowercase().as_str() {
        "moodle" => Some(IntegrationType::Moodle),
        "canvas" => Some(IntegrationType::Canvas),
        "googleclassroom" | "google_classroom" => Some(IntegrationType::GoogleClassroom),
        "microsoftteams" | "microsoft_teams" => Some(IntegrationType::MicrosoftTeams),
        "zoom" => Some(IntegrationType::Zoom),
        "salesforce" => Some(IntegrationType::Salesforce),
        _ => Some(IntegrationType::Custom),
    }
}
