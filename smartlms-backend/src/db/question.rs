//! Per-institution question-bank CRUD.

use rust_decimal::Decimal;
use sqlx::{PgPool, Row};

use crate::models::assessment::{CreateQuestionRequest, Question};

const COLUMNS: &str = "id, author_id, kind, stem, body, answer, default_points, \
                       explanation, tags, created_at, updated_at";

fn row_to_question(row: sqlx::postgres::PgRow) -> Question {
    Question {
        id: row.get("id"),
        author_id: row.get("author_id"),
        kind: row.get("kind"),
        stem: row.get("stem"),
        body: row.get("body"),
        answer: row.get("answer"),
        default_points: row.get::<Decimal, _>("default_points"),
        explanation: row.try_get("explanation").ok(),
        tags: row.get("tags"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    }
}

pub async fn create(
    pool: &PgPool,
    author_id: uuid::Uuid,
    req: &CreateQuestionRequest,
) -> Result<Question, sqlx::Error> {
    let id = uuid::Uuid::new_v4();
    let query = format!(
        "INSERT INTO questions \
           (id, author_id, kind, stem, body, answer, default_points, explanation, tags) \
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9) \
         RETURNING {COLUMNS}"
    );
    let row = sqlx::query(&query)
        .bind(id)
        .bind(author_id)
        .bind(req.kind.as_str())
        .bind(&req.stem)
        .bind(&req.body)
        .bind(&req.answer)
        .bind(req.default_points)
        .bind(&req.explanation)
        .bind(&req.tags)
        .fetch_one(pool)
        .await?;
    Ok(row_to_question(row))
}

pub async fn find_by_id(pool: &PgPool, id: uuid::Uuid) -> Result<Option<Question>, sqlx::Error> {
    let query = format!("SELECT {COLUMNS} FROM questions WHERE id = $1");
    let row = sqlx::query(&query).bind(id).fetch_optional(pool).await?;
    Ok(row.map(row_to_question))
}

pub async fn find_many(pool: &PgPool, ids: &[uuid::Uuid]) -> Result<Vec<Question>, sqlx::Error> {
    if ids.is_empty() {
        return Ok(vec![]);
    }
    let query = format!("SELECT {COLUMNS} FROM questions WHERE id = ANY($1)");
    let rows = sqlx::query(&query).bind(ids).fetch_all(pool).await?;
    Ok(rows.into_iter().map(row_to_question).collect())
}

pub async fn list_by_author(
    pool: &PgPool,
    author_id: uuid::Uuid,
    page: i64,
    per_page: i64,
) -> Result<(Vec<Question>, i64), sqlx::Error> {
    let offset = (page - 1) * per_page;
    let query = format!(
        "SELECT {COLUMNS} FROM questions \
         WHERE author_id = $1 ORDER BY created_at DESC LIMIT $2 OFFSET $3"
    );
    let rows = sqlx::query(&query)
        .bind(author_id)
        .bind(per_page)
        .bind(offset)
        .fetch_all(pool)
        .await?;
    let total: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM questions WHERE author_id = $1")
        .bind(author_id)
        .fetch_one(pool)
        .await?;
    Ok((rows.into_iter().map(row_to_question).collect(), total))
}
