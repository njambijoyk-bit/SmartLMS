//! Assessment / question / attempt domain models.

use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use validator::Validate;

// ---------------------------------------------------------------------------
// Questions
// ---------------------------------------------------------------------------

/// Question taxonomy. `Mcq` / `Multi` / `TrueFalse` / `ShortAnswer` / `Numeric`
/// are auto-graded; `Essay` / `Code` require manual or ML grading.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QuestionKind {
    Mcq,
    Multi,
    TrueFalse,
    ShortAnswer,
    Numeric,
    Essay,
    Code,
}

impl QuestionKind {
    pub fn as_str(self) -> &'static str {
        match self {
            QuestionKind::Mcq => "mcq",
            QuestionKind::Multi => "multi",
            QuestionKind::TrueFalse => "true_false",
            QuestionKind::ShortAnswer => "short_answer",
            QuestionKind::Numeric => "numeric",
            QuestionKind::Essay => "essay",
            QuestionKind::Code => "code",
        }
    }

    pub fn from_code(s: &str) -> Option<Self> {
        match s {
            "mcq" => Some(QuestionKind::Mcq),
            "multi" => Some(QuestionKind::Multi),
            "true_false" => Some(QuestionKind::TrueFalse),
            "short_answer" => Some(QuestionKind::ShortAnswer),
            "numeric" => Some(QuestionKind::Numeric),
            "essay" => Some(QuestionKind::Essay),
            "code" => Some(QuestionKind::Code),
            _ => None,
        }
    }

    pub fn is_auto_graded(self) -> bool {
        matches!(
            self,
            QuestionKind::Mcq
                | QuestionKind::Multi
                | QuestionKind::TrueFalse
                | QuestionKind::ShortAnswer
                | QuestionKind::Numeric
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Question {
    pub id: uuid::Uuid,
    pub author_id: uuid::Uuid,
    pub kind: String,
    pub stem: String,
    pub body: serde_json::Value,
    /// Intentionally NOT serialised to learners — the service layer must
    /// strip this before sending to anyone who isn't the author or admin.
    #[serde(skip_serializing_if = "serde_json::Value::is_null")]
    pub answer: serde_json::Value,
    pub default_points: Decimal,
    pub explanation: Option<String>,
    pub tags: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateQuestionRequest {
    pub kind: QuestionKind,
    #[validate(length(min = 1, max = 4000))]
    pub stem: String,
    #[serde(default)]
    pub body: serde_json::Value,
    #[serde(default)]
    pub answer: serde_json::Value,
    #[serde(default = "default_points")]
    pub default_points: Decimal,
    #[validate(length(max = 4000))]
    pub explanation: Option<String>,
    #[serde(default)]
    pub tags: Vec<String>,
}

fn default_points() -> Decimal {
    Decimal::ONE
}

// ---------------------------------------------------------------------------
// Assessments
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AssessmentKind {
    Quiz,
    Exam,
    Survey,
    Practice,
}

impl AssessmentKind {
    pub fn as_str(self) -> &'static str {
        match self {
            AssessmentKind::Quiz => "quiz",
            AssessmentKind::Exam => "exam",
            AssessmentKind::Survey => "survey",
            AssessmentKind::Practice => "practice",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AssessmentStatus {
    Draft,
    Published,
    Archived,
}

impl AssessmentStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            AssessmentStatus::Draft => "draft",
            AssessmentStatus::Published => "published",
            AssessmentStatus::Archived => "archived",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Assessment {
    pub id: uuid::Uuid,
    pub course_id: uuid::Uuid,
    pub title: String,
    pub description: Option<String>,
    pub kind: String,
    pub status: String,
    pub time_limit_minutes: Option<i32>,
    pub max_attempts: Option<i32>,
    pub passing_score_pct: Decimal,
    pub shuffle_questions: bool,
    pub show_results_policy: String,
    pub available_from: Option<DateTime<Utc>>,
    pub available_until: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateAssessmentRequest {
    #[validate(length(min = 1, max = 200))]
    pub title: String,
    #[validate(length(max = 4000))]
    pub description: Option<String>,
    #[serde(default = "default_kind")]
    pub kind: AssessmentKind,
    #[validate(range(min = 1, max = 1440))]
    pub time_limit_minutes: Option<i32>,
    #[validate(range(min = 1, max = 100))]
    pub max_attempts: Option<i32>,
    #[serde(default = "default_passing")]
    pub passing_score_pct: Decimal,
    #[serde(default)]
    pub shuffle_questions: bool,
}

fn default_kind() -> AssessmentKind {
    AssessmentKind::Quiz
}

fn default_passing() -> Decimal {
    Decimal::from(60)
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateAssessmentRequest {
    #[validate(length(min = 1, max = 200))]
    pub title: Option<String>,
    pub description: Option<String>,
    pub status: Option<AssessmentStatus>,
    #[validate(range(min = 1, max = 1440))]
    pub time_limit_minutes: Option<i32>,
    #[validate(range(min = 1, max = 100))]
    pub max_attempts: Option<i32>,
    pub passing_score_pct: Option<Decimal>,
    pub shuffle_questions: Option<bool>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct AddQuestionToAssessmentRequest {
    pub question_id: uuid::Uuid,
    pub position: Option<i32>,
    pub points_override: Option<Decimal>,
}

// ---------------------------------------------------------------------------
// Attempts
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AttemptState {
    InProgress,
    Submitted,
    Graded,
    Expired,
}

impl AttemptState {
    pub fn as_str(self) -> &'static str {
        match self {
            AttemptState::InProgress => "in_progress",
            AttemptState::Submitted => "submitted",
            AttemptState::Graded => "graded",
            AttemptState::Expired => "expired",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attempt {
    pub id: uuid::Uuid,
    pub assessment_id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub state: String,
    pub started_at: DateTime<Utc>,
    pub submitted_at: Option<DateTime<Utc>>,
    pub due_at: Option<DateTime<Utc>>,
    pub score_points: Decimal,
    pub max_points: Decimal,
    pub score_pct: Option<Decimal>,
    pub passed: Option<bool>,
    pub requires_manual: bool,
    pub attempt_no: i32,
}

#[derive(Debug, Deserialize, Validate)]
pub struct SubmitAnswersRequest {
    #[validate(length(min = 0, max = 1000))]
    pub answers: Vec<AnswerInput>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AnswerInput {
    pub question_id: uuid::Uuid,
    #[serde(default)]
    pub response: serde_json::Value,
}

#[derive(Debug, Clone, Serialize)]
pub struct AttemptAnswer {
    pub attempt_id: uuid::Uuid,
    pub question_id: uuid::Uuid,
    pub response: serde_json::Value,
    pub is_correct: Option<bool>,
    pub points_earned: Decimal,
    pub graded_by: String,
    pub feedback: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn question_kind_roundtrip() {
        for kind in [
            QuestionKind::Mcq,
            QuestionKind::Multi,
            QuestionKind::TrueFalse,
            QuestionKind::ShortAnswer,
            QuestionKind::Numeric,
            QuestionKind::Essay,
            QuestionKind::Code,
        ] {
            assert_eq!(QuestionKind::from_code(kind.as_str()), Some(kind));
        }
        assert!(QuestionKind::from_code("bogus").is_none());
    }

    #[test]
    fn auto_graded_excludes_essay_and_code() {
        assert!(QuestionKind::Mcq.is_auto_graded());
        assert!(QuestionKind::TrueFalse.is_auto_graded());
        assert!(QuestionKind::Numeric.is_auto_graded());
        assert!(QuestionKind::ShortAnswer.is_auto_graded());
        assert!(QuestionKind::Multi.is_auto_graded());
        assert!(!QuestionKind::Essay.is_auto_graded());
        assert!(!QuestionKind::Code.is_auto_graded());
    }

    #[test]
    fn attempt_state_codes_stable() {
        assert_eq!(AttemptState::InProgress.as_str(), "in_progress");
        assert_eq!(AttemptState::Submitted.as_str(), "submitted");
        assert_eq!(AttemptState::Graded.as_str(), "graded");
        assert_eq!(AttemptState::Expired.as_str(), "expired");
    }
}
