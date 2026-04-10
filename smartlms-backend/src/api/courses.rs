// Courses API routes
use axum::{
    extract::{State, Json, Path, Query, Extension},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post, put, delete},
    Router,
};
use crate::models::course::*;
use crate::services::courses as course_service;
use crate::tenant::InstitutionCtx;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ListCoursesQuery {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
    pub category: Option<String>,
    pub search: Option<String>,
}

/// List courses
pub async fn list_courses(
    Extension(ctx): Extension<InstitutionCtx>,
    Query(query): Query<ListCoursesQuery>,
) -> Result<Json<CourseListResponse>, (StatusCode, String)> {
    let page = query.page.unwrap_or(1);
    let per_page = query.per_page.unwrap_or(20).min(100);
    
    let response = if let Some(search) = query.search {
        course_service::search_courses(&ctx.db_pool, &search, page, per_page).await
    } else {
        course_service::list_courses(&ctx.db_pool, page, per_page, query.category.as_deref(), None, None).await
    }.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;
    
    Ok(Json(response))
}

/// Get course detail
pub async fn get_course(
    Extension(ctx): Extension<InstitutionCtx>,
    Path(course_id): Path<uuid::Uuid>,
) -> Result<Json<CourseDetailResponse>, (StatusCode, String)> {
    let detail = course_service::get_course_detail(&ctx.db_pool, course_id).await
        .map_err(|e| (StatusCode::NOT_FOUND, e))?;
    Ok(Json(detail))
}

/// Create course (instructor/admin only)
pub async fn create_course(
    Extension(ctx): Extension<InstitutionCtx>,
    State(pool): State<sqlx::PgPool>,
    Json(req): Json<CreateCourseRequest>,
) -> Result<Json<Course>, (StatusCode, String)> {
    // Check permission - for now use first user as instructor
    let course = course_service::create_course(&ctx.db_pool, ctx.id, &req).await
        .map_err(|e| (StatusCode::BAD_REQUEST, e))?;
    Ok(Json(course))
}

/// Update course
pub async fn update_course(
    Extension(ctx): Extension<InstitutionCtx>,
    Path(course_id): Path<uuid::Uuid>,
    Json(req): Json<UpdateCourseRequest>,
) -> Result<Json<Course>, (StatusCode, String)> {
    let course = course_service::update_course(&ctx.db_pool, course_id, &req).await
        .map_err(|e| (StatusCode::BAD_REQUEST, e))?;
    Ok(Json(course))
}

/// Publish course
pub async fn publish_course(
    Extension(ctx): Extension<InstitutionCtx>,
    Path(course_id): Path<uuid::Uuid>,
) -> Result<Json<Course>, (StatusCode, String)> {
    let course = course_service::publish_course(&ctx.db_pool, course_id).await
        .map_err(|e| (StatusCode::BAD_REQUEST, e))?;
    Ok(Json(course))
}

/// Enroll in course
pub async fn enroll(
    Extension(ctx): Extension<InstitutionCtx>,
    Path(course_id): Path<uuid::Uuid>,
) -> Result<Json<Enrollment>, (StatusCode, String)> {
    let enrollment = course_service::enroll_user(&ctx.db_pool, ctx.id, course_id).await
        .map_err(|e| (StatusCode::BAD_REQUEST, e))?;
    Ok(Json(enrollment))
}

/// Get course progress
pub async fn get_progress(
    Extension(ctx): Extension<InstitutionCtx>,
    Path(course_id): Path<uuid::Uuid>,
) -> Result<Json<CourseProgress>, (StatusCode, String)> {
    let progress = course_service::get_progress(&ctx.db_pool, ctx.id, course_id).await
        .map_err(|e| (StatusCode::NOT_FOUND, e))?;
    Ok(Json(progress))
}

/// Create module
pub async fn create_module(
    Extension(ctx): Extension<InstitutionCtx>,
    Json(req): Json<CreateModuleRequest>,
) -> Result<Json<Module>, (StatusCode, String)> {
    let module = course_service::create_module(&ctx.db_pool, &req).await
        .map_err(|e| (StatusCode::BAD_REQUEST, e))?;
    Ok(Json(module))
}

/// Create lesson
pub async fn create_lesson(
    Extension(ctx): Extension<InstitutionCtx>,
    Json(req): Json<CreateLessonRequest>,
) -> Result<Json<Lesson>, (StatusCode, String)> {
    let lesson = course_service::create_lesson(&ctx.db_pool, &req).await
        .map_err(|e| (StatusCode::BAD_REQUEST, e))?;
    Ok(Json(lesson))
}

/// Mark lesson complete
pub async fn complete_lesson(
    Extension(ctx): Extension<InstitutionCtx>,
    Path(lesson_id): Path<uuid::Uuid>,
) -> Result<Json<CourseProgress>, (StatusCode, String)> {
    let progress = course_service::complete_lesson(&ctx.db_pool, ctx.id, lesson_id).await
        .map_err(|e| (StatusCode::BAD_REQUEST, e))?;
    Ok(Json(progress))
}

/// Create courses router
pub fn courses_router() -> Router {
    Router::new()
        .route("/", get(list_courses))
        .route("/", post(create_course))
        .route("/:id", get(get_course))
        .route("/:id", put(update_course))
        .route("/:id/publish", post(publish_course))
        .route("/:id/enroll", post(enroll))
        .route("/:id/progress", get(get_progress))
        .route("/modules", post(create_module))
        .route("/lessons", post(create_lesson))
        .route("/lessons/:id/complete", post(complete_lesson))
}