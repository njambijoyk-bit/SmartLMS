//! /institutions admin endpoints (master-DB scoped).
//!
//! These endpoints are intentionally UNAUTHENTICATED in Phase 1 — they run
//! against the master DB, not any tenant, so `tenant_middleware` doesn't
//! give them an `InstitutionCtx`. A follow-up PR will add operator-level
//! auth (either a static admin token or a privileged role on the master
//! institution) before Phase 2.

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use serde::Deserialize;
use serde_json::json;

use crate::db;
use crate::models::institution::InstitutionListResponse;
use crate::models::onboarding::SignupRequest;
use crate::services::onboarding::{self, OnboardingError};
use crate::tenant::RouterState;
use validator::Validate;

pub fn router() -> Router<RouterState> {
    Router::new()
        .route("/", get(list))
        .route("/:slug", get(get_by_slug))
        .route("/signup", post(signup))
}

#[derive(Debug, Deserialize)]
pub struct ListQuery {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}

async fn list(State(state): State<RouterState>, Query(q): Query<ListQuery>) -> Response {
    let page = q.page.unwrap_or(1).max(1);
    let per_page = q.per_page.unwrap_or(50).clamp(1, 200);

    match onboarding::list_active(&state.master_pool, page, per_page).await {
        Ok((institutions, total)) => Json(InstitutionListResponse {
            institutions,
            total,
            page,
            per_page,
        })
        .into_response(),
        Err(e) => {
            tracing::error!(error = %e, "list institutions failed");
            internal_error()
        }
    }
}

async fn get_by_slug(State(state): State<RouterState>, Path(slug): Path<String>) -> Response {
    match db::institution::find_by_slug(&state.master_pool, &slug).await {
        Ok(Some(inst)) => Json(inst).into_response(),
        Ok(None) => not_found(),
        Err(e) => {
            tracing::error!(error = %e, "institution lookup failed");
            internal_error()
        }
    }
}

async fn signup(State(state): State<RouterState>, Json(req): Json<SignupRequest>) -> Response {
    if let Err(e) = req.validate() {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": "validation_failed", "detail": e.to_string() })),
        )
            .into_response();
    }

    match onboarding::signup(&state, req).await {
        Ok(resp) => (StatusCode::CREATED, Json(resp)).into_response(),
        Err(OnboardingError::SlugTaken) => {
            (StatusCode::CONFLICT, Json(json!({ "error": "slug_taken" }))).into_response()
        }
        Err(OnboardingError::DomainTaken) => (
            StatusCode::CONFLICT,
            Json(json!({ "error": "domain_taken" })),
        )
            .into_response(),
        Err(OnboardingError::Db(e)) => {
            tracing::error!(error = %e, "signup failed");
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
