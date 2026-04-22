//! HTTP route handlers.

use axum::{routing::get, Json, Router};
use serde_json::json;

use crate::tenant::RouterState;

pub mod assessments;
pub mod auth;
pub mod courses;
pub mod enrollments;
pub mod institutions;
pub mod users;

pub fn create_api_router(state: RouterState) -> Router {
    Router::new()
        .route(
            "/health",
            get(|| async { Json(json!({ "status": "ok", "phase": "1" })) }),
        )
        .nest("/institutions", institutions::router().with_state(state))
        .nest("/auth", auth::router())
        .nest("/users", users::router())
        .nest("/courses", courses::router())
        .nest("/enrollments", enrollments::router())
        .nest("/questions", assessments::questions_router())
        .nest("/assessments", assessments::assessments_router())
}
