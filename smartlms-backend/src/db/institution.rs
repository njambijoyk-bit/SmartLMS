// Database operations for institutions (master database)
use crate::models::institution::{Institution, CreateInstitutionRequest, UpdateInstitutionRequest};
use sqlx::{PgPool, Row};

/// Find institution by slug in master database
pub async fn find_by_slug(pool: &PgPool, slug: &str) -> Result<Option<Institution>, sqlx::Error> {
    let row = sqlx::query!(
        r#"
        SELECT id, slug, name, domain, database_url, config, plan_tier, 
               quotas, license_key, is_active, created_at, updated_at
        FROM institutions 
        WHERE slug = $1 AND is_active = true
        "#,
        slug
    )
    .fetch_optional(pool)
    .await?;

    Ok(row.map(|r| Institution {
        id: r.id,
        slug: r.slug,
        name: r.name,
        domain: r.domain,
        database_url: r.database_url,
        config: None, // Parse from JSON if needed
        plan_tier: None,
        quotas: None,
        license_key: r.license_key,
        is_active: r.is_active,
        created_at: r.created_at,
        updated_at: r.updated_at,
    }))
}

/// Find institution by custom domain
pub async fn find_by_domain(pool: &PgPool, domain: &str) -> Result<Option<Institution>, sqlx::Error> {
    let row = sqlx::query!(
        r#"
        SELECT id, slug, name, domain, database_url, config, plan_tier, 
               quotas, license_key, is_active, created_at, updated_at
        FROM institutions 
        WHERE domain = $1 AND is_active = true
        "#,
        domain
    )
    .fetch_optional(pool)
    .await?;

    Ok(row.map(|r| Institution {
        id: r.id,
        slug: r.slug,
        name: r.name,
        domain: r.domain,
        database_url: r.database_url,
        config: None,
        plan_tier: None,
        quotas: None,
        license_key: r.license_key,
        is_active: r.is_active,
        created_at: r.created_at,
        updated_at: r.updated_at,
    }))
}

/// List all institutions with pagination
pub async fn list(pool: &PgPool, page: i64, per_page: i64) -> Result<(Vec<Institution>, i64), sqlx::Error> {
    let offset = (page - 1) * per_page;
    
    let rows = sqlx::query!(
        r#"
        SELECT id, slug, name, domain, database_url, config, plan_tier, 
               quotas, license_key, is_active, created_at, updated_at
        FROM institutions 
        ORDER BY created_at DESC
        LIMIT $1 OFFSET $2
        "#,
        per_page,
        offset
    )
    .fetch_all(pool)
    .await?;

    let total: i64 = sqlx::query!("SELECT COUNT(*) as count FROM institutions")
        .fetch_one(pool)
        .await?
        .count;

    let institutions = rows.into_iter().map(|r| Institution {
        id: r.id,
        slug: r.slug,
        name: r.name,
        domain: r.domain,
        database_url: r.database_url,
        config: None,
        plan_tier: None,
        quotas: None,
        license_key: r.license_key,
        is_active: r.is_active,
        created_at: r.created_at,
        updated_at: r.updated_at,
    }).collect();

    Ok((institutions, total))
}

/// Create new institution
pub async fn create(pool: &PgPool, req: &CreateInstitutionRequest) -> Result<Institution, sqlx::Error> {
    let id = uuid::Uuid::new_v4();
    let now = chrono::Utc::now();

    sqlx::query!(
        r#"
        INSERT INTO institutions (id, slug, name, domain, is_active, created_at, updated_at)
        VALUES ($1, $2, $3, $4, true, $5, $6)
        "#,
        id,
        req.slug,
        req.name,
        req.domain,
        now,
        now
    )
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

/// Update institution
pub async fn update(pool: &PgPool, id: uuid::Uuid, req: &UpdateInstitutionRequest) -> Result<Option<Institution>, sqlx::Error> {
    let now = chrono::Utc::now();
    
    sqlx::query!(
        r#"
        UPDATE institutions 
        SET name = COALESCE($1, name),
            domain = COALESCE($2, domain),
            updated_at = $3
        WHERE id = $4
        "#,
        req.name,
        req.domain,
        now,
        id
    )
    .execute(pool)
    .await?;

    // Fetch updated record
    let row = sqlx::query!(
        r#"
        SELECT id, slug, name, domain, database_url, config, plan_tier, 
               quotas, license_key, is_active, created_at, updated_at
        FROM institutions WHERE id = $1
        "#,
        id
    )
    .fetch_optional(pool)
    .await?;

    Ok(row.map(|r| Institution {
        id: r.id,
        slug: r.slug,
        name: r.name,
        domain: r.domain,
        database_url: r.database_url,
        config: None,
        plan_tier: None,
        quotas: None,
        license_key: r.license_key,
        is_active: r.is_active,
        created_at: r.created_at,
        updated_at: r.updated_at,
    }))
}

/// Delete (deactivate) institution
pub async fn delete(pool: &PgPool, id: uuid::Uuid) -> Result<bool, sqlx::Error> {
    let now = chrono::Utc::now();
    
    let rows = sqlx::query!(
        "UPDATE institutions SET is_active = false, updated_at = $1 WHERE id = $2",
        now,
        id
    )
    .execute(pool)
    .await?;

    Ok(rows.rows_affected() > 0)
}