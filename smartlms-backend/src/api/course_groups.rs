// Course Groups API routes - managing student groups within courses
use crate::models::course_group::*;
use crate::db::course_group as group_db;
use crate::tenant::InstitutionCtx;
use axum::{
    extract::{Extension, Json, Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post, put, delete},
    Router,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ListGroupsQuery {
    pub course_id: Option<uuid::Uuid>,
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}

/// List groups for a course
pub async fn list_groups(
    Extension(ctx): Extension<InstitutionCtx>,
    Query(query): Query<ListGroupsQuery>,
) -> Result<Json<CourseGroupListResponse>, (StatusCode, String)> {
    let course_id = query.course_id.ok_or((StatusCode::BAD_REQUEST, "course_id required".to_string()))?;
    let page = query.page.unwrap_or(1);
    let per_page = query.per_page.unwrap_or(20);

    let (groups, total) = group_db::get_groups_by_course(&ctx.db_pool, course_id, page, per_page)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(CourseGroupListResponse {
        groups,
        total,
        page,
        per_page,
    }))
}

/// Get group detail with students, sessions, and assessments
pub async fn get_group_detail(
    Extension(ctx): Extension<InstitutionCtx>,
    Path(group_id): Path<uuid::Uuid>,
) -> Result<Json<CourseGroupDetailResponse>, (StatusCode, String)> {
    let group = group_db::get_group_by_id(&ctx.db_pool, group_id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Group not found".to_string()))?;

    let students = group_db::get_group_students(&ctx.db_pool, group_id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let sessions = group_db::get_group_sessions(&ctx.db_pool, group_id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let assessments = group_db::get_group_assessments(&ctx.db_pool, group_id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(CourseGroupDetailResponse {
        group,
        students,
        sessions,
        assessments,
    }))
}

/// Create a new course group
pub async fn create_group(
    Extension(ctx): Extension<InstitutionCtx>,
    Json(req): Json<CreateCourseGroupRequest>,
) -> Result<Json<CourseGroup>, (StatusCode, String)> {
    // In production, get instructor_id from authenticated user context
    let instructor_id = ctx.id; // Using institution ID as placeholder

    let group = group_db::create_group(&ctx.db_pool, &req, instructor_id)
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

    Ok(Json(group))
}

/// Update a course group
pub async fn update_group(
    Extension(ctx): Extension<InstitutionCtx>,
    Path(group_id): Path<uuid::Uuid>,
    Json(req): Json<UpdateCourseGroupRequest>,
) -> Result<Json<CourseGroup>, (StatusCode, String)> {
    let group = group_db::update_group(&ctx.db_pool, group_id, &req)
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

    Ok(Json(group))
}

/// Delete a course group (soft delete)
pub async fn delete_group(
    Extension(ctx): Extension<InstitutionCtx>,
    Path(group_id): Path<uuid::Uuid>,
) -> Result<StatusCode, (StatusCode, String)> {
    group_db::delete_group(&ctx.db_pool, group_id)
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

    Ok(StatusCode::NO_CONTENT)
}

/// Add a student to a group
pub async fn add_student_to_group(
    Extension(ctx): Extension<InstitutionCtx>,
    Path(group_id): Path<uuid::Uuid>,
    Json(req): Json<AddStudentToGroupRequest>,
) -> Result<Json<CourseGroupEnrollment>, (StatusCode, String)> {
    // In production, get enrolled_by from authenticated user
    let enrolled_by = ctx.id;

    let enrollment = group_db::add_student_to_group(
        &ctx.db_pool,
        group_id,
        req.user_id,
        enrolled_by,
        req.notes,
    )
    .await
    .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

    Ok(Json(enrollment))
}

/// Bulk add students to a group
pub async fn bulk_add_students(
    Extension(ctx): Extension<InstitutionCtx>,
    Path(group_id): Path<uuid::Uuid>,
    Json(req): Json<BulkAddStudentsRequest>,
) -> Result<Json<Vec<CourseGroupEnrollment>>, (StatusCode, String)> {
    let mut enrollments = Vec::new();
    let enrolled_by = ctx.id;

    for user_id in req.user_ids {
        let enrollment = group_db::add_student_to_group(
            &ctx.db_pool,
            group_id,
            user_id,
            enrolled_by,
            None,
        )
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, format!("Failed to add user {}: {}", user_id, e)))?;

        enrollments.push(enrollment);
    }

    Ok(Json(enrollments))
}

/// Remove a student from a group
pub async fn remove_student_from_group(
    Extension(ctx): Extension<InstitutionCtx>,
    Path((group_id, user_id)): Path<(uuid::Uuid, uuid::Uuid)>,
) -> Result<StatusCode, (StatusCode, String)> {
    group_db::remove_student_from_group(&ctx.db_pool, group_id, user_id)
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

    Ok(StatusCode::NO_CONTENT)
}

/// Get groups for a user (student view)
pub async fn get_user_groups(
    Extension(ctx): Extension<InstitutionCtx>,
    Path(user_id): Path<uuid::Uuid>,
    Query(query): Query<ListGroupsQuery>,
) -> Result<Json<Vec<CourseGroup>>, (StatusCode, String)> {
    let groups = group_db::get_user_groups(&ctx.db_pool, user_id, query.course_id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(groups))
}

/// Link a live session to a group
pub async fn link_session_to_group(
    Extension(ctx): Extension<InstitutionCtx>,
    Path(group_id): Path<uuid::Uuid>,
    Json(req): Json<LinkSessionRequest>,
) -> Result<Json<GroupSession>, (StatusCode, String)> {
    let session = group_db::link_session_to_group(
        &ctx.db_pool,
        group_id,
        req.session_id,
        req.is_mandatory.unwrap_or(false),
    )
    .await
    .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

    Ok(Json(session))
}

/// Link an assessment to a group
pub async fn link_assessment_to_group(
    Extension(ctx): Extension<InstitutionCtx>,
    Path(group_id): Path<uuid::Uuid>,
    Json(req): Json<LinkAssessmentRequest>,
) -> Result<Json<GroupAssessment>, (StatusCode, String)> {
    let assessment = group_db::link_assessment_to_group(
        &ctx.db_pool,
        group_id,
        req.assessment_id,
        req.is_group_only.unwrap_or(false),
    )
    .await
    .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

    Ok(Json(assessment))
}

#[derive(Debug, Deserialize)]
pub struct LinkSessionRequest {
    pub session_id: uuid::Uuid,
    pub is_mandatory: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct LinkAssessmentRequest {
    pub assessment_id: uuid::Uuid,
    pub is_group_only: Option<bool>,
}

/// Create course groups router
pub fn course_groups_router() -> Router {
    Router::new()
        // List and create groups
        .route("/", get(list_groups))
        .route("/", post(create_group))
        // Group detail and operations
        .route("/:id", get(get_group_detail))
        .route("/:id", put(update_group))
        .route("/:id", delete(delete_group))
        // Student management
        .route("/:id/students", post(add_student_to_group))
        .route("/:id/students/bulk", post(bulk_add_students))
        .route("/:id/students/:user_id", delete(remove_student_from_group))
        // User's groups
        .route("/user/:user_id", get(get_user_groups))
        // Linking
        .route("/:id/sessions", post(link_session_to_group))
        .route("/:id/assessments", post(link_assessment_to_group))
}
