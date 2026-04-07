// Institution onboarding service - signup, setup wizard, sandbox
use crate::models::institution::{CreateInstitutionRequest, Institution};
use crate::tenant::{InstitutionConfig, PlanTier, QuotaLimits};
use chrono::{Duration, Utc};
use sqlx::PgPool;

/// Onboarding step tracking
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct OnboardingStep {
    pub step: i32,
    pub completed: bool,
    pub data: serde_json::Value,
}

/// Complete onboarding state for an institution
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct OnboardingState {
    pub institution_id: uuid::Uuid,
    pub current_step: i32,
    pub steps: Vec<OnboardingStep>,
    pub started_at: chrono::DateTime<Utc>,
    pub expires_at: chrono::DateTime<Utc>,
    pub is_sandbox: bool,
}

impl OnboardingState {
    pub fn new(institution_id: uuid::Uuid, is_sandbox: bool) -> Self {
        let now = Utc::now();
        let expiry = if is_sandbox {
            now + Duration::days(14) // 14-day sandbox
        } else {
            now + Duration::days(30) // 30 days for production
        };

        Self {
            institution_id,
            current_step: 1,
            steps: vec![
                OnboardingStep {
                    step: 1,
                    completed: false,
                    data: serde_json::json!({}),
                },
                OnboardingStep {
                    step: 2,
                    completed: false,
                    data: serde_json::json!({}),
                },
                OnboardingStep {
                    step: 3,
                    completed: false,
                    data: serde_json::json!({}),
                },
                OnboardingStep {
                    step: 4,
                    completed: false,
                    data: serde_json::json!({}),
                },
                OnboardingStep {
                    step: 5,
                    completed: false,
                    data: serde_json::json!({}),
                },
            ],
            started_at: now,
            expires_at: expiry,
            is_sandbox,
        }
    }

    /// Check if onboarding is expired
    pub fn is_expired(&self) -> bool {
        Utc::now() > self.expires_at
    }

    /// Get progress percentage
    pub fn progress(&self) -> f32 {
        let completed = self.steps.iter().filter(|s| s.completed).count() as f32;
        (completed / self.steps.len() as f32) * 100.0
    }
}

/// Onboarding step definitions (for documentation/API)
pub const ONBOARDING_STEPS: &[&str] = &[
    "Basic Info",   // Name, slug, type
    "Admin User",   // Create first admin
    "Database",     // Configure DB connection
    " Branding",    // Logo, colors, domain
    "Verification", // Email verification, license key
];

/// Create new institution during signup
pub async fn create_institution(
    pool: &PgPool,
    req: &CreateInstitutionRequest,
    is_sandbox: bool,
) -> Result<Institution, String> {
    // Check slug uniqueness
    if crate::db::institution::find_by_slug(pool, &req.slug)
        .await
        .map_err(|e| e.to_string())?
        .is_some()
    {
        return Err("Institution slug already taken".to_string());
    }

    // Create with default config based on plan
    let (plan, quotas) = if is_sandbox {
        (Some(PlanTier::Starter), Some(QuotaLimits::default()))
    } else {
        (req.plan_tier, None)
    };

    let institution = crate::db::institution::create(pool, req)
        .await
        .map_err(|e| e.to_string())?;

    // Note: In production, you'd also provision the per-institution database here

    Ok(Institution {
        id: institution.id,
        slug: institution.slug,
        name: institution.name,
        domain: institution.domain,
        database_url: institution.database_url,
        config: None,
        plan_tier: plan,
        quotas,
        license_key: None,
        is_active: true,
        created_at: institution.created_at,
        updated_at: institution.updated_at,
    })
}

/// Initialize onboarding for new institution
pub async fn init_onboarding(
    pool: &PgPool,
    institution_id: uuid::Uuid,
    is_sandbox: bool,
) -> Result<OnboardingState, String> {
    let state = OnboardingState::new(institution_id, is_sandbox);

    // Store in DB (implementation depends on your schema)
    // For now, return the state
    Ok(state)
}

/// Get onboarding state
pub async fn get_onboarding_state(
    pool: &PgPool,
    institution_id: uuid::Uuid,
) -> Result<Option<OnboardingState>, String> {
    // TODO: Query from DB
    Ok(None)
}

/// Complete an onboarding step
pub async fn complete_step(
    pool: &PgPool,
    institution_id: uuid::Uuid,
    step: i32,
    data: serde_json::Value,
) -> Result<OnboardingState, String> {
    // TODO: Update in DB
    // For now, return placeholder
    Err("Not implemented".to_string())
}

/// Generate sample data for sandbox
pub async fn generate_sandbox_data(
    pool: &PgPool,
    institution_id: uuid::Uuid,
) -> Result<(), String> {
    // TODO: Create sample courses, users, assignments for demo purposes
    Ok(())
}

/// Validate institution slug (alphanumeric + hyphens)
pub fn validate_slug(slug: &str) -> Result<(), String> {
    if slug.len() < 3 || slug.len() > 50 {
        return Err("Slug must be 3-50 characters".to_string());
    }

    if !slug.chars().all(|c| c.is_alphanumeric() || c == '-') {
        return Err("Slug can only contain letters, numbers, and hyphens".to_string());
    }

    if slug.chars().next().unwrap().is_numeric() {
        return Err("Slug cannot start with a number".to_string());
    }

    Ok(())
}

/// Check if sandbox has expired and apply restrictions
pub fn check_sandbox_status(state: &OnboardingState) -> SandboxStatus {
    if state.is_expired() {
        return SandboxStatus::Expired;
    }

    let days_remaining = (state.expires_at - Utc::now()).num_days();

    if days_remaining <= 3 {
        SandboxStatus::ExpiringSoon(days_remaining as i32)
    } else {
        SandboxStatus::Active(days_remaining as i32)
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum SandboxStatus {
    Active(i32),       // Days remaining
    ExpiringSoon(i32), // Days until expiry
    Expired,
}
