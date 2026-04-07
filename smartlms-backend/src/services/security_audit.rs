// Security & Audit Service - Security hardening, compliance, audit logging
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// Audit log entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditLog {
    pub id: uuid::Uuid,
    pub user_id: Option<uuid::Uuid>,
    pub institution_id: uuid::Uuid,
    pub action: String,
    pub resource_type: String,
    pub resource_id: Option<uuid::Uuid>,
    pub details: Option<String>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub created_at: DateTime<Utc>,
}

/// Security event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityEvent {
    pub id: uuid::Uuid,
    pub event_type: SecurityEventType,
    pub severity: EventSeverity,
    pub user_id: Option<uuid::Uuid>,
    pub ip_address: Option<String>,
    pub description: String,
    pub metadata: std::collections::HashMap<String, String>,
    pub resolved: bool,
    pub created_at: DateTime<Utc>,
}

/// Security event types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SecurityEventType {
    LoginSuccess,
    LoginFailed,
    PasswordChanged,
    PasswordReset,
    MfaEnabled,
    MfaDisabled,
    SuspiciousActivity,
    BruteForceAttempt,
    SqlInjectionAttempt,
    XssAttempt,
    RateLimitExceeded,
}

/// Event severity
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EventSeverity {
    Info,
    Warning,
    Error,
    Critical,
}

/// Access policy (session management)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessPolicy {
    pub id: uuid::Uuid,
    pub institution_id: uuid::Uuid,
    pub max_concurrent_sessions: i32,
    pub session_timeout_minutes: i32,
    pub require_mfa: bool,
    pub allowed_ip_ranges: Vec<String>,
    pub blocked_countries: Vec<String>,
}

/// API key for external access
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKey {
    pub id: uuid::Uuid,
    pub institution_id: uuid::Uuid,
    pub name: String,
    pub key_hash: String,
    pub permissions: Vec<String>,
    pub last_used_at: Option<DateTime<Utc>>,
    pub expires_at: Option<DateTime<Utc>>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
}

// Service functions
pub mod service {
    use super::*;
    
    /// Log audit event
    pub async fn log_audit(
        pool: &PgPool,
        institution_id: uuid::Uuid,
        action: &str,
        resource_type: &str,
        resource_id: Option<uuid::Uuid>,
        user_id: Option<uuid::Uuid>,
        details: Option<&str>,
        ip_address: Option<&str>,
    ) -> Result<AuditLog, String> {
        let id = Uuid::new_v4();
        
        sqlx::query!(
            "INSERT INTO audit_logs (id, user_id, institution_id, action, resource_type,
             resource_id, details, ip_address, created_at)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)",
            id, user_id, institution_id, action, resource_type, resource_id, 
            details, ip_address, Utc::now()
        )
        .execute(pool)
        .await
        .map_err(|e| e.to_string())?;
        
        Ok(AuditLog {
            id,
            user_id,
            institution_id,
            action: action.to_string(),
            resource_type: resource_type.to_string(),
            resource_id,
            details: details.map(String::from),
            ip_address: ip_address.map(String::from),
            user_agent: None,
            created_at: Utc::now(),
        })
    }
    
    /// Get audit logs with filters
    pub async fn get_audit_logs(
        pool: &PgPool,
        institution_id: uuid::Uuid,
        user_id: Option<uuid::Uuid>,
        action: Option<&str>,
        resource_type: Option<&str>,
        start_date: Option<DateTime<Utc>>,
        end_date: Option<DateTime<Utc>>,
        limit: i64,
    ) -> Result<Vec<AuditLog>, String> {
        let rows = sqlx::query!(
            "SELECT id, user_id, institution_id, action, resource_type, resource_id,
             details, ip_address, user_agent, created_at
             FROM audit_logs 
             WHERE institution_id = $1
             ORDER BY created_at DESC LIMIT $2",
            institution_id, limit
        )
        .fetch_all(pool)
        .await
        .map_err(|e| e.to_string())?;
        
        Ok(rows.into_iter().map(|r| AuditLog {
            id: r.id,
            user_id: r.user_id,
            institution_id: r.institution_id,
            action: r.action,
            resource_type: r.resource_type,
            resource_id: r.resource_id,
            details: r.details,
            ip_address: r.ip_address,
            user_agent: r.user_agent,
            created_at: r.created_at,
        }).collect())
    }
    
    /// Log security event
    pub async fn log_security_event(
        pool: &PgPool,
        event_type: SecurityEventType,
        severity: EventSeverity,
        description: &str,
        user_id: Option<uuid::Uuid>,
        ip_address: Option<&str>,
    ) -> Result<SecurityEvent, String> {
        let id = Uuid::new_v4();
        
        sqlx::query!(
            "INSERT INTO security_events (id, event_type, severity, user_id, ip_address,
             description, resolved, created_at)
             VALUES ($1, $2, $3, $4, $5, $6, false, $7)",
            id, format!("{:?}", event_type).to_lowercase(), format!("{:?}", severity).to_lowercase(),
            user_id, ip_address, description, Utc::now()
        )
        .execute(pool)
        .await
        .map_err(|e| e.to_string())?;
        
        // Alert if critical
        if severity == EventSeverity::Critical {
            tracing::error!("SECURITY ALERT: {} - {}", event_type, description);
        }
        
        Ok(SecurityEvent {
            id,
            event_type,
            severity,
            user_id,
            ip_address: ip_address.map(String::from),
            description: description.to_string(),
            metadata: std::collections::HashMap::new(),
            resolved: false,
            created_at: Utc::now(),
        })
    }
    
    /// Get unresolved security events
    pub async fn get_security_alerts(
        pool: &PgPool,
        institution_id: uuid::Uuid,
    ) -> Result<Vec<SecurityEvent>, String> {
        let rows = sqlx::query!(
            "SELECT id, event_type, severity, user_id, ip_address, description, 
             metadata, resolved, created_at
             FROM security_events 
             WHERE resolved = false AND created_at > NOW() - INTERVAL '7 days'
             ORDER BY created_at DESC",
            institution_id
        )
        .fetch_all(pool)
        .await
        .map_err(|e| e.to_string())?;
        
        Ok(rows.into_iter().map(|r| SecurityEvent {
            id: r.id,
            event_type: SecurityEventType::LoginSuccess,
            severity: EventSeverity::Info,
            user_id: r.user_id,
            ip_address: r.ip_address,
            description: r.description,
            metadata: serde_json::from_str(&r.metadata).unwrap_or_default(),
            resolved: r.resolved,
            created_at: r.created_at,
        }).collect())
    }
    
    /// Resolve security event
    pub async fn resolve_security_event(
        pool: &PgPool,
        event_id: uuid::Uuid,
    ) -> Result<(), String> {
        sqlx::query!("UPDATE security_events SET resolved = true WHERE id = $1", event_id)
            .execute(pool)
            .await
            .map_err(|e| e.to_string())?;
        
        Ok(())
    }
    
    /// Generate API key
    pub async fn create_api_key(
        pool: &PgPool,
        institution_id: uuid::Uuid,
        name: &str,
        permissions: Vec<String>,
    ) -> Result<(ApiKey, String), String> {
        let id = Uuid::new_v4();
        
        // Generate secure random key
        let key = generate_secure_token(32);
        let key_hash = hash_token(&key);
        
        sqlx::query!(
            "INSERT INTO api_keys (id, institution_id, name, key_hash, permissions, is_active, created_at)
             VALUES ($1, $2, $3, $4, $5, true, $6)",
            id, institution_id, name, key_hash, serde_json::to_string(&permissions).unwrap(), Utc::now()
        )
        .execute(pool)
        .await
        .map_err(|e| e.to_string())?;
        
        Ok((ApiKey {
            id,
            institution_id,
            name: name.to_string(),
            key_hash,
            permissions,
            last_used_at: None,
            expires_at: None,
            is_active: true,
            created_at: Utc::now(),
        }, key))  // Return the raw key - only shown once!
    }
    
    /// Validate API key
    pub async fn validate_api_key(
        pool: &PgPool,
        key: &str,
    ) -> Result<Option<ApiKey>, String> {
        let key_hash = hash_token(key);
        
        let row = sqlx::query!(
            "SELECT id, institution_id, name, key_hash, permissions, last_used_at,
             expires_at, is_active, created_at
             FROM api_keys WHERE key_hash = $1 AND is_active = true",
            key_hash
        )
        .fetch_optional(pool)
        .await
        .map_err(|e| e.to_string())?;
        
        // Update last used
        if let Some(r) = &row {
            sqlx::query!("UPDATE api_keys SET last_used_at = $1 WHERE id = $2", Utc::now(), r.id)
                .execute(pool)
                .await
                .ok();
        }
        
        Ok(row.map(|r| ApiKey {
            id: r.id,
            institution_id: r.institution_id,
            name: r.name,
            key_hash: r.key_hash,
            permissions: serde_json::from_str(&r.permissions).unwrap_or_default(),
            last_used_at: r.last_used_at,
            expires_at: r.expires_at,
            is_active: r.is_active,
            created_at: r.created_at,
        }))
    }
    
    /// Revoke API key
    pub async fn revoke_api_key(
        pool: &PgPool,
        key_id: uuid::Uuid,
    ) -> Result<(), String> {
        sqlx::query!("UPDATE api_keys SET is_active = false WHERE id = $1", key_id)
            .execute(pool)
            .await
            .map_err(|e| e.to_string())?;
        
        Ok(())
    }
    
    /// Get access policy
    pub async fn get_access_policy(
        pool: &PgPool,
        institution_id: uuid::Uuid,
    ) -> Result<AccessPolicy, String> {
        let row = sqlx::query!(
            "SELECT id, institution_id, max_concurrent_sessions, session_timeout_minutes,
             require_mfa, allowed_ip_ranges, blocked_countries
             FROM access_policies WHERE institution_id = $1",
            institution_id
        )
        .fetch_optional(pool)
        .await
        .map_err(|e| e.to_string())?;
        
        Ok(row.map(|r| AccessPolicy {
            id: r.id,
            institution_id: r.institution_id,
            max_concurrent_sessions: r.max_concurrent_sessions,
            session_timeout_minutes: r.session_timeout_minutes,
            require_mfa: r.require_mfa,
            allowed_ip_ranges: serde_json::from_str(&r.allowed_ip_ranges).unwrap_or_default(),
            blocked_countries: serde_json::from_str(&r.blocked_countries).unwrap_or_default(),
        }).unwrap_or(AccessPolicy {
            id: Uuid::new_v4(),
            institution_id,
            max_concurrent_sessions: 3,
            session_timeout_minutes: 480,
            require_mfa: false,
            allowed_ip_ranges: vec![],
            blocked_countries: vec![],
        }))
    }
}

fn generate_secure_token(length: usize) -> String {
    use rand::Rng;
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
    let mut rng = rand::thread_rng();
    (0..length)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect()
}

fn hash_token(token: &str) -> String {
    use sha2::{Sha256, Digest};
    let mut hasher = Sha256::new();
    hasher.update(token.as_bytes());
    format!("{:x}", hasher.finalize())
}