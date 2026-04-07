// Users API routes placeholder
use axum::{routing::get, Router};

pub fn users_router() -> Router {
    Router::new().route("/health", get(|| async { "Users API OK" }))
}
