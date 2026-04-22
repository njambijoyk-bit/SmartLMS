//! JWT auth middleware.
//!
//! Verifies the `Authorization: Bearer <token>` header, decodes the JWT,
//! cross-checks the `tid` claim against the current `InstitutionCtx`, and
//! injects `AuthUser` into the request's extensions. Handlers that want
//! auth take `Extension<AuthUser>`; handlers that are intentionally public
//! (login, register, /health) just don't.
//!
//! Cross-institution token replay is rejected: a token issued by institution
//! A presented against institution B returns 401.

use axum::{
    body::Body,
    extract::Request,
    http::{header::AUTHORIZATION, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

use crate::models::user::RoleCode;
use crate::services::jwt;
use crate::tenant::InstitutionCtx;

/// The decoded-and-verified identity of the caller. Injected by
/// `auth_middleware` so handlers can take `Extension<AuthUser>`.
#[derive(Debug, Clone)]
pub struct AuthUser {
    pub id: uuid::Uuid,
    pub institution_id: uuid::Uuid,
    pub email: String,
    pub roles: Vec<String>,
}

impl AuthUser {
    pub fn has_role(&self, role: RoleCode) -> bool {
        self.roles.iter().any(|r| r == role.as_str())
    }

    pub fn is_admin(&self) -> bool {
        self.has_role(RoleCode::Admin)
    }
}

/// Requires a valid JWT whose `tid` matches the current `InstitutionCtx`.
/// Returns 401 on any failure — no information about why (master ref §8).
pub async fn require_auth(mut request: Request<Body>, next: Next) -> Response {
    let token = match bearer_token(&request) {
        Some(t) => t,
        None => return unauthorized(),
    };

    let claims = match jwt::decode_access_token(token) {
        Ok(c) => c,
        Err(_) => return unauthorized(),
    };

    // Cross-tenant defence: if a tenant context was resolved by the tenant
    // middleware, the token's `tid` MUST match. If no tenant context exists
    // (unknown host), reject — authenticated endpoints must be tenant-scoped.
    let ctx = match request.extensions().get::<InstitutionCtx>() {
        Some(ctx) => ctx.clone(),
        None => return unauthorized(),
    };
    if claims.tid != ctx.id {
        return unauthorized();
    }

    let user = AuthUser {
        id: claims.sub,
        institution_id: claims.tid,
        email: claims.email,
        roles: claims.roles,
    };
    request.extensions_mut().insert(user);
    next.run(request).await
}

fn bearer_token(request: &Request<Body>) -> Option<&str> {
    request
        .headers()
        .get(AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .and_then(|s| {
            s.strip_prefix("Bearer ")
                .or_else(|| s.strip_prefix("bearer "))
        })
}

fn unauthorized() -> Response {
    (
        StatusCode::UNAUTHORIZED,
        Json(json!({ "error": "unauthorized" })),
    )
        .into_response()
}
