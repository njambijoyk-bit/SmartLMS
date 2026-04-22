// Analytics & Reporting API - Dashboards, reports, xAPI export
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::api::AppState;
use crate::services::analytics::service::*;

/// Create the analytics router
pub fn analytics_router() -> Router<AppState> {
    Router::new()
        .route("/learner-dashboard", get(get_learner_dashboard_handler))
        .route("/course/:id", get(get_course_analytics_handler))
        .route("/cohort/:id", get(get_cohort_comparison_handler))
        .route("/reports", post(create_custom_report_handler))
        .route("/xapi/export", get(export_xapi_handler))
        .route("/predictions/student", get(predict_student_success_handler))
        .route("/alerts/early-warning", get(get_early_warning_alerts_handler))
        .route("/patterns/learning", get(analyze_learning_pattern_handler))
        .route("/cohort/:id/predictions", get(get_cohort_predictions_handler))
        .route("/course/:id/effectiveness", get(analyze_course_effectiveness_handler))
}

// ==================== Request/Response Models ====================

#[derive(Debug, Deserialize)]
pub struct LearnerDashboardQuery {
    pub user_id: String,
}

#[derive(Debug, Deserialize)]
pub struct CourseIdParam {
    pub id: String,
}

#[derive(Debug, Deserialize)]
pub struct CohortIdParam {
    pub id: String,
}

#[derive(Debug, Deserialize)]
pub struct CustomReportRequest {
    pub name: String,
    pub description: Option<String>,
    pub metrics: Vec<String>,
    pub filters: Option<serde_json::Value>,
    pub group_by: Option<String>,
    pub date_range: Option<DateRange>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DateRange {
    pub start: chrono::DateTime<chrono::Utc>,
    pub end: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Deserialize)]
pub struct XApiExportQuery {
    pub course_id: Option<String>,
    pub user_id: Option<String>,
    pub since: Option<String>,
    pub limit: Option<usize>,
}

#[derive(Debug, Deserialize)]
pub struct PredictionQuery {
    pub student_id: String,
    pub course_id: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct EarlyWarningQuery {
    pub course_id: Option<String>,
    pub cohort_id: Option<String>,
    pub severity: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct LearningPatternQuery {
    pub student_id: String,
    pub weeks: Option<i32>,
}

// ==================== Handler Functions ====================

/// GET /analytics/learner-dashboard?user_id=xxx
async fn get_learner_dashboard_handler(
    State(state): State<AppState>,
    query: Query<LearnerDashboardQuery>,
) -> Result<Json<LearnerDashboard>, StatusCode> {
    let user_id = Uuid::parse_str(&query.user_id)
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    AnalyticsService::get_learner_dashboard(&state.db_pool, user_id)
        .await
        .map(Json)
        .map_err(|e| {
            tracing::error!("Failed to get learner dashboard: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })
}

/// GET /analytics/course/:id
async fn get_course_analytics_handler(
    State(state): State<AppState>,
    Path(params): Path<CourseIdParam>,
) -> Result<Json<CourseAnalytics>, StatusCode> {
    let course_id = Uuid::parse_str(&params.id)
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    AnalyticsService::get_course_analytics(&state.db_pool, course_id)
        .await
        .map(Json)
        .map_err(|e| {
            tracing::error!("Failed to get course analytics: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })
}

/// GET /analytics/cohort/:id
async fn get_cohort_comparison_handler(
    State(state): State<AppState>,
    Path(params): Path<CohortIdParam>,
) -> Result<Json<CohortComparison>, StatusCode> {
    let cohort_id = Uuid::parse_str(&params.id)
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    AnalyticsService::get_cohort_comparison(&state.db_pool, cohort_id)
        .await
        .map(Json)
        .map_err(|e| {
            tracing::error!("Failed to get cohort comparison: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })
}

/// POST /analytics/reports
async fn create_custom_report_handler(
    State(state): State<AppState>,
    Json(payload): Json<CustomReportRequest>,
) -> Result<Json<CustomReport>, StatusCode> {
    AnalyticsService::create_custom_report(&state.db_pool, payload)
        .await
        .map(Json)
        .map_err(|e| {
            tracing::error!("Failed to create custom report: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })
}

/// GET /analytics/xapi/export
async fn export_xapi_handler(
    State(state): State<AppState>,
    query: Query<XApiExportQuery>,
) -> Result<Json<XApiExport>, StatusCode> {
    let course_id = query.course_id.as_ref().and_then(|s| Uuid::parse_str(s).ok());
    let user_id = query.user_id.as_ref().and_then(|s| Uuid::parse_str(s).ok());

    AnalyticsService::export_xapi(&state.db_pool, course_id, user_id, query.limit.unwrap_or(1000))
        .await
        .map(Json)
        .map_err(|e| {
            tracing::error!("Failed to export xAPI data: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })
}

/// GET /analytics/predictions/student?student_id=xxx&course_id=yyy
async fn predict_student_success_handler(
    State(state): State<AppState>,
    query: Query<PredictionQuery>,
) -> Result<Json<StudentSuccessPrediction>, StatusCode> {
    let student_id = Uuid::parse_str(&query.student_id)
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    
    let course_id = query.course_id.as_ref().and_then(|s| Uuid::parse_str(s).ok());

    AnalyticsService::predict_student_success(&state.db_pool, student_id, course_id)
        .await
        .map(Json)
        .map_err(|e| {
            tracing::error!("Failed to predict student success: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })
}

/// GET /analytics/alerts/early-warning
async fn get_early_warning_alerts_handler(
    State(state): State<AppState>,
    query: Query<EarlyWarningQuery>,
) -> Result<Json<Vec<EarlyWarningAlert>>, StatusCode> {
    let course_id = query.course_id.as_ref().and_then(|s| Uuid::parse_str(s).ok());
    let cohort_id = query.cohort_id.as_ref().and_then(|s| Uuid::parse_str(s).ok());

    AnalyticsService::get_early_warning_alerts(&state.db_pool, course_id, cohort_id)
        .await
        .map(Json)
        .map_err(|e| {
            tracing::error!("Failed to get early warning alerts: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })
}

/// GET /analytics/patterns/learning?student_id=xxx&weeks=4
async fn analyze_learning_pattern_handler(
    State(state): State<AppState>,
    query: Query<LearningPatternQuery>,
) -> Result<Json<LearningPatternAnalysis>, StatusCode> {
    let student_id = Uuid::parse_str(&query.student_id)
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    
    let weeks = query.weeks.unwrap_or(4);

    AnalyticsService::analyze_learning_pattern(&state.db_pool, student_id, weeks)
        .await
        .map(Json)
        .map_err(|e| {
            tracing::error!("Failed to analyze learning pattern: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })
}

/// GET /analytics/cohort/:id/predictions
async fn get_cohort_predictions_handler(
    State(state): State<AppState>,
    Path(params): Path<CohortIdParam>,
) -> Result<Json<CohortPredictions>, StatusCode> {
    let cohort_id = Uuid::parse_str(&params.id)
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    AnalyticsService::get_cohort_predictions(&state.db_pool, cohort_id)
        .await
        .map(Json)
        .map_err(|e| {
            tracing::error!("Failed to get cohort predictions: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })
}

/// GET /analytics/course/:id/effectiveness
async fn analyze_course_effectiveness_handler(
    State(state): State<AppState>,
    Path(params): Path<CourseIdParam>,
) -> Result<Json<CourseEffectiveness>, StatusCode> {
    let course_id = Uuid::parse_str(&params.id)
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    AnalyticsService::analyze_course_effectiveness(&state.db_pool, course_id)
        .await
        .map(Json)
        .map_err(|e| {
            tracing::error!("Failed to analyze course effectiveness: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })
}
