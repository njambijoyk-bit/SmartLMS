// Certificate Service - Issue certificates with QR verification
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// Certificate template
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CertificateTemplate {
    pub id: uuid::Uuid,
    pub institution_id: uuid::Uuid,
    pub name: String,
    pub description: Option<String>,
    pub template_type: CertificateType,
    pub background_url: Option<String>,
    pub logo_url: Option<String>,
    pub signature_urls: Vec<String>,
    pub content_html: String,
    pub is_default: bool,
}

/// Certificate type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CertificateType {
    CourseCompletion,
    CourseCompletionWithGrade,
    Attendance,
    Participation,
    Custom,
}

/// Issued certificate
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Certificate {
    pub id: uuid::Uuid,
    pub template_id: uuid::Uuid,
    pub recipient_user_id: uuid::Uuid,
    pub course_id: Option<uuid::Uuid>,
    pub credential_id: String,  // Unique ID for verification
    pub qr_code_url: String,
    pub recipient_name: String,
    pub issue_date: DateTime<Utc>,
    pub expiry_date: Option<DateTime<Utc>>,
    pub metadata: std::collections::HashMap<String, String>,
    pub status: CertificateStatus,
    pub pdf_url: Option<String>,
}

/// Certificate status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CertificateStatus {
    Active,
    Revoked,
    Expired,
}

/// Verification response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationResponse {
    pub valid: bool,
    pub certificate: Option<Certificate>,
    pub message: String,
}

// Service functions
pub mod service {
    use super::*;
    
    /// Create certificate template
    pub async fn create_template(
        pool: &PgPool,
        institution_id: uuid::Uuid,
        name: &str,
        template_type: CertificateType,
        content_html: &str,
    ) -> Result<CertificateTemplate, String> {
        let id = Uuid::new_v4();
        
        sqlx::query!(
            "INSERT INTO certificate_templates (id, institution_id, name, template_type, 
             content_html, is_default, created_at)
             VALUES ($1, $2, $3, $4, $5, false, $6)",
            id, institution_id, name, format!("{:?}", template_type).to_lowercase(),
            content_html, Utc::now()
        )
        .execute(pool)
        .await
        .map_err(|e| e.to_string())?;
        
        Ok(CertificateTemplate {
            id,
            institution_id,
            name: name.to_string(),
            description: None,
            template_type,
            background_url: None,
            logo_url: None,
            signature_urls: vec![],
            content_html: content_html.to_string(),
            is_default: false,
        })
    }
    
    /// Issue certificate to user
    pub async fn issue_certificate(
        pool: &PgPool,
        template_id: uuid::Uuid,
        user_id: uuid::Uuid,
        recipient_name: &str,
        course_id: Option<uuid::Uuid>,
    ) -> Result<Certificate, String> {
        let id = Uuid::new_v4();
        let now = Utc::now();
        
        // Generate unique credential ID
        let credential_id = format!("CERT-{}-{}", now.format("%Y%m%d"), &Uuid::new_v4().to_string()[..8].to_uppercase());
        
        // Generate QR code (verification URL)
        let qr_code_url = format!("/verify/{}", credential_id);
        
        sqlx::query!(
            "INSERT INTO certificates (id, template_id, recipient_user_id, course_id, 
             credential_id, qr_code_url, recipient_name, issue_date, status)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, 'active')",
            id, template_id, user_id, course_id, credential_id, qr_code_url, recipient_name, now
        )
        .execute(pool)
        .await
        .map_err(|e| e.to_string())?;
        
        // In production: generate PDF and upload to storage
        
        Ok(Certificate {
            id,
            template_id,
            recipient_user_id: user_id,
            course_id,
            credential_id,
            qr_code_url,
            recipient_name: recipient_name.to_string(),
            issue_date: now,
            expiry_date: None,
            metadata: std::collections::HashMap::new(),
            status: CertificateStatus::Active,
            pdf_url: None,
        })
    }
    
    /// Verify certificate by credential ID
    pub async fn verify_certificate(
        pool: &PgPool,
        credential_id: &str,
    ) -> Result<VerificationResponse, String> {
        let row = sqlx::query!(
            "SELECT c.id, c.template_id, c.recipient_user_id, c.course_id, c.credential_id,
                    c.qr_code_url, c.recipient_name, c.issue_date, c.expiry_date, c.status,
                    c.pdf_url, ct.name as template_name
             FROM certificates c
             JOIN certificate_templates ct ON c.template_id = ct.id
             WHERE c.credential_id = $1",
            credential_id
        )
        .fetch_optional(pool)
        .await
        .map_err(|e| e.to_string())?;
        
        match row {
            Some(r) => {
                let status = if r.status == "active" { 
                    // Check if expired
                    if let Some(exp) = r.expiry_date {
                        if exp < Utc::now() {
                            CertificateStatus::Expired
                        } else {
                            CertificateStatus::Active
                        }
                    } else {
                        CertificateStatus::Active
                    }
                } else {
                    CertificateStatus::Revoked
                };
                
                Ok(VerificationResponse {
                    valid: status == CertificateStatus::Active,
                    certificate: Some(Certificate {
                        id: r.id,
                        template_id: r.template_id,
                        recipient_user_id: r.recipient_user_id,
                        course_id: r.course_id,
                        credential_id: r.credential_id,
                        qr_code_url: r.qr_code_url,
                        recipient_name: r.recipient_name,
                        issue_date: r.issue_date,
                        expiry_date: r.expiry_date,
                        metadata: std::collections::HashMap::new(),
                        status,
                        pdf_url: r.pdf_url,
                    }),
                    message: if status == CertificateStatus::Active {
                        "Certificate is valid".to_string()
                    } else {
                        format!("Certificate is {}", status.to_string().to_lowercase())
                    },
                })
            }
            None => Ok(VerificationResponse {
                valid: false,
                certificate: None,
                message: "Certificate not found".to_string(),
            }),
        }
    }
    
    /// Revoke certificate
    pub async fn revoke_certificate(
        pool: &PgPool,
        certificate_id: uuid::Uuid,
        reason: Option<String>,
    ) -> Result<(), String> {
        sqlx::query!(
            "UPDATE certificates SET status = 'revoked' WHERE id = $1",
            certificate_id
        )
        .execute(pool)
        .await
        .map_err(|e| e.to_string())?;
        
        // Log revocation
        tracing::info!("Certificate {} revoked. Reason: {:?}", certificate_id, reason);
        
        Ok(())
    }
    
    /// Get user's certificates
    pub async fn get_user_certificates(
        pool: &PgPool,
        user_id: uuid::Uuid,
    ) -> Result<Vec<Certificate>, String> {
        let rows = sqlx::query!(
            "SELECT id, template_id, recipient_user_id, course_id, credential_id,
                    qr_code_url, recipient_name, issue_date, expiry_date, status, pdf_url
             FROM certificates WHERE recipient_user_id = $1 AND status = 'active'
             ORDER BY issue_date DESC",
            user_id
        )
        .fetch_all(pool)
        .await
        .map_err(|e| e.to_string())?;
        
        Ok(rows.into_iter().map(|r| Certificate {
            id: r.id,
            template_id: r.template_id,
            recipient_user_id: r.recipient_user_id,
            course_id: r.course_id,
            credential_id: r.credential_id,
            qr_code_url: r.qr_code_url,
            recipient_name: r.recipient_name,
            issue_date: r.issue_date,
            expiry_date: r.expiry_date,
            metadata: std::collections::HashMap::new(),
            status: CertificateStatus::Active,
            pdf_url: r.pdf_url,
        }).collect())
    }
}