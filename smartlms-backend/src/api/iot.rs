//! IoT API Endpoints

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
    routing::{get, post, put, delete},
    Router,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::models::*;
use crate::services::iot::IotService;
use std::sync::Arc;

/// Application state for IoT routes
#[derive(Clone)]
pub struct IotAppState {
    pub iot_service: Arc<IotService>,
}

/// Device list query parameters
#[derive(Debug, Deserialize)]
pub struct DeviceListQuery {
    pub device_type: Option<String>,
    pub status: Option<String>,
}

/// Register device request wrapper
#[derive(Debug, Deserialize)]
pub struct RegisterDevicePayload {
    #[serde(flatten)]
    pub request: RegisterDeviceRequest,
}

/// Submit reading request wrapper
#[derive(Debug, Deserialize)]
pub struct SubmitReadingPayload {
    #[serde(flatten)]
    pub request: SubmitReadingRequest,
}

/// Send command request wrapper
#[derive(Debug, Deserialize)]
pub struct SendCommandPayload {
    #[serde(flatten)]
    pub request: SendCommandRequest,
}

/// Create threshold request wrapper
#[derive(Debug, Deserialize)]
pub struct CreateThresholdPayload {
    #[serde(flatten)]
    pub request: AlertThresholdRequest,
}

/// Response for device registration
#[derive(Debug, Serialize)]
pub struct RegisterDeviceResponse {
    pub success: bool,
    pub device: IotDevice,
}

/// Response for reading submission
#[derive(Debug, Serialize)]
pub struct SubmitReadingResponse {
    pub success: bool,
    pub reading: SensorReading,
}

/// Response for command sending
#[derive(Debug, Serialize)]
pub struct SendCommandResponse {
    pub success: bool,
    pub command: DeviceCommand,
    pub message: String,
}

/// Response with device list
#[derive(Debug, Serialize)]
pub struct DeviceListResponse {
    pub devices: Vec<IotDevice>,
    pub count: usize,
}

/// Response with readings
#[derive(Debug, Serialize)]
pub struct ReadingsListResponse {
    pub readings: Vec<SensorReading>,
    pub count: usize,
}

/// Response with alerts
#[derive(Debug, Serialize)]
pub struct AlertsListResponse {
    pub alerts: Vec<IotAlert>,
    pub count: usize,
}

/// Response with analytics
#[derive(Debug, Serialize)]
pub struct AnalyticsResponse {
    pub analytics: SensorAnalytics,
}

/// Health check response
#[derive(Debug, Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub service: String,
}

/// Register a new IoT device
pub async fn register_device(
    State(state): State<IotAppState>,
    Json(payload): Json<RegisterDevicePayload>,
) -> Result<Json<RegisterDeviceResponse>, StatusCode> {
    match state.iot_service.register_device(payload.request).await {
        Ok(device) => Ok(Json(RegisterDeviceResponse {
            success: true,
            device,
        })),
        Err(e) => {
            eprintln!("Error registering device: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Get device by ID
pub async fn get_device(
    State(state): State<IotAppState>,
    Path(device_id): Path<Uuid>,
) -> Result<Json<IotDevice>, StatusCode> {
    match state.iot_service.get_device(device_id).await {
        Ok(Some(device)) => Ok(Json(device)),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(e) => {
            eprintln!("Error getting device: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// List devices for an institution
pub async fn list_devices(
    State(state): State<IotAppState>,
    Path(institution_id): Path<Uuid>,
    query: Option<DeviceListQuery>,
) -> Result<Json<DeviceListResponse>, StatusCode> {
    let device_type = query.and_then(|q| q.device_type).and_then(|dt| {
        serde_json::from_str(&format!("\"{}\"", dt)).ok()
    });
    
    let status = query.and_then(|q| q.status).and_then(|s| {
        serde_json::from_str(&format!("\"{}\"", s)).ok()
    });

    match state.iot_service.list_devices(institution_id, device_type, status).await {
        Ok(devices) => Ok(Json(DeviceListResponse {
            devices,
            count: devices.len(),
        })),
        Err(e) => {
            eprintln!("Error listing devices: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Update device status
pub async fn update_device_status(
    State(state): State<IotAppState>,
    Path(device_id): Path<Uuid>,
    Json(payload): Json<UpdateDeviceStatusRequest>,
) -> Result<Json<IotDevice>, StatusCode> {
    match state.iot_service.update_device_status(device_id, payload.status, payload.firmware_version).await {
        Ok(device) => Ok(Json(device)),
        Err(e) => {
            eprintln!("Error updating device status: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Submit sensor reading
pub async fn submit_reading(
    State(state): State<IotAppState>,
    Path(device_id): Path<Uuid>,
    Json(payload): Json<SubmitReadingPayload>,
) -> Result<Json<SubmitReadingResponse>, StatusCode> {
    match state.iot_service.submit_reading(device_id, payload.request).await {
        Ok(reading) => Ok(Json(SubmitReadingResponse {
            success: true,
            reading,
        })),
        Err(e) => {
            eprintln!("Error submitting reading: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Get recent readings for a device
pub async fn get_readings(
    State(state): State<IotAppState>,
    Path((device_id, limit)): Path<(Uuid, i32)>,
) -> Result<Json<ReadingsListResponse>, StatusCode> {
    match state.iot_service.get_readings(device_id, limit, None).await {
        Ok(readings) => Ok(Json(ReadingsListResponse {
            readings,
            count: readings.len(),
        })),
        Err(e) => {
            eprintln!("Error getting readings: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Send command to device
pub async fn send_command(
    State(state): State<IotAppState>,
    Path(device_id): Path<Uuid>,
    Json(payload): Json<SendCommandPayload>,
) -> Result<Json<SendCommandResponse>, StatusCode> {
    match state.iot_service.send_command(device_id, payload.request).await {
        Ok(command) => Ok(Json(SendCommandResponse {
            success: true,
            command,
            message: "Command queued for execution".to_string(),
        })),
        Err(e) => {
            eprintln!("Error sending command: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Get command status
pub async fn get_command(
    State(state): State<IotAppState>,
    Path(command_id): Path<Uuid>,
) -> Result<Json<DeviceCommand>, StatusCode> {
    match state.iot_service.get_command(command_id).await {
        Ok(Some(command)) => Ok(Json(command)),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(e) => {
            eprintln!("Error getting command: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Create alert threshold
pub async fn create_alert_threshold(
    State(state): State<IotAppState>,
    Json(payload): Json<CreateThresholdPayload>,
) -> Result<Json<AlertThreshold>, StatusCode> {
    match state.iot_service.create_alert_threshold(payload.request).await {
        Ok(threshold) => Ok(Json(threshold)),
        Err(e) => {
            eprintln!("Error creating threshold: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Get alerts for a device
pub async fn get_alerts(
    State(state): State<IotAppState>,
    Path((device_id, include_resolved)): Path<(Uuid, bool)>,
) -> Result<Json<AlertsListResponse>, StatusCode> {
    match state.iot_service.get_alerts(device_id, include_resolved).await {
        Ok(alerts) => Ok(Json(AlertsListResponse {
            alerts,
            count: alerts.len(),
        })),
        Err(e) => {
            eprintln!("Error getting alerts: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Resolve an alert
pub async fn resolve_alert(
    State(state): State<IotAppState>,
    Path(alert_id): Path<Uuid>,
) -> Result<Json<IotAlert>, StatusCode> {
    match state.iot_service.resolve_alert(alert_id).await {
        Ok(alert) => Ok(Json(alert)),
        Err(e) => {
            eprintln!("Error resolving alert: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Get sensor analytics
pub async fn get_analytics(
    State(state): State<IotAppState>,
    Path((device_id, sensor_type, start, end)): Path<(Uuid, String, chrono::DateTime<chrono::Utc>, chrono::DateTime<chrono::Utc>)>,
) -> Result<Json<AnalyticsResponse>, StatusCode> {
    match state.iot_service.get_sensor_analytics(device_id, &sensor_type, start, end).await {
        Ok(analytics) => Ok(Json(AnalyticsResponse { analytics })),
        Err(e) => {
            eprintln!("Error getting analytics: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Delete a device
pub async fn delete_device(
    State(state): State<IotAppState>,
    Path(device_id): Path<Uuid>,
) -> Result<StatusCode, StatusCode> {
    match state.iot_service.delete_device(device_id).await {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(e) => {
            eprintln!("Error deleting device: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Health check endpoint
pub async fn health_check() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "healthy".to_string(),
        service: "iot".to_string(),
    })
}

/// Create the IoT router
pub fn iot_router() -> Router {
    Router::new()
        .route("/health", get(health_check))
        .route("/devices", post(register_device))
        .route("/devices/:institution_id", get(list_devices))
        .route("/devices/:id", get(get_device))
        .route("/devices/:id", delete(delete_device))
        .route("/devices/:id/status", put(update_device_status))
        .route("/devices/:id/readings", post(submit_reading))
        .route("/devices/:id/readings/:limit", get(get_readings))
        .route("/devices/:id/commands", post(send_command))
        .route("/commands/:id", get(get_command))
        .route("/thresholds", post(create_alert_threshold))
        .route("/alerts/:device_id/:include_resolved", get(get_alerts))
        .route("/alerts/:id/resolve", post(resolve_alert))
        .route("/analytics/:device_id/:sensor_type/:start/:end", get(get_analytics))
}
