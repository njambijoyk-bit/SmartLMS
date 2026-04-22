// Assessments API routes
use crate::models::assessment::*;
use crate::services::assessments as assessment_service;
use crate::tenant::InstitutionCtx;
use axum::{
    extract::{Extension, Json, Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, post, put},
    Router,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ListAssessmentsQuery {
    pub course_id: Option<uuid::Uuid>,
    pub course_group_id: Option<uuid::Uuid>,
    pub assessment_type: Option<AssessmentType>,
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct AddQuestionToAssessmentRequest {
    pub question_id: uuid::Uuid,
    pub points: i32,
}

/// List question banks
pub async fn list_question_banks(
    Extension(ctx): Extension<InstitutionCtx>,
    Query(query): Query<ListAssessmentsQuery>,
) -> Result<Json<Vec<QuestionBank>>, (StatusCode, String)> {
    let (banks, _) = assessment_service::get_question_banks(&ctx.db_pool, query.course_id, 1, 20)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;
    Ok(Json(banks))
}

/// Create question bank
pub async fn create_question_bank(
    Extension(ctx): Extension<InstitutionCtx>,
    Json(req): Json<CreateQuestionBankRequest>,
) -> Result<Json<QuestionBank>, (StatusCode, String)> {
    let bank = assessment_service::create_question_bank(&ctx.db_pool, ctx.id, &req)
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, e))?;
    Ok(Json(bank))
}

/// Create question
pub async fn create_question(
    Extension(ctx): Extension<InstitutionCtx>,
    Json(req): Json<CreateQuestionRequest>,
) -> Result<Json<Question>, (StatusCode, String)> {
    let question = assessment_service::create_question(&ctx.db_pool, &req)
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, e))?;
    Ok(Json(question))
}

/// List assessments with filtering by course and group
pub async fn list_assessments(
    Extension(ctx): Extension<InstitutionCtx>,
    Query(query): Query<ListAssessmentsQuery>,
) -> Result<Json<Vec<Assessment>>, (StatusCode, String)> {
    let page = query.page.unwrap_or(1);
    let per_page = query.per_page.unwrap_or(20);
    
    let (assessments, _) = assessment_service::list_assessments(
        &ctx.db_pool,
        query.course_id,
        query.course_group_id,
        page,
        per_page,
    )
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;
    
    Ok(Json(assessments))
}

/// Get assessment detail
pub async fn get_assessment(
    Extension(ctx): Extension<InstitutionCtx>,
    Path(assessment_id): Path<uuid::Uuid>,
) -> Result<Json<AssessmentDetailResponse>, (StatusCode, String)> {
    let detail = assessment_service::get_assessment_detail(&ctx.db_pool, assessment_id)
        .await
        .map_err(|e| (StatusCode::NOT_FOUND, e))?;
    Ok(Json(detail))
}

/// Create assessment
pub async fn create_assessment(
    Extension(ctx): Extension<InstitutionCtx>,
    Json(req): Json<CreateAssessmentRequest>,
) -> Result<Json<Assessment>, (StatusCode, String)> {
    let assessment = assessment_service::create_assessment(&ctx.db_pool, &req)
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, e))?;
    Ok(Json(assessment))
}

/// Update assessment
pub async fn update_assessment(
    Extension(ctx): Extension<InstitutionCtx>,
    Path(assessment_id): Path<uuid::Uuid>,
    Json(req): Json<UpdateAssessmentRequest>,
) -> Result<Json<Assessment>, (StatusCode, String)> {
    let assessment = assessment_service::update_assessment(&ctx.db_pool, assessment_id, &req)
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, e))?;
    Ok(Json(assessment))
}

/// Delete assessment
pub async fn delete_assessment(
    Extension(ctx): Extension<InstitutionCtx>,
    Path(assessment_id): Path<uuid::Uuid>,
) -> Result<StatusCode, (StatusCode, String)> {
    assessment_service::delete_assessment(&ctx.db_pool, assessment_id)
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, e))?;
    Ok(StatusCode::NO_CONTENT)
}

/// Publish assessment
pub async fn publish_assessment(
    Extension(ctx): Extension<InstitutionCtx>,
    Path(assessment_id): Path<uuid::Uuid>,
) -> Result<Json<Assessment>, (StatusCode, String)> {
    let assessment = assessment_service::publish_assessment(&ctx.db_pool, assessment_id)
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, e))?;
    Ok(Json(assessment))
}

/// Add question to assessment
pub async fn add_question_to_assessment(
    Extension(ctx): Extension<InstitutionCtx>,
    Path(assessment_id): Path<uuid::Uuid>,
    Json(req): Json<AddQuestionToAssessmentRequest>,
) -> Result<Json<AssessmentQuestion>, (StatusCode, String)> {
    let aq = assessment_service::add_question_to_assessment(
        &ctx.db_pool,
        assessment_id,
        req.question_id,
        req.points,
    )
    .await
    .map_err(|e| (StatusCode::BAD_REQUEST, e))?;
    Ok(Json(aq))
}

/// Remove question from assessment
pub async fn remove_question_from_assessment(
    Extension(ctx): Extension<InstitutionCtx>,
    Path((assessment_id, question_id)): Path<(uuid::Uuid, uuid::Uuid)>,
) -> Result<StatusCode, (StatusCode, String)> {
    assessment_service::remove_question_from_assessment(&ctx.db_pool, assessment_id, question_id)
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, e))?;
    Ok(StatusCode::NO_CONTENT)
}

/// Start attempt
pub async fn start_attempt(
    Extension(ctx): Extension<InstitutionCtx>,
    Path(assessment_id): Path<uuid::Uuid>,
) -> Result<Json<Attempt>, (StatusCode, String)> {
    let attempt = assessment_service::start_attempt(&ctx.db_pool, ctx.id, assessment_id)
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, e))?;
    Ok(Json(attempt))
}

/// Submit answer
pub async fn submit_answer(
    Extension(ctx): Extension<InstitutionCtx>,
    Path(attempt_id): Path<uuid::Uuid>,
    Json(req): Json<SubmitAnswerRequest>,
) -> Result<Json<Answer>, (StatusCode, String)> {
    let answer = assessment_service::submit_answer(&ctx.db_pool, ctx.id, attempt_id, &req)
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, e))?;
    Ok(Json(answer))
}

/// Submit attempt (finish)
pub async fn submit_attempt(
    Extension(ctx): Extension<InstitutionCtx>,
    Path(attempt_id): Path<uuid::Uuid>,
) -> Result<Json<AttemptDetailResponse>, (StatusCode, String)> {
    let result = assessment_service::submit_attempt(&ctx.db_pool, ctx.id, attempt_id)
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, e))?;
    Ok(Json(result))
}

/// Get user attempts for an assessment
pub async fn get_user_attempts(
    Extension(ctx): Extension<InstitutionCtx>,
    Path(assessment_id): Path<uuid::Uuid>,
) -> Result<Json<Vec<Attempt>>, (StatusCode, String)> {
    let attempts = assessment_service::get_user_attempts(&ctx.db_pool, ctx.id, assessment_id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;
    Ok(Json(attempts))
}

/// Get gradebook
pub async fn get_gradebook(
    Extension(ctx): Extension<InstitutionCtx>,
    Path(course_id): Path<uuid::Uuid>,
) -> Result<Json<GradebookResponse>, (StatusCode, String)> {
    let gradebook = assessment_service::get_gradebook(&ctx.db_pool, course_id, None)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;
    Ok(Json(gradebook))
}

/// Create grade (manual grading)
pub async fn create_grade(
    Extension(ctx): Extension<InstitutionCtx>,
    Path(course_id): Path<uuid::Uuid>,
    Json(req): Json<CreateGradeRequest>,
) -> Result<Json<Grade>, (StatusCode, String)> {
    let grade = assessment_service::create_grade(
        &ctx.db_pool,
        req.user_id,
        course_id,
        GradeSubmissionRequest {
            score: req.score,
            max_score: req.max_score,
            feedback: req.feedback,
        },
        req.assessment_id,
        ctx.id,
    )
    .await
    .map_err(|e| (StatusCode::BAD_REQUEST, e))?;
    Ok(Json(grade))
}

#[derive(serde::Deserialize)]
pub struct CreateGradeRequest {
    pub user_id: uuid::Uuid,
    pub assessment_id: Option<uuid::Uuid>,
    pub score: f32,
    pub max_score: f32,
    pub feedback: Option<String>,
}

/// Create assessments router
pub fn assessments_router() -> Router {
    Router::new()
        // Question banks
        .route("/banks", get(list_question_banks))
        .route("/banks", post(create_question_bank))
        // Questions
        .route("/questions", post(create_question))
        // Assessments
        .route("/", get(list_assessments))
        .route("/", post(create_assessment))
        .route("/:id", get(get_assessment))
        .route("/:id", put(update_assessment))
        .route("/:id", delete(delete_assessment))
        .route("/:id/publish", post(publish_assessment))
        .route("/:id/questions", post(add_question_to_assessment))
        .route("/:id/questions/:question_id", delete(remove_question_from_assessment))
        // Attempts
        .route("/:id/start", post(start_attempt))
        .route("/attempts/:attempt_id/answer", post(submit_answer))
        .route("/attempts/:attempt_id/submit", post(submit_attempt))
        .route("/:id/attempts", get(get_user_attempts))
        // Gradebook
        .route("/gradebook/:course_id", get(get_gradebook))
        .route("/gradebook/:course_id", post(create_grade))
}
