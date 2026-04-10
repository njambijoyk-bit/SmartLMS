// Custom Domain Service - handles custom domain with auto-TLS
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// Custom domain status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DomainStatus {
    Pending,      // Awaiting DNS verification
    Verifying,    // DNS check in progress
    Active,       // Verified and active
    Failed,       // Verification failed
    Expired,      // TLS certificate expired
}

/// Custom domain record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomDomain {
    pub id: uuid::Uuid,
    pub institution_id: uuid::Uuid,
    pub domain: String,
    pub status: DomainStatus,
    pub verification_token: String,
    pub ssl_cert_arn: Option<String>,  // AWS ACM certificate ARN
    pub cloudfront_arn: Option<String>,
    pub cname_record: Option<String>,
    pub verified_at: Option<DateTime<Utc>>,
    pub expires_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

/// DNS records needed for verification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DnsRecords {
    pub txt_record: TxtRecord,
    pub cname_record: Option<CnameRecord>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TxtRecord {
    pub name: String,
    pub value: String,
    pub ttl: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CnameRecord {
    pub name: String,
    pub value: String,
    pub ttl: i64,
}

// Service functions
pub mod service {
    use super::*;
    
    /// Request a new custom domain
    pub async fn request_domain(
        pool: &PgPool,
        institution_id: uuid::Uuid,
        domain: &str,
    ) -> Result<CustomDomain, String> {
        // Validate domain format
        if !is_valid_domain(domain) {
            return Err("Invalid domain format".to_string());
        }
        
        // Check if domain already exists
        if let Ok(Some(_)) = find_by_domain(pool, domain).await {
            return Err("Domain already in use".to_string());
        }
        
        let id = Uuid::new_v4();
        let now = Utc::now();
        let verification_token = format!("smartlms-{}", Uuid::new_v4());
        
        sqlx::query!(
            "INSERT INTO custom_domains (id, institution_id, domain, status, verification_token, created_at)
             VALUES ($1, $2, $3, 'pending', $4, $5)",
            id, institution_id, domain, verification_token, now
        )
        .execute(pool)
        .await
        .map_err(|e| e.to_string())?;
        
        Ok(CustomDomain {
            id,
            institution_id,
            domain: domain.to_string(),
            status: DomainStatus::Pending,
            verification_token,
            ssl_cert_arn: None,
            cloudfront_arn: None,
            cname_record: None,
            verified_at: None,
            expires_at: None,
            created_at: now,
        })
    }
    
    /// Get DNS records for domain verification
    pub async fn get_dns_records(
        pool: &PgPool,
        domain_id: uuid::Uuid,
    ) -> Result<DnsRecords, String> {
        let domain = find_by_id(pool, domain_id)
            .await
            .map_err(|e| e.to_string())?
            .ok_or("Domain not found")?;
        
        // TXT record for domain verification
        let txt_record = TxtRecord {
            name: domain.domain.clone(),
            value: domain.verification_token.clone(),
            ttl: 3600,
        };
        
        // CNAME record for CDN (if using CloudFront)
        let cname_record = Some(CnameRecord {
            name: format!("cdn.{}", domain.domain),
            value: "d1234567890.cloudfront.net".to_string(),
            ttl: 3600,
        });
        
        Ok(DnsRecords {
            txt_record,
            cname_record,
        })
    }
    
    /// Verify domain ownership (check DNS)
    pub async fn verify_domain(
        pool: &PgPool,
        domain_id: uuid::Uuid,
    ) -> Result<CustomDomain, String> {
        // Update status to verifying
        sqlx::query!(
            "UPDATE custom_domains SET status = 'verifying' WHERE id = $1",
            domain_id
        )
        .execute(pool)
        .await
        .map_err(|e| e.to_string())?;
        
        // In production: query DNS and verify TXT record exists
        // For now, simulate success
        let now = Utc::now();
        let expires = now + chrono::Duration::days(365);
        
        sqlx::query!(
            "UPDATE custom_domains SET status = 'active', verified_at = $1, expires_at = $2 WHERE id = $3",
            now, expires, domain_id
        )
        .execute(pool)
        .await
        .map_err(|e| e.to_string())?;
        
        // In production: trigger TLS certificate provisioning via AWS ACM
        
        find_by_id(pool, domain_id)
            .await
            .map_err(|e| e.to_string())?
            .ok_or("Domain not found".to_string())
    }
    
    /// Renew SSL certificate
    pub async fn renew_certificate(
        pool: &PgPool,
        domain_id: uuid::Uuid,
    ) -> Result<CustomDomain, String> {
        // In production: trigger ACM certificate renewal
        // For now, just update expiration
        
        let new_expiry = Utc::now() + chrono::Duration::days(365);
        
        sqlx::query!(
            "UPDATE custom_domains SET expires_at = $1 WHERE id = $2",
            new_expiry, domain_id
        )
        .execute(pool)
        .await
        .map_err(|e| e.to_string())?;
        
        find_by_id(pool, domain_id)
            .await
            .map_err(|e| e.to_string())?
            .ok_or("Domain not found".to_string())
    }
    
    /// List domains for an institution
    pub async fn list_domains(
        pool: &PgPool,
        institution_id: uuid::Uuid,
    ) -> Result<Vec<CustomDomain>, String> {
        let rows = sqlx::query!(
            "SELECT id, institution_id, domain, status, verification_token, 
             ssl_cert_arn, cloudfront_arn, cname_record, verified_at, expires_at, created_at
             FROM custom_domains WHERE institution_id = $1 ORDER BY created_at DESC",
            institution_id
        )
        .fetch_all(pool)
        .await
        .map_err(|e| e.to_string())?;
        
        Ok(rows.into_iter().map(|r| CustomDomain {
            id: r.id,
            institution_id: r.institution_id,
            domain: r.domain,
            status: DomainStatus::Pending,  // Parse from string
            verification_token: r.verification_token,
            ssl_cert_arn: r.ssl_cert_arn,
            cloudfront_arn: r.cloudfront_arn,
            cname_record: r.cname_record,
            verified_at: r.verified_at,
            expires_at: r.expires_at,
            created_at: r.created_at,
        }).collect())
    }
}

fn is_valid_domain(domain: &str) -> bool {
    // Basic domain validation
    let domain_regex = regex::Regex::new(r"^[a-zA-Z0-9][a-zA-Z0-9-]{0,61}[a-zA-Z0-9]?\.[a-zA-Z]{2,}$").unwrap();
    domain_regex.is_match(domain)
}

async fn find_by_domain(pool: &PgPool, domain: &str) -> Result<Option<CustomDomain>, sqlx::Error> {
    let row = sqlx::query!(
        "SELECT id, institution_id, domain, status, verification_token, 
         ssl_cert_arn, cloudfront_arn, cname_record, verified_at, expires_at, created_at
         FROM custom_domains WHERE domain = $1",
        domain
    )
    .fetch_optional(pool)
    .await?;
    
    Ok(row.map(|r| CustomDomain {
        id: r.id,
        institution_id: r.institution_id,
        domain: r.domain,
        status: DomainStatus::Pending,
        verification_token: r.verification_token,
        ssl_cert_arn: r.ssl_cert_arn,
        cloudfront_arn: r.cloudfront_arn,
        cname_record: r.cname_record,
        verified_at: r.verified_at,
        expires_at: r.expires_at,
        created_at: r.created_at,
    }))
}

async fn find_by_id(pool: &PgPool, id: uuid::Uuid) -> Result<Option<CustomDomain>, sqlx::Error> {
    let row = sqlx::query!(
        "SELECT id, institution_id, domain, status, verification_token, 
         ssl_cert_arn, cloudfront_arn, cname_record, verified_at, expires_at, created_at
         FROM custom_domains WHERE id = $1",
        id
    )
    .fetch_optional(pool)
    .await?;
    
    Ok(row.map(|r| CustomDomain {
        id: r.id,
        institution_id: r.institution_id,
        domain: r.domain,
        status: DomainStatus::Pending,
        verification_token: r.verification_token,
        ssl_cert_arn: r.ssl_cert_arn,
        cloudfront_arn: r.cloudfront_arn,
        cname_record: r.cname_record,
        verified_at: r.verified_at,
        expires_at: r.expires_at,
        created_at: r.created_at,
    }))
}