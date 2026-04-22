//! HTTP route handlers.
//!
//! Phase 1 surface:
//!   /health                — liveness probe (no tenant required)
//!   /api/auth/register     — create an account (tenant required, public)
//!   /api/auth/login        — password login        (tenant required, public)
//!   /api/auth/refresh      — rotate refresh token  (tenant required, public)
//!   /api/auth/logout       — revoke refresh token  (tenant required, public)
//!   /api/users/me          — current user profile  (tenant + JWT required)
//!   /api/users/me/logout-all — revoke all refresh tokens for current user

use axum::{routing::get, Json, Router};
use serde_json::json;

pub mod auth;
pub mod users;

/// Combine all routers into the main API router. Nested under `/api` by
/// the top-level bootstrap in `main.rs`.
pub fn create_api_router() -> Router {
    Router::new()
        .route(
            "/health",
            get(|| async { Json(json!({ "status": "ok", "phase": "1" })) }),
        )
        .nest("/auth", auth::router())
        .nest("/users", users::router())
}
