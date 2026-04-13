// Timetable API — Physical class timetables and exam scheduling
use crate::services::timetable::*;
use axum::{
    extract::{Extension, Json, Path, Query, State},
    http::{HeaderMap, HeaderValue, StatusCode},
    routing::{get, post, put},
    Router,
};
use chrono::NaiveTime;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

// ============================================
// REQUEST/RESPONSE STRUCTURES
// ============================================

#[derive(Debug, Deserialize)]
pub struct CreateSlotRequestQuery {
    pub institution_id: Uuid,
    pub course_id: Uuid,
    pub instructor_id: Uuid,
    pub room_id: Uuid,
    pub day_of_week: String, // monday, tuesday, etc.
    pub start_time: String,  // HH:MM format
    pub end_time: String,    // HH:MM format
    pub cohort_ids: Vec<Uuid>,
    pub academic_year: String,
    pub semester: u8,
    pub slot_type: String, // lecture, tutorial, lab, seminar, workshop
}

#[derive(Debug, Deserialize)]
pub struct StudentTimetableParams {
    pub academic_year: String,
    pub semester: u8,
}

#[derive(Debug, Deserialize)]
pub struct ScheduleExamBody {
    pub exam_paper_id: Uuid,
    pub room_id: Uuid,
    pub exam_date: String, // ISO 8601 date
    pub start_time: String, // HH:MM
    pub end_time: String,   // HH:MM
    pub invigilator_ids: Vec<Uuid>,
}

#[derive(Debug, Deserialize)]
pub struct RoomAvailabilityParams {
    pub room_id: Uuid,
    pub day: String, // monday, tuesday, etc.
    pub start_time: String,
    pub end_time: String,
}

// ============================================
// API HANDLERS
// ============================================

/// Get student's personal timetable
pub async fn get_student_timetable_handler(
    Extension(user): Extension<crate::models::user::User>,
    State(pool): State<PgPool>,
    Query(params): Query<StudentTimetableParams>,
) -> Result<Json<StudentTimetable>, (StatusCode, String)> {
    let timetable = TimetableService::get_student_timetable(
        &pool,
        user.id,
        &params.academic_year,
        params.semester,
    )
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    Ok(Json(timetable))
}

/// Create a timetable slot (admin)
pub async fn create_slot_handler(
    Extension(user): Extension<crate::models::user::User>,
    State(pool): State<PgPool>,
    Json(req): Json<CreateSlotRequest>,
) -> Result<Json<TimetableSlot>, (StatusCode, String)> {
    if user.role != "admin" && user.role != "registrar" {
        return Err((StatusCode::FORBIDDEN, "Insufficient permissions".to_string()));
    }
    
    let (slot, conflicts) = TimetableService::create_slot(&pool, req)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    // TODO: Return conflicts in response if any
    let _ = conflicts;
    
    Ok(Json(slot))
}

/// Publish timetable (admin)
pub async fn publish_timetable_handler(
    Extension(user): Extension<crate::models::user::User>,
    State(pool): State<PgPool>,
    Json(req): Json<PublishTimetableRequest>,
) -> Result<Json<PublishResponse>, (StatusCode, String)> {
    if user.role != "admin" && user.role != "registrar" {
        return Err((StatusCode::FORBIDDEN, "Insufficient permissions".to_string()));
    }
    
    let updated_count = TimetableService::publish_timetable(
        &pool,
        req.institution_id,
        &req.academic_year,
        req.semester,
    )
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    Ok(Json(PublishResponse { 
        updated_count,
        message: "Timetable published successfully".to_string(),
    }))
}

#[derive(Debug, Deserialize)]
pub struct PublishTimetableRequest {
    pub institution_id: Uuid,
    pub academic_year: String,
    pub semester: u8,
}

#[derive(Debug, Serialize)]
pub struct PublishResponse {
    pub updated_count: i64,
    pub message: String,
}

/// Schedule an exam (admin/exams officer)
pub async fn schedule_exam_handler(
    Extension(user): Extension<crate::models::user::User>,
    State(pool): State<PgPool>,
    Json(req): Json<ScheduleExamBody>,
) -> Result<Json<ExamTimetableEntry>, (StatusCode, String)> {
    if user.role != "admin" && user.role != "exams_officer" {
        return Err((StatusCode::FORBIDDEN, "Insufficient permissions".to_string()));
    }
    
    // Parse date and time
    let exam_date = chrono::DateTime::parse_from_rfc3339(&req.exam_date)
        .map_err(|e| (StatusCode::BAD_REQUEST, format!("Invalid date format: {}", e)))?;
    
    let start_time = NaiveTime::parse_from_str(&req.start_time, "%H:%M")
        .map_err(|e| (StatusCode::BAD_REQUEST, format!("Invalid start time: {}", e)))?;
    
    let end_time = NaiveTime::parse_from_str(&req.end_time, "%H:%M")
        .map_err(|e| (StatusCode::BAD_REQUEST, format!("Invalid end time: {}", e)))?;
    
    let (entry, conflicts) = TimetableService::schedule_exam(
        &pool,
        user.institution_id.unwrap_or_default(),
        req.exam_paper_id,
        req.room_id,
        exam_date.with_timezone(&chrono::Utc),
        start_time,
        end_time,
        req.invigilator_ids,
    )
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    let _ = conflicts;
    
    Ok(Json(entry))
}

/// Check room availability
pub async fn check_room_availability_handler(
    State(pool): State<PgPool>,
    Query(params): Query<RoomAvailabilityParams>,
) -> Result<Json<RoomAvailability>, (StatusCode, String)> {
    let day = match params.day.to_lowercase().as_str() {
        "monday" => DayOfWeek::Monday,
        "tuesday" => DayOfWeek::Tuesday,
        "wednesday" => DayOfWeek::Wednesday,
        "thursday" => DayOfWeek::Thursday,
        "friday" => DayOfWeek::Friday,
        "saturday" => DayOfWeek::Saturday,
        "sunday" => DayOfWeek::Sunday,
        _ => return Err((StatusCode::BAD_REQUEST, "Invalid day of week".to_string())),
    };
    
    let start_time = NaiveTime::parse_from_str(&params.start_time, "%H:%M")
        .map_err(|e| (StatusCode::BAD_REQUEST, format!("Invalid start time: {}", e)))?;
    
    let end_time = NaiveTime::parse_from_str(&params.end_time, "%H:%M")
        .map_err(|e| (StatusCode::BAD_REQUEST, format!("Invalid end time: {}", e)))?;
    
    let availability = TimetableService::check_room_availability(
        &pool,
        Uuid::nil(), // TODO: Get from auth context
        params.room_id,
        day,
        start_time,
        end_time,
    )
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    Ok(Json(availability))
}

/// Export student timetable as iCal
pub async fn export_ical_handler(
    Extension(user): Extension<crate::models::user::User>,
    State(pool): State<PgPool>,
    Query(params): Query<StudentTimetableParams>,
) -> Result<(HeaderMap, String), (StatusCode, String)> {
    let export = TimetableService::export_ical(
        &pool,
        user.id,
        &params.academic_year,
        params.semester,
    )
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    let mut headers = HeaderMap::new();
    headers.insert(
        "Content-Type",
        HeaderValue::from_static("text/calendar"),
    );
    headers.insert(
        "Content-Disposition",
        HeaderValue::from_str("attachment; filename=\"timetable.ics\"").unwrap(),
    );
    
    Ok((headers, export.ical_content))
}

/// List all rooms for an institution
pub async fn list_rooms_handler(
    Extension(user): Extension<crate::models::user::User>,
    State(pool): State<PgPool>,
) -> Result<Json<Vec<Room>>, (StatusCode, String)> {
    let rooms = TimetableService::list_rooms(
        &pool,
        user.institution_id.unwrap_or_default(),
    )
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    Ok(Json(rooms))
}

/// Create a new room (admin)
pub async fn create_room_handler(
    Extension(user): Extension<crate::models::user::User>,
    State(pool): State<PgPool>,
    Json(req): Json<CreateRoomRequest>,
) -> Result<Json<Room>, (StatusCode, String)> {
    if user.role != "admin" {
        return Err((StatusCode::FORBIDDEN, "Admin access required".to_string()));
    }
    
    let room = TimetableService::create_room(
        &pool,
        req.institution_id,
        &req.name,
        req.building,
        req.capacity,
        req.equipment_tags,
    )
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    Ok(Json(room))
}

#[derive(Debug, Deserialize)]
pub struct CreateRoomRequest {
    pub institution_id: Uuid,
    pub name: String,
    pub building: Option<String>,
    pub capacity: i32,
    pub equipment_tags: Vec<String>,
}

// ============================================
// ROUTER CREATION
// ============================================

pub fn timetable_router() -> Router {
    Router::new()
        .route("/my-timetable", get(get_student_timetable_handler))
        .route("/slots/create", post(create_slot_handler))
        .route("/publish", post(publish_timetable_handler))
        .route("/exams/schedule", post(schedule_exam_handler))
        .route("/rooms/availability", get(check_room_availability_handler))
        .route("/export-ical", get(export_ical_handler))
        .route("/rooms", get(list_rooms_handler))
        .route("/rooms/create", post(create_room_handler))
}
