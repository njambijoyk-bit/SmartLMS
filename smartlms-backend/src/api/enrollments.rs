// Enrollments API routes placeholder
use crate::models::live::*;
use crate::services::live as live_service;
use crate::tenant::InstitutionCtx;
use axum::{
    extract::{Extension, Json, Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post, put},
    Router,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ListSessionsQuery {
    pub course_id: Option<uuid::Uuid>,
    pub status: Option<SessionStatus>,
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}

/// List live sessions
pub async fn list_sessions(
    Extension(ctx): Extension<InstitutionCtx>,
    Query(query): Query<ListSessionsQuery>,
) -> Result<Json<SessionListResponse>, (StatusCode, String)> {
    let page = query.page.unwrap_or(1);
    let per_page = query.per_page.unwrap_or(20).min(100);

    let response =
        live_service::list_sessions(&ctx.db_pool, query.course_id, query.status, page, per_page)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;

    Ok(Json(response))
}

/// Get session detail
pub async fn get_session(
    Extension(ctx): Extension<InstitutionCtx>,
    Path(session_id): Path<uuid::Uuid>,
) -> Result<Json<SessionDetailResponse>, (StatusCode, String)> {
    let detail = live_service::get_session(&ctx.db_pool, session_id)
        .await
        .map_err(|e| (StatusCode::NOT_FOUND, e))?;
    Ok(Json(detail))
}

/// Create live session
pub async fn create_session(
    Extension(ctx): Extension<InstitutionCtx>,
    Json(req): Json<CreateSessionRequest>,
) -> Result<Json<LiveSession>, (StatusCode, String)> {
    let session = live_service::create_session(&ctx.db_pool, ctx.id, &req)
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, e))?;
    Ok(Json(session))
}

/// Update session
pub async fn update_session(
    Extension(ctx): Extension<InstitutionCtx>,
    Path(session_id): Path<uuid::Uuid>,
    Json(req): Json<UpdateSessionRequest>,
) -> Result<Json<LiveSession>, (StatusCode, String)> {
    let session = live_service::update_session(&ctx.db_pool, session_id, &req)
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, e))?;
    Ok(Json(session))
}

/// Start session
pub async fn start_session(
    Extension(ctx): Extension<InstitutionCtx>,
    Path(session_id): Path<uuid::Uuid>,
) -> Result<Json<LiveSession>, (StatusCode, String)> {
    let session = live_service::start_session(&ctx.db_pool, session_id)
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, e))?;
    Ok(Json(session))
}

/// End session
pub async fn end_session(
    Extension(ctx): Extension<InstitutionCtx>,
    Path(session_id): Path<uuid::Uuid>,
) -> Result<Json<LiveSession>, (StatusCode, String)> {
    let session = live_service::end_session(&ctx.db_pool, session_id)
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, e))?;
    Ok(Json(session))
}

/// Cancel session
pub async fn cancel_session(
    Extension(ctx): Extension<InstitutionCtx>,
    Path(session_id): Path<uuid::Uuid>,
) -> Result<Json<LiveSession>, (StatusCode, String)> {
    let session = live_service::cancel_session(&ctx.db_pool, session_id)
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, e))?;
    Ok(Json(session))
}

/// Mark attendance
pub async fn mark_attendance(
    Extension(ctx): Extension<InstitutionCtx>,
    Path(session_id): Path<uuid::Uuid>,
    Json(req): Json<MarkAttendanceRequest>,
) -> Result<Json<Attendance>, (StatusCode, String)> {
    let attendance = live_service::mark_attendance(&ctx.db_pool, session_id, ctx.id, &req)
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, e))?;
    Ok(Json(attendance))
}

/// Bulk mark attendance
pub async fn bulk_mark_attendance(
    Extension(ctx): Extension<InstitutionCtx>,
    Path(session_id): Path<uuid::Uuid>,
    Json(req): Json<BulkAttendanceRequest>,
) -> Result<Json<Vec<Attendance>>, (StatusCode, String)> {
    let attendances = live_service::bulk_mark_attendance(&ctx.db_pool, session_id, ctx.id, &req)
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, e))?;
    Ok(Json(attendances))
}

/// Get session attendance
pub async fn get_session_attendance(
    Extension(ctx): Extension<InstitutionCtx>,
    Path(session_id): Path<uuid::Uuid>,
) -> Result<Json<Vec<Attendance>>, (StatusCode, String)> {
    let attendance = live_service::get_session_attendance(&ctx.db_pool, session_id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;
    Ok(Json(attendance))
}

/// Get user attendance
pub async fn get_user_attendance(
    Extension(ctx): Extension<InstitutionCtx>,
    Query(query): Query<GetUserAttendanceQuery>,
) -> Result<Json<Vec<Attendance>>, (StatusCode, String)> {
    let attendance = live_service::get_user_attendance(&ctx.db_pool, ctx.id, query.course_id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;
    Ok(Json(attendance))
}

#[derive(Deserialize)]
pub struct GetUserAttendanceQuery {
    pub course_id: Option<uuid::Uuid>,
}

/// Create recurring schedule
pub async fn create_recurring_schedule(
    Extension(ctx): Extension<InstitutionCtx>,
    Json(req): Json<CreateRecurringScheduleRequest>,
) -> Result<Json<RecurringSchedule>, (StatusCode, String)> {
    let schedule = live_service::create_recurring_schedule(&ctx.db_pool, &req)
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, e))?;
    Ok(Json(schedule))
}

/// Create enrollments router
pub fn enrollments_router() -> Router {
    Router::new()
        // Sessions
        .route("/sessions", get(list_sessions))
        .route("/sessions", post(create_session))
        .route("/sessions/:id", get(get_session))
        .route("/sessions/:id", put(update_session))
        .route("/sessions/:id/start", post(start_session))
        .route("/sessions/:id/end", post(end_session))
        .route("/sessions/:id/cancel", post(cancel_session))
        // Attendance
        .route("/sessions/:id/attendance", get(get_session_attendance))
        .route("/sessions/:id/attendance", post(mark_attendance))
        .route("/sessions/:id/attendance/bulk", post(bulk_mark_attendance))
        // User attendance
        .route("/my-attendance", get(get_user_attendance))
        // Recurring
        .route("/recurring", post(create_recurring_schedule))
}
