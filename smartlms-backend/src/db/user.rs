// Database operations for users (per-institution database)
use crate::models::user::User;
use sqlx::{PgPool, Row};

/// Find user by email in institution's database
pub async fn find_by_email(pool: &PgPool, email: &str) -> Result<Option<User>, sqlx::Error> {
    let row = sqlx::query!(
        "SELECT id, email, password_hash, first_name, last_name, role FROM users WHERE email = $1",
        email
    )
    .fetch_optional(pool)
    .await?;

    Ok(row.map(|r| User {
        id: r.id,
        email: r.email,
        password_hash: r.password_hash,
        first_name: r.first_name,
        last_name: r.last_name,
        role: r.role,
    }))
}

/// Find user by ID
pub async fn find_by_id(pool: &PgPool, id: uuid::Uuid) -> Result<Option<User>, sqlx::Error> {
    let row = sqlx::query!(
        "SELECT id, email, password_hash, first_name, last_name, role FROM users WHERE id = $1",
        id
    )
    .fetch_optional(pool)
    .await?;

    Ok(row.map(|r| User {
        id: r.id,
        email: r.email,
        password_hash: r.password_hash,
        first_name: r.first_name,
        last_name: r.last_name,
        role: r.role,
    }))
}

/// Create new user
pub async fn create(
    pool: &PgPool,
    email: &str,
    password_hash: &str,
    first_name: &str,
    last_name: &str,
    role: &str,
) -> Result<User, sqlx::Error> {
    let id = uuid::Uuid::new_v4();

    sqlx::query!(
        "INSERT INTO users (id, email, password_hash, first_name, last_name, role) VALUES ($1, $2, $3, $4, $5, $6)",
        id,
        email,
        password_hash,
        first_name,
        last_name,
        role
    )
    .execute(pool)
    .await?;

    Ok(User {
        id,
        email: email.to_string(),
        password_hash: password_hash.to_string(),
        first_name: first_name.to_string(),
        last_name: last_name.to_string(),
        role: role.to_string(),
    })
}

/// Update user password
pub async fn update_password(
    pool: &PgPool,
    user_id: uuid::Uuid,
    new_hash: &str,
) -> Result<bool, sqlx::Error> {
    let rows = sqlx::query!(
        "UPDATE users SET password_hash = $1 WHERE id = $2",
        new_hash,
        user_id
    )
    .execute(pool)
    .await?;

    Ok(rows.rows_affected() > 0)
}

/// List users with pagination
pub async fn list(
    pool: &PgPool,
    page: i64,
    per_page: i64,
) -> Result<(Vec<User>, i64), sqlx::Error> {
    let offset = (page - 1) * per_page;

    let rows = sqlx::query!(
        "SELECT id, email, password_hash, first_name, last_name, role FROM users ORDER BY created_at DESC LIMIT $1 OFFSET $2",
        per_page,
        offset
    )
    .fetch_all(pool)
    .await?;

    let total: i64 = sqlx::query!("SELECT COUNT(*) as count FROM users")
        .fetch_one(pool)
        .await?
        .count;

    let users = rows
        .into_iter()
        .map(|r| User {
            id: r.id,
            email: r.email,
            password_hash: r.password_hash,
            first_name: r.first_name,
            last_name: r.last_name,
            role: r.role,
        })
        .collect();

    Ok((users, total))
}

/// Update user
pub async fn update(
    pool: &PgPool,
    id: uuid::Uuid,
    first_name: Option<&str>,
    last_name: Option<&str>,
    role: Option<&str>,
) -> Result<Option<User>, sqlx::Error> {
    sqlx::query!(
        "UPDATE users SET first_name = COALESCE($1, first_name), last_name = COALESCE($2, last_name), role = COALESCE($3, role) WHERE id = $4",
        first_name,
        last_name,
        role,
        id
    )
    .execute(pool)
    .await?;

    find_by_id(pool, id).await
}

/// Delete user
pub async fn delete(pool: &PgPool, id: uuid::Uuid) -> Result<bool, sqlx::Error> {
    let rows = sqlx::query!("DELETE FROM users WHERE id = $1", id)
        .execute(pool)
        .await?;

    Ok(rows.rows_affected() > 0)
}
