// Phase 18: Blockchain Certificate Service
// Provides business logic for minting, verifying, and managing blockchain-based certificates

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use sqlx::PgPool;

/// Blockchain network configuration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BlockchainNetwork {
    Ethereum,
    Polygon,
    PolygonMumbai,
    BinanceSmartChain,
}

impl BlockchainNetwork {
    pub fn chain_id(&self) -> u64 {
        match self {
            BlockchainNetwork::Ethereum => 1,
            BlockchainNetwork::Polygon => 137,
            BlockchainNetwork::PolygonMumbai => 80001,
            BlockchainNetwork::BinanceSmartChain => 56,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            BlockchainNetwork::Ethereum => "ethereum",
            BlockchainNetwork::Polygon => "polygon",
            BlockchainNetwork::PolygonMumbai => "polygon_mumbai",
            BlockchainNetwork::BinanceSmartChain => "bsc",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "ethereum" | "eth" => Some(BlockchainNetwork::Ethereum),
            "polygon" | "matic" => Some(BlockchainNetwork::Polygon),
            "polygon_mumbai" | "mumbai" => Some(BlockchainNetwork::PolygonMumbai),
            "binance_smart_chain" | "bsc" => Some(BlockchainNetwork::BinanceSmartChain),
            _ => None,
        }
    }

    pub fn explorer_url(&self) -> &'static str {
        match self {
            BlockchainNetwork::Ethereum => "https://etherscan.io",
            BlockchainNetwork::Polygon => "https://polygonscan.com",
            BlockchainNetwork::PolygonMumbai => "https://mumbai.polygonscan.com",
            BlockchainNetwork::BinanceSmartChain => "https://bscscan.com",
        }
    }
}

/// Mint status for NFT certificates
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MintStatus {
    Pending,
    Minting,
    Minted,
    Failed,
    Revoked,
}

/// Batch minting priority
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MintPriority {
    Low,
    Normal,
    High,
}

/// Batch job status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BatchStatus {
    Queued,
    Processing,
    Completed,
    PartiallyCompleted,
    Failed,
}

/// User wallet information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserWallet {
    pub id: Uuid,
    pub user_id: Uuid,
    pub wallet_address: String,
    pub network: BlockchainNetwork,
    pub is_primary: bool,
    pub verified: bool,
    pub connected_at: DateTime<Utc>,
    pub last_used_at: Option<DateTime<Utc>>,
}

/// NFT Certificate record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NftCertificate {
    pub id: Uuid,
    pub certificate_id: Uuid,
    pub user_id: Uuid,
    pub course_id: Uuid,
    pub institution_id: Uuid,
    pub token_id: Option<String>,
    pub transaction_hash: Option<String>,
    pub contract_address: Option<String>,
    pub network: BlockchainNetwork,
    pub mint_status: MintStatus,
    pub ipfs_hash: Option<String>,
    pub metadata_uri: Option<String>,
    pub block_number: Option<u64>,
    pub gas_used: Option<u64>,
    pub gas_price_gwei: Option<u64>,
    pub minted_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

/// Batch mint job
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchMintJob {
    pub id: Uuid,
    pub institution_id: Uuid,
    pub network: BlockchainNetwork,
    pub priority: MintPriority,
    pub status: BatchStatus,
    pub total_certificates: i32,
    pub completed_count: i32,
    pub failed_count: i32,
    pub pending_count: i32,
    pub estimated_gas_cost_eth: Option<rust_decimal::Decimal>,
    pub actual_gas_cost_eth: Option<rust_decimal::Decimal>,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

/// Gas price information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GasPriceInfo {
    pub network: String,
    pub slow_price_gwei: u64,
    pub standard_price_gwei: u64,
    pub fast_price_gwei: u64,
    pub instant_price_gwei: u64,
    pub base_fee_gwei: u64,
    pub last_updated: DateTime<Utc>,
}

/// Certificate verification log
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationLog {
    pub id: Uuid,
    pub certificate_id: Option<Uuid>,
    pub nft_certificate_id: Option<Uuid>,
    pub verification_method: String,
    pub identifier_used: String,
    pub verifier_ip: Option<String>,
    pub is_valid: bool,
    pub verification_timestamp: DateTime<Utc>,
}

// ==================== Service Functions ====================

pub mod service {
    use super::*;
    use crate::services::certificate::Certificate;

    /// Connect a wallet to a user account
    pub async fn connect_wallet(
        pool: &PgPool,
        user_id: Uuid,
        wallet_address: &str,
        network: BlockchainNetwork,
        signature: &str,
        message: &str,
    ) -> Result<UserWallet, String> {
        // Verify signature (in production, use ethers-rs or web3-rs)
        let verified = verify_signature(wallet_address, signature, message)
            .await
            .unwrap_or(false);

        if !verified {
            return Err("Invalid signature".to_string());
        }

        let id = Uuid::new_v4();
        let now = Utc::now();

        // Check if this is the user's first wallet
        let is_first = sqlx::query!(
            "SELECT COUNT(*) as count FROM user_wallets WHERE user_id = $1",
            user_id
        )
        .fetch_one(pool)
        .await
        .map_err(|e| e.to_string())?
        .count
        .unwrap_or(0)
            == 0;

        sqlx::query!(
            "INSERT INTO user_wallets (id, user_id, wallet_address, network, is_primary, \
             verified, verification_signature, connected_at)\n\
             VALUES ($1, $2, $3, $4, $5, true, $6, $7)",
            id,
            user_id,
            wallet_address.to_lowercase(),
            network.as_str(),
            is_first,
            signature,
            now
        )
        .execute(pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(UserWallet {
            id,
            user_id,
            wallet_address: wallet_address.to_string(),
            network,
            is_primary: is_first,
            verified: true,
            connected_at: now,
            last_used_at: Some(now),
        })
    }

    /// Verify cryptographic signature
    async fn verify_signature(
        wallet_address: &str,
        signature: &str,
        message: &str,
    ) -> Result<bool, String> {
        // Placeholder - in production, use proper crypto verification
        // with ethers-rs or similar library
        if signature.starts_with("0x") && signature.len() == 132 {
            // Basic format validation
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Get user's connected wallets
    pub async fn get_user_wallets(
        pool: &PgPool,
        user_id: Uuid,
    ) -> Result<Vec<UserWallet>, String> {
        let rows = sqlx::query!(
            "SELECT id, user_id, wallet_address, network, is_primary, \
                    verified, connected_at, last_used_at\n\
             FROM user_wallets WHERE user_id = $1\n\
             ORDER BY is_primary DESC, connected_at DESC",
            user_id
        )
        .fetch_all(pool)
        .await
        .map_err(|e| e.to_string())?;

        let mut wallets = Vec::new();
        for row in rows {
            let network = BlockchainNetwork::from_str(&row.network)
                .unwrap_or(BlockchainNetwork::PolygonMumbai);

            wallets.push(UserWallet {
                id: row.id,
                user_id: row.user_id,
                wallet_address: row.wallet_address,
                network,
                is_primary: row.is_primary,
                verified: row.verified,
                connected_at: row.connected_at,
                last_used_at: row.last_used_at,
            });
        }

        Ok(wallets)
    }

    /// Disconnect a wallet from user account
    pub async fn disconnect_wallet(
        pool: &PgPool,
        user_id: Uuid,
        wallet_address: &str,
    ) -> Result<(), String> {
        sqlx::query!(
            "DELETE FROM user_wallets WHERE user_id = $1 AND wallet_address = $2",
            user_id,
            wallet_address.to_lowercase()
        )
        .execute(pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(())
    }

    /// Create NFT certificate record (prepare for minting)
    pub async fn create_nft_certificate(
        pool: &PgPool,
        certificate: &Certificate,
        institution_id: Uuid,
        network: BlockchainNetwork,
    ) -> Result<NftCertificate, String> {
        let id = Uuid::new_v4();
        let now = Utc::now();

        sqlx::query!(
            "INSERT INTO nft_certificates (id, certificate_id, user_id, course_id, \
             institution_id, network, mint_status, created_at)\n\
             VALUES ($1, $2, $3, $4, $5, $6, 'pending', $7)",
            id,
            certificate.id,
            certificate.recipient_user_id,
            certificate.course_id.unwrap_or_else(Uuid::nil),
            institution_id,
            network.as_str(),
            now
        )
        .execute(pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(NftCertificate {
            id,
            certificate_id: certificate.id,
            user_id: certificate.recipient_user_id,
            course_id: certificate.course_id.unwrap_or_else(Uuid::nil),
            institution_id,
            token_id: None,
            transaction_hash: None,
            contract_address: None,
            network,
            mint_status: MintStatus::Pending,
            ipfs_hash: None,
            metadata_uri: None,
            block_number: None,
            gas_used: None,
            gas_price_gwei: None,
            minted_at: None,
            created_at: now,
        })
    }

    /// Update NFT certificate after minting
    pub async fn update_nft_certificate_minted(
        pool: &PgPool,
        nft_cert_id: Uuid,
        token_id: &str,
        transaction_hash: &str,
        contract_address: &str,
        block_number: u64,
        gas_used: u64,
        gas_price_gwei: u64,
        ipfs_hash: Option<&str>,
    ) -> Result<(), String> {
        let now = Utc::now();

        sqlx::query!(
            "UPDATE nft_certificates SET \
                token_id = $2, transaction_hash = $3, contract_address = $4,\n\
                block_number = $5, gas_used = $6, gas_price_gwei = $7,\n\
                ipfs_hash = $8, mint_status = 'minted', minted_at = $9, updated_at = NOW()\n\
             WHERE id = $1",
            nft_cert_id,
            token_id,
            transaction_hash,
            contract_address,
            block_number as i64,
            gas_used as i64,
            gas_price_gwei as i64,
            ipfs_hash,
            now
        )
        .execute(pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(())
    }

    /// Create batch mint job
    pub async fn create_batch_job(
        pool: &PgPool,
        institution_id: Uuid,
        certificate_ids: &[Uuid],
        network: BlockchainNetwork,
        priority: MintPriority,
    ) -> Result<BatchMintJob, String> {
        let batch_id = Uuid::new_v4();
        let now = Utc::now();
        let total = certificate_ids.len() as i32;

        sqlx::query!(
            "INSERT INTO batch_mint_jobs (id, institution_id, network, priority, \
             status, total_certificates, pending_count, created_at)\n\
             VALUES ($1, $2, $3, $4, 'queued', $5, $5, $6)",
            batch_id,
            institution_id,
            network.as_str(),
            format!("{:?}", priority).to_lowercase().as_str(),
            total,
            now
        )
        .execute(pool)
        .await
        .map_err(|e| e.to_string())?;

        // Insert batch items
        for cert_id in certificate_ids {
            let item_id = Uuid::new_v4();
            sqlx::query!(
                "INSERT INTO batch_mint_items (id, batch_id, certificate_id, status, created_at)\n\
                 VALUES ($1, $2, $3, 'pending', $4)",
                item_id,
                batch_id,
                cert_id,
                now
            )
            .execute(pool)
            .await
            .map_err(|e| e.to_string())?;
        }

        Ok(BatchMintJob {
            id: batch_id,
            institution_id,
            network,
            priority,
            status: BatchStatus::Queued,
            total_certificates: total,
            completed_count: 0,
            failed_count: 0,
            pending_count: total,
            estimated_gas_cost_eth: None,
            actual_gas_cost_eth: None,
            started_at: None,
            completed_at: None,
            created_at: now,
        })
    }

    /// Get batch job status
    pub async fn get_batch_job_status(
        pool: &PgPool,
        batch_id: Uuid,
    ) -> Result<BatchMintJob, String> {
        let row = sqlx::query!(
            "SELECT id, institution_id, network, priority, status, \
                    total_certificates, completed_count, failed_count, pending_count,\n\
                    estimated_gas_cost_eth, actual_gas_cost_eth, started_at, completed_at, created_at\n\
             FROM batch_mint_jobs WHERE id = $1",
            batch_id
        )
        .fetch_one(pool)
        .await
        .map_err(|e| e.to_string())?;

        let network = BlockchainNetwork::from_str(&row.network)
            .unwrap_or(BlockchainNetwork::PolygonMumbai);
        let priority = match row.priority.as_str() {
            "high" => MintPriority::High,
            "low" => MintPriority::Low,
            _ => MintPriority::Normal,
        };
        let status = match row.status.as_str() {
            "processing" => BatchStatus::Processing,
            "completed" => BatchStatus::Completed,
            "partially_completed" => BatchStatus::PartiallyCompleted,
            "failed" => BatchStatus::Failed,
            _ => BatchStatus::Queued,
        };

        Ok(BatchMintJob {
            id: row.id,
            institution_id: row.institution_id,
            network,
            priority,
            status,
            total_certificates: row.total_certificates,
            completed_count: row.completed_count,
            failed_count: row.failed_count,
            pending_count: row.pending_count,
            estimated_gas_cost_eth: row.estimated_gas_cost_eth,
            actual_gas_cost_eth: row.actual_gas_cost_eth,
            started_at: row.started_at,
            completed_at: row.completed_at,
            created_at: row.created_at,
        })
    }

    /// Log certificate verification
    pub async fn log_verification(
        pool: &PgPool,
        certificate_id: Option<Uuid>,
        nft_certificate_id: Option<Uuid>,
        method: &str,
        identifier: &str,
        verifier_ip: Option<&str>,
        is_valid: bool,
    ) -> Result<VerificationLog, String> {
        let id = Uuid::new_v4();
        let now = Utc::now();

        sqlx::query!(
            "INSERT INTO certificate_verifications (id, certificate_id, nft_certificate_id, \
             verification_method, identifier_used, verifier_ip, is_valid, verification_timestamp)\n\
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8)",
            id,
            certificate_id,
            nft_certificate_id,
            method,
            identifier,
            verifier_ip,
            is_valid,
            now
        )
        .execute(pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(VerificationLog {
            id,
            certificate_id,
            nft_certificate_id,
            verification_method: method.to_string(),
            identifier_used: identifier.to_string(),
            verifier_ip: verifier_ip.map(String::from),
            is_valid,
            verification_timestamp: now,
        })
    }

    /// Get current gas prices (cached)
    pub async fn get_gas_prices(
        pool: &PgPool,
        network: BlockchainNetwork,
    ) -> Result<GasPriceInfo, String> {
        let network_str = network.as_str();

        // Try to get cached prices
        let row = sqlx::query!(
            "SELECT network, slow_price_gwei, standard_price_gwei, fast_price_gwei, \
                    instant_price_gwei, base_fee_gwei, last_updated\n\
             FROM gas_prices_cache \n\
             WHERE network = $1 AND expires_at > NOW()\n\
             ORDER BY last_updated DESC LIMIT 1",
            network_str
        )
        .fetch_optional(pool)
        .await
        .map_err(|e| e.to_string())?;

        if let Some(r) = row {
            return Ok(GasPriceInfo {
                network: r.network,
                slow_price_gwei: r.slow_price_gwei.unwrap_or(20) as u64,
                standard_price_gwei: r.standard_price_gwei.unwrap_or(30) as u64,
                fast_price_gwei: r.fast_price_gwei.unwrap_or(45) as u64,
                instant_price_gwei: r.instant_price_gwei.unwrap_or(60) as u64,
                base_fee_gwei: r.base_fee_gwei.unwrap_or(25) as u64,
                last_updated: r.last_updated,
            });
        }

        // Return default prices if cache miss
        Ok(GasPriceInfo {
            network: network_str.to_string(),
            slow_price_gwei: 20,
            standard_price_gwei: 30,
            fast_price_gwei: 45,
            instant_price_gwei: 60,
            base_fee_gwei: 25,
            last_updated: Utc::now(),
        })
    }

    /// Revoke NFT certificate
    pub async fn revoke_nft_certificate(
        pool: &PgPool,
        nft_cert_id: Uuid,
        revoked_by: Uuid,
        reason: &str,
    ) -> Result<(), String> {
        let now = Utc::now();
        let revocation_id = Uuid::new_v4();

        // Start transaction
        let mut tx = pool.begin().await.map_err(|e| e.to_string())?;

        // Update NFT certificate status
        sqlx::query!(
            "UPDATE nft_certificates SET mint_status = 'revoked', revoked_at = $2, updated_at = NOW()\n\
             WHERE id = $1",
            nft_cert_id,
            now
        )
        .execute(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

        // Update base certificate status
        sqlx::query!(
            "UPDATE certificates SET status = 'revoked', updated_at = NOW()\n\
             WHERE id = (SELECT certificate_id FROM nft_certificates WHERE id = $1)",
            nft_cert_id
        )
        .execute(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

        // Record revocation
        sqlx::query!(
            "INSERT INTO certificate_revocations (id, certificate_id, nft_certificate_id, \
             revoked_by, reason, revocation_timestamp)\n\
             SELECT $1, certificate_id, $2, $3, $4, $5\n\
             FROM nft_certificates WHERE id = $2",
            revocation_id,
            nft_cert_id,
            revoked_by,
            reason,
            now
        )
        .execute(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

        tx.commit().await.map_err(|e| e.to_string())?;

        Ok(())
    }
}
