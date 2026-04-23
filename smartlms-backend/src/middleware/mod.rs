//! Request/response middleware. Phase 1: tenant resolver (Host → InstitutionCtx)
//! plus JWT auth middleware that cross-checks the token's institution claim.

pub mod auth;
pub mod tenant;

pub use auth::{require_auth, AuthUser};
pub use tenant::tenant_middleware;
