//! User CRUD + lookup against the per-institution database. Uses
//! runtime-checked queries so `cargo check` works without a live DB.

use crate::models::user::{User, UserRecord};
use sqlx::{PgPool, Row};

const COLUMNS: &str = "id, email, password_hash, first_name, last_name, display_name, phone, \
                       avatar_url, locale, timezone, is_active, is_verified, last_login_at, \
                       failed_login_count, locked_until, deleted_at, created_at, updated_at";

fn row_to_record(row: sqlx::postgres::PgRow) -> UserRecord {
    let user = User {
        id: row.get("id"),
        email: row.get("email"),
        first_name: row.get("first_name"),
        last_name: row.get("last_name"),
        display_name: row.try_get("display_name").ok(),
        phone: row.try_get("phone").ok(),
        avatar_url: row.try_get("avatar_url").ok(),
        locale: row.get("locale"),
        timezone: row.get("timezone"),
        is_active: row.get("is_active"),
        is_verified: row.get("is_verified"),
        last_login_at: row.try_get("last_login_at").ok(),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    };
    UserRecord {
        user,
        password_hash: row.try_get("password_hash").ok(),
        failed_login_count: row.get("failed_login_count"),
        locked_until: row.try_get("locked_until").ok(),
        deleted_at: row.try_get("deleted_at").ok(),
    }
}

pub async fn find_by_email(pool: &PgPool, email: &str) -> Result<Option<UserRecord>, sqlx::Error> {
    let query = format!(
        "SELECT {COLUMNS} FROM users \
         WHERE lower(email) = lower($1) AND deleted_at IS NULL"
    );
    let row = sqlx::query(&query).bind(email).fetch_optional(pool).await?;
    Ok(row.map(row_to_record))
}

pub async fn find_by_id(pool: &PgPool, id: uuid::Uuid) -> Result<Option<UserRecord>, sqlx::Error> {
    let query = format!("SELECT {COLUMNS} FROM users WHERE id = $1 AND deleted_at IS NULL");
    let row = sqlx::query(&query).bind(id).fetch_optional(pool).await?;
    Ok(row.map(row_to_record))
}

/// Insert a new user with a pre-hashed password (may be NULL for SSO-only).
pub async fn create(
    pool: &PgPool,
    email: &str,
    password_hash: Option<&str>,
    first_name: &str,
    last_name: &str,
) -> Result<User, sqlx::Error> {
    let id = uuid::Uuid::new_v4();
    let query = format!(
        "INSERT INTO users (id, email, password_hash, first_name, last_name) \
         VALUES ($1, $2, $3, $4, $5) \
         RETURNING {COLUMNS}"
    );
    let row = sqlx::query(&query)
        .bind(id)
        .bind(email)
        .bind(password_hash)
        .bind(first_name)
        .bind(last_name)
        .fetch_one(pool)
        .await?;
    Ok(row_to_record(row).user)
}

pub async fn record_successful_login(pool: &PgPool, id: uuid::Uuid) -> Result<(), sqlx::Error> {
    sqlx::query(
        "UPDATE users SET last_login_at = NOW(), failed_login_count = 0, locked_until = NULL \
         WHERE id = $1",
    )
    .bind(id)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn record_failed_login(pool: &PgPool, id: uuid::Uuid) -> Result<i32, sqlx::Error> {
    // Bump the counter; lock for 15 minutes after 10 failures per master ref §8.
    let row = sqlx::query(
        "UPDATE users \
         SET failed_login_count = failed_login_count + 1, \
             locked_until = CASE \
                 WHEN failed_login_count + 1 >= 10 THEN NOW() + INTERVAL '15 minutes' \
                 ELSE locked_until \
             END \
         WHERE id = $1 \
         RETURNING failed_login_count",
    )
    .bind(id)
    .fetch_one(pool)
    .await?;
    Ok(row.get::<i32, _>("failed_login_count"))
}

pub async fn soft_delete(pool: &PgPool, id: uuid::Uuid) -> Result<bool, sqlx::Error> {
    let result = sqlx::query(
        "UPDATE users SET deleted_at = NOW(), is_active = false \
         WHERE id = $1 AND deleted_at IS NULL",
    )
    .bind(id)
    .execute(pool)
    .await?;
    Ok(result.rows_affected() > 0)
}

pub async fn email_exists(pool: &PgPool, email: &str) -> Result<bool, sqlx::Error> {
    let count: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM users \
         WHERE lower(email) = lower($1) AND deleted_at IS NULL",
    )
    .bind(email)
    .fetch_one(pool)
    .await?;
    Ok(count > 0)
}
