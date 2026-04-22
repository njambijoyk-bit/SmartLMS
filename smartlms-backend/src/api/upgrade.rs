// Upgrade & License API routes - plan management and license validation
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::services::license;
use crate::services::upgrade;
use crate::tenant::{InstitutionCtx, PlanTier, RouterState};

/// Extract institution from request extensions
fn get_institution_ctx(req: &axum::extract::Request) -> Option<InstitutionCtx> {
    req.extensions().get::<InstitutionCtx>().cloned()
}

/// Upgrade request payload
#[derive(Debug, Deserialize)]
pub struct ApiUpgradeRequest {
    pub target_tier: String,
    pub license_key: Option<String>,
    pub payment_method_id: Option<String>,
}

/// Downgrade request payload
#[derive(Debug, Deserialize)]
pub struct ApiDowngradeRequest {
    pub target_tier: String,
    pub reason: Option<String>,
}

/// Tier comparison response
#[derive(Debug, Serialize)]
pub struct TierComparisonResponse {
    pub current_tier: String,
    pub target_tier: String,
    pub features_gained: Vec<String>,
    pub features_lost: Vec<String>,
    pub quota_changes: QuotaDiffResponse,
}

#[derive(Debug, Serialize)]
pub struct QuotaDiffResponse {
    pub users: (i64, i64),
    pub courses: (i64, i64),
    pub storage_mb: (i64, i64),
    pub concurrent: (i64, i64),
}

/// License validation response
#[derive(Debug, Serialize)]
pub struct LicenseValidationResponse {
    pub valid: bool,
    pub plan: String,
    pub quotas: LicenseQuotasResponse,
    pub expires_at: Option<chrono::DateTime<chrono::Utc>>,
    pub message: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct LicenseQuotasResponse {
    pub max_users: i64,
    pub max_courses: i64,
    pub max_storage_mb: i64,
    pub max_concurrent_users: i64,
}

/// Generate license request
#[derive(Debug, Deserialize)]
pub struct GenerateLicenseRequest {
    pub plan: String,
}

/// API Routes
pub fn upgrade_router() -> Router {
    Router::new()
        .route("/upgrade", axum::routing::post(handler_upgrade))
        .route("/downgrade", axum::routing::post(handler_downgrade))
        .route("/compare", axum::routing::get(handler_compare_tiers))
        .route("/features", axum::routing::get(handler_list_features))
        .route("/license/validate", axum::routing::post(handler_validate_license))
        .route("/license/generate", axum::routing::post(handler_generate_license))
        .route("/quota/check", axum::routing::get(handler_check_quota))
}

/// POST /api/upgrade/upgrade - Upgrade institution plan
async fn handler_upgrade(
    State(_state): State<RouterState>,
    req: axum::extract::Request,
    Json(payload): Json<ApiUpgradeRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let ctx = get_institution_ctx(&req).ok_or((
        StatusCode::BAD_REQUEST,
        "Institution context not found".to_string(),
    ))?;

    let target_tier = parse_plan_tier(&payload.target_tier).ok_or((
        StatusCode::BAD_REQUEST,
        format!("Invalid plan tier: {}", payload.target_tier),
    ))?;

    let upgrade_req = upgrade::UpgradeRequest {
        target_tier,
        license_key: payload.license_key,
        payment_method_id: payload.payment_method_id,
    };

    match upgrade::service::upgrade_institution(&ctx.db_pool, ctx.id, &upgrade_req).await {
        Ok(result) => Ok(Json(serde_json::json!({
            "success": result.success,
            "previous_tier": format!("{:?}", result.previous_tier),
            "new_tier": format!("{:?}", result.new_tier),
            "effective_from": result.effective_from,
            "message": result.message,
        }))),
        Err(e) => Err((StatusCode::BAD_REQUEST, e)),
    }
}

/// POST /api/upgrade/downgrade - Schedule plan downgrade
async fn handler_downgrade(
    State(_state): State<RouterState>,
    req: axum::extract::Request,
    Json(payload): Json<ApiDowngradeRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let ctx = get_institution_ctx(&req).ok_or((
        StatusCode::BAD_REQUEST,
        "Institution context not found".to_string(),
    ))?;

    let target_tier = parse_plan_tier(&payload.target_tier).ok_or((
        StatusCode::BAD_REQUEST,
        format!("Invalid plan tier: {}", payload.target_tier),
    ))?;

    let downgrade_req = upgrade::DowngradeRequest {
        target_tier,
        reason: payload.reason,
    };

    match upgrade::service::downgrade_institution(&ctx.db_pool, ctx.id, &downgrade_req).await {
        Ok(result) => Ok(Json(serde_json::json!({
            "success": result.success,
            "previous_tier": format!("{:?}", result.previous_tier),
            "new_tier": format!("{:?}", result.new_tier),
            "effective_from": result.effective_from,
            "message": result.message,
        }))),
        Err(e) => Err((StatusCode::BAD_REQUEST, e)),
    }
}

/// GET /api/upgrade/compare?current=<tier>&target=<tier> - Compare plan tiers
async fn handler_compare_tiers(
    Query(params): Query<std::collections::HashMap<String, String>>,
) -> Result<Json<TierComparisonResponse>, (StatusCode, String)> {
    let current_str = params.get("current").ok_or((
        StatusCode::BAD_REQUEST,
        "Missing 'current' parameter".to_string(),
    ))?;
    let target_str = params.get("target").ok_or((
        StatusCode::BAD_REQUEST,
        "Missing 'target' parameter".to_string(),
    ))?;

    let current = parse_plan_tier(current_str).ok_or((
        StatusCode::BAD_REQUEST,
        format!("Invalid plan tier: {}", current_str),
    ))?;
    let target = parse_plan_tier(target_str).ok_or((
        StatusCode::BAD_REQUEST,
        format!("Invalid plan tier: {}", target_str),
    ))?;

    let comparison = upgrade::service::compare_tiers(current, target);

    Ok(Json(TierComparisonResponse {
        current_tier: format!("{:?}", comparison.current_tier),
        target_tier: format!("{:?}", comparison.target_tier),
        features_gained: comparison.features_gained,
        features_lost: comparison.features_lost,
        quota_changes: QuotaDiffResponse {
            users: comparison.quota_changes.users,
            courses: comparison.quota_changes.courses,
            storage_mb: comparison.quota_changes.storage_mb,
            concurrent: comparison.quota_changes.concurrent,
        },
    }))
}

/// GET /api/upgrade/features?tier=<tier> - List features for a plan tier
async fn handler_list_features(
    Query(params): Query<std::collections::HashMap<String, String>>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let tier_str = params.get("tier").unwrap_or(&"starter".to_string());
    let tier = parse_plan_tier(tier_str).ok_or((
        StatusCode::BAD_REQUEST,
        format!("Invalid plan tier: {}", tier_str),
    ))?;

    let features = license::FeatureMatrix::features_for_plan(tier);

    Ok(Json(serde_json::json!({
        "tier": format!("{:?}", tier),
        "features": features,
    })))
}

/// POST /api/upgrade/license/validate - Validate a license key
async fn handler_validate_license(
    Json(payload): Json<serde_json::Value>,
) -> Result<Json<LicenseValidationResponse>, (StatusCode, String)> {
    let key = payload["key"]
        .as_str()
        .ok_or((StatusCode::BAD_REQUEST, "Missing 'key' field".to_string()))?;

    match license::validate_license(key) {
        Ok(status) => Ok(Json(LicenseValidationResponse {
            valid: status.valid,
            plan: format!("{:?}", status.plan),
            quotas: LicenseQuotasResponse {
                max_users: status.quotas.max_users,
                max_courses: status.quotas.max_courses,
                max_storage_mb: status.quotas.max_storage_mb,
                max_concurrent_users: status.quotas.max_concurrent_users,
            },
            expires_at: status.expires_at,
            message: status.message,
        })),
        Err(e) => Err((StatusCode::BAD_REQUEST, e)),
    }
}

/// POST /api/upgrade/license/generate - Generate a license key (admin only)
async fn handler_generate_license(
    Json(payload): Json<GenerateLicenseRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let tier = parse_plan_tier(&payload.plan).ok_or((
        StatusCode::BAD_REQUEST,
        format!("Invalid plan tier: {}", payload.plan),
    ))?;

    let key = license::generate_license_key(tier);

    Ok(Json(serde_json::json!({
        "license_key": key,
        "plan": format!("{:?}", tier),
        "note": "This is a demo key. In production, keys are cryptographically signed."
    })))
}

/// GET /api/upgrade/quota/check?resource=<name>&current=<count> - Check quota limit
async fn handler_check_quota(
    req: axum::extract::Request,
    Query(params): Query<std::collections::HashMap<String, String>>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let ctx = get_institution_ctx(&req).ok_or((
        StatusCode::BAD_REQUEST,
        "Institution context not found".to_string(),
    ))?;

    let resource = params.get("resource").ok_or((
        StatusCode::BAD_REQUEST,
        "Missing 'resource' parameter".to_string(),
    ))?;
    let current: i64 = params
        .get("current")
        .and_then(|s| s.parse().ok())
        .ok_or((
            StatusCode::BAD_REQUEST,
            "Missing or invalid 'current' parameter".to_string(),
        ))?;

    let limit = match resource.as_str() {
        "users" => ctx.quotas.max_users,
        "courses" => ctx.quotas.max_courses,
        "storage" => ctx.quotas.max_storage_mb,
        "concurrent" => ctx.quotas.max_concurrent_users,
        _ => {
            return Err((
                StatusCode::BAD_REQUEST,
                format!("Unknown resource: {}", resource),
            ))
        }
    };

    let available = limit.saturating_sub(current);
    let exceeded = current >= limit;

    Ok(Json(serde_json::json!({
        "resource": resource,
        "current": current,
        "limit": limit,
        "available": available,
        "exceeded": exceeded,
        "percent_used": if limit > 0 { (current as f64 / limit as f64 * 100.0).round() } else { 0.0 },
    })))
}

/// Helper to parse plan tier from string
fn parse_plan_tier(s: &str) -> Option<PlanTier> {
    match s.to_lowercase().as_str() {
        "starter" => Some(PlanTier::Starter),
        "growth" => Some(PlanTier::Growth),
        "enterprise" => Some(PlanTier::Enterprise),
        _ => None,
    }
}

use axum::extract::Query;
