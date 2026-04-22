//! Argon2id password hashing + verification. Per master ref §8 layer 2.
//!
//! We use defaults from the `argon2` crate which are OWASP-safe for 2026.
//! The hash string is self-describing (algo, params, salt) so no side
//! parameter storage is needed.

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

#[derive(Debug, thiserror::Error)]
pub enum PasswordError {
    #[error("hash failed: {0}")]
    Hash(String),
    #[error("verify failed")]
    Verify,
}

pub fn hash_password(plaintext: &str) -> Result<String, PasswordError> {
    let salt = SaltString::generate(&mut OsRng);
    Argon2::default()
        .hash_password(plaintext.as_bytes(), &salt)
        .map(|h| h.to_string())
        .map_err(|e| PasswordError::Hash(e.to_string()))
}

pub fn verify_password(plaintext: &str, hashed: &str) -> Result<(), PasswordError> {
    let parsed = PasswordHash::new(hashed).map_err(|e| PasswordError::Hash(e.to_string()))?;
    Argon2::default()
        .verify_password(plaintext.as_bytes(), &parsed)
        .map_err(|_| PasswordError::Verify)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip_succeeds() {
        let hash = hash_password("correct horse battery staple").unwrap();
        assert!(verify_password("correct horse battery staple", &hash).is_ok());
    }

    #[test]
    fn wrong_password_rejected() {
        let hash = hash_password("correct").unwrap();
        assert!(verify_password("wrong", &hash).is_err());
    }

    #[test]
    fn distinct_hashes_for_same_input() {
        let a = hash_password("same").unwrap();
        let b = hash_password("same").unwrap();
        assert_ne!(a, b, "salted hashes must differ across calls");
    }
}
