// License server - validates license keys, enforces plan tiers, quota limits
use crate::tenant::{PlanTier, QuotaLimits};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// License key details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LicenseKey {
    pub key: String,
    pub plan: PlanTier,
    pub max_users: i64,
    pub max_storage_mb: i64,
    pub issued_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
    pub is_active: bool,
}

/// License validation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LicenseStatus {
    pub valid: bool,
    pub plan: PlanTier,
    pub quotas: QuotaLimits,
    pub expires_at: Option<DateTime<Utc>>,
    pub message: Option<String>,
}

/// Feature availability by plan
pub struct FeatureMatrix;

impl FeatureMatrix {
    /// Check if feature is available for plan
    pub fn is_available(plan: PlanTier, feature: &str) -> bool {
        match feature {
            // Starter features
            "courses" | "assessments" | "grades" | "discussions" => true,
            "live_classes" | "video_hosting" => {
                matches!(plan, PlanTier::Growth | PlanTier::Enterprise)
            }
            "advanced_analytics" | "api_access" => {
                matches!(plan, PlanTier::Growth | PlanTier::Enterprise)
            }
            "proctoring" | "custom_domain" | "white_label" => {
                matches!(plan, PlanTier::Growth | PlanTier::Enterprise)
            }
            "priority_support" | "dedicated_infra" | "sla" => matches!(plan, PlanTier::Enterprise),
            "ml_engine" | "adaptive_learning" => {
                matches!(plan, PlanTier::Growth | PlanTier::Enterprise)
            }
            "library" | "employer_portal" | "competency_tracking" => {
                matches!(plan, PlanTier::Enterprise)
            }
            _ => false,
        }
    }

    /// Get all features for a plan
    pub fn features_for_plan(plan: PlanTier) -> Vec<&'static str> {
        let mut features = vec!["courses", "assessments", "grades", "discussions"];

        if matches!(plan, PlanTier::Growth | PlanTier::Enterprise) {
            features.extend([
                "live_classes",
                "video_hosting",
                "advanced_analytics",
                "api_access",
                "proctoring",
                "custom_domain",
                "white_label",
                "ml_engine",
                "adaptive_learning",
            ]);
        }

        if matches!(plan, PlanTier::Enterprise) {
            features.extend([
                "priority_support",
                "dedicated_infra",
                "sla",
                "library",
                "employer_portal",
                "competency_tracking",
            ]);
        }

        features
    }
}

/// Quota exceeded error
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuotaExceededError {
    pub resource: String,
    pub current: i64,
    pub limit: i64,
    pub upgrade_url: Option<String>,
}

/// Validate license key
pub fn validate_license(key: &str) -> Result<LicenseStatus, String> {
    // In production, this would:
    // 1. Check license format (encrypted, signed)
    // 2. Query license server or cache
    // 3. Verify expiration

    // Demo implementation - decode from format: PLAN-XXXX-XXXX-XXXX
    let parts: Vec<&str> = key.split('-').collect();
    if parts.len() != 4 {
        return Ok(LicenseStatus {
            valid: false,
            plan: PlanTier::Starter,
            quotas: QuotaLimits::default(),
            expires_at: None,
            message: Some("Invalid license key format".to_string()),
        });
    }

    let plan = match parts[0] {
        "STARTER" => PlanTier::Starter,
        "GROWTH" => PlanTier::Growth,
        "ENTERPRISE" => PlanTier::Enterprise,
        _ => PlanTier::Starter,
    };

    let quotas = match plan {
        PlanTier::Starter => QuotaLimits {
            max_users: 1000,
            max_courses: 100,
            max_storage_mb: 1024,
            max_concurrent_users: 100,
        },
        PlanTier::Growth => QuotaLimits {
            max_users: 10000,
            max_courses: 1000,
            max_storage_mb: 10240,
            max_concurrent_users: 500,
        },
        PlanTier::Enterprise => QuotaLimits {
            max_users: i64::MAX,
            max_courses: i64::MAX,
            max_storage_mb: i64::MAX,
            max_concurrent_users: i64::MAX,
        },
    };

    Ok(LicenseStatus {
        valid: true,
        plan,
        quotas,
        expires_at: None,
        message: Some("License valid".to_string()),
    })
}

/// Check quota before creating resource
pub fn check_quota(current: i64, limit: i64, resource: &str) -> Result<(), QuotaExceededError> {
    if current >= limit {
        return Err(QuotaExceededError {
            resource: resource.to_string(),
            current,
            limit,
            upgrade_url: Some("/billing/upgrade".to_string()),
        });
    }
    Ok(())
}

/// Generate license key (for testing/admin)
pub fn generate_license_key(plan: PlanTier) -> String {
    let prefix = match plan {
        PlanTier::Starter => "STARTER",
        PlanTier::Growth => "GROWTH",
        PlanTier::Enterprise => "ENTERPRISE",
    };

    let random: String = (0..3)
        .map(|_| {
            let idx = rand::random::<u8>() % 36;
            if idx < 10 {
                (b'0' + idx) as char
            } else {
                (b'A' + idx - 10) as char
            }
        })
        .collect();

    format!("{}-{}000-0000-0000", prefix, random)
}
