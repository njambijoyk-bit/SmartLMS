// Assessments API routes placeholder
use axum::{routing::get, Router};

pub fn assessments_router() -> Router {
    Router::new()
        .route("/health", get(|| async { "Assessments API OK" }))
}