// Database operations for course groups
use crate::models::course_group::*;
use sqlx::{PgPool, Row};
use uuid::Uuid;

pub async fn create_group(
    pool: &PgPool,
    req: &CreateCourseGroupRequest,
    instructor_id: Uuid,
) -> Result<CourseGroup, sqlx::Error> {
    let id = Uuid::new_v4();
    let now = chrono::Utc::now();

    sqlx::query!(
        "INSERT INTO course_groups (id, course_id, instructor_id, name, description, max_students, student_count, is_active, created_at, updated_at)
         VALUES ($1, $2, $3, $4, $5, $6, 0, true, $7, $8)",
        id,
        req.course_id,
        instructor_id,
        req.name,
        req.description,
        req.max_students,
        now,
        now
    )
    .execute(pool)
    .await?;

    Ok(CourseGroup {
        id,
        course_id: req.course_id,
        instructor_id,
        name: req.name.clone(),
        description: req.description.clone(),
        max_students: req.max_students,
        student_count: 0,
        is_active: true,
        created_at: now,
        updated_at: now,
    })
}

pub async fn get_group_by_id(pool: &PgPool, id: Uuid) -> Result<Option<CourseGroup>, sqlx::Error> {
    let row = sqlx::query!(
        "SELECT id, course_id, instructor_id, name, description, max_students, student_count, is_active, created_at, updated_at
         FROM course_groups WHERE id = $1",
        id
    )
    .fetch_optional(pool)
    .await?;

    Ok(row.map(|r| CourseGroup {
        id: r.id,
        course_id: r.course_id,
        instructor_id: r.instructor_id,
        name: r.name,
        description: r.description,
        max_students: r.max_students,
        student_count: r.student_count,
        is_active: r.is_active,
        created_at: r.created_at,
        updated_at: r.updated_at,
    }))
}

pub async fn get_groups_by_course(
    pool: &PgPool,
    course_id: Uuid,
    page: i64,
    per_page: i64,
) -> Result<(Vec<CourseGroupWithInstructor>, i64), sqlx::Error> {
    let offset = (page - 1) * per_page;

    let rows = sqlx::query!(
        "SELECT cg.id, cg.course_id, cg.instructor_id, cg.name, cg.description, 
                cg.max_students, cg.student_count, cg.is_active, cg.created_at, cg.updated_at,
                u.first_name as instructor_first_name, u.last_name as instructor_last_name, u.email as instructor_email
         FROM course_groups cg
         JOIN users u ON cg.instructor_id = u.id
         WHERE cg.course_id = $1 AND cg.is_active = true
         ORDER BY cg.created_at DESC
         LIMIT $2 OFFSET $3",
        course_id,
        per_page,
        offset
    )
    .fetch_all(pool)
    .await?;

    let groups = rows
        .into_iter()
        .map(|r| CourseGroupWithInstructor {
            group: CourseGroup {
                id: r.id,
                course_id: r.course_id,
                instructor_id: r.instructor_id,
                name: r.name,
                description: r.description,
                max_students: r.max_students,
                student_count: r.student_count,
                is_active: r.is_active,
                created_at: r.created_at,
                updated_at: r.updated_at,
            },
            instructor_name: format!("{} {}", r.instructor_first_name, r.instructor_last_name),
            instructor_email: r.instructor_email,
        })
        .collect();

    // Get total count
    let total = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM course_groups WHERE course_id = $1 AND is_active = true"
    )
    .bind(course_id)
    .fetch_one(pool)
    .await?;

    Ok((groups, total))
}

pub async fn update_group(
    pool: &PgPool,
    group_id: Uuid,
    req: &UpdateCourseGroupRequest,
) -> Result<CourseGroup, sqlx::Error> {
    let now = chrono::Utc::now();

    sqlx::query!(
        "UPDATE course_groups SET
            name = COALESCE($1, name),
            description = COALESCE($2, description),
            max_students = COALESCE($3, max_students),
            is_active = COALESCE($4, is_active),
            updated_at = $5
         WHERE id = $6",
        req.name,
        req.description,
        req.max_students,
        req.is_active,
        now,
        group_id
    )
    .execute(pool)
    .await?;

    get_group_by_id(pool, group_id).await.map(|o| o.unwrap())
}

pub async fn delete_group(pool: &PgPool, group_id: Uuid) -> Result<(), sqlx::Error> {
    // Soft delete by setting is_active to false
    sqlx::query!(
        "UPDATE course_groups SET is_active = false, updated_at = NOW() WHERE id = $1",
        group_id
    )
    .execute(pool)
    .await?;

    Ok(())
}

// Group enrollment operations
pub async fn add_student_to_group(
    pool: &PgPool,
    group_id: Uuid,
    user_id: Uuid,
    enrolled_by: Uuid,
    notes: Option<String>,
) -> Result<CourseGroupEnrollment, sqlx::Error> {
    let id = Uuid::new_v4();
    let now = chrono::Utc::now();

    // Check if already enrolled
    let existing = sqlx::query!(
        "SELECT id FROM course_group_enrollments WHERE group_id = $1 AND user_id = $2 AND is_active = true",
        group_id,
        user_id
    )
    .fetch_optional(pool)
    .await?;

    if existing.is_some() {
        return Err(sqlx::Error::RowNotFound); // Or custom error
    }

    sqlx::query!(
        "INSERT INTO course_group_enrollments (id, group_id, user_id, enrolled_at, enrolled_by, is_active, notes)
         VALUES ($1, $2, $3, $4, $5, true, $6)",
        id,
        group_id,
        user_id,
        now,
        enrolled_by,
        notes
    )
    .execute(pool)
    .await?;

    // Update student count
    sqlx::query!(
        "UPDATE course_groups SET student_count = student_count + 1 WHERE id = $1",
        group_id
    )
    .execute(pool)
    .await?;

    Ok(CourseGroupEnrollment {
        id,
        group_id,
        user_id,
        enrolled_at: now,
        enrolled_by,
        is_active: true,
        notes,
    })
}

pub async fn remove_student_from_group(
    pool: &PgPool,
    group_id: Uuid,
    user_id: Uuid,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "UPDATE course_group_enrollments SET is_active = false WHERE group_id = $1 AND user_id = $2",
        group_id,
        user_id
    )
    .execute(pool)
    .await?;

    // Update student count
    sqlx::query!(
        "UPDATE course_groups SET student_count = student_count - 1 WHERE id = $1 AND student_count > 0",
        group_id
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn get_group_students(
    pool: &PgPool,
    group_id: Uuid,
) -> Result<Vec<GroupStudentInfo>, sqlx::Error> {
    let rows = sqlx::query!(
        "SELECT cge.user_id, u.first_name, u.last_name, u.email, cge.enrolled_at, 
                'active' as enrollment_status
         FROM course_group_enrollments cge
         JOIN users u ON cge.user_id = u.id
         WHERE cge.group_id = $1 AND cge.is_active = true
         ORDER BY cge.enrolled_at DESC",
        group_id
    )
    .fetch_all(pool)
    .await?;

    Ok(rows
        .into_iter()
        .map(|r| GroupStudentInfo {
            user_id: r.user_id,
            first_name: r.first_name,
            last_name: r.last_name,
            email: r.email,
            enrolled_at: r.enrolled_at,
            enrollment_status: r.enrollment_status,
        })
        .collect())
}

pub async fn get_user_groups(
    pool: &PgPool,
    user_id: Uuid,
    course_id: Option<Uuid>,
) -> Result<Vec<CourseGroup>, sqlx::Error> {
    let rows = if let Some(cid) = course_id {
        sqlx::query!(
            "SELECT cg.id, cg.course_id, cg.instructor_id, cg.name, cg.description,
                    cg.max_students, cg.student_count, cg.is_active, cg.created_at, cg.updated_at
             FROM course_groups cg
             JOIN course_group_enrollments cge ON cg.id = cge.group_id
             WHERE cge.user_id = $1 AND cg.course_id = $2 AND cge.is_active = true
             ORDER BY cg.created_at",
            user_id,
            cid
        )
        .fetch_all(pool)
        .await?
    } else {
        sqlx::query!(
            "SELECT cg.id, cg.course_id, cg.instructor_id, cg.name, cg.description,
                    cg.max_students, cg.student_count, cg.is_active, cg.created_at, cg.updated_at
             FROM course_groups cg
             JOIN course_group_enrollments cge ON cg.id = cge.group_id
             WHERE cge.user_id = $1 AND cge.is_active = true
             ORDER BY cg.created_at",
            user_id
        )
        .fetch_all(pool)
        .await?
    };

    Ok(rows
        .into_iter()
        .map(|r| CourseGroup {
            id: r.id,
            course_id: r.course_id,
            instructor_id: r.instructor_id,
            name: r.name,
            description: r.description,
            max_students: r.max_students,
            student_count: r.student_count,
            is_active: r.is_active,
            created_at: r.created_at,
            updated_at: r.updated_at,
        })
        .collect())
}

// Group session linking
pub async fn link_session_to_group(
    pool: &PgPool,
    group_id: Uuid,
    session_id: Uuid,
    is_mandatory: bool,
) -> Result<GroupSession, sqlx::Error> {
    let id = Uuid::new_v4();
    let now = chrono::Utc::now();

    sqlx::query!(
        "INSERT INTO group_sessions (id, group_id, session_id, is_mandatory, created_at)
         VALUES ($1, $2, $3, $4, $5)",
        id,
        group_id,
        session_id,
        is_mandatory,
        now
    )
    .execute(pool)
    .await?;

    Ok(GroupSession {
        id,
        group_id,
        session_id,
        is_mandatory,
        created_at: now,
    })
}

pub async fn get_group_sessions(
    pool: &PgPool,
    group_id: Uuid,
) -> Result<Vec<GroupSessionInfo>, sqlx::Error> {
    let rows = sqlx::query!(
        "SELECT ls.id as session_id, ls.title, ls.scheduled_start, ls.status
         FROM group_sessions gs
         JOIN live_sessions ls ON gs.session_id = ls.id
         WHERE gs.group_id = $1
         ORDER BY ls.scheduled_start DESC",
        group_id
    )
    .fetch_all(pool)
    .await?;

    Ok(rows
        .into_iter()
        .map(|r| GroupSessionInfo {
            session_id: r.session_id,
            title: r.title,
            scheduled_start: r.scheduled_start,
            status: r.status,
        })
        .collect())
}

// Group assessment linking
pub async fn link_assessment_to_group(
    pool: &PgPool,
    group_id: Uuid,
    assessment_id: Uuid,
    is_group_only: bool,
) -> Result<GroupAssessment, sqlx::Error> {
    let id = Uuid::new_v4();
    let now = chrono::Utc::now();

    sqlx::query!(
        "INSERT INTO group_assessments (id, group_id, assessment_id, is_group_only, created_at)
         VALUES ($1, $2, $3, $4, $5)",
        id,
        group_id,
        assessment_id,
        is_group_only,
        now
    )
    .execute(pool)
    .await?;

    Ok(GroupAssessment {
        id,
        group_id,
        assessment_id,
        is_group_only,
        created_at: now,
    })
}

pub async fn get_group_assessments(
    pool: &PgPool,
    group_id: Uuid,
) -> Result<Vec<GroupAssessmentInfo>, sqlx::Error> {
    let rows = sqlx::query!(
        "SELECT a.id as assessment_id, a.title, a.assessment_type, ga.is_group_only
         FROM group_assessments ga
         JOIN assessments a ON ga.assessment_id = a.id
         WHERE ga.group_id = $1
         ORDER BY a.created_at DESC",
        group_id
    )
    .fetch_all(pool)
    .await?;

    Ok(rows
        .into_iter()
        .map(|r| GroupAssessmentInfo {
            assessment_id: r.assessment_id,
            title: r.title,
            assessment_type: r.assessment_type,
            is_group_only: r.is_group_only,
        })
        .collect())
}
