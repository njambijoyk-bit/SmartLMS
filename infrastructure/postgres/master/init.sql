-- SmartLMS Master Database Initialization Script
-- PostgreSQL 16+
-- This script creates the master database schema for multi-tenant management

-- Enable required extensions
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE EXTENSION IF NOT EXISTS "pgcrypto";

-- ============================================
-- MASTER DATABASE SCHEMA (smartlms_master)
-- ============================================

-- Institutions table - core multi-tenant data
CREATE TABLE IF NOT EXISTS institutions (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    slug VARCHAR(63) NOT NULL UNIQUE,  -- URL-friendly identifier
    name VARCHAR(255) NOT NULL,
    display_name VARCHAR(255),
    legal_name VARCHAR(255),
    
    -- Contact information
    email VARCHAR(255) NOT NULL,
    phone VARCHAR(50),
    website VARCHAR(255),
    address TEXT,
    
    -- Domain mapping for multi-tenant routing
    custom_domain VARCHAR(255) UNIQUE,
    subdomain VARCHAR(63),
    
    -- White-label branding
    logo_url VARCHAR(500),
    favicon_url VARCHAR(500),
    primary_color VARCHAR(7),  -- Hex color
    secondary_color VARCHAR(7),
    
    -- Plan and billing
    plan_tier VARCHAR(20) NOT NULL DEFAULT 'starter',
    license_key VARCHAR(100) UNIQUE,
    license_expires_at TIMESTAMP,
    max_students INTEGER DEFAULT 1000,
    max_storage_gb INTEGER DEFAULT 10,
    
    -- Feature flags (JSONB for flexibility)
    feature_flags JSONB DEFAULT '{}'::jsonb,
    
    -- Status
    status VARCHAR(20) NOT NULL DEFAULT 'pending',
    is_active BOOLEAN DEFAULT true,
    
    -- Timestamps
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    activated_at TIMESTAMP WITH TIME ZONE,
    
    -- Constraints
    CONSTRAINT valid_slug CHECK (slug ~* '^[a-z0-9][a-z0-9-]*[a-z0-9]$'),
    CONSTRAINT valid_plan CHECK (plan_tier IN ('starter', 'growth', 'enterprise'))
);

-- Create index for fast lookups
CREATE INDEX idx_institutions_slug ON institutions(slug);
CREATE INDEX idx_institutions_domain ON institutions(custom_domain);
CREATE INDEX idx_institutions_status ON institutions(status);

-- License keys table
CREATE TABLE IF NOT EXISTS license_keys (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    key VARCHAR(100) NOT NULL UNIQUE,
    institution_id UUID REFERENCES institutions(id) ON DELETE SET NULL,
    plan_tier VARCHAR(20) NOT NULL,
    max_students INTEGER,
    max_storage_gb INTEGER,
    features JSONB DEFAULT '{}'::jsonb,
    issued_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    expires_at TIMESTAMP WITH TIME ZONE,
    is_active BOOLEAN DEFAULT true
);

CREATE INDEX idx_license_keys_key ON license_keys(key);
CREATE INDEX idx_license_keys_institution ON license_keys(institution_id);

-- Master users table (for SmartLMS platform admin, not institution users)
CREATE TABLE IF NOT EXISTS master_users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    email VARCHAR(255) NOT NULL UNIQUE,
    password_hash VARCHAR(255) NOT NULL,
    full_name VARCHAR(255),
    role VARCHAR(20) NOT NULL DEFAULT 'admin',
    is_super_admin BOOLEAN DEFAULT false,
    is_active BOOLEAN DEFAULT true,
    last_login_at TIMESTAMP WITH TIME ZONE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE INDEX idx_master_users_email ON master_users(email);

-- Audit log for master operations
CREATE TABLE IF NOT EXISTS audit_logs (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    institution_id UUID REFERENCES institutions(id) ON DELETE SET NULL,
    user_id UUID REFERENCES master_users(id) ON DELETE SET NULL,
    action VARCHAR(50) NOT NULL,
    entity_type VARCHAR(50),
    entity_id UUID,
    details JSONB DEFAULT '{}'::jsonb,
    ip_address INET,
    user_agent TEXT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE INDEX idx_audit_logs_institution ON audit_logs(institution_id);
CREATE INDEX idx_audit_logs_created ON audit_logs(created_at DESC);

-- Telemetry receipts (anonymized, opt-in)
CREATE TABLE IF NOT EXISTS telemetry_receipts (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    institution_id UUID REFERENCES institutions(id) ON DELETE CASCADE,
    period_start DATE NOT NULL,
    period_end DATE NOT NULL,
    
    -- Usage counts (aggregated, no PII)
    active_learners INTEGER DEFAULT 0,
    total_logins INTEGER DEFAULT 0,
    courses_created INTEGER DEFAULT 0,
    assignments_submitted INTEGER DEFAULT 0,
    live_sessions_held INTEGER DEFAULT 0,
    storage_used_mb BIGINT DEFAULT 0,
    
    -- Behavior patterns (anonymized)
    behavior_data JSONB DEFAULT '{}'::jsonb,
    
    received_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE INDEX idx_telemetry_institution ON telemetry_receipts(institution_id);
CREATE INDEX idx_telemetry_period ON telemetry_receipts(period_start, period_end);

-- Update trigger function
CREATE OR REPLACE FUNCTION update_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Apply triggers
CREATE TRIGGER update_institutions_updated_at
    BEFORE UPDATE ON institutions
    FOR EACH ROW EXECUTE FUNCTION update_updated_at();

CREATE TRIGGER update_master_users_updated_at
    BEFORE UPDATE ON master_users
    FOR EACH ROW EXECUTE FUNCTION update_updated_at();

-- Insert default super admin (password: change_me_in_production - MUST CHANGE)
INSERT INTO master_users (email, password_hash, full_name, role, is_super_admin)
VALUES ('admin@smartlms.io', crypt('change_me_in_production', gen_salt('bf')), 'System Admin', 'super_admin', true)
ON CONFLICT (email) DO NOTHING;

-- Insert sample institution for development
INSERT INTO institutions (slug, name, email, plan_tier, status, is_active)
VALUES ('demo', 'Demo Institution', 'admin@demo.smartlms.io', 'enterprise', 'active', true)
ON CONFLICT (slug) DO NOTHING;

SELECT 'SmartLMS Master Database initialized successfully' AS status;