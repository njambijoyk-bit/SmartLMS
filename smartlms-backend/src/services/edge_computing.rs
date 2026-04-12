//! Edge Computing Module for IoT
//! 
//! This module provides edge computing capabilities for processing IoT data
//! closer to the source, reducing latency and bandwidth usage.

use tokio::sync::mpsc;
use std::collections::HashMap;
use std::time::Duration;
use tracing::{info, warn, error};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use flume::{Sender, Receiver, async_channel};

/// Edge computing node configuration
#[derive(Debug, Clone)]
pub struct EdgeNodeConfig {
    pub node_id: String,
    pub location: String,
    pub max_buffer_size: usize,
    pub processing_interval_ms: u64,
    pub enabled_rules: Vec<String>,
}

/// Data batch for edge processing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgeDataBatch {
    pub node_id: String,
    pub device_id: Uuid,
    pub timestamp: i64,
    pub readings: Vec<EdgeSensorReading>,
    pub aggregated: Option<EdgeAggregatedData>,
}

/// Single sensor reading at the edge
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgeSensorReading {
    pub sensor_type: String,
    pub value: f64,
    pub unit: String,
    pub quality: f32,
}

/// Aggregated data computed at the edge
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgeAggregatedData {
    pub count: usize,
    pub min: f64,
    pub max: f64,
    pub avg: f64,
    pub sum: f64,
    pub stddev: Option<f64>,
}

/// Edge processing rule
#[derive(Debug, Clone)]
pub struct EdgeRule {
    pub rule_id: String,
    pub rule_type: EdgeRuleType,
    pub threshold: f64,
    pub action: EdgeAction,
    pub enabled: bool,
}

/// Types of edge rules
#[derive(Debug, Clone)]
pub enum EdgeRuleType {
    ThresholdAbove,
    ThresholdBelow,
    RateOfChange,
    Anomaly,
    Pattern,
}

/// Actions to take when rule triggers
#[derive(Debug, Clone)]
pub enum EdgeAction {
    Alert(String),
    Command(String, serde_json::Value),
    Filter,
    Aggregate,
    Forward,
}

/// Edge Computing Processor
pub struct EdgeProcessor {
    config: EdgeNodeConfig,
    rules: HashMap<String, EdgeRule>,
    data_tx: Sender<EdgeDataBatch>,
    data_rx: Receiver<EdgeDataBatch>,
    alert_tx: mpsc::Sender<crate::services::mqtt::MqttAlert>,
}

impl EdgeProcessor {
    /// Create a new edge processor
    pub fn new(
        config: EdgeNodeConfig,
        alert_tx: mpsc::Sender<crate::services::mqtt::MqttAlert>,
    ) -> Self {
        let (data_tx, data_rx) = async_channel::bounded(config.max_buffer_size);
        
        Self {
            config,
            rules: HashMap::new(),
            data_tx,
            data_rx,
            alert_tx,
        }
    }

    /// Add a processing rule
    pub fn add_rule(&mut self, rule: EdgeRule) {
        if self.config.enabled_rules.contains(&rule.rule_id) {
            self.rules.insert(rule.rule_id.clone(), rule);
            info!("Added edge rule: {}", rule.rule_id);
        }
    }

    /// Remove a processing rule
    pub fn remove_rule(&mut self, rule_id: &str) {
        self.rules.remove(rule_id);
        info!("Removed edge rule: {}", rule_id);
    }

    /// Process incoming sensor data at the edge
    pub async fn process_reading(
        &self,
        device_id: Uuid,
        sensor_type: &str,
        value: f64,
        unit: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let reading = EdgeSensorReading {
            sensor_type: sensor_type.to_string(),
            value,
            unit: unit.to_string(),
            quality: 1.0,
        };

        // Apply rules
        for (_, rule) in &self.rules {
            if !rule.enabled {
                continue;
            }

            let should_trigger = match rule.rule_type {
                EdgeRuleType::ThresholdAbove => value > rule.threshold,
                EdgeRuleType::ThresholdBelow => value < rule.threshold,
                _ => false, // Other rules require more context
            };

            if should_trigger {
                self.execute_action(&rule.action, device_id, sensor_type, value).await?;
            }
        }

        // Send batch for aggregation
        let batch = EdgeDataBatch {
            node_id: self.config.node_id.clone(),
            device_id,
            timestamp: chrono::Utc::now().timestamp(),
            readings: vec![reading],
            aggregated: None,
        };

        let _ = self.data_tx.send_async(batch).await;

        Ok(())
    }

    /// Execute action based on rule trigger
    async fn execute_action(
        &self,
        action: &EdgeAction,
        device_id: Uuid,
        sensor_type: &str,
        value: f64,
    ) -> Result<(), Box<dyn std::error::Error>> {
        match action {
            EdgeAction::Alert(message) => {
                let alert = crate::services::mqtt::MqttAlert {
                    device_id,
                    alert_type: "edge_rule_trigger".to_string(),
                    severity: "warning".to_string(),
                    message: format!("{}: {} = {}", message, sensor_type, value),
                    timestamp: chrono::Utc::now().timestamp(),
                    data: Some(serde_json::json!({
                        "sensor_type": sensor_type,
                        "value": value,
                        "node_id": self.config.node_id
                    })),
                };
                let _ = self.alert_tx.send(alert).await;
                info!("Edge alert triggered for device {}: {}", device_id, message);
            }
            EdgeAction::Command(cmd_type, params) => {
                // In a real implementation, this would send an MQTT command
                info!("Edge command triggered: {} for device {}", cmd_type, device_id);
            }
            EdgeAction::Filter => {
                // Filter out this reading (don't forward to cloud)
                info!("Filtered reading at edge for device {}", device_id);
            }
            EdgeAction::Aggregate | EdgeAction::Forward => {
                // Forward to cloud (default behavior)
            }
        }

        Ok(())
    }

    /// Aggregate data locally before sending to cloud
    pub async fn aggregate_and_forward(
        &self,
        device_id: Uuid,
        window_size: usize,
    ) -> Result<EdgeAggregatedData, Box<dyn std::error::Error>> {
        let mut values = Vec::new();
        
        // Collect readings for aggregation
        for _ in 0..window_size {
            if let Ok(batch) = self.data_rx.recv_async().await {
                for reading in batch.readings {
                    values.push(reading.value);
                }
            }
        }

        if values.is_empty() {
            return Err("No data to aggregate".into());
        }

        let count = values.len();
        let min = *values.iter().min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
        let max = *values.iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
        let sum: f64 = values.iter().sum();
        let avg = sum / count as f64;

        // Calculate standard deviation
        let variance = values.iter().map(|x| (x - avg).powi(2)).sum::<f64>() / count as f64;
        let stddev = Some(variance.sqrt());

        let aggregated = EdgeAggregatedData {
            count,
            min,
            max,
            avg,
            sum,
            stddev,
        };

        Ok(aggregated)
    }

    /// Get data receiver for cloud forwarding
    pub fn get_data_receiver(&self) -> Receiver<EdgeDataBatch> {
        self.data_rx.clone()
    }
}

/// Edge Computing Manager for multiple nodes
pub struct EdgeComputingManager {
    nodes: HashMap<String, EdgeProcessor>,
}

impl EdgeComputingManager {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
        }
    }

    pub fn register_node(
        &mut self,
        config: EdgeNodeConfig,
        alert_tx: mpsc::Sender<crate::services::mqtt::MqttAlert>,
    ) {
        let processor = EdgeProcessor::new(config.clone(), alert_tx);
        self.nodes.insert(config.node_id, processor);
        info!("Registered edge node: {}", config.node_id);
    }

    pub fn get_node(&self, node_id: &str) -> Option<&EdgeProcessor> {
        self.nodes.get(node_id)
    }

    pub fn get_node_mut(&mut self, node_id: &str) -> Option<&mut EdgeProcessor> {
        self.nodes.get_mut(node_id)
    }
}

impl Default for EdgeComputingManager {
    fn default() -> Self {
        Self::new()
    }
}
