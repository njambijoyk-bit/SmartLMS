//! SmartLMS Engine - Main entry point
//! 
//! A complete Learning Management System engine built once, 
//! packaged and deployed to institutions worldwide.

use std::net::SocketAddr;
use std::sync::Arc;

use axum::{
    Router,
    routing::get,
};
use tower_http::cors::{CorsLayer, Any};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod api;
mod db;
mod middleware;
mod models;
mod services;
mod utils;

use api::routes::auth::router as auth_router;
use api::routes::institutions::router as institutions_router;
use api::routes::users::router as users_router;
use api::routes::courses::router as courses_router;
use api::routes::assessments::router as assessments_router;
use api::routes::enrollments::router as enrollments_router;
use utils::app_state::AppState;

/// Initialize logging with JSON formatting for production
fn init_logging() {
    let filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| "smartlms=debug,tower=info".into());
    
    tracing_subscriber::registry()
        .with(filter)
        .with(tracing_subscriber::fmt::layer())
        .init();
}

#[tokio::main]
async fn main() {
    init_logging();
    
    tracing::info!("Starting SmartLMS Engine v0.1.0");
    
    // Load environment variables
    dotenv::dotenv().ok();
    
    // Create database pool
    let db_pool = match DbPool::new().await {
        Ok(pool) => {
            tracing::info!("Database connection established");
            pool
        }
        Err(e) => {
            tracing::warn!("Database connection failed, starting without DB: {}", e);
            // Continue without DB for development/demo
            Arc::new(tokio::sync::RwLock::new(None))
        }
    };
    
    // Create application state
    let app_state = AppState::new(db_pool);
    
    // Build router with middleware
    let app = Router::new()
        .route("/", get(|| async { "SmartLMS Engine v0.1.0" }))
        .route("/health", get(health_check))
        .merge(auth_router())
        .merge(institutions_router())
        .merge(users_router())
        .merge(courses_router())
        .merge(assessments_router())
        .merge(enrollments_router())
        .layer(TraceLayer::new_for_http())
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        )
        .with_state(app_state);
    
    // Determine bind address
    let addr = std::env::var("SERVER_ADDR")
        .unwrap_or_else(|_| "0.0.0.0:8080".to_string())
        .parse::<SocketAddr>()
        .unwrap_or_else(|_| SocketAddr::from(([0, 0, 0, 0], 8080)));
    
    tracing::info!("Server listening on {}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn health_check() -> &'static str {
    "OK"
}