//! Per-institution enrollment + lesson-progress CRUD.

use sqlx::{PgPool, Row};

use crate::models::course::Enrollment;

const COLUMNS: &str =
    "id, course_id, user_id, status, enrolled_at, completed_at, progress_pct, last_seen_at";

fn row_to_enrollment(row: sqlx::postgres::PgRow) -> Enrollment {
    Enrollment {
        id: row.get("id"),
        course_id: row.get("course_id"),
        user_id: row.get("user_id"),
        status: row.get("status"),
        enrolled_at: row.get("enrolled_at"),
        completed_at: row.try_get("completed_at").ok(),
        progress_pct: row.get("progress_pct"),
        last_seen_at: row.try_get("last_seen_at").ok(),
    }
}

/// Idempotent enrollment. If the row already exists, returns the existing
/// enrollment (unless it was dropped — in which case we re-activate).
pub async fn enroll(
    pool: &PgPool,
    course_id: uuid::Uuid,
    user_id: uuid::Uuid,
) -> Result<Enrollment, sqlx::Error> {
    let id = uuid::Uuid::new_v4();
    let query = format!(
        "INSERT INTO enrollments (id, course_id, user_id) \
         VALUES ($1, $2, $3) \
         ON CONFLICT (user_id, course_id) DO UPDATE SET \
           status       = CASE \
               WHEN enrollments.status = 'dropped' THEN 'active' \
               ELSE enrollments.status END, \
           completed_at = CASE \
               WHEN enrollments.status = 'dropped' THEN NULL \
               ELSE enrollments.completed_at END \
         RETURNING {COLUMNS}"
    );
    let row = sqlx::query(&query)
        .bind(id)
        .bind(course_id)
        .bind(user_id)
        .fetch_one(pool)
        .await?;
    Ok(row_to_enrollment(row))
}

pub async fn find_for(
    pool: &PgPool,
    course_id: uuid::Uuid,
    user_id: uuid::Uuid,
) -> Result<Option<Enrollment>, sqlx::Error> {
    let query = format!("SELECT {COLUMNS} FROM enrollments WHERE course_id = $1 AND user_id = $2");
    let row = sqlx::query(&query)
        .bind(course_id)
        .bind(user_id)
        .fetch_optional(pool)
        .await?;
    Ok(row.map(row_to_enrollment))
}

pub async fn list_for_user(
    pool: &PgPool,
    user_id: uuid::Uuid,
) -> Result<Vec<Enrollment>, sqlx::Error> {
    let query = format!(
        "SELECT {COLUMNS} FROM enrollments \
         WHERE user_id = $1 AND status != 'dropped' \
         ORDER BY enrolled_at DESC"
    );
    let rows = sqlx::query(&query).bind(user_id).fetch_all(pool).await?;
    Ok(rows.into_iter().map(row_to_enrollment).collect())
}

pub async fn list_for_course(
    pool: &PgPool,
    course_id: uuid::Uuid,
    page: i64,
    per_page: i64,
) -> Result<(Vec<Enrollment>, i64), sqlx::Error> {
    let offset = (page - 1) * per_page;
    let query = format!(
        "SELECT {COLUMNS} FROM enrollments \
         WHERE course_id = $1 AND status != 'dropped' \
         ORDER BY enrolled_at DESC \
         LIMIT $2 OFFSET $3"
    );
    let rows = sqlx::query(&query)
        .bind(course_id)
        .bind(per_page)
        .bind(offset)
        .fetch_all(pool)
        .await?;
    let total: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM enrollments WHERE course_id = $1 AND status != 'dropped'",
    )
    .bind(course_id)
    .fetch_one(pool)
    .await?;
    Ok((rows.into_iter().map(row_to_enrollment).collect(), total))
}

pub async fn drop_enrollment(
    pool: &PgPool,
    course_id: uuid::Uuid,
    user_id: uuid::Uuid,
) -> Result<bool, sqlx::Error> {
    let result = sqlx::query(
        "UPDATE enrollments SET status = 'dropped' \
         WHERE course_id = $1 AND user_id = $2 AND status != 'dropped'",
    )
    .bind(course_id)
    .bind(user_id)
    .execute(pool)
    .await?;
    Ok(result.rows_affected() > 0)
}

/// Record a lesson as completed and update the enrollment's progress_pct
/// to the ratio of completed required lessons to total required lessons.
pub async fn complete_lesson(
    pool: &PgPool,
    enrollment_id: uuid::Uuid,
    user_id: uuid::Uuid,
    lesson_id: uuid::Uuid,
) -> Result<i32, sqlx::Error> {
    let mut tx = pool.begin().await?;

    sqlx::query(
        "INSERT INTO lesson_progress \
           (user_id, lesson_id, enrollment_id, status, completed_at) \
         VALUES ($1, $2, $3, 'completed', NOW()) \
         ON CONFLICT (user_id, lesson_id) DO UPDATE SET \
             status = 'completed', \
             completed_at = NOW()",
    )
    .bind(user_id)
    .bind(lesson_id)
    .bind(enrollment_id)
    .execute(&mut *tx)
    .await?;

    // Recalculate progress.
    let pct: i32 = sqlx::query_scalar(
        "WITH total AS ( \
            SELECT COUNT(*)::INT AS n FROM lessons l \
              JOIN modules m ON m.id = l.module_id \
              JOIN enrollments e ON e.course_id = m.course_id \
             WHERE e.id = $1 AND l.is_required = true \
         ), done AS ( \
            SELECT COUNT(*)::INT AS n FROM lesson_progress lp \
              JOIN lessons l ON l.id = lp.lesson_id \
             WHERE lp.enrollment_id = $1 AND lp.status = 'completed' AND l.is_required = true \
         ) \
         SELECT CASE WHEN total.n = 0 THEN 0 \
                     ELSE (done.n * 100 / total.n) END \
           FROM total, done",
    )
    .bind(enrollment_id)
    .fetch_one(&mut *tx)
    .await?;

    let completed = pct >= 100;
    sqlx::query(
        "UPDATE enrollments SET \
           progress_pct = $2, \
           last_seen_at = NOW(), \
           status = CASE WHEN $3 THEN 'completed' ELSE status END, \
           completed_at = CASE WHEN $3 AND completed_at IS NULL THEN NOW() ELSE completed_at END \
         WHERE id = $1",
    )
    .bind(enrollment_id)
    .bind(pct)
    .bind(completed)
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;
    Ok(pct)
}
