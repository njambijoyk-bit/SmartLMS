// API module - HTTP route handlers
pub mod auth;
pub mod institutions;
pub mod users;
pub mod courses;
pub mod assessments;
pub mod enrollments;

/// Combine all routers into main API
pub fn create_api_router() -> axum::Router {
    axum::Router::new()
        .nest("/auth", auth::auth_router())
        .nest("/institutions", institutions::institutions_router())
        .nest("/courses", courses::courses_router())
        .nest("/assessments", assessments::assessments_router())
        .nest("/live", enrollments::enrollments_router())
        // .nest("/users", users::users_router())
}