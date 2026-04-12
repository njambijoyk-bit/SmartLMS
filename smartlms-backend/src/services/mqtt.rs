//! MQTT Integration for IoT Devices
//! 
//! This module provides MQTT client functionality for real-time IoT device communication,
//! supporting pub/sub patterns for sensor data, commands, and alerts.

use rumqttc::{AsyncClient, MqttOptions, QoS, Packet, Incoming};
use tokio::sync::broadcast;
use std::time::Duration;
use tracing::{info, warn, error};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// MQTT message payload for sensor data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MqttSensorData {
    pub device_id: Uuid,
    pub timestamp: i64,
    pub sensor_type: String,
    pub value: f64,
    pub unit: String,
    pub metadata: Option<serde_json::Value>,
}

/// MQTT message payload for device commands
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MqttCommand {
    pub device_id: Uuid,
    pub command_type: String,
    pub parameters: Option<serde_json::Value>,
    pub correlation_id: Option<String>,
}

/// MQTT message payload for alerts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MqttAlert {
    pub device_id: Uuid,
    pub alert_type: String,
    pub severity: String,
    pub message: String,
    pub timestamp: i64,
    pub data: Option<serde_json::Value>,
}

/// MQTT Topic configuration
pub struct MqttTopics {
    pub base_topic: String,
}

impl MqttTopics {
    pub fn new(base_topic: &str) -> Self {
        Self {
            base_topic: base_topic.to_string(),
        }
    }

    pub fn sensor_data(&self, device_id: Uuid) -> String {
        format!("{}/sensors/{}", self.base_topic, device_id)
    }

    pub fn command(&self, device_id: Uuid) -> String {
        format!("{}/commands/{}", self.base_topic, device_id)
    }

    pub fn alert(&self, device_id: Uuid) -> String {
        format!("{}/alerts/{}", self.base_topic, device_id)
    }

    pub fn status(&self, device_id: Uuid) -> String {
        format!("{}/status/{}", self.base_topic, device_id)
    }

    pub fn all_sensors(&self) -> String {
        format!("{}/sensors/+", self.base_topic)
    }

    pub fn all_commands(&self) -> String {
        format!("{}/commands/+", self.base_topic)
    }

    pub fn all_alerts(&self) -> String {
        format!("{}/alerts/+", self.base_topic)
    }
}

/// MQTT Client Manager for SmartLMS IoT
pub struct MqttClientManager {
    client: AsyncClient,
    topics: MqttTopics,
    alert_tx: broadcast::Sender<MqttAlert>,
}

impl MqttClientManager {
    /// Create a new MQTT client manager
    pub async fn new(
        broker_url: &str,
        port: u16,
        client_id: &str,
        username: Option<&str>,
        password: Option<&str>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let mut mqtt_options = MqttOptions::new(client_id, broker_url, port);
        mqtt_options.set_keep_alive(Duration::from_secs(60));
        mqtt_options.set_clean_session(true);

        if let Some(user) = username {
            mqtt_options.set_credentials(user, password.unwrap_or(""))?;
        }

        let (client, mut eventloop) = AsyncClient::new(mqtt_options, 10);
        let topics = MqttTopics::new("smartlms/iot");
        let (alert_tx, _) = broadcast::channel(1000);

        // Spawn task to handle incoming messages
        let alert_tx_clone = alert_tx.clone();
        tokio::spawn(async move {
            loop {
                match eventloop.poll().await {
                    Ok(Packet::Publish(pub_msg)) => {
                        info!("Received MQTT message on topic: {}", pub_msg.topic);
                        
                        // Try to parse as alert
                        if let Ok(alert) = serde_json::from_slice::<MqttAlert>(&pub_msg.payload) {
                            let _ = alert_tx_clone.send(alert);
                        }
                    }
                    Ok(_) => {}
                    Err(e) => {
                        error!("MQTT event loop error: {}", e);
                        tokio::time::sleep(Duration::from_secs(5)).await;
                    }
                }
            }
        });

        Ok(Self {
            client,
            topics,
            alert_tx,
        })
    }

    /// Subscribe to all sensor data topics
    pub async fn subscribe_to_sensors(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.client
            .subscribe(self.topics.all_sensors(), QoS::AtLeastOnce)
            .await?;
        info!("Subscribed to sensor data topic: {}", self.topics.all_sensors());
        Ok(())
    }

    /// Subscribe to command topics for a specific device
    pub async fn subscribe_to_device_commands(
        &self,
        device_id: Uuid,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.client
            .subscribe(self.topics.command(device_id), QoS::AtLeastOnce)
            .await?;
        info!("Subscribed to commands for device: {}", device_id);
        Ok(())
    }

    /// Publish sensor data
    pub async fn publish_sensor_data(
        &self,
        device_id: Uuid,
        sensor_type: &str,
        value: f64,
        unit: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let payload = MqttSensorData {
            device_id,
            timestamp: chrono::Utc::now().timestamp(),
            sensor_type: sensor_type.to_string(),
            value,
            unit: unit.to_string(),
            metadata: None,
        };

        let payload_bytes = serde_json::to_vec(&payload)?;
        self.client
            .publish(
                self.topics.sensor_data(device_id),
                QoS::AtLeastOnce,
                false,
                payload_bytes,
            )
            .await?;

        Ok(())
    }

    /// Send command to device
    pub async fn send_command(
        &self,
        device_id: Uuid,
        command_type: &str,
        parameters: Option<serde_json::Value>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let correlation_id = Uuid::new_v4().to_string();
        let payload = MqttCommand {
            device_id,
            command_type: command_type.to_string(),
            parameters,
            correlation_id: Some(correlation_id.clone()),
        };

        let payload_bytes = serde_json::to_vec(&payload)?;
        self.client
            .publish(
                self.topics.command(device_id),
                QoS::AtLeastOnce,
                false,
                payload_bytes,
            )
            .await?;

        Ok(correlation_id)
    }

    /// Broadcast alert
    pub async fn publish_alert(
        &self,
        device_id: Uuid,
        alert_type: &str,
        severity: &str,
        message: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let payload = MqttAlert {
            device_id,
            alert_type: alert_type.to_string(),
            severity: severity.to_string(),
            message: message.to_string(),
            timestamp: chrono::Utc::now().timestamp(),
            data: None,
        };

        let payload_bytes = serde_json::to_vec(&payload)?;
        self.client
            .publish(
                self.topics.alert(device_id),
                QoS::AtLeastOnce,
                false,
                payload_bytes,
            )
            .await?;

        // Also broadcast internally
        let _ = self.alert_tx.send(payload);

        Ok(())
    }

    /// Get alert receiver for internal processing
    pub fn get_alert_receiver(&self) -> broadcast::Receiver<MqttAlert> {
        self.alert_tx.subscribe()
    }

    /// Publish device status update
    pub async fn publish_status(
        &self,
        device_id: Uuid,
        status: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let payload = serde_json::json!({
            "device_id": device_id,
            "status": status,
            "timestamp": chrono::Utc::now().timestamp()
        });

        let payload_bytes = serde_json::to_vec(&payload)?;
        self.client
            .publish(
                self.topics.status(device_id),
                QoS::AtLeastOnce,
                false,
                payload_bytes,
            )
            .await?;

        Ok(())
    }
}

/// Initialize MQTT connection from environment variables
pub async fn init_mqtt_client() -> Result<MqttClientManager, Box<dyn std::error::Error>> {
    let broker_url = std::env::var("MQTT_BROKER_URL").unwrap_or_else(|_| "localhost".to_string());
    let port: u16 = std::env::var("MQTT_BROKER_PORT")
        .unwrap_or_else(|_| "1883".to_string())
        .parse()?;
    let client_id = std::env::var("MQTT_CLIENT_ID")
        .unwrap_or_else(|_| format!("smartlms_{}", Uuid::new_v4()));
    let username = std::env::var("MQTT_USERNAME").ok();
    let password = std::env::var("MQTT_PASSWORD").ok();

    MqttClientManager::new(&broker_url, port, &client_id, username.as_deref(), password.as_deref())
        .await
}
