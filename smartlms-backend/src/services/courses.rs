// Course service - business logic for courses, modules, lessons
use crate::models::course::{
    Course, Module, Lesson, Enrollment, CourseProgress, CourseStatus,
    CreateCourseRequest, UpdateCourseRequest, CreateModuleRequest, CreateLessonRequest,
    ModuleWithLessons, CourseListResponse,
};
use crate::db::course as course_db;
use sqlx::PgPool;
use uuid::Uuid;

/// Create a new course
pub async fn create_course(
    pool: &PgPool,
    instructor_id: Uuid,
    req: &CreateCourseRequest,
) -> Result<Course, String> {
    // Validate title
    if req.title.is_empty() || req.title.len() > 200 {
        return Err("Course title must be 1-200 characters".to_string());
    }
    
    course_db::create(pool, instructor_id, req).await
        .map_err(|e| e.to_string())
}

/// Update course
pub async fn update_course(
    pool: &PgPool,
    course_id: Uuid,
    req: &UpdateCourseRequest,
) -> Result<Course, String> {
    course_db::update(pool, course_id, req).await
        .map_err(|e| e.to_string())
}

/// Publish course
pub async fn publish_course(pool: &PgPool, course_id: Uuid) -> Result<Course, String> {
    let mut req = UpdateCourseRequest {
        title: None,
        description: None,
        category: None,
        tags: None,
        status: Some(CourseStatus::Published),
        thumbnail_url: None,
    };
    
    update_course(pool, course_id, &mut req).await
}

/// Archive course
pub async fn archive_course(pool: &PgPool, course_id: Uuid) -> Result<Course, String> {
    let mut req = UpdateCourseRequest {
        title: None,
        description: None,
        category: None,
        tags: None,
        status: Some(CourseStatus::Archived),
        thumbnail_url: None,
    };
    
    update_course(pool, course_id, &mut req).await
}

/// Get course with modules and lessons
pub async fn get_course_detail(pool: &PgPool, course_id: Uuid) -> Result<CourseDetailResponse, String> {
    let course = course_db::find_by_id(pool, course_id).await
        .map_err(|e| e.to_string())?
        .ok_or("Course not found")?;
    
    let modules = course_db::get_modules(pool, course_id).await
        .map_err(|e| e.to_string())?;
    
    let mut modules_with_lessons = Vec::new();
    for module in modules {
        let lessons = course_db::get_lessons(pool, module.id).await
            .map_err(|e| e.to_string())?;
        modules_with_lessons.push(ModuleWithLessons { module, lessons });
    }
    
    Ok(CourseDetailResponse {
        course,
        modules: modules_with_lessons,
    })
}

/// List courses with filtering
pub async fn list_courses(
    pool: &PgPool,
    page: i64,
    per_page: i64,
    category: Option<&str>,
    status: Option<CourseStatus>,
    instructor_id: Option<Uuid>,
) -> Result<CourseListResponse, String> {
    let (courses, total) = course_db::list(pool, page, per_page, category, status, instructor_id)
        .await
        .map_err(|e| e.to_string())?;
    
    Ok(CourseListResponse {
        courses,
        total,
        page,
        per_page,
    })
}

/// Search courses
pub async fn search_courses(
    pool: &PgPool,
    query: &str,
    page: i64,
    per_page: i64,
) -> Result<CourseListResponse, String> {
    let (courses, total) = course_db::search(pool, query, page, per_page)
        .await
        .map_err(|e| e.to_string())?;
    
    Ok(CourseListResponse {
        courses,
        total,
        page,
        per_page,
    })
}

/// Create module
pub async fn create_module(
    pool: &PgPool,
    req: &CreateModuleRequest,
) -> Result<Module, String> {
    course_db::create_module(pool, req).await
        .map_err(|e| e.to_string())
}

/// Create lesson
pub async fn create_lesson(
    pool: &PgPool,
    req: &CreateLessonRequest,
) -> Result<Lesson, String> {
    // Validate
    if req.title.is_empty() {
        return Err("Lesson title required".to_string());
    }
    
    course_db::create_lesson(pool, req).await
        .map_err(|e| e.to_string())
}

/// Enroll user in course
pub async fn enroll_user(
    pool: &PgPool,
    user_id: Uuid,
    course_id: Uuid,
) -> Result<Enrollment, String> {
    // Check if already enrolled
    if course_db::get_enrollment(pool, user_id, course_id).await
        .map_err(|e| e.to_string())?
        .is_some() 
    {
        return Err("Already enrolled".to_string());
    }
    
    course_db::create_enrollment(pool, user_id, course_id).await
        .map_err(|e| e.to_string())
}

/// Get user progress in course
pub async fn get_progress(
    pool: &PgPool,
    user_id: Uuid,
    course_id: Uuid,
) -> Result<CourseProgress, String> {
    course_db::get_progress(pool, user_id, course_id).await
        .map_err(|e| e.to_string())?
        .ok_or("Not enrolled".to_string())
}

/// Mark lesson as completed
pub async fn complete_lesson(
    pool: &PgPool,
    user_id: Uuid,
    lesson_id: uuid::Uuid,
) -> Result<CourseProgress, String> {
    // Get lesson to find course
    let lesson = course_db::get_lesson_by_id(pool, lesson_id).await
        .map_err(|e| e.to_string())?
        .ok_or("Lesson not found")?;
    
    let module = course_db::get_module_by_id(pool, lesson.module_id).await
        .map_err(|e| e.to_string())?
        .ok_or("Module not found")?;
    
    // Update progress
    course_db::mark_lesson_complete(pool, user_id, module.course_id, lesson_id).await
        .map_err(|e| e.to_string())
}

/// Get course statistics
pub async fn get_course_stats(pool: &PgPool, course_id: Uuid) -> Result<CourseStats, String> {
    let course = course_db::find_by_id(pool, course_id).await
        .map_err(|e| e.to_string())?
        .ok_or("Course not found")?;
    
    let enrollment_count = course_db::count_enrollments(pool, course_id).await
        .map_err(|e| e.to_string())?;
    
    let avg_progress = course_db::avg_progress(pool, course_id).await
        .map_err(|e| e.to_string())?;
    
    let avg_rating = course_db::avg_rating(pool, course_id).await
        .map_err(|e| e.to_string())?;
    
    Ok(CourseStats {
        enrollment_count,
        avg_progress_percent: avg_progress,
        avg_rating,
        completion_rate: course.completion_rate,
    })
}

#[derive(Debug, serde::Serialize)]
pub struct CourseStats {
    pub enrollment_count: i64,
    pub avg_progress_percent: f32,
    pub avg_rating: f32,
    pub completion_rate: f32,
}