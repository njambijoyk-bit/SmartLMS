use std::net::SocketAddr;
use axum::{Router, routing::get};
use tower_http::{cors::Any, trace::TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

fn main() {
    init_logging();
    println!("Starting SmartLMS Engine v0.1.0");
    
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let app = Router::new()
            .route("/", get(|| async { "SmartLMS Engine v0.1.0" }))
            .route("/health", get(|| async { "OK" }))
            .route("/api/auth/login", get(|| async { "login" }))
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