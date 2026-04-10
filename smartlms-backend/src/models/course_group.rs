// Course Group model - lecturer-specific student groups within courses
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Course group for organizing students by lecturer/section
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CourseGroup {
    pub id: uuid::Uuid,
    pub course_id: uuid::Uuid,
    pub instructor_id: uuid::Uuid, // The lecturer who owns this group
    pub name: String,              // e.g., "Section A", "Group 1", "Morning Class"
    pub description: Option<String>,
    pub max_students: Option<i32>,
    pub student_count: i32,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Enrollment in a specific course group
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CourseGroupEnrollment {
    pub id: uuid::Uuid,
    pub group_id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub enrolled_at: DateTime<Utc>,
    pub enrolled_by: uuid::Uuid, // Who added them (instructor or admin)
    pub is_active: bool,
    pub notes: Option<String>,
}

/// Group session (links live sessions to specific groups)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupSession {
    pub id: uuid::Uuid,
    pub group_id: uuid::Uuid,
    pub session_id: uuid::Uuid, // References live_sessions
    pub is_mandatory: bool,
    pub created_at: DateTime<Utc>,
}

/// Group assessment (assessments specific to a group)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupAssessment {
    pub id: uuid::Uuid,
    pub group_id: uuid::Uuid,
    pub assessment_id: uuid::Uuid, // References assessments
    pub is_group_only: bool,       // true = only this group takes it
    pub created_at: DateTime<Utc>,
}

// Request/Response types
#[derive(Debug, Deserialize)]
pub struct CreateCourseGroupRequest {
    pub course_id: uuid::Uuid,
    pub name: String,
    pub description: Option<String>,
    pub max_students: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateCourseGroupRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub max_students: Option<i32>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct AddStudentToGroupRequest {
    pub user_id: uuid::Uuid,
    pub notes: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct BulkAddStudentsRequest {
    pub user_ids: Vec<uuid::Uuid>,
}

#[derive(Debug, Serialize)]
pub struct CourseGroupDetailResponse {
    pub group: CourseGroup,
    pub students: Vec<GroupStudentInfo>,
    pub sessions: Vec<GroupSessionInfo>,
    pub assessments: Vec<GroupAssessmentInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupStudentInfo {
    pub user_id: uuid::Uuid,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub enrolled_at: DateTime<Utc>,
    pub enrollment_status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupSessionInfo {
    pub session_id: uuid::Uuid,
    pub title: String,
    pub scheduled_start: DateTime<Utc>,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupAssessmentInfo {
    pub assessment_id: uuid::Uuid,
    pub title: String,
    pub assessment_type: String,
    pub is_group_only: bool,
}

#[derive(Debug, Serialize)]
pub struct CourseGroupListResponse {
    pub groups: Vec<CourseGroupWithInstructor>,
    pub total: i64,
    pub page: i64,
    pub per_page: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CourseGroupWithInstructor {
    pub group: CourseGroup,
    pub instructor_name: String,
    pub instructor_email: String,
}
