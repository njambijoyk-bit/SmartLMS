//! Institution models

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Institution entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Institution {
    pub id: Uuid,
    pub slug: String,
    pub name: String,
    pub domain: Option<String>,
    pub logo_url: Option<String>,
    pub plan_tier: PlanTier,
    pub status: InstitutionStatus,
    pub settings: InstitutionSettings,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Institution {
    pub fn new(slug: String, name: String) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            slug,
            name,
            domain: None,
            logo_url: None,
            plan_tier: PlanTier::Starter,
            status: InstitutionStatus::Active,
            settings: InstitutionSettings::default(),
            created_at: now,
            updated_at: now,
        }
    }
}

/// Institution status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum InstitutionStatus {
    Active,
    Suspended,
    Trial,
    Deleted,
}

/// Plan tier
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PlanTier {
    Starter,
    Growth,
    Enterprise,
}

impl Default for PlanTier {
    fn default() -> Self {
        PlanTier::Starter
    }
}

/// Institution settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstitutionSettings {
    pub timezone: String,
    pub date_format: String,
    pub language: String,
    pub require_email_verification: bool,
    pub allow_self_registration: bool,
    pub max_users: i32,
    pub max_courses: i32,
    pub storage_limit_gb: i32,
    pub custom_css: Option<String>,
    pub primary_color: Option<String>,
    pub secondary_color: Option<String>,
}

impl Default for InstitutionSettings {
    fn default() -> Self {
        Self {
            timezone: "UTC".to_string(),
            date_format: "YYYY-MM-DD".to_string(),
            language: "en".to_string(),
            require_email_verification: true,
            allow_self_registration: false,
            max_users: 1000,
            max_courses: 100,
            storage_limit_gb: 10,
            custom_css: None,
            primary_color: None,
            secondary_color: None,
        }
    }
}

/// Create institution request
#[derive(Debug, Deserialize)]
pub struct CreateInstitutionRequest {
    pub slug: String,
    pub name: String,
    pub domain: Option<String>,
    pub admin_email: String,
    pub admin_password: String,
    pub plan_tier: Option<PlanTier>,
}

/// Update institution request
#[derive(Debug, Deserialize)]
pub struct UpdateInstitutionRequest {
    pub name: Option<String>,
    pub domain: Option<String>,
    pub logo_url: Option<String>,
    pub plan_tier: Option<PlanTier>,
    pub settings: Option<InstitutionSettings>,
}

/// Institution response
#[derive(Debug, Serialize)]
pub struct InstitutionResponse {
    pub id: Uuid,
    pub slug: String,
    pub name: String,
    pub domain: Option<String>,
    pub logo_url: Option<String>,
    pub plan_tier: PlanTier,
    pub status: InstitutionStatus,
    pub created_at: DateTime<Utc>,
}