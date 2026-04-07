// Database operations for attendance system
use super::models::attendance::*;
use super::models::live::{Attendance, AttendanceStatus};
use chrono::{DateTime, Utc};
use sqlx::{PgPool, Row};
use uuid::Uuid;

pub async fn create_qr_session(
    pool: &PgPool,
    session_id: uuid::Uuid,
    code: &str,
    expires_at: DateTime<Utc>,
    max_uses: Option<i32>,
) -> Result<QRCodeSession, sqlx::Error> {
    let id = Uuid::new_v4();
    let now = Utc::now();

    sqlx::query!(
        "INSERT INTO qr_sessions (id, session_id, code, expires_at, is_active, max_uses, used_count)
         VALUES ($1, $2, $3, $4, true, $5, 0)",
        id, session_id, code, expires_at, max_uses
    )
    .execute(pool)
    .await?;

    Ok(QRCodeSession {
        id,
        session_id,
        code: code.to_string(),
        code_url: None,
        expires_at,
        is_active: true,
        max_uses,
        used_count: 0,
        location_radius_meters: None,
    })
}

pub async fn get_active_qr_by_code(
    pool: &PgPool,
    code: &str,
) -> Result<Option<QRCodeSession>, sqlx::Error> {
    let row = sqlx::query!(
        "SELECT id, session_id, code, expires_at, is_active, max_uses, used_count, location_radius_meters
         FROM qr_sessions WHERE code = $1 AND is_active = true AND expires_at > NOW()",
        code
    )
    .fetch_optional(pool)
    .await?;

    Ok(row.map(|r| QRCodeSession {
        id: r.id,
        session_id: r.session_id,
        code: r.code,
        code_url: None,
        expires_at: r.expires_at,
        is_active: r.is_active,
        max_uses: r.max_uses,
        used_count: r.used_count,
        location_radius_meters: r.location_radius_meters,
    }))
}

pub async fn increment_qr_use_count(pool: &PgPool, id: Uuid) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "UPDATE qr_sessions SET used_count = used_count + 1 WHERE id = $1",
        id
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn create_class_attendance(
    pool: &PgPool,
    attendance: &ClassAttendance,
) -> Result<ClassAttendance, sqlx::Error> {
    sqlx::query!(
        "INSERT INTO class_attendance (id, session_id, course_id, user_id, attendance_type, 
         status, marked_at, marked_by, location, device_info, notes)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)",
        attendance.id,
        attendance.session_id,
        attendance.course_id,
        attendance.user_id,
        format!("{:?}", attendance.attendance_type).to_lowercase(),
        format!("{:?}", attendance.status).to_lowercase(),
        attendance.marked_at,
        attendance.marked_by,
        attendance.location,
        attendance.device_info,
        attendance.notes
    )
    .execute(pool)
    .await?;

    Ok(attendance.clone())
}

pub async fn get_course_attendance(
    pool: &PgPool,
    course_id: uuid::Uuid,
    start_date: DateTime<Utc>,
    end_date: DateTime<Utc>,
) -> Result<Vec<ClassAttendance>, sqlx::Error> {
    let rows = sqlx::query!(
        "SELECT id, session_id, course_id, user_id, attendance_type, status, 
                marked_at, marked_by, location, device_info, notes
         FROM class_attendance 
         WHERE course_id = $1 AND marked_at BETWEEN $2 AND $3
         ORDER BY marked_at",
        course_id,
        start_date,
        end_date
    )
    .fetch_all(pool)
    .await?;

    Ok(rows
        .into_iter()
        .map(|r| ClassAttendance {
            id: r.id,
            session_id: r.session_id,
            course_id: r.course_id,
            user_id: r.user_id,
            attendance_type: AttendanceType::Manual,
            status: AttendanceStatus::Present,
            marked_at: r.marked_at,
            marked_by: r.marked_by,
            location: r.location,
            device_info: r.device_info,
            notes: r.notes,
        })
        .collect())
}

pub async fn get_user_class_attendance(
    pool: &PgPool,
    user_id: uuid::Uuid,
    course_id: Option<uuid::Uuid>,
) -> Result<Vec<ClassAttendance>, sqlx::Error> {
    let rows = if let Some(cid) = course_id {
        sqlx::query!(
            "SELECT id, session_id, course_id, user_id, attendance_type, status, 
                    marked_at, marked_by, location, device_info, notes
             FROM class_attendance 
             WHERE user_id = $1 AND course_id = $2
             ORDER BY marked_at DESC",
            user_id,
            cid
        )
        .fetch_all(pool)
        .await?
    } else {
        sqlx::query!(
            "SELECT id, session_id, course_id, user_id, attendance_type, status, 
                    marked_at, marked_by, location, device_info, notes
             FROM class_attendance 
             WHERE user_id = $1
             ORDER BY marked_at DESC",
            user_id
        )
        .fetch_all(pool)
        .await?
    };

    Ok(rows
        .into_iter()
        .map(|r| ClassAttendance {
            id: r.id,
            session_id: r.session_id,
            course_id: r.course_id,
            user_id: r.user_id,
            attendance_type: AttendanceType::Manual,
            status: AttendanceStatus::Present,
            marked_at: r.marked_at,
            marked_by: r.marked_by,
            location: r.location,
            device_info: r.device_info,
            notes: r.notes,
        })
        .collect())
}
