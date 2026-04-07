// Institution model for master database
use crate::tenant::{InstitutionConfig, PlanTier, QuotaLimits};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Institution entity stored in master database
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Institution {
    pub id: uuid::Uuid,
    pub slug: String,
    pub name: String,
    pub domain: Option<String>,
    pub database_url: Option<String>,
    pub config: Option<InstitutionConfig>,
    pub plan_tier: Option<PlanTier>,
    pub quotas: Option<QuotaLimits>,
    pub license_key: Option<String>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Create new institution request
#[derive(Debug, Deserialize)]
pub struct CreateInstitutionRequest {
    pub slug: String,
    pub name: String,
    pub domain: Option<String>,
    pub plan_tier: Option<PlanTier>,
}

/// Update institution request
#[derive(Debug, Deserialize)]
pub struct UpdateInstitutionRequest {
    pub name: Option<String>,
    pub domain: Option<String>,
    pub config: Option<InstitutionConfig>,
    pub plan_tier: Option<PlanTier>,
    pub is_active: Option<bool>,
}

/// List institutions with pagination
#[derive(Debug, Serialize)]
pub struct InstitutionListResponse {
    pub institutions: Vec<Institution>,
    pub total: i64,
    pub page: i64,
    pub per_page: i64,
}
