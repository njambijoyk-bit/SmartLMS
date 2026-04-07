// Authentication middleware - validates JWT tokens on protected routes
use crate::models::user::User;
use crate::services::auth::jwt;
use axum::{
    body::Body,
    extract::{Request, State},
    http::{header::AUTHORIZATION, StatusCode},
    middleware::Next,
    response::Response,
};

pub type AuthUser = User;

/// Auth middleware - validates Bearer token and extracts user
pub async fn auth_middleware(
    State(state): State<crate::tenant::RouterState>,
    mut request: Request<Body>,
    next: Next,
) -> Response {
    // Skip auth for public routes
    let path = request.uri().path();
    if is_public_route(path) {
        return next.run(request).await;
    }

    // Extract Authorization header
    let auth_header = request
        .headers()
        .get(AUTHORIZATION)
        .and_then(|h| h.to_str().ok());

    let token = auth_header
        .and_then(|h| h.strip_prefix("Bearer "))
        .or_else(|| auth_header);

    // Validate token and get user
    if let Some(token_str) = token {
        match jwt::validate_token(token_str) {
            Ok(claims) => {
                // Create minimal user from JWT claims
                let user = User {
                    id: claims.sub,
                    email: claims.email,
                    password_hash: String::new(), // Don't expose in request
                    first_name: claims.first_name,
                    last_name: claims.last_name,
                    role: claims.role,
                };
                request.extensions_mut().insert(user);
            }
            Err(e) => {
                tracing::warn!("JWT validation failed: {}", e);
            }
        }
    }

    next.run(request).await
}

/// Check if route is public (no auth required)
fn is_public_route(path: &str) -> bool {
    matches!(
        path,
        "/" | "/health" | "/api/auth/login" | "/api/auth/register" | "/api/institutions/init"
    )
}

pub mod axum_extract {
    use super::AuthUser;
    use axum::extract::Extension;

    /// Extension extractor for authenticated user
    pub async fn auth_user(Extension(user): Extension<AuthUser>) -> AuthUser {
        user
    }
}
