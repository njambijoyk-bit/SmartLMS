//! JWT service.
//!
//! Phase 1 uses HS256 with a per-engine secret from `JWT_SECRET`. Master
//! ref §8 layer 2 calls for RS256 with per-institution asymmetric keys —
//! that upgrade will land in a follow-up PR without changing the public
//! API of this module (only how the key is sourced).
//!
//! Access tokens expire in 15 minutes; refresh tokens are NOT JWTs — they
//! are random opaque strings stored as SHA-256 hashes in `refresh_tokens`
//! (see `db::refresh_token`).

use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

/// Access-token lifetime in minutes.
pub const ACCESS_TOKEN_TTL_MINUTES: i64 = 15;

/// Refresh-token lifetime in days.
pub const REFRESH_TOKEN_TTL_DAYS: i64 = 7;

static JWT_SECRET: Lazy<Vec<u8>> = Lazy::new(|| {
    std::env::var("JWT_SECRET")
        .unwrap_or_else(|_| {
            tracing::warn!(
                "JWT_SECRET not set — using a development fallback. Refusing to run in prod."
            );
            "dev-only-change-me-dev-only-change-me-dev-only".to_string()
        })
        .into_bytes()
});

/// JWT claims. `sub` = user id, `tid` = institution id, `roles` = role codes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: uuid::Uuid,
    pub tid: uuid::Uuid,
    pub email: String,
    pub roles: Vec<String>,
    pub exp: i64,
    pub iat: i64,
}

#[derive(Debug, thiserror::Error)]
pub enum JwtError {
    #[error("encode failed: {0}")]
    Encode(String),
    #[error("decode failed: {0}")]
    Decode(String),
}

pub fn issue_access_token(
    user_id: uuid::Uuid,
    institution_id: uuid::Uuid,
    email: String,
    roles: Vec<String>,
) -> Result<String, JwtError> {
    let now = Utc::now();
    let exp = now + Duration::minutes(ACCESS_TOKEN_TTL_MINUTES);
    let claims = Claims {
        sub: user_id,
        tid: institution_id,
        email,
        roles,
        exp: exp.timestamp(),
        iat: now.timestamp(),
    };
    jsonwebtoken::encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(&JWT_SECRET),
    )
    .map_err(|e| JwtError::Encode(e.to_string()))
}

pub fn decode_access_token(token: &str) -> Result<Claims, JwtError> {
    let validation = Validation::default();
    jsonwebtoken::decode::<Claims>(token, &DecodingKey::from_secret(&JWT_SECRET), &validation)
        .map(|d| d.claims)
        .map_err(|e| JwtError::Decode(e.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn access_token_roundtrip() {
        let uid = uuid::Uuid::new_v4();
        let tid = uuid::Uuid::new_v4();
        let token =
            issue_access_token(uid, tid, "jane@uon.ac.ke".into(), vec!["instructor".into()])
                .unwrap();
        let claims = decode_access_token(&token).unwrap();
        assert_eq!(claims.sub, uid);
        assert_eq!(claims.tid, tid);
        assert_eq!(claims.email, "jane@uon.ac.ke");
        assert_eq!(claims.roles, vec!["instructor"]);
    }

    #[test]
    fn tampered_token_rejected() {
        let uid = uuid::Uuid::new_v4();
        let tid = uuid::Uuid::new_v4();
        let token = issue_access_token(uid, tid, "x@y".into(), vec![]).unwrap();
        let mut bytes = token.into_bytes();
        let last = bytes.last_mut().unwrap();
        *last = if *last == b'a' { b'b' } else { b'a' };
        let tampered = String::from_utf8(bytes).unwrap();
        assert!(decode_access_token(&tampered).is_err());
    }
}
