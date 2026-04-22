//! HTTP route handlers.
//!
//! Phase 1 surface:
//!   /health                         — liveness probe (no tenant required)
//!   /api/institutions               — list active tenants  (master-DB, public)
//!   /api/institutions/:slug         — read one tenant      (master-DB, public)
//!   /api/institutions/signup        — self-service signup  (master-DB, public)
//!   /api/auth/register              — create an account    (tenant required, public)
//!   /api/auth/login                 — password login       (tenant required, public)
//!   /api/auth/refresh               — rotate refresh token (tenant required, public)
//!   /api/auth/logout                — revoke refresh token (tenant required, public)
//!   /api/users/me                   — current user profile (tenant + JWT)
//!   /api/users/me/logout-all        — revoke all refresh tokens for current user

use axum::{routing::get, Json, Router};
use serde_json::json;

use crate::tenant::RouterState;

pub mod auth;
pub mod institutions;
pub mod users;

/// Combine all routers into the main API router. Nested under `/api` by
/// the top-level bootstrap in `main.rs`.
pub fn create_api_router(state: RouterState) -> Router {
    Router::new()
        .route(
            "/health",
            get(|| async { Json(json!({ "status": "ok", "phase": "1" })) }),
        )
        .nest("/institutions", institutions::router().with_state(state))
        .nest("/auth", auth::router())
        .nest("/users", users::router())
}
