// Multi-tenant module - handles institution isolation
pub mod context;

pub use context::{InstitutionConfig, InstitutionCtx, PlanTier, QuotaLimits, RouterState};
