// Course model - courses, modules, lessons, content
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Course status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CourseStatus {
    Draft,
    Published,
    Archived,
}

/// Course entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Course {
    pub id: uuid::Uuid,
    pub title: String,
    pub description: Option<String>,
    pub short_description: Option<String>,
    pub thumbnail_url: Option<String>,
    pub status: CourseStatus,
    pub category: Option<String>,
    pub tags: Vec<String>,
    pub instructor_id: uuid::Uuid,
    pub enrollment_count: i64,
    pub completion_rate: f32,
    pub rating: f32,
    pub language: String,
    pub difficulty: String,
    pub duration_hours: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub published_at: Option<DateTime<Utc>>,
}

/// Module (section) within a course
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Module {
    pub id: uuid::Uuid,
    pub course_id: uuid::Uuid,
    pub title: String,
    pub description: Option<String>,
    pub order: i32,
    pub duration_minutes: i32,
    pub is_preview: bool,
    pub created_at: DateTime<Utc>,
}

/// Lesson within a module
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Lesson {
    pub id: uuid::Uuid,
    pub module_id: uuid::Uuid,
    pub title: String,
    pub lesson_type: LessonType,
    pub content: Option<String>,   // HTML/Markdown for text lessons
    pub video_url: Option<String>, // Video hosting URL
    pub video_duration_seconds: Option<i32>,
    pub duration_minutes: i32,
    pub order: i32,
    pub is_preview: bool,
    pub is_free: bool,
    pub created_at: DateTime<Utc>,
}

/// Lesson content types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LessonType {
    Video,
    Text,
    Quiz,
    Assignment,
    Document,
    External,
    SCORM,
}

/// Course enrollment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Enrollment {
    pub id: uuid::Uuid,
    pub course_id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub progress_percent: f32,
    pub completed_lessons: Vec<uuid::Uuid>,
    pub started_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub last_accessed_at: DateTime<Utc>,
}

/// Course progress for a user
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CourseProgress {
    pub course_id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub current_lesson_id: Option<uuid::Uuid>,
    pub completed_modules: Vec<uuid::Uuid>,
    pub completed_lessons: Vec<uuid::Uuid>,
    pub time_spent_seconds: i64,
    pub progress_percent: f32,
}

/// Course template for quick course creation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CourseTemplate {
    pub id: uuid::Uuid,
    pub name: String,
    pub description: String,
    pub category: String,
    pub module_count: i32,
    pub thumbnail_url: Option<String>,
    pub is_premium: bool,
}

// Request/Response types
#[derive(Debug, Deserialize)]
pub struct CreateCourseRequest {
    pub title: String,
    pub description: Option<String>,
    pub category: Option<String>,
    pub tags: Option<Vec<String>>,
    pub language: Option<String>,
    pub difficulty: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateCourseRequest {
    pub title: Option<String>,
    pub description: Option<String>,
    pub category: Option<String>,
    pub tags: Option<Vec<String>>,
    pub status: Option<CourseStatus>,
    pub thumbnail_url: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateModuleRequest {
    pub course_id: uuid::Uuid,
    pub title: String,
    pub description: Option<String>,
    pub order: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct CreateLessonRequest {
    pub module_id: uuid::Uuid,
    pub title: String,
    pub lesson_type: LessonType,
    pub content: Option<String>,
    pub video_url: Option<String>,
    pub order: Option<i32>,
    pub is_preview: Option<bool>,
    pub is_free: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateModuleRequest {
    pub title: Option<String>,
    pub description: Option<String>,
    pub order: Option<i32>,
    pub duration_minutes: Option<i32>,
    pub is_preview: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateLessonRequest {
    pub title: Option<String>,
    pub lesson_type: Option<LessonType>,
    pub content: Option<String>,
    pub video_url: Option<String>,
    pub duration_minutes: Option<i32>,
    pub order: Option<i32>,
    pub is_preview: Option<bool>,
    pub is_free: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct ReorderItemsRequest {
    pub items: Vec<ReorderItem>,
}

#[derive(Debug, Deserialize)]
pub struct ReorderItem {
    pub id: uuid::Uuid,
    pub order: i32,
}

#[derive(Debug, Serialize)]
pub struct CourseListResponse {
    pub courses: Vec<Course>,
    pub total: i64,
    pub page: i64,
    pub per_page: i64,
}

#[derive(Debug, Serialize)]
pub struct CourseDetailResponse {
    pub course: Course,
    pub modules: Vec<ModuleWithLessons>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleWithLessons {
    pub module: Module,
    pub lessons: Vec<Lesson>,
}
