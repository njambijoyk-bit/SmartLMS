// Institution API routes - signup, setup, onboarding
use axum::{
    extract::{State, Json, Path, Query, Extension},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post, put},
    Router,
};
use crate::models::institution::{CreateInstitutionRequest, UpdateInstitutionRequest, Institution, InstitutionListResponse};
use crate::services::onboarding::{self, OnboardingState, SandboxStatus};
use crate::services::license;
use crate::tenant::InstitutionCtx;
use serde::Deserialize;

/// Pagination params
#[derive(Debug, Deserialize)]
pub struct Pagination {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}

impl Default for Pagination {
    fn default() -> Self {
        Self { page: Some(1), per_page: Some(20) }
    }
}

/// Create new institution (signup)
pub async fn create_institution(
    State(pool): State<sqlx::PgPool>,
    Json(req): Json<CreateInstitutionRequest>,
) -> Result<Json<Institution>, (StatusCode, String)> {
    // Validate slug
    if let Err(e) = onboarding::validate_slug(&req.slug) {
        return Err((StatusCode::BAD_REQUEST, e));
    }
    
    // Check for license key if provided
    if let Some(ref key) = req.license_key {
        let status = license::validate_license(key)
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;
        
        if !status.valid {
            return Err((StatusCode::BAD_REQUEST, status.message.unwrap_or("Invalid license".to_string())));
        }
    }
    
    match onboarding::create_institution(&pool, &req, false).await {
        Ok(inst) => Ok(Json(inst)),
        Err(e) => Err((StatusCode::BAD_REQUEST, e)),
    }
}

/// Get institution by ID
pub async fn get_institution(
    Extension(ctx): Extension<InstitutionCtx>,
) -> Json<Institution> {
    // Return basic info from context
    Json(Institution {
        id: ctx.id,
        slug: ctx.slug,
        name: ctx.config.name.clone(),
        domain: None,
        database_url: None,
        config: Some(ctx.config.clone()),
        plan_tier: Some(ctx.plan),
        quotas: Some(ctx.quotas.clone()),
        license_key: None,
        is_active: true,
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    })
}

/// Update institution settings
pub async fn update_institution(
    Extension(ctx): Extension<InstitutionCtx>,
    State(pool): State<sqlx::PgPool>,
    Json(req): Json<UpdateInstitutionRequest>,
) -> Result<Json<Institution>, (StatusCode, String)> {
    match crate::db::institution::update(&pool, ctx.id, &req).await {
        Ok(Some(inst)) => Ok(Json(inst)),
        Ok(None) => Err((StatusCode::NOT_FOUND, "Institution not found".to_string())),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

/// List all institutions (super admin only)
pub async fn list_institutions(
    State(pool): State<sqlx::PgPool>,
    Query(pagination): Query<Pagination>,
) -> Result<Json<InstitutionListResponse>, (StatusCode, String)> {
    let page = pagination.page.unwrap_or(1);
    let per_page = pagination.per_page.unwrap_or(20).min(100);
    
    match crate::db::institution::list(&pool, page, per_page).await {
        Ok((institutions, total)) => Ok(Json(InstitutionListResponse {
            institutions,
            total,
            page,
            per_page,
        })),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

/// Get onboarding state
pub async fn get_onboarding(
    Extension(ctx): Extension<InstitutionCtx>,
) -> Result<Json<OnboardingState>, (StatusCode, String)> {
    // TODO: Fetch from DB
    let state = onboarding::OnboardingState::new(ctx.id, false);
    Ok(Json(state))
}

/// Complete onboarding step
pub async fn complete_onboarding_step(
    Extension(ctx): Extension<InstitutionCtx>,
    State(pool): State<sqlx::PgPool>,
    Json(req): Json<CompleteStepRequest>,
) -> Result<Json<OnboardingState>, (StatusCode, String)> {
    match onboarding::complete_step(&pool, ctx.id, req.step, req.data).await {
        Ok(state) => Ok(Json(state)),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e)),
    }
}

/// Get sandbox status
pub async fn get_sandbox_status(
    Extension(ctx): Extension<InstitutionCtx>,
) -> Result<Json<SandboxStatus>, (StatusCode, String)> {
    let state = onboarding::OnboardingState::new(ctx.id, true);
    Ok(Json(onboarding::check_sandbox_status(&state)))
}

/// Get license status
pub async fn get_license_status(
    Extension(ctx): Extension<InstitutionCtx>,
) -> Json<license::LicenseStatus> {
    // TODO: Fetch actual license
    Json(license::LicenseStatus {
        valid: true,
        plan: ctx.plan,
        quotas: ctx.quotas.clone(),
        expires_at: None,
        message: Some("Active".to_string()),
    })
}

/// Validate license key
pub async fn validate_license(
    Json(req): Json<ValidateLicenseRequest>,
) -> Result<Json<license::LicenseStatus>, (StatusCode, String)> {
    match license::validate_license(&req.key) {
        Ok(status) => Ok(Json(status)),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e)),
    }
}

// Request types
#[derive(serde::Deserialize)]
pub struct CompleteStepRequest {
    pub step: i32,
    pub data: serde_json::Value,
}

#[derive(serde::Deserialize)]
pub struct ValidateLicenseRequest {
    pub key: String,
}

/// Create institution router
pub fn institutions_router() -> Router {
    Router::new()
        .route("/", post(create_institution))
        .route("/", get(get_institution))
        .route("/", put(update_institution))
        .route("/list", get(list_institutions))
        .route("/onboarding", get(get_onboarding))
        .route("/onboarding/complete", post(complete_onboarding_step))
        .route("/sandbox-status", get(get_sandbox_status))
        .route("/license", get(get_license_status))
        .route("/license/validate", post(validate_license))
}