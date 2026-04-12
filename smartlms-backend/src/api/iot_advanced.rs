//! API Endpoints for MQTT, Edge Computing, and Predictive Maintenance
//! 
//! This module provides REST API endpoints for:
//! - MQTT configuration and status
//! - Edge computing node management
//! - Predictive maintenance analytics

use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Shared application state for IoT advanced features
#[derive(Clone)]
pub struct IotAdvancedState {
    pub mqtt_client: Option<Arc<crate::services::mqtt::MqttClientManager>>,
    pub edge_manager: Arc<RwLock<crate::services::edge_computing::EdgeComputingManager>>,
    pub maintenance_manager: Arc<RwLock<crate::services::predictive_maintenance::PredictiveMaintenanceManager>>,
}

// ============== MQTT Endpoints ==============

/// MQTT configuration request
#[derive(Debug, Deserialize)]
pub struct MqttConfigRequest {
    pub broker_url: String,
    pub port: u16,
    pub username: Option<String>,
    pub password: Option<String>,
    pub client_id: Option<String>,
}

/// MQTT status response
#[derive(Debug, Serialize)]
pub struct MqttStatusResponse {
    pub connected: bool,
    pub broker_url: String,
    pub client_id: String,
    pub subscribed_topics: Vec<String>,
}

/// Configure MQTT connection
pub async fn configure_mqtt_handler(
    State(state): State<IotAdvancedState>,
    Json(payload): Json<MqttConfigRequest>,
) -> Result<Json<MqttStatusResponse>, StatusCode> {
    match crate::services::mqtt::init_mqtt_client().await {
        Ok(client) => {
            // In a real implementation, we'd store the client in state
            Ok(Json(MqttStatusResponse {
                connected: true,
                broker_url: payload.broker_url,
                client_id: payload.client_id.unwrap_or_else(|| "smartlms".to_string()),
                subscribed_topics: vec!["smartlms/iot/sensors/+".to_string()],
            }))
        }
        Err(e) => {
            eprintln!("Failed to configure MQTT: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Get MQTT status
pub async fn get_mqtt_status_handler(
    State(state): State<IotAdvancedState>,
) -> Result<Json<MqttStatusResponse>, StatusCode> {
    let connected = state.mqtt_client.is_some();
    
    Ok(Json(MqttStatusResponse {
        connected,
        broker_url: std::env::var("MQTT_BROKER_URL").unwrap_or_else(|_| "not configured".to_string()),
        client_id: std::env::var("MQTT_CLIENT_ID").unwrap_or_else(|_| "not configured".to_string()),
        subscribed_topics: if connected {
            vec!["smartlms/iot/sensors/+".to_string()]
        } else {
            vec![]
        },
    }))
}

// ============== Edge Computing Endpoints ==============

/// Edge node registration request
#[derive(Debug, Deserialize)]
pub struct RegisterEdgeNodeRequest {
    pub node_id: String,
    pub location: String,
    pub max_buffer_size: Option<usize>,
    pub processing_interval_ms: Option<u64>,
    pub enabled_rules: Vec<String>,
}

/// Edge node response
#[derive(Debug, Serialize)]
pub struct EdgeNodeResponse {
    pub node_id: String,
    pub location: String,
    pub status: String,
    pub rules_count: usize,
}

/// Register edge node
pub async fn register_edge_node_handler(
    State(state): State<IotAdvancedState>,
    Json(payload): Json<RegisterEdgeNodeRequest>,
) -> Result<Json<EdgeNodeResponse>, StatusCode> {
    let config = crate::services::edge_computing::EdgeNodeConfig {
        node_id: payload.node_id.clone(),
        location: payload.location.clone(),
        max_buffer_size: payload.max_buffer_size.unwrap_or(1000),
        processing_interval_ms: payload.processing_interval_ms.unwrap_or(1000),
        enabled_rules: payload.enabled_rules.clone(),
    };

    // Create alert channel for edge node
    let (alert_tx, _) = tokio::sync::mpsc::channel(1000);
    
    let mut edge_manager = state.edge_manager.write().await;
    edge_manager.register_node(config, alert_tx);

    Ok(Json(EdgeNodeResponse {
        node_id: payload.node_id,
        location: payload.location,
        status: "active".to_string(),
        rules_count: payload.enabled_rules.len(),
    }))
}

/// Get edge nodes
pub async fn get_edge_nodes_handler(
    State(state): State<IotAdvancedState>,
) -> Result<Json<Vec<EdgeNodeResponse>>, StatusCode> {
    let edge_manager = state.edge_manager.read().await;
    
    // In a real implementation, we'd iterate through actual nodes
    Ok(Json(vec![]))
}

/// Add edge rule
#[derive(Debug, Deserialize)]
pub struct AddEdgeRuleRequest {
    pub node_id: String,
    pub rule_id: String,
    pub rule_type: String,
    pub threshold: f64,
    pub action: String,
}

pub async fn add_edge_rule_handler(
    State(state): State<IotAdvancedState>,
    Json(payload): Json<AddEdgeRuleRequest>,
) -> Result<StatusCode, StatusCode> {
    let rule_type = match payload.rule_type.as_str() {
        "threshold_above" => crate::services::edge_computing::EdgeRuleType::ThresholdAbove,
        "threshold_below" => crate::services::edge_computing::EdgeRuleType::ThresholdBelow,
        _ => return Err(StatusCode::BAD_REQUEST),
    };

    let action = match payload.action.as_str() {
        "alert" => crate::services::edge_computing::EdgeAction::Alert("Rule triggered".to_string()),
        "forward" => crate::services::edge_computing::EdgeAction::Forward,
        _ => return Err(StatusCode::BAD_REQUEST),
    };

    let rule = crate::services::edge_computing::EdgeRule {
        rule_id: payload.rule_id,
        rule_type,
        threshold: payload.threshold,
        action,
        enabled: true,
    };

    let mut edge_manager = state.edge_manager.write().await;
    if let Some(node) = edge_manager.get_node_mut(&payload.node_id) {
        node.add_rule(rule);
        Ok(StatusCode::OK)
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

// ============== Predictive Maintenance Endpoints ==============

/// Device health metrics request
#[derive(Debug, Deserialize)]
pub struct DeviceMetricsRequest {
    pub device_id: Uuid,
    pub uptime_hours: f64,
    pub error_count_24h: usize,
    pub avg_temperature: f64,
    pub temperature_variance: f64,
    pub vibration_level: f64,
    pub power_consumption: f64,
    pub last_maintenance_days_ago: usize,
    pub age_days: usize,
    pub failure_probability: Option<f32>,
}

/// Prediction response
#[derive(Debug, Serialize)]
pub struct PredictionResponse {
    pub device_id: Uuid,
    pub status: String,
    pub confidence: f64,
    pub recommended_action: String,
    pub estimated_days_to_failure: usize,
}

/// Analyze device health
pub async fn analyze_device_health_handler(
    State(state): State<IotAdvancedState>,
    Json(payload): Json<DeviceMetricsRequest>,
) -> Result<Json<PredictionResponse>, StatusCode> {
    let metrics = crate::services::predictive_maintenance::DeviceHealthMetrics {
        device_id: payload.device_id,
        uptime_hours: payload.uptime_hours,
        error_count_24h: payload.error_count_24h,
        avg_temperature: payload.avg_temperature,
        temperature_variance: payload.temperature_variance,
        vibration_level: payload.vibration_level,
        power_consumption: payload.power_consumption,
        last_maintenance_days_ago: payload.last_maintenance_days_ago,
        age_days: payload.age_days,
        failure_probability: payload.failure_probability.unwrap_or(0.0),
    };

    let mut maintenance_manager = state.maintenance_manager.write().await;
    
    match maintenance_manager.analyze_device(metrics) {
        Ok(prediction) => Ok(Json(PredictionResponse {
            device_id: payload.device_id,
            status: format!("{:?}", prediction.status),
            confidence: prediction.confidence,
            recommended_action: prediction.recommended_action,
            estimated_days_to_failure: prediction.estimated_days_to_failure,
        })),
        Err(e) => {
            eprintln!("Error analyzing device: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Get maintenance schedules
#[derive(Debug, Serialize)]
pub struct MaintenanceScheduleResponse {
    pub device_id: Uuid,
    pub priority: u8,
    pub recommended_date: chrono::DateTime<chrono::Utc>,
    pub estimated_duration_minutes: usize,
    pub notes: String,
}

pub async fn get_maintenance_schedules_handler(
    State(state): State<IotAdvancedState>,
) -> Result<Json<Vec<MaintenanceScheduleResponse>>, StatusCode> {
    let maintenance_manager = state.maintenance_manager.read().await;
    let schedules = maintenance_manager.get_upcoming_schedules(10);

    let response: Vec<MaintenanceScheduleResponse> = schedules
        .iter()
        .map(|s| MaintenanceScheduleResponse {
            device_id: s.device_id,
            priority: s.priority,
            recommended_date: s.recommended_date,
            estimated_duration_minutes: s.estimated_duration_minutes,
            notes: s.notes.clone(),
        })
        .collect();

    Ok(Json(response))
}

/// Train predictive model
#[derive(Debug, Serialize)]
pub struct TrainingResponse {
    pub success: bool,
    pub message: String,
    pub samples_used: usize,
}

pub async fn train_predictive_model_handler(
    State(state): State<IotAdvancedState>,
) -> Result<Json<TrainingResponse>, StatusCode> {
    let mut maintenance_manager = state.maintenance_manager.write().await;
    
    match maintenance_manager.train_model() {
        Ok(_) => Ok(Json(TrainingResponse {
            success: true,
            message: "Model trained successfully".to_string(),
            samples_used: 0, // Would count actual samples in real implementation
        })),
        Err(e) => {
            eprintln!("Error training model: {}", e);
            Ok(Json(TrainingResponse {
                success: false,
                message: format!("Training failed: {}", e),
                samples_used: 0,
            }))
        }
    }
}

/// Get device health trend
#[derive(Debug, Deserialize)]
pub struct HealthTrendRequest {
    pub device_id: Uuid,
    pub days: Option<usize>,
}

#[derive(Debug, Serialize)]
pub struct HealthTrendResponse {
    pub device_id: Uuid,
    pub trend: String,
    pub data_points: usize,
    pub avg_failure_probability: f32,
}

pub async fn get_device_health_trend_handler(
    State(state): State<IotAdvancedState>,
    Json(payload): Json<HealthTrendRequest>,
) -> Result<Json<HealthTrendResponse>, StatusCode> {
    let maintenance_manager = state.maintenance_manager.read().await;
    let days = payload.days.unwrap_or(30);

    match maintenance_manager.get_health_trend(payload.device_id, days) {
        Some(trend) => Ok(Json(HealthTrendResponse {
            device_id: payload.device_id,
            trend: format!("{:?}", trend.trend),
            data_points: trend.data_points,
            avg_failure_probability: trend.avg_failure_probability,
        })),
        None => Err(StatusCode::NOT_FOUND),
    }
}

// ============== Route Configuration ==============

/// Create IoT advanced routes
pub fn create_iot_advanced_routes(state: IotAdvancedState) -> Router {
    Router::new()
        // MQTT routes
        .route("/mqtt/config", post(configure_mqtt_handler))
        .route("/mqtt/status", get(get_mqtt_status_handler))
        
        // Edge computing routes
        .route("/edge/nodes", post(register_edge_node_handler))
        .route("/edge/nodes", get(get_edge_nodes_handler))
        .route("/edge/rules", post(add_edge_rule_handler))
        
        // Predictive maintenance routes
        .route("/maintenance/analyze", post(analyze_device_health_handler))
        .route("/maintenance/schedules", get(get_maintenance_schedules_handler))
        .route("/maintenance/train", post(train_predictive_model_handler))
        .route("/maintenance/trend", get(get_device_health_trend_handler))
        
        .with_state(state)
}
