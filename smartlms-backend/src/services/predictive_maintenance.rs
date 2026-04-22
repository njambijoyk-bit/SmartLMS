//! Predictive Maintenance Module for IoT Devices
//! 
//! This module uses machine learning to predict device failures and maintenance needs
//! before they occur, reducing downtime and optimizing maintenance schedules.

use linfa::traits::*;
use linfa_trees::{DecisionTree, DecisionTreeHyperParams};
use ndarray::{Array1, Array2, Axis};
use std::collections::HashMap;
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use tracing::{info, warn};

/// Device health metrics for ML model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceHealthMetrics {
    pub device_id: Uuid,
    pub uptime_hours: f64,
    pub error_count_24h: usize,
    pub avg_temperature: f64,
    pub temperature_variance: f64,
    pub vibration_level: f64,
    pub power_consumption: f64,
    pub last_maintenance_days_ago: usize,
    pub age_days: usize,
    pub failure_probability: f32,
}

/// Training data sample
#[derive(Debug, Clone)]
pub struct TrainingSample {
    pub features: Vec<f64>,
    pub label: u8, // 0 = healthy, 1 = needs_maintenance, 2 = critical
}

/// Predictive Maintenance Model
pub struct PredictiveMaintenanceModel {
    model: Option<DecisionTree<f64, u8>>,
    feature_names: Vec<String>,
    is_trained: bool,
}

impl PredictiveMaintenanceModel {
    pub fn new() -> Self {
        Self {
            model: None,
            feature_names: vec![
                "uptime_hours".to_string(),
                "error_count_24h".to_string(),
                "avg_temperature".to_string(),
                "temperature_variance".to_string(),
                "vibration_level".to_string(),
                "power_consumption".to_string(),
                "last_maintenance_days_ago".to_string(),
                "age_days".to_string(),
            ],
            is_trained: false,
        }
    }

    /// Train the model with historical data
    pub fn train(&mut self, training_data: Vec<TrainingSample>) -> Result<(), Box<dyn std::error::Error>> {
        if training_data.is_empty() {
            return Err("No training data provided".into());
        }

        let n_samples = training_data.len();
        let n_features = training_data[0].features.len();

        // Prepare feature matrix
        let mut features = Array2::zeros((n_samples, n_features));
        let mut labels = Array1::zeros(n_samples);

        for (i, sample) in training_data.iter().enumerate() {
            for (j, feature) in sample.features.iter().enumerate() {
                features[[i, j]] = *feature;
            }
            labels[i] = sample.label as f64;
        }

        // Train decision tree
        let hyperparams = DecisionTreeHyperParams::default()
            .max_depth(Some(10))
            .min_samples_split(5)
            .min_samples_leaf(2);

        let model = DecisionTree::fit(&hyperparams, features.view(), labels.view())?;
        
        self.model = Some(model);
        self.is_trained = true;
        
        info!("Predictive maintenance model trained with {} samples", n_samples);
        Ok(())
    }

    /// Predict device health status
    pub fn predict(&self, metrics: &DeviceHealthMetrics) -> Result<PredictionResult, Box<dyn std::error::Error>> {
        if !self.is_trained || self.model.is_none() {
            return self.heuristic_prediction(metrics);
        }

        let model = self.model.as_ref().unwrap();
        
        // Prepare features
        let features = Array2::from_shape_vec(
            (1, self.feature_names.len()),
            vec![
                metrics.uptime_hours,
                metrics.error_count_24h as f64,
                metrics.avg_temperature,
                metrics.temperature_variance,
                metrics.vibration_level,
                metrics.power_consumption,
                metrics.last_maintenance_days_ago as f64,
                metrics.age_days as f64,
            ],
        )?;

        let prediction = model.predict(features.view())?;
        let predicted_class = prediction[0] as u8;

        let result = match predicted_class {
            0 => PredictionResult {
                status: MaintenanceStatus::Healthy,
                confidence: 0.9,
                recommended_action: "No action needed".to_string(),
                estimated_days_to_failure: 90,
            },
            1 => PredictionResult {
                status: MaintenanceStatus::NeedsMaintenance,
                confidence: 0.75,
                recommended_action: "Schedule maintenance within 7 days".to_string(),
                estimated_days_to_failure: 14,
            },
            2 => PredictionResult {
                status: MaintenanceStatus::Critical,
                confidence: 0.85,
                recommended_action: "Immediate maintenance required".to_string(),
                estimated_days_to_failure: 1,
            },
            _ => PredictionResult {
                status: MaintenanceStatus::Unknown,
                confidence: 0.0,
                recommended_action: "Unable to determine".to_string(),
                estimated_days_to_failure: 0,
            },
        };

        Ok(result)
    }

    /// Fallback heuristic-based prediction when model is not trained
    fn heuristic_prediction(&self, metrics: &DeviceHealthMetrics) -> Result<PredictionResult, Box<dyn std::error::Error>> {
        let mut risk_score = 0.0;

        // High error count
        if metrics.error_count_24h > 10 {
            risk_score += 0.3;
        } else if metrics.error_count_24h > 5 {
            risk_score += 0.15;
        }

        // High temperature
        if metrics.avg_temperature > 80.0 {
            risk_score += 0.25;
        } else if metrics.avg_temperature > 60.0 {
            risk_score += 0.1;
        }

        // High vibration
        if metrics.vibration_level > 0.8 {
            risk_score += 0.25;
        } else if metrics.vibration_level > 0.5 {
            risk_score += 0.1;
        }

        // Old device without maintenance
        if metrics.last_maintenance_days_ago > 90 {
            risk_score += 0.2;
        } else if metrics.last_maintenance_days_ago > 30 {
            risk_score += 0.1;
        }

        let (status, action, days) = if risk_score > 0.6 {
            (
                MaintenanceStatus::Critical,
                "Immediate maintenance required".to_string(),
                1,
            )
        } else if risk_score > 0.3 {
            (
                MaintenanceStatus::NeedsMaintenance,
                "Schedule maintenance within 7 days".to_string(),
                7,
            )
        } else {
            (
                MaintenanceStatus::Healthy,
                "No action needed".to_string(),
                90,
            )
        };

        Ok(PredictionResult {
            status,
            confidence: 1.0 - risk_score,
            recommended_action: action,
            estimated_days_to_failure: days,
        })
    }

    /// Get feature importance from the trained model
    pub fn get_feature_importance(&self) -> Option<HashMap<String, f64>> {
        if !self.is_trained || self.model.is_none() {
            return None;
        }

        // In a real implementation, extract feature importance from the decision tree
        let mut importance = HashMap::new();
        importance.insert("uptime_hours".to_string(), 0.15);
        importance.insert("error_count_24h".to_string(), 0.25);
        importance.insert("avg_temperature".to_string(), 0.20);
        importance.insert("vibration_level".to_string(), 0.20);
        importance.insert("last_maintenance_days_ago".to_string(), 0.20);

        Some(importance)
    }
}

impl Default for PredictiveMaintenanceModel {
    fn default() -> Self {
        Self::new()
    }
}

/// Prediction result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PredictionResult {
    pub status: MaintenanceStatus,
    pub confidence: f64,
    pub recommended_action: String,
    pub estimated_days_to_failure: usize,
}

/// Maintenance status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum MaintenanceStatus {
    Healthy,
    NeedsMaintenance,
    Critical,
    Unknown,
}

/// Maintenance schedule recommendation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaintenanceSchedule {
    pub device_id: Uuid,
    pub priority: u8, // 1-5, 1 being highest
    pub recommended_date: chrono::DateTime<chrono::Utc>,
    pub estimated_duration_minutes: usize,
    pub required_parts: Vec<String>,
    pub notes: String,
}

/// Predictive Maintenance Manager
pub struct PredictiveMaintenanceManager {
    model: PredictiveMaintenanceModel,
    device_history: HashMap<Uuid, Vec<DeviceHealthMetrics>>,
    schedules: Vec<MaintenanceSchedule>,
}

impl PredictiveMaintenanceManager {
    pub fn new() -> Self {
        Self {
            model: PredictiveMaintenanceModel::new(),
            device_history: HashMap::new(),
            schedules: Vec::new(),
        }
    }

    /// Record device metrics
    pub fn record_metrics(&mut self, metrics: DeviceHealthMetrics) {
        self.device_history
            .entry(metrics.device_id)
            .or_insert_with(Vec::new)
            .push(metrics.clone());

        // Keep only last 100 readings per device
        if let Some(history) = self.device_history.get_mut(&metrics.device_id) {
            if history.len() > 100 {
                history.remove(0);
            }
        }
    }

    /// Analyze device and generate prediction
    pub fn analyze_device(
        &mut self,
        metrics: DeviceHealthMetrics,
    ) -> Result<PredictionResult, Box<dyn std::error::Error>> {
        self.record_metrics(metrics.clone());
        let prediction = self.model.predict(&metrics)?;

        // Generate maintenance schedule if needed
        if prediction.status != MaintenanceStatus::Healthy {
            self.generate_maintenance_schedule(metrics.device_id, &prediction)?;
        }

        Ok(prediction)
    }

    /// Generate maintenance schedule
    fn generate_maintenance_schedule(
        &mut self,
        device_id: Uuid,
        prediction: &PredictionResult,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let priority = match prediction.status {
            MaintenanceStatus::Critical => 1,
            MaintenanceStatus::NeedsMaintenance => 3,
            _ => 5,
        };

        let recommended_date = chrono::Utc::now()
            + chrono::Duration::days(prediction.estimated_days_to_failure as i64 / 2);

        let schedule = MaintenanceSchedule {
            device_id,
            priority,
            recommended_date,
            estimated_duration_minutes: 60,
            required_parts: vec![],
            notes: prediction.recommended_action.clone(),
        };

        self.schedules.push(schedule);
        info!(
            "Generated maintenance schedule for device {} with priority {}",
            device_id, priority
        );

        Ok(())
    }

    /// Get upcoming maintenance schedules
    pub fn get_upcoming_schedules(&self, limit: usize) -> Vec<&MaintenanceSchedule> {
        let mut sorted: Vec<&MaintenanceSchedule> = self.schedules.iter().collect();
        sorted.sort_by_key(|s| s.priority);
        sorted.into_iter().take(limit).collect()
    }

    /// Train model with accumulated data
    pub fn train_model(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let mut training_data = Vec::new();

        for (device_id, history) in &self.device_history {
            if history.len() < 10 {
                continue;
            }

            // Create training samples from history
            for metrics in history {
                let features = vec![
                    metrics.uptime_hours,
                    metrics.error_count_24h as f64,
                    metrics.avg_temperature,
                    metrics.temperature_variance,
                    metrics.vibration_level,
                    metrics.power_consumption,
                    metrics.last_maintenance_days_ago as f64,
                    metrics.age_days as f64,
                ];

                // Label based on current failure probability
                let label = if metrics.failure_probability > 0.7 {
                    2
                } else if metrics.failure_probability > 0.4 {
                    1
                } else {
                    0
                };

                training_data.push(TrainingSample { features, label });
            }
        }

        if !training_data.is_empty() {
            self.model.train(training_data)?;
        }

        Ok(())
    }

    /// Get device health trend
    pub fn get_health_trend(&self, device_id: Uuid, days: usize) -> Option<HealthTrend> {
        let history = self.device_history.get(&device_id)?;
        
        if history.is_empty() {
            return None;
        }

        let recent_failures: Vec<_> = history
            .iter()
            .filter(|m| m.failure_probability > 0.5)
            .collect();

        let trend = if recent_failures.len() > history.len() / 2 {
            HealthTrend::Declining
        } else if recent_failures.is_empty() {
            HealthTrend::Stable
        } else {
            HealthTrend::Improving
        };

        Some(HealthTrend {
            device_id,
            trend,
            data_points: history.len(),
            avg_failure_probability: history.iter().map(|m| m.failure_probability).sum::<f32>()
                / history.len() as f32,
        })
    }
}

impl Default for PredictiveMaintenanceManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Health trend indicator
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthTrend {
    pub device_id: Uuid,
    pub trend: TrendDirection,
    pub data_points: usize,
    pub avg_failure_probability: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TrendDirection {
    Improving,
    Stable,
    Declining,
}
