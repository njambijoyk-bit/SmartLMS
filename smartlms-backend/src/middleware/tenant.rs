//! Multi-tenant middleware: resolves `Host` header → `InstitutionCtx` and
//! injects it into the request's extensions. Handlers that don't declare an
//! `Extension<InstitutionCtx>` simply don't see it (unknown-host requests
//! still pass through so `/health` etc. keep working).

use crate::tenant::RouterState;
use axum::{
    body::Body,
    extract::{Request, State},
    http::header::HOST,
    middleware::Next,
    response::Response,
};

/// Extract institution context from Host header and inject as request extension
pub async fn tenant_middleware(
    State(state): State<RouterState>,
    mut request: Request<Body>,
    next: Next,
) -> Response {
    // Get Host header
    let host = request
        .headers()
        .get(HOST)
        .and_then(|h| h.to_str().ok())
        .unwrap_or("localhost");

    // Resolve institution context
    if let Some(ctx) = state.resolve_institution(host).await {
        // Inject institution context into request extensions
        request.extensions_mut().insert(ctx);
    }

    // Continue with request
    next.run(request).await
}

/// Helper to extract `InstitutionCtx` from request in handlers.
pub mod axum_extract {
    use crate::tenant::InstitutionCtx;
    use axum::extract::Extension;

    /// Extension extractor for `InstitutionCtx`.
    pub async fn institution_ctx(Extension(ctx): Extension<InstitutionCtx>) -> InstitutionCtx {
        ctx
    }
}
