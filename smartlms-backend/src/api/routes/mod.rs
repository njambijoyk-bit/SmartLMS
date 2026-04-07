//! API Routes

pub mod auth;
pub mod institutions;
pub mod users;
pub mod courses;
pub mod assessments;
pub mod enrollments;

pub use auth::router as auth_router;
pub use institutions::router as institutions_router;
pub use users::router as users_router;
pub use courses::router as courses_router;
pub use assessments::router as assessments_router;
pub use enrollments::router as enrollments_router;

/// Merge all routers into one
pub fn merge() -> axum::Router {
    use axum::Router;
    
    Router::new()
        .merge(auth::router())
        .merge(institutions::router())
        .merge(users::router())
        .merge(courses::router())
        .merge(assessments::router())
        .merge(enrollments::router())
}