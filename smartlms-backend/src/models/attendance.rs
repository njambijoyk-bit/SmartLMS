// Standalone Attendance model - QR codes, manual entry, reports
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Attendance type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AttendanceType {
    QRCode,
    Manual,
    Auto,
    SelfCheck,
}

/// Attendance status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AttendanceStatus {
    Present,
    Late,
    Absent,
    Excused,
}

/// Class attendance record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassAttendance {
    pub id: uuid::Uuid,
    pub session_id: uuid::Uuid,
    pub course_id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub attendance_type: AttendanceType,
    pub status: AttendanceStatus,
    pub marked_at: DateTime<Utc>,
    pub marked_by: Option<uuid::Uuid>,
    pub location: Option<String>,
    pub device_info: Option<String>,
    pub notes: Option<String>,
}

/// QR code session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QRCodeSession {
    pub id: uuid::Uuid,
    pub session_id: uuid::Uuid,
    pub code: String,
    pub code_url: Option<String>,
    pub expires_at: DateTime<Utc>,
    pub is_active: bool,
    pub max_uses: Option<i32>,
    pub used_count: i32,
    pub location_radius_meters: Option<i32>,
}

// Request types
#[derive(Debug, Deserialize)]
pub struct QRCheckInRequest {
    pub code: String,
    pub user_id: uuid::Uuid,
    pub location: Option<String>,
    pub device_info: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ManualAttendanceRequest {
    pub user_id: uuid::Uuid,
    pub course_id: uuid::Uuid,
    pub session_id: Option<uuid::Uuid>,
    pub date: Option<DateTime<Utc>>,
    pub status: AttendanceStatus,
    pub notes: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct BulkAttendanceRequest {
    pub attendances: Vec<ManualAttendanceRequest>,
    pub marked_by: uuid::Uuid,
}

#[derive(Debug, Deserialize)]
pub struct GenerateQRRequest {
    pub session_id: uuid::Uuid,
    pub expires_minutes: Option<i64>,
    pub max_uses: Option<i32>,
}

// Report types
#[derive(Debug, Serialize)]
pub struct AttendanceReport {
    pub course_id: uuid::Uuid,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub total_sessions: i32,
    pub total_students: i32,
    pub present_count: i32,
    pub late_count: i32,
    pub absent_count: i32,
    pub excused_count: i32,
    pub overall_attendance_rate: f32,
    pub student_details: Vec<StudentAttendanceRecord>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StudentAttendanceRecord {
    pub user_id: uuid::Uuid,
    pub student_name: String,
    pub present: i32,
    pub late: i32,
    pub absent: i32,
    pub excused: i32,
    pub attendance_rate: f32,
}
