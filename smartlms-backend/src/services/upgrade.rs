// Upgrade Service - handles plan tier changes and license management
use crate::models::institution::{Institution, PlanTier};
use crate::services::license::{self, LicenseStatus};
use sqlx::PgPool;
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// Upgrade request
#[derive(Debug, Clone, serde::Deserialize)]
pub struct UpgradeRequest {
    pub target_tier: PlanTier,
    pub license_key: Option<String>,
    pub payment_method_id: Option<String>,
}

/// Upgrade result
#[derive(Debug, Clone, serde::Serialize)]
pub struct UpgradeResult {
    pub success: bool,
    pub institution: Institution,
    pub previous_tier: PlanTier,
    pub new_tier: PlanTier,
    pub effective_from: DateTime<Utc>,
    pub message: Option<String>,
}

/// Downgrade request (with grace period)
#[derive(Debug, Clone, serde::Deserialize)]
pub struct DowngradeRequest {
    pub target_tier: PlanTier,
    pub reason: Option<String>,
}

/// Feature comparison between tiers
#[derive(Debug, Clone, serde::Serialize)]
pub struct TierFeatureComparison {
    pub current_tier: PlanTier,
    pub target_tier: PlanTier,
    pub features_gained: Vec<String>,
    pub features_lost: Vec<String>,
    pub quota_changes: QuotaDiff,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct QuotaDiff {
    pub users: (i64, i64),       // (current, new)
    pub courses: (i64, i64),
    pub storage_mb: (i64, i64),
    pub concurrent: (i64, i64),
}

/// Service functions
pub mod service {
    use super::*;
    
    /// Upgrade an institution to a new plan tier
    pub async fn upgrade_institution(
        pool: &PgPool,
        institution_id: Uuid,
        req: &UpgradeRequest,
    ) -> Result<UpgradeResult, String> {
        // Get current institution
        let institution = crate::db::institution::find_by_id(pool, institution_id)
            .await
            .map_err(|e| e.to_string())?
            .ok_or("Institution not found")?;
        
        let previous_tier = institution.plan_tier.unwrap_or(PlanTier::Starter);
        
        // Validate upgrade path
        if !is_valid_upgrade_path(previous_tier, req.target_tier) {
            return Err(format!("Invalid upgrade path from {:?} to {:?}", previous_tier, req.target_tier));
        }
        
        // If new license key provided, validate it
        let new_license = if let Some(ref key) = req.license_key {
            let status = license::validate_license(key)
                .map_err(|e| e.to_string())?;
            
            if !status.valid {
                return Err(status.message.unwrap_or("Invalid license key".to_string()));
            }
            
            if status.plan != req.target_tier {
                return Err(format!("License key is for {:?}, requested {:?}", status.plan, req.target_tier));
            }
            
            Some(key.clone())
        } else {
            None
        };
        
        // In production: process payment here
        
        // Update institution with new plan
        let mut updated_institution = institution.clone();
        updated_institution.plan_tier = Some(req.target_tier);
        updated_institution.updated_at = Utc::now();
        
        if let Some(ref lic) = new_license {
            updated_institution.license_key = Some(lic.clone());
        }
        
        // Update quotas based on new tier
        updated_institution.quotas = Some(get_quotas_for_tier(req.target_tier));
        
        // Save to DB
        crate::db::institution::update_plan_tier(pool, institution_id, req.target_tier, new_license)
            .await
            .map_err(|e| e.to_string())?;
        
        // Log the upgrade event
        log_upgrade_event(pool, institution_id, previous_tier, req.target_tier).await;
        
        Ok(UpgradeResult {
            success: true,
            institution: updated_institution,
            previous_tier,
            new_tier: req.target_tier,
            effective_from: Utc::now(),
            message: Some(format!("Successfully upgraded to {:?}", req.target_tier)),
        })
    }
    
    /// Downgrade with grace period (current features preserved until period ends)
    pub async fn downgrade_institution(
        pool: &PgPool,
        institution_id: Uuid,
        req: &DowngradeRequest,
    ) -> Result<UpgradeResult, String> {
        let institution = crate::db::institution::find_by_id(pool, institution_id)
            .await
            .map_err(|e| e.to_string())?
            .ok_or("Institution not found")?;
        
        let previous_tier = institution.plan_tier.unwrap_or(PlanTier::Starter);
        
        // Downgrades go into effect at end of billing period
        // For simplicity, we'll use a 30-day grace period
        let grace_period_end = Utc::now() + chrono::Duration::days(30);
        
        // Log downgrade request
        log_downgrade_event(pool, institution_id, previous_tier, req.target_tier, req.reason.clone()).await;
        
        Ok(UpgradeResult {
            success: true,
            institution,
            previous_tier,
            new_tier: req.target_tier,
            effective_from: grace_period_end,
            message: Some(format!("Downgrade scheduled. Changes take effect in 30 days.")),
        })
    }
    
    /// Compare features between tiers
    pub fn compare_tiers(current: PlanTier, target: PlanTier) -> TierFeatureComparison {
        let current_features = get_features_for_tier(current);
        let target_features = get_features_for_tier(target);
        
        let features_gained: Vec<String> = target_features.iter()
            .filter(|f| !current_features.contains(f))
            .cloned()
            .collect();
        
        let features_lost: Vec<String> = current_features.iter()
            .filter(|f| !target_features.contains(f))
            .cloned()
            .collect();
        
        let current_quotas = get_quotas_for_tier(current);
        let target_quotas = get_quotas_for_tier(target);
        
        TierFeatureComparison {
            current_tier: current,
            target_tier: target,
            features_gained,
            features_lost,
            quota_changes: QuotaDiff {
                users: (current_quotas.max_users, target_quotas.max_users),
                courses: (current_quotas.max_courses, target_quotas.max_courses),
                storage_mb: (current_quotas.max_storage_mb, target_quotas.max_storage_mb),
                concurrent: (current_quotas.max_concurrent_users, target_quotas.max_concurrent_users),
            },
        }
    }
    
    /// Check if user can perform action based on current plan
    pub fn check_feature_available(pool: &PgPool, institution_id: Uuid, feature: &str) -> Result<bool, String> {
        // Get institution's current plan
        let institution = crate::db::institution::find_by_id(pool, institution_id)
            .await
            .map_err(|e| e.to_string())?
            .ok_or("Institution not found")?;
        
        let tier = institution.plan_tier.unwrap_or(PlanTier::Starter);
        
        Ok(crate::services::license::FeatureMatrix::is_available(tier, feature))
    }
}

fn is_valid_upgrade_path(from: PlanTier, to: PlanTier) -> bool {
    match (from, to) {
        (PlanTier::Starter, PlanTier::Growth) => true,
        (PlanTier::Starter, PlanTier::Enterprise) => true,
        (PlanTier::Growth, PlanTier::Enterprise) => true,
        _ => from == to,  // Same tier is valid (no change)
    }
}

fn get_quotas_for_tier(tier: PlanTier) -> crate::tenant::QuotaLimits {
    match tier {
        PlanTier::Starter => crate::tenant::QuotaLimits {
            max_users: 1000,
            max_courses: 100,
            max_storage_mb: 1024,
            max_concurrent_users: 100,
        },
        PlanTier::Growth => crate::tenant::QuotaLimits {
            max_users: 10000,
            max_courses: 1000,
            max_storage_mb: 10240,
            max_concurrent_users: 500,
        },
        PlanTier::Enterprise => crate::tenant::QuotaLimits {
            max_users: i64::MAX,
            max_courses: i64::MAX,
            max_storage_mb: i64::MAX,
            max_concurrent_users: i64::MAX,
        },
    }
}

fn get_features_for_tier(tier: PlanTier) -> Vec<String> {
    crate::services::license::FeatureMatrix::features_for_plan(tier)
        .iter()
        .map(|s| s.to_string())
        .collect()
}

async fn log_upgrade_event(pool: &PgPool, institution_id: Uuid, from: PlanTier, to: PlanTier) {
    // In production, insert into audit log
    tracing::info!("Institution {} upgraded from {:?} to {:?}", institution_id, from, to);
}

async fn log_downgrade_event(pool: &PgPool, institution_id: Uuid, from: PlanTier, to: PlanTier, reason: Option<String>) {
    tracing::info!("Institution {} scheduled downgrade from {:?} to {:?}. Reason: {:?}", 
        institution_id, from, to, reason);
}