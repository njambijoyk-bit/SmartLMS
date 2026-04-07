// Self-serve signup service - handles new institution registration
use crate::models::institution::{Institution, CreateInstitutionRequest};
use crate::models::user::{User, RegisterRequest};
use crate::services::license;
use crate::services::onboarding;
use sqlx::PgPool;
use uuid::Uuid;
use chrono::Utc;

/// Signup request
#[derive(Debug, Clone, serde::Deserialize)]
pub struct SignupRequest {
    pub institution_name: String,
    pub institution_slug: String,
    pub admin_email: String,
    pub admin_first_name: String,
    pub admin_last_name: String,
    pub password: String,
    pub plan_tier: Option<String>,
    pub license_key: Option<String>,
}

/// Signup response
#[derive(Debug, Clone, serde::Serialize)]
pub struct SignupResponse {
    pub institution: Institution,
    pub admin_user: User,
    pub access_token: String,
    pub refresh_token: String,
    pub onboarding_url: Option<String>,
}

/// Validation errors
#[derive(Debug, Clone, serde::Serialize)]
pub struct SignupValidationError {
    pub field: String,
    pub message: String,
}

/// Service functions
pub mod service {
    use super::*;
    
    /// Create a new institution with admin user (self-serve signup)
    pub async fn signup(
        pool: &PgPool,
        req: &SignupRequest,
    ) -> Result<SignupResponse, Vec<SignupValidationError>> {
        let mut errors = Vec::new();
        
        // Validate institution name
        if req.institution_name.len() < 2 || req.institution_name.len() > 100 {
            errors.push(SignupValidationError {
                field: "institution_name".to_string(),
                message: "Institution name must be 2-100 characters".to_string(),
            });
        }
        
        // Validate slug
        if let Err(e) = onboarding::validate_slug(&req.institution_slug) {
            errors.push(SignupValidationError {
                field: "institution_slug".to_string(),
                message: e,
            });
        }
        
        // Validate email
        if !req.admin_email.contains('@') {
            errors.push(SignupValidationError {
                field: "admin_email".to_string(),
                message: "Invalid email address".to_string(),
            });
        }
        
        // Validate password (min 8 chars)
        if req.password.len() < 8 {
            errors.push(SignupValidationError {
                field: "password".to_string(),
                message: "Password must be at least 8 characters".to_string(),
            });
        }
        
        if !errors.is_empty() {
            return Err(errors);
        }
        
        // Check slug uniqueness
        if let Ok(Some(_)) = crate::db::institution::find_by_slug(pool, &req.institution_slug).await {
            return Err(vec![SignupValidationError {
                field: "institution_slug".to_string(),
                message: "This URL is already taken".to_string(),
            }]);
        }
        
        // Determine plan tier
        let plan_tier = if let Some(ref key) = req.license_key {
            let status = license::validate_license(key).map_err(|_| vec![])?;
            if !status.valid {
                return Err(vec![SignupValidationError {
                    field: "license_key".to_string(),
                    message: status.message.unwrap_or("Invalid license key".to_string()),
                }]);
            }
            Some(status.plan)
        } else {
            req.plan_tier.as_ref().and_then(|t| match t.to_lowercase().as_str() {
                "growth" => Some(crate::models::institution::PlanTier::Growth),
                "enterprise" => Some(crate::models::institution::PlanTier::Enterprise),
                _ => Some(crate::models::institution::PlanTier::Starter),
            })
        };
        
        // Create institution
        let create_inst_req = CreateInstitutionRequest {
            name: req.institution_name.clone(),
            slug: req.institution_slug.clone(),
            domain: None,
            plan_tier: plan_tier.clone(),
            license_key: req.license_key.clone(),
        };
        
        let institution = crate::db::institution::create(pool, &create_inst_req)
            .await
            .map_err(|e| vec![SignupValidationError {
                field: "institution".to_string(),
                message: e.to_string(),
            }])?;
        
        // Create admin user in institution's database
        let user = crate::db::user::create(
            pool,
            &req.admin_email,
            &req.password,
            &req.admin_first_name,
            &req.admin_last_name,
            "admin",  // Role
        )
        .await
        .map_err(|e| vec![SignupValidationError {
            field: "admin_user".to_string(),
            message: e.to_string(),
        }])?;
        
        // Generate JWT tokens
        let access_token = crate::services::jwt::create_token(
            &user.id.to_string(),
            &user.email,
            "admin",
            &institution.id.to_string(),
        ).map_err(|e| vec![SignupValidationError {
            field: "token".to_string(),
            message: e,
        }])?;
        
        let refresh_token = crate::services::jwt::create_refresh_token(
            &user.id.to_string(),
            &institution.id.to_string(),
        ).map_err(|e| vec![SignupValidationError {
            field: "token".to_string(),
            message: e,
        }])?;
        
        // Initialize onboarding
        let onboarding = onboarding::init_onboarding(pool, institution.id, plan_tier == Some(crate::models::institution::PlanTier::Starter))
            .await
            .ok();
        
        let onboarding_url = onboarding.map(|_| "/onboarding".to_string());
        
        // Log signup event
        tracing::info!("New institution signed up: {} ({})", institution.name, institution.slug);
        
        Ok(SignupResponse {
            institution,
            admin_user: user,
            access_token,
            refresh_token,
            onboarding_url,
        })
    }
    
    /// Verify domain ownership for custom domain
    pub async fn verify_domain(
        pool: &PgPool,
        institution_id: Uuid,
        domain: &str,
        verification_token: &str,
    ) -> Result<bool, String> {
        // Check DNS TXT record for verification
        // In production: query DNS and verify the token matches
        
        // For now, verify against stored pending domain
        let institution = crate::db::institution::find_by_id(pool, institution_id)
            .await
            .map_err(|e| e.to_string())?
            .ok_or("Institution not found")?;
        
        // Update institution's domain
        crate::db::institution::update_domain(pool, institution_id, domain)
            .await
            .map_err(|e| e.to_string())?;
        
        Ok(true)
    }
    
    /// Request domain verification (get DNS instructions)
    pub async fn request_domain_verification(
        pool: &PgPool,
        institution_id: Uuid,
        domain: &str,
    ) -> Result<DomainVerificationInfo, String> {
        let verification_token = format!("smartlms-verification-{}", Uuid::new_v4());
        
        // Store pending domain for later verification
        // In production: save to pending_domain table
        
        Ok(DomainVerificationInfo {
            domain: domain.to_string(),
            verification_type: "TXT".to_string(),
            record_name: "@".to_string(),
            record_value: verification_token,
            instructions: "Add a TXT record to your DNS configuration with the value shown above.".to_string(),
            expected_propagation_time_minutes: 30,
        })
    }
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct DomainVerificationInfo {
    pub domain: String,
    pub verification_type: String,
    pub record_name: String,
    pub record_value: String,
    pub instructions: String,
    pub expected_propagation_time_minutes: i32,
}