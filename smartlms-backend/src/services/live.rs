// Live class service - video sessions, scheduling, attendance
use crate::db::live as live_db;
use crate::models::live::*;
use sqlx::PgPool;
use uuid::Uuid;

// Video provider integration
pub mod video_providers {
    use super::*;

    /// Create a meeting via Zoom API
    pub async fn create_zoom_meeting(
        req: &CreateSessionRequest,
        config: &ZoomConfig,
    ) -> Result<ZoomMeeting, String> {
        let client = reqwest::Client::new();

        let response = client
            .post(&format!("https://api.zoom.us/v2/users/{}/meetings", config.user_id))
            .header("Authorization", format!("Bearer {}", config.access_token))
            .json(&serde_json::json!({
                "topic": req.title,
                "type": 2, // Scheduled meeting
                "start_time": req.scheduled_start.format("%Y-%m-%dT%H:%M:%S"),
                "duration": (req.scheduled_end - req.scheduled_start).num_minutes() as i64,
                "timezone": "UTC",
                "password": req.password.as_deref().unwrap_or(""),
                "settings": {
                    "host_video": true,
                    "participant_video": true,
                    "join_before_host": false,
                    "mute_upon_entry": true,
                    "waiting_room": true,
                    "auto_recording": if req.is_recording_enabled.unwrap_or(false) { "cloud" } else { "none" }
                }
            }))
            .send()
            .await
            .map_err(|e| e.to_string())?;

        let meeting: ZoomMeeting = response.json().await.map_err(|e| e.to_string())?;
        Ok(meeting)
    }

    /// Create Google Meet
    pub async fn create_google_meet(
        req: &CreateSessionRequest,
        access_token: &str,
    ) -> Result<GoogleMeet, String> {
        let client = reqwest::Client::new();

        let response = client
            .post("https://www.googleapis.com/calendar/v3/calendars/primary/events")
            .header("Authorization", format!("Bearer {}", access_token))
            .json(&serde_json::json!({
                "summary": req.title,
                "description": req.description,
                "start": {
                    "dateTime": req.scheduled_start.to_rfc3339(),
                    "timeZone": "UTC"
                },
                "end": {
                    "dateTime": req.scheduled_end.to_rfc3339(),
                    "timeZone": "UTC"
                },
                "conferenceData": {
                    "createRequest": {
                        "requestId": Uuid::new_v4().to_string(),
                        "conferenceSolutionKey": { "type": "hangoutsMeet" }
                    }
                }
            }))
            .send()
            .await
            .map_err(|e| e.to_string())?;

        let event: serde_json::Value = response.json().await.map_err(|e| e.to_string())?;

        let meet_link = event["conferenceData"]["entryPoints"][0]["uri"]
            .as_str()
            .unwrap_or("")
            .to_string();

        Ok(GoogleMeet {
            meeting_id: Uuid::new_v4().to_string(),
            join_url: meet_link,
            host_url: meet_link,
        })
    }

    /// Generate Jitsi meeting URL (no API needed)
    pub fn create_jitsi_meeting(title: &str) -> (String, String) {
        let meeting_id = format!(
            "{}-{}",
            title.to_lowercase().replace(" ", "-"),
            &Uuid::new_v4().to_string()[..8]
        );

        let base_url = "meet.jit.si";
        let join_url = format!("https://{}/{}", base_url, meeting_id);
        let host_url = format!("https://{}/{}+host", base_url, meeting_id);

        (join_url, host_url)
    }
}

#[derive(Debug, Clone)]
pub struct ZoomConfig {
    pub account_id: String,
    pub client_id: String,
    pub client_secret: String,
    pub access_token: String,
    pub user_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZoomMeeting {
    pub id: i64,
    pub join_url: String,
    pub start_url: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoogleMeet {
    pub meeting_id: String,
    pub join_url: String,
    pub host_url: String,
}

// Session operations
pub async fn create_session(
    pool: &PgPool,
    instructor_id: Uuid,
    req: &CreateSessionRequest,
) -> Result<LiveSession, String> {
    if req.title.is_empty() {
        return Err("Title required".to_string());
    }

    // Create meeting based on provider
    let (meeting_id, join_url, host_url, password) = match req.provider {
        VideoProvider::Zoom => {
            // In production, call Zoom API
            let id = Uuid::new_v4().to_string();
            (
                Some(id.clone()),
                Some(format!("https://zoom.us/j/{}", id)),
                Some(format!("https://zoom.us/s/{}", id)),
                None,
            )
        }
        VideoProvider::GoogleMeet => {
            let id = Uuid::new_v4().to_string();
            (
                Some(id),
                Some(format!("https://meet.google.com/{}", id)),
                None,
                None,
            )
        }
        VideoProvider::Jitsi => {
            let (join, host) = video_providers::create_jitsi_meeting(&req.title);
            (None, Some(join), Some(host), None)
        }
        VideoProvider::Custom => (None, None, None, None),
    };

    let session = live_db::create_session(
        pool,
        instructor_id,
        req,
        meeting_id,
        join_url,
        host_url,
        password,
    )
    .await
    .map_err(|e| e.to_string())?;

    Ok(session)
}

pub async fn get_session(pool: &PgPool, session_id: Uuid) -> Result<SessionDetailResponse, String> {
    let session = live_db::get_session(pool, session_id)
        .await
        .map_err(|e| e.to_string())?
        .ok_or("Session not found")?;

    let attendee_count = live_db::count_attendees(pool, session_id)
        .await
        .map_err(|e| e.to_string())?;

    let summary = live_db::get_attendance_summary(pool, session_id)
        .await
        .map_err(|e| e.to_string())?
        .unwrap_or(AttendanceSummary {
            session_id,
            total_enrolled: 0,
            present: 0,
            late: 0,
            absent: 0,
            excused: 0,
            attendance_rate: 0.0,
        });

    Ok(SessionDetailResponse {
        session,
        attendee_count,
        attendance_summary: summary,
    })
}

pub async fn list_sessions(
    pool: &PgPool,
    course_id: Option<Uuid>,
    status: Option<SessionStatus>,
    page: i64,
    per_page: i64,
) -> Result<SessionListResponse, String> {
    let (sessions, total) = live_db::list_sessions(pool, course_id, status, page, per_page)
        .await
        .map_err(|e| e.to_string())?;

    Ok(SessionListResponse {
        sessions,
        total,
        page,
        per_page,
    })
}

pub async fn update_session(
    pool: &PgPool,
    session_id: Uuid,
    req: &UpdateSessionRequest,
) -> Result<LiveSession, String> {
    live_db::update_session(pool, session_id, req)
        .await
        .map_err(|e| e.to_string())
}

pub async fn cancel_session(pool: &PgPool, session_id: Uuid) -> Result<LiveSession, String> {
    let req = UpdateSessionRequest {
        title: None,
        description: None,
        scheduled_start: None,
        scheduled_end: None,
        status: Some(SessionStatus::Cancelled),
        max_participants: None,
    };

    update_session(pool, session_id, &req).await
}

pub async fn start_session(pool: &PgPool, session_id: Uuid) -> Result<LiveSession, String> {
    let req = UpdateSessionRequest {
        title: None,
        description: None,
        scheduled_start: None,
        scheduled_end: None,
        status: Some(SessionStatus::Live),
        max_participants: None,
    };

    update_session(pool, session_id, &req).await
}

pub async fn end_session(pool: &PgPool, session_id: Uuid) -> Result<LiveSession, String> {
    let req = UpdateSessionRequest {
        title: None,
        description: None,
        scheduled_start: None,
        scheduled_end: None,
        status: Some(SessionStatus::Ended),
        max_participants: None,
    };

    update_session(pool, session_id, &req).await
}

// Attendance operations
pub async fn mark_attendance(
    pool: &PgPool,
    session_id: Uuid,
    marker_id: Uuid,
    req: &MarkAttendanceRequest,
) -> Result<Attendance, String> {
    live_db::mark_attendance(pool, session_id, marker_id, req)
        .await
        .map_err(|e| e.to_string())
}

pub async fn bulk_mark_attendance(
    pool: &PgPool,
    session_id: Uuid,
    marker_id: Uuid,
    req: &BulkAttendanceRequest,
) -> Result<Vec<Attendance>, String> {
    let mut results = Vec::new();

    for attendance_req in &req.attendances {
        let result = mark_attendance(
            pool,
            session_id,
            marker_id,
            &MarkAttendanceRequest {
                user_id: attendance_req.user_id,
                status: attendance_req.status,
                notes: attendance_req.notes.clone(),
            },
        )
        .await?;

        results.push(result);
    }

    Ok(results)
}

pub async fn get_session_attendance(
    pool: &PgPool,
    session_id: Uuid,
) -> Result<Vec<Attendance>, String> {
    live_db::get_session_attendance(pool, session_id)
        .await
        .map_err(|e| e.to_string())
}

pub async fn get_user_attendance(
    pool: &PgPool,
    user_id: Uuid,
    course_id: Option<Uuid>,
) -> Result<Vec<Attendance>, String> {
    live_db::get_user_attendance(pool, user_id, course_id)
        .await
        .map_err(|e| e.to_string())
}

// Recurring schedule
pub async fn create_recurring_schedule(
    pool: &PgPool,
    req: &CreateRecurringScheduleRequest,
) -> Result<RecurringSchedule, String> {
    live_db::create_recurring_schedule(pool, req)
        .await
        .map_err(|e| e.to_string())
}

pub async fn generate_sessions_from_schedule(
    pool: &PgPool,
    schedule_id: Uuid,
) -> Result<Vec<LiveSession>, String> {
    // TODO: Implement to generate sessions for the next few weeks
    Ok(vec![])
}
