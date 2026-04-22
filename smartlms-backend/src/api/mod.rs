//! HTTP route handlers. Phase 0: empty router — Phase 1 PRs (#54-#57) will
//! nest /auth, /institutions, /users, /courses, /enrollments, /assessments.

use axum::{routing::get, Json, Router};
use serde_json::json;

/// Combine all routers into main API. Phase 0 just exposes /health so we can
/// verify the crate boots and the tenant middleware runs.
pub fn create_api_router() -> Router {
    Router::new().route(
        "/health",
        get(|| async { Json(json!({ "status": "ok", "phase": "0" })) }),
    )
}
