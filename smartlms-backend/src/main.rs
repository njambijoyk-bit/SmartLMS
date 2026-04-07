use std::net::SocketAddr;
use axum::{Router, routing::get};
use tower_http::{cors::Any, trace::TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

fn main() {
    init_logging();
    println!("Starting SmartLMS Engine v0.1.0");
    
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        // TODO: Initialize master DB pool from environment
        // let master_pool = sqlx::PgPool::connect(&std::env::var("DATABASE_URL").unwrap()).await.unwrap();
        // let state = smartlms_backend::tenant::RouterState::new(master_pool);
        
        let app = Router::new()
            .route("/", get(|| async { "SmartLMS Engine v0.1.0" }))
            .route("/health", get(|| async { "OK" }))
            // Public auth routes
            .route("/api/auth/login", get(|| async { "POST /api/auth/login to login" }))
            .route("/api/auth/register", get(|| async { "POST /api/auth/register to register" }))
            // Institution routes
            .route("/api/institutions/init", get(|| async { "POST /api/institutions/init to create new institution" }))
            .layer(TraceLayer::new_for_http())
            .layer(tower_http::cors::CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any));

        let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
        println!("Server listening on {}", addr);
        
        let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
        axum::serve(listener, app).await.unwrap();
    });
}

fn init_logging() {
    let filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| "smartlms=debug".into());
    tracing_subscriber::registry()
        .with(filter)
        .with(tracing_subscriber::fmt::layer())
        .init();
}