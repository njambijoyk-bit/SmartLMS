//! Institutions API routes

use axum::{
    extract::State,
    response::Json,
    routing::{get, post, put, delete},
    Router,
};

use crate::utils::app_state::AppState;

/// Institution router
pub fn router() -> Router {
    Router::new()
        .route("/api/institutions", get(list_institutions))
        .route("/api/institutions", post(create_institution))
        .route("/api/institutions/:id", get(get_institution))
        .route("/api/institutions/:id", put(update_institution))
        .route("/api/institutions/:id/settings", get(get_settings))
        .route("/api/institutions/:id/settings", put(update_settings))
}

/// List institutions
async fn list_institutions(State(state): State<AppState>) -> Result<Json<serde_json::Value>, AppError> {
    // In production, list from database
    Ok(Json(serde_json::json!({
        "institutions": [],
        "total": 0
    })))
}

/// Get institution by ID
async fn get_institution(
    State(state): State<AppState>,
    axum::extract::Path(id): axum::extract::Path<uuid::Uuid>,
) -> Result<Json<serde_json::Value>, AppError> {
    Ok(Json(serde_json::json!({
        "id": id,
        "slug": "demo-university",
        "name": "Demo University",
        "plan_tier": "starter",
        "status": "active"
    })))
}

/// Create institution
async fn create_institution(
    State(state): State<AppState>,
    Json(payload): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, AppError> {
    // In production, create in database
    Ok(Json(serde_json::json!({
        "message": "Institution created",
        "id": uuid::Uuid::new_v4()
    })))
}

/// Update institution
async fn update_institution(
    State(state): State<AppState>,
    axum::extract::Path(id): axum::extract::Path<uuid::Uuid>,
    Json(payload): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, AppError> {
    Ok(Json(serde_json::json!({
        "message": "Institution updated",
        "id": id
    })))
}

/// Get institution settings
async fn get_settings(
    State(state): State<AppState>,
    axum::extract::Path(id): axum::extract::Path<uuid::Uuid>,
) -> Result<Json<serde_json::Value>, AppError> {
    Ok(Json(serde_json::json!({
        "timezone": "UTC",
        "language": "en",
        "date_format": "YYYY-MM-DD"
    })))
}

/// Update institution settings
async fn update_settings(
    State(state): State<AppState>,
    axum::extract::Path(id): axum::extract::Path<uuid::Uuid>,
    Json(payload): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, AppError> {
    Ok(Json(serde_json::json!({
        "message": "Settings updated"
    })))
}

use crate::api::routes::auth::AppError;