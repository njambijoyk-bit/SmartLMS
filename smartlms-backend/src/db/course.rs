// Database operations for courses
use crate::models::course::*;
use sqlx::{PgPool, Row};
use uuid::Uuid;

pub async fn find_by_id(pool: &PgPool, id: Uuid) -> Result<Option<Course>, sqlx::Error> {
    let row = sqlx::query!(
        "SELECT id, title, description, short_description, thumbnail_url, status, 
                category, tags, instructor_id, enrollment_count, completion_rate, 
                rating, language, difficulty, duration_hours, created_at, updated_at, published_at
         FROM courses WHERE id = $1",
        id
    )
    .fetch_optional(pool)
    .await?;

    Ok(row.map(|r| Course {
        id: r.id,
        title: r.title,
        description: r.description,
        short_description: r.short_description,
        thumbnail_url: r.thumbnail_url,
        status: CourseStatus::Published, // Parse from DB
        category: r.category,
        tags: serde_json::from_str(&r.tags.unwrap_or_default()).unwrap_or_default(),
        instructor_id: r.instructor_id,
        enrollment_count: r.enrollment_count,
        completion_rate: r.completion_rate as f32,
        rating: r.rating as f32,
        language: r.language.unwrap_or_else(|| "en".to_string()),
        difficulty: r.difficulty.unwrap_or_else(|| "beginner".to_string()),
        duration_hours: r.duration_hours,
        created_at: r.created_at,
        updated_at: r.updated_at,
        published_at: r.published_at,
    }))
}

pub async fn create(
    pool: &PgPool,
    instructor_id: Uuid,
    req: &CreateCourseRequest,
) -> Result<Course, sqlx::Error> {
    let id = Uuid::new_v4();
    let now = chrono::Utc::now();
    let tags = serde_json::to_string(&req.tags.clone().unwrap_or_default()).unwrap_or_default();

    sqlx::query!(
        "INSERT INTO courses (id, title, description, category, tags, language, difficulty, 
                            instructor_id, status, created_at, updated_at)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)",
        id,
        req.title,
        req.description,
        req.category,
        tags,
        req.language.clone().unwrap_or_else(|| "en".to_string()),
        req.difficulty
            .clone()
            .unwrap_or_else(|| "beginner".to_string()),
        instructor_id,
        "draft",
        now,
        now
    )
    .execute(pool)
    .await?;

    Ok(Course {
        id,
        title: req.title.clone(),
        description: req.description.clone(),
        short_description: None,
        thumbnail_url: None,
        status: CourseStatus::Draft,
        category: req.category.clone(),
        tags: req.tags.clone().unwrap_or_default(),
        instructor_id,
        enrollment_count: 0,
        completion_rate: 0.0,
        rating: 0.0,
        language: req.language.clone().unwrap_or_else(|| "en".to_string()),
        difficulty: req
            .difficulty
            .clone()
            .unwrap_or_else(|| "beginner".to_string()),
        duration_hours: 0,
        created_at: now,
        updated_at: now,
        published_at: None,
    })
}

pub async fn update(
    pool: &PgPool,
    course_id: Uuid,
    req: &UpdateCourseRequest,
) -> Result<Course, sqlx::Error> {
    let now = chrono::Utc::now();

    sqlx::query!(
        "UPDATE courses SET 
            title = COALESCE($1, title),
            description = COALESCE($2, description),
            category = COALESCE($3, category),
            tags = COALESCE($4, tags),
            status = COALESCE($5, status),
            thumbnail_url = COALESCE($6, thumbnail_url),
            updated_at = $7
         WHERE id = $8",
        req.title,
        req.description,
        req.category,
        req.tags
            .as_ref()
            .map(|t| serde_json::to_string(t).unwrap_or_default()),
        req.status.map(|s| format!("{:?}", s).to_lowercase()),
        req.thumbnail_url,
        now,
        course_id
    )
    .execute(pool)
    .await?;

    find_by_id(pool, course_id).await.map(|o| o.unwrap())
}

pub async fn list(
    pool: &PgPool,
    page: i64,
    per_page: i64,
    category: Option<&str>,
    status: Option<CourseStatus>,
    instructor_id: Option<Uuid>,
) -> Result<(Vec<Course>, i64), sqlx::Error> {
    let offset = (page - 1) * per_page;

    let rows = sqlx::query!(
        "SELECT id, title, description, short_description, thumbnail_url, status,
                category, tags, instructor_id, enrollment_count, completion_rate,
                rating, language, difficulty, duration_hours, created_at, updated_at, published_at
         FROM courses WHERE status = 'published' ORDER BY created_at DESC LIMIT $1 OFFSET $2",
        per_page,
        offset
    )
    .fetch_all(pool)
    .await?;

    // Simplified - in production use proper query
    let courses: Vec<Course> = rows
        .into_iter()
        .map(|r| Course {
            id: r.id,
            title: r.title,
            description: r.description,
            short_description: r.short_description,
            thumbnail_url: r.thumbnail_url,
            status: CourseStatus::Published,
            category: r.category,
            tags: vec![],
            instructor_id: r.instructor_id,
            enrollment_count: r.enrollment_count,
            completion_rate: r.completion_rate as f32,
            rating: r.rating as f32,
            language: "en".to_string(),
            difficulty: "beginner".to_string(),
            duration_hours: r.duration_hours,
            created_at: r.created_at,
            updated_at: r.updated_at,
            published_at: r.published_at,
        })
        .collect();

    let total = courses.len() as i64; // Simplified

    Ok((courses, total))
}

pub async fn search(
    pool: &PgPool,
    query: &str,
    page: i64,
    per_page: i64,
) -> Result<(Vec<Course>, i64), sqlx::Error> {
    let offset = (page - 1) * per_page;
    let search = format!("%{}%", query);

    let rows = sqlx::query!(
        "SELECT id, title, description, short_description, thumbnail_url, status,
                category, tags, instructor_id, enrollment_count, completion_rate,
                rating, language, difficulty, duration_hours, created_at, updated_at, published_at
         FROM courses WHERE status = 'published' AND (title ILIKE $1 OR description ILIKE $1)
         ORDER BY created_at DESC LIMIT $2 OFFSET $3",
        search,
        per_page,
        offset
    )
    .fetch_all(pool)
    .await?;

    let courses: Vec<Course> = rows
        .into_iter()
        .map(|r| Course {
            id: r.id,
            title: r.title,
            description: r.description,
            short_description: r.short_description,
            thumbnail_url: r.thumbnail_url,
            status: CourseStatus::Published,
            category: r.category,
            tags: vec![],
            instructor_id: r.instructor_id,
            enrollment_count: r.enrollment_count,
            completion_rate: r.completion_rate as f32,
            rating: r.rating as f32,
            language: "en".to_string(),
            difficulty: "beginner".to_string(),
            duration_hours: r.duration_hours,
            created_at: r.created_at,
            updated_at: r.updated_at,
            published_at: r.published_at,
        })
        .collect();

    let total = courses.len() as i64;

    Ok((courses, total))
}

// Module operations
pub async fn get_modules(pool: &PgPool, course_id: Uuid) -> Result<Vec<Module>, sqlx::Error> {
    let rows = sqlx::query!(
        "SELECT id, course_id, title, description, order_index, duration_minutes, is_preview, created_at
         FROM modules WHERE course_id = $1 ORDER BY order_index",
        course_id
    )
    .fetch_all(pool)
    .await?;

    Ok(rows
        .into_iter()
        .map(|r| Module {
            id: r.id,
            course_id: r.course_id,
            title: r.title,
            description: r.description,
            order: r.order_index,
            duration_minutes: r.duration_minutes,
            is_preview: r.is_preview,
            created_at: r.created_at,
        })
        .collect())
}

pub async fn create_module(
    pool: &PgPool,
    req: &CreateModuleRequest,
) -> Result<Module, sqlx::Error> {
    let id = Uuid::new_v4();
    let now = chrono::Utc::now();
    let order = req.order.unwrap_or(0);

    sqlx::query!(
        "INSERT INTO modules (id, course_id, title, description, order_index, created_at)
         VALUES ($1, $2, $3, $4, $5, $6)",
        id,
        req.course_id,
        req.title,
        req.description,
        order,
        now
    )
    .execute(pool)
    .await?;

    Ok(Module {
        id,
        course_id: req.course_id,
        title: req.title.clone(),
        description: req.description.clone(),
        order,
        duration_minutes: 0,
        is_preview: false,
        created_at: now,
    })
}

// Lesson operations
pub async fn get_lessons(pool: &PgPool, module_id: Uuid) -> Result<Vec<Lesson>, sqlx::Error> {
    let rows = sqlx::query!(
        "SELECT id, module_id, title, lesson_type, content, video_url, video_duration_seconds,
                duration_minutes, order_index, is_preview, is_free, created_at
         FROM lessons WHERE module_id = $1 ORDER BY order_index",
        module_id
    )
    .fetch_all(pool)
    .await?;

    Ok(rows
        .into_iter()
        .map(|r| Lesson {
            id: r.id,
            module_id: r.module_id,
            title: r.title,
            lesson_type: LessonType::Text,
            content: r.content,
            video_url: r.video_url,
            video_duration_seconds: r.video_duration_seconds,
            duration_minutes: r.duration_minutes,
            order: r.order_index,
            is_preview: r.is_preview,
            is_free: r.is_free,
            created_at: r.created_at,
        })
        .collect())
}

pub async fn create_lesson(
    pool: &PgPool,
    req: &CreateLessonRequest,
) -> Result<Lesson, sqlx::Error> {
    let id = Uuid::new_v4();
    let now = chrono::Utc::now();
    let order = req.order.unwrap_or(0);

    sqlx::query!(
        "INSERT INTO lessons (id, module_id, title, lesson_type, content, video_url, 
                            order_index, is_preview, is_free, created_at)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)",
        id,
        req.module_id,
        req.title,
        format!("{:?}", req.lesson_type).to_lowercase(),
        req.content,
        req.video_url,
        order,
        req.is_preview.unwrap_or(false),
        req.is_free.unwrap_or(false),
        now
    )
    .execute(pool)
    .await?;

    Ok(Lesson {
        id,
        module_id: req.module_id,
        title: req.title.clone(),
        lesson_type: req.lesson_type,
        content: req.content.clone(),
        video_url: req.video_url.clone(),
        video_duration_seconds: None,
        duration_minutes: 0,
        order,
        is_preview: req.is_preview.unwrap_or(false),
        is_free: req.is_free.unwrap_or(false),
        created_at: now,
    })
}

pub async fn get_lesson_by_id(pool: &PgPool, id: Uuid) -> Result<Option<Lesson>, sqlx::Error> {
    let row = sqlx::query!(
        "SELECT id, module_id, title, lesson_type, content, video_url, video_duration_seconds,
                duration_minutes, order_index, is_preview, is_free, created_at
         FROM lessons WHERE id = $1",
        id
    )
    .fetch_optional(pool)
    .await?;

    Ok(row.map(|r| Lesson {
        id: r.id,
        module_id: r.module_id,
        title: r.title,
        lesson_type: LessonType::Text,
        content: r.content,
        video_url: r.video_url,
        video_duration_seconds: r.video_duration_seconds,
        duration_minutes: r.duration_minutes,
        order: r.order_index,
        is_preview: r.is_preview,
        is_free: r.is_free,
        created_at: r.created_at,
    }))
}

pub async fn get_module_by_id(pool: &PgPool, id: Uuid) -> Result<Option<Module>, sqlx::Error> {
    let row = sqlx::query!(
        "SELECT id, course_id, title, description, order_index, duration_minutes, is_preview, created_at
         FROM modules WHERE id = $1",
        id
    )
    .fetch_optional(pool)
    .await?;

    Ok(row.map(|r| Module {
        id: r.id,
        course_id: r.course_id,
        title: r.title,
        description: r.description,
        order: r.order_index,
        duration_minutes: r.duration_minutes,
        is_preview: r.is_preview,
        created_at: r.created_at,
    }))
}

// Enrollment operations
pub async fn get_enrollment(
    pool: &PgPool,
    user_id: Uuid,
    course_id: Uuid,
) -> Result<Option<Enrollment>, sqlx::Error> {
    let row = sqlx::query!(
        "SELECT id, course_id, user_id, progress_percent, completed_lessons, 
                started_at, completed_at, last_accessed_at
         FROM enrollments WHERE user_id = $1 AND course_id = $2",
        user_id,
        course_id
    )
    .fetch_optional(pool)
    .await?;

    Ok(row.map(|r| Enrollment {
        id: r.id,
        course_id: r.course_id,
        user_id: r.user_id,
        progress_percent: r.progress_percent as f32,
        completed_lessons: vec![],
        started_at: r.started_at,
        completed_at: r.completed_at,
        last_accessed_at: r.last_accessed_at,
    }))
}

pub async fn create_enrollment(
    pool: &PgPool,
    user_id: Uuid,
    course_id: Uuid,
) -> Result<Enrollment, sqlx::Error> {
    let id = Uuid::new_v4();
    let now = chrono::Utc::now();

    sqlx::query!(
        "INSERT INTO enrollments (id, course_id, user_id, progress_percent, started_at, last_accessed_at)
         VALUES ($1, $2, $3, 0, $4, $4)",
        id, course_id, user_id, now
    )
    .execute(pool)
    .await?;

    // Update enrollment count
    sqlx::query!(
        "UPDATE courses SET enrollment_count = enrollment_count + 1 WHERE id = $1",
        course_id
    )
    .execute(pool)
    .await?;

    Ok(Enrollment {
        id,
        course_id,
        user_id,
        progress_percent: 0.0,
        completed_lessons: vec![],
        started_at: now,
        completed_at: None,
        last_accessed_at: now,
    })
}

pub async fn get_progress(
    pool: &PgPool,
    user_id: Uuid,
    course_id: Uuid,
) -> Result<Option<CourseProgress>, sqlx::Error> {
    let row = sqlx::query!(
        "SELECT course_id, user_id, current_lesson_id, completed_modules, completed_lessons,
                time_spent_seconds, progress_percent
         FROM course_progress WHERE user_id = $1 AND course_id = $2",
        user_id,
        course_id
    )
    .fetch_optional(pool)
    .await?;

    Ok(row.map(|r| CourseProgress {
        course_id: r.course_id,
        user_id: r.user_id,
        current_lesson_id: r.current_lesson_id,
        completed_modules: vec![],
        completed_lessons: vec![],
        time_spent_seconds: r.time_spent_seconds,
        progress_percent: r.progress_percent as f32,
    }))
}

pub async fn mark_lesson_complete(
    pool: &PgPool,
    user_id: Uuid,
    course_id: Uuid,
    lesson_id: Uuid,
) -> Result<CourseProgress, sqlx::Error> {
    // Simplified - in production use proper DB update
    Ok(CourseProgress {
        course_id,
        user_id,
        current_lesson_id: Some(lesson_id),
        completed_modules: vec![],
        completed_lessons: vec![lesson_id],
        time_spent_seconds: 0,
        progress_percent: 10.0,
    })
}

pub async fn count_enrollments(pool: &PgPool, course_id: Uuid) -> Result<i64, sqlx::Error> {
    let row = sqlx::query!(
        "SELECT COUNT(*) as count FROM enrollments WHERE course_id = $1",
        course_id
    )
    .fetch_one(pool)
    .await?;
    Ok(row.count)
}

pub async fn avg_progress(pool: &PgPool, course_id: Uuid) -> Result<f32, sqlx::Error> {
    let row = sqlx::query!(
        "SELECT AVG(progress_percent) as avg FROM enrollments WHERE course_id = $1",
        course_id
    )
    .fetch_one(pool)
    .await?;
    Ok(row.avg.unwrap_or(0.0) as f32)
}

pub async fn avg_rating(pool: &PgPool, course_id: Uuid) -> Result<f32, sqlx::Error> {
    Ok(0.0) // Simplified
}

// Module update and delete operations
pub async fn update_module(
    pool: &PgPool,
    module_id: Uuid,
    req: &crate::models::course::UpdateModuleRequest,
) -> Result<Module, sqlx::Error> {
    use chrono::Utc;
    let now = Utc::now();

    sqlx::query!(
        "UPDATE modules SET 
            title = COALESCE($1, title),
            description = COALESCE($2, description),
            order_index = COALESCE($3, order_index),
            duration_minutes = COALESCE($4, duration_minutes),
            is_preview = COALESCE($5, is_preview),
            updated_at = $6
         WHERE id = $7",
        req.title,
        req.description,
        req.order,
        req.duration_minutes,
        req.is_preview,
        now,
        module_id
    )
    .execute(pool)
    .await?;

    get_module_by_id(pool, module_id).await.map(|o| o.unwrap())
}

pub async fn delete_module(pool: &PgPool, module_id: Uuid) -> Result<(), sqlx::Error> {
    sqlx::query!("DELETE FROM modules WHERE id = $1", module_id)
        .execute(pool)
        .await?;
    Ok(())
}

// Lesson update and delete operations
pub async fn update_lesson(
    pool: &PgPool,
    lesson_id: Uuid,
    req: &crate::models::course::UpdateLessonRequest,
) -> Result<Lesson, sqlx::Error> {
    use chrono::Utc;
    let now = Utc::now();

    sqlx::query!(
        "UPDATE lessons SET 
            title = COALESCE($1, title),
            lesson_type = COALESCE($2, lesson_type),
            content = COALESCE($3, content),
            video_url = COALESCE($4, video_url),
            duration_minutes = COALESCE($5, duration_minutes),
            order_index = COALESCE($6, order_index),
            is_preview = COALESCE($7, is_preview),
            is_free = COALESCE($8, is_free),
            updated_at = $9
         WHERE id = $10",
        req.title,
        req.lesson_type.as_ref().map(|t| format!("{:?}", t).to_lowercase()),
        req.content,
        req.video_url,
        req.duration_minutes,
        req.order,
        req.is_preview,
        req.is_free,
        now,
        lesson_id
    )
    .execute(pool)
    .await?;

    get_lesson_by_id(pool, lesson_id).await.map(|o| o.unwrap())
}

pub async fn delete_lesson(pool: &PgPool, lesson_id: Uuid) -> Result<(), sqlx::Error> {
    sqlx::query!("DELETE FROM lessons WHERE id = $1", lesson_id)
        .execute(pool)
        .await?;
    Ok(())
}

// Reorder modules
pub async fn reorder_modules(
    pool: &PgPool,
    items: &[crate::models::course::ReorderItem],
) -> Result<(), sqlx::Error> {
    for item in items {
        sqlx::query!(
            "UPDATE modules SET order_index = $1 WHERE id = $2",
            item.order,
            item.id
        )
        .execute(pool)
        .await?;
    }
    Ok(())
}

// Reorder lessons
pub async fn reorder_lessons(
    pool: &PgPool,
    items: &[crate::models::course::ReorderItem],
) -> Result<(), sqlx::Error> {
    for item in items {
        sqlx::query!(
            "UPDATE lessons SET order_index = $1 WHERE id = $2",
            item.order,
            item.id
        )
        .execute(pool)
        .await?;
    }
    Ok(())
}

// Delete course
pub async fn delete_course(pool: &PgPool, course_id: Uuid) -> Result<(), sqlx::Error> {
    sqlx::query!("DELETE FROM courses WHERE id = $1", course_id)
        .execute(pool)
        .await?;
    Ok(())
}

// Get instructor's courses
pub async fn get_instructor_courses(
    pool: &PgPool,
    instructor_id: Uuid,
    page: i64,
    per_page: i64,
) -> Result<(Vec<Course>, i64), sqlx::Error> {
    let offset = (page - 1) * per_page;

    let rows = sqlx::query!(
        "SELECT id, title, description, short_description, thumbnail_url, status,
                category, tags, instructor_id, enrollment_count, completion_rate,
                rating, language, difficulty, duration_hours, created_at, updated_at, published_at
         FROM courses WHERE instructor_id = $1 ORDER BY created_at DESC LIMIT $2 OFFSET $3",
        instructor_id,
        per_page,
        offset
    )
    .fetch_all(pool)
    .await?;

    let courses: Vec<Course> = rows
        .into_iter()
        .map(|r| Course {
            id: r.id,
            title: r.title,
            description: r.description,
            short_description: r.short_description,
            thumbnail_url: r.thumbnail_url,
            status: match r.status.as_str() {
                "published" => CourseStatus::Published,
                "archived" => CourseStatus::Archived,
                _ => CourseStatus::Draft,
            },
            category: r.category,
            tags: serde_json::from_str(&r.tags.unwrap_or_default()).unwrap_or_default(),
            instructor_id: r.instructor_id,
            enrollment_count: r.enrollment_count,
            completion_rate: r.completion_rate as f32,
            rating: r.rating as f32,
            language: r.language.unwrap_or_else(|| "en".to_string()),
            difficulty: r.difficulty.unwrap_or_else(|| "beginner".to_string()),
            duration_hours: r.duration_hours,
            created_at: r.created_at,
            updated_at: r.updated_at,
            published_at: r.published_at,
        })
        .collect();

    let total = courses.len() as i64;

    Ok((courses, total))
}
