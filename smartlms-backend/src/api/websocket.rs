//! Phase 15: WebSocket Real-time API Endpoints
//! Provides WebSocket endpoints for real-time features

use axum::{
    extract::{Path, State, ws::WebSocketUpgrade},
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::services::websocket::WebSocketManager;

/// Application state for WebSocket routes
#[derive(Clone)]
pub struct WebSocketState {
    pub manager: Arc<WebSocketManager>,
}

/// Create the WebSocket router
pub fn create_websocket_routes(state: WebSocketState) -> axum::Router {
    axum::Router::new()
        .route("/ws", axum::routing::get(ws_handler))
        .route("/ws/:channel", axum::routing::get(ws_channel_handler))
        .route("/stats", axum::routing::get(stats_handler))
        .route("/notification", axum::routing::post(broadcast_notification_handler))
        .with_state(state)
}

/// Main WebSocket handler
async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<WebSocketState>,
) -> impl IntoResponse {
    let connection_id = uuid::Uuid::new_v4().to_string();
    ws.on_upgrade(move |socket| async move {
        super::super::services::websocket::handle_socket(socket, connection_id, state.manager).await;
    })
}

/// WebSocket handler with channel subscription
async fn ws_channel_handler(
    Path(channel): Path<String>,
    ws: WebSocketUpgrade,
    State(state): State<WebSocketState>,
) -> impl IntoResponse {
    let connection_id = uuid::Uuid::new_v4().to_string();
    let manager = state.manager.clone();
    
    ws.on_upgrade(move |socket| async move {
        // Register connection
        manager.register_connection(connection_id.clone()).await;
        
        // Auto-subscribe to the channel
        if let Err(e) = manager.subscribe(&connection_id, &channel).await {
            tracing::error!("Failed to subscribe to channel {}: {}", channel, e);
        }
        
        // Handle socket
        super::super::services::websocket::handle_socket(socket, connection_id, manager).await;
    })
}

/// Get WebSocket statistics
async fn stats_handler(
    State(state): State<WebSocketState>,
) -> Json<serde_json::Value> {
    let stats = state.manager.get_stats().await;
    Json(serde_json::to_value(stats).unwrap_or_default())
}

/// Request to broadcast a notification
#[derive(Debug, Deserialize)]
pub struct BroadcastNotificationRequest {
    pub title: String,
    pub message: String,
    pub notification_type: String,
    pub data: Option<serde_json::Value>,
}

/// Response from broadcast operation
#[derive(Debug, Serialize)]
pub struct BroadcastNotificationResponse {
    pub success: bool,
    pub recipients: usize,
}

/// Broadcast notification to all users
async fn broadcast_notification_handler(
    State(state): State<WebSocketState>,
    Json(payload): Json<BroadcastNotificationRequest>,
) -> Json<BroadcastNotificationResponse> {
    state.manager.broadcast_notification(
        payload.title,
        payload.message,
        payload.notification_type,
    ).await;
    
    // Get approximate recipient count
    let stats = state.manager.get_stats().await;
    let recipients = stats.get("authenticated_users")
        .and_then(|v| v.as_u64())
        .unwrap_or(0) as usize;
    
    Json(BroadcastNotificationResponse {
        success: true,
        recipients,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_websocket_state_creation() {
        let manager = Arc::new(WebSocketManager::new());
        let state = WebSocketState { manager };
        assert_eq!(*state.manager.connection_count.read().await, 0);
    }
}
