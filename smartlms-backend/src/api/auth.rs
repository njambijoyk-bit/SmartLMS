// Auth API routes - login, register, password reset
use crate::models::user::{LoginRequest, LoginResponse, RegisterRequest};
use crate::services::auth as auth_service;
use crate::tenant::InstitutionCtx;
use axum::{
    extract::{Extension, Json, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Router,
};

/// Login handler
pub async fn login(
    State(pool): State<sqlx::PgPool>,
    Json(req): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, (StatusCode, String)> {
    match auth_service::login(&pool, &req).await {
        Ok(Some(response)) => Ok(Json(response)),
        Ok(None) => Err((StatusCode::UNAUTHORIZED, "Invalid credentials".to_string())),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e)),
    }
}

/// Register handler
pub async fn register(
    State(pool): State<sqlx::PgPool>,
    Json(req): Json<RegisterRequest>,
) -> Result<Json<crate::models::user::User>, (StatusCode, String)> {
    match auth_service::register(&pool, &req).await {
        Ok(user) => Ok(Json(user)),
        Err(e) => Err((StatusCode::BAD_REQUEST, e)),
    }
}

/// Change password handler (requires auth)
pub async fn change_password(
    Extension(user): Extension<crate::models::user::User>,
    State(pool): State<sqlx::PgPool>,
    Json(req): Json<ChangePasswordRequest>,
) -> Result<StatusCode, (StatusCode, String)> {
    match auth_service::change_password(&pool, user.id, &req.old_password, &req.new_password).await
    {
        Ok(true) => Ok(StatusCode::OK),
        Ok(false) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to change password".to_string(),
        )),
        Err(e) => Err((StatusCode::BAD_REQUEST, e)),
    }
}

/// Request password reset
pub async fn request_password_reset(
    State(pool): State<sqlx::PgPool>,
    Json(req): Json<PasswordResetRequest>,
) -> Result<StatusCode, (StatusCode, String)> {
    match auth_service::request_password_reset(&pool, &req.email).await {
        Ok(_) => Ok(StatusCode::OK), // Don't reveal if email exists
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e)),
    }
}

/// Logout handler (client-side token removal, but we can track it)
pub async fn logout(Extension(user): Extension<crate::models::user::User>) -> StatusCode {
    // TODO: In production, add token to blacklist for faster invalidation
    tracing::info!("User {} logged out", user.email);
    StatusCode::NO_CONTENT
}

/// Health check - verify auth system working
pub async fn health() -> impl IntoResponse {
    "Auth system OK"
}

// Request/Response types
#[derive(serde::Deserialize)]
pub struct ChangePasswordRequest {
    pub old_password: String,
    pub new_password: String,
}

#[derive(serde::Deserialize)]
pub struct PasswordResetRequest {
    pub email: String,
}

/// Create auth router
pub fn auth_router() -> Router {
    Router::new()
        .route("/login", post(login))
        .route("/register", post(register))
        .route("/logout", post(logout))
        .route("/change-password", post(change_password))
        .route("/reset-password", post(request_password_reset))
        .route("/health", get(health))
}
