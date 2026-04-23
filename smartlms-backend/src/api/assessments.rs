//! /assessments endpoints.
//!
//! Mounted under `/api/assessments` and `/api/courses/:id/assessments` —
//! the nested path provides the course context for listing / creation,
//! the flat path is the handle for an individual assessment + its attempts.

use axum::{
    extract::{Extension, Path, Query},
    http::StatusCode,
    middleware,
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use serde::Deserialize;
use serde_json::json;
use validator::Validate;

use crate::middleware::auth::{require_auth, AuthUser};
use crate::models::assessment::{
    AddQuestionToAssessmentRequest, CreateAssessmentRequest, CreateQuestionRequest,
    SubmitAnswersRequest, UpdateAssessmentRequest,
};
use crate::services::assessment::{self, AssessmentError};
use crate::tenant::InstitutionCtx;

pub fn assessments_router() -> Router {
    Router::new()
        .route("/:id", axum::routing::patch(update))
        .route(
            "/:id/questions",
            post(add_question).delete(remove_question_fallback),
        )
        .route(
            "/:id/questions/:qid",
            axum::routing::delete(remove_question),
        )
        .route("/:id/attempts", post(start_attempt))
        .route("/attempts/:aid/submit", post(submit_attempt))
        .route("/attempts/my", get(my_attempts))
        .route_layer(middleware::from_fn(require_auth))
}

pub fn questions_router() -> Router {
    Router::new()
        .route("/", get(list_my_questions).post(create_question))
        .route_layer(middleware::from_fn(require_auth))
}

/// Sub-router mounted from `api::courses` as `/courses/:id/assessments`.
pub fn course_assessments_router() -> Router {
    Router::new()
        .route("/", get(list_for_course).post(create_for_course))
        .route_layer(middleware::from_fn(require_auth))
}

#[derive(Debug, Deserialize)]
pub struct ListQuery {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}

// ---------------------------------------------------------------------------
// Questions
// ---------------------------------------------------------------------------

async fn create_question(
    Extension(ctx): Extension<InstitutionCtx>,
    Extension(user): Extension<AuthUser>,
    Json(req): Json<CreateQuestionRequest>,
) -> Response {
    if let Err(e) = req.validate() {
        return bad_request(e);
    }
    match assessment::create_question(&ctx.db_pool, &user, req).await {
        Ok(q) => (StatusCode::CREATED, Json(q)).into_response(),
        Err(e) => map_error(e),
    }
}

async fn list_my_questions(
    Extension(ctx): Extension<InstitutionCtx>,
    Extension(user): Extension<AuthUser>,
    Query(q): Query<ListQuery>,
) -> Response {
    let page = q.page.unwrap_or(1).max(1);
    let per_page = q.per_page.unwrap_or(50).clamp(1, 200);
    match assessment::list_my_questions(&ctx.db_pool, &user, page, per_page).await {
        Ok((items, total)) => Json(json!({
            "items": items,
            "total": total,
            "page": page,
            "per_page": per_page,
        }))
        .into_response(),
        Err(e) => map_error(e),
    }
}

// ---------------------------------------------------------------------------
// Assessments (course-scoped)
// ---------------------------------------------------------------------------

async fn create_for_course(
    Extension(ctx): Extension<InstitutionCtx>,
    Extension(user): Extension<AuthUser>,
    Path(course_id): Path<uuid::Uuid>,
    Json(req): Json<CreateAssessmentRequest>,
) -> Response {
    if let Err(e) = req.validate() {
        return bad_request(e);
    }
    match assessment::create_assessment(&ctx.db_pool, &user, course_id, req).await {
        Ok(a) => (StatusCode::CREATED, Json(a)).into_response(),
        Err(e) => map_error(e),
    }
}

async fn list_for_course(
    Extension(ctx): Extension<InstitutionCtx>,
    Extension(user): Extension<AuthUser>,
    Path(course_id): Path<uuid::Uuid>,
) -> Response {
    match assessment::list_for_course(&ctx.db_pool, &user, course_id).await {
        Ok(items) => Json(items).into_response(),
        Err(e) => map_error(e),
    }
}

async fn update(
    Extension(ctx): Extension<InstitutionCtx>,
    Extension(user): Extension<AuthUser>,
    Path(id): Path<uuid::Uuid>,
    Json(req): Json<UpdateAssessmentRequest>,
) -> Response {
    if let Err(e) = req.validate() {
        return bad_request(e);
    }
    match assessment::update_assessment(&ctx.db_pool, &user, id, req).await {
        Ok(a) => Json(a).into_response(),
        Err(e) => map_error(e),
    }
}

async fn add_question(
    Extension(ctx): Extension<InstitutionCtx>,
    Extension(user): Extension<AuthUser>,
    Path(id): Path<uuid::Uuid>,
    Json(req): Json<AddQuestionToAssessmentRequest>,
) -> Response {
    if let Err(e) = req.validate() {
        return bad_request(e);
    }
    match assessment::add_question_to_assessment(&ctx.db_pool, &user, id, req).await {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(e) => map_error(e),
    }
}

async fn remove_question(
    Extension(ctx): Extension<InstitutionCtx>,
    Extension(user): Extension<AuthUser>,
    Path((id, qid)): Path<(uuid::Uuid, uuid::Uuid)>,
) -> Response {
    match assessment::remove_question_from_assessment(&ctx.db_pool, &user, id, qid).await {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(e) => map_error(e),
    }
}

/// Axum method routers don't mix GET + DELETE at the same path without
/// an explicit fallback — DELETE without a question id isn't meaningful
/// and returns 405.
async fn remove_question_fallback() -> Response {
    StatusCode::METHOD_NOT_ALLOWED.into_response()
}

// ---------------------------------------------------------------------------
// Attempts
// ---------------------------------------------------------------------------

async fn start_attempt(
    Extension(ctx): Extension<InstitutionCtx>,
    Extension(user): Extension<AuthUser>,
    Path(id): Path<uuid::Uuid>,
) -> Response {
    match assessment::start_attempt(&ctx.db_pool, &user, id).await {
        Ok(a) => (StatusCode::CREATED, Json(a)).into_response(),
        Err(e) => map_error(e),
    }
}

async fn submit_attempt(
    Extension(ctx): Extension<InstitutionCtx>,
    Extension(user): Extension<AuthUser>,
    Path(attempt_id): Path<uuid::Uuid>,
    Json(req): Json<SubmitAnswersRequest>,
) -> Response {
    if let Err(e) = req.validate() {
        return bad_request(e);
    }
    match assessment::submit_attempt(&ctx.db_pool, &user, attempt_id, req.answers).await {
        Ok(g) => Json(json!({
            "attempt": g.attempt,
            "answers": g.answers,
        }))
        .into_response(),
        Err(e) => map_error(e),
    }
}

async fn my_attempts(
    Extension(ctx): Extension<InstitutionCtx>,
    Extension(user): Extension<AuthUser>,
) -> Response {
    match assessment::list_my_attempts(&ctx.db_pool, &user).await {
        Ok(items) => Json(items).into_response(),
        Err(e) => map_error(e),
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn bad_request<E: std::fmt::Display>(e: E) -> Response {
    (
        StatusCode::BAD_REQUEST,
        Json(json!({ "error": "validation_failed", "detail": e.to_string() })),
    )
        .into_response()
}

fn map_error(e: AssessmentError) -> Response {
    let (status, code) = match &e {
        AssessmentError::NotFound => (StatusCode::NOT_FOUND, "not_found"),
        AssessmentError::Forbidden => (StatusCode::FORBIDDEN, "forbidden"),
        AssessmentError::CourseUnavailable => (StatusCode::NOT_FOUND, "course_unavailable"),
        AssessmentError::NotPublished => (StatusCode::CONFLICT, "not_published"),
        AssessmentError::NoAttemptsLeft => (StatusCode::CONFLICT, "no_attempts_left"),
        AssessmentError::NoOpenAttempt => (StatusCode::CONFLICT, "no_open_attempt"),
        AssessmentError::NotEnrolled => (StatusCode::FORBIDDEN, "not_enrolled"),
        AssessmentError::Db(err) => {
            tracing::error!(error = %err, "assessment db error");
            (StatusCode::INTERNAL_SERVER_ERROR, "internal_error")
        }
    };
    (status, Json(json!({ "error": code }))).into_response()
}
