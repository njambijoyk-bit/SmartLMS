//! Course / module / lesson / enrollment service layer.
//!
//! Enforces role-based access rules on top of the raw db:: helpers. The
//! rules here mirror master ref §4 module 6 (Course Builder) and module 7
//! (Enrollment):
//!
//!   * admin     — full control, any course in the institution
//!   * instructor — can create + edit own courses and act as course staff
//!   * learner   — can enroll in published courses and mark lessons complete
//!
//! These rules are intentionally coarse in Phase 1; finer-grained ABAC
//! (e.g. per-course staff overrides) lands with the ABAC service in
//! Phase 2.

use sqlx::PgPool;

use crate::db;
use crate::middleware::auth::AuthUser;
use crate::models::course::{
    Course, CourseStatus, CreateCourseRequest, CreateLessonRequest, CreateModuleRequest,
    Enrollment, Lesson, Module, UpdateCourseRequest,
};
use crate::models::user::RoleCode;

#[derive(Debug, thiserror::Error)]
pub enum CourseError {
    #[error("permission denied")]
    Forbidden,
    #[error("course not found")]
    NotFound,
    #[error("course slug already taken")]
    SlugTaken,
    #[error("course is not published")]
    NotPublished,
    #[error("database error: {0}")]
    Db(#[from] sqlx::Error),
}

fn require_instructor_or_admin(user: &AuthUser) -> Result<(), CourseError> {
    if user.is_admin() || user.has_role(RoleCode::Instructor) {
        Ok(())
    } else {
        Err(CourseError::Forbidden)
    }
}

pub async fn create_course(
    pool: &PgPool,
    user: &AuthUser,
    req: CreateCourseRequest,
) -> Result<Course, CourseError> {
    require_instructor_or_admin(user)?;

    if db::course::find_by_slug(pool, &req.slug).await?.is_some() {
        return Err(CourseError::SlugTaken);
    }

    let course = db::course::create(pool, &req, user.id).await?;
    Ok(course)
}

pub async fn list_courses(
    pool: &PgPool,
    user: &AuthUser,
    page: i64,
    per_page: i64,
) -> Result<(Vec<Course>, i64), CourseError> {
    let (mut courses, total) = db::course::list(pool, page, per_page).await?;
    // Learners only see published courses; staff see all.
    if !(user.is_admin() || user.has_role(RoleCode::Instructor)) {
        courses.retain(|c| c.status == CourseStatus::Published.as_str());
    }
    Ok((courses, total))
}

pub async fn get_course(
    pool: &PgPool,
    user: &AuthUser,
    id: uuid::Uuid,
) -> Result<Course, CourseError> {
    let course = db::course::find_by_id(pool, id)
        .await?
        .ok_or(CourseError::NotFound)?;
    let is_staff =
        user.is_admin() || user.has_role(RoleCode::Instructor) || course.instructor_id == user.id;
    if course.status != CourseStatus::Published.as_str() && !is_staff {
        return Err(CourseError::NotFound);
    }
    Ok(course)
}

pub async fn update_course(
    pool: &PgPool,
    user: &AuthUser,
    id: uuid::Uuid,
    req: UpdateCourseRequest,
) -> Result<Course, CourseError> {
    let course = db::course::find_by_id(pool, id)
        .await?
        .ok_or(CourseError::NotFound)?;
    if !(user.is_admin() || course.instructor_id == user.id) {
        return Err(CourseError::Forbidden);
    }
    let updated = db::course::update(pool, id, &req)
        .await?
        .ok_or(CourseError::NotFound)?;
    Ok(updated)
}

pub async fn archive_course(
    pool: &PgPool,
    user: &AuthUser,
    id: uuid::Uuid,
) -> Result<(), CourseError> {
    let course = db::course::find_by_id(pool, id)
        .await?
        .ok_or(CourseError::NotFound)?;
    if !(user.is_admin() || course.instructor_id == user.id) {
        return Err(CourseError::Forbidden);
    }
    if !db::course::archive(pool, id).await? {
        return Err(CourseError::NotFound);
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// Modules / lessons
// ---------------------------------------------------------------------------

async fn ensure_course_staff(
    pool: &PgPool,
    user: &AuthUser,
    course_id: uuid::Uuid,
) -> Result<Course, CourseError> {
    let course = db::course::find_by_id(pool, course_id)
        .await?
        .ok_or(CourseError::NotFound)?;
    if !(user.is_admin() || course.instructor_id == user.id) {
        return Err(CourseError::Forbidden);
    }
    Ok(course)
}

pub async fn create_module(
    pool: &PgPool,
    user: &AuthUser,
    course_id: uuid::Uuid,
    req: CreateModuleRequest,
) -> Result<Module, CourseError> {
    ensure_course_staff(pool, user, course_id).await?;
    Ok(db::module_db::create_module(pool, course_id, &req).await?)
}

pub async fn list_modules(
    pool: &PgPool,
    course_id: uuid::Uuid,
) -> Result<Vec<Module>, CourseError> {
    Ok(db::module_db::list_modules(pool, course_id).await?)
}

pub async fn create_lesson(
    pool: &PgPool,
    user: &AuthUser,
    course_id: uuid::Uuid,
    module_id: uuid::Uuid,
    req: CreateLessonRequest,
) -> Result<Lesson, CourseError> {
    ensure_course_staff(pool, user, course_id).await?;
    Ok(db::module_db::create_lesson(pool, module_id, &req).await?)
}

pub async fn list_lessons(
    pool: &PgPool,
    module_id: uuid::Uuid,
) -> Result<Vec<Lesson>, CourseError> {
    Ok(db::module_db::list_lessons(pool, module_id).await?)
}

// ---------------------------------------------------------------------------
// Enrollments
// ---------------------------------------------------------------------------

pub async fn enroll_self(
    pool: &PgPool,
    user: &AuthUser,
    course_id: uuid::Uuid,
) -> Result<Enrollment, CourseError> {
    let course = db::course::find_by_id(pool, course_id)
        .await?
        .ok_or(CourseError::NotFound)?;
    // Instructors + admins can enroll in any course; learners only in
    // published courses.
    let is_staff =
        user.is_admin() || user.has_role(RoleCode::Instructor) || course.instructor_id == user.id;
    if course.status != CourseStatus::Published.as_str() && !is_staff {
        return Err(CourseError::NotPublished);
    }
    Ok(db::enrollment::enroll(pool, course_id, user.id).await?)
}

pub async fn list_my_enrollments(
    pool: &PgPool,
    user: &AuthUser,
) -> Result<Vec<Enrollment>, CourseError> {
    Ok(db::enrollment::list_for_user(pool, user.id).await?)
}

pub async fn list_course_enrollments(
    pool: &PgPool,
    user: &AuthUser,
    course_id: uuid::Uuid,
    page: i64,
    per_page: i64,
) -> Result<(Vec<Enrollment>, i64), CourseError> {
    ensure_course_staff(pool, user, course_id).await?;
    Ok(db::enrollment::list_for_course(pool, course_id, page, per_page).await?)
}

pub async fn drop_self(
    pool: &PgPool,
    user: &AuthUser,
    course_id: uuid::Uuid,
) -> Result<(), CourseError> {
    if !db::enrollment::drop_enrollment(pool, course_id, user.id).await? {
        return Err(CourseError::NotFound);
    }
    Ok(())
}

pub async fn complete_lesson(
    pool: &PgPool,
    user: &AuthUser,
    course_id: uuid::Uuid,
    lesson_id: uuid::Uuid,
) -> Result<i32, CourseError> {
    let enrollment = db::enrollment::find_for(pool, course_id, user.id)
        .await?
        .ok_or(CourseError::Forbidden)?;
    Ok(db::enrollment::complete_lesson(pool, enrollment.id, user.id, lesson_id).await?)
}
