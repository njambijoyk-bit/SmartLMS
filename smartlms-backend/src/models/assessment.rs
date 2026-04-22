// Assessment model - questions, quizzes, assignments, grades, exams
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Question types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum QuestionType {
    MultipleChoice,
    TrueFalse,
    ShortAnswer,
    LongAnswer,
    Matching,
    Ordering,
    FillInBlank,
    Code,
}

/// Question entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Question {
    pub id: uuid::Uuid,
    pub bank_id: uuid::Uuid,
    pub question_text: String,
    pub question_type: QuestionType,
    pub options: Vec<QuestionOption>,
    pub correct_answer: String,
    pub explanation: Option<String>,
    pub points: i32,
    pub difficulty: String,
    pub tags: Vec<String>,
    pub created_at: DateTime<Utc>,
}

/// Question option for MCQ
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestionOption {
    pub id: uuid::Uuid,
    pub text: String,
    pub is_correct: bool,
}

/// Code execution result (for auto-grading code questions)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeExecutionResult {
    pub passed: bool,
    pub output: String,
    pub error: Option<String>,
    pub execution_time_ms: i32,
    pub memory_used_kb: i32,
    pub test_cases_passed: i32,
    pub total_test_cases: i32,
}

/// Question bank
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestionBank {
    pub id: uuid::Uuid,
    pub name: String,
    pub description: Option<String>,
    pub category: Option<String>,
    pub course_id: Option<uuid::Uuid>,
    pub question_count: i32,
    pub created_by: uuid::Uuid,
    pub created_at: DateTime<Utc>,
}

/// Quiz/Assessment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Assessment {
    pub id: uuid::Uuid,
    pub title: String,
    pub description: Option<String>,
    pub assessment_type: AssessmentType,
    pub course_id: uuid::Uuid,
    pub course_group_id: Option<uuid::Uuid>, // For lecturer-specific groups
    pub module_id: Option<uuid::Uuid>,
    pub created_by: uuid::Uuid, // Lecturer who created it
    pub time_limit_minutes: Option<i32>,
    pub passing_score: i32,
    pub shuffle_questions: bool,
    pub shuffle_options: bool,
    pub show_results: bool,
    pub show_results_immediately: bool, // New: control when results shown
    pub allow_retries: bool,
    pub max_retries: Option<i32>,
    pub require_lockdown_browser: bool, // New: exam mode
    pub allow_late_submission: bool, // New
    pub late_penalty_percent: i32, // New: 0-100
    pub is_published: bool,
    pub status: String, // draft, published, archived
    pub start_time: Option<DateTime<Utc>>, // New: availability window
    pub due_date: Option<DateTime<Utc>>,
    pub end_time: Option<DateTime<Utc>>, // New: hard deadline
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Assessment types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AssessmentType {
    Quiz,
    Exam,
    Assignment,
    Practice,
}

/// Quiz question (assessment question link)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssessmentQuestion {
    pub id: uuid::Uuid,
    pub assessment_id: uuid::Uuid,
    pub question_id: uuid::Uuid,
    pub question: Question,
    pub order: i32,
    pub points: i32,
}

/// Student attempt
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attempt {
    pub id: uuid::Uuid,
    pub assessment_id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub started_at: DateTime<Utc>,
    pub submitted_at: Option<DateTime<Utc>>,
    pub score: Option<f32>,
    pub percent_score: Option<f32>,
    pub passed: Option<bool>,
    pub time_spent_seconds: i32,
    pub status: String, // in_progress, submitted, graded, expired
    pub is_late: bool,
    pub lockdown_session_id: Option<String>, // Track browser session for exam mode
    pub ip_address: Option<String>, // For proctoring/audit
    pub attempt_number: i32, // For retry tracking
}

/// Student answer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Answer {
    pub id: uuid::Uuid,
    pub attempt_id: uuid::Uuid,
    pub question_id: uuid::Uuid,
    pub answer_text: Option<String>, // For essay/short answer
    pub selected_options: Vec<uuid::Uuid>, // For MCQ
    pub code_content: Option<String>, // For code questions
    pub file_url: Option<String>, // For file uploads
    pub is_correct: Option<bool>,
    pub points_earned: Option<i32>,
    pub auto_grade_score: Option<f32>, // Auto-graded portion
    pub manual_grade_score: Option<f32>, // Manually graded portion
    pub feedback: Option<String>, // Instructor feedback
    pub graded_by: Option<uuid::Uuid>,
    pub graded_at: Option<DateTime<Utc>>,
}

/// Gradebook entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Grade {
    pub id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub course_id: uuid::Uuid,
    pub assessment_id: Option<uuid::Uuid>,
    pub category: String,
    pub score: f32,
    pub max_score: f32,
    pub percent: f32,
    pub letter_grade: Option<String>,
    pub feedback: Option<String>,
    pub graded_by: Option<uuid::Uuid>,
    pub graded_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

/// Gradebook category (homework, exam, project, etc.)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GradeCategory {
    pub id: uuid::Uuid,
    pub course_id: uuid::Uuid,
    pub name: String,
    pub weight: f32,
    pub drop_lowest: Option<i32>,
    pub created_at: DateTime<Utc>,
}

// Request/Response types
#[derive(Debug, Deserialize)]
pub struct CreateQuestionRequest {
    pub bank_id: uuid::Uuid,
    pub question_text: String,
    pub question_type: QuestionType,
    pub options: Option<Vec<CreateQuestionOptionRequest>>,
    pub correct_answer: String,
    pub explanation: Option<String>,
    pub points: Option<i32>,
    pub difficulty: Option<String>,
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct CreateQuestionOptionRequest {
    pub text: String,
    pub is_correct: bool,
}

#[derive(Debug, Deserialize)]
pub struct CreateQuestionBankRequest {
    pub name: String,
    pub description: Option<String>,
    pub category: Option<String>,
    pub course_id: Option<uuid::Uuid>,
}

#[derive(Debug, Deserialize)]
pub struct CreateAssessmentRequest {
    pub title: String,
    pub description: Option<String>,
    pub assessment_type: AssessmentType,
    pub course_id: uuid::Uuid,
    pub course_group_id: Option<uuid::Uuid>, // For lecturer-specific groups
    pub module_id: Option<uuid::Uuid>,
    pub time_limit_minutes: Option<i32>,
    pub passing_score: Option<i32>,
    pub shuffle_questions: Option<bool>,
    pub shuffle_options: Option<bool>,
    pub show_results: Option<bool>,
    pub show_results_immediately: Option<bool>,
    pub allow_retries: Option<bool>,
    pub max_retries: Option<i32>,
    pub require_lockdown_browser: Option<bool>,
    pub allow_late_submission: Option<bool>,
    pub late_penalty_percent: Option<i32>,
    pub start_time: Option<DateTime<Utc>>,
    pub due_date: Option<DateTime<Utc>>,
    pub end_time: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
pub struct SubmitAnswerRequest {
    pub question_id: uuid::Uuid,
    pub answer_text: Option<String>,
    pub selected_options: Option<Vec<uuid::Uuid>>,
    pub code_content: Option<String>, // For code questions
}

#[derive(Debug, Deserialize)]
pub struct GradeSubmissionRequest {
    pub score: f32,
    pub max_score: f32,
    pub feedback: Option<String>,
    pub auto_grade_score: Option<f32>,
    pub manual_grade_score: Option<f32>,
}

#[derive(Debug, Deserialize)]
pub struct SubmitCodeRequest {
    pub question_id: uuid::Uuid,
    pub code: String,
    pub language: String, // python, java, cpp, etc.
}

#[derive(Debug, Serialize)]
pub struct CodeSubmissionResponse {
    pub result: CodeExecutionResult,
    pub score: f32,
    pub feedback: String,
}

#[derive(Debug, Serialize)]
pub struct AssessmentDetailResponse {
    pub assessment: Assessment,
    pub questions: Vec<AssessmentQuestion>,
    pub attempt_count: i64,
    pub avg_score: Option<f32>,
}

#[derive(Debug, Serialize)]
pub struct AttemptDetailResponse {
    pub attempt: Attempt,
    pub answers: Vec<Answer>,
}

#[derive(Debug, Serialize)]
pub struct GradebookResponse {
    pub grades: Vec<Grade>,
    pub total: i64,
    pub average: f32,
    pub letter_distribution: std::collections::HashMap<String, i32>,
}
