//! Assessment models

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Assessment entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Assessment {
    pub id: Uuid,
    pub course_id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub assessment_type: AssessmentType,
    pub status: AssessmentStatus,
    pub created_by: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub published_at: Option<DateTime<Utc>>,
    pub due_date: Option<DateTime<Utc>>,
    pub time_limit_minutes: Option<i32>,
    pub max_attempts: Option<i32>,
    pub passing_score: Option<f32>,
    pub randomize_questions: bool,
    pub show_results: bool,
}

impl Assessment {
    pub fn new(
        course_id: Uuid,
        title: String,
        assessment_type: AssessmentType,
        created_by: Uuid,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            course_id,
            title,
            description: None,
            assessment_type,
            status: AssessmentStatus::Draft,
            created_by,
            created_at: now,
            updated_at: now,
            published_at: None,
            due_date: None,
            time_limit_minutes: None,
            max_attempts: None,
            passing_score: None,
            randomize_questions: false,
            show_results: true,
        }
    }
}

/// Assessment type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AssessmentType {
    Quiz,
    Exam,
    Assignment,
    Project,
    Practice,
}

/// Assessment status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AssessmentStatus {
    Draft,
    Published,
    Archived,
}

/// Question entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Question {
    pub id: Uuid,
    pub assessment_id: Uuid,
    pub question_text: String,
    pub question_type: QuestionType,
    pub points: f32,
    pub order_index: i32,
    pub options: Vec<QuestionOption>,
    pub correct_answer: Option<String>,
    pub explanation: Option<String>,
    pub created_at: DateTime<Utc>,
}

/// Question type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum QuestionType {
    MultipleChoice,
    TrueFalse,
    ShortAnswer,
    Essay,
    FillInBlank,
    Matching,
    Ordering,
}

/// Question option
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestionOption {
    pub id: String,
    pub text: String,
    pub is_correct: bool,
}

/// Assessment submission
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Submission {
    pub id: Uuid,
    pub assessment_id: Uuid,
    pub student_id: Uuid,
    pub started_at: DateTime<Utc>,
    pub submitted_at: Option<DateTime<Utc>>,
    pub score: Option<f32>,
    pub max_score: f32,
    pub status: SubmissionStatus,
    pub answers: Vec<SubmissionAnswer>,
}

/// Submission status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SubmissionStatus {
    InProgress,
    Submitted,
    Graded,
    Reviewed,
}

/// Submission answer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubmissionAnswer {
    pub question_id: Uuid,
    pub answer: String,
    pub is_correct: Option<bool>,
    pub points_earned: Option<f32>,
    pub feedback: Option<String>,
}

/// Create assessment request
#[derive(Debug, Deserialize)]
pub struct CreateAssessmentRequest {
    pub course_id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub assessment_type: AssessmentType,
    pub due_date: Option<DateTime<Utc>>,
    pub time_limit_minutes: Option<i32>,
    pub max_attempts: Option<i32>,
    pub passing_score: Option<f32>,
    pub randomize_questions: bool,
    pub show_results: bool,
}

/// Add question request
#[derive(Debug, Deserialize)]
pub struct AddQuestionRequest {
    pub question_text: String,
    pub question_type: QuestionType,
    pub points: f32,
    pub options: Vec<QuestionOption>,
    pub correct_answer: Option<String>,
    pub explanation: Option<String>,
}

/// Assessment response
#[derive(Debug, Serialize)]
pub struct AssessmentResponse {
    pub id: Uuid,
    pub course_id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub assessment_type: AssessmentType,
    pub status: AssessmentStatus,
    pub due_date: Option<DateTime<Utc>>,
    pub time_limit_minutes: Option<i32>,
    pub max_attempts: Option<i32>,
    pub passing_score: Option<f32>,
    pub randomize_questions: bool,
    pub show_results: bool,
    pub created_at: DateTime<Utc>,
    pub published_at: Option<DateTime<Utc>>,
    pub question_count: Option<i32>,
}

impl From<Assessment> for AssessmentResponse {
    fn from(assessment: Assessment) -> Self {
        Self {
            id: assessment.id,
            course_id: assessment.course_id,
            title: assessment.title,
            description: assessment.description,
            assessment_type: assessment.assessment_type,
            status: assessment.status,
            due_date: assessment.due_date,
            time_limit_minutes: assessment.time_limit_minutes,
            max_attempts: assessment.max_attempts,
            passing_score: assessment.passing_score,
            randomize_questions: assessment.randomize_questions,
            show_results: assessment.show_results,
            created_at: assessment.created_at,
            published_at: assessment.published_at,
            question_count: None,
        }
    }
}

/// Assessment filter
#[derive(Debug, Deserialize, Default)]
pub struct AssessmentFilter {
    pub course_id: Option<Uuid>,
    pub assessment_type: Option<AssessmentType>,
    pub status: Option<AssessmentStatus>,
    pub search: Option<String>,
    pub page: Option<usize>,
    pub page_size: Option<usize>,
}