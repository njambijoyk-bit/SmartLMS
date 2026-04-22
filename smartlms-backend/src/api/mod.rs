//! HTTP route handlers.
//!
//! Phase 1 surface:
//!   /health                                 — liveness probe
//!   /api/institutions                       — list active tenants     (master-DB, public)
//!   /api/institutions/:slug                 — read one tenant         (master-DB, public)
//!   /api/institutions/signup                — self-service signup     (master-DB, public)
//!   /api/auth/{register,login,refresh,logout} — auth flows            (tenant-scoped)
//!   /api/users/me                           — current user profile    (tenant + JWT)
//!   /api/users/me/logout-all                — revoke all refresh tokens for current user
//!   /api/courses                            — list / create courses   (tenant + JWT)
//!   /api/courses/:id                        — read / update / archive (tenant + JWT)
//!   /api/courses/:id/modules[/lessons]      — nested content tree
//!   /api/courses/:id/enroll|drop            — learner-side enrolment
//!   /api/courses/:id/lessons/:lid/complete  — mark lesson complete
//!   /api/courses/:id/enrollments            — staff-side roster       (tenant + JWT)
//!   /api/enrollments                        — current user's enrolments

use axum::{routing::get, Json, Router};
use serde_json::json;

use crate::tenant::RouterState;

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
}
