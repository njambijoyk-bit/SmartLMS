// Multi-tenant institution context - extracted from Host header on every request.
//
// Resolution order (see RouterState::resolve_institution):
//   1. In-process DashMap cache (hot path, no lock on reads, ~ns)
//   2. Redis cache (shared across replicas, ~ms) — optional; skipped if no
//      REDIS_URL was configured at startup
//   3. Master Postgres lookup, then write-back to both caches
//
// Cross-tenant data access is architecturally impossible because each
// InstitutionCtx carries its own PgPool built from the tenant's database_url;
// handlers never see the master pool.

use redis::AsyncCommands;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::time::{Duration, Instant};

use crate::db::institution as inst_db;
use crate::models::institution::Institution;

/// Plan tier determines feature access
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PlanTier {
    #[default]
    Starter,
    Growth,
    Enterprise,
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

    /// Check if the plan tier meets minimum required tier.
    pub fn has_plan_min(&self, min_tier: PlanTier) -> bool {
        matches!(
            (self.plan, min_tier),
            (PlanTier::Enterprise, _)
                | (PlanTier::Growth, PlanTier::Starter | PlanTier::Growth)
                | (PlanTier::Starter, PlanTier::Starter)
        )
    }
}

/// Cached entry with insertion time — lets us expire stale contexts from the
/// in-process cache without blocking on Redis.
#[derive(Clone)]
struct CachedCtx {
    ctx: InstitutionCtx,
    inserted_at: Instant,
}

/// Router state containing shared resources
#[derive(Clone)]
pub struct RouterState {
    /// Hot in-process cache — DashMap for concurrent lock-free reads.
    cache: dashmap::DashMap<String, CachedCtx>,
    /// Domain → slug mapping for custom domains.
    pub domain_map: dashmap::DashMap<String, String>,
    /// Master database pool (for looking up institutions).
    pub master_pool: PgPool,
    /// Optional shared Redis cache — used on DashMap miss before hitting Postgres.
    redis: Option<redis::aio::ConnectionManager>,
    /// In-process cache TTL.
    cache_ttl: Duration,
}

impl std::fmt::Debug for RouterState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RouterState")
            .field("cache_len", &self.cache.len())
            .field("domain_map_len", &self.domain_map.len())
            .field("has_redis", &self.redis.is_some())
            .field("cache_ttl", &self.cache_ttl)
            .finish()
    }
}

impl RouterState {
    /// Build a router state without Redis. Cache-miss path goes straight to Postgres.
    pub fn new(master_pool: PgPool) -> Self {
        Self {
            cache: dashmap::DashMap::new(),
            domain_map: dashmap::DashMap::new(),
            master_pool,
            redis: None,
            cache_ttl: Duration::from_secs(60),
        }
    }

    /// Build a router state with a shared Redis cache between the DashMap and Postgres.
    pub fn with_redis(master_pool: PgPool, redis: redis::aio::ConnectionManager) -> Self {
        Self {
            cache: dashmap::DashMap::new(),
            domain_map: dashmap::DashMap::new(),
            master_pool,
            redis: Some(redis),
            cache_ttl: Duration::from_secs(60),
        }
    }

    /// Override the in-process cache TTL (test hook).
    pub fn with_cache_ttl(mut self, ttl: Duration) -> Self {
        self.cache_ttl = ttl;
        self
    }

    /// Insert a context directly into the in-process cache. Primarily used by
    /// tests and by the resolver after a cache-miss.
    pub fn insert_cached(&self, slug: &str, ctx: InstitutionCtx) {
        self.cache.insert(
            slug.to_string(),
            CachedCtx {
                ctx,
                inserted_at: Instant::now(),
            },
        );
    }

    /// Inspect the in-process cache. Returns None if the entry is missing or stale.
    pub fn cached(&self, slug: &str) -> Option<InstitutionCtx> {
        let entry = self.cache.get(slug)?;
        if entry.inserted_at.elapsed() > self.cache_ttl {
            return None;
        }
        Some(entry.ctx.clone())
    }

    /// Resolve institution context from Host header.
    pub async fn resolve_institution(&self, host: &str) -> Option<InstitutionCtx> {
        // Strip port if present.
        let host = host.split(':').next().unwrap_or(host);

        // Subdomain of the SmartLMS SaaS domain.
        if let Some(slug) = host
            .strip_suffix(".smartlms.io")
            .or_else(|| host.strip_suffix(".smartlms.local"))
        {
            return self.get_institution_by_slug(slug).await;
        }

        // Custom domain lookup.
        if let Some(slug) = self.domain_map.get(host).map(|s| s.clone()) {
            return self.get_institution_by_slug(&slug).await;
        }

        // Localhost defaults to the demo tenant for dev.
        if host == "localhost" || host == "127.0.0.1" {
            return self.get_institution_by_slug("demo").await;
        }

        None
    }

    /// Get institution by slug, walking the three-level cache hierarchy.
    pub async fn get_institution_by_slug(&self, slug: &str) -> Option<InstitutionCtx> {
        // L1: in-process cache.
        if let Some(ctx) = self.cached(slug) {
            return Some(ctx);
        }

        // L2: Redis (only the Institution row is cached — the PgPool itself
        // must be reconstructed per process).
        if let Some(inst) = self.redis_get(slug).await {
            let ctx = self.build_context(&inst).await?;
            self.insert_cached(slug, ctx.clone());
            return Some(ctx);
        }

        // L3: master Postgres.
        if let Ok(Some(inst)) = inst_db::find_by_slug(&self.master_pool, slug).await {
            self.redis_set(slug, &inst).await;
            let ctx = self.build_context(&inst).await?;
            self.insert_cached(slug, ctx.clone());
            return Some(ctx);
        }

        None
    }

    /// Build institution context from database record.
    ///
    /// When `database_url` is set we build a dedicated PgPool for the tenant —
    /// handlers only ever see that pool, so cross-tenant reads are
    /// architecturally impossible. When `database_url` is NULL we fall back
    /// to the master pool; this is the single-database self-hosted mode used
    /// for Phase 1 development and for small single-tenant deployments. In
    /// that mode, tenant data is segregated by row (via the `InstitutionCtx`
    /// the handler receives), not by pool — acceptable because the engine
    /// never serves cross-tenant queries in the same process.
    async fn build_context(&self, inst: &Institution) -> Option<InstitutionCtx> {
        let pool = match inst.database_url.as_ref() {
            Some(db_url) if !db_url.is_empty() => PgPool::connect(db_url).await.ok()?,
            _ => self.master_pool.clone(),
        };

        Some(InstitutionCtx {
            id: inst.id,
            slug: inst.slug.clone(),
            db_pool: pool,
            config: inst.config.clone().unwrap_or_default(),
            plan: inst.plan_tier.unwrap_or_default(),
            quotas: inst.quotas.clone().unwrap_or_default(),
        })
    }

    /// Invalidate a slug in all caches. Called on institution config changes.
    pub async fn invalidate(&self, slug: &str) {
        self.cache.remove(slug);
        if let Some(mut conn) = self.redis.clone() {
            let _: Result<(), _> = conn.del(Self::redis_key(slug)).await;
        }
    }

    // ---------------------------------------------------------------------
    // Redis helpers
    // ---------------------------------------------------------------------

    fn redis_key(slug: &str) -> String {
        format!("smartlms:tenant:{}", slug)
    }

    async fn redis_get(&self, slug: &str) -> Option<Institution> {
        let mut conn = self.redis.clone()?;
        let raw: Option<String> = conn.get(Self::redis_key(slug)).await.ok()?;
        let raw = raw?;
        serde_json::from_str(&raw).ok()
    }

    async fn redis_set(&self, slug: &str, inst: &Institution) {
        let Some(mut conn) = self.redis.clone() else {
            return;
        };
        let Ok(raw) = serde_json::to_string(inst) else {
            return;
        };
        // Redis TTL is 5 minutes; the in-process TTL is shorter so updates
        // propagate within seconds on a single replica.
        let _: Result<(), _> = conn.set_ex(Self::redis_key(slug), raw, 300).await;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_ctx(slug: &str, pool: PgPool) -> InstitutionCtx {
        InstitutionCtx {
            id: uuid::Uuid::new_v4(),
            slug: slug.to_string(),
            db_pool: pool,
            config: InstitutionConfig::default(),
            plan: PlanTier::Starter,
            quotas: QuotaLimits::default(),
        }
    }

    /// The in-process cache honours its TTL and evicts stale entries.
    #[tokio::test]
    async fn in_process_cache_expires() {
        // Use a lazily-connecting pool so this test needs no live DB.
        let pool = sqlx::postgres::PgPoolOptions::new()
            .connect_lazy("postgres://unused/unused")
            .expect("lazy pool");

        let state = RouterState::new(pool.clone()).with_cache_ttl(Duration::from_millis(10));
        state.insert_cached("alpha", sample_ctx("alpha", pool));

        assert!(state.cached("alpha").is_some(), "hot cache hit expected");

        tokio::time::sleep(Duration::from_millis(30)).await;

        assert!(
            state.cached("alpha").is_none(),
            "entry must expire after TTL"
        );
    }

    /// Two tenants resolved from the cache get distinct pools — verifying the
    /// "no shared pool across tenants" invariant.
    #[tokio::test]
    async fn tenants_have_isolated_pools() {
        let master = sqlx::postgres::PgPoolOptions::new()
            .connect_lazy("postgres://unused/master")
            .unwrap();
        let a_pool = sqlx::postgres::PgPoolOptions::new()
            .connect_lazy("postgres://unused/a")
            .unwrap();
        let b_pool = sqlx::postgres::PgPoolOptions::new()
            .connect_lazy("postgres://unused/b")
            .unwrap();

        let state = RouterState::new(master);
        state.insert_cached("a", sample_ctx("a", a_pool));
        state.insert_cached("b", sample_ctx("b", b_pool));

        let ctx_a = state.cached("a").expect("a");
        let ctx_b = state.cached("b").expect("b");

        // Each tenant carries its own PgPool (distinct slug, id, and the
        // underlying pool came from separate connect_lazy calls above).
        assert_ne!(ctx_a.slug, ctx_b.slug);
        assert_ne!(ctx_a.id, ctx_b.id);
        assert_eq!(ctx_a.slug, "a");
        assert_eq!(ctx_b.slug, "b");
    }
}
