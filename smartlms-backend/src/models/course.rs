//! Course / module / lesson / enrollment domain models.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

/// Course publishing state. Write-time transitions are handled by the
/// service layer (e.g. draft → published requires at least one module).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CourseStatus {
    Draft,
    Published,
    Archived,
}

impl CourseStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            CourseStatus::Draft => "draft",
            CourseStatus::Published => "published",
            CourseStatus::Archived => "archived",
        }
    }

    pub fn from_code(s: &str) -> Option<Self> {
        match s {
            "draft" => Some(CourseStatus::Draft),
            "published" => Some(CourseStatus::Published),
            "archived" => Some(CourseStatus::Archived),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CourseVisibility {
    Private,
    Institution,
    Public,
}

impl CourseVisibility {
    pub fn as_str(self) -> &'static str {
        match self {
            CourseVisibility::Private => "private",
            CourseVisibility::Institution => "institution",
            CourseVisibility::Public => "public",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Course {
    pub id: uuid::Uuid,
    pub slug: String,
    pub title: String,
    pub subtitle: Option<String>,
    pub description: Option<String>,
    pub cover_url: Option<String>,
    pub instructor_id: uuid::Uuid,
    pub status: String,
    pub visibility: String,
    pub language: String,
    pub config: serde_json::Value,
    pub published_at: Option<DateTime<Utc>>,
    pub archived_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateCourseRequest {
    #[validate(length(min = 2, max = 120), custom(function = "validate_slug"))]
    pub slug: String,
    #[validate(length(min = 1, max = 200))]
    pub title: String,
    #[validate(length(max = 300))]
    pub subtitle: Option<String>,
    pub description: Option<String>,
    #[validate(url)]
    pub cover_url: Option<String>,
    #[serde(default = "default_language")]
    pub language: String,
    #[serde(default)]
    pub visibility: Option<CourseVisibility>,
}

fn default_language() -> String {
    "en".to_string()
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateCourseRequest {
    #[validate(length(min = 1, max = 200))]
    pub title: Option<String>,
    #[validate(length(max = 300))]
    pub subtitle: Option<String>,
    pub description: Option<String>,
    #[validate(url)]
    pub cover_url: Option<String>,
    pub visibility: Option<CourseVisibility>,
    pub status: Option<CourseStatus>,
    pub language: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Module {
    pub id: uuid::Uuid,
    pub course_id: uuid::Uuid,
    pub title: String,
    pub summary: Option<String>,
    pub position: i32,
    pub unlock_rule: serde_json::Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateModuleRequest {
    #[validate(length(min = 1, max = 200))]
    pub title: String,
    #[validate(length(max = 500))]
    pub summary: Option<String>,
    pub position: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Lesson {
    pub id: uuid::Uuid,
    pub module_id: uuid::Uuid,
    pub title: String,
    pub kind: String,
    pub content: serde_json::Value,
    pub position: i32,
    pub duration_s: Option<i32>,
    pub is_required: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LessonKind {
    Video,
    Text,
    File,
    Link,
    Quiz,
    Live,
}

impl LessonKind {
    pub fn as_str(self) -> &'static str {
        match self {
            LessonKind::Video => "video",
            LessonKind::Text => "text",
            LessonKind::File => "file",
            LessonKind::Link => "link",
            LessonKind::Quiz => "quiz",
            LessonKind::Live => "live",
        }
    }
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateLessonRequest {
    #[validate(length(min = 1, max = 200))]
    pub title: String,
    pub kind: LessonKind,
    #[serde(default = "default_content")]
    pub content: serde_json::Value,
    pub position: Option<i32>,
    pub duration_s: Option<i32>,
    #[serde(default = "default_required")]
    pub is_required: bool,
}

fn default_content() -> serde_json::Value {
    serde_json::json!({})
}

fn default_required() -> bool {
    true
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum EnrollmentStatus {
    Pending,
    Active,
    Completed,
    Dropped,
    Suspended,
}

impl EnrollmentStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            EnrollmentStatus::Pending => "pending",
            EnrollmentStatus::Active => "active",
            EnrollmentStatus::Completed => "completed",
            EnrollmentStatus::Dropped => "dropped",
            EnrollmentStatus::Suspended => "suspended",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Enrollment {
    pub id: uuid::Uuid,
    pub course_id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub status: String,
    pub enrolled_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub progress_pct: i32,
    pub last_seen_at: Option<DateTime<Utc>>,
}

/// Shared slug validator. Re-uses the DNS-label rule from onboarding to
/// keep slugs URL-safe.
pub fn validate_slug(s: &str) -> Result<(), validator::ValidationError> {
    crate::models::onboarding::validate_slug(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn course_status_roundtrip() {
        for status in [
            CourseStatus::Draft,
            CourseStatus::Published,
            CourseStatus::Archived,
        ] {
            assert_eq!(CourseStatus::from_code(status.as_str()), Some(status));
        }
        assert!(CourseStatus::from_code("bogus").is_none());
    }

    #[test]
    fn enrollment_status_codes_are_stable() {
        assert_eq!(EnrollmentStatus::Pending.as_str(), "pending");
        assert_eq!(EnrollmentStatus::Active.as_str(), "active");
        assert_eq!(EnrollmentStatus::Completed.as_str(), "completed");
        assert_eq!(EnrollmentStatus::Dropped.as_str(), "dropped");
        assert_eq!(EnrollmentStatus::Suspended.as_str(), "suspended");
    }
}
