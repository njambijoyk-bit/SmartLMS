//! Enrollment models

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Course enrollment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Enrollment {
    pub id: Uuid,
    pub course_id: Uuid,
    pub student_id: Uuid,
    pub status: EnrollmentStatus,
    pub enrolled_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub progress: f32,
    pub last_accessed_at: Option<DateTime<Utc>>,
}

impl Enrollment {
    pub fn new(course_id: Uuid, student_id: Uuid) -> Self {
        Self {
            id: Uuid::new_v4(),
            course_id,
            student_id,
            status: EnrollmentStatus::Active,
            enrolled_at: Utc::now(),
            completed_at: None,
            progress: 0.0,
            last_accessed_at: None,
        }
    }
}

/// Enrollment status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum EnrollmentStatus {
    Active,
    Completed,
    Dropped,
    Suspended,
}

/// Grade record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Grade {
    pub id: Uuid,
    pub student_id: Uuid,
    pub course_id: Uuid,
    pub assessment_id: Option<Uuid>,
    pub grade_type: GradeType,
    pub score: f32,
    pub max_score: f32,
    pub letter_grade: Option<String>,
    pub comment: Option<String>,
    pub graded_by: Option<Uuid>,
    pub graded_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

/// Grade type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum GradeType {
    Quiz,
    Exam,
    Assignment,
    Project,
    Participation,
    Midterm,
    Final,
}

/// Gradebook entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GradebookEntry {
    pub student_id: Uuid,
    pub student_name: String,
    pub course_id: Uuid,
    pub course_code: String,
    pub assessments: Vec<Grade>,
    pub total_score: f32,
    pub max_possible: f32,
    pub weighted_score: f32,
    pub letter_grade: Option<String>,
}

/// Create enrollment request
#[derive(Debug, Deserialize)]
pub struct CreateEnrollmentRequest {
    pub course_id: Uuid,
    pub student_ids: Vec<Uuid>,
}

/// Bulk enrollment response
#[derive(Debug, Serialize)]
pub struct BulkEnrollmentResponse {
    pub enrolled: Vec<Uuid>,
    pub failed: Vec<FailedEnrollment>,
}

/// Failed enrollment record
#[derive(Debug, Serialize)]
pub struct FailedEnrollment {
    pub student_id: Uuid,
    pub reason: String,
}

/// Grade request
#[derive(Debug, Deserialize)]
pub struct CreateGradeRequest {
    pub student_id: Uuid,
    pub course_id: Uuid,
    pub assessment_id: Option<Uuid>,
    pub grade_type: GradeType,
    pub score: f32,
    pub max_score: f32,
    pub comment: Option<String>,
}

/// Enrollment filter
#[derive(Debug, Deserialize, Default)]
pub struct EnrollmentFilter {
    pub course_id: Option<Uuid>,
    pub student_id: Option<Uuid>,
    pub status: Option<EnrollmentStatus>,
    pub page: Option<usize>,
    pub page_size: Option<usize>,
}