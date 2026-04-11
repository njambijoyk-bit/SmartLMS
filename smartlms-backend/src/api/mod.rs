// API module - HTTP route handlers
pub mod abac;
pub mod ai_assistant;
pub mod assessments;
pub mod auth;
pub mod code_sandbox;
pub mod communication;
pub mod courses;
pub mod course_groups;
pub mod enrollments;
pub mod institutions;
pub mod upgrade;
pub mod users;

/// Combine all routers into main API
pub fn create_api_router() -> axum::Router {
    axum::Router::new()
        .nest("/auth", auth::auth_router())
        .nest("/institutions", institutions::institutions_router())
        .nest("/courses", courses::courses_router())
        .nest("/course-groups", course_groups::course_groups_router())
        .nest("/assessments", assessments::assessments_router())
        .nest("/communication", communication::communication_router())
        .nest("/live", enrollments::enrollments_router())
        .nest("/abac", abac::abac_router())
        .nest("/upgrade", upgrade::upgrade_router())
        .nest("/code-sandbox", code_sandbox::code_sandbox_router())
        .nest("/ai", ai_assistant::ai_router())
    // .nest("/users", users::users_router())
}
