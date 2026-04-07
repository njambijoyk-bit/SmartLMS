// Backup model - backup records, schedules, verification
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// Backup type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BackupType {
    Full,
    Incremental,
    Schema,
    Config,
}

/// Backup status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BackupStatus {
    Pending,
    Running,
    Completed,
    Failed,
    Verified,
}

/// Backup record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Backup {
    pub id: uuid::Uuid,
    pub institution_id: uuid::Uuid,
    pub backup_type: BackupType,
    pub status: BackupStatus,
    pub file_path: Option<String>,
    pub file_size_bytes: Option<i64>,
    pub started_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub checksum: Option<String>,
    pub retention_days: i32,
    pub created_by: Option<uuid::Uuid>,
}

/// Schedule type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScheduleType {
    Hourly,
    Daily,
    Weekly,
    Monthly,
}

/// Backup schedule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupSchedule {
    pub id: uuid::Uuid,
    pub institution_id: uuid::Uuid,
    pub schedule_type: ScheduleType,
    pub backup_type: BackupType,
    pub interval_hours: i32,
    pub retention_days: i32,
    pub is_active: bool,
    pub next_run_at: Option<DateTime<Utc>>,
    pub last_run_at: Option<DateTime<Utc>>,
}

/// Restore options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RestoreOptions {
    pub restore_schema: bool,
    pub restore_data: bool,
    pub overwrite_existing: bool,
    pub validate_only: bool,
}

/// Restore result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RestoreResult {
    pub success: bool,
    pub restored_tables: Vec<String>,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
}

/// Verification result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationResult {
    pub backup_id: uuid::Uuid,
    pub is_valid: bool,
    pub schema_valid: bool,
    pub data_integrity: bool,
    pub table_counts: std::collections::HashMap<String, i64>,
    pub errors: Vec<String>,
    pub verified_at: DateTime<Utc>,
}

/// Backup config
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupConfig {
    pub enabled: bool,
    pub schedule_type: ScheduleType,
    pub backup_type: BackupType,
    pub retention_days: i32,
    pub max_backups: i32,
    pub compression_enabled: bool,
    pub encryption_enabled: bool,
    pub notify_on_failure: bool,
    pub notification_email: Option<String>,
}