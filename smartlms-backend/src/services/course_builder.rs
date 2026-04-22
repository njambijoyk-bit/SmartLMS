// Course Builder Service - Drag-drop course creation and management
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Course structure with modules and lessons
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CourseStructure {
    pub id: uuid::Uuid,
    pub title: String,
    pub description: Option<String>,
    pub modules: Vec<Module>,
}

/// Course module
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Module {
    pub id: uuid::Uuid,
    pub title: String,
    pub description: Option<String>,
    pub order: i32,
    pub lessons: Vec<Lesson>,
    pub prerequisites: Vec<uuid::Uuid>, // Module IDs that must be completed first
    pub is_published: bool,
}

/// Lesson within a module
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Lesson {
    pub id: uuid::Uuid,
    pub title: String,
    pub lesson_type: LessonType,
    pub content: Option<String>,   // Text content or HTML
    pub video_url: Option<String>, // URL to video (HLS stream)
    pub duration_minutes: Option<i32>,
    pub order: i32,
    pub is_published: bool,
    pub is_free: bool, // Preview lesson
}

/// Lesson content types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LessonType {
    Video,
    Text,
    Quiz,
    Assignment,
    Document,
    SCORM,
    External,
}

/// Reorder request for drag-drop
#[derive(Debug, Clone, Deserialize)]
pub struct ReorderRequest {
    pub items: Vec<ReorderItem>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ReorderItem {
    pub id: uuid::Uuid,
    pub order: i32,
    pub parent_id: Option<uuid::Uuid>, // For nesting lessons under modules
}

/// Move item between modules
#[derive(Debug, Clone, Deserialize)]
pub struct MoveItemRequest {
    pub item_id: uuid::Uuid,
    pub target_module_id: uuid::Uuid,
    pub new_order: i32,
}

/// Course template
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CourseTemplate {
    pub id: uuid::Uuid,
    pub name: String,
    pub description: String,
    pub category: String,
    pub structure: CourseStructure,
    pub thumbnail_url: Option<String>,
    pub is_public: bool,
    pub usage_count: i32,
}

// Service functions
pub mod service {
    use super::*;

    /// Create a new course with initial structure
    pub async fn create_course(
        pool: &PgPool,
        institution_id: uuid::Uuid,
        creator_id: uuid::Uuid,
        title: &str,
        description: Option<&str>,
    ) -> Result<CourseStructure, String> {
        let course_id = Uuid::new_v4();

        sqlx::query!(
            "INSERT INTO courses (id, institution_id, title, description, created_by, created_at, updated_at)
             VALUES ($1, $2, $3, $4, $5, $6, $7)",
            course_id, institution_id, title, description, creator_id, Utc::now(), Utc::now()
        )
        .execute(pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(CourseStructure {
            id: course_id,
            title: title.to_string(),
            description: description.map(String::from),
            modules: vec![],
        })
    }

    /// Add a new module to a course
    pub async fn add_module(
        pool: &PgPool,
        course_id: uuid::Uuid,
        title: &str,
    ) -> Result<Module, String> {
        // Get current max order
        let max_order: Option<i32> = sqlx::query_scalar!(
            "SELECT MAX(order_index) FROM course_modules WHERE course_id = $1",
            course_id
        )
        .fetch_optional(pool)
        .await
        .map_err(|e| e.to_string())?;

        let module_id = Uuid::new_v4();
        let order = max_order.unwrap_or(0) + 1;

        sqlx::query!(
            "INSERT INTO course_modules (id, course_id, title, order_index, created_at)
             VALUES ($1, $2, $3, $4, $5)",
            module_id,
            course_id,
            title,
            order,
            Utc::now()
        )
        .execute(pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(Module {
            id: module_id,
            title: title.to_string(),
            description: None,
            order,
            lessons: vec![],
            prerequisites: vec![],
            is_published: false,
        })
    }

    /// Add a lesson to a module
    pub async fn add_lesson(
        pool: &PgPool,
        module_id: uuid::Uuid,
        lesson_type: LessonType,
        title: &str,
    ) -> Result<Lesson, String> {
        // Get current max order for this module
        let max_order: Option<i32> = sqlx::query_scalar!(
            "SELECT MAX(order_index) FROM module_lessons WHERE module_id = $1",
            module_id
        )
        .fetch_optional(pool)
        .await
        .map_err(|e| e.to_string())?;

        let lesson_id = Uuid::new_v4();
        let order = max_order.unwrap_or(0) + 1;

        sqlx::query!(
            "INSERT INTO module_lessons (id, module_id, title, lesson_type, order_index, created_at)
             VALUES ($1, $2, $3, $4, $5, $6)",
            lesson_id, module_id, title, format!("{:?}", lesson_type).to_lowercase(), order, Utc::now()
        )
        .execute(pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(Lesson {
            id: lesson_id,
            title: title.to_string(),
            lesson_type,
            content: None,
            video_url: None,
            duration_minutes: None,
            order,
            is_published: false,
            is_free: false,
        })
    }

    /// Reorder modules and lessons (drag-drop)
    pub async fn reorder_items(
        pool: &PgPool,
        course_id: uuid::Uuid,
        items: Vec<ReorderItem>,
    ) -> Result<(), String> {
        for item in items {
            // Determine if it's a module or lesson by checking both tables
            let module_result = sqlx::query!(
                "UPDATE course_modules SET order_index = $1 WHERE id = $2 AND course_id = $3",
                item.order,
                item.id,
                course_id
            )
            .execute(pool)
            .await;

            if module_result.map(|r| r.rows_affected()).unwrap_or(0) == 0 {
                // Try as lesson
                sqlx::query!(
                    "UPDATE module_lessons SET order_index = $1 WHERE id = $2",
                    item.order,
                    item.id
                )
                .execute(pool)
                .await
                .map_err(|e| e.to_string())?;
            }
        }

        Ok(())
    }

    /// Move lesson between modules
    pub async fn move_lesson(
        pool: &PgPool,
        lesson_id: uuid::Uuid,
        target_module_id: uuid::Uuid,
        new_order: i32,
    ) -> Result<(), String> {
        // Update lesson's module and order
        sqlx::query!(
            "UPDATE module_lessons SET module_id = $1, order_index = $2 WHERE id = $3",
            target_module_id,
            new_order,
            lesson_id
        )
        .execute(pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(())
    }

    /// Update lesson content
    pub async fn update_lesson_content(
        pool: &PgPool,
        lesson_id: uuid::Uuid,
        content: Option<&str>,
        video_url: Option<&str>,
        duration_minutes: Option<i32>,
    ) -> Result<Lesson, String> {
        sqlx::query!(
            "UPDATE module_lessons SET content = $1, video_url = $2, duration_minutes = $3 WHERE id = $4",
            content, video_url, duration_minutes, lesson_id
        )
        .execute(pool)
        .await
        .map_err(|e| e.to_string())?;

        // Fetch and return updated lesson
        let row = sqlx::query!(
            "SELECT id, module_id, title, lesson_type, content, video_url, duration_minutes, 
             order_index, is_published, is_free FROM module_lessons WHERE id = $1",
            lesson_id
        )
        .fetch_one(pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(Lesson {
            id: row.id,
            title: row.title,
            lesson_type: LessonType::Text, // Parse from string
            content: row.content,
            video_url: row.video_url,
            duration_minutes: row.duration_minutes,
            order: row.order_index,
            is_published: row.is_published,
            is_free: row.is_free,
        })
    }

    /// Set module prerequisites
    pub async fn set_prerequisites(
        pool: &PgPool,
        module_id: uuid::Uuid,
        prerequisite_ids: Vec<uuid::Uuid>,
    ) -> Result<(), String> {
        // Clear existing prerequisites
        sqlx::query!(
            "DELETE FROM module_prerequisites WHERE module_id = $1",
            module_id
        )
        .execute(pool)
        .await
        .map_err(|e| e.to_string())?;

        // Add new prerequisites
        for prereq_id in prerequisite_ids {
            sqlx::query!(
                "INSERT INTO module_prerequisites (module_id, prerequisite_module_id) VALUES ($1, $2)",
                module_id, prereq_id
            )
            .execute(pool)
            .await
            .map_err(|e| e.to_string())?;
        }

        Ok(())
    }

    /// Publish course (all modules and lessons)
    pub async fn publish_course(pool: &PgPool, course_id: uuid::Uuid) -> Result<(), String> {
        // Publish all modules in course
        sqlx::query!(
            "UPDATE course_modules SET is_published = true WHERE course_id = $1",
            course_id
        )
        .execute(pool)
        .await
        .map_err(|e| e.to_string())?;

        // Publish all lessons in all modules
        sqlx::query!(
            "UPDATE module_lessons SET is_published = true 
             WHERE module_id IN (SELECT id FROM course_modules WHERE course_id = $1)",
            course_id
        )
        .execute(pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(())
    }

    /// Get course with full structure
    pub async fn get_course_structure(
        pool: &PgPool,
        course_id: uuid::Uuid,
    ) -> Result<Option<CourseStructure>, String> {
        let course_row = sqlx::query!(
            "SELECT id, title, description FROM courses WHERE id = $1",
            course_id
        )
        .fetch_optional(pool)
        .await
        .map_err(|e| e.to_string())?;

        let course = match course_row {
            Some(r) => CourseStructure {
                id: r.id,
                title: r.title,
                description: r.description,
                modules: vec![],
            },
            None => return Ok(None),
        };

        // Get modules
        let module_rows = sqlx::query!(
            "SELECT id, course_id, title, description, order_index, is_published 
             FROM course_modules WHERE course_id = $1 ORDER BY order_index",
            course_id
        )
        .fetch_all(pool)
        .await
        .map_err(|e| e.to_string())?;

        let mut modules = Vec::new();
        for m in module_rows {
            // Get lessons for this module
            let lesson_rows = sqlx::query!(
                "SELECT id, module_id, title, lesson_type, content, video_url, 
                 duration_minutes, order_index, is_published, is_free
                 FROM module_lessons WHERE module_id = $1 ORDER BY order_index",
                m.id
            )
            .fetch_all(pool)
            .await
            .map_err(|e| e.to_string())?;

            let lessons: Vec<Lesson> = lesson_rows
                .into_iter()
                .map(|l| Lesson {
                    id: l.id,
                    title: l.title,
                    lesson_type: LessonType::Text, // Parse from string
                    content: l.content,
                    video_url: l.video_url,
                    duration_minutes: l.duration_minutes,
                    order: l.order_index,
                    is_published: l.is_published,
                    is_free: l.is_free,
                })
                .collect();

            // Get prerequisites
            let prereq_rows = sqlx::query!(
                "SELECT prerequisite_module_id FROM module_prerequisites WHERE module_id = $1",
                m.id
            )
            .fetch_all(pool)
            .await
            .map_err(|e| e.to_string())?;

            modules.push(Module {
                id: m.id,
                title: m.title,
                description: m.description,
                order: m.order_index,
                lessons,
                prerequisites: prereq_rows
                    .into_iter()
                    .map(|r| r.prerequisite_module_id)
                    .collect(),
                is_published: m.is_published,
            });
        }

        Ok(Some(CourseStructure { modules, ..course }))
    }

    /// Create course from template
    pub async fn create_from_template(
        pool: &PgPool,
        template_id: uuid::Uuid,
        institution_id: uuid::Uuid,
        creator_id: uuid::Uuid,
        new_title: &str,
    ) -> Result<CourseStructure, String> {
        // Get template structure
        // In production, fetch from template table

        // Create new course
        let course = create_course(pool, institution_id, creator_id, new_title, None).await?;

        // Copy modules and lessons from template
        // (simplified - in production would iterate and copy)

        Ok(course)
    }
}
