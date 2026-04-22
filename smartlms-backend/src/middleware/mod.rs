//! Request/response middleware. Phase 0: just the tenant resolver. Phase 1
//! PR #54 will re-add the JWT auth middleware once `User` and the auth
//! service exist again.

pub mod tenant;

pub use tenant::tenant_middleware;
