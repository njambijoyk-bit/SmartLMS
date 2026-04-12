-- Phase 18: Blockchain Certificates Database Schema
-- Provides tables for certificate templates, issued certificates, and blockchain NFT records

-- Certificate Templates Table
CREATE TABLE IF NOT EXISTS certificate_templates (
    id UUID PRIMARY KEY,
    institution_id UUID NOT NULL REFERENCES institutions(id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    template_type VARCHAR(50) NOT NULL, -- course_completion, course_completion_with_grade, attendance, participation, custom
    background_url TEXT,
    logo_url TEXT,
    signature_urls TEXT[] DEFAULT '{}',
    content_html TEXT NOT NULL,
    is_default BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_certificate_templates_institution ON certificate_templates(institution_id);
CREATE INDEX idx_certificate_templates_type ON certificate_templates(template_type);

-- Issued Certificates Table
CREATE TABLE IF NOT EXISTS certificates (
    id UUID PRIMARY KEY,
    template_id UUID NOT NULL REFERENCES certificate_templates(id) ON DELETE RESTRICT,
    recipient_user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    course_id UUID REFERENCES courses(id) ON DELETE SET NULL,
    credential_id VARCHAR(64) UNIQUE NOT NULL,
    qr_code_url TEXT NOT NULL,
    recipient_name VARCHAR(255) NOT NULL,
    issue_date TIMESTAMPTZ NOT NULL,
    expiry_date TIMESTAMPTZ,
    metadata JSONB DEFAULT '{}',
    status VARCHAR(20) NOT NULL DEFAULT 'active', -- active, revoked, expired
    pdf_url TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_certificates_recipient ON certificates(recipient_user_id);
CREATE INDEX idx_certificates_course ON certificates(course_id);
CREATE INDEX idx_certificates_credential ON certificates(credential_id);
CREATE INDEX idx_certificates_status ON certificates(status);
CREATE INDEX idx_certificates_template ON certificates(template_id);

-- Blockchain Wallet Associations
CREATE TABLE IF NOT EXISTS user_wallets (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    wallet_address VARCHAR(42) NOT NULL, -- Ethereum address format (0x...)
    network VARCHAR(50) NOT NULL, -- ethereum, polygon, bsc, etc.
    is_primary BOOLEAN DEFAULT FALSE,
    verified BOOLEAN DEFAULT FALSE,
    verification_signature TEXT,
    connected_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    last_used_at TIMESTAMPTZ,
    UNIQUE(user_id, wallet_address)
);

CREATE INDEX idx_user_wallets_user ON user_wallets(user_id);
CREATE INDEX idx_user_wallets_address ON user_wallets(wallet_address);

-- Blockchain Smart Contract Configurations
CREATE TABLE IF NOT EXISTS blockchain_contracts (
    id UUID PRIMARY KEY,
    institution_id UUID REFERENCES institutions(id) ON DELETE CASCADE,
    network VARCHAR(50) NOT NULL, -- ethereum, polygon, polygon_mumbai, bsc
    contract_address VARCHAR(42) NOT NULL,
    contract_abi TEXT NOT NULL,
    contract_name VARCHAR(255) NOT NULL,
    contract_version VARCHAR(50),
    gas_limit BIGINT DEFAULT 300000,
    is_active BOOLEAN DEFAULT TRUE,
    deployed_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(network, contract_address)
);

CREATE INDEX idx_blockchain_contracts_network ON blockchain_contracts(network);
CREATE INDEX idx_blockchain_contracts_institution ON blockchain_contracts(institution_id);

-- NFT Certificate Records (Blockchain Minted Certificates)
CREATE TABLE IF NOT EXISTS nft_certificates (
    id UUID PRIMARY KEY,
    certificate_id UUID NOT NULL REFERENCES certificates(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    course_id UUID NOT NULL REFERENCES courses(id) ON DELETE CASCADE,
    institution_id UUID NOT NULL REFERENCES institutions(id) ON DELETE CASCADE,
    token_id VARCHAR(128), -- NFT token ID from blockchain
    transaction_hash VARCHAR(66), -- Blockchain transaction hash
    contract_address VARCHAR(42), -- Smart contract address
    network VARCHAR(50) NOT NULL,
    mint_status VARCHAR(20) NOT NULL DEFAULT 'pending', -- pending, minting, minted, failed, revoked
    ipfs_hash VARCHAR(128), -- IPFS hash for metadata
    metadata_uri TEXT, -- Full IPFS URI
    block_number BIGINT,
    gas_used BIGINT,
    gas_price_gwei BIGINT,
    minted_at TIMESTAMPTZ,
    revoked_at TIMESTAMPTZ,
    revocation_reason TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_nft_certificates_certificate ON nft_certificates(certificate_id);
CREATE INDEX idx_nft_certificates_user ON nft_certificates(user_id);
CREATE INDEX idx_nft_certificates_token ON nft_certificates(token_id);
CREATE INDEX idx_nft_certificates_transaction ON nft_certificates(transaction_hash);
CREATE INDEX idx_nft_certificates_status ON nft_certificates(mint_status);
CREATE INDEX idx_nft_certificates_network ON nft_certificates(network);

-- Batch Minting Jobs
CREATE TABLE IF NOT EXISTS batch_mint_jobs (
    id UUID PRIMARY KEY,
    institution_id UUID NOT NULL REFERENCES institutions(id) ON DELETE CASCADE,
    network VARCHAR(50) NOT NULL,
    priority VARCHAR(20) NOT NULL DEFAULT 'normal', -- low, normal, high
    status VARCHAR(20) NOT NULL DEFAULT 'queued', -- queued, processing, completed, partially_completed, failed
    total_certificates INTEGER NOT NULL,
    completed_count INTEGER DEFAULT 0,
    failed_count INTEGER DEFAULT 0,
    pending_count INTEGER DEFAULT 0,
    estimated_gas_cost_eth DECIMAL(18, 8),
    actual_gas_cost_eth DECIMAL(18, 8),
    started_at TIMESTAMPTZ,
    completed_at TIMESTAMPTZ,
    error_message TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_batch_mint_jobs_institution ON batch_mint_jobs(institution_id);
CREATE INDEX idx_batch_mint_jobs_status ON batch_mint_jobs(status);

-- Batch Job Items (Individual certificates in a batch)
CREATE TABLE IF NOT EXISTS batch_mint_items (
    id UUID PRIMARY KEY,
    batch_id UUID NOT NULL REFERENCES batch_mint_jobs(id) ON DELETE CASCADE,
    certificate_id UUID NOT NULL REFERENCES certificates(id) ON DELETE CASCADE,
    status VARCHAR(20) NOT NULL DEFAULT 'pending',
    token_id VARCHAR(128),
    transaction_hash VARCHAR(66),
    error_message TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    processed_at TIMESTAMPTZ
);

CREATE INDEX idx_batch_mint_items_batch ON batch_mint_items(batch_id);
CREATE INDEX idx_batch_mint_items_certificate ON batch_mint_items(certificate_id);
CREATE INDEX idx_batch_mint_items_status ON batch_mint_items(status);

-- Certificate Verification Log
CREATE TABLE IF NOT EXISTS certificate_verifications (
    id UUID PRIMARY KEY,
    certificate_id UUID REFERENCES certificates(id) ON DELETE SET NULL,
    nft_certificate_id UUID REFERENCES nft_certificates(id) ON DELETE SET NULL,
    verification_method VARCHAR(50) NOT NULL, -- credential_id, token_id, transaction_hash, qr_code
    identifier_used TEXT NOT NULL, -- The value used for verification
    verifier_ip INET,
    verifier_user_agent TEXT,
    is_valid BOOLEAN NOT NULL,
    verification_timestamp TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_certificate_verifications_certificate ON certificate_verifications(certificate_id);
CREATE INDEX idx_certificate_verifications_method ON certificate_verifications(verification_method);
CREATE INDEX idx_certificate_verifications_timestamp ON certificate_verifications(verification_timestamp);

-- Gas Price Cache
CREATE TABLE IF NOT EXISTS gas_prices_cache (
    id SERIAL PRIMARY KEY,
    network VARCHAR(50) NOT NULL,
    slow_price_gwei BIGINT,
    standard_price_gwei BIGINT,
    fast_price_gwei BIGINT,
    instant_price_gwei BIGINT,
    base_fee_gwei BIGINT,
    last_updated TIMESTAMPTZ NOT NULL,
    expires_at TIMESTAMPTZ NOT NULL
);

CREATE INDEX idx_gas_prices_cache_network ON gas_prices_cache(network);

-- Certificate Revocation Registry
CREATE TABLE IF NOT EXISTS certificate_revocations (
    id UUID PRIMARY KEY,
    certificate_id UUID NOT NULL REFERENCES certificates(id) ON DELETE CASCADE,
    nft_certificate_id UUID REFERENCES nft_certificates(id) ON DELETE SET NULL,
    revoked_by UUID NOT NULL REFERENCES users(id),
    reason TEXT NOT NULL,
    evidence_urls TEXT[],
    revocation_timestamp TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    blockchain_tx_hash VARCHAR(66) -- If revocation was recorded on blockchain
);

CREATE INDEX idx_certificate_revocations_certificate ON certificate_revocations(certificate_id);
CREATE INDEX idx_certificate_revocations_revoked_by ON certificate_revocations(revoked_by);

-- Add trigger to update updated_at timestamps
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';

CREATE TRIGGER update_certificate_templates_updated_at
    BEFORE UPDATE ON certificate_templates
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_certificates_updated_at
    BEFORE UPDATE ON certificates
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_nft_certificates_updated_at
    BEFORE UPDATE ON nft_certificates
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

-- Insert default smart contract configuration for testnet
INSERT INTO blockchain_contracts (id, institution_id, network, contract_address, contract_abi, contract_name, contract_version, gas_limit, is_active)
VALUES (
    gen_random_uuid(),
    NULL, -- Global default
    'polygon_mumbai',
    '0x0000000000000000000000000000000000000000', -- Placeholder - to be updated
    '[]', -- Placeholder ABI
    'SmartLMSCertificate',
    '1.0.0',
    300000,
    FALSE -- Inactive until configured
)
ON CONFLICT (network, contract_address) DO NOTHING;
