//! Auth flows: register / login / refresh / logout.
//!
//! Each flow takes the per-institution `PgPool` from `InstitutionCtx`, never
//! the master pool — so cross-tenant data access is architecturally
//! impossible (master ref §2).

use chrono::{Duration, Utc};
use rand::{rngs::OsRng, RngCore};
use sha2::{Digest, Sha256};
use sqlx::PgPool;

use crate::db;
use crate::models::auth::{LoginRequest, RefreshRequest, RegisterRequest, TokenResponse};
use crate::models::user::{RoleCode, UserWithRoles};
use crate::services::jwt::{self, ACCESS_TOKEN_TTL_MINUTES, REFRESH_TOKEN_TTL_DAYS};
use crate::services::password;

#[derive(Debug, thiserror::Error)]
pub enum AuthError {
    #[error("email already registered")]
    EmailTaken,
    #[error("invalid credentials")]
    InvalidCredentials,
    #[error("account is locked — try again later")]
    AccountLocked,
    #[error("account is disabled")]
    AccountDisabled,
    #[error("refresh token invalid or expired")]
    InvalidRefreshToken,
    #[error("database error: {0}")]
    Db(#[from] sqlx::Error),
    #[error("password error: {0}")]
    Password(#[from] password::PasswordError),
    #[error("jwt error: {0}")]
    Jwt(#[from] jwt::JwtError),
}

/// Context passed from the HTTP layer — the request's IP and User-Agent for
/// the refresh_tokens row. Thin struct so tests can build one without
/// pulling an axum::Request.
#[derive(Debug, Default, Clone)]
pub struct SessionMeta {
    pub user_agent: Option<String>,
    pub ip: Option<std::net::IpAddr>,
}

/// SHA-256 of an opaque refresh token. The plaintext token is returned to
/// the client; only the hash is persisted (master ref §8 layer 3).
fn hash_refresh(token: &str) -> String {
    let mut h = Sha256::new();
    h.update(token.as_bytes());
    hex_encode(&h.finalize())
}

fn generate_refresh_token() -> String {
    let mut buf = [0u8; 32];
    OsRng.fill_bytes(&mut buf);
    hex_encode(&buf)
}

fn hex_encode(bytes: &[u8]) -> String {
    const HEX: &[u8] = b"0123456789abcdef";
    let mut s = String::with_capacity(bytes.len() * 2);
    for b in bytes {
        s.push(HEX[(b >> 4) as usize] as char);
        s.push(HEX[(b & 0x0f) as usize] as char);
    }
    s
}

/// Register a new user in the current institution. First user gets the
/// `admin` role; subsequent registrants default to `learner`. Admins can
/// re-assign roles via the /users admin endpoints (Phase 1 PR #55).
pub async fn register(
    pool: &PgPool,
    institution_id: uuid::Uuid,
    req: RegisterRequest,
    meta: SessionMeta,
) -> Result<TokenResponse, AuthError> {
    if db::user::email_exists(pool, &req.email).await? {
        return Err(AuthError::EmailTaken);
    }

    let password_hash = password::hash_password(&req.password)?;
    let user = db::user::create(
        pool,
        &req.email,
        Some(&password_hash),
        &req.first_name,
        &req.last_name,
    )
    .await?;

    // First user in an institution gets 'admin'; everyone else gets 'learner'.
    let user_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM users WHERE deleted_at IS NULL")
        .fetch_one(pool)
        .await?;
    let initial_role = if user_count == 1 {
        RoleCode::Admin
    } else {
        RoleCode::Learner
    };
    db::role::assign(pool, user.id, initial_role, None).await?;

    issue_token_pair(
        pool,
        institution_id,
        user,
        vec![initial_role.as_str().to_string()],
        meta,
    )
    .await
}

pub async fn login(
    pool: &PgPool,
    institution_id: uuid::Uuid,
    req: LoginRequest,
    meta: SessionMeta,
) -> Result<TokenResponse, AuthError> {
    let record = db::user::find_by_email(pool, &req.email)
        .await?
        .ok_or(AuthError::InvalidCredentials)?;

    if !record.user.is_active {
        return Err(AuthError::AccountDisabled);
    }
    if let Some(locked_until) = record.locked_until {
        if locked_until > Utc::now() {
            return Err(AuthError::AccountLocked);
        }
    }

    let hash = record
        .password_hash
        .as_deref()
        .ok_or(AuthError::InvalidCredentials)?;
    if password::verify_password(&req.password, hash).is_err() {
        db::user::record_failed_login(pool, record.user.id).await?;
        return Err(AuthError::InvalidCredentials);
    }

    db::user::record_successful_login(pool, record.user.id).await?;
    let roles = db::role::roles_for_user(pool, record.user.id).await?;
    issue_token_pair(pool, institution_id, record.user, roles, meta).await
}

pub async fn refresh(
    pool: &PgPool,
    institution_id: uuid::Uuid,
    req: RefreshRequest,
    meta: SessionMeta,
) -> Result<TokenResponse, AuthError> {
    let hash = hash_refresh(&req.refresh_token);
    let stored = db::refresh_token::find_active(pool, &hash)
        .await?
        .ok_or(AuthError::InvalidRefreshToken)?;

    let record = db::user::find_by_id(pool, stored.user_id)
        .await?
        .ok_or(AuthError::InvalidRefreshToken)?;
    if !record.user.is_active {
        return Err(AuthError::AccountDisabled);
    }

    let roles = db::role::roles_for_user(pool, record.user.id).await?;

    // Rotate: issue new pair, revoke old token linked to new one.
    let pair = issue_token_pair(pool, institution_id, record.user, roles, meta).await?;
    // The new refresh token id isn't returned from issue_token_pair; re-look it up.
    let new_hash = hash_refresh(&pair.refresh_token);
    let new_stored = db::refresh_token::find_active(pool, &new_hash).await?;
    db::refresh_token::revoke(pool, stored.id, new_stored.map(|s| s.id)).await?;
    Ok(pair)
}

pub async fn logout(pool: &PgPool, refresh_token: &str) -> Result<(), AuthError> {
    let hash = hash_refresh(refresh_token);
    if let Some(stored) = db::refresh_token::find_active(pool, &hash).await? {
        db::refresh_token::revoke(pool, stored.id, None).await?;
    }
    Ok(())
}

pub async fn logout_everywhere(pool: &PgPool, user_id: uuid::Uuid) -> Result<u64, AuthError> {
    Ok(db::refresh_token::revoke_all_for_user(pool, user_id).await?)
}

async fn issue_token_pair(
    pool: &PgPool,
    institution_id: uuid::Uuid,
    user: crate::models::user::User,
    roles: Vec<String>,
    meta: SessionMeta,
) -> Result<TokenResponse, AuthError> {
    let access =
        jwt::issue_access_token(user.id, institution_id, user.email.clone(), roles.clone())?;
    let refresh = generate_refresh_token();
    let refresh_hash = hash_refresh(&refresh);
    let expires = Utc::now() + Duration::days(REFRESH_TOKEN_TTL_DAYS);
    db::refresh_token::issue(
        pool,
        user.id,
        &refresh_hash,
        expires,
        meta.user_agent.as_deref(),
        meta.ip,
    )
    .await?;

    Ok(TokenResponse {
        access_token: access,
        refresh_token: refresh,
        token_type: "Bearer",
        expires_in: ACCESS_TOKEN_TTL_MINUTES * 60,
        user: UserWithRoles { user, roles },
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hashes_are_stable_and_hex() {
        let h1 = hash_refresh("abc");
        let h2 = hash_refresh("abc");
        assert_eq!(h1, h2);
        assert_eq!(h1.len(), 64);
        assert!(h1.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn generated_tokens_differ() {
        let a = generate_refresh_token();
        let b = generate_refresh_token();
        assert_ne!(a, b);
        assert_eq!(a.len(), 64);
    }
}
