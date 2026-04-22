use axum::{http::StatusCode, routing::get, Json, Router};
use serde_json::json;
use smartlms_backend::tenant::RouterState;
use std::net::SocketAddr;
use tower_http::{compression::CompressionLayer, cors::Any, trace::TraceLayer};
use tracing::{info, warn};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    init_logging();
    info!("Starting SmartLMS Engine v0.2.0");

    // ------------------------------------------------------------------
    // Master DB pool — source of truth for institutions/domain_map/licences.
    // ------------------------------------------------------------------
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://smartlms:smartlms@localhost:5432/smartlms".to_string());

    let master_pool = match sqlx::PgPool::connect(&database_url).await {
        Ok(pool) => {
            info!("Connected to master database");
            pool
        }
        Err(e) => {
            tracing::error!("Cannot connect to master database ({}). Exiting.", e);
            std::process::exit(1);
        }
    };

    // ------------------------------------------------------------------
    // Optional Redis cache — used on DashMap miss in the tenant router.
    // ------------------------------------------------------------------
    let router_state = match std::env::var("REDIS_URL").ok() {
        Some(url) if !url.is_empty() => match redis::Client::open(url.as_str()) {
            Ok(client) => match redis::aio::ConnectionManager::new(client).await {
                Ok(conn) => {
                    info!("Connected to Redis ({})", url);
                    RouterState::with_redis(master_pool.clone(), conn)
                }
                Err(e) => {
                    warn!(
                        "Redis connect failed ({}); falling back to in-process cache only",
                        e
                    );
                    RouterState::new(master_pool.clone())
                }
            },
            Err(e) => {
                warn!("Invalid REDIS_URL ({}); using in-process cache only", e);
                RouterState::new(master_pool.clone())
            }
        },
        _ => {
            info!("REDIS_URL not set — using in-process tenant cache only");
            RouterState::new(master_pool.clone())
        }
    };

    // ------------------------------------------------------------------
    // Router
    // ------------------------------------------------------------------
    let api_router = smartlms_backend::api::create_api_router();

    // Warm the domain_map from the master DB so custom domains resolve
    // without a first-request cache miss. If the table is missing (first boot,
    // no migrations yet) this is a soft failure.
    if let Err(e) = warm_domain_map(&router_state).await {
        warn!("Could not warm domain_map: {}", e);
    }

    let app = Router::new()
        .route("/", get(root_handler))
        .route("/health", get(health_handler))
        .route("/ready", get(ready_handler))
        .nest("/api", api_router)
        // Tenant middleware runs first — resolves Host header → InstitutionCtx
        // and injects it as an Extension on every request. Handlers that
        // declare `Extension<InstitutionCtx>` will find it there; requests
        // for unknown hosts simply have no extension set.
        .layer(axum::middleware::from_fn_with_state(
            router_state.clone(),
            smartlms_backend::middleware::tenant_middleware,
        ))
        .layer(TraceLayer::new_for_http())
        .layer(CompressionLayer::new())
        .layer(
            tower_http::cors::CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        )
        .with_state(router_state);

    let port: u16 = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(8080);

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    info!("SmartLMS Engine listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

/// Pre-populate RouterState.domain_map from the master DB so custom-domain
/// resolution is O(1) from the first request onwards. Uses a runtime-checked
/// query so it compiles even when the DB schema hasn't been migrated yet.
async fn warm_domain_map(state: &RouterState) -> Result<(), sqlx::Error> {
    let rows: Vec<(String, String)> =
        sqlx::query_as::<_, (String, String)>("SELECT host, slug FROM domain_map")
            .fetch_all(&state.master_pool)
            .await?;
    for (host, slug) in rows {
        state.domain_map.insert(host, slug);
    }
    info!(
        "Warmed domain_map with {} custom domains",
        state.domain_map.len()
    );
    Ok(())
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
    Json(json!({ "status": "healthy" }))
}

async fn ready_handler() -> StatusCode {
    StatusCode::OK
}

fn init_logging() {
    let filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| "smartlms=info,tower_http=debug".into());
    tracing_subscriber::registry()
        .with(filter)
        .with(tracing_subscriber::fmt::layer())
        .init();
}
