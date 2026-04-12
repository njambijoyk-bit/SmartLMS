//! IoT Services - Device Management and Sensor Data Processing

use crate::models::*;
use sqlx::{PgPool, FromRow};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::sync::Arc;
use tokio::sync::broadcast;

/// Service for managing IoT devices and sensor data
pub struct IotService {
    pool: PgPool,
    alert_tx: broadcast::Sender<IotAlert>,
}

impl IotService {
    /// Create a new IoT service instance
    pub fn new(pool: PgPool) -> Self {
        let (alert_tx, _) = broadcast::channel(100);
        Self { pool, alert_tx }
    }

    /// Get a clone of the alert sender for subscribing to alerts
    pub fn get_alert_sender(&self) -> broadcast::Sender<IotAlert> {
        self.alert_tx.clone()
    }

    /// Register a new IoT device
    pub async fn register_device(&self, req: RegisterDeviceRequest) -> Result<IotDevice, Box<dyn std::error::Error + Send + Sync>> {
        let id = Uuid::new_v4();
        let now = Utc::now();
        
        let device = IotDevice {
            id,
            name: req.name,
            device_type: req.device_type,
            status: DeviceStatus::Offline, // Start as offline until first heartbeat
            location: req.location,
            institution_id: req.institution_id,
            course_id: req.course_id,
            room_id: req.room_id,
            ip_address: req.ip_address,
            mac_address: req.mac_address,
            firmware_version: None,
            last_seen: None,
            metadata: req.metadata.unwrap_or(serde_json::Value::Null),
            created_at: now,
            updated_at: now,
        };

        // Insert into database
        sqlx::query_as::<_, IotDevice>(
            r#"
            INSERT INTO iot_devices 
            (id, name, device_type, status, location, institution_id, course_id, room_id, 
             ip_address, mac_address, firmware_version, last_seen, metadata, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15)
            RETURNING *
            "#
        )
        .bind(device.id)
        .bind(&device.name)
        .bind(&device.device_type)
        .bind(&device.status)
        .bind(&device.location)
        .bind(device.institution_id)
        .bind(device.course_id)
        .bind(device.room_id)
        .bind(&device.ip_address)
        .bind(&device.mac_address)
        .bind(&device.firmware_version)
        .bind(&device.last_seen)
        .bind(&device.metadata)
        .bind(device.created_at)
        .bind(device.updated_at)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)
    }

    /// Get device by ID
    pub async fn get_device(&self, device_id: Uuid) -> Result<Option<IotDevice>, Box<dyn std::error::Error + Send + Sync>> {
        let device = sqlx::query_as::<_, IotDevice>(
            "SELECT * FROM iot_devices WHERE id = $1"
        )
        .bind(device_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;

        Ok(device)
    }

    /// List devices for an institution
    pub async fn list_devices(&self, institution_id: Uuid, device_type: Option<DeviceType>, status: Option<DeviceStatus>) 
        -> Result<Vec<IotDevice>, Box<dyn std::error::Error + Send + Sync>> 
    {
        let mut query = String::from("SELECT * FROM iot_devices WHERE institution_id = $1");
        let mut params = vec![institution_id];
        let mut param_count = 1;

        if let Some(dt) = device_type {
            param_count += 1;
            query.push_str(&format!(" AND device_type = ${}", param_count));
            // Note: In production, you'd want to handle enum conversion properly
        }

        if let Some(s) = status {
            param_count += 1;
            query.push_str(&format!(" AND status = ${}", param_count));
        }

        let devices = sqlx::query_as::<_, IotDevice>(&query)
            .bind(institution_id)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;

        Ok(devices)
    }

    /// Update device status
    pub async fn update_device_status(&self, device_id: Uuid, status: DeviceStatus, firmware_version: Option<String>) 
        -> Result<IotDevice, Box<dyn std::error::Error + Send + Sync>> 
    {
        let now = Utc::now();
        
        let device = sqlx::query_as::<_, IotDevice>(
            r#"
            UPDATE iot_devices 
            SET status = $2, firmware_version = COALESCE($3, firmware_version), 
                last_seen = $4, updated_at = $5
            WHERE id = $1
            RETURNING *
            "#
        )
        .bind(device_id)
        .bind(&status)
        .bind(&firmware_version)
        .bind(now)
        .bind(now)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;

        Ok(device)
    }

    /// Submit sensor reading
    pub async fn submit_reading(&self, device_id: Uuid, req: SubmitReadingRequest) 
        -> Result<SensorReading, Box<dyn std::error::Error + Send + Sync>> 
    {
        let id = Uuid::new_v4();
        let now = Utc::now();

        let reading = SensorReading {
            id,
            device_id,
            timestamp: now,
            sensor_type: req.sensor_type,
            value: req.value,
            unit: req.unit,
            metadata: req.metadata,
        };

        // Insert reading
        sqlx::query_as::<_, SensorReading>(
            r#"
            INSERT INTO sensor_readings (id, device_id, timestamp, sensor_type, value, unit, metadata)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING *
            "#
        )
        .bind(reading.id)
        .bind(reading.device_id)
        .bind(reading.timestamp)
        .bind(&reading.sensor_type)
        .bind(reading.value)
        .bind(&reading.unit)
        .bind(&reading.metadata)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;

        // Check for alert thresholds
        self.check_alert_thresholds(&reading).await?;

        Ok(reading)
    }

    /// Get recent readings for a device
    pub async fn get_readings(&self, device_id: Uuid, limit: i32, sensor_type: Option<&str>) 
        -> Result<Vec<SensorReading>, Box<dyn std::error::Error + Send + Sync>> 
    {
        let mut query = String::from("SELECT * FROM sensor_readings WHERE device_id = $1");
        
        if let Some(st) = sensor_type {
            query.push_str(" AND sensor_type = $2");
        }
        
        query.push_str(" ORDER BY timestamp DESC LIMIT ");
        query.push_str(&limit.to_string());

        let readings = if sensor_type.is_some() {
            sqlx::query_as::<_, SensorReading>(&query)
                .bind(device_id)
                .bind(sensor_type.unwrap())
                .fetch_all(&self.pool)
                .await
        } else {
            sqlx::query_as::<_, SensorReading>(&query)
                .bind(device_id)
                .fetch_all(&self.pool)
                .await
        }
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;

        Ok(readings)
    }

    /// Send command to device
    pub async fn send_command(&self, device_id: Uuid, req: SendCommandRequest) 
        -> Result<DeviceCommand, Box<dyn std::error::Error + Send + Sync>> 
    {
        let id = Uuid::new_v4();
        let now = Utc::now();

        let command = DeviceCommand {
            id,
            device_id,
            command_type: req.command_type,
            parameters: req.parameters,
            status: CommandStatus::Pending,
            result: None,
            created_at: now,
            executed_at: None,
        };

        // Insert command
        sqlx::query_as::<_, DeviceCommand>(
            r#"
            INSERT INTO device_commands (id, device_id, command_type, parameters, status, result, created_at, executed_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            RETURNING *
            "#
        )
        .bind(command.id)
        .bind(command.device_id)
        .bind(&command.command_type)
        .bind(&command.parameters)
        .bind(&command.status)
        .bind(&command.result)
        .bind(command.created_at)
        .bind(command.executed_at)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;

        // In a real implementation, you would send this to the device via MQTT/CoAP/etc.
        // For now, we just store it in the database

        Ok(command)
    }

    /// Get command status
    pub async fn get_command(&self, command_id: Uuid) -> Result<Option<DeviceCommand>, Box<dyn std::error::Error + Send + Sync>> {
        let command = sqlx::query_as::<_, DeviceCommand>(
            "SELECT * FROM device_commands WHERE id = $1"
        )
        .bind(command_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;

        Ok(command)
    }

    /// Update command status
    pub async fn update_command_status(&self, command_id: Uuid, status: CommandStatus, result: Option<serde_json::Value>) 
        -> Result<DeviceCommand, Box<dyn std::error::Error + Send + Sync>> 
    {
        let now = Utc::now();
        let executed_at = if status == CommandStatus::Executed || status == CommandStatus::Failed {
            Some(now)
        } else {
            None
        };

        let command = sqlx::query_as::<_, DeviceCommand>(
            r#"
            UPDATE device_commands 
            SET status = $2, result = COALESCE($3, result), executed_at = COALESCE($4, executed_at), updated_at = $5
            WHERE id = $1
            RETURNING *
            "#
        )
        .bind(command_id)
        .bind(&status)
        .bind(&result)
        .bind(executed_at)
        .bind(now)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;

        Ok(command)
    }

    /// Create alert threshold
    pub async fn create_alert_threshold(&self, req: AlertThresholdRequest) 
        -> Result<AlertThreshold, Box<dyn std::error::Error + Send + Sync>> 
    {
        let id = Uuid::new_v4();
        let now = Utc::now();

        let threshold = AlertThreshold {
            id,
            device_id: req.device_id,
            sensor_type: req.sensor_type,
            min_value: req.min_value,
            max_value: req.max_value,
            severity: req.severity,
            notification_enabled: req.notification_enabled,
            created_at: now,
            updated_at: now,
        };

        sqlx::query_as::<_, AlertThreshold>(
            r#"
            INSERT INTO alert_thresholds (id, device_id, sensor_type, min_value, max_value, severity, 
                                          notification_enabled, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            RETURNING *
            "#
        )
        .bind(threshold.id)
        .bind(threshold.device_id)
        .bind(&threshold.sensor_type)
        .bind(&threshold.min_value)
        .bind(&threshold.max_value)
        .bind(&threshold.severity)
        .bind(threshold.notification_enabled)
        .bind(threshold.created_at)
        .bind(threshold.updated_at)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;

        Ok(threshold)
    }

    /// Check if a reading triggers any alert thresholds
    async fn check_alert_thresholds(&self, reading: &SensorReading) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let thresholds = sqlx::query_as::<_, AlertThreshold>(
            "SELECT * FROM alert_thresholds WHERE device_id = $1 AND sensor_type = $2"
        )
        .bind(reading.device_id)
        .bind(&reading.sensor_type)
        .fetch_all(&self.pool)
        .await?;

        for threshold in thresholds {
            let triggered = match (threshold.min_value, threshold.max_value) {
                (Some(min), Some(max)) => reading.value < min || reading.value > max,
                (Some(min), None) => reading.value < min,
                (None, Some(max)) => reading.value > max,
                (None, None) => false,
            };

            if triggered {
                let alert = IotAlert {
                    id: Uuid::new_v4(),
                    device_id: reading.device_id,
                    alert_type: format!("Threshold exceeded for {}", reading.sensor_type),
                    severity: threshold.severity,
                    message: format!(
                        "Sensor {} reading {} {} is outside threshold [{:?}, {:?}]",
                        reading.sensor_type, reading.value, reading.unit, threshold.min_value, threshold.max_value
                    ),
                    is_resolved: false,
                    resolved_at: None,
                    created_at: Utc::now(),
                };

                // Insert alert into database
                self.insert_alert(&alert).await?;

                // Broadcast alert
                let _ = self.alert_tx.send(alert);
            }
        }

        Ok(())
    }

    /// Insert alert into database
    async fn insert_alert(&self, alert: &IotAlert) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            INSERT INTO iot_alerts (id, device_id, alert_type, severity, message, is_resolved, resolved_at, created_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            "#
        )
        .bind(alert.id)
        .bind(alert.device_id)
        .bind(&alert.alert_type)
        .bind(&alert.severity)
        .bind(&alert.message)
        .bind(alert.is_resolved)
        .bind(alert.resolved_at)
        .bind(alert.created_at)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// Get alerts for a device
    pub async fn get_alerts(&self, device_id: Uuid, include_resolved: bool) 
        -> Result<Vec<IotAlert>, Box<dyn std::error::Error + Send + Sync>> 
    {
        let mut query = String::from("SELECT * FROM iot_alerts WHERE device_id = $1");
        
        if !include_resolved {
            query.push_str(" AND is_resolved = false");
        }
        
        query.push_str(" ORDER BY created_at DESC");

        let alerts = sqlx::query_as::<_, IotAlert>(&query)
            .bind(device_id)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;

        Ok(alerts)
    }

    /// Resolve an alert
    pub async fn resolve_alert(&self, alert_id: Uuid) -> Result<IotAlert, Box<dyn std::error::Error + Send + Sync>> {
        let now = Utc::now();
        
        let alert = sqlx::query_as::<_, IotAlert>(
            r#"
            UPDATE iot_alerts 
            SET is_resolved = true, resolved_at = $2
            WHERE id = $1
            RETURNING *
            "#
        )
        .bind(alert_id)
        .bind(now)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;

        Ok(alert)
    }

    /// Get sensor analytics for a time period
    pub async fn get_sensor_analytics(&self, device_id: Uuid, sensor_type: &str, 
                                      start: DateTime<Utc>, end: DateTime<Utc>) 
        -> Result<SensorAnalytics, Box<dyn std::error::Error + Send + Sync>> 
    {
        let analytics = sqlx::query_as::<_, SensorAnalytics>(
            r#"
            SELECT 
                device_id,
                sensor_type,
                MIN(value) as min_value,
                MAX(value) as max_value,
                AVG(value) as avg_value,
                COUNT(*) as count,
                $3 as period_start,
                $4 as period_end
            FROM sensor_readings
            WHERE device_id = $1 AND sensor_type = $2 
              AND timestamp BETWEEN $3 AND $4
            GROUP BY device_id, sensor_type
            "#
        )
        .bind(device_id)
        .bind(sensor_type)
        .bind(start)
        .bind(end)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;

        Ok(analytics)
    }

    /// Delete a device
    pub async fn delete_device(&self, device_id: Uuid) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        sqlx::query("DELETE FROM iot_devices WHERE id = $1")
            .bind(device_id)
            .execute(&self.pool)
            .await
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;

        Ok(())
    }
}

// Implement FromRow for custom types if needed
impl FromRow<'_, sqlx::postgres::PgRow> for IotDevice {
    fn from_row(row: &sqlx::postgres::PgRow) -> Result<Self, sqlx::Error> {
        use sqlx::Row;
        
        let device_type_str: String = row.try_get("device_type")?;
        let status_str: String = row.try_get("status")?;
        
        let device_type = serde_json::from_str(&format!("\"{}\"", device_type_str))
            .map_err(|e| sqlx::Error::Decode(Box::new(e)))?;
        let status = serde_json::from_str(&format!("\"{}\"", status_str))
            .map_err(|e| sqlx::Error::Decode(Box::new(e)))?;

        Ok(IotDevice {
            id: row.try_get("id")?,
            name: row.try_get("name")?,
            device_type,
            status,
            location: row.try_get("location")?,
            institution_id: row.try_get("institution_id")?,
            course_id: row.try_get("course_id")?,
            room_id: row.try_get("room_id")?,
            ip_address: row.try_get("ip_address")?,
            mac_address: row.try_get("mac_address")?,
            firmware_version: row.try_get("firmware_version")?,
            last_seen: row.try_get("last_seen")?,
            metadata: row.try_get("metadata")?,
            created_at: row.try_get("created_at")?,
            updated_at: row.try_get("updated_at")?,
        })
    }
}

impl FromRow<'_, sqlx::postgres::PgRow> for SensorReading {
    fn from_row(row: &sqlx::postgres::PgRow) -> Result<Self, sqlx::Error> {
        use sqlx::Row;

        Ok(SensorReading {
            id: row.try_get("id")?,
            device_id: row.try_get("device_id")?,
            timestamp: row.try_get("timestamp")?,
            sensor_type: row.try_get("sensor_type")?,
            value: row.try_get("value")?,
            unit: row.try_get("unit")?,
            metadata: row.try_get("metadata")?,
        })
    }
}

impl FromRow<'_, sqlx::postgres::PgRow> for DeviceCommand {
    fn from_row(row: &sqlx::postgres::PgRow) -> Result<Self, sqlx::Error> {
        use sqlx::Row;

        let status_str: String = row.try_get("status")?;
        let status = serde_json::from_str(&format!("\"{}\"", status_str))
            .map_err(|e| sqlx::Error::Decode(Box::new(e)))?;

        Ok(DeviceCommand {
            id: row.try_get("id")?,
            device_id: row.try_get("device_id")?,
            command_type: row.try_get("command_type")?,
            parameters: row.try_get("parameters")?,
            status,
            result: row.try_get("result")?,
            created_at: row.try_get("created_at")?,
            executed_at: row.try_get("executed_at")?,
        })
    }
}

impl FromRow<'_, sqlx::postgres::PgRow> for IotAlert {
    fn from_row(row: &sqlx::postgres::PgRow) -> Result<Self, sqlx::Error> {
        use sqlx::Row;

        let severity_str: String = row.try_get("severity")?;
        let severity = serde_json::from_str(&format!("\"{}\"", severity_str))
            .map_err(|e| sqlx::Error::Decode(Box::new(e)))?;

        Ok(IotAlert {
            id: row.try_get("id")?,
            device_id: row.try_get("device_id")?,
            alert_type: row.try_get("alert_type")?,
            severity,
            message: row.try_get("message")?,
            is_resolved: row.try_get("is_resolved")?,
            resolved_at: row.try_get("resolved_at")?,
            created_at: row.try_get("created_at")?,
        })
    }
}

impl FromRow<'_, sqlx::postgres::PgRow> for AlertThreshold {
    fn from_row(row: &sqlx::postgres::PgRow) -> Result<Self, sqlx::Error> {
        use sqlx::Row;

        let severity_str: String = row.try_get("severity")?;
        let severity = serde_json::from_str(&format!("\"{}\"", severity_str))
            .map_err(|e| sqlx::Error::Decode(Box::new(e)))?;

        Ok(AlertThreshold {
            id: row.try_get("id")?,
            device_id: row.try_get("device_id")?,
            sensor_type: row.try_get("sensor_type")?,
            min_value: row.try_get("min_value")?,
            max_value: row.try_get("max_value")?,
            severity,
            notification_enabled: row.try_get("notification_enabled")?,
            created_at: row.try_get("created_at")?,
            updated_at: row.try_get("updated_at")?,
        })
    }
}

impl FromRow<'_, sqlx::postgres::PgRow> for SensorAnalytics {
    fn from_row(row: &sqlx::postgres::PgRow) -> Result<Self, sqlx::Error> {
        use sqlx::Row;

        Ok(SensorAnalytics {
            device_id: row.try_get("device_id")?,
            sensor_type: row.try_get("sensor_type")?,
            min_value: row.try_get("min_value")?,
            max_value: row.try_get("max_value")?,
            avg_value: row.try_get("avg_value")?,
            count: row.try_get("count")?,
            period_start: row.try_get("period_start")?,
            period_end: row.try_get("period_end")?,
        })
    }
}
