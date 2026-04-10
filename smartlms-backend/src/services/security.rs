// Data Protection service - encryption, PII handling, secure storage
use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Nonce,
};
use rand::Rng;
use serde::{Deserialize, Serialize};

/// Field-level encryption for sensitive PII
pub mod encryption {
    use super::*;
    
    /// Encrypt sensitive data using AES-256-GCM
    pub fn encrypt(plaintext: &str, key: &[u8; 32]) -> Result<String, String> {
        let cipher = Aes256Gcm::new_from_slice(key)
            .map_err(|e| format!("Invalid key: {}", e))?;
        
        // Generate random 96-bit nonce
        let mut nonce_bytes = [0u8; 12];
        rand::thread_rng().fill(&mut nonce_bytes);
        let nonce = Nonce::from_slice(&nonce_bytes);
        
        let ciphertext = cipher
            .encrypt(nonce, plaintext.as_bytes())
            .map_err(|e| format!("Encryption failed: {}", e))?;
        
        // Prepend nonce to ciphertext for storage
        let mut result = nonce_bytes.to_vec();
        result.extend(ciphertext);
        
        Ok(base64::encode(&result))
    }
    
    /// Decrypt data encrypted with encrypt()
    pub fn decrypt(encrypted: &str, key: &[u8; 32]) -> Result<String, String> {
        let data = base64::decode(encrypted)
            .map_err(|e| format!("Invalid base64: {}", e))?;
        
        if data.len() < 12 {
            return Err("Invalid encrypted data".to_string());
        }
        
        let (nonce_bytes, ciphertext) = data.split_at(12);
        let nonce = Nonce::from_slice(nonce_bytes);
        
        let cipher = Aes256Gcm::new_from_slice(key)
            .map_err(|e| format!("Invalid key: {}", e))?;
        
        let plaintext = cipher
            .decrypt(nonce, ciphertext)
            .map_err(|e| format!("Decryption failed: {}", e))?;
        
        String::from_utf8(plaintext)
            .map_err(|e| format!("Invalid UTF-8: {}", e))
    }
    
    /// Generate a new random encryption key
    pub fn generate_key() -> [u8; 32] {
        let mut key = [0u8; 32];
        rand::thread_rng().fill(&mut key);
        key
    }
}

/// PII detection and handling
pub mod pii {
    use regex::Regex;
    
    /// Categories of PII
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum PiiCategory {
        Email,
        Phone,
        NationalId,
        Passport,
        CreditCard,
        BankAccount,
        DateOfBirth,
        Address,
        Custom(String),
    }
    
    /// PII detection result
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct PiiMatch {
        pub category: PiiCategory,
        pub value: String,
        pub start: usize,
        pub end: usize,
        pub masked: String,
    }
    
    /// Detect PII in text and return matches with masked versions
    pub fn detect(text: &str) -> Vec<PiiMatch> {
        let mut matches = Vec::new();
        
        // Email pattern
        let email_re = Regex::new(r"[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}").unwrap();
        for m in email_re.find_iter(text) {
            matches.push(PiiMatch {
                category: PiiCategory::Email,
                value: m.as_str().to_string(),
                start: m.start(),
                end: m.end(),
                masked: mask_email(m.as_str()),
            });
        }
        
        // Phone patterns (various formats)
        let phone_re = Regex::new(r"\+?[0-9]{10,15}").unwrap();
        for m in phone_re.find_iter(text) {
            matches.push(PiiMatch {
                category: PiiCategory::Phone,
                value: m.as_str().to_string(),
                start: m.start(),
                end: m.end(),
                masked: mask_phone(m.as_str()),
            });
        }
        
        // National ID (generic pattern - customize per country)
        let id_re = Regex::new(r"\b[A-Z0-9]{6,12}\b").unwrap();
        for m in id_re.find_iter(text) {
            if !m.as_str().chars().all(|c| c.is_ascii_digit()) {
                matches.push(PiiMatch {
                    category: PiiCategory::NationalId,
                    value: m.as_str().to_string(),
                    start: m.start(),
                    end: m.end(),
                    masked: mask_generic(m.as_str()),
                });
            }
        }
        
        matches
    }
    
    /// Mask email, showing only first char and domain
    fn mask_email(email: &str) -> String {
        if let Some(at_idx) = email.find('@') {
            let first = email.chars().next().unwrap_or('*');
            let domain = &email[at_idx..];
            format!("{}****{}", first, domain)
        }
        mask_generic(email)
    }
    
    /// Mask phone number
    fn mask_phone(phone: &str) -> String {
        let digits: String = phone.chars().filter(|c| c.is_ascii_digit()).collect();
        if digits.len() >= 4 {
            format!("****{}", &digits[digits.len()-4..])
        } else {
            "****".to_string()
        }
    }
    
    /// Generic mask - show first 2 and last 2 chars
    fn mask_generic(s: &str) -> String {
        let chars: Vec<char> = s.chars().collect();
        if chars.len() <= 4 {
            "*".repeat(chars.len())
        } else {
            format!("{}****{}", chars[0], chars[chars.len()-1])
        }
    }
    
    /// Redact all PII from text
    pub fn redact(text: &str) -> String {
        let matches = detect(text);
        let mut result = text.to_string();
        
        // Replace from end to avoid offset issues
        for m in matches.iter().rev() {
            result.replace_range(m.start..m.end(), &m.masked);
        }
        
        result
    }
}

/// Data retention policies
pub mod retention {
    use chrono::{DateTime, Utc, Duration};
    
    /// Retention period by data type
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum RetentionPeriod {
        Days(i64),      // Keep for N days, then delete
        Forever,        // Never auto-delete
        OnDeletion,     // Delete when parent record deleted
    }
    
    /// Check if data should be retained or purged
    pub fn should_purge(created_at: DateTime<Utc>, period: &RetentionPeriod) -> bool {
        match period {
            RetentionPeriod::Days(n) => {
                let cutoff = Utc::now() - Duration::days(*n);
                created_at < cutoff
            }
            RetentionPeriod::Forever => false,
            RetentionPeriod::OnDeletion => false, // Handle via cascade
        }
    }
}

/// Secure config storage
pub mod config {
    use super::*;
    use std::collections::HashMap;
    
    /// Application secrets (loaded from env, never stored in DB)
    #[derive(Debug, Clone)]
    pub struct Secrets {
        pub jwt_secret: Vec<u8>,
        pub encryption_key: [u8; 32],
        pub db_encryption_key: [u8; 32],
    }
    
    impl Secrets {
        pub fn from_env() -> Self {
            Self {
                jwt_secret: std::env::var("JWT_SECRET")
                    .unwrap_or_else(|_| "change_me_in_production".to_string())
                    .into_bytes(),
                encryption_key: std::env::var("ENCRYPTION_KEY")
                    .map(|k| {
                        let decoded = base64::decode(&k).unwrap_or_default();
                        let mut key = [0u8; 32];
                        key.copy_from_slice(&decoded[..32.min(decoded.len())]);
                        key
                    })
                    .unwrap_or_else(|_| encryption::generate_key()),
                db_encryption_key: std::env::var("DB_ENCRYPTION_KEY")
                    .map(|k| {
                        let decoded = base64::decode(&k).unwrap_or_default();
                        let mut key = [0u8; 32];
                        key.copy_from_slice(&decoded[..32.min(decoded.len())]);
                        key
                    })
                    .unwrap_or_else(|_| encryption::generate_key()),
            }
        }
    }
}