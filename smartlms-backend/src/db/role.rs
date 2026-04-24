//! Role assignments for a user in the current institution.

use crate::models::user::RoleCode;
use sqlx::{PgPool, Row};

pub async fn roles_for_user(
    pool: &PgPool,
    user_id: uuid::Uuid,
) -> Result<Vec<String>, sqlx::Error> {
    let rows = sqlx::query("SELECT role_code FROM user_roles WHERE user_id = $1")
        .bind(user_id)
        .fetch_all(pool)
        .await?;
    Ok(rows
        .into_iter()
        .map(|r| r.get::<String, _>("role_code"))
        .collect())
}

pub async fn assign(
    pool: &PgPool,
    user_id: uuid::Uuid,
    role: RoleCode,
    granted_by: Option<uuid::Uuid>,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO user_roles (user_id, role_code, granted_by) \
         VALUES ($1, $2, $3) \
         ON CONFLICT (user_id, role_code) DO NOTHING",
    )
    .bind(user_id)
    .bind(role.as_str())
    .bind(granted_by)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn revoke(
    pool: &PgPool,
    user_id: uuid::Uuid,
    role: RoleCode,
) -> Result<bool, sqlx::Error> {
    let result = sqlx::query("DELETE FROM user_roles WHERE user_id = $1 AND role_code = $2")
        .bind(user_id)
        .bind(role.as_str())
        .execute(pool)
        .await?;
    Ok(result.rows_affected() > 0)
}

pub async fn user_has_role(
    pool: &PgPool,
    user_id: uuid::Uuid,
    role: RoleCode,
) -> Result<bool, sqlx::Error> {
    let count: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM user_roles WHERE user_id = $1 AND role_code = $2")
            .bind(user_id)
            .bind(role.as_str())
            .fetch_one(pool)
            .await?;
    Ok(count > 0)
}
