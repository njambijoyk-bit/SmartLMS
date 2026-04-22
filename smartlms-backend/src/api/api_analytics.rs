// Phase 17 Enhancement: API Analytics Dashboard API
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use crate::services::api_analytics::{ApiAnalyticsService, AnalyticsQuery, TimeRange};
use crate::utils::app_state::AppState;

#[derive(Debug, Deserialize)]
pub struct AnalyticsQueryParams {
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub time_range: Option<String>, // "1h", "24h", "7d", "30d"
    pub endpoint: Option<String>,
    pub client_id: Option<String>,
    pub status_code: Option<u16>,
    pub tenant_id: Option<Uuid>,
}

#[derive(Debug, Serialize)]
pub struct MetricsResponse {
    pub total_requests: u64,
    pub successful_requests: u64,
    pub failed_requests: u64,
    pub average_latency_ms: f64,
    pub p50_latency_ms: f64,
    pub p95_latency_ms: f64,
    pub p99_latency_ms: f64,
    pub requests_per_second: f64,
    pub error_rate: f64,
    pub bandwidth_bytes: u64,
}

#[derive(Debug, Serialize)]
pub struct EndpointStats {
    pub endpoint: String,
    pub method: String,
    pub request_count: u64,
    pub average_latency_ms: f64,
    pub error_rate: f64,
    pub p95_latency_ms: f64,
}

#[derive(Debug, Serialize)]
pub struct ClientStats {
    pub client_id: String,
    pub client_name: Option<String>,
    pub request_count: u64,
    pub error_count: u64,
    pub bandwidth_bytes: u64,
    pub last_seen: String,
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub status_code: u16,
    pub count: u64,
    pub percentage: f64,
    pub sample_errors: Vec<ErrorSample>,
}

#[derive(Debug, Serialize)]
pub struct ErrorSample {
    pub timestamp: String,
    pub endpoint: String,
    pub method: String,
    pub error_message: Option<String>,
    pub client_id: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct RateLimitStats {
    pub client_id: String,
    pub limit: u64,
    pub remaining: u64,
    pub reset_at: String,
    pub exceeded_count: u64,
}

#[derive(Debug, Serialize)]
pub struct UsageTrendPoint {
    pub timestamp: String,
    pub request_count: u64,
    pub error_count: u64,
    pub average_latency_ms: f64,
}

#[derive(Debug, Serialize)]
pub struct UsageTrendResponse {
    pub points: Vec<UsageTrendPoint>,
    pub interval: String,
}

/// GET /api/analytics/metrics - Get overall API metrics
pub async fn get_metrics(
    State(state): State<AppState>,
    Query(params): Query<AnalyticsQueryParams>,
) -> Result<Json<MetricsResponse>, StatusCode> {
    let query = build_analytics_query(params);
    
    match ApiAnalyticsService::get_metrics(&query) {
        Ok(metrics) => Ok(Json(MetricsResponse {
            total_requests: metrics.total_requests,
            successful_requests: metrics.successful_requests,
            failed_requests: metrics.failed_requests,
            average_latency_ms: metrics.average_latency_ms,
            p50_latency_ms: metrics.p50_latency_ms,
            p95_latency_ms: metrics.p95_latency_ms,
            p99_latency_ms: metrics.p99_latency_ms,
            requests_per_second: metrics.requests_per_second,
            error_rate: metrics.error_rate,
            bandwidth_bytes: metrics.bandwidth_bytes,
        })),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

/// GET /api/analytics/endpoints - Get endpoint-level statistics
pub async fn get_endpoint_stats(
    State(state): State<AppState>,
    Query(params): Query<AnalyticsQueryParams>,
) -> Result<Json<Vec<EndpointStats>>, StatusCode> {
    let query = build_analytics_query(params);
    
    match ApiAnalyticsService::get_endpoint_stats(&query) {
        Ok(stats) => {
            let response: Vec<EndpointStats> = stats.iter().map(|s| EndpointStats {
                endpoint: s.endpoint.clone(),
                method: s.method.clone(),
                request_count: s.request_count,
                average_latency_ms: s.average_latency_ms,
                error_rate: s.error_rate,
                p95_latency_ms: s.p95_latency_ms,
            }).collect();
            Ok(Json(response))
        }
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

/// GET /api/analytics/clients - Get client usage statistics
pub async fn get_client_stats(
    State(state): State<AppState>,
    Query(params): Query<AnalyticsQueryParams>,
) -> Result<Json<Vec<ClientStats>>, StatusCode> {
    let query = build_analytics_query(params);
    
    match ApiAnalyticsService::get_client_stats(&query) {
        Ok(clients) => {
            let response: Vec<ClientStats> = clients.iter().map(|c| ClientStats {
                client_id: c.client_id.clone(),
                client_name: c.client_name.clone(),
                request_count: c.request_count,
                error_count: c.error_count,
                bandwidth_bytes: c.bandwidth_bytes,
                last_seen: c.last_seen.to_string(),
            }).collect();
            Ok(Json(response))
        }
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

/// GET /api/analytics/errors - Get error breakdown
pub async fn get_error_breakdown(
    State(state): State<AppState>,
    Query(params): Query<AnalyticsQueryParams>,
) -> Result<Json<Vec<ErrorResponse>>, StatusCode> {
    let query = build_analytics_query(params);
    
    match ApiAnalyticsService::get_error_breakdown(&query) {
        Ok(errors) => {
            let response: Vec<ErrorResponse> = errors.iter().map(|e| ErrorResponse {
                status_code: e.status_code,
                count: e.count,
                percentage: e.percentage,
                sample_errors: e.sample_errors.iter().map(|s| ErrorSample {
                    timestamp: s.timestamp.to_string(),
                    endpoint: s.endpoint.clone(),
                    method: s.method.clone(),
                    error_message: s.error_message.clone(),
                    client_id: s.client_id.clone(),
                }).collect(),
            }).collect();
            Ok(Json(response))
        }
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

/// GET /api/analytics/trends - Get usage trends over time
pub async fn get_usage_trends(
    State(state): State<AppState>,
    Query(params): Query<AnalyticsQueryParams>,
) -> Result<Json<UsageTrendResponse>, StatusCode> {
    let query = build_analytics_query(params);
    
    match ApiAnalyticsService::get_usage_trends(&query) {
        Ok((points, interval)) => {
            let response = UsageTrendResponse {
                points: points.iter().map(|p| UsageTrendPoint {
                    timestamp: p.timestamp.to_string(),
                    request_count: p.request_count,
                    error_count: p.error_count,
                    average_latency_ms: p.average_latency_ms,
                }).collect(),
                interval,
            };
            Ok(Json(response))
        }
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

/// GET /api/analytics/rate-limits/:client_id - Get rate limit status for a client
pub async fn get_rate_limit_status(
    State(state): State<AppState>,
    Path(client_id): Path<String>,
) -> Result<Json<RateLimitStats>, StatusCode> {
    match ApiAnalyticsService::get_rate_limit_status(&client_id) {
        Ok(status) => Ok(Json(RateLimitStats {
            client_id: status.client_id,
            limit: status.limit,
            remaining: status.remaining,
            reset_at: status.reset_at.to_string(),
            exceeded_count: status.exceeded_count,
        })),
        Err(_) => Err(StatusCode::NOT_FOUND),
    }
}

/// POST /api/analytics/alerts/configure - Configure alert thresholds
pub async fn configure_alerts(
    State(state): State<AppState>,
    Json(payload): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    // TODO: Configure alert thresholds
    Ok(Json(serde_json::json!({
        "status": "configured",
        "alerts": ["high_error_rate", "latency_spike", "rate_limit_exceeded"]
    })))
}

/// GET /api/analytics/export - Export analytics data
pub async fn export_analytics(
    State(state): State<AppState>,
    Query(params): Query<AnalyticsQueryParams>,
) -> Result<(Vec<u8>, &'static str), StatusCode> {
    let query = build_analytics_query(params);
    
    match ApiAnalyticsService::export_data(&query, "csv") {
        Ok(data) => Ok((data, "text/csv")),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

/// GET /api/analytics/dashboard - Get dashboard summary
pub async fn get_dashboard_summary(
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    // Get current metrics
    let now = Utc::now();
    let query = AnalyticsQuery {
        start_date: now - chrono::Duration::hours(24),
        end_date: now,
        endpoint: None,
        client_id: None,
        status_code: None,
        tenant_id: None,
    };
    
    match ApiAnalyticsService::get_metrics(&query) {
        Ok(metrics) => Ok(Json(serde_json::json!({
            "total_requests_24h": metrics.total_requests,
            "error_rate_24h": metrics.error_rate,
            "avg_latency_24h": metrics.average_latency_ms,
            "requests_per_second": metrics.requests_per_second,
            "top_endpoints": [],
            "active_clients": 0,
            "alerts_active": 0
        }))),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

fn build_analytics_query(params: AnalyticsQueryParams) -> AnalyticsQuery {
    let now = Utc::now();
    
    let (start_date, end_date) = if let Some(range) = params.time_range {
        match range.as_str() {
            "1h" => (now - chrono::Duration::hours(1), now),
            "24h" => (now - chrono::Duration::hours(24), now),
            "7d" => (now - chrono::Duration::days(7), now),
            "30d" => (now - chrono::Duration::days(30), now),
            _ => (now - chrono::Duration::hours(24), now),
        }
    } else {
        let start = params.start_date
            .and_then(|d| DateTime::parse_from_rfc3339(&d).ok())
            .map(|d| d.with_timezone(&Utc))
            .unwrap_or_else(|| now - chrono::Duration::hours(24));
        
        let end = params.end_date
            .and_then(|d| DateTime::parse_from_rfc3339(&d).ok())
            .map(|d| d.with_timezone(&Utc))
            .unwrap_or(now);
        
        (start, end)
    };
    
    AnalyticsQuery {
        start_date,
        end_date,
        endpoint: params.endpoint,
        client_id: params.client_id,
        status_code: params.status_code,
        tenant_id: params.tenant_id,
    }
}

pub fn api_analytics_router() -> axum::Router {
    axum::Router::new()
        .route("/metrics", axum::routing::get(get_metrics))
        .route("/endpoints", axum::routing::get(get_endpoint_stats))
        .route("/clients", axum::routing::get(get_client_stats))
        .route("/errors", axum::routing::get(get_error_breakdown))
        .route("/trends", axum::routing::get(get_usage_trends))
        .route("/rate-limits/:client_id", axum::routing::get(get_rate_limit_status))
        .route("/alerts/configure", axum::routing::post(configure_alerts))
        .route("/export", axum::routing::get(export_analytics))
        .route("/dashboard", axum::routing::get(get_dashboard_summary))
}
