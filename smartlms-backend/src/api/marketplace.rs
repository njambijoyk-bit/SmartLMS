// Phase 17 Enhancement: Developer Marketplace API
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::services::marketplace::{MarketplaceService, AppListing, AppCategory};
use crate::utils::app_state::AppState;

#[derive(Debug, Deserialize)]
pub struct CreateAppRequest {
    pub name: String,
    pub description: String,
    pub category: String,
    pub version: String,
    pub price: Option<f64>,
    pub is_free: bool,
    pub documentation_url: Option<String>,
    pub support_url: Option<String>,
    pub privacy_policy_url: Option<String>,
    pub terms_url: Option<String>,
    pub icon_url: Option<String>,
    pub screenshot_urls: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct AppResponse {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub category: String,
    pub version: String,
    pub developer_id: Uuid,
    pub price: Option<f64>,
    pub is_free: bool,
    pub rating: f64,
    pub review_count: i32,
    pub install_count: i32,
    pub status: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateAppRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub version: Option<String>,
    pub price: Option<f64>,
    pub is_free: Option<bool>,
    pub documentation_url: Option<String>,
    pub support_url: Option<String>,
    pub icon_url: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct SubmitReviewRequest {
    pub rating: u8, // 1-5
    pub title: String,
    pub comment: String,
}

#[derive(Debug, Serialize)]
pub struct ReviewResponse {
    pub id: Uuid,
    pub app_id: Uuid,
    pub user_id: Uuid,
    pub rating: u8,
    pub title: String,
    pub comment: String,
    pub created_at: String,
    pub helpful_count: i32,
}

#[derive(Debug, Deserialize)]
pub struct InstallAppRequest {
    pub institution_id: Option<Uuid>,
    pub configuration: Option<serde_json::Value>,
}

#[derive(Debug, Serialize)]
pub struct InstallResponse {
    pub installation_id: Uuid,
    pub app_id: Uuid,
    pub status: String,
    pub api_key: Option<String>,
    pub webhook_url: Option<String>,
    pub configuration: Option<serde_json::Value>,
}

/// GET /api/marketplace/apps - List all marketplace apps
pub async fn list_apps(
    State(state): State<AppState>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<Vec<AppResponse>>, StatusCode> {
    let category = params.get("category").map(|s| s.as_str());
    let search = params.get("search").map(|s| s.as_str());
    let sort = params.get("sort").map(|s| s.as_str()).unwrap_or("popular");
    
    match MarketplaceService::list_apps(category, search, sort) {
        Ok(apps) => {
            let response: Vec<AppResponse> = apps.iter().map(|app| AppResponse {
                id: app.id,
                name: app.name.clone(),
                description: app.description.clone(),
                category: app.category.clone(),
                version: app.version.clone(),
                developer_id: app.developer_id,
                price: app.price,
                is_free: app.is_free,
                rating: app.rating,
                review_count: app.review_count,
                install_count: app.install_count,
                status: app.status.clone(),
                created_at: app.created_at.to_string(),
                updated_at: app.updated_at.to_string(),
            }).collect();
            Ok(Json(response))
        }
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

/// GET /api/marketplace/apps/:id - Get app details
pub async fn get_app(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<AppResponse>, StatusCode> {
    match MarketplaceService::get_app(&id) {
        Some(app) => Ok(Json(AppResponse {
            id: app.id,
            name: app.name.clone(),
            description: app.description.clone(),
            category: app.category.clone(),
            version: app.version.clone(),
            developer_id: app.developer_id,
            price: app.price,
            is_free: app.is_free,
            rating: app.rating,
            review_count: app.review_count,
            install_count: app.install_count,
            status: app.status.clone(),
            created_at: app.created_at.to_string(),
            updated_at: app.updated_at.to_string(),
        })),
        None => Err(StatusCode::NOT_FOUND),
    }
}

/// POST /api/marketplace/apps - Create new app listing
pub async fn create_app(
    State(state): State<AppState>,
    Json(payload): Json<CreateAppRequest>,
) -> Result<Json<AppResponse>, StatusCode> {
    // TODO: Validate and create app listing
    // For now, return a placeholder response
    Err(StatusCode::NOT_IMPLEMENTED)
}

/// PUT /api/marketplace/apps/:id - Update app listing
pub async fn update_app(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateAppRequest>,
) -> Result<Json<AppResponse>, StatusCode> {
    // TODO: Update app listing
    Err(StatusCode::NOT_IMPLEMENTED)
}

/// DELETE /api/marketplace/apps/:id - Delete app listing
pub async fn delete_app(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, StatusCode> {
    // TODO: Delete app listing
    Err(StatusCode::NOT_IMPLEMENTED)
}

/// GET /api/marketplace/apps/:id/reviews - Get app reviews
pub async fn get_app_reviews(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<Vec<ReviewResponse>>, StatusCode> {
    // TODO: Fetch reviews from database
    Ok(Json(vec![]))
}

/// POST /api/marketplace/apps/:id/reviews - Submit app review
pub async fn submit_review(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(payload): Json<SubmitReviewRequest>,
) -> Result<Json<ReviewResponse>, StatusCode> {
    if payload.rating < 1 || payload.rating > 5 {
        return Err(StatusCode::BAD_REQUEST);
    }

    // TODO: Submit review
    Err(StatusCode::NOT_IMPLEMENTED)
}

/// POST /api/marketplace/apps/:id/install - Install app
pub async fn install_app(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(payload): Json<InstallAppRequest>,
) -> Result<Json<InstallResponse>, StatusCode> {
    match MarketplaceService::install_app(&id, payload.institution_id, payload.configuration) {
        Ok(installation) => Ok(Json(InstallResponse {
            installation_id: installation.0,
            app_id: id,
            status: "active".to_string(),
            api_key: Some("sk_marketplace_".to_string() + &Uuid::new_v4().to_string()),
            webhook_url: Some(format!("https://smartlms.com/webhooks/marketplace/{}", installation.0)),
            configuration: payload.configuration,
        })),
        Err(_) => Err(StatusCode::BAD_REQUEST),
    }
}

/// DELETE /api/marketplace/installations/:id - Uninstall app
pub async fn uninstall_app(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, StatusCode> {
    match MarketplaceService::uninstall_app(&id) {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(_) => Err(StatusCode::NOT_FOUND),
    }
}

/// GET /api/marketplace/categories - Get app categories
pub async fn get_categories() -> Result<Json<Vec<&'static str>>, StatusCode> {
    Ok(Json(vec![
        "Analytics",
        "Assessment",
        "Communication",
        "Content Management",
        "Gamification",
        "Integration",
        "Productivity",
        "Security",
        "Student Engagement",
        "Other",
    ]))
}

/// GET /api/marketplace/my-apps - Get current user's installed apps
pub async fn get_my_apps(
    State(state): State<AppState>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<Vec<AppResponse>>, StatusCode> {
    // TODO: Fetch user's installed apps
    Ok(Json(vec![]))
}

/// POST /api/marketplace/apps/:id/purchase - Purchase paid app
pub async fn purchase_app(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<InstallResponse>, StatusCode> {
    // TODO: Process payment and install app
    Err(StatusCode::NOT_IMPLEMENTED)
}

use std::collections::HashMap;

pub fn marketplace_router() -> axum::Router {
    axum::Router::new()
        .route("/apps", axum::routing::get(list_apps))
        .route("/apps", axum::routing::post(create_app))
        .route("/apps/:id", axum::routing::get(get_app))
        .route("/apps/:id", axum::routing::put(update_app))
        .route("/apps/:id", axum::routing::delete(delete_app))
        .route("/apps/:id/reviews", axum::routing::get(get_app_reviews))
        .route("/apps/:id/reviews", axum::routing::post(submit_review))
        .route("/apps/:id/install", axum::routing::post(install_app))
        .route("/apps/:id/purchase", axum::routing::post(purchase_app))
        .route("/installations/:id", axum::routing::delete(uninstall_app))
        .route("/categories", axum::routing::get(get_categories))
        .route("/my-apps", axum::routing::get(get_my_apps))
}
