//! Per-institution module + lesson CRUD. Named `module_db` to avoid colliding
//! with Rust's `module` keyword.

use sqlx::{PgPool, Row};

use crate::models::course::{CreateLessonRequest, CreateModuleRequest, Lesson, Module};

const MODULE_COLUMNS: &str =
    "id, course_id, title, summary, position, unlock_rule, created_at, updated_at";
const LESSON_COLUMNS: &str =
    "id, module_id, title, kind, content, position, duration_s, is_required, created_at, updated_at";

fn row_to_module(row: sqlx::postgres::PgRow) -> Module {
    Module {
        id: row.get("id"),
        course_id: row.get("course_id"),
        title: row.get("title"),
        summary: row.try_get("summary").ok(),
        position: row.get("position"),
        unlock_rule: row.get("unlock_rule"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    }
}

fn row_to_lesson(row: sqlx::postgres::PgRow) -> Lesson {
    Lesson {
        id: row.get("id"),
        module_id: row.get("module_id"),
        title: row.get("title"),
        kind: row.get("kind"),
        content: row.get("content"),
        position: row.get("position"),
        duration_s: row.try_get("duration_s").ok(),
        is_required: row.get("is_required"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    }
}

pub async fn create_module(
    pool: &PgPool,
    course_id: uuid::Uuid,
    req: &CreateModuleRequest,
) -> Result<Module, sqlx::Error> {
    let id = uuid::Uuid::new_v4();
    let query = format!(
        "INSERT INTO modules (id, course_id, title, summary, position) \
         VALUES ($1, $2, $3, $4, COALESCE($5, 0)) \
         RETURNING {MODULE_COLUMNS}"
    );
    let row = sqlx::query(&query)
        .bind(id)
        .bind(course_id)
        .bind(&req.title)
        .bind(&req.summary)
        .bind(req.position)
        .fetch_one(pool)
        .await?;
    Ok(row_to_module(row))
}

pub async fn list_modules(
    pool: &PgPool,
    course_id: uuid::Uuid,
) -> Result<Vec<Module>, sqlx::Error> {
    let query = format!(
        "SELECT {MODULE_COLUMNS} FROM modules WHERE course_id = $1 ORDER BY position, created_at"
    );
    let rows = sqlx::query(&query).bind(course_id).fetch_all(pool).await?;
    Ok(rows.into_iter().map(row_to_module).collect())
}

pub async fn create_lesson(
    pool: &PgPool,
    module_id: uuid::Uuid,
    req: &CreateLessonRequest,
) -> Result<Lesson, sqlx::Error> {
    let id = uuid::Uuid::new_v4();
    let query = format!(
        "INSERT INTO lessons \
           (id, module_id, title, kind, content, position, duration_s, is_required) \
         VALUES ($1, $2, $3, $4, $5, COALESCE($6, 0), $7, $8) \
         RETURNING {LESSON_COLUMNS}"
    );
    let row = sqlx::query(&query)
        .bind(id)
        .bind(module_id)
        .bind(&req.title)
        .bind(req.kind.as_str())
        .bind(&req.content)
        .bind(req.position)
        .bind(req.duration_s)
        .bind(req.is_required)
        .fetch_one(pool)
        .await?;
    Ok(row_to_lesson(row))
}

pub async fn list_lessons(
    pool: &PgPool,
    module_id: uuid::Uuid,
) -> Result<Vec<Lesson>, sqlx::Error> {
    let query = format!(
        "SELECT {LESSON_COLUMNS} FROM lessons WHERE module_id = $1 ORDER BY position, created_at"
    );
    let rows = sqlx::query(&query).bind(module_id).fetch_all(pool).await?;
    Ok(rows.into_iter().map(row_to_lesson).collect())
}

pub async fn list_lessons_for_course(
    pool: &PgPool,
    course_id: uuid::Uuid,
) -> Result<Vec<Lesson>, sqlx::Error> {
    let query = format!(
        "SELECT {LESSON_COLUMNS} FROM lessons l \
          JOIN modules m ON m.id = l.module_id \
          WHERE m.course_id = $1 \
          ORDER BY m.position, l.position"
    );
    let rows = sqlx::query(&query).bind(course_id).fetch_all(pool).await?;
    Ok(rows.into_iter().map(row_to_lesson).collect())
}
