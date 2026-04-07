// Multi-tenant institution context - extracted from Host header on every request
use sqlx::PgPool;
use serde::{Deserialize, Serialize};

use crate::models::institution::Institution;
use crate::db::institution as inst_db;

/// Plan tier determines feature access
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
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

/// Institution-specific configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstitutionConfig {
    pub name: String,
    pub logo_url: Option<String>,
    pub primary_color: String,
    pub secondary_color: String,
    pub timezone: String,
    pub locale: String,
    pub feature_flags: Vec<String>,
}

impl Default for InstitutionConfig {
    fn default() -> Self {
        Self {
            name: "New Institution".to_string(),
            logo_url: None,
            primary_color: "#3b82f6".to_string(),
            secondary_color: "#1e40af".to_string(),
            timezone: "UTC".to_string(),
            locale: "en-US".to_string(),
            feature_flags: vec![],
        }
    }
}

/// Quota limits enforced per plan
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuotaLimits {
    pub max_users: i64,
    pub max_courses: i64,
    pub max_storage_mb: i64,
    pub max_concurrent_users: i64,
}

impl Default for QuotaLimits {
    fn default() -> Self {
        Self {
            max_users: 1000,
            max_courses: 100,
            max_storage_mb: 1024,
            max_concurrent_users: 100,
        }
    }
}

/// Full institution context injected into every request
#[derive(Debug, Clone)]
pub struct InstitutionCtx {
    pub id: uuid::Uuid,
    pub slug: String,
    pub db_pool: PgPool,
    pub config: InstitutionConfig,
    pub plan: PlanTier,
    pub quotas: QuotaLimits,
}

impl InstitutionCtx {
    /// Check if a feature is available for this institution's plan
    pub fn has_feature(&self, feature: &str) -> bool {
        self.config.feature_flags.contains(&feature.to_string())
    }
    
    /// Check if the plan tier meets minimum required tier
    pub fn has_plan_min(&self, min_tier: PlanTier) -> bool {
        match (self.plan, min_tier) {
            (PlanTier::Enterprise, _) => true,
            (PlanTier::Growth, PlanTier::Starter) => true,
            (PlanTier::Growth, PlanTier::Growth) => true,
            (PlanTier::Starter, PlanTier::Starter) => true,
            _ => false,
        }
    }
}

/// Router state containing shared resources
#[derive(Debug, Clone)]
pub struct RouterState {
    /// Hot cache for institution contexts - DashMap for concurrent reads
    pub institution_cache: dashmap::DashMap<String, InstitutionCtx>,
    /// Domain to slug mapping for custom domains
    pub domain_map: dashmap::DashMap<String, String>,
    /// Master database pool (for looking up institutions)
    pub master_pool: PgPool,
    /// Cache TTL in seconds
    pub cache_ttl_secs: u64,
}

impl RouterState {
    pub fn new(master_pool: PgPool) -> Self {
        Self {
            institution_cache: dashmap::DashMap::new(),
            domain_map: dashmap::DashMap::new(),
            master_pool,
            cache_ttl_secs: 60,
        }
    }
}

impl RouterState {
    /// Resolve institution context from Host header
    pub async fn resolve_institution(&self, host: &str) -> Option<InstitutionCtx> {
        // Strip port if present
        let host = host.trim_start_matches("localhost:")
            .trim_start_matches("127.0.0.1:")
            .split(':').next()
            .unwrap_or(host);
        
        // Check if it's a subdomain of smartlms.io
        if host.ends_with(".smartlms.io") || host.ends_with(".smartlms.local") {
            let slug = host.trim_end_matches(".smartlms.io")
                .trim_end_matches(".smartlms.local");
            return self.get_institution_by_slug(slug).await;
        }
        
        // Check custom domain map
        if let Some(slug) = self.domain_map.get(host) {
            return self.get_institution_by_slug(&slug).await;
        }
        
        // Default to demo for localhost
        if host == "localhost" || host == "127.0.0.1" {
            return self.get_institution_by_slug("demo").await;
        }
        
        None
    }
    
    /// Get institution by slug, checking cache first
    pub async fn get_institution_by_slug(&self, slug: &str) -> Option<InstitutionCtx> {
        // Check cache first
        if let Some(cached) = self.institution_cache.get(slug) {
            return Some(cached.clone());
        }
        
        // Query master DB
        if let Ok(Some(inst)) = inst_db::find_by_slug(&self.master_pool, slug).await {
            let ctx = self.build_context(&inst).await?;
            self.institution_cache.insert(slug.to_string(), ctx.clone());
            return Some(ctx);
        }
        
        None
    }
    
    /// Build institution context from database record
    async fn build_context(&self, inst: &Institution) -> Option<InstitutionCtx> {
        // Create per-institution DB pool
        let db_url = inst.database_url.as_ref()?;
        let pool = PgPool::connect(db_url).await.ok()?;
        
        Some(InstitutionCtx {
            id: inst.id,
            slug: inst.slug.clone(),
            db_pool: pool,
            config: inst.config.clone().unwrap_or_default(),
            plan: inst.plan_tier.unwrap_or_default(),
            quotas: inst.quotas.clone().unwrap_or_default(),
        })
    }
}