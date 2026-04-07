// Assessment service - business logic for quizzes, exams, grades
use crate::db::assessment as assessment_db;
use crate::models::assessment::*;
use sqlx::PgPool;
use uuid::Uuid;

// Question Bank operations
pub async fn create_question_bank(
    pool: &PgPool,
    user_id: Uuid,
    req: &CreateQuestionBankRequest,
) -> Result<QuestionBank, String> {
    assessment_db::create_question_bank(pool, user_id, req)
        .await
        .map_err(|e| e.to_string())
}

pub async fn get_question_banks(
    pool: &PgPool,
    course_id: Option<Uuid>,
    page: i64,
    per_page: i64,
) -> Result<(Vec<QuestionBank>, i64), String> {
    assessment_db::list_question_banks(pool, course_id, page, per_page)
        .await
        .map_err(|e| e.to_string())
}

// Question operations
pub async fn create_question(
    pool: &PgPool,
    req: &CreateQuestionRequest,
) -> Result<Question, String> {
    // Validate question
    if req.question_text.is_empty() {
        return Err("Question text required".to_string());
    }

    if req.points.unwrap_or(0) <= 0 {
        return Err("Points must be positive".to_string());
    }

    assessment_db::create_question(pool, req)
        .await
        .map_err(|e| e.to_string())
}

pub async fn get_question(pool: &PgPool, question_id: Uuid) -> Result<Question, String> {
    assessment_db::get_question(pool, question_id)
        .await
        .map_err(|e| e.to_string())?
        .ok_or("Question not found".to_string())
}

pub async fn get_questions_in_bank(pool: &PgPool, bank_id: Uuid) -> Result<Vec<Question>, String> {
    assessment_db::get_questions_in_bank(pool, bank_id)
        .await
        .map_err(|e| e.to_string())
}

// Assessment operations
pub async fn create_assessment(
    pool: &PgPool,
    req: &CreateAssessmentRequest,
) -> Result<Assessment, String> {
    if req.title.is_empty() {
        return Err("Title required".to_string());
    }

    assessment_db::create_assessment(pool, req)
        .await
        .map_err(|e| e.to_string())
}

pub async fn get_assessment_detail(
    pool: &PgPool,
    assessment_id: Uuid,
) -> Result<AssessmentDetailResponse, String> {
    let assessment = assessment_db::get_assessment(pool, assessment_id)
        .await
        .map_err(|e| e.to_string())?
        .ok_or("Assessment not found")?;

    let questions = assessment_db::get_assessment_questions(pool, assessment_id)
        .await
        .map_err(|e| e.to_string())?;

    let attempt_count = assessment_db::count_attempts(pool, assessment_id)
        .await
        .map_err(|e| e.to_string())?;

    let avg_score = assessment_db::avg_assessment_score(pool, assessment_id)
        .await
        .map_err(|e| e.to_string())?;

    Ok(AssessmentDetailResponse {
        assessment,
        questions,
        attempt_count,
        avg_score,
    })
}

pub async fn publish_assessment(pool: &PgPool, assessment_id: Uuid) -> Result<Assessment, String> {
    assessment_db::publish_assessment(pool, assessment_id)
        .await
        .map_err(|e| e.to_string())
}

// Attempt operations
pub async fn start_attempt(
    pool: &PgPool,
    user_id: Uuid,
    assessment_id: Uuid,
) -> Result<Attempt, String> {
    // Check if user has reached max retries
    let assessment = assessment_db::get_assessment(pool, assessment_id)
        .await
        .map_err(|e| e.to_string())?
        .ok_or("Assessment not found")?;

    let attempt_count = assessment_db::count_user_attempts(pool, user_id, assessment_id)
        .await
        .map_err(|e| e.to_string())?;

    if let Some(max) = assessment.max_retries {
        if attempt_count >= max {
            return Err("Maximum attempts reached".to_string());
        }
    }

    assessment_db::create_attempt(pool, user_id, assessment_id)
        .await
        .map_err(|e| e.to_string())
}

pub async fn submit_answer(
    pool: &PgPool,
    user_id: Uuid,
    attempt_id: Uuid,
    req: &SubmitAnswerRequest,
) -> Result<Answer, String> {
    // Verify attempt belongs to user and is not submitted
    let attempt = assessment_db::get_attempt(pool, attempt_id)
        .await
        .map_err(|e| e.to_string())?
        .ok_or("Attempt not found")?;

    if attempt.user_id != user_id {
        return Err("Unauthorized".to_string());
    }

    if attempt.submitted_at.is_some() {
        return Err("Attempt already submitted".to_string());
    }

    // Get question to check answer
    let question = assessment_db::get_question(pool, req.question_id)
        .await
        .map_err(|e| e.to_string())?
        .ok_or("Question not found")?;

    // Check if answer is correct
    let is_correct = check_answer(&question, &req);

    let answer = Answer {
        id: Uuid::new_v4(),
        attempt_id,
        question_id: req.question_id,
        answer_text: req.answer_text.clone(),
        selected_options: req.selected_options.clone().unwrap_or_default(),
        is_correct: Some(is_correct),
        points_earned: if is_correct {
            Some(question.points)
        } else {
            Some(0)
        },
    };

    assessment_db::save_answer(pool, &answer)
        .await
        .map_err(|e| e.to_string())?;

    Ok(answer)
}

fn check_answer(question: &Question, req: &SubmitAnswerRequest) -> bool {
    match question.question_type {
        QuestionType::MultipleChoice | QuestionType::TrueFalse => {
            if let Some(selected) = &req.selected_options {
                // Check if correct option is selected
                question
                    .options
                    .iter()
                    .any(|o| o.is_correct && selected.contains(&o.id))
            } else {
                false
            }
        }
        QuestionType::ShortAnswer | QuestionType::LongAnswer => {
            // Simple exact match - in production use fuzzy matching
            req.answer_text
                .as_ref()
                .map(|a| a.trim().to_lowercase() == question.correct_answer.to_lowercase())
                .unwrap_or(false)
        }
        _ => false,
    }
}

pub async fn submit_attempt(
    pool: &PgPool,
    user_id: Uuid,
    attempt_id: Uuid,
) -> Result<AttemptDetailResponse, String> {
    // Get attempt
    let attempt = assessment_db::get_attempt(pool, attempt_id)
        .await
        .map_err(|e| e.to_string())?
        .ok_or("Attempt not found")?;

    if attempt.user_id != user_id {
        return Err("Unauthorized".to_string());
    }

    if attempt.submitted_at.is_some() {
        return Err("Already submitted".to_string());
    }

    // Calculate score
    let answers = assessment_db::get_attempt_answers(pool, attempt_id)
        .await
        .map_err(|e| e.to_string())?;

    let total_points: i32 = answers.iter().filter_map(|a| a.points_earned).sum();

    // Get assessment for max points
    let assessment = assessment_db::get_assessment(pool, attempt.assessment_id)
        .await
        .map_err(|e| e.to_string())?
        .ok_or("Assessment not found")?;

    // For now calculate from questions
    let questions = assessment_db::get_assessment_questions(pool, attempt.assessment_id)
        .await
        .map_err(|e| e.to_string())?;

    let max_points: i32 = questions.iter().map(|q| q.points).sum();

    let score = if max_points > 0 {
        (total_points as f32 / max_points as f32) * 100.0
    } else {
        0.0
    };

    let passed = score >= assessment.passing_score as f32;

    // Update attempt
    let updated = assessment_db::complete_attempt(pool, attempt_id, score, passed)
        .await
        .map_err(|e| e.to_string())?;

    Ok(AttemptDetailResponse {
        attempt: updated,
        answers,
    })
}

pub async fn get_attempt_result(
    pool: &PgPool,
    user_id: Uuid,
    attempt_id: Uuid,
) -> Result<AttemptDetailResponse, String> {
    let attempt = assessment_db::get_attempt(pool, attempt_id)
        .await
        .map_err(|e| e.to_string())?
        .ok_or("Attempt not found")?;

    if attempt.user_id != user_id {
        return Err("Unauthorized".to_string());
    }

    let answers = assessment_db::get_attempt_answers(pool, attempt_id)
        .await
        .map_err(|e| e.to_string())?;

    Ok(AttemptDetailResponse { attempt, answers })
}

// Gradebook operations
pub async fn get_gradebook(
    pool: &PgPool,
    course_id: Uuid,
    user_id: Option<Uuid>,
) -> Result<GradebookResponse, String> {
    let (grades, total) = assessment_db::get_gradebook(pool, course_id, user_id)
        .await
        .map_err(|e| e.to_string())?;

    let average = if total > 0 {
        grades.iter().map(|g| g.percent).sum::<f32>() / total as f32
    } else {
        0.0
    };

    let mut letter_distribution = std::collections::HashMap::new();
    for grade in &grades {
        if let Some(letter) = &grade.letter_grade {
            *letter_distribution.entry(letter.clone()).or_insert(0) += 1;
        }
    }

    Ok(GradebookResponse {
        grades,
        total,
        average,
        letter_distribution,
    })
}

pub async fn create_grade(
    pool: &PgPool,
    user_id: Uuid,
    course_id: Uuid,
    req: GradeSubmissionRequest,
    assessment_id: Option<Uuid>,
    graded_by: Uuid,
) -> Result<Grade, String> {
    let percent = (req.score / req.max_score) * 100.0;
    let letter = get_letter_grade(percent);

    let grade = Grade {
        id: Uuid::new_v4(),
        user_id,
        course_id,
        assessment_id,
        category: "assignment".to_string(),
        score: req.score,
        max_score: req.max_score,
        percent,
        letter_grade: Some(letter),
        feedback: req.feedback,
        graded_by: Some(graded_by),
        graded_at: Some(chrono::Utc::now()),
        created_at: chrono::Utc::now(),
    };

    assessment_db::create_grade(pool, &grade)
        .await
        .map_err(|e| e.to_string())
}

fn get_letter_grade(percent: f32) -> String {
    if percent >= 90.0 {
        "A"
    } else if percent >= 80.0 {
        "B"
    } else if percent >= 70.0 {
        "C"
    } else if percent >= 60.0 {
        "D"
    } else {
        "F"
    }
    .to_string()
}
