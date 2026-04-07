// Enrollments API routes placeholder
use axum::{routing::get, Router};

pub fn enrollments_router() -> Router {
    Router::new()
        .route("/health", get(|| async { "Enrollments API OK" }))
}