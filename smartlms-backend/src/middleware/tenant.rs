// Multi-tenant middleware - extracts institution from Host header and injects context
use axum::{
    body::Body,
    extract::{Request, State},
    http::{header::HOST, Method},
    middleware::Next,
    response::Response,
};
use crate::tenant::{InstitutionCtx, RouterState};

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

/// Helper to extract InstitutionCtx from request in handlers
pub mod axum_extract {
    use axum::{
        extract::Extension,
        http::Request,
    };
    use crate::tenant::InstitutionCtx;
    
    /// Extension extractor for InstitutionCtx
    pub async fn institution_ctx(
        Extension(ctx): Extension<InstitutionCtx>,
    ) -> InstitutionCtx {
        ctx
    }
}