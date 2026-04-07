// Database operations for live classes
use crate::models::live::*;
use sqlx::{PgPool, Row};
use uuid::Uuid;
use chrono::{DateTime, Utc};

pub async fn create_session(
    pool: &PgPool,
    instructor_id: Uuid,
    req: &CreateSessionRequest,
    provider_meeting_id: Option<String>,
    provider_join_url: Option<String>,
    provider_host_url: Option<String>,
    password: Option<String>,
) -> Result<LiveSession, sqlx::Error> {
    let id = Uuid::new_v4();
    let now = Utc::now();

    sqlx::query!(
        "INSERT INTO live_sessions (id, title, description, course_id, module_id, instructor_id,
         provider, provider_meeting_id, provider_join_url, provider_host_url, password,
         scheduled_start, scheduled_end, status, is_recording_enabled, created_at)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16)",
        id, req.title, req.description, req.course_id, req.module_id, instructor_id,
        format!("{:?}", req.provider).to_lowercase(), provider_meeting_id, provider_join_url,
        provider_host_url, password, req.scheduled_start, req.scheduled_end, "scheduled",
        req.is_recording_enabled.unwrap_or(false), now
    )
    .execute(pool)
    .await?;

    Ok(LiveSession {
        id,
        title: req.title.clone(),
        description: req.description.clone(),
        course_id: req.course_id,
        module_id: req.module_id,
        instructor_id,
        provider: req.provider,
        provider_meeting_id,
        provider_join_url,
        provider_host_url,
        password,
        scheduled_start: req.scheduled_start,
        scheduled_end: req.scheduled_end,
        actual_start: None,
        actual_end: None,
        status: SessionStatus::Scheduled,
        max_participants: req.max_participants,
        is_recording_enabled: req.is_recording_enabled.unwrap_or(false),
        recording_url: None,
        created_at: now,
    })
}

pub async fn get_session(pool: &PgPool, id: Uuid) -> Result<Option<LiveSession>, sqlx::Error> {
    let row = sqlx::query!(
        "SELECT id, title, description, course_id, module_id, instructor_id, provider,
                provider_meeting_id, provider_join_url, provider_host_url, password,
                scheduled_start, scheduled_end, actual_start, actual_end, status,
                max_participants, is_recording_enabled, recording_url, created_at
         FROM live_sessions WHERE id = $1",
        id
    )
    .fetch_optional(pool)
    .await?;

    Ok(row.map(|r| LiveSession {
        id: r.id,
        title: r.title,
        description: r.description,
        course_id: r.course_id,
        module_id: r.module_id,
        instructor_id: r.instructor_id,
        provider: VideoProvider::Zoom,
        provider_meeting_id: r.provider_meeting_id,
        provider_join_url: r.provider_join_url,
        provider_host_url: r.provider_host_url,
        password: r.password,
        scheduled_start: r.scheduled_start,
        scheduled_end: r.scheduled_end,
        actual_start: r.actual_start,
        actual_end: r.actual_end,
        status: SessionStatus::Scheduled,
        max_participants: r.max_participants,
        is_recording_enabled: r.is_recording_enabled,
        recording_url: r.recording_url,
        created_at: r.created_at,
    }))
}

pub async fn list_sessions(
    pool: &PgPool,
    course_id: Option<Uuid>,
    status: Option<SessionStatus>,
    page: i64,
    per_page: i64,
) -> Result<(Vec<LiveSession>, i64), sqlx::Error> {
    let offset = (page - 1) * per_page;
    
    let rows = sqlx::query!(
        "SELECT id, title, description, course_id, module_id, instructor_id, provider,
                provider_meeting_id, provider_join_url, provider_host_url, password,
                scheduled_start, scheduled_end, actual_start, actual_end, status,
                max_participants, is_recording_enabled, recording_url, created_at
         FROM live_sessions ORDER BY scheduled_start DESC LIMIT $1 OFFSET $2",
        per_page, offset
    )
    .fetch_all(pool)
    .await?;

    let sessions: Vec<LiveSession> = rows.into_iter().map(|r| LiveSession {
        id: r.id,
        title: r.title,
        description: r.description,
        course_id: r.course_id,
        module_id: r.module_id,
        instructor_id: r.instructor_id,
        provider: VideoProvider::Zoom,
        provider_meeting_id: r.provider_meeting_id,
        provider_join_url: r.provider_join_url,
        provider_host_url: r.provider_host_url,
        password: r.password,
        scheduled_start: r.scheduled_start,
        scheduled_end: r.scheduled_end,
        actual_start: r.actual_start,
        actual_end: r.actual_end,
        status: SessionStatus::Scheduled,
        max_participants: r.max_participants,
        is_recording_enabled: r.is_recording_enabled,
        recording_url: r.recording_url,
        created_at: r.created_at,
    }).collect();

    Ok((sessions, sessions.len() as i64))
}

pub async fn update_session(
    pool: &PgPool,
    session_id: Uuid,
    req: &UpdateSessionRequest,
) -> Result<LiveSession, sqlx::Error> {
    let now = Utc::now();
    
    if let Some(status) = &req.status {
        sqlx::query!(
            "UPDATE live_sessions SET status = $1, updated_at = $2 WHERE id = $3",
            format!("{:?}", status).to_lowercase(), now, session_id
        )
        .execute(pool)
        .await?;
        
        // Update actual start/end times
        match status {
            SessionStatus::Live => {
                sqlx::query!("UPDATE live_sessions SET actual_start = $1 WHERE id = $2 AND actual_start IS NULL", now, session_id)
                    .execute(pool)
                    .await?;
            }
            SessionStatus::Ended => {
                sqlx::query!("UPDATE live_sessions SET actual_end = $1 WHERE id = $2 AND actual_end IS NULL", now, session_id)
                    .execute(pool)
                    .await?;
            }
            _ => {}
        }
    }
    
    get_session(pool, session_id).await.map(|o| o.unwrap())
}

pub async fn count_attendees(pool: &PgPool, session_id: Uuid) -> Result<i32, sqlx::Error> {
    let row = sqlx::query!(
        "SELECT COUNT(DISTINCT user_id) as count FROM session_attendees WHERE session_id = $1",
        session_id
    )
    .fetch_one(pool)
    .await?;
    Ok(row.count as i32)
}

pub async fn get_attendance_summary(pool: &PgPool, session_id: Uuid) -> Result<Option<AttendanceSummary>, sqlx::Error> {
    let row = sqlx::query!(
        "SELECT 
            SUM(CASE WHEN status = 'present' THEN 1 ELSE 0 END) as present,
            SUM(CASE WHEN status = 'late' THEN 1 ELSE 0 END) as late,
            SUM(CASE WHEN status = 'absent' THEN 1 ELSE 0 END) as absent,
            SUM(CASE WHEN status = 'excused' THEN 1 ELSE 0 END) as excused,
            COUNT(*) as total
         FROM attendance WHERE session_id = $1",
        session_id
    )
    .fetch_one(pool)
    .await?;
    
    let total = row.total.unwrap_or(0) as i32;
    let present = row.present.unwrap_or(0) as i32;
    let late = row.late.unwrap_or(0) as i32;
    
    let rate = if total > 0 {
        ((present + late) as f32 / total as f32) * 100.0
    } else {
        0.0
    };
    
    Ok(Some(AttendanceSummary {
        session_id,
        total_enrolled: total,
        present,
        late,
        absent: row.absent.unwrap_or(0) as i32,
        excused: row.excused.unwrap_or(0) as i32,
        attendance_rate: rate,
    }))
}

pub async fn mark_attendance(
    pool: &PgPool,
    session_id: Uuid,
    marker_id: Uuid,
    req: &MarkAttendanceRequest,
) -> Result<Attendance, sqlx::Error> {
    let id = Uuid::new_v4();
    let now = Utc::now();

    sqlx::query!(
        "INSERT INTO attendance (id, session_id, user_id, status, marked_by, marked_at, notes)
         VALUES ($1, $2, $3, $4, $5, $6, $7)",
        id, session_id, req.user_id, format!("{:?}", req.status).to_lowercase(),
        Some(marker_id), Some(now), req.notes
    )
    .execute(pool)
    .await?;

    Ok(Attendance {
        id,
        session_id,
        user_id: req.user_id,
        status: req.status,
        marked_by: Some(marker_id),
        marked_at: Some(now),
        notes: req.notes.clone(),
    })
}

pub async fn get_session_attendance(pool: &PgPool, session_id: Uuid) -> Result<Vec<Attendance>, sqlx::Error> {
    let rows = sqlx::query!(
        "SELECT id, session_id, user_id, status, marked_by, marked_at, notes
         FROM attendance WHERE session_id = $1",
        session_id
    )
    .fetch_all(pool)
    .await?;

    Ok(rows.into_iter().map(|r| Attendance {
        id: r.id,
        session_id: r.session_id,
        user_id: r.user_id,
        status: AttendanceStatus::Present,
        marked_by: r.marked_by,
        marked_at: r.marked_at,
        notes: r.notes,
    }).collect())
}

pub async fn get_user_attendance(
    pool: &PgPool,
    user_id: Uuid,
    course_id: Option<Uuid>,
) -> Result<Vec<Attendance>, sqlx::Error> {
    let rows = sqlx::query!(
        "SELECT a.id, a.session_id, a.user_id, a.status, a.marked_by, a.marked_at, a.notes
         FROM attendance a
         JOIN live_sessions s ON a.session_id = s.id
         WHERE a.user_id = $1",
        user_id
    )
    .fetch_all(pool)
    .await?;

    Ok(rows.into_iter().map(|r| Attendance {
        id: r.id,
        session_id: r.session_id,
        user_id: r.user_id,
        status: AttendanceStatus::Present,
        marked_by: r.marked_by,
        marked_at: r.marked_at,
        notes: r.notes,
    }).collect())
}

pub async fn create_recurring_schedule(
    pool: &PgPool,
    req: &CreateRecurringScheduleRequest,
) -> Result<RecurringSchedule, sqlx::Error> {
    let id = Uuid::new_v4();
    let now = Utc::now();

    sqlx::query!(
        "INSERT INTO recurring_schedules (id, course_id, title, instructor_id, day_of_week,
         start_time, end_time, start_date, end_date, provider, repeat_weekly, is_active)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, true)",
        id, req.course_id, req.title, req.instructor_id, req.day_of_week,
        req.start_time, req.end_time, req.start_date, req.end_date,
        format!("{:?}", req.provider).to_lowercase(), req.repeat_weekly.unwrap_or(true)
    )
    .execute(pool)
    .await?;

    let duration = chrono::Duration::minutes(
        chrono::NaiveTime::parse_from_str(&req.end_time, "%H:%M")
            .unwrap()
            .signed_duration_since(
                chrono::NaiveTime::parse_from_str(&req.start_time, "%H:%M").unwrap()
            )
            .num_minutes()
    );

    Ok(RecurringSchedule {
        id,
        course_id: req.course_id,
        title: req.title.clone(),
        instructor_id: req.instructor_id,
        day_of_week: req.day_of_week,
        start_time: req.start_time.clone(),
        end_time: req.end_time.clone(),
        duration_minutes: duration.num_minutes() as i32,
        start_date: req.start_date,
        end_date: req.end_date,
        provider: req.provider,
        repeat_weekly: req.repeat_weekly.unwrap_or(true),
        is_active: true,
    })
}