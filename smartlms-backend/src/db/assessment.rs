//! Per-institution assessment + attempt CRUD.

use rust_decimal::Decimal;
use sqlx::{PgPool, Row};

use crate::models::assessment::{
    AddQuestionToAssessmentRequest, Assessment, Attempt, AttemptAnswer, CreateAssessmentRequest,
    UpdateAssessmentRequest,
};

const ASSESSMENT_COLUMNS: &str =
    "id, course_id, title, description, kind, status, time_limit_minutes, max_attempts, \
     passing_score_pct, shuffle_questions, show_results_policy, available_from, available_until, \
     created_at, updated_at";

const ATTEMPT_COLUMNS: &str =
    "id, assessment_id, user_id, state, started_at, submitted_at, due_at, \
     score_points, max_points, score_pct, passed, requires_manual, attempt_no";

fn row_to_assessment(row: sqlx::postgres::PgRow) -> Assessment {
    Assessment {
        id: row.get("id"),
        course_id: row.get("course_id"),
        title: row.get("title"),
        description: row.try_get("description").ok(),
        kind: row.get("kind"),
        status: row.get("status"),
        time_limit_minutes: row.try_get("time_limit_minutes").ok(),
        max_attempts: row.try_get("max_attempts").ok(),
        passing_score_pct: row.get::<Decimal, _>("passing_score_pct"),
        shuffle_questions: row.get("shuffle_questions"),
        show_results_policy: row.get("show_results_policy"),
        available_from: row.try_get("available_from").ok(),
        available_until: row.try_get("available_until").ok(),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    }
}

fn row_to_attempt(row: sqlx::postgres::PgRow) -> Attempt {
    Attempt {
        id: row.get("id"),
        assessment_id: row.get("assessment_id"),
        user_id: row.get("user_id"),
        state: row.get("state"),
        started_at: row.get("started_at"),
        submitted_at: row.try_get("submitted_at").ok(),
        due_at: row.try_get("due_at").ok(),
        score_points: row.get::<Decimal, _>("score_points"),
        max_points: row.get::<Decimal, _>("max_points"),
        score_pct: row.try_get::<Decimal, _>("score_pct").ok(),
        passed: row.try_get("passed").ok(),
        requires_manual: row.get("requires_manual"),
        attempt_no: row.get("attempt_no"),
    }
}

// ---------------------------------------------------------------------------
// Assessments
// ---------------------------------------------------------------------------

pub async fn create(
    pool: &PgPool,
    course_id: uuid::Uuid,
    req: &CreateAssessmentRequest,
) -> Result<Assessment, sqlx::Error> {
    let id = uuid::Uuid::new_v4();
    let query = format!(
        "INSERT INTO assessments \
           (id, course_id, title, description, kind, time_limit_minutes, \
            max_attempts, passing_score_pct, shuffle_questions) \
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9) \
         RETURNING {ASSESSMENT_COLUMNS}"
    );
    let row = sqlx::query(&query)
        .bind(id)
        .bind(course_id)
        .bind(&req.title)
        .bind(&req.description)
        .bind(req.kind.as_str())
        .bind(req.time_limit_minutes)
        .bind(req.max_attempts)
        .bind(req.passing_score_pct)
        .bind(req.shuffle_questions)
        .fetch_one(pool)
        .await?;
    Ok(row_to_assessment(row))
}

pub async fn find_by_id(pool: &PgPool, id: uuid::Uuid) -> Result<Option<Assessment>, sqlx::Error> {
    let query = format!("SELECT {ASSESSMENT_COLUMNS} FROM assessments WHERE id = $1");
    let row = sqlx::query(&query).bind(id).fetch_optional(pool).await?;
    Ok(row.map(row_to_assessment))
}

pub async fn list_for_course(
    pool: &PgPool,
    course_id: uuid::Uuid,
) -> Result<Vec<Assessment>, sqlx::Error> {
    let query = format!(
        "SELECT {ASSESSMENT_COLUMNS} FROM assessments \
         WHERE course_id = $1 ORDER BY created_at DESC"
    );
    let rows = sqlx::query(&query).bind(course_id).fetch_all(pool).await?;
    Ok(rows.into_iter().map(row_to_assessment).collect())
}

pub async fn update(
    pool: &PgPool,
    id: uuid::Uuid,
    req: &UpdateAssessmentRequest,
) -> Result<Option<Assessment>, sqlx::Error> {
    let status_code = req.status.map(|s| s.as_str());
    let query = format!(
        "UPDATE assessments SET \
           title               = COALESCE($2, title), \
           description         = COALESCE($3, description), \
           status              = COALESCE($4, status), \
           time_limit_minutes  = COALESCE($5, time_limit_minutes), \
           max_attempts        = COALESCE($6, max_attempts), \
           passing_score_pct   = COALESCE($7, passing_score_pct), \
           shuffle_questions   = COALESCE($8, shuffle_questions) \
         WHERE id = $1 \
         RETURNING {ASSESSMENT_COLUMNS}"
    );
    let row = sqlx::query(&query)
        .bind(id)
        .bind(&req.title)
        .bind(&req.description)
        .bind(status_code)
        .bind(req.time_limit_minutes)
        .bind(req.max_attempts)
        .bind(req.passing_score_pct)
        .bind(req.shuffle_questions)
        .fetch_optional(pool)
        .await?;
    Ok(row.map(row_to_assessment))
}

// ---------------------------------------------------------------------------
// Assessment ↔ question
// ---------------------------------------------------------------------------

pub async fn add_question(
    pool: &PgPool,
    assessment_id: uuid::Uuid,
    req: &AddQuestionToAssessmentRequest,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO assessment_questions \
           (assessment_id, question_id, position, points_override) \
         VALUES ($1, $2, COALESCE($3, 0), $4) \
         ON CONFLICT (assessment_id, question_id) DO UPDATE SET \
           position = EXCLUDED.position, \
           points_override = EXCLUDED.points_override",
    )
    .bind(assessment_id)
    .bind(req.question_id)
    .bind(req.position)
    .bind(req.points_override)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn remove_question(
    pool: &PgPool,
    assessment_id: uuid::Uuid,
    question_id: uuid::Uuid,
) -> Result<bool, sqlx::Error> {
    let result = sqlx::query(
        "DELETE FROM assessment_questions WHERE assessment_id = $1 AND question_id = $2",
    )
    .bind(assessment_id)
    .bind(question_id)
    .execute(pool)
    .await?;
    Ok(result.rows_affected() > 0)
}

#[derive(Debug, Clone)]
pub struct AssessmentQuestionRef {
    pub question_id: uuid::Uuid,
    pub position: i32,
    pub points_override: Option<Decimal>,
}

pub async fn list_questions(
    pool: &PgPool,
    assessment_id: uuid::Uuid,
) -> Result<Vec<AssessmentQuestionRef>, sqlx::Error> {
    let rows = sqlx::query(
        "SELECT question_id, position, points_override FROM assessment_questions \
         WHERE assessment_id = $1 ORDER BY position, question_id",
    )
    .bind(assessment_id)
    .fetch_all(pool)
    .await?;
    Ok(rows
        .into_iter()
        .map(|r| AssessmentQuestionRef {
            question_id: r.get("question_id"),
            position: r.get("position"),
            points_override: r.try_get::<Decimal, _>("points_override").ok(),
        })
        .collect())
}

// ---------------------------------------------------------------------------
// Attempts
// ---------------------------------------------------------------------------

pub async fn start_attempt(
    pool: &PgPool,
    assessment_id: uuid::Uuid,
    user_id: uuid::Uuid,
    due_at: Option<chrono::DateTime<chrono::Utc>>,
    attempt_no: i32,
) -> Result<Attempt, sqlx::Error> {
    let id = uuid::Uuid::new_v4();
    let query = format!(
        "INSERT INTO attempts (id, assessment_id, user_id, due_at, attempt_no) \
         VALUES ($1, $2, $3, $4, $5) \
         RETURNING {ATTEMPT_COLUMNS}"
    );
    let row = sqlx::query(&query)
        .bind(id)
        .bind(assessment_id)
        .bind(user_id)
        .bind(due_at)
        .bind(attempt_no)
        .fetch_one(pool)
        .await?;
    Ok(row_to_attempt(row))
}

pub async fn find_attempt(pool: &PgPool, id: uuid::Uuid) -> Result<Option<Attempt>, sqlx::Error> {
    let query = format!("SELECT {ATTEMPT_COLUMNS} FROM attempts WHERE id = $1");
    let row = sqlx::query(&query).bind(id).fetch_optional(pool).await?;
    Ok(row.map(row_to_attempt))
}

pub async fn find_open_attempt(
    pool: &PgPool,
    assessment_id: uuid::Uuid,
    user_id: uuid::Uuid,
) -> Result<Option<Attempt>, sqlx::Error> {
    let query = format!(
        "SELECT {ATTEMPT_COLUMNS} FROM attempts \
         WHERE assessment_id = $1 AND user_id = $2 AND state = 'in_progress' \
         ORDER BY started_at DESC LIMIT 1"
    );
    let row = sqlx::query(&query)
        .bind(assessment_id)
        .bind(user_id)
        .fetch_optional(pool)
        .await?;
    Ok(row.map(row_to_attempt))
}

pub async fn count_attempts(
    pool: &PgPool,
    assessment_id: uuid::Uuid,
    user_id: uuid::Uuid,
) -> Result<i64, sqlx::Error> {
    let count: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM attempts WHERE assessment_id = $1 AND user_id = $2",
    )
    .bind(assessment_id)
    .bind(user_id)
    .fetch_one(pool)
    .await?;
    Ok(count)
}

pub async fn list_attempts_for_user(
    pool: &PgPool,
    user_id: uuid::Uuid,
) -> Result<Vec<Attempt>, sqlx::Error> {
    let query = format!(
        "SELECT {ATTEMPT_COLUMNS} FROM attempts WHERE user_id = $1 ORDER BY started_at DESC"
    );
    let rows = sqlx::query(&query).bind(user_id).fetch_all(pool).await?;
    Ok(rows.into_iter().map(row_to_attempt).collect())
}

// ---------------------------------------------------------------------------
// Attempt answers
// ---------------------------------------------------------------------------

pub struct GradedAnswer {
    pub question_id: uuid::Uuid,
    pub response: serde_json::Value,
    pub is_correct: Option<bool>,
    pub points_earned: Decimal,
    pub graded_by: &'static str,
    pub feedback: Option<String>,
}

/// Final tallies + state transition for `finalize_attempt`.
pub struct AttemptFinalTotals {
    pub max_points: Decimal,
    pub score_points: Decimal,
    pub score_pct: Option<Decimal>,
    pub passed: Option<bool>,
    pub requires_manual: bool,
}

/// Persist graded answers and update the attempt totals in one transaction.
pub async fn finalize_attempt(
    pool: &PgPool,
    attempt_id: uuid::Uuid,
    answers: &[GradedAnswer],
    totals: AttemptFinalTotals,
) -> Result<(), sqlx::Error> {
    let AttemptFinalTotals {
        max_points,
        score_points,
        score_pct,
        passed,
        requires_manual,
    } = totals;
    let mut tx = pool.begin().await?;

    for ans in answers {
        sqlx::query(
            "INSERT INTO attempt_answers \
               (attempt_id, question_id, response, is_correct, points_earned, graded_by, graded_at, feedback) \
             VALUES ($1, $2, $3, $4, $5, $6, \
                     CASE WHEN $6 = 'pending' THEN NULL ELSE NOW() END, $7) \
             ON CONFLICT (attempt_id, question_id) DO UPDATE SET \
               response      = EXCLUDED.response, \
               is_correct    = EXCLUDED.is_correct, \
               points_earned = EXCLUDED.points_earned, \
               graded_by     = EXCLUDED.graded_by, \
               graded_at     = EXCLUDED.graded_at, \
               feedback      = EXCLUDED.feedback",
        )
        .bind(attempt_id)
        .bind(ans.question_id)
        .bind(&ans.response)
        .bind(ans.is_correct)
        .bind(ans.points_earned)
        .bind(ans.graded_by)
        .bind(&ans.feedback)
        .execute(&mut *tx)
        .await?;
    }

    let new_state = if requires_manual {
        "submitted"
    } else {
        "graded"
    };
    sqlx::query(
        "UPDATE attempts SET \
           state           = $2, \
           submitted_at    = COALESCE(submitted_at, NOW()), \
           score_points    = $3, \
           max_points      = $4, \
           score_pct       = $5, \
           passed          = $6, \
           requires_manual = $7 \
         WHERE id = $1",
    )
    .bind(attempt_id)
    .bind(new_state)
    .bind(score_points)
    .bind(max_points)
    .bind(score_pct)
    .bind(passed)
    .bind(requires_manual)
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;
    Ok(())
}

pub async fn list_answers(
    pool: &PgPool,
    attempt_id: uuid::Uuid,
) -> Result<Vec<AttemptAnswer>, sqlx::Error> {
    let rows = sqlx::query(
        "SELECT attempt_id, question_id, response, is_correct, points_earned, graded_by, feedback \
         FROM attempt_answers WHERE attempt_id = $1",
    )
    .bind(attempt_id)
    .fetch_all(pool)
    .await?;
    Ok(rows
        .into_iter()
        .map(|r| AttemptAnswer {
            attempt_id: r.get("attempt_id"),
            question_id: r.get("question_id"),
            response: r.get("response"),
            is_correct: r.try_get("is_correct").ok(),
            points_earned: r.get::<Decimal, _>("points_earned"),
            graded_by: r.get("graded_by"),
            feedback: r.try_get("feedback").ok(),
        })
        .collect())
}
