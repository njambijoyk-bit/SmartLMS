// Courses API routes placeholder
use axum::{routing::get, Router};

pub fn courses_router() -> Router {
    Router::new()
        .route("/health", get(|| async { "Courses API OK" }))
}