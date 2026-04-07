// Live class model - video sessions, scheduling, attendance
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// Video provider
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VideoProvider {
    Zoom,
    GoogleMeet,
    Jitsi,
    Custom,
}

/// Live session status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SessionStatus {
    Scheduled,
    Live,
    Ended,
    Cancelled,
}

/// Live session entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiveSession {
    pub id: uuid::Uuid,
    pub title: String,
    pub description: Option<String>,
    pub course_id: uuid::Uuid,
    pub module_id: Option<uuid::Uuid>,
    pub instructor_id: uuid::Uuid,
    pub provider: VideoProvider,
    pub provider_meeting_id: Option<String>,
    pub provider_join_url: Option<String>,
    pub provider_host_url: Option<String>,
    pub password: Option<String>,
    pub scheduled_start: DateTime<Utc>,
    pub scheduled_end: DateTime<Utc>,
    pub actual_start: Option<DateTime<Utc>>,
    pub actual_end: Option<DateTime<Utc>>,
    pub status: SessionStatus,
    pub max_participants: Option<i32>,
    pub is_recording_enabled: bool,
    pub recording_url: Option<String>,
    pub created_at: DateTime<Utc>,
}

/// Session attendee (user in session)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionAttendee {
    pub id: uuid::Uuid,
    pub session_id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub joined_at: DateTime<Utc>,
    pub left_at: Option<DateTime<Utc>>,
    pub duration_seconds: i64,
    pub is_present: bool,
}

/// Attendance record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attendance {
    pub id: uuid::Uuid,
    pub session_id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub status: AttendanceStatus,
    pub marked_by: Option<uuid::Uuid>,
    pub marked_at: Option<DateTime<Utc>>,
    pub notes: Option<String>,
}

/// Attendance status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AttendanceStatus {
    Present,
    Late,
    Absent,
    Excused,
}

/// Attendance summary for a course/session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttendanceSummary {
    pub session_id: uuid::Uuid,
    pub total_enrolled: i32,
    pub present: i32,
    pub late: i32,
    pub absent: i32,
    pub excused: i32,
    pub attendance_rate: f32,
}

/// Schedule recurring sessions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecurringSchedule {
    pub id: uuid::Uuid,
    pub course_id: uuid::Uuid,
    pub title: String,
    pub instructor_id: uuid::Uuid,
    pub day_of_week: i32,        // 0=Sunday, 6=Saturday
    pub start_time: String,      // "14:00"
    pub end_time: String,        // "15:30"
    pub duration_minutes: i32,
    pub start_date: DateTime<Utc>,
    pub end_date: Option<DateTime<Utc>>,
    pub provider: VideoProvider,
    pub repeat_weekly: bool,
    pub is_active: bool,
}

// Request/Response types
#[derive(Debug, Deserialize)]
pub struct CreateSessionRequest {
    pub title: String,
    pub description: Option<String>,
    pub course_id: uuid::Uuid,
    pub module_id: Option<uuid::Uuid>,
    pub provider: VideoProvider,
    pub scheduled_start: DateTime<Utc>,
    pub scheduled_end: DateTime<Utc>,
    pub max_participants: Option<i32>,
    pub is_recording_enabled: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateSessionRequest {
    pub title: Option<String>,
    pub description: Option<String>,
    pub scheduled_start: Option<DateTime<Utc>>,
    pub scheduled_end: Option<DateTime<Utc>>,
    pub status: Option<SessionStatus>,
    pub max_participants: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct MarkAttendanceRequest {
    pub user_id: uuid::Uuid,
    pub status: AttendanceStatus,
    pub notes: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct BulkAttendanceRequest {
    pub attendances: Vec<MarkAttendanceRequest>,
}

#[derive(Debug, Deserialize)]
pub struct CreateRecurringScheduleRequest {
    pub course_id: uuid::Uuid,
    pub title: String,
    pub day_of_week: i32,
    pub start_time: String,
    pub end_time: String,
    pub start_date: DateTime<Utc>,
    pub end_date: Option<DateTime<Utc>>,
    pub provider: VideoProvider,
    pub repeat_weekly: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct SessionDetailResponse {
    pub session: LiveSession,
    pub attendee_count: i32,
    pub attendance_summary: AttendanceSummary,
}

#[derive(Debug, Serialize)]
pub struct SessionListResponse {
    pub sessions: Vec<LiveSession>,
    pub total: i64,
    pub page: i64,
    pub per_page: i64,
}