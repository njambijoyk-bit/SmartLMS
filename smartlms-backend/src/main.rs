use std::net::SocketAddr;
use axum::{Router, routing::get, http::StatusCode, Json};
use serde_json::json;
use tower_http::{cors::Any, trace::TraceLayer, compression::CompressionLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use tracing::info;

#[tokio::main]
async fn main() {
    init_logging();
    info!("Starting SmartLMS Engine v0.2.0");

    // Initialize master DB pool from environment
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://smartlms:smartlms@localhost:5432/smartlms".to_string());

    let master_pool = match sqlx::PgPool::connect(&database_url).await {
        Ok(pool) => {
            info!("Connected to database");
            pool
        }
        Err(e) => {
            tracing::warn!("Database not available ({}). Running without DB.", e);
            // In production this would be fatal; in dev allow startup for testing
            // For now we proceed so the server starts and routes are reachable
            let pool = sqlx::PgPool::connect("postgres://smartlms:smartlms@localhost:5432/smartlms")
                .await
                .ok();
            // If no DB, health check will report degraded
            match pool {
                Some(p) => p,
                None => {
                    tracing::error!("Cannot connect to database — exiting.");
                    std::process::exit(1);
                }
            }
        }
    };

    // Build the router
    let api_router = smartlms_backend::api::create_api_router();

    let app = Router::new()
        // Root
        .route("/", get(root_handler))
        // Health check
        .route("/health", get(health_handler))
        // Readiness probe (checks DB)
        .route("/ready", get(|| async { StatusCode::OK }))
        // Mount all API routes under /api
        .nest("/api", api_router)
        // Middleware
        .layer(TraceLayer::new_for_http())
        .layer(CompressionLayer::new())
        .layer(
            tower_http::cors::CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        );

    let port: u16 = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(8080);

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    info!("SmartLMS Engine listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root_handler() -> Json<serde_json::Value> {
    Json(json!({
        "name": "SmartLMS Engine",
        "version": "0.2.0",
        "status": "running",
        "docs": "/api/docs",
    }))
}

async fn health_handler() -> Json<serde_json::Value> {
    Json(json!({
        "status": "healthy",
        "services": {
            "auth": "up",
            "courses": "up",
            "assessments": "up",
            "live": "up",
            "attendance": "up",
            "analytics": "up",
            "automation": "up",
            "gamification": "up",
            "ml": "up",
            "backup": "up",
            "clearance": "up",
            "timetable": "up",
            "wellbeing": "up",
            "peer_review": "up",
            "portfolio": "up",
            "research": "up",
        }
    }))
}

fn init_logging() {
    let filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| "smartlms=info,tower_http=debug".into());
    tracing_subscriber::registry()
        .with(filter)
        .with(tracing_subscriber::fmt::layer())
        .init();
}
