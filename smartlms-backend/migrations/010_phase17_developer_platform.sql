-- Phase 17 Enhancements: Developer Marketplace, OAuth 2.0, SDK Generator, API Analytics

-- ============================================================================
-- Developer Marketplace Tables
-- ============================================================================

CREATE TABLE marketplace_listings (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) NOT NULL,
    description TEXT,
    developer_id UUID NOT NULL,
    developer_name VARCHAR(255) NOT NULL,
    category VARCHAR(100) NOT NULL, -- Plugin, Integration, Theme, Template, API Extension
    subcategory VARCHAR(100),
    version VARCHAR(50) NOT NULL DEFAULT '1.0.0',
    price DECIMAL(10,2) DEFAULT 0.00,
    currency VARCHAR(10) DEFAULT 'USD',
    license_type VARCHAR(100) DEFAULT 'MIT', -- MIT, Apache 2.0, GPL, Commercial, Freemium
    rating DECIMAL(3,2) DEFAULT 0.00,
    total_ratings INTEGER DEFAULT 0,
    total_downloads INTEGER DEFAULT 0,
    total_installs INTEGER DEFAULT 0,
    status VARCHAR(50) DEFAULT 'draft', -- draft, pending_review, approved, rejected, suspended
    featured BOOLEAN DEFAULT false,
    tags TEXT[] DEFAULT '{}',
    screenshot_urls TEXT[] DEFAULT '{}',
    documentation_url TEXT,
    demo_url TEXT,
    support_url TEXT,
    repository_url TEXT,
    compatibility JSONB DEFAULT '{}'::jsonb, -- Supported versions, platforms
    requirements JSONB DEFAULT '[]'::jsonb, -- Dependencies, system requirements
    changelog JSONB DEFAULT '[]'::jsonb,
    metadata JSONB DEFAULT '{}'::jsonb,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    published_at TIMESTAMP WITH TIME ZONE,
    reviewed_at TIMESTAMP WITH TIME ZONE,
    reviewed_by UUID
);

CREATE INDEX idx_marketplace_listings_developer ON marketplace_listings(developer_id);
CREATE INDEX idx_marketplace_listings_category ON marketplace_listings(category);
CREATE INDEX idx_marketplace_listings_status ON marketplace_listings(status);
CREATE INDEX idx_marketplace_listings_featured ON marketplace_listings(featured);
CREATE INDEX idx_marketplace_listings_rating ON marketplace_listings(rating DESC);
CREATE INDEX idx_marketplace_listings_tags ON marketplace_listings USING GIN(tags);

CREATE TABLE marketplace_reviews (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    listing_id UUID NOT NULL REFERENCES marketplace_listings(id) ON DELETE CASCADE,
    user_id UUID NOT NULL,
    rating INTEGER NOT NULL CHECK (rating >= 1 AND rating <= 5),
    title VARCHAR(255),
    review_text TEXT,
    pros TEXT,
    cons TEXT,
    is_verified_purchase BOOLEAN DEFAULT false,
    helpful_count INTEGER DEFAULT 0,
    not_helpful_count INTEGER DEFAULT 0,
    developer_response TEXT,
    developer_response_at TIMESTAMP WITH TIME ZONE,
    status VARCHAR(50) DEFAULT 'published', -- published, hidden, flagged
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE INDEX idx_marketplace_reviews_listing ON marketplace_reviews(listing_id);
CREATE INDEX idx_marketplace_reviews_user ON marketplace_reviews(user_id);
CREATE INDEX idx_marketplace_reviews_rating ON marketplace_reviews(rating);

CREATE TABLE marketplace_downloads (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    listing_id UUID NOT NULL REFERENCES marketplace_listings(id) ON DELETE CASCADE,
    user_id UUID NOT NULL,
    institution_id UUID,
    download_version VARCHAR(50) NOT NULL,
    download_type VARCHAR(50) DEFAULT 'install', -- install, source, archive
    ip_address INET,
    user_agent TEXT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE INDEX idx_marketplace_downloads_listing ON marketplace_downloads(listing_id);
CREATE INDEX idx_marketplace_downloads_user ON marketplace_downloads(user_id);
CREATE INDEX idx_marketplace_downloads_created ON marketplace_downloads(created_at);

CREATE TABLE marketplace_transactions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    listing_id UUID NOT NULL REFERENCES marketplace_listings(id) ON DELETE CASCADE,
    buyer_id UUID NOT NULL,
    seller_id UUID NOT NULL,
    amount DECIMAL(10,2) NOT NULL,
    currency VARCHAR(10) NOT NULL DEFAULT 'USD',
    platform_fee DECIMAL(10,2) NOT NULL, -- Platform commission
    seller_payout DECIMAL(10,2) NOT NULL,
    payment_status VARCHAR(50) DEFAULT 'pending', -- pending, completed, refunded, disputed
    payment_method VARCHAR(50),
    transaction_reference VARCHAR(255),
    license_key VARCHAR(255) UNIQUE,
    refund_reason TEXT,
    refunded_at TIMESTAMP WITH TIME ZONE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE INDEX idx_marketplace_transactions_buyer ON marketplace_transactions(buyer_id);
CREATE INDEX idx_marketplace_transactions_seller ON marketplace_transactions(seller_id);
CREATE INDEX idx_marketplace_transactions_listing ON marketplace_transactions(listing_id);
CREATE INDEX idx_marketplace_transactions_status ON marketplace_transactions(payment_status);

-- ============================================================================
-- OAuth 2.0 Authorization Server Tables
-- ============================================================================

CREATE TABLE oauth_applications (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) NOT NULL,
    description TEXT,
    client_id VARCHAR(255) UNIQUE NOT NULL,
    client_secret_hash VARCHAR(255) NOT NULL,
    client_type VARCHAR(50) DEFAULT 'confidential', -- confidential, public
    application_type VARCHAR(50) DEFAULT 'web', -- web, native, spa, mobile
    owner_id UUID NOT NULL,
    redirect_uris TEXT[] NOT NULL DEFAULT '{}',
    post_logout_redirect_uris TEXT[] DEFAULT '{}',
    allowed_origins TEXT[] DEFAULT '{}',
    scopes TEXT[] DEFAULT '{}',
    grant_types TEXT[] DEFAULT '{"authorization_code"}', -- authorization_code, implicit, client_credentials, refresh_token, password
    response_types TEXT[] DEFAULT '{"code"}', -- code, token, id_token
    token_endpoint_auth_method VARCHAR(50) DEFAULT 'client_secret_basic', -- client_secret_basic, client_secret_post, private_key_jwt, none
    jwks_uri TEXT,
    jwks JSONB,
    sector_identifier_uri TEXT,
    subject_type VARCHAR(50) DEFAULT 'public', -- public, pairwise
    id_token_signed_response_alg VARCHAR(50) DEFAULT 'RS256',
    access_token_lifetime INTEGER DEFAULT 3600, -- seconds
    refresh_token_lifetime INTEGER DEFAULT 604800, -- seconds (7 days)
    require_pkce BOOLEAN DEFAULT false,
    require_consent BOOLEAN DEFAULT true,
    allow_implicit_flow BOOLEAN DEFAULT false,
    is_active BOOLEAN DEFAULT true,
    logo_url TEXT,
    policy_url TEXT,
    terms_of_service_url TEXT,
    metadata JSONB DEFAULT '{}'::jsonb,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    last_used_at TIMESTAMP WITH TIME ZONE
);

CREATE INDEX idx_oauth_applications_owner ON oauth_applications(owner_id);
CREATE INDEX idx_oauth_applications_client_id ON oauth_applications(client_id);
CREATE INDEX idx_oauth_applications_active ON oauth_applications(is_active);

CREATE TABLE oauth_authorization_codes (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    application_id UUID NOT NULL REFERENCES oauth_applications(id) ON DELETE CASCADE,
    user_id UUID NOT NULL,
    code VARCHAR(255) UNIQUE NOT NULL,
    scopes TEXT[] NOT NULL DEFAULT '{}',
    redirect_uri TEXT NOT NULL,
    code_challenge VARCHAR(255), -- PKCE code challenge
    code_challenge_method VARCHAR(50), -- S256, plain
    nonce VARCHAR(255),
    state VARCHAR(255),
    expires_at TIMESTAMP WITH TIME ZONE NOT NULL,
    used BOOLEAN DEFAULT false,
    used_at TIMESTAMP WITH TIME ZONE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE INDEX idx_oauth_auth_codes_code ON oauth_authorization_codes(code);
CREATE INDEX idx_oauth_auth_codes_application ON oauth_authorization_codes(application_id);
CREATE INDEX idx_oauth_auth_codes_expires ON oauth_authorization_codes(expires_at);

CREATE TABLE oauth_access_tokens (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    application_id UUID NOT NULL REFERENCES oauth_applications(id) ON DELETE CASCADE,
    user_id UUID, -- NULL for client_credentials grant
    token_hash VARCHAR(255) UNIQUE NOT NULL,
    refresh_token_hash VARCHAR(255) UNIQUE,
    scopes TEXT[] NOT NULL DEFAULT '{}',
    audience TEXT[] DEFAULT '{}',
    issuer VARCHAR(255) DEFAULT 'smartlms',
    subject VARCHAR(255),
    expires_at TIMESTAMP WITH TIME ZONE NOT NULL,
    refresh_expires_at TIMESTAMP WITH TIME ZONE,
    is_revoked BOOLEAN DEFAULT false,
    revoked_at TIMESTAMP WITH TIME ZONE,
    revoke_reason TEXT,
    last_used_at TIMESTAMP WITH TIME ZONE,
    client_ip INET,
    user_agent TEXT,
    metadata JSONB DEFAULT '{}'::jsonb,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE INDEX idx_oauth_access_tokens_token ON oauth_access_tokens(token_hash);
CREATE INDEX idx_oauth_access_tokens_refresh ON oauth_access_tokens(refresh_token_hash);
CREATE INDEX idx_oauth_access_tokens_application ON oauth_access_tokens(application_id);
CREATE INDEX idx_oauth_access_tokens_user ON oauth_access_tokens(user_id);
CREATE INDEX idx_oauth_access_tokens_expires ON oauth_access_tokens(expires_at);

CREATE TABLE oauth_consent_grants (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    application_id UUID NOT NULL REFERENCES oauth_applications(id) ON DELETE CASCADE,
    user_id UUID NOT NULL,
    scopes TEXT[] NOT NULL DEFAULT '{}',
    was_consent_given BOOLEAN NOT NULL,
    consent_method VARCHAR(50), -- explicit, implicit
    consent_screen_shown BOOLEAN DEFAULT false,
    remember_consent BOOLEAN DEFAULT false,
    consent_expires_at TIMESTAMP WITH TIME ZONE,
    client_ip INET,
    user_agent TEXT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE INDEX idx_oauth_consent_user_app ON oauth_consent_grants(user_id, application_id);
CREATE UNIQUE INDEX idx_oauth_consent_unique ON oauth_consent_grants(user_id, application_id);

CREATE TABLE oauth_device_codes (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    application_id UUID NOT NULL REFERENCES oauth_applications(id) ON DELETE CASCADE,
    device_code VARCHAR(255) UNIQUE NOT NULL,
    user_code VARCHAR(20) UNIQUE NOT NULL, -- Short code for user entry
    verification_uri TEXT NOT NULL,
    verification_uri_complete TEXT,
    scopes TEXT[] NOT NULL DEFAULT '{}',
    expires_at TIMESTAMP WITH TIME ZONE NOT NULL,
    interval_seconds INTEGER DEFAULT 5, -- Polling interval
    is_authorized BOOLEAN DEFAULT false,
    authorized_user_id UUID,
    authorized_at TIMESTAMP WITH TIME ZONE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE INDEX idx_oauth_device_codes_device ON oauth_device_codes(device_code);
CREATE INDEX idx_oauth_device_codes_user_code ON oauth_device_codes(user_code);
CREATE INDEX idx_oauth_device_codes_expires ON oauth_device_codes(expires_at);

-- ============================================================================
-- SDK Generator Tables
-- ============================================================================

CREATE TABLE sdk_generations (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    requested_by UUID NOT NULL,
    language VARCHAR(50) NOT NULL, -- typescript, python, java, php, ruby, go, csharp, swift, kotlin
    package_name VARCHAR(255) NOT NULL,
    version VARCHAR(50) NOT NULL,
    api_version VARCHAR(50) NOT NULL,
    include_examples BOOLEAN DEFAULT true,
    include_tests BOOLEAN DEFAULT true,
    include_docs BOOLEAN DEFAULT true,
    custom_templates JSONB DEFAULT '{}'::jsonb,
    generation_status VARCHAR(50) DEFAULT 'pending', -- pending, generating, completed, failed
    output_format VARCHAR(50) DEFAULT 'zip', -- zip, tar.gz, directory
    download_url TEXT,
    download_expires_at TIMESTAMP WITH TIME ZONE,
    file_size_bytes BIGINT,
    checksum_sha256 VARCHAR(255),
    generation_log JSONB DEFAULT '[]'::jsonb,
    error_message TEXT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    completed_at TIMESTAMP WITH TIME ZONE
);

CREATE INDEX idx_sdk_generations_requested_by ON sdk_generations(requested_by);
CREATE INDEX idx_sdk_generations_language ON sdk_generations(language);
CREATE INDEX idx_sdk_generations_status ON sdk_generations(generation_status);

CREATE TABLE sdk_templates (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) NOT NULL,
    language VARCHAR(50) NOT NULL,
    template_type VARCHAR(50) NOT NULL, -- client, server, model, api
    template_content TEXT NOT NULL,
    variables JSONB DEFAULT '[]'::jsonb, -- Available template variables
    is_default BOOLEAN DEFAULT false,
    version VARCHAR(50) NOT NULL,
    created_by UUID NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE INDEX idx_sdk_templates_language ON sdk_templates(language);
CREATE INDEX idx_sdk_templates_type ON sdk_templates(template_type);

CREATE TABLE sdk_packages_registry (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    package_name VARCHAR(255) NOT NULL,
    language VARCHAR(50) NOT NULL,
    version VARCHAR(50) NOT NULL,
    repository_url TEXT,
    npm_package VARCHAR(255),
    pypi_package VARCHAR(255),
    maven_artifact VARCHAR(255),
    nuget_package VARCHAR(255),
    gem_package VARCHAR(255),
    go_module VARCHAR(255),
    is_official BOOLEAN DEFAULT false,
    is_deprecated BOOLEAN DEFAULT false,
    deprecation_message TEXT,
    replacement_package VARCHAR(255),
    metadata JSONB DEFAULT '{}'::jsonb,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE UNIQUE INDEX idx_sdk_packages_unique ON sdk_packages_registry(package_name, language, version);

-- ============================================================================
-- API Analytics Tables
-- ============================================================================

CREATE TABLE api_usage_logs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    request_id UUID NOT NULL DEFAULT gen_random_uuid(),
    timestamp TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    endpoint VARCHAR(500) NOT NULL,
    method VARCHAR(20) NOT NULL,
    status_code INTEGER NOT NULL,
    response_time_ms INTEGER NOT NULL,
    request_size_bytes INTEGER,
    response_size_bytes INTEGER,
    user_id UUID,
    application_id UUID,
    api_key_id UUID,
    tenant_id UUID,
    client_ip INET,
    user_agent TEXT,
    referer TEXT,
    country VARCHAR(100),
    region VARCHAR(100),
    city VARCHAR(100),
    latency_breakdown JSONB, -- { db: x, cache: y, external: z }
    error_message TEXT,
    error_stack TEXT,
    request_headers JSONB,
    response_headers JSONB,
    metadata JSONB DEFAULT '{}'::jsonb
);

CREATE INDEX idx_api_usage_logs_timestamp ON api_usage_logs(timestamp);
CREATE INDEX idx_api_usage_logs_endpoint ON api_usage_logs(endpoint);
CREATE INDEX idx_api_usage_logs_user ON api_usage_logs(user_id);
CREATE INDEX idx_api_usage_logs_application ON api_usage_logs(application_id);
CREATE INDEX idx_api_usage_logs_status ON api_usage_logs(status_code);
CREATE INDEX idx_api_usage_logs_tenant ON api_usage_logs(tenant_id);

-- Partition by month for performance (optional, uncomment for production)
-- ALTER TABLE api_usage_logs PARTITION BY RANGE (timestamp);

CREATE TABLE api_usage_aggregated (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    date DATE NOT NULL,
    hour INTEGER, -- NULL for daily aggregates, 0-23 for hourly
    endpoint VARCHAR(500),
    method VARCHAR(20),
    status_code INTEGER,
    user_id UUID,
    application_id UUID,
    tenant_id UUID,
    total_requests BIGINT NOT NULL DEFAULT 0,
    successful_requests BIGINT NOT NULL DEFAULT 0,
    failed_requests BIGINT NOT NULL DEFAULT 0,
    total_response_time_ms BIGINT NOT NULL DEFAULT 0,
    min_response_time_ms INTEGER,
    max_response_time_ms INTEGER,
    avg_response_time_ms DOUBLE PRECISION,
    p50_response_time_ms DOUBLE PRECISION,
    p95_response_time_ms DOUBLE PRECISION,
    p99_response_time_ms DOUBLE PRECISION,
    total_request_bytes BIGINT DEFAULT 0,
    total_response_bytes BIGINT DEFAULT 0,
    unique_users INTEGER DEFAULT 0,
    unique_ips INTEGER DEFAULT 0,
    error_rate DOUBLE PRECISION,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE UNIQUE INDEX idx_api_usage_agg_unique ON api_usage_aggregated(date, hour, endpoint, method, status_code, COALESCE(user_id, '00000000-0000-0000-0000-000000000000'), COALESCE(application_id, '00000000-0000-0000-0000-000000000000'), COALESCE(tenant_id, '00000000-0000-0000-0000-000000000000'));
CREATE INDEX idx_api_usage_aggregated_date ON api_usage_aggregated(date);
CREATE INDEX idx_api_usage_aggregated_endpoint ON api_usage_aggregated(endpoint);

CREATE TABLE api_rate_limits (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    entity_type VARCHAR(50) NOT NULL, -- user, application, api_key, ip, tenant
    entity_id UUID NOT NULL,
    endpoint_pattern VARCHAR(500), -- NULL for global limit
    limit_type VARCHAR(50) NOT NULL, -- requests_per_second, requests_per_minute, requests_per_hour, requests_per_day, bandwidth
    limit_value INTEGER NOT NULL,
    window_seconds INTEGER NOT NULL,
    current_count INTEGER DEFAULT 0,
    window_start TIMESTAMP WITH TIME ZONE NOT NULL,
    is_blocked BOOLEAN DEFAULT false,
    blocked_until TIMESTAMP WITH TIME ZONE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE UNIQUE INDEX idx_api_rate_limits_unique ON api_rate_limits(entity_type, entity_id, endpoint_pattern, limit_type);
CREATE INDEX idx_api_rate_limits_entity ON api_rate_limits(entity_type, entity_id);

CREATE TABLE api_quotas (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    entity_type VARCHAR(50) NOT NULL, -- user, application, tenant
    entity_id UUID NOT NULL,
    quota_type VARCHAR(50) NOT NULL, -- daily_requests, monthly_requests, bandwidth, storage
    quota_limit BIGINT NOT NULL,
    quota_used BIGINT DEFAULT 0,
    period_start DATE NOT NULL,
    period_end DATE NOT NULL,
    is_unlimited BOOLEAN DEFAULT false,
    alert_threshold_percent INTEGER DEFAULT 80,
    alert_sent BOOLEAN DEFAULT false,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE UNIQUE INDEX idx_api_quotas_unique ON api_quotas(entity_type, entity_id, quota_type, period_start);
CREATE INDEX idx_api_quotas_entity ON api_quotas(entity_type, entity_id);

CREATE TABLE api_alerts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    alert_type VARCHAR(100) NOT NULL, -- rate_limit_exceeded, quota_exceeded, error_spike, latency_spike, anomaly_detected
    severity VARCHAR(50) NOT NULL, -- low, medium, high, critical
    entity_type VARCHAR(50),
    entity_id UUID,
    endpoint VARCHAR(500),
    threshold_value DOUBLE PRECISION,
    actual_value DOUBLE PRECISION,
    message TEXT NOT NULL,
    metadata JSONB DEFAULT '{}'::jsonb,
    is_acknowledged BOOLEAN DEFAULT false,
    acknowledged_by UUID,
    acknowledged_at TIMESTAMP WITH TIME ZONE,
    resolved BOOLEAN DEFAULT false,
    resolved_by UUID,
    resolved_at TIMESTAMP WITH TIME ZONE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE INDEX idx_api_alerts_type ON api_alerts(alert_type);
CREATE INDEX idx_api_alerts_severity ON api_alerts(severity);
CREATE INDEX idx_api_alerts_entity ON api_alerts(entity_type, entity_id);
CREATE INDEX idx_api_alerts_created ON api_alerts(created_at);
CREATE INDEX idx_api_alerts_resolved ON api_alerts(resolved);

CREATE TABLE api_webhooks (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    application_id UUID NOT NULL REFERENCES oauth_applications(id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL,
    url TEXT NOT NULL,
    events TEXT[] NOT NULL DEFAULT '{}', -- Events to subscribe to
    secret VARCHAR(255) NOT NULL, -- For HMAC signature verification
    content_type VARCHAR(50) DEFAULT 'application/json',
    is_active BOOLEAN DEFAULT true,
    retry_policy JSONB DEFAULT '{"max_retries": 3, "retry_delay_seconds": 60}'::jsonb,
    headers JSONB DEFAULT '{}'::jsonb, -- Custom headers
    last_triggered_at TIMESTAMP WITH TIME ZONE,
    last_success_at TIMESTAMP WITH TIME ZONE,
    last_failure_at TIMESTAMP WITH TIME ZONE,
    consecutive_failures INTEGER DEFAULT 0,
    total_deliveries INTEGER DEFAULT 0,
    successful_deliveries INTEGER DEFAULT 0,
    failed_deliveries INTEGER DEFAULT 0,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE INDEX idx_api_webhooks_application ON api_webhooks(application_id);
CREATE INDEX idx_api_webhooks_active ON api_webhooks(is_active);

CREATE TABLE api_webhook_deliveries (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    webhook_id UUID NOT NULL REFERENCES api_webhooks(id) ON DELETE CASCADE,
    event_type VARCHAR(100) NOT NULL,
    payload JSONB NOT NULL,
    http_status INTEGER,
    response_body TEXT,
    attempt_number INTEGER DEFAULT 1,
    next_retry_at TIMESTAMP WITH TIME ZONE,
    delivered BOOLEAN DEFAULT false,
    delivered_at TIMESTAMP WITH TIME ZONE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE INDEX idx_api_webhook_deliveries_webhook ON api_webhook_deliveries(webhook_id);
CREATE INDEX idx_api_webhook_deliveries_delivered ON api_webhook_deliveries(delivered);
CREATE INDEX idx_api_webhook_deliveries_created ON api_webhook_deliveries(created_at);

-- ============================================================================
-- API Keys Management
-- ============================================================================

CREATE TABLE api_keys (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) NOT NULL,
    key_prefix VARCHAR(50) NOT NULL, -- First few chars for identification
    key_hash VARCHAR(255) UNIQUE NOT NULL,
    owner_id UUID NOT NULL,
    application_id UUID REFERENCES oauth_applications(id) ON DELETE SET NULL,
    permissions TEXT[] DEFAULT '{"read"}',
    scopes TEXT[] DEFAULT '{}',
    allowed_ips INET[] DEFAULT '{}', -- Empty means all IPs allowed
    denied_ips INET[] DEFAULT '{}',
    rate_limit_id UUID REFERENCES api_rate_limits(id) ON DELETE SET NULL,
    quota_id UUID REFERENCES api_quotas(id) ON DELETE SET NULL,
    environment VARCHAR(50) DEFAULT 'production', -- development, staging, production
    is_active BOOLEAN DEFAULT true,
    last_used_at TIMESTAMP WITH TIME ZONE,
    expires_at TIMESTAMP WITH TIME ZONE,
    revoked BOOLEAN DEFAULT false,
    revoked_at TIMESTAMP WITH TIME ZONE,
    revoke_reason TEXT,
    metadata JSONB DEFAULT '{}'::jsonb,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE INDEX idx_api_keys_owner ON api_keys(owner_id);
CREATE INDEX idx_api_keys_key_prefix ON api_keys(key_prefix);
CREATE INDEX idx_api_keys_active ON api_keys(is_active);
CREATE INDEX idx_api_keys_environment ON api_keys(environment);

-- ============================================================================
-- Developer Portal Settings
-- ============================================================================

CREATE TABLE developer_portal_settings (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    setting_key VARCHAR(255) UNIQUE NOT NULL,
    setting_value JSONB NOT NULL,
    description TEXT,
    is_public BOOLEAN DEFAULT false, -- Visible in developer portal
    updated_by UUID,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Insert default settings
INSERT INTO developer_portal_settings (setting_key, setting_value, description, is_public) VALUES
('rate_limits.default', '{"requests_per_minute": 100, "requests_per_hour": 1000, "requests_per_day": 10000}', 'Default rate limits for new API keys', true),
('quotas.default', '{"daily_requests": 10000, "monthly_requests": 300000, "bandwidth_gb": 10}', 'Default quotas for new developers', true),
('oauth.token_lifetimes', '{"access_token_hours": 1, "refresh_token_days": 7, "authorization_code_minutes": 10}', 'OAuth token lifetime settings', false),
('marketplace.commission_rate', '0.15', 'Platform commission rate (15%)', false),
('sdk.supported_languages', '["typescript", "python", "java", "php", "ruby", "go", "csharp", "swift"]', 'Supported SDK languages', true),
('api.versioning.strategy', '{"current": "v1", "deprecated": [], "sunset_date": null}', 'API versioning information', true);

-- ============================================================================
-- Documentation & Guides
-- ============================================================================

CREATE TABLE api_documentation (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    title VARCHAR(255) NOT NULL,
    slug VARCHAR(255) UNIQUE NOT NULL,
    category VARCHAR(100) NOT NULL, -- Getting Started, Authentication, Endpoints, SDKs, Webhooks, Best Practices
    subcategory VARCHAR(100),
    content TEXT NOT NULL, -- Markdown content
    excerpt TEXT,
    order_index INTEGER DEFAULT 0,
    is_published BOOLEAN DEFAULT false,
    version_added VARCHAR(50),
    version_deprecated VARCHAR(50),
    related_docs UUID[],
    code_examples JSONB DEFAULT '[]'::jsonb,
    faq JSONB DEFAULT '[]'::jsonb,
    metadata JSONB DEFAULT '{}'::jsonb,
    author_id UUID,
    reviewer_id UUID,
    reviewed_at TIMESTAMP WITH TIME ZONE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE INDEX idx_api_documentation_category ON api_documentation(category);
CREATE INDEX idx_api_documentation_published ON api_documentation(is_published);
CREATE INDEX idx_api_documentation_slug ON api_documentation(slug);

CREATE TABLE api_changelog (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    version VARCHAR(50) NOT NULL,
    release_date DATE NOT NULL DEFAULT CURRENT_DATE,
    change_type VARCHAR(50) NOT NULL, -- added, improved, deprecated, removed, fixed, security
    title VARCHAR(255) NOT NULL,
    description TEXT NOT NULL,
    affected_endpoints TEXT[] DEFAULT '{}',
    breaking_change BOOLEAN DEFAULT false,
    migration_guide TEXT,
    issue_references TEXT[] DEFAULT '{}',
    author_id UUID,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE INDEX idx_api_changelog_version ON api_changelog(version);
CREATE INDEX idx_api_changelog_date ON api_changelog(release_date);
CREATE INDEX idx_api_changelog_type ON api_changelog(change_type);
