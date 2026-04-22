// Database operations for assessments
use crate::models::assessment::*;
use sqlx::{PgPool, Row};
use uuid::Uuid;

// Question Bank operations
pub async fn create_question_bank(
    pool: &PgPool,
    user_id: Uuid,
    req: &CreateQuestionBankRequest,
) -> Result<QuestionBank, sqlx::Error> {
    let id = Uuid::new_v4();
    let now = chrono::Utc::now();

    sqlx::query!(
        "INSERT INTO question_banks (id, name, description, category, course_id, created_by, created_at)
         VALUES ($1, $2, $3, $4, $5, $6, $7)",
        id, req.name, req.description, req.category, req.course_id, user_id, now
    )
    .execute(pool)
    .await?;

    Ok(QuestionBank {
        id,
        name: req.name.clone(),
        description: req.description.clone(),
        category: req.category.clone(),
        course_id: req.course_id,
        question_count: 0,
        created_by: user_id,
        created_at: now,
    })
}

pub async fn list_question_banks(
    pool: &PgPool,
    course_id: Option<Uuid>,
    page: i64,
    per_page: i64,
) -> Result<(Vec<QuestionBank>, i64), sqlx::Error> {
    let offset = (page - 1) * per_page;

    let rows = sqlx::query!(
        "SELECT id, name, description, category, course_id, created_by, created_at
         FROM question_banks ORDER BY created_at DESC LIMIT $1 OFFSET $2",
        per_page,
        offset
    )
    .fetch_all(pool)
    .await?;

    let banks: Vec<QuestionBank> = rows
        .into_iter()
        .map(|r| QuestionBank {
            id: r.id,
            name: r.name,
            description: r.description,
            category: r.category,
            course_id: r.course_id,
            question_count: 0,
            created_by: r.created_by,
            created_at: r.created_at,
        })
        .collect();

    Ok((banks, banks.len() as i64))
}

// Question operations
pub async fn create_question(
    pool: &PgPool,
    req: &CreateQuestionRequest,
) -> Result<Question, sqlx::Error> {
    let id = Uuid::new_v4();
    let now = chrono::Utc::now();
    let points = req.points.unwrap_or(1);

    let options: Vec<QuestionOption> = req
        .options
        .as_ref()
        .map(|opts| {
            opts.iter()
                .enumerate()
                .map(|(i, o)| QuestionOption {
                    id: Uuid::new_v4(),
                    text: o.text.clone(),
                    is_correct: o.is_correct,
                })
                .collect()
        })
        .unwrap_or_default();

    sqlx::query!(
        "INSERT INTO questions (id, bank_id, question_text, question_type, correct_answer, explanation, points, created_at)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8)",
        id, req.bank_id, req.question_text, format!("{:?}", req.question_type).to_lowercase(),
        req.correct_answer, req.explanation, points, now
    )
    .execute(pool)
    .await?;

    Ok(Question {
        id,
        bank_id: req.bank_id,
        question_text: req.question_text.clone(),
        question_type: req.question_type,
        options,
        correct_answer: req.correct_answer.clone(),
        explanation: req.explanation.clone(),
        points,
        difficulty: req.difficulty.clone().unwrap_or_default(),
        tags: req.tags.clone().unwrap_or_default(),
        created_at: now,
    })
}

pub async fn get_question(pool: &PgPool, id: Uuid) -> Result<Option<Question>, sqlx::Error> {
    let row = sqlx::query!(
        "SELECT id, bank_id, question_text, question_type, correct_answer, explanation, points, created_at
         FROM questions WHERE id = $1",
        id
    )
    .fetch_optional(pool)
    .await?;

    Ok(row.map(|r| Question {
        id: r.id,
        bank_id: r.bank_id,
        question_text: r.question_text,
        question_type: QuestionType::MultipleChoice,
        options: vec![],
        correct_answer: r.correct_answer,
        explanation: r.explanation,
        points: r.points,
        difficulty: "medium".to_string(),
        tags: vec![],
        created_at: r.created_at,
    }))
}

pub async fn get_questions_in_bank(
    pool: &PgPool,
    bank_id: Uuid,
) -> Result<Vec<Question>, sqlx::Error> {
    let rows = sqlx::query!(
        "SELECT id, bank_id, question_text, question_type, correct_answer, explanation, points, created_at
         FROM questions WHERE bank_id = $1",
        bank_id
    )
    .fetch_all(pool)
    .await?;

    Ok(rows
        .into_iter()
        .map(|r| Question {
            id: r.id,
            bank_id: r.bank_id,
            question_text: r.question_text,
            question_type: QuestionType::MultipleChoice,
            options: vec![],
            correct_answer: r.correct_answer,
            explanation: r.explanation,
            points: r.points,
            difficulty: "medium".to_string(),
            tags: vec![],
            created_at: r.created_at,
        })
        .collect())
}

// Assessment operations
pub async fn create_assessment(
    pool: &PgPool,
    req: &CreateAssessmentRequest,
) -> Result<Assessment, sqlx::Error> {
    let id = Uuid::new_v4();
    let now = chrono::Utc::now();

    sqlx::query!(
        "INSERT INTO assessments (id, title, description, assessment_type, course_id, module_id,
         time_limit_minutes, passing_score, shuffle_questions, shuffle_options, show_results,
         allow_retries, max_retries, is_published, created_at, due_date)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, false, $14, $15)",
        id,
        req.title,
        req.description,
        format!("{:?}", req.assessment_type).to_lowercase(),
        req.course_id,
        req.module_id,
        req.time_limit_minutes,
        req.passing_score.unwrap_or(60),
        req.shuffle_questions.unwrap_or(false),
        req.shuffle_options.unwrap_or(false),
        req.show_results.unwrap_or(true),
        req.allow_retries.unwrap_or(false),
        req.max_retries,
        now,
        req.due_date
    )
    .execute(pool)
    .await?;

    Ok(Assessment {
        id,
        title: req.title.clone(),
        description: req.description.clone(),
        assessment_type: req.assessment_type,
        course_id: req.course_id,
        module_id: req.module_id,
        time_limit_minutes: req.time_limit_minutes,
        passing_score: req.passing_score.unwrap_or(60),
        shuffle_questions: req.shuffle_questions.unwrap_or(false),
        shuffle_options: req.shuffle_options.unwrap_or(false),
        show_results: req.show_results.unwrap_or(true),
        allow_retries: req.allow_retries.unwrap_or(false),
        max_retries: req.max_retries,
        is_published: false,
        created_at: now,
        due_date: req.due_date,
    })
}

pub async fn get_assessment(pool: &PgPool, id: Uuid) -> Result<Option<Assessment>, sqlx::Error> {
    let row = sqlx::query!(
        "SELECT id, title, description, assessment_type, course_id, module_id, time_limit_minutes,
         passing_score, shuffle_questions, shuffle_options, show_results, allow_retries, max_retries,
         is_published, created_at, due_date
         FROM assessments WHERE id = $1",
        id
    )
    .fetch_optional(pool)
    .await?;

    Ok(row.map(|r| Assessment {
        id: r.id,
        title: r.title,
        description: r.description,
        assessment_type: AssessmentType::Quiz,
        course_id: r.course_id,
        module_id: r.module_id,
        time_limit_minutes: r.time_limit_minutes,
        passing_score: r.passing_score,
        shuffle_questions: r.shuffle_questions,
        shuffle_options: r.shuffle_options,
        show_results: r.show_results,
        allow_retries: r.allow_retries,
        max_retries: r.max_retries,
        is_published: r.is_published,
        created_at: r.created_at,
        due_date: r.due_date,
    }))
}

pub async fn get_assessment_questions(
    pool: &PgPool,
    assessment_id: Uuid,
) -> Result<Vec<AssessmentQuestion>, sqlx::Error> {
    let rows = sqlx::query!(
        "SELECT aq.id, aq.assessment_id, aq.question_id, aq.order_index, aq.points,
                q.question_text, q.question_type, q.correct_answer, q.explanation, q.points as q_points
         FROM assessment_questions aq
         JOIN questions q ON aq.question_id = q.id
         WHERE aq.assessment_id = $1 ORDER BY aq.order_index",
        assessment_id
    )
    .fetch_all(pool)
    .await?;

    Ok(rows
        .into_iter()
        .map(|r| AssessmentQuestion {
            id: r.id,
            assessment_id: r.assessment_id,
            question_id: r.question_id,
            question: Question {
                id: r.question_id,
                bank_id: uuid::Uuid::nil(),
                question_text: r.question_text,
                question_type: QuestionType::MultipleChoice,
                options: vec![],
                correct_answer: r.correct_answer,
                explanation: r.explanation,
                points: r.q_points,
                difficulty: "medium".to_string(),
                tags: vec![],
                created_at: chrono::Utc::now(),
            },
            order: r.order_index,
            points: r.points,
        })
        .collect())
}

pub async fn publish_assessment(pool: &PgPool, id: Uuid) -> Result<Assessment, sqlx::Error> {
    sqlx::query!(
        "UPDATE assessments SET is_published = true WHERE id = $1",
        id
    )
    .execute(pool)
    .await?;

    get_assessment(pool, id).await.map(|o| o.unwrap())
}

pub async fn count_attempts(pool: &PgPool, assessment_id: Uuid) -> Result<i64, sqlx::Error> {
    let row = sqlx::query!(
        "SELECT COUNT(*) as count FROM attempts WHERE assessment_id = $1",
        assessment_id
    )
    .fetch_one(pool)
    .await?;
    Ok(row.count)
}

pub async fn avg_assessment_score(
    pool: &PgPool,
    assessment_id: Uuid,
) -> Result<Option<f32>, sqlx::Error> {
    let row = sqlx::query!(
        "SELECT AVG(score) as avg FROM attempts WHERE assessment_id = $1",
        assessment_id
    )
    .fetch_one(pool)
    .await?;
    Ok(row.avg.map(|v| v as f32))
}

// Attempt operations
pub async fn create_attempt(
    pool: &PgPool,
    user_id: Uuid,
    assessment_id: Uuid,
) -> Result<Attempt, sqlx::Error> {
    let id = Uuid::new_v4();
    let now = chrono::Utc::now();

    sqlx::query!(
        "INSERT INTO attempts (id, assessment_id, user_id, started_at, time_spent_seconds)
         VALUES ($1, $2, $3, $4, 0)",
        id,
        assessment_id,
        user_id,
        now
    )
    .execute(pool)
    .await?;

    Ok(Attempt {
        id,
        assessment_id,
        user_id,
        started_at: now,
        submitted_at: None,
        score: None,
        percent_score: None,
        passed: None,
        time_spent_seconds: 0,
    })
}

pub async fn get_attempt(pool: &PgPool, id: Uuid) -> Result<Option<Attempt>, sqlx::Error> {
    let row = sqlx::query!(
        "SELECT id, assessment_id, user_id, started_at, submitted_at, score, percent_score, passed, time_spent_seconds
         FROM attempts WHERE id = $1",
        id
    )
    .fetch_optional(pool)
    .await?;

    Ok(row.map(|r| Attempt {
        id: r.id,
        assessment_id: r.assessment_id,
        user_id: r.user_id,
        started_at: r.started_at,
        submitted_at: r.submitted_at,
        score: r.score,
        percent_score: r.percent_score.map(|v| v as f32),
        passed: r.passed,
        time_spent_seconds: r.time_spent_seconds,
    }))
}

pub async fn count_user_attempts(
    pool: &PgPool,
    user_id: Uuid,
    assessment_id: Uuid,
) -> Result<i64, sqlx::Error> {
    let row = sqlx::query!(
        "SELECT COUNT(*) as count FROM attempts WHERE user_id = $1 AND assessment_id = $2",
        user_id,
        assessment_id
    )
    .fetch_one(pool)
    .await?;
    Ok(row.count)
}

pub async fn save_answer(pool: &PgPool, answer: &Answer) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "INSERT INTO answers (id, attempt_id, question_id, answer_text, is_correct, points_earned)
         VALUES ($1, $2, $3, $4, $5, $6)",
        answer.id,
        answer.attempt_id,
        answer.question_id,
        answer.answer_text,
        answer.is_correct,
        answer.points_earned
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn get_attempt_answers(
    pool: &PgPool,
    attempt_id: Uuid,
) -> Result<Vec<Answer>, sqlx::Error> {
    let rows = sqlx::query!(
        "SELECT id, attempt_id, question_id, answer_text, is_correct, points_earned
         FROM answers WHERE attempt_id = $1",
        attempt_id
    )
    .fetch_all(pool)
    .await?;

    Ok(rows
        .into_iter()
        .map(|r| Answer {
            id: r.id,
            attempt_id: r.attempt_id,
            question_id: r.question_id,
            answer_text: r.answer_text,
            selected_options: vec![],
            is_correct: r.is_correct,
            points_earned: r.points_earned,
        })
        .collect())
}

pub async fn complete_attempt(
    pool: &PgPool,
    attempt_id: Uuid,
    score: f32,
    passed: bool,
) -> Result<Attempt, sqlx::Error> {
    let now = chrono::Utc::now();

    sqlx::query!(
        "UPDATE attempts SET submitted_at = $1, score = $2, percent_score = $2, passed = $3 WHERE id = $4",
        now, score, passed, attempt_id
    )
    .execute(pool)
    .await?;

    get_attempt(pool, attempt_id).await.map(|o| o.unwrap())
}

// Gradebook operations
pub async fn get_gradebook(
    pool: &PgPool,
    course_id: Uuid,
    user_id: Option<Uuid>,
) -> Result<(Vec<Grade>, i64), sqlx::Error> {
    let rows = sqlx::query!(
        "SELECT id, user_id, course_id, assessment_id, category, score, max_score, percent,
                letter_grade, feedback, graded_by, graded_at, created_at
         FROM grades WHERE course_id = $1",
        course_id
    )
    .fetch_all(pool)
    .await?;

    let grades: Vec<Grade> = rows
        .into_iter()
        .map(|r| Grade {
            id: r.id,
            user_id: r.user_id,
            course_id: r.course_id,
            assessment_id: r.assessment_id,
            category: r.category,
            score: r.score as f32,
            max_score: r.max_score as f32,
            percent: r.percent as f32,
            letter_grade: r.letter_grade,
            feedback: r.feedback,
            graded_by: r.graded_by,
            graded_at: r.graded_at,
            created_at: r.created_at,
        })
        .collect();

    Ok((grades, grades.len() as i64))
}

pub async fn create_grade(pool: &PgPool, grade: &Grade) -> Result<Grade, sqlx::Error> {
    sqlx::query!(
        "INSERT INTO grades (id, user_id, course_id, assessment_id, category, score, max_score,
         percent, letter_grade, feedback, graded_by, graded_at, created_at)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)",
        grade.id,
        grade.user_id,
        grade.course_id,
        grade.assessment_id,
        grade.category,
        grade.score,
        grade.max_score,
        grade.percent,
        grade.letter_grade,
        grade.feedback,
        grade.graded_by,
        grade.graded_at,
        grade.created_at
    )
    .execute(pool)
    .await?;

    Ok(grade.clone())
}

// Additional Assessment CRUD operations
pub async fn update_assessment(
    pool: &PgPool,
    id: Uuid,
    req: &UpdateAssessmentRequest,
) -> Result<Assessment, sqlx::Error> {
    let now = chrono::Utc::now();

    sqlx::query!(
        "UPDATE assessments SET 
         title = COALESCE($1, title),
         description = COALESCE($2, description),
         time_limit_minutes = COALESCE($3, time_limit_minutes),
         passing_score = COALESCE($4, passing_score),
         shuffle_questions = COALESCE($5, shuffle_questions),
         shuffle_options = COALESCE($6, shuffle_options),
         show_results = COALESCE($7, show_results),
         allow_retries = COALESCE($8, allow_retries),
         max_retries = COALESCE($9, max_retries),
         due_date = COALESCE($10, due_date),
         updated_at = $11
         WHERE id = $12",
        req.title,
        req.description,
        req.time_limit_minutes,
        req.passing_score,
        req.shuffle_questions,
        req.shuffle_options,
        req.show_results,
        req.allow_retries,
        req.max_retries,
        req.due_date,
        now,
        id
    )
    .execute(pool)
    .await?;

    get_assessment(pool, id).await.map(|o| o.unwrap())
}

pub async fn delete_assessment(pool: &PgPool, id: Uuid) -> Result<(), sqlx::Error> {
    sqlx::query!("DELETE FROM assessments WHERE id = $1", id)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn list_assessments(
    pool: &PgPool,
    course_id: Option<Uuid>,
    course_group_id: Option<Uuid>,
    page: i64,
    per_page: i64,
) -> Result<(Vec<Assessment>, i64), sqlx::Error> {
    let offset = (page - 1) * per_page;

    let rows = if let Some(cid) = course_id {
        if let Some(gid) = course_group_id {
            sqlx::query!(
                "SELECT id, title, description, assessment_type, course_id, module_id, 
                        time_limit_minutes, passing_score, shuffle_questions, shuffle_options, 
                        show_results, allow_retries, max_retries, is_published, created_at, due_date
                 FROM assessments 
                 WHERE course_id = $1 AND course_group_id = $2
                 ORDER BY created_at DESC LIMIT $3 OFFSET $4",
                cid, gid, per_page, offset
            )
            .fetch_all(pool)
            .await?
        } else {
            sqlx::query!(
                "SELECT id, title, description, assessment_type, course_id, module_id, 
                        time_limit_minutes, passing_score, shuffle_questions, shuffle_options, 
                        show_results, allow_retries, max_retries, is_published, created_at, due_date
                 FROM assessments 
                 WHERE course_id = $1
                 ORDER BY created_at DESC LIMIT $2 OFFSET $3",
                cid, per_page, offset
            )
            .fetch_all(pool)
            .await?
        }
    } else {
        sqlx::query!(
            "SELECT id, title, description, assessment_type, course_id, module_id, 
                    time_limit_minutes, passing_score, shuffle_questions, shuffle_options, 
                    show_results, allow_retries, max_retries, is_published, created_at, due_date
             FROM assessments 
             ORDER BY created_at DESC LIMIT $1 OFFSET $2",
            per_page, offset
        )
        .fetch_all(pool)
        .await?
    };

    let assessments: Vec<Assessment> = rows
        .into_iter()
        .map(|r| Assessment {
            id: r.id,
            title: r.title,
            description: r.description,
            assessment_type: AssessmentType::Quiz,
            course_id: r.course_id,
            course_group_id: None,
            module_id: r.module_id,
            created_by: uuid::Uuid::nil(),
            time_limit_minutes: r.time_limit_minutes,
            passing_score: r.passing_score,
            shuffle_questions: r.shuffle_questions,
            shuffle_options: r.shuffle_options,
            show_results: r.show_results,
            show_results_immediately: true,
            allow_retries: r.allow_retries,
            max_retries: r.max_retries,
            require_lockdown_browser: false,
            allow_late_submission: false,
            late_penalty_percent: 0,
            is_published: r.is_published,
            status: "draft".to_string(),
            start_time: None,
            due_date: r.due_date,
            end_time: None,
            created_at: r.created_at,
            updated_at: r.created_at,
        })
        .collect();

    Ok((assessments, assessments.len() as i64))
}

pub async fn get_user_attempts(
    pool: &PgPool,
    user_id: Uuid,
    assessment_id: Uuid,
) -> Result<Vec<Attempt>, sqlx::Error> {
    let rows = sqlx::query!(
        "SELECT id, assessment_id, user_id, started_at, submitted_at, score, percent_score, 
                passed, time_spent_seconds
         FROM attempts 
         WHERE user_id = $1 AND assessment_id = $2
         ORDER BY started_at DESC",
        user_id,
        assessment_id
    )
    .fetch_all(pool)
    .await?;

    Ok(rows
        .into_iter()
        .map(|r| Attempt {
            id: r.id,
            assessment_id: r.assessment_id,
            user_id: r.user_id,
            started_at: r.started_at,
            submitted_at: r.submitted_at,
            score: r.score,
            percent_score: r.percent_score.map(|v| v as f32),
            passed: r.passed,
            time_spent_seconds: r.time_spent_seconds,
            status: "submitted".to_string(),
            is_late: false,
            lockdown_session_id: None,
            ip_address: None,
            attempt_number: 0,
        })
        .collect())
}

pub async fn add_question_to_assessment(
    pool: &PgPool,
    assessment_id: Uuid,
    question_id: Uuid,
    points: i32,
) -> Result<AssessmentQuestion, sqlx::Error> {
    let id = Uuid::new_v4();

    // Get max order
    let max_order = sqlx::query!("SELECT COALESCE(MAX(order_index), -1) as max_order FROM assessment_questions WHERE assessment_id = $1", assessment_id)
        .fetch_one(pool)
        .await?;

    let order = max_order.max_order + 1;

    sqlx::query!(
        "INSERT INTO assessment_questions (id, assessment_id, question_id, order_index, points)
         VALUES ($1, $2, $3, $4, $5)",
        id, assessment_id, question_id, order, points
    )
    .execute(pool)
    .await?;

    // Fetch the question details
    let q = sqlx::query!(
        "SELECT id, bank_id, question_text, question_type, correct_answer, explanation, points as q_points, created_at
         FROM questions WHERE id = $1",
        question_id
    )
    .fetch_one(pool)
    .await?;

    Ok(AssessmentQuestion {
        id,
        assessment_id,
        question_id,
        question: Question {
            id: q.id,
            bank_id: q.bank_id,
            question_text: q.question_text,
            question_type: QuestionType::MultipleChoice,
            options: vec![],
            correct_answer: q.correct_answer,
            explanation: q.explanation,
            points: q.q_points,
            difficulty: "medium".to_string(),
            tags: vec![],
            created_at: q.created_at,
        },
        order,
        points,
    })
}

pub async fn remove_question_from_assessment(
    pool: &PgPool,
    assessment_id: Uuid,
    question_id: Uuid,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "DELETE FROM assessment_questions WHERE assessment_id = $1 AND question_id = $2",
        assessment_id, question_id
    )
    .execute(pool)
    .await?;
    Ok(())
}

#[derive(Debug, Deserialize)]
pub struct UpdateAssessmentRequest {
    pub title: Option<String>,
    pub description: Option<Option<String>>,
    pub time_limit_minutes: Option<Option<i32>>,
    pub passing_score: Option<i32>,
    pub shuffle_questions: Option<bool>,
    pub shuffle_options: Option<bool>,
    pub show_results: Option<bool>,
    pub allow_retries: Option<bool>,
    pub max_retries: Option<Option<i32>>,
    pub due_date: Option<Option<chrono::DateTime<chrono::Utc>>>,
}
