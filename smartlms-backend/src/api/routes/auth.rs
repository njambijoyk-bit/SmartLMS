//! Authentication API routes

use axum::{
    extract::State,
    response::Json,
    routing::{get, post, put, delete},
    Router,
};
use serde::{Deserialize, Serialize};

use crate::models::{LoginRequest, LoginResponse, UserRole, UserResponse};
use crate::utils::app_state::AppState;

/// Auth router
pub fn router() -> Router {
    Router::new()
        .route("/api/auth/login", post(login))
        .route("/api/auth/logout", post(logout))
        .route("/api/auth/me", get(me))
        .route("/api/auth/refresh", post(refresh_token))
}

/// Login handler
async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, AppError> {
    // In production, validate against database
    // For demo, create a mock response
    let user = crate::models::User::new(
        uuid::Uuid::new_v4(),
        payload.email.clone(),
        "hashed_password".to_string(),
        "Demo".to_string(),
        "User".to_string(),
        UserRole::Admin,
    );
    
    let user_response: UserResponse = user.clone().into();
    let token = generate_jwt(&user.id, &state.jwt_secret, state.jwt_expiry_hours)?;
    
    Ok(Json(LoginResponse {
        token,
        user: user_response,
        expires_in: state.jwt_expiry_hours * 3600,
    }))
}

/// Logout handler
async fn logout() -> Result<Json<serde_json::Value>, AppError> {
    Ok(Json(serde_json::json!({ "message": "Logged out successfully" })))
}

/// Get current user handler
async fn me(State(state): State<AppState>) -> Result<Json<UserResponse>, AppError> {
    // In production, extract user from JWT token
    let user = crate::models::User::new(
        uuid::Uuid::new_v4(),
        "admin@example.com".to_string(),
        "hashed_password".to_string(),
        "Admin".to_string(),
        "User".to_string(),
        UserRole::Admin,
    );
    
    Ok(Json(user.into()))
}

/// Refresh token handler
async fn refresh_token(State(state): State<AppState>) -> Result<Json<serde_json::Value>, AppError> {
    // In production, validate refresh token and issue new access token
    Ok(Json(serde_json::json!({ 
        "message": "Token refreshed",
        "expires_in": state.jwt_expiry_hours * 3600
    })))
}

/// Generate JWT token
fn generate_jwt(user_id: &uuid::Uuid, secret: &str, expiry_hours: i64) -> Result<String, AppError> {
    use jsonwebtoken::{encode, decode, Header, EncodingKey, DecodingKey, Validation};
    use chrono::{Utc, Duration};
    
    #[derive(Debug, serde::Serialize, serde::Deserialize)]
    struct JwtClaims {
        sub: String,
        iat: i64,
        exp: i64,
    }
    
    let expiration = Utc::now() + Duration::hours(expiry_hours);
    
    let claims = JwtClaims {
        sub: user_id.to_string(),
        exp: expiration.timestamp(),
        iat: Utc::now().timestamp(),
        ..Default::default()
    };
    
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )?;
    
    Ok(token)
}

/// Application error type
#[derive(Debug, Serialize)]
pub struct AppError {
    pub message: String,
    pub code: String,
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for AppError {}

impl axum::response::IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        Json(self).into_response()
    }
}

impl From<jsonwebtoken::errors::Error> for AppError {
    fn from(err: jsonwebtoken::errors::Error) -> Self {
        AppError {
            message: err.to_string(),
            code: "JWT_ERROR".to_string(),
        }
    }
}