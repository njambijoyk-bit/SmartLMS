//! User service

use crate::models::{User, UserRole, UserStatus};

/// Hash password using bcrypt
pub fn hash_password(password: &str) -> Result<String, PasswordError> {
    bcrypt::hash(password, 10).map_err(PasswordError::Hash)
}

/// Verify password
pub fn verify_password(password: &str, hash: &str) -> Result<bool, PasswordError> {
    bcrypt::verify(password, hash).map_err(PasswordError::Verify)
}

/// Password error
#[derive(Debug)]
pub enum PasswordError {
    Hash(bcrypt::BcryptError),
    Verify(bcrypt::BcryptError),
}

impl std::fmt::Display for PasswordError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PasswordError::Hash(e) => write!(f, "Password hash error: {}", e),
            PasswordError::Verify(e) => write!(f, "Password verify error: {}", e),
        }
    }
}

impl std::error::Error for PasswordError {}

/// Validate password strength
pub fn validate_password(password: &str) -> Result<(), PasswordError> {
    if password.len() < 8 {
        return Err(PasswordError::Hash(
            bcrypt::BcryptError::from(bcrypt::BcryptError::InvalidPassword)
        ));
    }
    Ok(())
}