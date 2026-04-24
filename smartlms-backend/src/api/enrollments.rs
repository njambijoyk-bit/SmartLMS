//! /enrollments endpoints — "my" views for the current user.
//!
//! Course-scoped staff views of enrollments live under /courses/:id/enrollments.

use axum::{
    extract::Extension,
    middleware,
    response::{IntoResponse, Response},
    routing::get,
    Json, Router,
};

use crate::middleware::auth::{require_auth, AuthUser};
use crate::services::course::{self, CourseError};
use crate::tenant::InstitutionCtx;

pub fn router() -> Router {
    Router::new()
        .route("/", get(list_my))
        .route_layer(middleware::from_fn(require_auth))
}

async fn list_my(
    Extension(ctx): Extension<InstitutionCtx>,
    Extension(user): Extension<AuthUser>,
) -> Response {
    match course::list_my_enrollments(&ctx.db_pool, &user).await {
        Ok(items) => Json(items).into_response(),
        Err(e) => map_error(e),
    }
}

fn map_error(e: CourseError) -> Response {
    use axum::http::StatusCode;
    use serde_json::json;
    let (status, code) = match &e {
        CourseError::NotFound => (StatusCode::NOT_FOUND, "not_found"),
        CourseError::Forbidden => (StatusCode::FORBIDDEN, "forbidden"),
        CourseError::SlugTaken => (StatusCode::CONFLICT, "slug_taken"),
        CourseError::NotPublished => (StatusCode::CONFLICT, "not_published"),
        CourseError::Db(err) => {
            tracing::error!(error = %err, "enrollment db error");
            (StatusCode::INTERNAL_SERVER_ERROR, "internal_error")
        }
    };
    (status, Json(json!({ "error": code }))).into_response()
}
