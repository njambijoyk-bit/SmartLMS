-- Master schema: tables owned by the central SmartLMS engine (not per-tenant).
-- These live in the master database and are the source of truth for tenant
-- resolution, licensing, and plan tiers.
--
-- Idempotent: safe to run on an existing database. New installs should run
-- this before any other migration.

CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- ---------------------------------------------------------------------------
-- plans
--   Static catalog of plan tiers the engine supports. Seeded below.
-- ---------------------------------------------------------------------------
CREATE TABLE IF NOT EXISTS plans (
    tier              TEXT PRIMARY KEY,          -- 'starter' | 'growth' | 'enterprise'
    display_name      TEXT NOT NULL,
    max_users         BIGINT NOT NULL,
    max_courses       BIGINT NOT NULL,
    max_storage_mb    BIGINT NOT NULL,
    max_concurrent    BIGINT NOT NULL,
    feature_flags     JSONB NOT NULL DEFAULT '[]'::jsonb,
    created_at        TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

INSERT INTO plans (tier, display_name, max_users, max_courses, max_storage_mb, max_concurrent, feature_flags)
VALUES
    ('starter',    'Starter',    1000,    100,   10240,   100,  '["whitelabel","core_lms"]'::jsonb),
    ('growth',     'Growth',     10000,   1000,  102400,  1000, '["whitelabel","core_lms","live_classes","automation","ai_basic"]'::jsonb),
    ('enterprise', 'Enterprise', 1000000, 100000, 1048576, 10000, '["whitelabel","core_lms","live_classes","automation","ai_full","sso_saml","soc2","audit_log","proctoring"]'::jsonb)
ON CONFLICT (tier) DO NOTHING;

-- ---------------------------------------------------------------------------
-- institutions
--   One row per tenant. `database_url` points at that tenant's isolated
--   Postgres instance/database. Cross-tenant data access is impossible because
--   handlers only ever receive a PgPool built from this URL.
-- ---------------------------------------------------------------------------
CREATE TABLE IF NOT EXISTS institutions (
    id              UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    slug            TEXT NOT NULL UNIQUE,
    name            TEXT NOT NULL,
    domain          TEXT UNIQUE,                  -- optional custom domain
    database_url    TEXT,                         -- libpq URL for this tenant's DB
    config          JSONB NOT NULL DEFAULT '{}'::jsonb,
    plan_tier       TEXT NOT NULL DEFAULT 'starter' REFERENCES plans(tier),
    quotas          JSONB NOT NULL DEFAULT '{}'::jsonb,
    license_key     TEXT UNIQUE,
    is_active       BOOLEAN NOT NULL DEFAULT true,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_institutions_slug_active
    ON institutions (slug) WHERE is_active;

CREATE INDEX IF NOT EXISTS idx_institutions_domain_active
    ON institutions (domain) WHERE is_active AND domain IS NOT NULL;

-- ---------------------------------------------------------------------------
-- domain_map
--   Fast host-header → slug lookup for custom domains. Populated whenever an
--   institution sets or changes its `domain`. Kept as a separate table so the
--   multi-tenant router can page it into a DashMap at startup without joining
--   the much heavier institutions row.
-- ---------------------------------------------------------------------------
CREATE TABLE IF NOT EXISTS domain_map (
    host            TEXT PRIMARY KEY,             -- e.g. 'lms.uon.ac.ke'
    slug            TEXT NOT NULL,
    institution_id  UUID NOT NULL REFERENCES institutions(id) ON DELETE CASCADE,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_domain_map_slug ON domain_map (slug);

-- ---------------------------------------------------------------------------
-- licences
--   Append-only log of licence keys issued per institution. Validation and
--   feature-gating at request time reads the most recent active licence.
-- ---------------------------------------------------------------------------
CREATE TABLE IF NOT EXISTS licences (
    id              UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    institution_id  UUID NOT NULL REFERENCES institutions(id) ON DELETE CASCADE,
    key             TEXT NOT NULL UNIQUE,
    plan_tier       TEXT NOT NULL REFERENCES plans(tier),
    issued_at       TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    expires_at      TIMESTAMPTZ,
    revoked_at      TIMESTAMPTZ,
    metadata        JSONB NOT NULL DEFAULT '{}'::jsonb
);

CREATE INDEX IF NOT EXISTS idx_licences_institution
    ON licences (institution_id)
    WHERE revoked_at IS NULL;

-- ---------------------------------------------------------------------------
-- Helper trigger: keep institutions.updated_at honest.
-- ---------------------------------------------------------------------------
CREATE OR REPLACE FUNCTION set_updated_at() RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS trg_institutions_updated ON institutions;
CREATE TRIGGER trg_institutions_updated
    BEFORE UPDATE ON institutions
    FOR EACH ROW
    EXECUTE FUNCTION set_updated_at();
