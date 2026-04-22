# Phase 18: Blockchain Certificates - Implementation Guide

## Overview

Phase 18 implements blockchain-based certificate verification using NFTs (Non-Fungible Tokens). This feature allows institutions to issue tamper-proof, verifiable certificates on the blockchain that can be publicly verified without accessing the LMS database.

## Features Implemented

### 1. Database Schema (`migrations/008_blockchain_certificates.sql`)

- **certificate_templates**: Store certificate designs and layouts
- **certificates**: Issued certificates with QR codes
- **user_wallets**: Connect user accounts to crypto wallets
- **blockchain_contracts**: Smart contract configurations
- **nft_certificates**: Blockchain minting records
- **batch_mint_jobs**: Bulk certificate minting
- **certificate_verifications**: Verification audit log
- **gas_prices_cache**: Cached gas prices for cost estimation
- **certificate_revocations**: Revocation registry

### 2. Backend Service (`src/services/blockchain.rs`)

Core business logic for:
- Wallet connection and management
- NFT certificate creation and minting
- Batch operations for bulk minting
- Gas price management
- Certificate revocation
- Verification logging

### 3. API Endpoints (`src/api/blockchain.rs`)

#### Certificate Minting
- `POST /api/blockchain/certificates/:certificate_id/mint` - Mint a single certificate as NFT
- `POST /api/blockchain/certificates/batch-mint` - Queue batch minting job
- `GET /api/blockchain/certificates/batch/:batch_id/status` - Check batch job status

#### Verification
- `POST /api/blockchain/verify` - Verify certificate by transaction hash or token ID
- `GET /api/blockchain/verify/qr/:code` - Verify via QR code
- `GET /api/blockchain/certificates/:certificate_id/proof` - Get blockchain proof

#### Wallet Integration
- `POST /api/blockchain/wallet/connect` - Connect crypto wallet
- `POST /api/blockchain/wallet/:user_id/disconnect` - Disconnect wallet
- `POST /api/blockchain/wallet/withdraw` - Transfer NFT to user wallet

#### Utilities
- `GET /api/blockchain/certificates/:certificate_id/qr` - Generate QR code
- `GET /api/blockchain/gas/prices` - Current gas prices
- `POST /api/blockchain/gas/estimate` - Estimate gas costs
- `GET /api/blockchain/public/:identifier` - Public verification portal

## Supported Blockchains

| Network | Chain ID | Explorer |
|---------|----------|----------|
| Ethereum | 1 | etherscan.io |
| Polygon | 137 | polygonscan.com |
| Polygon Mumbai (Testnet) | 80001 | mumbai.polygonscan.com |
| Binance Smart Chain | 56 | bscscan.com |

## Usage Examples

### 1. Connect User Wallet

```bash
curl -X POST http://localhost:8000/api/blockchain/wallet/connect \
  -H "Content-Type: application/json" \
  -d '{
    "user_id": "550e8400-e29b-41d4-a716-446655440000",
    "wallet_address": "0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb",
    "signature": "0x...",
    "message": "Sign to connect wallet"
  }'
```

### 2. Mint Certificate as NFT

```bash
curl -X POST http://localhost:8000/api/blockchain/certificates/550e8400-e29b-41d4-a716-446655440001/mint \
  -H "Content-Type: application/json" \
  -d '{
    "user_id": "550e8400-e29b-41d4-a716-446655440000",
    "network": "polygon_mumbai"
  }'
```

Response:
```json
{
  "success": true,
  "certificate_id": "550e8400-e29b-41d4-a716-446655440001",
  "token_id": "ABC12345-XYZ9",
  "transaction_hash": "0x1234567890abcdef...",
  "status": "minted",
  "error": null
}
```

### 3. Verify Certificate

```bash
curl -X POST http://localhost:8000/api/blockchain/verify \
  -H "Content-Type: application/json" \
  -d '{
    "transaction_hash": "0x1234567890abcdef..."
  }'
```

Response:
```json
{
  "is_valid": true,
  "certificate_info": {
    "certificate_name": "Course Completion",
    "recipient_name": "John Doe",
    "institution_name": "University",
    "issue_date": "2024-01-15T10:00:00Z",
    "credential_type": "NFT Certificate"
  },
  "blockchain_proof": {
    "network": "polygon_mumbai",
    "contract_address": "0x1234567890123456789012345678901234567890",
    "token_id": "ABC12345-XYZ9",
    "transaction_hash": "0x1234567890abcdef...",
    "block_number": 12345678,
    "explorer_url": "https://mumbai.polygonscan.com/tx/0x..."
  },
  "verification_timestamp": "2024-01-15T12:00:00Z",
  "error": null
}
```

### 4. Batch Mint Certificates

```bash
curl -X POST http://localhost:8000/api/blockchain/certificates/batch-mint \
  -H "Content-Type: application/json" \
  -d '{
    "institution_id": "550e8400-e29b-41d4-a716-446655440000",
    "certificate_ids": [
      "550e8400-e29b-41d4-a716-446655440001",
      "550e8400-e29b-41d4-a716-446655440002"
    ],
    "network": "polygon_mumbai",
    "priority": "normal"
  }'
```

## Architecture

```
┌─────────────┐     ┌──────────────┐     ┌─────────────────┐
│   Frontend  │────▶│  API Layer   │────▶│ Service Layer   │
│  (React)    │     │ (Axum/Rust)  │     │ (Business Logic)│
└─────────────┘     └──────────────┘     └────────┬────────┘
                                                   │
                    ┌──────────────────────────────┼──────────────────────────────┐
                    │                              │                              │
                    ▼                              ▼                              ▼
           ┌────────────────┐          ┌──────────────────┐          ┌─────────────────┐
           │  PostgreSQL    │          │  Blockchain Node │          │  IPFS Storage   │
           │  (Database)    │          │  (RPC Provider)  │          │  (Metadata)     │
           └────────────────┘          └──────────────────┘          └─────────────────┘
```

## Production Deployment Considerations

### 1. Smart Contract Deployment

Before using the system in production:
1. Deploy the NFT certificate smart contract to your chosen blockchain
2. Update the `blockchain_contracts` table with the deployed contract address
3. Configure the contract ABI and gas limits

Example smart contract features needed:
- `mint(address recipient, uint256 tokenId, string memory uri)` 
- `revoke(uint256 tokenId)`
- `ownerOf(uint256 tokenId) -> address`
- `tokenURI(uint256 tokenId) -> string`

### 2. IPFS Integration

For decentralized metadata storage:
```rust
// Upload certificate metadata to IPFS
let ipfs_client = IpfsClient::default();
let metadata = CertificateMetadata {
    name: "Course Completion Certificate",
    description: "...",
    image: "ipfs://Qm...",
    attributes: vec![...],
    // ...
};
let ipfs_hash = ipfs_client.upload_json(&metadata).await?;
```

### 3. Blockchain RPC Provider

Configure RPC endpoints for blockchain interaction:
- Use services like Infura, Alchemy, or QuickNode
- Implement retry logic for failed transactions
- Monitor gas prices for optimal transaction timing

### 4. Security Considerations

- **Signature Verification**: Implement proper cryptographic signature verification using `ethers-rs` or `web3-rs`
- **Private Key Management**: Use secure key management (AWS KMS, HashiCorp Vault)
- **Rate Limiting**: Protect verification endpoints from abuse
- **Audit Trail**: Log all minting and verification operations

### 5. Cost Optimization

- Use Layer 2 solutions (Polygon) for lower gas fees
- Implement batch transactions for multiple certificates
- Cache gas prices and update periodically
- Allow users to choose network/priority

## Testing

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_connect_wallet() {
        let pool = get_test_pool().await;
        let result = blockchain::service::connect_wallet(
            &pool,
            test_user_id(),
            "0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb",
            BlockchainNetwork::PolygonMumbai,
            "0x...",
            "test message"
        ).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_verify_certificate() {
        // Test verification logic
    }
}
```

### Integration Tests

1. Deploy contracts to testnet (Mumbai)
2. Run end-to-end minting flow
3. Verify on blockchain explorer
4. Test revocation process

## Migration Guide

### From Traditional Certificates

1. Run migration: `sqlx migrate run`
2. Existing certificates remain valid
3. Enable NFT minting for new certificates
4. Offer retroactive minting for past graduates

### Data Migration Script

```sql
-- Example: Migrate existing certificates to NFT format
INSERT INTO nft_certificates (id, certificate_id, user_id, course_id, institution_id, network, mint_status, created_at)
SELECT gen_random_uuid(), id, recipient_user_id, course_id, 
       (SELECT institution_id FROM certificate_templates WHERE id = template_id),
       'polygon_mumbai', 'pending', NOW()
FROM certificates
WHERE course_id IS NOT NULL;
```

## Troubleshooting

### Common Issues

1. **Transaction Fails**
   - Check gas limit is sufficient
   - Verify contract is deployed correctly
   - Ensure wallet has enough funds

2. **Verification Returns Invalid**
   - Check certificate hasn't been revoked
   - Verify transaction is confirmed on blockchain
   - Confirm correct network is being queried

3. **High Gas Costs**
   - Switch to Layer 2 (Polygon)
   - Use batch minting
   - Schedule minting during low-traffic periods

## Future Enhancements

- [ ] Multi-chain support (automatic network selection)
- [ ] Dynamic NFTs (updatable metadata)
- [ ] Verifiable Credentials (W3C standard)
- [ ] Zero-knowledge proofs for privacy
- [ ] Cross-chain bridge support
- [ ] Mobile wallet integration (WalletConnect)
- [ ] Automatic royalty distribution

## References

- [ERC-721 Standard](https://eips.ethereum.org/EIPS/eip-721)
- [OpenZeppelin NFT Contracts](https://docs.openzeppelin.com/contracts/4.x/erc721)
- [IPFS Documentation](https://docs.ipfs.tech/)
- [Ethers.js](https://docs.ethers.org/)
- [W3C Verifiable Credentials](https://www.w3.org/TR/vc-data-model/)
