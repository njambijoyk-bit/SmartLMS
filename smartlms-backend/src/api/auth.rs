//! /auth endpoints — register, login, refresh, logout.
//!
//! Every endpoint takes the per-institution `PgPool` from `InstitutionCtx`.
//! Unknown-host requests are rejected with 404 because there's no institution
//! context to authenticate against (master ref §2).

use axum::{
    extract::Extension,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::post,
    Json, Router,
};
use serde_json::json;
use std::net::IpAddr;
use validator::Validate;

use crate::models::auth::{LoginRequest, RefreshRequest, RegisterRequest};
use crate::services::auth::{self, AuthError, SessionMeta};
use crate::tenant::InstitutionCtx;

pub fn router() -> Router {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .route("/refresh", post(refresh))
        .route("/logout", post(logout))
}

fn meta(headers: &axum::http::HeaderMap) -> SessionMeta {
    let user_agent = headers
        .get(axum::http::header::USER_AGENT)
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string());
    let ip = headers
        .get("x-forwarded-for")
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.split(',').next())
        .and_then(|s| s.trim().parse::<IpAddr>().ok());
    SessionMeta { user_agent, ip }
}

async fn register(
    ctx: Option<Extension<InstitutionCtx>>,
    headers: axum::http::HeaderMap,
    Json(req): Json<RegisterRequest>,
) -> Response {
    let Extension(ctx) = match ctx {
        Some(c) => c,
        None => return not_found(),
    };
    if let Err(e) = req.validate() {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": e.to_string() })),
        )
            .into_response();
    }
    match auth::register(&ctx.db_pool, ctx.id, req, meta(&headers)).await {
        Ok(pair) => (StatusCode::CREATED, Json(pair)).into_response(),
        Err(e) => map_auth_error(e),
    }
}

async fn login(
    ctx: Option<Extension<InstitutionCtx>>,
    headers: axum::http::HeaderMap,
    Json(req): Json<LoginRequest>,
) -> Response {
    let Extension(ctx) = match ctx {
        Some(c) => c,
        None => return not_found(),
    };
    if let Err(e) = req.validate() {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": e.to_string() })),
        )
            .into_response();
    }
    match auth::login(&ctx.db_pool, ctx.id, req, meta(&headers)).await {
        Ok(pair) => (StatusCode::OK, Json(pair)).into_response(),
        Err(e) => map_auth_error(e),
    }
}

async fn refresh(
    ctx: Option<Extension<InstitutionCtx>>,
    headers: axum::http::HeaderMap,
    Json(req): Json<RefreshRequest>,
) -> Response {
    let Extension(ctx) = match ctx {
        Some(c) => c,
        None => return not_found(),
    };
    match auth::refresh(&ctx.db_pool, ctx.id, req, meta(&headers)).await {
        Ok(pair) => (StatusCode::OK, Json(pair)).into_response(),
        Err(e) => map_auth_error(e),
    }
}

async fn logout(
    ctx: Option<Extension<InstitutionCtx>>,
    Json(req): Json<RefreshRequest>,
) -> Response {
    let Extension(ctx) = match ctx {
        Some(c) => c,
        None => return not_found(),
    };
    match auth::logout(&ctx.db_pool, &req.refresh_token).await {
        Ok(_) => (StatusCode::NO_CONTENT, ()).into_response(),
        Err(e) => map_auth_error(e),
    }
}

fn not_found() -> Response {
    (
        StatusCode::NOT_FOUND,
        Json(json!({ "error": "unknown institution" })),
    )
        .into_response()
}

fn map_auth_error(e: AuthError) -> Response {
    let (status, code) = match &e {
        AuthError::EmailTaken => (StatusCode::CONFLICT, "email_taken"),
        AuthError::InvalidCredentials => (StatusCode::UNAUTHORIZED, "invalid_credentials"),
        AuthError::AccountLocked => (StatusCode::LOCKED, "account_locked"),
        AuthError::AccountDisabled => (StatusCode::FORBIDDEN, "account_disabled"),
        AuthError::InvalidRefreshToken => (StatusCode::UNAUTHORIZED, "invalid_refresh_token"),
        AuthError::Db(_) | AuthError::Password(_) | AuthError::Jwt(_) => {
            tracing::error!(error = ?e, "auth internal error");
            (StatusCode::INTERNAL_SERVER_ERROR, "internal_error")
        }
    };
    (status, Json(json!({ "error": code }))).into_response()
}
