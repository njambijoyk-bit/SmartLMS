//! Institution onboarding — self-service signup + provisioning.
//!
//! Creates an `institutions` row in the master DB, optionally a
//! `domain_map` entry for custom domains, and returns the host the new
//! admin should register against. The institution's tables (users, roles,
//! refresh_tokens, audit_log) are created lazily on first
//! `/api/auth/register` call against the new tenant — the migration file
//! `migrations_institution/001_users_and_rbac.sql` is idempotent so this
//! is safe. A dedicated operator CLI for pre-provisioning a tenant DB
//! lands in a follow-up.

use sqlx::PgPool;

use crate::db;
use crate::models::institution::Institution;
use crate::models::onboarding::{SignupRequest, SignupResponse};
use crate::tenant::RouterState;

#[derive(Debug, thiserror::Error)]
pub enum OnboardingError {
    #[error("slug already taken")]
    SlugTaken,
    #[error("custom domain already taken")]
    DomainTaken,
    #[error("database error: {0}")]
    Db(#[from] sqlx::Error),
}

/// Default host format for new tenants without a custom domain.
fn smartlms_host(slug: &str) -> String {
    format!("{slug}.smartlms.io")
}

fn plan_tier_code(tier: crate::tenant::PlanTier) -> &'static str {
    match tier {
        crate::tenant::PlanTier::Starter => "starter",
        crate::tenant::PlanTier::Growth => "growth",
        crate::tenant::PlanTier::Enterprise => "enterprise",
    }
}

/// Create a new institution. Runs inside a single transaction so a partial
/// failure (e.g. domain_map insert) rolls back the institution row.
pub async fn signup(
    state: &RouterState,
    req: SignupRequest,
) -> Result<SignupResponse, OnboardingError> {
    let pool: &PgPool = &state.master_pool;

    if db::institution::find_by_slug(pool, &req.slug)
        .await?
        .is_some()
    {
        return Err(OnboardingError::SlugTaken);
    }
    if let Some(domain) = req.custom_domain.as_deref() {
        if db::institution::find_by_domain(pool, domain)
            .await?
            .is_some()
        {
            return Err(OnboardingError::DomainTaken);
        }
    }

    let id = uuid::Uuid::new_v4();
    let now = chrono::Utc::now();
    let plan_tier = req.plan_tier.map(plan_tier_code).unwrap_or("starter");

    let mut tx = pool.begin().await?;

    sqlx::query(
        "INSERT INTO institutions (id, slug, name, domain, plan_tier, is_active, created_at, updated_at) \
         VALUES ($1, $2, $3, $4, $5, true, $6, $6)",
    )
    .bind(id)
    .bind(&req.slug)
    .bind(&req.name)
    .bind(&req.custom_domain)
    .bind(plan_tier)
    .bind(now)
    .execute(&mut *tx)
    .await?;

    if let Some(domain) = req.custom_domain.as_deref() {
        sqlx::query(
            "INSERT INTO domain_map (host, slug, institution_id) VALUES ($1, $2, $3) \
             ON CONFLICT (host) DO NOTHING",
        )
        .bind(domain)
        .bind(&req.slug)
        .bind(id)
        .execute(&mut *tx)
        .await?;
    }

    tx.commit().await?;

    let institution = Institution {
        id,
        slug: req.slug.clone(),
        name: req.name.clone(),
        domain: req.custom_domain.clone(),
        database_url: None,
        config: None,
        plan_tier: req.plan_tier,
        quotas: None,
        license_key: None,
        is_active: true,
        created_at: now,
        updated_at: now,
    };

    // Warm the domain_map so custom domains resolve immediately on the next
    // request without a cache-miss Postgres query.
    if let Some(domain) = req.custom_domain.as_deref() {
        state
            .domain_map
            .insert(domain.to_string(), req.slug.clone());
    }

    let admin_host = req
        .custom_domain
        .clone()
        .unwrap_or_else(|| smartlms_host(&req.slug));

    Ok(SignupResponse {
        institution,
        admin_host: admin_host.clone(),
        next_steps: vec![
            format!("POST https://{admin_host}/api/auth/register with the admin credentials"),
            "The first registrant in a new institution is automatically granted the `admin` role"
                .to_string(),
            "Switch to the admin host (Host header) to access /api/users/me and future admin endpoints"
                .to_string(),
        ],
    })
}

/// Public list view of institutions (master-DB level). Scoped to active
/// rows — soft-deleted tenants are excluded.
pub async fn list_active(
    pool: &PgPool,
    page: i64,
    per_page: i64,
) -> Result<(Vec<Institution>, i64), sqlx::Error> {
    db::institution::list(pool, page, per_page).await
}
