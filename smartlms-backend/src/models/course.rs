//! Course models

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Course entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Course {
    pub id: Uuid,
    pub institution_id: Uuid,
    pub code: String,
    pub title: String,
    pub description: Option<String>,
    pub instructor_id: Uuid,
    pub category: Option<String>,
    pub units: i32,
    pub status: CourseStatus,
    pub thumbnail_url: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub published_at: Option<DateTime<Utc>>,
}

impl Course {
    pub fn new(
        institution_id: Uuid,
        code: String,
        title: String,
        instructor_id: Uuid,
        units: i32,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            institution_id,
            code,
            title,
            description: None,
            instructor_id,
            category: None,
            units,
            status: CourseStatus::Draft,
            thumbnail_url: None,
            created_at: now,
            updated_at: now,
            published_at: None,
        }
    }
}

/// Course status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CourseStatus {
    Draft,
    Published,
    Archived,
}

/// Course module/section
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CourseModule {
    pub id: Uuid,
    pub course_id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub order_index: i32,
    pub created_at: DateTime<Utc>,
}

/// Course content item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentItem {
    pub id: Uuid,
    pub module_id: Uuid,
    pub title: String,
    pub content_type: ContentType,
    pub content: String,
    pub order_index: i32,
    pub duration_minutes: Option<i32>,
    pub created_at: DateTime<Utc>,
}

/// Content type enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ContentType {
    Text,
    Video,
    Document,
    Quiz,
    Assignment,
    Link,
}

/// Create course request
#[derive(Debug, Deserialize, Serialize)]
pub struct CreateCourseRequest {
    pub code: String,
    pub title: String,
    pub description: Option<String>,
    pub instructor_id: Uuid,
    pub category: Option<String>,
    pub units: i32,
}

/// Update course request
#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateCourseRequest {
    pub code: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub instructor_id: Option<Uuid>,
    pub category: Option<String>,
    pub units: Option<i32>,
    pub status: Option<CourseStatus>,
    pub thumbnail_url: Option<String>,
}

/// Course response
#[derive(Debug, Serialize)]
pub struct CourseResponse {
    pub id: Uuid,
    pub institution_id: Uuid,
    pub code: String,
    pub title: String,
    pub description: Option<String>,
    pub instructor_id: Uuid,
    pub category: Option<String>,
    pub units: i32,
    pub status: CourseStatus,
    pub thumbnail_url: Option<String>,
    pub created_at: DateTime<Utc>,
    pub published_at: Option<DateTime<Utc>>,
    pub enrolled_count: Option<i32>,
}

impl From<Course> for CourseResponse {
    fn from(course: Course) -> Self {
        Self {
            id: course.id,
            institution_id: course.institution_id,
            code: course.code,
            title: course.title,
            description: course.description,
            instructor_id: course.instructor_id,
            category: course.category,
            units: course.units,
            status: course.status,
            thumbnail_url: course.thumbnail_url,
            created_at: course.created_at,
            published_at: course.published_at,
            enrolled_count: None,
        }
    }
}

/// Course list filter
#[derive(Debug, Deserialize, Default)]
pub struct CourseFilter {
    pub status: Option<CourseStatus>,
    pub category: Option<String>,
    pub instructor_id: Option<Uuid>,
    pub search: Option<String>,
    pub page: Option<usize>,
    pub page_size: Option<usize>,
}