//! DTOs for self-service institution onboarding.

use serde::{Deserialize, Serialize};
use validator::Validate;

use super::institution::Institution;
use crate::tenant::PlanTier;

/// Public signup request — creates an institution row. The admin user
/// itself is created via `POST /api/auth/register` once the caller switches
/// to the new institution's host (first registrant gets the `admin` role).
#[derive(Debug, Deserialize, Validate)]
pub struct SignupRequest {
    /// URL-safe identifier used as `<slug>.smartlms.io` until a custom
    /// domain is configured.
    #[validate(length(min = 2, max = 63), custom(function = "validate_slug"))]
    pub slug: String,
    #[validate(length(min = 1, max = 200))]
    pub name: String,
    pub plan_tier: Option<PlanTier>,
    /// Optional custom domain (e.g. `lms.uon.ac.ke`). If supplied, a matching
    /// entry is created in the master `domain_map` table.
    #[validate(length(max = 253))]
    pub custom_domain: Option<String>,
    #[validate(email)]
    pub admin_email: String,
    #[validate(length(min = 1, max = 200))]
    pub admin_display_name: String,
}

#[derive(Debug, Serialize)]
pub struct SignupResponse {
    pub institution: Institution,
    /// The host the new admin should `POST /api/auth/register` against to
    /// create their account. The first registrant becomes `admin` by
    /// convention (see services::auth::register).
    pub admin_host: String,
    pub next_steps: Vec<String>,
}

/// Slugs are lowercase alphanumeric, with optional internal single hyphens.
/// Chosen to match DNS label rules since the slug is used as a subdomain.
pub fn validate_slug(s: &str) -> Result<(), validator::ValidationError> {
    if s.len() < 2 || s.len() > 63 {
        return Err(validator::ValidationError::new("slug_length"));
    }
    let bytes = s.as_bytes();
    if !(bytes[0].is_ascii_alphanumeric() && bytes[bytes.len() - 1].is_ascii_alphanumeric()) {
        return Err(validator::ValidationError::new("slug_boundary"));
    }
    for b in bytes {
        if !(b.is_ascii_lowercase() || b.is_ascii_digit() || *b == b'-') {
            return Err(validator::ValidationError::new("slug_chars"));
        }
    }
    // No consecutive hyphens.
    if s.contains("--") {
        return Err(validator::ValidationError::new("slug_double_hyphen"));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::validate_slug;

    #[test]
    fn accepts_valid_slugs() {
        for s in [
            "uon",
            "strathmore-u",
            "k12-001",
            "demo",
            "a1",
            "uni-of-nairobi",
        ] {
            assert!(validate_slug(s).is_ok(), "{s} should be valid");
        }
    }

    #[test]
    fn rejects_invalid_slugs() {
        for s in [
            "",
            "a",
            "-bad",
            "bad-",
            "Bad",
            "bad_slug",
            "bad.slug",
            "bad--slug",
        ] {
            assert!(validate_slug(s).is_err(), "{s} should be rejected");
        }
    }
}
