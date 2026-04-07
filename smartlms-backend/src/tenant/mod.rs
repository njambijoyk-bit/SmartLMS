// Multi-tenant module - handles institution isolation
pub mod context;

pub use context::{InstitutionCtx, InstitutionConfig, PlanTier, QuotaLimits, RouterState};