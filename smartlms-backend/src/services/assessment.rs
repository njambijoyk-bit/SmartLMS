//! Assessment service layer.
//!
//! Orchestrates the question bank, assessment CRUD, and attempt lifecycle
//! (start → submit → auto-grade or park for manual). Enforces RBAC rules
//! on top of `db::assessment` + `db::question`.

use std::collections::HashMap;

use rust_decimal::Decimal;
use sqlx::PgPool;

use crate::db;
use crate::middleware::auth::AuthUser;
use crate::models::assessment::{
    AddQuestionToAssessmentRequest, AnswerInput, Assessment, AssessmentStatus, Attempt,
    AttemptAnswer, CreateAssessmentRequest, CreateQuestionRequest, Question, QuestionKind,
    UpdateAssessmentRequest,
};
use crate::models::user::RoleCode;
use crate::services::grading;

#[derive(Debug, thiserror::Error)]
pub enum AssessmentError {
    #[error("permission denied")]
    Forbidden,
    #[error("not found")]
    NotFound,
    #[error("course is not accessible")]
    CourseUnavailable,
    #[error("assessment is not published")]
    NotPublished,
    #[error("attempt limit reached")]
    NoAttemptsLeft,
    #[error("no open attempt to submit")]
    NoOpenAttempt,
    #[error("not enrolled in the parent course")]
    NotEnrolled,
    #[error("database error: {0}")]
    Db(#[from] sqlx::Error),
}

fn require_author(user: &AuthUser) -> Result<(), AssessmentError> {
    if user.is_admin() || user.has_role(RoleCode::Instructor) {
        Ok(())
    } else {
        Err(AssessmentError::Forbidden)
    }
}

async fn ensure_course_staff(
    pool: &PgPool,
    user: &AuthUser,
    course_id: uuid::Uuid,
) -> Result<(), AssessmentError> {
    let course = db::course::find_by_id(pool, course_id)
        .await?
        .ok_or(AssessmentError::CourseUnavailable)?;
    if user.is_admin() || course.instructor_id == user.id {
        Ok(())
    } else {
        Err(AssessmentError::Forbidden)
    }
}

// ---------------------------------------------------------------------------
// Questions
// ---------------------------------------------------------------------------

pub async fn create_question(
    pool: &PgPool,
    user: &AuthUser,
    req: CreateQuestionRequest,
) -> Result<Question, AssessmentError> {
    require_author(user)?;
    Ok(db::question::create(pool, user.id, &req).await?)
}

pub async fn list_my_questions(
    pool: &PgPool,
    user: &AuthUser,
    page: i64,
    per_page: i64,
) -> Result<(Vec<Question>, i64), AssessmentError> {
    require_author(user)?;
    Ok(db::question::list_by_author(pool, user.id, page, per_page).await?)
}

// ---------------------------------------------------------------------------
// Assessments
// ---------------------------------------------------------------------------

pub async fn create_assessment(
    pool: &PgPool,
    user: &AuthUser,
    course_id: uuid::Uuid,
    req: CreateAssessmentRequest,
) -> Result<Assessment, AssessmentError> {
    ensure_course_staff(pool, user, course_id).await?;
    Ok(db::assessment::create(pool, course_id, &req).await?)
}

pub async fn list_for_course(
    pool: &PgPool,
    user: &AuthUser,
    course_id: uuid::Uuid,
) -> Result<Vec<Assessment>, AssessmentError> {
    let course = db::course::find_by_id(pool, course_id)
        .await?
        .ok_or(AssessmentError::CourseUnavailable)?;
    let is_staff = user.is_admin() || course.instructor_id == user.id;
    let mut items = db::assessment::list_for_course(pool, course_id).await?;
    if !is_staff {
        items.retain(|a| a.status == AssessmentStatus::Published.as_str());
    }
    Ok(items)
}

pub async fn update_assessment(
    pool: &PgPool,
    user: &AuthUser,
    id: uuid::Uuid,
    req: UpdateAssessmentRequest,
) -> Result<Assessment, AssessmentError> {
    let assessment = db::assessment::find_by_id(pool, id)
        .await?
        .ok_or(AssessmentError::NotFound)?;
    ensure_course_staff(pool, user, assessment.course_id).await?;
    let updated = db::assessment::update(pool, id, &req)
        .await?
        .ok_or(AssessmentError::NotFound)?;
    Ok(updated)
}

pub async fn add_question_to_assessment(
    pool: &PgPool,
    user: &AuthUser,
    assessment_id: uuid::Uuid,
    req: AddQuestionToAssessmentRequest,
) -> Result<(), AssessmentError> {
    let assessment = db::assessment::find_by_id(pool, assessment_id)
        .await?
        .ok_or(AssessmentError::NotFound)?;
    ensure_course_staff(pool, user, assessment.course_id).await?;
    db::assessment::add_question(pool, assessment_id, &req).await?;
    Ok(())
}

pub async fn remove_question_from_assessment(
    pool: &PgPool,
    user: &AuthUser,
    assessment_id: uuid::Uuid,
    question_id: uuid::Uuid,
) -> Result<(), AssessmentError> {
    let assessment = db::assessment::find_by_id(pool, assessment_id)
        .await?
        .ok_or(AssessmentError::NotFound)?;
    ensure_course_staff(pool, user, assessment.course_id).await?;
    if !db::assessment::remove_question(pool, assessment_id, question_id).await? {
        return Err(AssessmentError::NotFound);
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// Attempts
// ---------------------------------------------------------------------------

pub async fn start_attempt(
    pool: &PgPool,
    user: &AuthUser,
    assessment_id: uuid::Uuid,
) -> Result<Attempt, AssessmentError> {
    let assessment = db::assessment::find_by_id(pool, assessment_id)
        .await?
        .ok_or(AssessmentError::NotFound)?;

    if assessment.status != AssessmentStatus::Published.as_str() {
        return Err(AssessmentError::NotPublished);
    }

    // Learner must be enrolled in the parent course (staff bypasses).
    let course = db::course::find_by_id(pool, assessment.course_id)
        .await?
        .ok_or(AssessmentError::CourseUnavailable)?;
    let is_staff = user.is_admin() || course.instructor_id == user.id;
    if !is_staff {
        let enrollment = db::enrollment::find_for(pool, assessment.course_id, user.id).await?;
        if enrollment.is_none() {
            return Err(AssessmentError::NotEnrolled);
        }
    }

    // If there's already an open attempt, return it verbatim.
    if let Some(open) = db::assessment::find_open_attempt(pool, assessment_id, user.id).await? {
        return Ok(open);
    }

    if let Some(max) = assessment.max_attempts {
        let taken = db::assessment::count_attempts(pool, assessment_id, user.id).await?;
        if taken >= max as i64 {
            return Err(AssessmentError::NoAttemptsLeft);
        }
    }

    let next_attempt =
        (db::assessment::count_attempts(pool, assessment_id, user.id).await? + 1) as i32;
    let due_at = assessment
        .time_limit_minutes
        .map(|m| chrono::Utc::now() + chrono::Duration::minutes(m as i64));

    Ok(db::assessment::start_attempt(pool, assessment_id, user.id, due_at, next_attempt).await?)
}

pub struct GradedAttempt {
    pub attempt: Attempt,
    pub answers: Vec<AttemptAnswer>,
}

pub async fn submit_attempt(
    pool: &PgPool,
    user: &AuthUser,
    attempt_id: uuid::Uuid,
    answers: Vec<AnswerInput>,
) -> Result<GradedAttempt, AssessmentError> {
    let attempt = db::assessment::find_attempt(pool, attempt_id)
        .await?
        .ok_or(AssessmentError::NotFound)?;
    if attempt.user_id != user.id {
        return Err(AssessmentError::Forbidden);
    }
    if attempt.state != "in_progress" {
        return Err(AssessmentError::NoOpenAttempt);
    }

    let assessment = db::assessment::find_by_id(pool, attempt.assessment_id)
        .await?
        .ok_or(AssessmentError::NotFound)?;

    // Lookup the assessment's questions + their per-assessment points overrides.
    let links = db::assessment::list_questions(pool, attempt.assessment_id).await?;
    let question_ids: Vec<uuid::Uuid> = links.iter().map(|l| l.question_id).collect();
    let questions = db::question::find_many(pool, &question_ids).await?;
    let by_id: HashMap<uuid::Uuid, Question> = questions.into_iter().map(|q| (q.id, q)).collect();
    let points_for: HashMap<uuid::Uuid, Decimal> = links
        .iter()
        .filter_map(|l| {
            let q = by_id.get(&l.question_id)?;
            let pts = l.points_override.unwrap_or(q.default_points);
            Some((l.question_id, pts))
        })
        .collect();

    let mut graded: Vec<db::assessment::GradedAnswer> = Vec::with_capacity(links.len());
    let mut max_points = Decimal::ZERO;
    let mut score_points = Decimal::ZERO;
    let mut requires_manual = false;

    // Build a response lookup so unknown / missing answers are graded as wrong.
    let mut response_by_id: HashMap<uuid::Uuid, serde_json::Value> = answers
        .into_iter()
        .map(|a| (a.question_id, a.response))
        .collect();

    for link in &links {
        let question = match by_id.get(&link.question_id) {
            Some(q) => q,
            None => continue, // question was deleted; skip
        };
        let points_possible = *points_for.get(&link.question_id).unwrap_or(&Decimal::ZERO);
        max_points += points_possible;

        let kind = QuestionKind::from_code(&question.kind).unwrap_or(QuestionKind::Essay);
        let response = response_by_id
            .remove(&link.question_id)
            .unwrap_or(serde_json::Value::Null);

        let outcome = grading::grade(
            kind,
            &question.body,
            &question.answer,
            &response,
            points_possible,
        );

        if outcome.graded_by == "pending" {
            requires_manual = true;
        }
        score_points += outcome.points_earned;

        graded.push(db::assessment::GradedAnswer {
            question_id: link.question_id,
            response,
            is_correct: outcome.is_correct,
            points_earned: outcome.points_earned,
            graded_by: outcome.graded_by,
            feedback: outcome.feedback,
        });
    }

    let score_pct = if max_points > Decimal::ZERO {
        Some((score_points * Decimal::from(100)) / max_points)
    } else {
        None
    };
    let passed = score_pct.map(|p| p >= assessment.passing_score_pct);

    db::assessment::finalize_attempt(
        pool,
        attempt_id,
        &graded,
        db::assessment::AttemptFinalTotals {
            max_points,
            score_points,
            score_pct,
            passed,
            requires_manual,
        },
    )
    .await?;

    let attempt = db::assessment::find_attempt(pool, attempt_id)
        .await?
        .ok_or(AssessmentError::NotFound)?;
    let answers = db::assessment::list_answers(pool, attempt_id).await?;
    Ok(GradedAttempt { attempt, answers })
}

pub async fn list_my_attempts(
    pool: &PgPool,
    user: &AuthUser,
) -> Result<Vec<Attempt>, AssessmentError> {
    Ok(db::assessment::list_attempts_for_user(pool, user.id).await?)
}
