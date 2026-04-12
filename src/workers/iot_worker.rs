//! IoT Telemetry Processing & Automation Worker
//! Processes incoming telemetry, triggers automation rules, and monitors device health

use std::time::Duration;
use tokio::time::interval;
use sqlx::{PgPool, Row};
use uuid::Uuid;
use serde_json::Value;

pub struct IotWorker {
    pool: PgPool,
}

impl IotWorker {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Start the IoT worker background tasks
    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut telemetry_interval = interval(Duration::from_secs(5));
        let mut automation_interval = interval(Duration::from_secs(10));
        let mut health_interval = interval(Duration::from_secs(60));

        loop {
            tokio::select! {
                _ = telemetry_interval.tick() => {
                    if let Err(e) = self.process_pending_telemetry().await {
                        log::error!("Failed to process telemetry: {}", e);
                    }
                }
                _ = automation_interval.tick() => {
                    if let Err(e) = self.evaluate_automation_rules().await {
                        log::error!("Failed to evaluate automation rules: {}", e);
                    }
                }
                _ = health_interval.tick() => {
                    if let Err(e) = self.check_device_health().await {
                        log::error!("Failed to check device health: {}", e);
                    }
                }
            }
        }
    }

    /// Process pending telemetry data (aggregation, alerts, etc.)
    async fn process_pending_telemetry(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut conn = self.pool.acquire().await?;

        // Get unprocessed telemetry
        let rows = sqlx::query(
            r#"
            SELECT id, device_id, metric_name, metric_value, timestamp
            FROM iot_telemetry
            WHERE processed = FALSE
            LIMIT 1000
            "#
        )
        .fetch_all(&mut *conn)
        .await?;

        for row in rows {
            let id: i64 = row.get(0);
            let device_id: Uuid = row.get(1);
            let metric_name: String = row.get(2);
            let metric_value: Option<f64> = row.get(3);

            // Check for threshold alerts
            self.check_threshold_alerts(device_id, &metric_name, metric_value).await?;

            // Mark as processed
            sqlx::query("UPDATE iot_telemetry SET processed = TRUE WHERE id = $1")
                .bind(id)
                .execute(&mut *conn)
                .await?;
        }

        Ok(())
    }

    /// Check if telemetry values exceed configured thresholds
    async fn check_threshold_alerts(
        &self,
        device_id: Uuid,
        metric_name: &str,
        value: Option<f64>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut conn = self.pool.acquire().await?;

        // Get device info
        let device_info: Option<(Uuid, Option<Uuid>)> = sqlx::query_as(
            "SELECT id, room_id FROM iot_devices WHERE id = $1"
        )
        .bind(device_id)
        .fetch_optional(&mut *conn)
        .await?;

        if let Some((_, Some(room_id))) = device_info {
            // Check for safety thresholds (e.g., high CO2, temperature)
            if metric_name == "co2" && value.unwrap_or(0.0) > 1000.0 {
                self.create_safety_incident(
                    room_id,
                    device_id,
                    "high_co2",
                    "medium",
                    format!("CO2 level exceeded 1000ppm: {:.0}ppm", value.unwrap_or(0.0)),
                )
                .await?;
            }

            if metric_name == "temp" && value.unwrap_or(0.0) > 35.0 {
                self.create_safety_incident(
                    room_id,
                    device_id,
                    "high_temperature",
                    "high",
                    format!("Temperature exceeded 35°C: {:.1}°C", value.unwrap_or(0.0)),
                )
                .await?;
            }
        }

        Ok(())
    }

    /// Create a safety incident record
    async fn create_safety_incident(
        &self,
        lab_id: Uuid,
        device_id: Uuid,
        incident_type: &str,
        severity: &str,
        description: String,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut conn = self.pool.acquire().await?;

        // Check if similar incident already active
        let existing = sqlx::query(
            r#"
            SELECT id FROM lab_safety_incidents
            WHERE lab_id = $1 AND incident_type = $2 AND resolved_at IS NULL
            "#
        )
        .bind(lab_id)
        .bind(incident_type)
        .fetch_optional(&mut *conn)
        .await?;

        if existing.is_none() {
            sqlx::query(
                r#"
                INSERT INTO lab_safety_incidents (lab_id, device_id, incident_type, severity, description)
                VALUES ($1, $2, $3, $4, $5)
                "#
            )
            .bind(lab_id)
            .bind(device_id)
            .bind(incident_type)
            .bind(severity)
            .bind(&description)
            .execute(&mut *conn)
            .await?;

            log::warn!("Safety incident created: {} - {}", incident_type, description);

            // TODO: Send notifications to relevant personnel
        }

        Ok(())
    }

    /// Evaluate smart classroom automation rules
    async fn evaluate_automation_rules(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut conn = self.pool.acquire().await?;

        // Get active rules
        let rules = sqlx::query(
            r#"
            SELECT id, room_id, trigger_type, trigger_config, action_type, action_config
            FROM smart_classroom_rules
            WHERE is_active = TRUE
            "#
        )
        .fetch_all(&mut *conn)
        .await?;

        for rule in rules {
            let rule_id: Uuid = rule.get(0);
            let room_id: Uuid = rule.get(1);
            let trigger_type: String = rule.get(2);
            let trigger_config: Value = rule.get(3);
            let action_type: String = rule.get(4);
            let action_config: Value = rule.get(5);

            let should_trigger = match trigger_type.as_str() {
                "motion_detected" => self.check_motion_trigger(room_id, &trigger_config).await?,
                "schedule_start" => self.check_schedule_trigger(&trigger_config).await?,
                "occupancy_threshold" => self.check_occupancy_trigger(room_id, &trigger_config).await?,
                _ => false,
            };

            if should_trigger {
                self.execute_action(action_type, action_config, room_id).await?;
                
                // Log rule execution
                log::info!("Automation rule {} triggered in room {}", rule_id, room_id);
            }
        }

        Ok(())
    }

    /// Check if motion was detected recently
    async fn check_motion_trigger(
        &self,
        room_id: Uuid,
        config: &Value,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut conn = self.pool.acquire().await?;

        // Get device IDs from config
        let device_ids = config["device_ids"]
            .as_array()
            .map(|arr| arr.iter().filter_map(|v| v.as_str()).collect::<Vec<_>>())
            .unwrap_or_default();

        if device_ids.is_empty() {
            return Ok(false);
        }

        // Check for recent motion
        let count: i64 = sqlx::query_scalar(
            r#"
            SELECT COUNT(*) FROM iot_telemetry t
            JOIN iot_devices d ON t.device_id = d.id
            WHERE d.room_id = $1 
            AND t.metric_name = 'motion'
            AND t.metric_value = 1.0
            AND t.timestamp > NOW() - INTERVAL '1 minute'
            "#
        )
        .bind(room_id)
        .fetch_one(&mut *conn)
        .await?;

        Ok(count > 0)
    }

    /// Check if current time matches schedule
    async fn check_schedule_trigger(
        &self,
        config: &Value,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        // Parse schedule from config
        let start_time = config["start_time"].as_str().unwrap_or("08:00");
        let end_time = config["end_time"].as_str().unwrap_or("18:00");
        
        let now = chrono::Local::now();
        let current_time = now.format("%H:%M").to_string();

        Ok(current_time >= start_time.to_string() && current_time <= end_time.to_string())
    }

    /// Check occupancy count threshold
    async fn check_occupancy_trigger(
        &self,
        room_id: Uuid,
        config: &Value,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut conn = self.pool.acquire().await?;
        let threshold = config["threshold"].as_i64().unwrap_or(1);

        let latest_count: Option<i64> = sqlx::query_scalar(
            r#"
            SELECT t.metric_value::bigint
            FROM iot_telemetry t
            JOIN iot_devices d ON t.device_id = d.id
            WHERE d.room_id = $1 
            AND t.metric_name = 'occupancy_count'
            ORDER BY t.timestamp DESC
            LIMIT 1
            "#
        )
        .bind(room_id)
        .fetch_optional(&mut *conn)
        .await?;

        Ok(latest_count.unwrap_or(0) >= threshold)
    }

    /// Execute automation action
    async fn execute_action(
        &self,
        action_type: String,
        config: Value,
        room_id: Uuid,
    ) -> Result<(), Box<dyn std::error::Error>> {
        match action_type.as_str() {
            "start_recording" => {
                self.start_classroom_recording(room_id, config).await?;
            }
            "turn_on_lights" | "turn_off_lights" => {
                self.control_lights(room_id, action_type == "turn_on_lights", config).await?;
            }
            "adjust_ac" => {
                self.adjust_hvac(room_id, config).await?;
            }
            "send_notification" => {
                self.send_automation_notification(room_id, config).await?;
            }
            _ => {
                log::warn!("Unknown action type: {}", action_type);
            }
        }

        Ok(())
    }

    /// Start automatic classroom recording session
    async fn start_classroom_recording(
        &self,
        room_id: Uuid,
        config: Value,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut conn = self.pool.acquire().await?;

        // Check if session already active
        let active = sqlx::query(
            "SELECT id FROM smart_classroom_sessions WHERE room_id = $1 AND status = 'active'"
        )
        .bind(room_id)
        .fetch_optional(&mut *conn)
        .await?;

        if active.is_none() {
            // Get current course offering if class is scheduled
            let course_offering_id: Option<Uuid> = sqlx::query_scalar(
                r#"
                SELECT co.id FROM course_offerings co
                JOIN rooms r ON co.location_id = r.location_id
                WHERE r.id = $1 
                AND NOW() BETWEEN co.start_date AND co.end_date
                LIMIT 1
                "#
            )
            .bind(room_id)
            .fetch_optional(&mut *conn)
            .await?;

            sqlx::query(
                r#"
                INSERT INTO smart_classroom_sessions (room_id, course_offering_id, status)
                VALUES ($1, $2, 'active')
                "#
            )
            .bind(room_id)
            .bind(course_offering_id)
            .execute(&mut *conn)
            .await?;

            log::info!("Started recording session in room {}", room_id);
        }

        Ok(())
    }

    /// Control room lighting via IoT devices
    async fn control_lights(
        &self,
        room_id: Uuid,
        turn_on: bool,
        config: Value,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // In production, this would send commands to actual smart light devices
        // via MQTT or HTTP API
        log::info!(
            "Light control command for room {}: {}",
            room_id,
            if turn_on { "ON" } else { "OFF" }
        );
        Ok(())
    }

    /// Adjust HVAC settings
    async fn adjust_hvac(
        &self,
        room_id: Uuid,
        config: Value,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let target_temp = config["temperature"].as_f64().unwrap_or(22.0);
        
        log::info!("HVAC adjustment for room {} to {}°C", room_id, target_temp);
        Ok(())
    }

    /// Send notification about automation event
    async fn send_automation_notification(
        &self,
        room_id: Uuid,
        config: Value,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let message = config["message"].as_str().unwrap_or("Automation event triggered");
        
        log::info!("Notification for room {}: {}", room_id, message);
        Ok(())
    }

    /// Check device health and mark offline devices
    async fn check_device_health(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut conn = self.pool.acquire().await?;

        // Mark devices as offline if not seen in 5 minutes
        sqlx::query(
            r#"
            UPDATE iot_devices
            SET status = 'offline'
            WHERE last_seen < NOW() - INTERVAL '5 minutes'
            AND status != 'offline'
            "#
        )
        .execute(&mut *conn)
        .await?;

        // Check for low battery devices
        let low_battery: Vec<(Uuid, String, i32)> = sqlx::query_as(
            r#"
            SELECT id, name, battery_level
            FROM iot_devices
            WHERE battery_level IS NOT NULL 
            AND battery_level < 20
            AND status = 'online'
            "#
        )
        .fetch_all(&mut *conn)
        .await?;

        for (device_id, name, level) in low_battery {
            log::warn!(
                "Low battery alert: Device {} ({}) at {}%",
                device_id,
                name.unwrap_or_else(|| "unknown".to_string()),
                level
            );
            // TODO: Send maintenance notification
        }

        Ok(())
    }
}
