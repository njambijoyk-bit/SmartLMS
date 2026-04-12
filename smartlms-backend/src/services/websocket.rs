//! Phase 15: WebSocket Real-time Features Service
//! Provides real-time communication for notifications, live classes, collaboration, and IoT updates

use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::IntoResponse,
};
use futures::{sink::SinkExt, stream::StreamExt};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{broadcast, RwLock};
use tracing::{info, warn, error};
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// Message types for WebSocket communication
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum WsMessage {
    /// Authentication message
    Auth { token: String },
    
    /// Subscribe to a channel
    Subscribe { channel: String },
    
    /// Unsubscribe from a channel
    Unsubscribe { channel: String },
    
    /// Send a message to a channel
    Send {
        channel: String,
        payload: serde_json::Value,
    },
    
    /// Receive a message from a channel
    Receive {
        channel: String,
        payload: serde_json::Value,
        timestamp: i64,
    },
    
    /// Notification message
    Notification {
        title: String,
        message: String,
        notification_type: String,
        data: Option<serde_json::Value>,
    },
    
    /// Live class events
    LiveClass {
        class_id: String,
        event: String,
        data: Option<serde_json::Value>,
    },
    
    /// Collaboration events (whiteboard, code editing, etc.)
    Collaboration {
        room_id: String,
        event: String,
        user_id: String,
        data: serde_json::Value,
    },
    
    /// IoT real-time updates
    IotUpdate {
        device_id: String,
        sensor_type: String,
        value: f64,
        timestamp: i64,
    },
    
    /// Error message
    Error {
        code: String,
        message: String,
    },
    
    /// Success acknowledgment
    Ack {
        message_id: String,
        status: String,
    },
}

/// Connection state for a WebSocket client
#[derive(Debug, Clone)]
pub struct ConnectionState {
    pub user_id: Option<String>,
    pub channels: Vec<String>,
    pub connected_at: chrono::DateTime<chrono::Utc>,
    pub last_activity: chrono::DateTime<chrono::Utc>,
}

impl Default for ConnectionState {
    fn default() -> Self {
        Self {
            user_id: None,
            channels: Vec::new(),
            connected_at: chrono::Utc::now(),
            last_activity: chrono::Utc::now(),
        }
    }
}

/// WebSocket Manager for handling connections and broadcasting
pub struct WebSocketManager {
    /// Broadcast channel for global messages
    broadcast_tx: broadcast::Sender<WsMessage>,
    
    /// User-specific channels
    user_channels: Arc<RwLock<HashMap<String, broadcast::Sender<WsMessage>>>>,
    
    /// Channel-specific subscribers
    channel_subscribers: Arc<RwLock<HashMap<String, Vec<String>>>>,
    
    /// Active connections
    connections: Arc<RwLock<HashMap<String, ConnectionState>>>,
    
    /// Connection count
    connection_count: Arc<RwLock<usize>>,
}

impl WebSocketManager {
    /// Create a new WebSocket manager
    pub fn new() -> Self {
        let (broadcast_tx, _) = broadcast::channel(1000);
        
        Self {
            broadcast_tx,
            user_channels: Arc::new(RwLock::new(HashMap::new())),
            channel_subscribers: Arc::new(RwLock::new(HashMap::new())),
            connections: Arc::new(RwLock::new(HashMap::new())),
            connection_count: Arc::new(RwLock::new(0)),
        }
    }
    
    /// Get a clone of the manager for sharing
    pub fn clone(&self) -> Self {
        Self {
            broadcast_tx: self.broadcast_tx.clone(),
            user_channels: Arc::clone(&self.user_channels),
            channel_subscribers: Arc::clone(&self.channel_subscribers),
            connections: Arc::clone(&self.connections),
            connection_count: Arc::clone(&self.connection_count),
        }
    }
    
    /// Register a new connection
    pub async fn register_connection(&self, connection_id: String) {
        let mut connections = self.connections.write().await;
        connections.insert(connection_id.clone(), ConnectionState::default());
        
        let mut count = self.connection_count.write().await;
        *count += 1;
        
        info!("New WebSocket connection: {} (total: {})", connection_id, *count);
    }
    
    /// Unregister a connection
    pub async fn unregister_connection(&self, connection_id: &str) {
        {
            let mut connections = self.connections.write().await;
            if let Some(state) = connections.remove(connection_id) {
                // Remove from all channels
                for channel in state.channels {
                    if let Ok(mut subscribers) = self.channel_subscribers.write() {
                        if let Some(subs) = subscribers.get_mut(&channel) {
                            subs.retain(|id| id != connection_id);
                        }
                    }
                }
            }
        }
        
        {
            let mut count = self.connection_count.write().await;
            *count = count.saturating_sub(1);
            info!("WebSocket connection closed: {} (total: {})", connection_id, *count);
        }
    }
    
    /// Authenticate a connection with a user
    pub async fn authenticate(&self, connection_id: &str, user_id: String) -> Result<(), String> {
        let mut connections = self.connections.write().await;
        
        if let Some(state) = connections.get_mut(connection_id) {
            state.user_id = Some(user_id.clone());
            state.last_activity = chrono::Utc::now();
            
            // Create user-specific channel
            let (tx, _) = broadcast::channel(500);
            let mut user_channels = self.user_channels.write().await;
            user_channels.insert(user_id, tx);
            
            info!("User {} authenticated on connection {}", user_id, connection_id);
            Ok(())
        } else {
            Err("Connection not found".to_string())
        }
    }
    
    /// Subscribe to a channel
    pub async fn subscribe(&self, connection_id: &str, channel: &str) -> Result<(), String> {
        let mut connections = self.connections.write().await;
        
        if let Some(state) = connections.get_mut(connection_id) {
            if !state.channels.contains(&channel.to_string()) {
                state.channels.push(channel.to_string());
                state.last_activity = chrono::Utc::now();
            }
        } else {
            return Err("Connection not found".to_string());
        }
        
        let mut subscribers = self.channel_subscribers.write().await;
        subscribers
            .entry(channel.to_string())
            .or_insert_with(Vec::new)
            .push(connection_id.to_string());
        
        info!("Connection {} subscribed to channel {}", connection_id, channel);
        Ok(())
    }
    
    /// Unsubscribe from a channel
    pub async fn unsubscribe(&self, connection_id: &str, channel: &str) -> Result<(), String> {
        {
            let mut connections = self.connections.write().await;
            
            if let Some(state) = connections.get_mut(connection_id) {
                state.channels.retain(|c| c != channel);
                state.last_activity = chrono::Utc::now();
            }
        }
        
        {
            let mut subscribers = self.channel_subscribers.write().await;
            if let Some(subs) = subscribers.get_mut(channel) {
                subs.retain(|id| id != connection_id);
            }
        }
        
        info!("Connection {} unsubscribed from channel {}", connection_id, channel);
        Ok(())
    }
    
    /// Broadcast a message to all subscribers of a channel
    pub async fn broadcast_to_channel(&self, channel: &str, message: WsMessage) {
        let subscribers = {
            let subs = self.channel_subscribers.read().await;
            subs.get(channel).cloned().unwrap_or_default()
        };
        
        for connection_id in subscribers {
            if let Some(tx) = self.get_user_sender(&connection_id).await {
                let _ = tx.send(message.clone());
            }
        }
        
        info!("Broadcasted message to channel {} ({} subscribers)", channel, subscribers.len());
    }
    
    /// Send a message to a specific user
    pub async fn send_to_user(&self, user_id: &str, message: WsMessage) -> Result<(), String> {
        let user_channels = self.user_channels.read().await;
        
        if let Some(tx) = user_channels.get(user_id) {
            tx.send(message).map_err(|e| e.to_string())?;
            Ok(())
        } else {
            Err(format!("User {} not connected", user_id))
        }
    }
    
    /// Broadcast a notification to all users
    pub async fn broadcast_notification(&self, title: String, message: String, notification_type: String) {
        let ws_message = WsMessage::Notification {
            title,
            message,
            notification_type,
            data: None,
        };
        
        let _ = self.broadcast_tx.send(ws_message);
    }
    
    /// Send live class event
    pub async fn send_live_class_event(&self, class_id: &str, event: String, data: Option<serde_json::Value>) {
        let channel = format!("live_class:{}", class_id);
        let message = WsMessage::LiveClass {
            class_id: class_id.to_string(),
            event,
            data,
        };
        
        self.broadcast_to_channel(&channel, message).await;
    }
    
    /// Send collaboration event
    pub async fn send_collaboration_event(
        &self,
        room_id: &str,
        event: String,
        user_id: String,
        data: serde_json::Value,
    ) {
        let channel = format!("collab:{}", room_id);
        let message = WsMessage::Collaboration {
            room_id: room_id.to_string(),
            event,
            user_id,
            data,
        };
        
        self.broadcast_to_channel(&channel, message).await;
    }
    
    /// Send IoT update
    pub async fn send_iot_update(&self, device_id: &str, sensor_type: String, value: f64) {
        let channel = format!("iot:{}", device_id);
        let message = WsMessage::IotUpdate {
            device_id: device_id.to_string(),
            sensor_type,
            value,
            timestamp: chrono::Utc::now().timestamp(),
        };
        
        self.broadcast_to_channel(&channel, message).await;
    }
    
    /// Get sender for a connection
    async fn get_user_sender(&self, connection_id: &str) -> Option<broadcast::Sender<WsMessage>> {
        let connections = self.connections.read().await;
        
        if let Some(state) = connections.get(connection_id) {
            if let Some(ref user_id) = state.user_id {
                let user_channels = self.user_channels.read().await;
                return user_channels.get(user_id).cloned();
            }
        }
        
        None
    }
    
    /// Get connection statistics
    pub async fn get_stats(&self) -> HashMap<String, serde_json::Value> {
        let count = *self.connection_count.read().await;
        let connections = self.connections.read().await;
        let channels = self.channel_subscribers.read().await;
        
        let mut stats = HashMap::new();
        stats.insert("total_connections".to_string(), serde_json::json!(count));
        
        let authenticated = connections.values().filter(|c| c.user_id.is_some()).count();
        stats.insert("authenticated_users".to_string(), serde_json::json!(authenticated));
        
        stats.insert("active_channels".to_string(), serde_json::json!(channels.len()));
        
        let mut channel_stats = HashMap::new();
        for (channel, subs) in channels.iter() {
            channel_stats.insert(channel.clone(), serde_json::json!(subs.len()));
        }
        stats.insert("channel_subscribers".to_string(), serde_json::json!(channel_stats));
        
        stats
    }
    
    /// Send assignment submission notification
    pub async fn notify_assignment_submission(
        &self,
        student_id: &str,
        assignment_id: &str,
        course_id: &str,
    ) {
        let message = WsMessage::Notification {
            title: "Assignment Submitted".to_string(),
            message: format!("Your assignment {} has been submitted", assignment_id),
            notification_type: "assignment".to_string(),
            data: Some(json!({
                "assignment_id": assignment_id,
                "course_id": course_id,
                "submitted_at": Utc::now()
            })),
        };
        
        if let Err(e) = self.send_to_user(student_id, message).await {
            warn!("Failed to send assignment submission notification to {}: {}", student_id, e);
        }
    }
    
    /// Send grade release notification
    pub async fn notify_grade_released(
        &self,
        student_id: &str,
        assignment_id: &str,
        course_id: &str,
        grade: f64,
    ) {
        let message = WsMessage::Notification {
            title: "Grade Released".to_string(),
            message: format!("Your grade for assignment {} is available", assignment_id),
            notification_type: "grade".to_string(),
            data: Some(json!({
                "assignment_id": assignment_id,
                "course_id": course_id,
                "grade": grade,
                "released_at": Utc::now()
            })),
        };
        
        if let Err(e) = self.send_to_user(student_id, message).await {
            warn!("Failed to send grade notification to {}: {}", student_id, e);
        }
    }
    
    /// Send live class event (student joined/left)
    pub async fn notify_live_class_participant(
        &self,
        class_id: &str,
        user_id: &str,
        user_name: &str,
        action: &str, // "joined" or "left"
    ) {
        self.send_live_class_event(
            class_id,
            format!("participant_{}", action),
            Some(json!({
                "user_id": user_id,
                "user_name": user_name,
                "timestamp": Utc::now()
            })),
        ).await;
    }
    
    /// Send quiz/assessment notification
    pub async fn notify_assessment(
        &self,
        user_id: &str,
        assessment_id: &str,
        assessment_type: &str,
        message: &str,
        data: Option<serde_json::Value>,
    ) {
        let ws_message = WsMessage::Notification {
            title: format!("{} Update", assessment_type),
            message: message.to_string(),
            notification_type: "assessment".to_string(),
            data: data.or_else(|| Some(json!({
                "assessment_id": assessment_id,
                "type": assessment_type,
                "timestamp": Utc::now()
            }))),
        };
        
        if let Err(e) = self.send_to_user(user_id, ws_message).await {
            warn!("Failed to send assessment notification to {}: {}", user_id, e);
        }
    }
    
    /// Get online users count
    pub async fn get_online_count(&self) -> usize {
        let connections = self.connections.read().await;
        connections.values().filter(|c| c.user_id.is_some()).count()
    }
    
    /// Check if a user is online
    pub async fn is_user_online(&self, user_id: &str) -> bool {
        let user_channels = self.user_channels.read().await;
        user_channels.contains_key(user_id)
    }
    
    /// Get list of online users
    pub async fn get_online_users(&self) -> Vec<String> {
        let connections = self.connections.read().await;
        connections
            .values()
            .filter_map(|c| c.user_id.clone())
            .collect()
    }
}

impl Default for WebSocketManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Handle WebSocket upgrade
pub async fn handle_ws_connection(
    ws: WebSocketUpgrade,
    connection_id: String,
    manager: Arc<WebSocketManager>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| async move {
        handle_socket(socket, connection_id, manager).await;
    })
}

/// Handle individual WebSocket connection
async fn handle_socket(socket: WebSocket, connection_id: String, manager: Arc<WebSocketManager>) {
    let (mut sender, mut receiver) = socket.split();
    
    // Register connection
    manager.register_connection(connection_id.clone()).await;
    
    // Create channel for receiving messages from broadcast
    let mut broadcast_rx = manager.broadcast_tx.subscribe();
    
    // Spawn task to handle incoming messages
    let recv_manager = manager.clone();
    let recv_connection_id = connection_id.clone();
    
    let recv_handle = tokio::spawn(async move {
        while let Some(msg) = receiver.next().await {
            match msg {
                Ok(Message::Text(text)) => {
                    if let Ok(ws_msg) = serde_json::from_str::<WsMessage>(&text) {
                        handle_incoming_message(&recv_manager, &recv_connection_id, ws_msg).await;
                    }
                }
                Ok(Message::Close(_)) => {
                    break;
                }
                Err(e) => {
                    error!("WebSocket error: {}", e);
                    break;
                }
                _ => {}
            }
        }
    });
    
    // Spawn task to handle outgoing messages
    let send_manager = manager.clone();
    let send_connection_id = connection_id.clone();
    
    let send_handle = tokio::spawn(async move {
        loop {
            tokio::select! {
                Ok(msg) = broadcast_rx.recv() => {
                    // Check if connection is subscribed to relevant channels
                    let connections = send_manager.connections.read().await;
                    if let Some(state) = connections.get(&send_connection_id) {
                        // Send message if appropriate
                        let ws_msg = serde_json::to_string(&msg).unwrap_or_default();
                        if sender.send(Message::Text(ws_msg)).await.is_err() {
                            break;
                        }
                    }
                }
                else => break,
            }
        }
    });
    
    // Wait for either task to complete
    tokio::select! {
        _ = recv_handle => {},
        _ = send_handle => {},
    }
    
    // Cleanup
    manager.unregister_connection(&connection_id).await;
}

/// Handle incoming WebSocket messages
async fn handle_incoming_message(manager: &WebSocketManager, connection_id: &str, message: WsMessage) {
    match message {
        WsMessage::Auth { token } => {
            // In production, validate the token and extract user_id
            // For now, we'll use a placeholder
            let user_id = Uuid::new_v4().to_string();
            if let Err(e) = manager.authenticate(connection_id, user_id).await {
                error!("Authentication failed: {}", e);
            }
        }
        WsMessage::Subscribe { channel } => {
            if let Err(e) = manager.subscribe(connection_id, &channel).await {
                error!("Subscribe failed: {}", e);
            }
        }
        WsMessage::Unsubscribe { channel } => {
            if let Err(e) = manager.unsubscribe(connection_id, &channel).await {
                error!("Unsubscribe failed: {}", e);
            }
        }
        WsMessage::Send { channel, payload } => {
            manager.broadcast_to_channel(&channel, WsMessage::Receive {
                channel,
                payload,
                timestamp: chrono::Utc::now().timestamp(),
            }).await;
        }
        _ => {
            warn!("Unhandled message type: {:?}", message);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_websocket_manager_creation() {
        let manager = WebSocketManager::new();
        assert_eq!(*manager.connection_count.read().await, 0);
    }
    
    #[tokio::test]
    async fn test_connection_registration() {
        let manager = WebSocketManager::new();
        manager.register_connection("test_conn".to_string()).await;
        
        assert_eq!(*manager.connection_count.read().await, 1);
        
        manager.unregister_connection("test_conn").await;
        assert_eq!(*manager.connection_count.read().await, 0);
    }
    
    #[tokio::test]
    async fn test_subscription() {
        let manager = WebSocketManager::new();
        manager.register_connection("test_conn".to_string()).await;
        
        manager.subscribe("test_conn", "test_channel").await.unwrap();
        
        {
            let connections = manager.connections.read().await;
            let state = connections.get("test_conn").unwrap();
            assert!(state.channels.contains(&"test_channel".to_string()));
        }
        
        manager.unsubscribe("test_conn", "test_channel").await.unwrap();
        
        {
            let connections = manager.connections.read().await;
            let state = connections.get("test_conn").unwrap();
            assert!(!state.channels.contains(&"test_channel".to_string()));
        }
        
        manager.unregister_connection("test_conn").await;
    }
}
