// Phase 18: Blockchain Certificates - NFT-based certificates
// Provides endpoints for minting, verifying, and managing blockchain credentials

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

use crate::services::blockchain::{self, BlockchainNetwork, MintPriority};
use crate::services::certificate;

// ==================== Blockchain Configuration ====================

/// Supported blockchain networks
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BlockchainNetwork {
    Ethereum,
    Polygon,
    PolygonMumbai, // Testnet
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
    
    pub fn explorer_url(&self) -> &'static str {
        match self {
            BlockchainNetwork::Ethereum => "https://etherscan.io",
            BlockchainNetwork::Polygon => "https://polygonscan.com",
            BlockchainNetwork::PolygonMumbai => "https://mumbai.polygonscan.com",
            BlockchainNetwork::BinanceSmartChain => "https://bscscan.com",
        }
    }
}

/// Smart contract configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SmartContractConfig {
    pub network: BlockchainNetwork,
    pub contract_address: String,
    pub abi: String,
    pub gas_limit: u64,
}

// ==================== NFT Certificate Models ====================

/// NFT Certificate metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NftCertificate {
    pub id: Uuid,
    pub certificate_id: Uuid, // Reference to LMS certificate
    pub user_id: Uuid,
    pub course_id: Uuid,
    pub institution_id: Uuid,
    pub token_id: Option<String>, // NFT token ID from blockchain
    pub transaction_hash: Option<String>,
    pub contract_address: Option<String>,
    pub network: BlockchainNetwork,
    pub mint_status: MintStatus,
    pub ipfs_hash: Option<String>, // Metadata stored on IPFS
    pub minted_at: Option<chrono::DateTime<chrono::Utc>>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MintStatus {
    Pending,
    Minting,
    Minted,
    Failed,
    Revoked,
}

/// Certificate metadata for IPFS
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CertificateMetadata {
    pub name: String,
    pub description: String,
    pub image: String, // IPFS hash of certificate image
    pub attributes: Vec<Attribute>,
    pub external_url: String,
    pub issuer: IssuerInfo,
    pub recipient: RecipientInfo,
    pub issuance_date: String,
    pub expiration_date: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attribute {
    pub trait_type: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IssuerInfo {
    pub name: String,
    pub address: String, // Blockchain address
    pub verification_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecipientInfo {
    pub name: String,
    pub wallet_address: Option<String>,
    pub student_id: String,
}

// ==================== Verification Models ====================

/// Public verification request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationRequest {
    pub certificate_hash: Option<String>,
    pub token_id: Option<String>,
    pub transaction_hash: Option<String>,
}

/// Verification result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationResult {
    pub is_valid: bool,
    pub certificate_info: Option<CertificatePublicInfo>,
    pub blockchain_proof: Option<BlockchainProof>,
    pub verification_timestamp: chrono::DateTime<chrono::Utc>,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CertificatePublicInfo {
    pub certificate_name: String,
    pub recipient_name: String,
    pub institution_name: String,
    pub issue_date: chrono::DateTime<chrono::Utc>,
    pub credential_type: String,
    pub grade: Option<String>,
    pub honors: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockchainProof {
    pub network: String,
    pub contract_address: String,
    pub token_id: String,
    pub transaction_hash: String,
    pub block_number: Option<u64>,
    pub explorer_url: String,
}

// ==================== Wallet Integration ====================

/// Connect wallet request
#[derive(Debug, Deserialize)]
pub struct ConnectWalletRequest {
    pub user_id: Uuid,
    pub wallet_address: String,
    pub signature: String, // Signed message for verification
    pub message: String, // Original message that was signed
}

#[derive(Debug, Serialize)]
pub struct ConnectWalletResponse {
    pub success: bool,
    pub wallet_address: String,
    pub verified: bool,
    pub error: Option<String>,
}

/// Withdraw certificate to wallet
#[derive(Debug, Deserialize)]
pub struct WithdrawCertificateRequest {
    pub user_id: Uuid,
    pub certificate_id: Uuid,
    pub wallet_address: String,
}

#[derive(Debug, Serialize)]
pub struct WithdrawCertificateResponse {
    pub success: bool,
    pub transaction_hash: Option<String>,
    pub estimated_gas: String,
    pub error: Option<String>,
}

// ==================== QR Code Verification ====================

/// Generate QR code for certificate
#[derive(Debug, Serialize)]
pub struct QrCodeData {
    pub qr_code_svg: String,
    pub verification_url: String,
    pub short_code: String,
}

/// Verify via QR code short code
#[derive(Debug, Deserialize)]
pub struct QrVerifyRequest {
    pub code: String,
}

// ==================== Batch Operations ====================

/// Batch mint certificates
#[derive(Debug, Deserialize)]
pub struct BatchMintRequest {
    pub institution_id: Uuid,
    pub certificate_ids: Vec<Uuid>,
    pub network: BlockchainNetwork,
    pub priority: MintPriority,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MintPriority {
    Low,
    Normal,
    High,
}

#[derive(Debug, Serialize)]
pub struct BatchMintResponse {
    pub batch_id: Uuid,
    pub total_certificates: usize,
    pub estimated_completion_time: chrono::DateTime<chrono::Utc>,
    pub estimated_total_gas: String,
    pub status: BatchStatus,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BatchStatus {
    Queued,
    Processing,
    Completed,
    PartiallyCompleted,
    Failed,
}

/// Batch operation status
#[derive(Debug, Serialize)]
pub struct BatchStatusResponse {
    pub batch_id: Uuid,
    pub status: BatchStatus,
    pub completed_count: usize,
    pub failed_count: usize,
    pub pending_count: usize,
    pub results: Vec<BatchResult>,
}

#[derive(Debug, Serialize)]
pub struct BatchResult {
    pub certificate_id: Uuid,
    pub success: bool,
    pub token_id: Option<String>,
    pub transaction_hash: Option<String>,
    pub error: Option<String>,
}

// ==================== Gas & Fee Management ====================

/// Gas estimation
#[derive(Debug, Serialize)]
pub struct GasEstimate {
    pub gas_limit: u64,
    pub gas_price_gwei: u64,
    pub estimated_cost_eth: String,
    pub estimated_cost_usd: String,
    pub priority_fee_gwei: Option<u64>,
}

/// Current gas prices
#[derive(Debug, Serialize)]
pub struct GasPrices {
    pub slow: GasEstimate,
    pub standard: GasEstimate,
    pub fast: GasEstimate,
    pub instant: GasEstimate,
    pub last_updated: chrono::DateTime<chrono::Utc>,
}

// ==================== API Routes ====================

pub fn blockchain_router() -> Router {
    Router::new()
        // Certificate minting
        .route("/certificates/:certificate_id/mint", axum::routing::post(handle_mint_certificate))
        .route("/certificates/batch-mint", axum::routing::post(handle_batch_mint))
        .route("/certificates/batch/:batch_id/status", axum::routing::get(handle_batch_status))
        // Verification
        .route("/verify", axum::routing::post(handle_verify_certificate))
        .route("/verify/qr/:code", axum::routing::get(handle_qr_verify))
        .route("/certificates/:certificate_id/proof", axum::routing::get(handle_get_proof))
        // Wallet integration
        .route("/wallet/connect", axum::routing::post(handle_connect_wallet))
        .route("/wallet/:user_id/disconnect", axum::routing::post(handle_disconnect_wallet))
        .route("/wallet/withdraw", axum::routing::post(handle_withdraw_certificate))
        // QR codes
        .route("/certificates/:certificate_id/qr", axum::routing::get(handle_generate_qr))
        // Gas & fees
        .route("/gas/prices", axum::routing::get(handle_get_gas_prices))
        .route("/gas/estimate", axum::routing::post(handle_estimate_gas))
        // Public portal
        .route("/public/:identifier", axum::routing::get(handle_public_verification))
}

// ==================== Handler Implementations ====================

async fn handle_mint_certificate(
    State(pool): State<PgPool>,
    Path(certificate_id): Path<Uuid>,
    Json(req): Json<MintCertificateRequest>,
) -> Result<Json<MintCertificateResponse>, StatusCode> {
    // 1. Verify certificate exists and belongs to user
    let cert = sqlx::query!(
        "SELECT id, template_id, recipient_user_id, course_id, credential_id, \
                qr_code_url, recipient_name, issue_date, expiry_date, status, pdf_url\n\
         FROM certificates WHERE id = $1",
        certificate_id
    )
    .fetch_optional(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let certificate_row = match cert {
        Some(c) => c,
        None => return Ok(Json(MintCertificateResponse {
            success: false,
            certificate_id,
            token_id: None,
            transaction_hash: None,
            status: "failed".to_string(),
            error: Some("Certificate not found".to_string()),
        })),
    };

    // Check if already minted
    let existing_nft = sqlx::query!(
        "SELECT token_id, transaction_hash FROM nft_certificates WHERE certificate_id = $1",
        certificate_id
    )
    .fetch_optional(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if let Some(nft) = existing_nft {
        if nft.token_id.is_some() {
            return Ok(Json(MintCertificateResponse {
                success: true,
                certificate_id,
                token_id: nft.token_id,
                transaction_hash: nft.transaction_hash,
                status: "already_minted".to_string(),
                error: None,
            }));
        }
    }

    // 2. Create NFT certificate record
    let network = req.network.unwrap_or(BlockchainNetwork::PolygonMumbai);
    
    let certificate = certificate::Certificate {
        id: certificate_row.id,
        template_id: certificate_row.template_id,
        recipient_user_id: certificate_row.recipient_user_id,
        course_id: certificate_row.course_id,
        credential_id: certificate_row.credential_id,
        qr_code_url: certificate_row.qr_code_url,
        recipient_name: certificate_row.recipient_name,
        issue_date: certificate_row.issue_date,
        expiry_date: certificate_row.expiry_date,
        metadata: std::collections::HashMap::new(),
        status: certificate::CertificateStatus::Active,
        pdf_url: certificate_row.pdf_url,
    };

    // Get institution_id from certificate template
    let institution_id = sqlx::query!(
        "SELECT institution_id FROM certificate_templates WHERE id = $1",
        certificate_row.template_id
    )
    .fetch_one(&pool)
    .await
    .map_err(|_| StatusCode::NOT_FOUND)?
    .institution_id;

    let nft_cert = blockchain::service::create_nft_certificate(
        &pool,
        &certificate,
        institution_id,
        network,
    )
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // 3. In production: Upload metadata to IPFS and call smart contract
    // For now, simulate minting with placeholder values
    
    // Simulate successful minting
    let token_id = format!("{}-{}", nft_cert.id.to_string()[..8].to_uppercase(), Uuid::new_v4().to_string()[..4].to_uppercase());
    let tx_hash = format!("0x{}", Uuid::new_v4().to_string().replace("-", ""));
    let contract_address = "0x1234567890123456789012345678901234567890".to_string();
    let block_number: u64 = 12345678;
    let gas_used: u64 = 150000;
    let gas_price_gwei: u64 = 30;

    blockchain::service::update_nft_certificate_minted(
        &pool,
        nft_cert.id,
        &token_id,
        &tx_hash,
        &contract_address,
        block_number,
        gas_used,
        gas_price_gwei,
        None,
    )
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(MintCertificateResponse {
        success: true,
        certificate_id,
        token_id: Some(token_id),
        transaction_hash: Some(tx_hash),
        status: "minted".to_string(),
        error: None,
    }))
}

#[derive(Debug, Deserialize)]
pub struct MintCertificateRequest {
    pub user_id: Uuid,
    pub network: Option<BlockchainNetwork>,
    pub wallet_address: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct MintCertificateResponse {
    pub success: bool,
    pub certificate_id: Uuid,
    pub token_id: Option<String>,
    pub transaction_hash: Option<String>,
    pub status: String,
    pub error: Option<String>,
}

async fn handle_batch_mint(
    State(pool): State<PgPool>,
    Json(req): Json<BatchMintRequest>,
) -> Result<Json<BatchMintResponse>, StatusCode> {
    let batch_id = Uuid::new_v4();
    let estimated_time = chrono::Utc::now() + chrono::Duration::minutes(
        (req.certificate_ids.len() as i64) * 2 // Estimate 2 minutes per certificate
    );
    
    // TODO: Queue batch job for processing
    
    Ok(Json(BatchMintResponse {
        batch_id,
        total_certificates: req.certificate_ids.len(),
        estimated_completion_time: estimated_time,
        estimated_total_gas: "0.05 ETH".to_string(),
        status: BatchStatus::Queued,
    }))
}

async fn handle_batch_status(
    State(pool): State<PgPool>,
    Path(batch_id): Path<Uuid>,
) -> Result<Json<BatchStatusResponse>, StatusCode> {
    // TODO: Fetch batch status from database/job queue
    
    Ok(Json(BatchStatusResponse {
        batch_id,
        status: BatchStatus::Queued,
        completed_count: 0,
        failed_count: 0,
        pending_count: 0,
        results: Vec::new(),
    }))
}

async fn handle_verify_certificate(
    State(pool): State<PgPool>,
    Json(req): Json<VerificationRequest>,
) -> Result<Json<VerificationResult>, StatusCode> {
    let now = chrono::Utc::now();

    // Try to find certificate by different identifiers
    let cert_info = if let Some(tx_hash) = &req.transaction_hash {
        // Look up by transaction hash
        sqlx::query!(
            "SELECT c.id, c.recipient_name, c.issue_date, ct.name as certificate_name,\n\
                    nc.token_id, nc.contract_address, nc.network, nc.block_number\n\
             FROM certificates c\n             JOIN certificate_templates ct ON c.template_id = ct.id\n             JOIN nft_certificates nc ON c.id = nc.certificate_id\n             WHERE nc.transaction_hash = $1",
            tx_hash
        )
        .fetch_optional(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    } else if let Some(token_id) = &req.token_id {
        // Look up by token ID
        sqlx::query!(
            "SELECT c.id, c.recipient_name, c.issue_date, ct.name as certificate_name,\n\
                    nc.token_id, nc.contract_address, nc.network, nc.block_number\n\
             FROM certificates c\n             JOIN certificate_templates ct ON c.template_id = ct.id\n             JOIN nft_certificates nc ON c.id = nc.certificate_id\n             WHERE nc.token_id = $1",
            token_id
        )
        .fetch_optional(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    } else {
        None
    };

    match cert_info {
        Some(row) => {
            // Check revocation status
            let is_revoked = sqlx::query!(
                "SELECT COUNT(*) as count FROM certificate_revocations WHERE certificate_id = $1",
                row.id
            )
            .fetch_one(&pool)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
            .count
            .unwrap_or(0) > 0;

            let network = blockchain::BlockchainNetwork::from_str(&row.network)
                .unwrap_or(blockchain::BlockchainNetwork::PolygonMumbai);
            let explorer_url = format!("{}/tx/{}", network.explorer_url(), row.token_id.as_ref().unwrap());

            // Log verification
            let _ = blockchain::service::log_verification(
                &pool,
                Some(row.id),
                None,
                "transaction_hash",
                req.transaction_hash.as_deref().unwrap_or(""),
                None,
                !is_revoked,
            ).await;

            Ok(Json(VerificationResult {
                is_valid: !is_revoked,
                certificate_info: Some(CertificatePublicInfo {
                    certificate_name: row.certificate_name,
                    recipient_name: row.recipient_name,
                    institution_name: "Institution".to_string(), // Could be joined from institutions table
                    issue_date: row.issue_date,
                    credential_type: "NFT Certificate".to_string(),
                    grade: None,
                    honors: None,
                }),
                blockchain_proof: Some(BlockchainProof {
                    network: row.network,
                    contract_address: row.contract_address.unwrap_or_default(),
                    token_id: row.token_id.unwrap_or_default(),
                    transaction_hash: req.transaction_hash.clone().unwrap_or_default(),
                    block_number: row.block_number.map(|b| b as u64),
                    explorer_url,
                }),
                verification_timestamp: now,
                error: if is_revoked { Some("Certificate has been revoked".to_string()) } else { None },
            }))
        }
        None => Ok(Json(VerificationResult {
            is_valid: false,
            certificate_info: None,
            blockchain_proof: None,
            verification_timestamp: now,
            error: Some("Certificate not found on blockchain".to_string()),
        })),
    }
}

async fn handle_qr_verify(
    State(pool): State<PgPool>,
    Path(code): Path<String>,
) -> Result<Json<VerificationResult>, StatusCode> {
    // Decode short code to certificate identifier
    // Perform verification
    
    Ok(Json(VerificationResult {
        is_valid: false,
        certificate_info: None,
        blockchain_proof: None,
        verification_timestamp: chrono::Utc::now(),
        error: Some("QR verification not implemented".to_string()),
    }))
}

async fn handle_get_proof(
    State(pool): State<PgPool>,
    Path(certificate_id): Path<Uuid>,
) -> Result<Json<BlockchainProof>, StatusCode> {
    // TODO: Fetch blockchain proof for certificate
    
    Err(StatusCode::NOT_FOUND)
}

async fn handle_connect_wallet(
    State(pool): State<PgPool>,
    Json(req): Json<ConnectWalletRequest>,
) -> Result<Json<ConnectWalletResponse>, StatusCode> {
    // Verify signature and connect wallet using blockchain service
    let network = BlockchainNetwork::PolygonMumbai; // Default to testnet
    
    match blockchain::service::connect_wallet(
        &pool,
        req.user_id,
        &req.wallet_address,
        network,
        &req.signature,
        &req.message,
    ).await {
        Ok(wallet) => Ok(Json(ConnectWalletResponse {
            success: true,
            wallet_address: wallet.wallet_address,
            verified: wallet.verified,
            error: None,
        })),
        Err(e) => Ok(Json(ConnectWalletResponse {
            success: false,
            wallet_address: req.wallet_address,
            verified: false,
            error: Some(e),
        })),
    }
}

async fn handle_disconnect_wallet(
    State(pool): State<PgPool>,
    Path((user_id, wallet_address)): Path<(Uuid, String)>,
) -> Result<Json<DisconnectWalletResponse>, StatusCode> {
    blockchain::service::disconnect_wallet(&pool, user_id, &wallet_address)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(Json(DisconnectWalletResponse {
        success: true,
        message: "Wallet disconnected successfully".to_string(),
    }))
}

#[derive(Debug, Serialize)]
pub struct DisconnectWalletResponse {
    pub success: bool,
    pub message: String,
}

async fn handle_withdraw_certificate(
    State(pool): State<PgPool>,
    Json(req): Json<WithdrawCertificateRequest>,
) -> Result<Json<WithdrawCertificateResponse>, StatusCode> {
    // TODO: Transfer NFT to user's wallet
    
    Ok(Json(WithdrawCertificateResponse {
        success: false,
        transaction_hash: None,
        estimated_gas: "0.002 ETH".to_string(),
        error: Some("Withdrawal not implemented".to_string()),
    }))
}

async fn handle_generate_qr(
    State(pool): State<PgPool>,
    Path(certificate_id): Path<Uuid>,
) -> Result<Json<QrCodeData>, StatusCode> {
    // Generate short verification code
    let short_code = format!("CERT-{}", uuid::Uuid::new_v4().to_string()[..8].to_uppercase());
    let verification_url = format!("https://verify.smartlms.com/qr/{}", short_code);
    
    // Generate SVG QR code (in production, use qrcode crate)
    let qr_svg = format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" width="200" height="200">
            <rect width="200" height="200" fill="white"/>
            <text x="100" y="100" text-anchor="middle" font-size="12">{}</text>
        </svg>"#,
        short_code
    );
    
    Ok(Json(QrCodeData {
        qr_code_svg: qr_svg,
        verification_url,
        short_code,
    }))
}

async fn handle_get_gas_prices(
    State(pool): State<PgPool>,
) -> Result<Json<GasPrices>, StatusCode> {
    // TODO: Fetch current gas prices from blockchain node or API
    
    let now = chrono::Utc::now();
    
    Ok(Json(GasPrices {
        slow: GasEstimate {
            gas_limit: 100000,
            gas_price_gwei: 20,
            estimated_cost_eth: "0.002".to_string(),
            estimated_cost_usd: "3.50".to_string(),
            priority_fee_gwei: Some(1),
        },
        standard: GasEstimate {
            gas_limit: 100000,
            gas_price_gwei: 30,
            estimated_cost_eth: "0.003".to_string(),
            estimated_cost_usd: "5.25".to_string(),
            priority_fee_gwei: Some(2),
        },
        fast: GasEstimate {
            gas_limit: 100000,
            gas_price_gwei: 45,
            estimated_cost_eth: "0.0045".to_string(),
            estimated_cost_usd: "7.88".to_string(),
            priority_fee_gwei: Some(3),
        },
        instant: GasEstimate {
            gas_limit: 100000,
            gas_price_gwei: 60,
            estimated_cost_eth: "0.006".to_string(),
            estimated_cost_usd: "10.50".to_string(),
            priority_fee_gwei: Some(5),
        },
        last_updated: now,
    }))
}

async fn handle_estimate_gas(
    State(pool): State<PgPool>,
    Json(req): Json<GasEstimateRequest>,
) -> Result<Json<GasEstimate>, StatusCode> {
    // TODO: Calculate gas estimate based on operation type
    
    Ok(Json(GasEstimate {
        gas_limit: 150000,
        gas_price_gwei: 30,
        estimated_cost_eth: "0.0045".to_string(),
        estimated_cost_usd: "7.88".to_string(),
        priority_fee_gwei: Some(2),
    }))
}

#[derive(Debug, Deserialize)]
pub struct GasEstimateRequest {
    pub operation: String, // mint, transfer, verify
    pub network: Option<BlockchainNetwork>,
    pub quantity: Option<usize>,
}

async fn handle_public_verification(
    State(pool): State<PgPool>,
    Path(identifier): Path<String>,
) -> Result<Json<PublicVerificationPage>, StatusCode> {
    // Public verification page data
    // Can be accessed without authentication
    
    // TODO: Look up certificate by identifier (could be hash, token_id, or short code)
    
    Ok(Json(PublicVerificationPage {
        is_valid: false,
        certificate_name: "Unknown".to_string(),
        recipient_name: "Unknown".to_string(),
        institution_name: "Unknown".to_string(),
        issue_date: None,
        blockchain_info: None,
        error: Some("Certificate not found".to_string()),
    }))
}

#[derive(Debug, Serialize)]
pub struct PublicVerificationPage {
    pub is_valid: bool,
    pub certificate_name: String,
    pub recipient_name: String,
    pub institution_name: String,
    pub issue_date: Option<chrono::DateTime<chrono::Utc>>,
    pub blockchain_info: Option<BlockchainProof>,
    pub error: Option<String>,
}

// ==================== Helper Functions ====================

/// Upload certificate metadata to IPFS
pub async fn upload_to_ipfs(metadata: &CertificateMetadata) -> Result<String, String> {
    // TODO: Integrate with IPFS service (Pinata, Infura, etc.)
    // Return IPFS hash (CID)
    
    Err("IPFS upload not configured".to_string())
}

/// Mint NFT on blockchain
pub async fn mint_nft(
    contract_config: &SmartContractConfig,
    recipient_address: &str,
    token_uri: &str, // IPFS URI
) -> Result<MintResult, String> {
    // TODO: 
    // 1. Connect to blockchain via Web3 provider (alchemy, infura, etc.)
    // 2. Create transaction to call mint function on smart contract
    // 3. Wait for confirmation
    // 4. Return transaction details
    
    Err("Blockchain minting not configured".to_string())
}

#[derive(Debug, Serialize)]
pub struct MintResult {
    pub token_id: String,
    pub transaction_hash: String,
    pub block_number: u64,
    pub gas_used: u64,
}

/// Verify NFT ownership on blockchain
pub async fn verify_nft_ownership(
    contract_address: &str,
    token_id: &str,
    expected_owner: &str,
    network: BlockchainNetwork,
) -> Result<bool, String> {
    // TODO: Query blockchain to verify owner of token_id
    
    Ok(false)
}

/// Check if certificate has been revoked
pub async fn check_revocation_status(
    contract_address: &str,
    token_id: &str,
    network: BlockchainNetwork,
) -> Result<bool, String> {
    // TODO: Check revocation registry on blockchain
    
    Ok(false)
}

/// Generate certificate hash for verification
pub fn generate_certificate_hash(
    certificate_id: Uuid,
    recipient_name: &str,
    issue_date: &str,
    institution_id: Uuid,
) -> String {
    use sha2::{Digest, Sha256};
    
    let mut hasher = Sha256::new();
    hasher.update(certificate_id.as_bytes());
    hasher.update(recipient_name.as_bytes());
    hasher.update(issue_date.as_bytes());
    hasher.update(institution_id.as_bytes());
    
    hex::encode(hasher.finalize())
}
