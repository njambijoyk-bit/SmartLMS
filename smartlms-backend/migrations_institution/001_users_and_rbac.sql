-- Per-institution schema — Phase 1.
--
-- This migration runs against each institution's ISOLATED Postgres database
-- (the one referenced by institutions.database_url in the master DB). The
-- master DB (see migrations/001_master_schema.sql) only holds the tenant
-- registry; student/course/grade data lives here.
--
-- Idempotent: safe to re-run.

CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- ---------------------------------------------------------------------------
-- roles — system role catalogue.
-- Per master ref §4, module 5: Admin / Instructor / Learner / Observer /
-- Parent / Advisor / Counsellor / Alumni. `code` is the stable enum value
-- used in JWT claims and RBAC checks.
-- ---------------------------------------------------------------------------
CREATE TABLE IF NOT EXISTS roles (
    code           TEXT PRIMARY KEY,
    display_name   TEXT NOT NULL,
    description    TEXT,
    is_system      BOOLEAN NOT NULL DEFAULT true,
    created_at     TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

INSERT INTO roles (code, display_name, description) VALUES
    ('admin',      'Administrator',     'Full engine access within this institution'),
    ('instructor', 'Instructor',        'Creates and manages courses and assessments'),
    ('learner',    'Learner',           'Enrols in courses and submits work'),
    ('observer',   'Observer',          'Read-only access to specific students'),
    ('parent',     'Parent/Guardian',   'Views linked child progress'),
    ('advisor',    'Academic Advisor',  'Advises learners on progress and pathways'),
    ('counsellor', 'Counsellor',        'Wellbeing and guidance'),
    ('alumni',     'Alumni',            'Graduated learner with limited access')
ON CONFLICT (code) DO NOTHING;

-- ---------------------------------------------------------------------------
-- users — per-institution user accounts.
--
-- Email is unique only among non-deleted users (soft-delete + re-registration
-- under the same address is permitted per master ref §4, module 5).
--
-- password_hash is NULL for SSO-only accounts (master ref §4, module 5).
-- ---------------------------------------------------------------------------
CREATE TABLE IF NOT EXISTS users (
    id                  UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    email               TEXT NOT NULL,
    password_hash       TEXT,
    first_name          TEXT NOT NULL,
    last_name           TEXT NOT NULL,
    display_name        TEXT,
    phone               TEXT,
    avatar_url          TEXT,
    locale              TEXT NOT NULL DEFAULT 'en-US',
    timezone            TEXT NOT NULL DEFAULT 'UTC',
    is_active           BOOLEAN NOT NULL DEFAULT true,
    is_verified         BOOLEAN NOT NULL DEFAULT false,
    last_login_at       TIMESTAMPTZ,
    failed_login_count  INTEGER NOT NULL DEFAULT 0,
    locked_until        TIMESTAMPTZ,
    deleted_at          TIMESTAMPTZ,
    created_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at          TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE UNIQUE INDEX IF NOT EXISTS idx_users_email_active
    ON users (lower(email)) WHERE deleted_at IS NULL;

CREATE INDEX IF NOT EXISTS idx_users_active
    ON users (is_active) WHERE deleted_at IS NULL;

-- ---------------------------------------------------------------------------
-- user_roles — many-to-many between users and roles.
-- ---------------------------------------------------------------------------
CREATE TABLE IF NOT EXISTS user_roles (
    user_id     UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    role_code   TEXT NOT NULL REFERENCES roles(code),
    granted_at  TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    granted_by  UUID REFERENCES users(id),
    PRIMARY KEY (user_id, role_code)
);

CREATE INDEX IF NOT EXISTS idx_user_roles_role ON user_roles (role_code);

-- ---------------------------------------------------------------------------
-- refresh_tokens — long-lived rotation tokens.
--
-- Per master ref §8: short-lived access JWT (15 min) + long-lived refresh
-- token (7 days, HttpOnly cookie). Each refresh issues a new token and
-- revokes the old one (rotation).
--
-- `token_hash` stores a SHA-256 of the random token — the plaintext never
-- hits the DB.
-- ---------------------------------------------------------------------------
CREATE TABLE IF NOT EXISTS refresh_tokens (
    id           UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id      UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    token_hash   TEXT NOT NULL UNIQUE,
    issued_at    TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    expires_at   TIMESTAMPTZ NOT NULL,
    revoked_at   TIMESTAMPTZ,
    replaced_by  UUID REFERENCES refresh_tokens(id),
    user_agent   TEXT,
    ip           INET
);

CREATE INDEX IF NOT EXISTS idx_refresh_tokens_user_active
    ON refresh_tokens (user_id) WHERE revoked_at IS NULL;

-- ---------------------------------------------------------------------------
-- audit_log — append-only.
--
-- Per master ref §8 layer 3: "Append-only table. No UPDATE or DELETE
-- permitted — enforced via database trigger." The trigger below makes
-- mutations outside INSERT raise an exception.
-- ---------------------------------------------------------------------------
CREATE TABLE IF NOT EXISTS audit_log (
    id             UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    actor_user_id  UUID REFERENCES users(id),
    action         TEXT NOT NULL,
    target_type    TEXT,
    target_id      TEXT,
    ip             INET,
    user_agent     TEXT,
    metadata       JSONB NOT NULL DEFAULT '{}'::jsonb,
    occurred_at    TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_audit_log_actor    ON audit_log (actor_user_id);
CREATE INDEX IF NOT EXISTS idx_audit_log_occurred ON audit_log (occurred_at DESC);

CREATE OR REPLACE FUNCTION block_audit_log_mutation() RETURNS TRIGGER AS $$
BEGIN
    RAISE EXCEPTION 'audit_log is append-only (% attempted)', TG_OP;
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS prevent_audit_log_update ON audit_log;
CREATE TRIGGER prevent_audit_log_update
    BEFORE UPDATE OR DELETE ON audit_log
    FOR EACH ROW EXECUTE FUNCTION block_audit_log_mutation();

-- ---------------------------------------------------------------------------
-- Helper trigger: keep users.updated_at honest.
-- ---------------------------------------------------------------------------
CREATE OR REPLACE FUNCTION set_updated_at() RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS set_users_updated_at ON users;
CREATE TRIGGER set_users_updated_at
    BEFORE UPDATE ON users
    FOR EACH ROW EXECUTE FUNCTION set_updated_at();
