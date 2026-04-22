//! Refresh token storage. Stores only the SHA-256 hash of each token so a
//! database dump is not useful on its own — the plaintext only lives in the
//! client's HttpOnly cookie.

use chrono::{DateTime, Utc};
use sqlx::{PgPool, Row};

#[derive(Debug, Clone)]
pub struct StoredRefreshToken {
    pub id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub expires_at: DateTime<Utc>,
    pub revoked_at: Option<DateTime<Utc>>,
}

pub async fn issue(
    pool: &PgPool,
    user_id: uuid::Uuid,
    token_hash: &str,
    expires_at: DateTime<Utc>,
    user_agent: Option<&str>,
    ip: Option<std::net::IpAddr>,
) -> Result<uuid::Uuid, sqlx::Error> {
    let id = uuid::Uuid::new_v4();
    sqlx::query(
        "INSERT INTO refresh_tokens (id, user_id, token_hash, expires_at, user_agent, ip) \
         VALUES ($1, $2, $3, $4, $5, $6)",
    )
    .bind(id)
    .bind(user_id)
    .bind(token_hash)
    .bind(expires_at)
    .bind(user_agent)
    .bind(ip)
    .execute(pool)
    .await?;
    Ok(id)
}

pub async fn find_active(
    pool: &PgPool,
    token_hash: &str,
) -> Result<Option<StoredRefreshToken>, sqlx::Error> {
    let row = sqlx::query(
        "SELECT id, user_id, expires_at, revoked_at FROM refresh_tokens \
         WHERE token_hash = $1 AND revoked_at IS NULL AND expires_at > NOW()",
    )
    .bind(token_hash)
    .fetch_optional(pool)
    .await?;
    Ok(row.map(|r| StoredRefreshToken {
        id: r.get("id"),
        user_id: r.get("user_id"),
        expires_at: r.get("expires_at"),
        revoked_at: r.try_get("revoked_at").ok(),
    }))
}

/// Mark a refresh token revoked and record which token (if any) replaces it.
pub async fn revoke(
    pool: &PgPool,
    id: uuid::Uuid,
    replaced_by: Option<uuid::Uuid>,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "UPDATE refresh_tokens SET revoked_at = NOW(), replaced_by = $1 \
         WHERE id = $2 AND revoked_at IS NULL",
    )
    .bind(replaced_by)
    .bind(id)
    .execute(pool)
    .await?;
    Ok(())
}

/// Revoke every active refresh token for a user. Used on logout-everywhere
/// and by the admin force-logout action.
pub async fn revoke_all_for_user(pool: &PgPool, user_id: uuid::Uuid) -> Result<u64, sqlx::Error> {
    let result = sqlx::query(
        "UPDATE refresh_tokens SET revoked_at = NOW() \
         WHERE user_id = $1 AND revoked_at IS NULL",
    )
    .bind(user_id)
    .execute(pool)
    .await?;
    Ok(result.rows_affected())
}
