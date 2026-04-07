//! Authentication service

use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};

use crate::models::User;
use crate::utils::app_state::AppState;

/// JWT claims
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Claims {
    pub sub: String,
    pub iat: i64,
    pub exp: i64,
    pub role: String,
    pub institution_id: String,
}

impl Claims {
    pub fn new(user: &User, expiry_hours: i64) -> Self {
        let now = Utc::now();
        Self {
            sub: user.id.to_string(),
            iat: now.timestamp(),
            exp: (now + Duration::hours(expiry_hours)).timestamp(),
            role: user.role.to_string(),
            institution_id: user.institution_id.to_string(),
        }
    }
}

/// Generate JWT token
pub fn generate_token(user: &User, secret: &str, expiry_hours: i64) -> Result<String, JwtError> {
    let claims = Claims::new(user, expiry_hours);
    
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(JwtError::Encode)
}

/// Validate JWT token
pub fn validate_token(token: &str, secret: &str) -> Result<Claims, JwtError> {
    let validation = Validation::default();
    
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &validation,
    )
    .map(|data| data.claims)
    .map_err(JwtError::Decode)
}

/// JWT error type
#[derive(Debug)]
pub enum JwtError {
    Encode(jsonwebtoken::errors::Error),
    Decode(jsonwebtoken::errors::Error),
}

impl std::fmt::Display for JwtError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JwtError::Encode(e) => write!(f, "JWT encode error: {}", e),
            JwtError::Decode(e) => write!(f, "JWT decode error: {}", e),
        }
    }
}

impl std::error::Error for JwtError {}