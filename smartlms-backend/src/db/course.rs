//! Per-institution course CRUD.

use sqlx::{PgPool, Row};

use crate::models::course::{Course, CreateCourseRequest, UpdateCourseRequest};

const COLUMNS: &str = "id, slug, title, subtitle, description, cover_url, instructor_id, \
                       status, visibility, language, config, published_at, archived_at, \
                       created_at, updated_at";

fn row_to_course(row: sqlx::postgres::PgRow) -> Course {
    Course {
        id: row.get("id"),
        slug: row.get("slug"),
        title: row.get("title"),
        subtitle: row.try_get("subtitle").ok(),
        description: row.try_get("description").ok(),
        cover_url: row.try_get("cover_url").ok(),
        instructor_id: row.get("instructor_id"),
        status: row.get("status"),
        visibility: row.get("visibility"),
        language: row.get("language"),
        config: row.get("config"),
        published_at: row.try_get("published_at").ok(),
        archived_at: row.try_get("archived_at").ok(),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    }
}

pub async fn create(
    pool: &PgPool,
    req: &CreateCourseRequest,
    instructor_id: uuid::Uuid,
) -> Result<Course, sqlx::Error> {
    let id = uuid::Uuid::new_v4();
    let visibility = req.visibility.map(|v| v.as_str()).unwrap_or("private");
    let query = format!(
        "INSERT INTO courses \
           (id, slug, title, subtitle, description, cover_url, instructor_id, language, visibility) \
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9) \
         RETURNING {COLUMNS}"
    );
    let row = sqlx::query(&query)
        .bind(id)
        .bind(&req.slug)
        .bind(&req.title)
        .bind(&req.subtitle)
        .bind(&req.description)
        .bind(&req.cover_url)
        .bind(instructor_id)
        .bind(&req.language)
        .bind(visibility)
        .fetch_one(pool)
        .await?;
    Ok(row_to_course(row))
}

pub async fn find_by_id(pool: &PgPool, id: uuid::Uuid) -> Result<Option<Course>, sqlx::Error> {
    let query = format!("SELECT {COLUMNS} FROM courses WHERE id = $1");
    let row = sqlx::query(&query).bind(id).fetch_optional(pool).await?;
    Ok(row.map(row_to_course))
}

pub async fn find_by_slug(pool: &PgPool, slug: &str) -> Result<Option<Course>, sqlx::Error> {
    let query = format!(
        "SELECT {COLUMNS} FROM courses \
         WHERE lower(slug) = lower($1) AND archived_at IS NULL"
    );
    let row = sqlx::query(&query).bind(slug).fetch_optional(pool).await?;
    Ok(row.map(row_to_course))
}

pub async fn list(
    pool: &PgPool,
    page: i64,
    per_page: i64,
) -> Result<(Vec<Course>, i64), sqlx::Error> {
    let offset = (page - 1) * per_page;
    let query = format!(
        "SELECT {COLUMNS} FROM courses \
         WHERE archived_at IS NULL \
         ORDER BY created_at DESC \
         LIMIT $1 OFFSET $2"
    );
    let rows = sqlx::query(&query)
        .bind(per_page)
        .bind(offset)
        .fetch_all(pool)
        .await?;
    let total: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM courses WHERE archived_at IS NULL")
        .fetch_one(pool)
        .await?;
    Ok((rows.into_iter().map(row_to_course).collect(), total))
}

/// Partial update via COALESCE — nulls in the request leave the column
/// untouched.
pub async fn update(
    pool: &PgPool,
    id: uuid::Uuid,
    req: &UpdateCourseRequest,
) -> Result<Option<Course>, sqlx::Error> {
    let status_code = req.status.map(|s| s.as_str());
    let visibility_code = req.visibility.map(|v| v.as_str());
    let query = format!(
        "UPDATE courses SET \
           title       = COALESCE($2, title), \
           subtitle    = COALESCE($3, subtitle), \
           description = COALESCE($4, description), \
           cover_url   = COALESCE($5, cover_url), \
           status      = COALESCE($6, status), \
           visibility  = COALESCE($7, visibility), \
           language    = COALESCE($8, language), \
           published_at = CASE \
               WHEN $6 = 'published' AND published_at IS NULL THEN NOW() \
               ELSE published_at END, \
           archived_at = CASE \
               WHEN $6 = 'archived' THEN NOW() ELSE archived_at END \
         WHERE id = $1 \
         RETURNING {COLUMNS}"
    );
    let row = sqlx::query(&query)
        .bind(id)
        .bind(&req.title)
        .bind(&req.subtitle)
        .bind(&req.description)
        .bind(&req.cover_url)
        .bind(status_code)
        .bind(visibility_code)
        .bind(&req.language)
        .fetch_optional(pool)
        .await?;
    Ok(row.map(row_to_course))
}

pub async fn archive(pool: &PgPool, id: uuid::Uuid) -> Result<bool, sqlx::Error> {
    let result = sqlx::query(
        "UPDATE courses SET status = 'archived', archived_at = NOW() \
         WHERE id = $1 AND archived_at IS NULL",
    )
    .bind(id)
    .execute(pool)
    .await?;
    Ok(result.rows_affected() > 0)
}
