//! /users endpoints — the Phase 1 subset (me, logout-everywhere).
//!
//! Admin-facing user management (list, create, role assignment) lands in
//! PR #55 alongside the institution onboarding service.

use axum::{
    extract::Extension,
    http::StatusCode,
    middleware,
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use serde_json::json;

use crate::db;
use crate::middleware::auth::{require_auth, AuthUser};
use crate::models::user::UserWithRoles;
use crate::services::auth;
use crate::tenant::InstitutionCtx;

pub fn router() -> Router {
    Router::new()
        .route("/me", get(me))
        .route("/me/logout-all", post(logout_all))
        .route_layer(middleware::from_fn(require_auth))
}

async fn me(
    Extension(ctx): Extension<InstitutionCtx>,
    Extension(user): Extension<AuthUser>,
) -> Response {
    let record = match db::user::find_by_id(&ctx.db_pool, user.id).await {
        Ok(Some(r)) => r,
        Ok(None) => return not_found(),
        Err(e) => {
            tracing::error!(error = %e, "db error on /users/me");
            return internal_error();
        }
    };
    let roles = match db::role::roles_for_user(&ctx.db_pool, user.id).await {
        Ok(r) => r,
        Err(e) => {
            tracing::error!(error = %e, "db error on /users/me roles");
            return internal_error();
        }
    };
    Json(UserWithRoles {
        user: record.user,
        roles,
    })
    .into_response()
}

async fn logout_all(
    Extension(ctx): Extension<InstitutionCtx>,
    Extension(user): Extension<AuthUser>,
) -> Response {
    match auth::logout_everywhere(&ctx.db_pool, user.id).await {
        Ok(count) => Json(json!({ "revoked": count })).into_response(),
        Err(e) => {
            tracing::error!(error = %e, "logout_all failed");
            internal_error()
        }
    }
}

fn not_found() -> Response {
    (StatusCode::NOT_FOUND, Json(json!({ "error": "not_found" }))).into_response()
}

fn internal_error() -> Response {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(json!({ "error": "internal_error" })),
    )
        .into_response()
}
