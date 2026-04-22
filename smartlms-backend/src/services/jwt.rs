// JWT token service - handles token creation, validation, and claims
use chrono::{Duration, Utc};
use serde::{Deserialize, Serialize};

const JWT_SECRET: &[u8] = b"smartlms_jwt_secret_change_in_production";
const JWT_EXPIRATION_HOURS: i64 = 24;

/// JWT claims embedded in tokens
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: uuid::Uuid, // User ID
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub role: String,               // User role: admin, instructor, learner
    pub institution_id: uuid::Uuid, // Tenant ID
    pub exp: i64,                   // Expiration timestamp
    pub iat: i64,                   // Issued at timestamp
}

/// Create new JWT token for user
pub fn create_token(
    user_id: uuid::Uuid,
    email: String,
    first_name: String,
    last_name: String,
    role: String,
    institution_id: uuid::Uuid,
) -> Result<String, jsonwebtoken::errors::Error> {
    let now = Utc::now();
    let exp = now + Duration::hours(JWT_EXPIRATION_HOURS);

    let claims = Claims {
        sub: user_id,
        email,
        first_name,
        last_name,
        role,
        institution_id,
        exp: exp.timestamp(),
        iat: now.timestamp(),
    };

    jsonwebtoken::encode(
        &jsonwebtoken::Header::new(jsonwebtoken::Algorithm::HS256),
        &claims,
        &jsonwebtoken::EncodingKey::from_secret(JWT_SECRET),
    )
}

/// Validate JWT token and extract claims
pub fn validate_token(token: &str) -> Result<Claims, String> {
    jsonwebtoken::decode(
        token,
        &jsonwebtoken::DecodingKey::from_secret(JWT_SECRET),
        &jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::HS256),
    )
    .map(|data| data.claims)
    .map_err(|e| e.to_string())
}

/// Refresh token (create new with extended expiration)
pub fn refresh_token(token: &str) -> Result<String, String> {
    let claims = validate_token(token)?;

    create_token(
        claims.sub,
        claims.email,
        claims.first_name,
        claims.last_name,
        claims.role,
        claims.institution_id,
    )
    .map_err(|e| e.to_string())
}

/// Get token expiration as Duration
pub fn get_expiration() -> Duration {
    Duration::hours(JWT_EXPIRATION_HOURS)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_and_validate_token() {
        let token = create_token(
            uuid::Uuid::new_v4(),
            "test@example.com".to_string(),
            "Test".to_string(),
            "User".to_string(),
            "admin".to_string(),
            uuid::Uuid::new_v4(),
        )
        .unwrap();

        let claims = validate_token(&token).unwrap();

        assert_eq!(claims.email, "test@example.com");
        assert_eq!(claims.role, "admin");
    }
}
