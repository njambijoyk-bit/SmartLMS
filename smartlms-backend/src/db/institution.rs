//! Master-DB operations for the `institutions` registry. Uses runtime-checked
//! queries (not the `sqlx::query!` macro) so `cargo check` doesn't depend on
//! a live DB or an offline cache — the schema is validated at runtime on the
//! first query, which is consistent with how the tenant router boots.

use crate::models::institution::{CreateInstitutionRequest, Institution, UpdateInstitutionRequest};
use sqlx::{PgPool, Row};

const COLUMNS: &str =
    "id, slug, name, domain, database_url, license_key, is_active, created_at, updated_at";

fn row_to_institution(row: sqlx::postgres::PgRow) -> Institution {
    Institution {
        id: row.get("id"),
        slug: row.get("slug"),
        name: row.get("name"),
        domain: row.try_get("domain").ok(),
        database_url: row.try_get("database_url").ok(),
        config: None,
        plan_tier: None,
        quotas: None,
        license_key: row.try_get("license_key").ok(),
        is_active: row.get("is_active"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    }
}

pub async fn find_by_slug(pool: &PgPool, slug: &str) -> Result<Option<Institution>, sqlx::Error> {
    let query = format!("SELECT {COLUMNS} FROM institutions WHERE slug = $1 AND is_active = true");
    let row = sqlx::query(&query).bind(slug).fetch_optional(pool).await?;
    Ok(row.map(row_to_institution))
}

pub async fn find_by_domain(
    pool: &PgPool,
    domain: &str,
) -> Result<Option<Institution>, sqlx::Error> {
    let query =
        format!("SELECT {COLUMNS} FROM institutions WHERE domain = $1 AND is_active = true");
    let row = sqlx::query(&query)
        .bind(domain)
        .fetch_optional(pool)
        .await?;
    Ok(row.map(row_to_institution))
}

pub async fn list(
    pool: &PgPool,
    page: i64,
    per_page: i64,
) -> Result<(Vec<Institution>, i64), sqlx::Error> {
    let offset = (page - 1) * per_page;

    let query =
        format!("SELECT {COLUMNS} FROM institutions ORDER BY created_at DESC LIMIT $1 OFFSET $2");
    let rows = sqlx::query(&query)
        .bind(per_page)
        .bind(offset)
        .fetch_all(pool)
        .await?;

    let total: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM institutions")
        .fetch_one(pool)
        .await?;

    let institutions = rows.into_iter().map(row_to_institution).collect();
    Ok((institutions, total))
}

pub async fn create(
    pool: &PgPool,
    req: &CreateInstitutionRequest,
) -> Result<Institution, sqlx::Error> {
    let id = uuid::Uuid::new_v4();
    let now = chrono::Utc::now();

    sqlx::query(
        "INSERT INTO institutions (id, slug, name, domain, is_active, created_at, updated_at) \
         VALUES ($1, $2, $3, $4, true, $5, $6)",
    )
    .bind(id)
    .bind(&req.slug)
    .bind(&req.name)
    .bind(&req.domain)
    .bind(now)
    .bind(now)
    .execute(pool)
    .await?;

    Ok(Institution {
        id,
        slug: req.slug.clone(),
        name: req.name.clone(),
        domain: req.domain.clone(),
        database_url: None,
        config: None,
        plan_tier: None,
        quotas: None,
        license_key: None,
        is_active: true,
        created_at: now,
        updated_at: now,
    })
}

pub async fn update(
    pool: &PgPool,
    id: uuid::Uuid,
    req: &UpdateInstitutionRequest,
) -> Result<Option<Institution>, sqlx::Error> {
    let now = chrono::Utc::now();

    sqlx::query(
        "UPDATE institutions SET \
            name = COALESCE($1, name), \
            domain = COALESCE($2, domain), \
            updated_at = $3 \
         WHERE id = $4",
    )
    .bind(&req.name)
    .bind(&req.domain)
    .bind(now)
    .bind(id)
    .execute(pool)
    .await?;

    let query = format!("SELECT {COLUMNS} FROM institutions WHERE id = $1");
    let row = sqlx::query(&query).bind(id).fetch_optional(pool).await?;

    Ok(row.map(row_to_institution))
}

pub async fn delete(pool: &PgPool, id: uuid::Uuid) -> Result<bool, sqlx::Error> {
    let now = chrono::Utc::now();
    let result =
        sqlx::query("UPDATE institutions SET is_active = false, updated_at = $1 WHERE id = $2")
            .bind(now)
            .bind(id)
            .execute(pool)
            .await?;

    Ok(result.rows_affected() > 0)
}
