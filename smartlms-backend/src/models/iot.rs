//! IoT Device and Sensor Models

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// Types of IoT devices supported
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum DeviceType {
    /// RFID/NFC reader for attendance
    AttendanceScanner,
    /// Environmental sensor (temperature, humidity, air quality)
    EnvironmentalSensor,
    /// Smart lock for room access
    SmartLock,
    /// Motion detector
    MotionSensor,
    /// Energy monitor
    EnergyMeter,
    /// Camera for security/proctoring
    SecurityCamera,
    /// Smart projector/display
    SmartDisplay,
    /// Emergency button
    EmergencyButton,
    /// Lab equipment monitor
    LabEquipment,
    /// Generic sensor
    Generic,
}

/// Status of an IoT device
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum DeviceStatus {
    Online,
    Offline,
    Maintenance,
    Error,
    Disabled,
}

/// An IoT device registered in the system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IotDevice {
    pub id: Uuid,
    pub name: String,
    pub device_type: DeviceType,
    pub status: DeviceStatus,
    pub location: String,
    pub institution_id: Uuid,
    pub course_id: Option<Uuid>,
    pub room_id: Option<Uuid>,
    pub ip_address: Option<String>,
    pub mac_address: Option<String>,
    pub firmware_version: Option<String>,
    pub last_seen: Option<DateTime<Utc>>,
    pub metadata: serde_json::Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Sensor data reading from an IoT device
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SensorReading {
    pub id: Uuid,
    pub device_id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub sensor_type: String,
    pub value: f64,
    pub unit: String,
    pub metadata: Option<serde_json::Value>,
}

/// Command to send to an IoT device
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceCommand {
    pub id: Uuid,
    pub device_id: Uuid,
    pub command_type: String,
    pub parameters: serde_json::Value,
    pub status: CommandStatus,
    pub result: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
    pub executed_at: Option<DateTime<Utc>>,
}

/// Status of a device command
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CommandStatus {
    Pending,
    Sent,
    Executed,
    Failed,
    Timeout,
}

/// Request to register a new IoT device
#[derive(Debug, Deserialize)]
pub struct RegisterDeviceRequest {
    pub name: String,
    pub device_type: DeviceType,
    pub location: String,
    pub institution_id: Uuid,
    pub course_id: Option<Uuid>,
    pub room_id: Option<Uuid>,
    pub ip_address: Option<String>,
    pub mac_address: Option<String>,
    pub metadata: Option<serde_json::Value>,
}

/// Request to update device status
#[derive(Debug, Deserialize)]
pub struct UpdateDeviceStatusRequest {
    pub status: DeviceStatus,
    pub firmware_version: Option<String>,
}

/// Request to submit sensor reading
#[derive(Debug, Deserialize)]
pub struct SubmitReadingRequest {
    pub sensor_type: String,
    pub value: f64,
    pub unit: String,
    pub metadata: Option<serde_json::Value>,
}

/// Request to send command to device
#[derive(Debug, Deserialize)]
pub struct SendCommandRequest {
    pub command_type: String,
    pub parameters: serde_json::Value,
}

/// Response with device information
#[derive(Debug, Serialize)]
pub struct DeviceResponse {
    pub device: IotDevice,
    pub recent_readings: Vec<SensorReading>,
}

/// Response with sensor readings
#[derive(Debug, Serialize)]
pub struct ReadingsResponse {
    pub readings: Vec<SensorReading>,
    pub device_info: Option<IotDevice>,
}

/// Aggregated sensor data for analytics
#[derive(Debug, Serialize, Deserialize)]
pub struct SensorAnalytics {
    pub device_id: Uuid,
    pub sensor_type: String,
    pub min_value: f64,
    pub max_value: f64,
    pub avg_value: f64,
    pub count: i64,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
}

/// Alert triggered by IoT device
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IotAlert {
    pub id: Uuid,
    pub device_id: Uuid,
    pub alert_type: String,
    pub severity: AlertSeverity,
    pub message: String,
    pub is_resolved: bool,
    pub resolved_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

/// Severity level for alerts
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum AlertSeverity {
    Low,
    Medium,
    High,
    Critical,
}

/// Request to create an alert threshold
#[derive(Debug, Deserialize)]
pub struct AlertThresholdRequest {
    pub device_id: Uuid,
    pub sensor_type: String,
    pub min_value: Option<f64>,
    pub max_value: Option<f64>,
    pub severity: AlertSeverity,
    pub notification_enabled: bool,
}

/// Alert threshold configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertThreshold {
    pub id: Uuid,
    pub device_id: Uuid,
    pub sensor_type: String,
    pub min_value: Option<f64>,
    pub max_value: Option<f64>,
    pub severity: AlertSeverity,
    pub notification_enabled: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
