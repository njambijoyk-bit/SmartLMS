//! /courses endpoints.
//!
//! All routes require a valid JWT (the tenant middleware runs first, then
//! `require_auth` cross-checks the token's institution claim). Role
//! enforcement is performed by `services::course`.

use axum::{
    extract::{Extension, Path, Query},
    http::StatusCode,
    middleware,
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use validator::Validate;

use crate::middleware::auth::{require_auth, AuthUser};
use crate::models::course::{
    CreateCourseRequest, CreateLessonRequest, CreateModuleRequest, UpdateCourseRequest,
};
use crate::services::course::{self, CourseError};
use crate::tenant::InstitutionCtx;

pub fn router() -> Router {
    Router::new()
        .route("/", get(list_courses).post(create_course))
        .route(
            "/:id",
            get(get_course).patch(update_course).delete(archive_course),
        )
        .route("/:id/modules", get(list_modules).post(create_module))
        .route(
            "/:id/modules/:module_id/lessons",
            get(list_lessons).post(create_lesson),
        )
        .route("/:id/enroll", post(enroll))
        .route("/:id/drop", post(drop_course))
        .route("/:id/lessons/:lesson_id/complete", post(complete_lesson))
        .route("/:id/enrollments", get(list_course_enrollments))
        .route_layer(middleware::from_fn(require_auth))
}

#[derive(Debug, Deserialize)]
pub struct ListQuery {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}

#[derive(Debug, Serialize)]
pub struct CourseListResponse<T> {
    pub items: Vec<T>,
    pub total: i64,
    pub page: i64,
    pub per_page: i64,
}

async fn list_courses(
    Extension(ctx): Extension<InstitutionCtx>,
    Extension(user): Extension<AuthUser>,
    Query(q): Query<ListQuery>,
) -> Response {
    let page = q.page.unwrap_or(1).max(1);
    let per_page = q.per_page.unwrap_or(20).clamp(1, 100);
    match course::list_courses(&ctx.db_pool, &user, page, per_page).await {
        Ok((items, total)) => Json(CourseListResponse {
            items,
            total,
            page,
            per_page,
        })
        .into_response(),
        Err(e) => map_error(e),
    }
}

async fn create_course(
    Extension(ctx): Extension<InstitutionCtx>,
    Extension(user): Extension<AuthUser>,
    Json(req): Json<CreateCourseRequest>,
) -> Response {
    if let Err(e) = req.validate() {
        return bad_request(e);
    }
    match course::create_course(&ctx.db_pool, &user, req).await {
        Ok(c) => (StatusCode::CREATED, Json(c)).into_response(),
        Err(e) => map_error(e),
    }
}

async fn get_course(
    Extension(ctx): Extension<InstitutionCtx>,
    Extension(user): Extension<AuthUser>,
    Path(id): Path<uuid::Uuid>,
) -> Response {
    match course::get_course(&ctx.db_pool, &user, id).await {
        Ok(c) => Json(c).into_response(),
        Err(e) => map_error(e),
    }
}

async fn update_course(
    Extension(ctx): Extension<InstitutionCtx>,
    Extension(user): Extension<AuthUser>,
    Path(id): Path<uuid::Uuid>,
    Json(req): Json<UpdateCourseRequest>,
) -> Response {
    if let Err(e) = req.validate() {
        return bad_request(e);
    }
    match course::update_course(&ctx.db_pool, &user, id, req).await {
        Ok(c) => Json(c).into_response(),
        Err(e) => map_error(e),
    }
}

async fn archive_course(
    Extension(ctx): Extension<InstitutionCtx>,
    Extension(user): Extension<AuthUser>,
    Path(id): Path<uuid::Uuid>,
) -> Response {
    match course::archive_course(&ctx.db_pool, &user, id).await {
        Ok(_) => (StatusCode::NO_CONTENT, ()).into_response(),
        Err(e) => map_error(e),
    }
}

async fn list_modules(
    Extension(ctx): Extension<InstitutionCtx>,
    Path(course_id): Path<uuid::Uuid>,
) -> Response {
    match course::list_modules(&ctx.db_pool, course_id).await {
        Ok(items) => Json(items).into_response(),
        Err(e) => map_error(e),
    }
}

async fn create_module(
    Extension(ctx): Extension<InstitutionCtx>,
    Extension(user): Extension<AuthUser>,
    Path(course_id): Path<uuid::Uuid>,
    Json(req): Json<CreateModuleRequest>,
) -> Response {
    if let Err(e) = req.validate() {
        return bad_request(e);
    }
    match course::create_module(&ctx.db_pool, &user, course_id, req).await {
        Ok(m) => (StatusCode::CREATED, Json(m)).into_response(),
        Err(e) => map_error(e),
    }
}

async fn list_lessons(
    Extension(ctx): Extension<InstitutionCtx>,
    Path((_course_id, module_id)): Path<(uuid::Uuid, uuid::Uuid)>,
) -> Response {
    match course::list_lessons(&ctx.db_pool, module_id).await {
        Ok(items) => Json(items).into_response(),
        Err(e) => map_error(e),
    }
}

async fn create_lesson(
    Extension(ctx): Extension<InstitutionCtx>,
    Extension(user): Extension<AuthUser>,
    Path((course_id, module_id)): Path<(uuid::Uuid, uuid::Uuid)>,
    Json(req): Json<CreateLessonRequest>,
) -> Response {
    if let Err(e) = req.validate() {
        return bad_request(e);
    }
    match course::create_lesson(&ctx.db_pool, &user, course_id, module_id, req).await {
        Ok(l) => (StatusCode::CREATED, Json(l)).into_response(),
        Err(e) => map_error(e),
    }
}

async fn enroll(
    Extension(ctx): Extension<InstitutionCtx>,
    Extension(user): Extension<AuthUser>,
    Path(id): Path<uuid::Uuid>,
) -> Response {
    match course::enroll_self(&ctx.db_pool, &user, id).await {
        Ok(e) => (StatusCode::CREATED, Json(e)).into_response(),
        Err(e) => map_error(e),
    }
}

async fn drop_course(
    Extension(ctx): Extension<InstitutionCtx>,
    Extension(user): Extension<AuthUser>,
    Path(id): Path<uuid::Uuid>,
) -> Response {
    match course::drop_self(&ctx.db_pool, &user, id).await {
        Ok(_) => (StatusCode::NO_CONTENT, ()).into_response(),
        Err(e) => map_error(e),
    }
}

async fn complete_lesson(
    Extension(ctx): Extension<InstitutionCtx>,
    Extension(user): Extension<AuthUser>,
    Path((course_id, lesson_id)): Path<(uuid::Uuid, uuid::Uuid)>,
) -> Response {
    match course::complete_lesson(&ctx.db_pool, &user, course_id, lesson_id).await {
        Ok(pct) => Json(json!({ "progress_pct": pct })).into_response(),
        Err(e) => map_error(e),
    }
}

async fn list_course_enrollments(
    Extension(ctx): Extension<InstitutionCtx>,
    Extension(user): Extension<AuthUser>,
    Path(course_id): Path<uuid::Uuid>,
    Query(q): Query<ListQuery>,
) -> Response {
    let page = q.page.unwrap_or(1).max(1);
    let per_page = q.per_page.unwrap_or(50).clamp(1, 200);
    match course::list_course_enrollments(&ctx.db_pool, &user, course_id, page, per_page).await {
        Ok((items, total)) => Json(CourseListResponse {
            items,
            total,
            page,
            per_page,
        })
        .into_response(),
        Err(e) => map_error(e),
    }
}

fn bad_request<E: std::fmt::Display>(e: E) -> Response {
    (
        StatusCode::BAD_REQUEST,
        Json(json!({ "error": "validation_failed", "detail": e.to_string() })),
    )
        .into_response()
}

fn map_error(e: CourseError) -> Response {
    let (status, code) = match &e {
        CourseError::NotFound => (StatusCode::NOT_FOUND, "not_found"),
        CourseError::Forbidden => (StatusCode::FORBIDDEN, "forbidden"),
        CourseError::SlugTaken => (StatusCode::CONFLICT, "slug_taken"),
        CourseError::NotPublished => (StatusCode::CONFLICT, "not_published"),
        CourseError::Db(err) => {
            tracing::error!(error = %err, "course db error");
            (StatusCode::INTERNAL_SERVER_ERROR, "internal_error")
        }
    };
    (status, Json(json!({ "error": code }))).into_response()
}
