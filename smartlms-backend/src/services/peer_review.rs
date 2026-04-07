// Peer Learning & Structured Peer Review — Calibrated peer assessment with grading
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// A peer review assignment configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeerReviewAssignment {
    pub id: Uuid,
    pub institution_id: Uuid,
    pub course_id: Uuid,
    pub title: String,
    pub description: String,
    pub submission_deadline: DateTime<Utc>,
    pub review_deadline: DateTime<Utc>,
    pub reviews_per_student: u8,  // N peers each student must review
    pub rubric: Vec<RubricCriterion>,
    pub grading_weights: GradingWeights,
    pub calibration_enabled: bool,
    pub anonymous_reviews: bool,
    pub status: ReviewAssignmentStatus,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ReviewAssignmentStatus {
    Draft,
    SubmissionOpen,
    ReviewOpen,
    Completed,
    Archived,
}

/// A structured rubric criterion
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RubricCriterion {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub max_score: f64,
    pub levels: Vec<RubricLevel>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RubricLevel {
    pub score: f64,
    pub label: String,
    pub description: String,
}

/// Weighting: student's own work + quality of reviews given
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GradingWeights {
    pub submission_weight: f64,  // e.g. 0.70
    pub review_quality_weight: f64, // e.g. 0.30
}

/// A student's submission for peer review
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeerReviewSubmission {
    pub id: Uuid,
    pub assignment_id: Uuid,
    pub student_id: Uuid,
    pub content: String,
    pub file_urls: Vec<String>,
    pub submitted_at: DateTime<Utc>,
    pub is_late: bool,
}

/// An assignment of reviewer → submission to review
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReviewAssignment {
    pub id: Uuid,
    pub peer_review_assignment_id: Uuid,
    pub reviewer_id: Uuid,
    pub submission_id: Uuid,
    /// Anonymised — reviewer never sees submitter identity directly
    pub submitter_alias: String,
    pub assigned_at: DateTime<Utc>,
    pub completed: bool,
}

/// A completed peer review
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeerReview {
    pub id: Uuid,
    pub review_assignment_id: Uuid,
    pub reviewer_id: Uuid,
    pub submission_id: Uuid,
    pub criterion_scores: Vec<CriterionScore>,
    pub overall_comment: String,
    pub submitted_at: DateTime<Utc>,
    pub calibration_adjustment: Option<f64>, // applied post-calibration
    pub flagged_as_random: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CriterionScore {
    pub criterion_id: Uuid,
    pub score: f64,
    pub comment: String,
}

/// Calibration: instructor grades a sample with gold-standard rubric
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalibrationSample {
    pub id: Uuid,
    pub assignment_id: Uuid,
    pub submission_id: Uuid,
    pub instructor_scores: Vec<CriterionScore>,
    pub created_by: Uuid, // instructor
    pub created_at: DateTime<Utc>,
}

/// Rater bias analysis result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RaterBiasAnalysis {
    pub reviewer_id: Uuid,
    pub bias_type: Option<RaterBiasType>,
    pub adjustment_factor: f64, // multiply raw scores by this
    pub confidence: f64,
    pub flagged_for_review: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RaterBiasType {
    Generous,   // consistently rates higher than calibration
    Harsh,      // consistently rates lower than calibration
    Random,     // shows no correlation with calibration — possible gaming
    Lenient,    // rates everyone maximum
}

/// Final calculated grade for a student
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeerReviewGrade {
    pub student_id: Uuid,
    pub assignment_id: Uuid,
    pub submission_score: f64,       // from peers reviewing their work
    pub review_quality_score: f64,   // how well they reviewed others (vs calibration)
    pub final_grade: f64,
    pub grade_breakdown: GradeBreakdown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GradeBreakdown {
    pub raw_peer_scores: Vec<f64>,
    pub calibrated_peer_scores: Vec<f64>,
    pub avg_peer_score: f64,
    pub review_quality: f64,
    pub weights_applied: GradingWeights,
}

/// Peer tutor marketplace entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeerTutor {
    pub id: Uuid,
    pub student_id: Uuid,
    pub institution_id: Uuid,
    pub course_ids: Vec<Uuid>,
    pub bio: String,
    pub gpa: f64,
    pub hourly_rate: Option<f64>, // None = volunteer
    pub availability_slots: Vec<AvailabilitySlot>,
    pub average_rating: f64,
    pub sessions_completed: i32,
    pub is_active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AvailabilitySlot {
    pub day_of_week: String,
    pub start_time: String,
    pub end_time: String,
}

pub struct PeerReviewService;

impl PeerReviewService {
    /// Create a peer review assignment
    pub async fn create_assignment(
        _pool: &sqlx::PgPool,
        assignment: PeerReviewAssignment,
    ) -> Result<PeerReviewAssignment, sqlx::Error> {
        // TODO: INSERT INTO peer_review_assignments
        Ok(assignment)
    }

    /// Student submits their work
    pub async fn submit_work(
        _pool: &sqlx::PgPool,
        assignment_id: Uuid,
        student_id: Uuid,
        content: String,
        file_urls: Vec<String>,
        deadline: DateTime<Utc>,
    ) -> Result<PeerReviewSubmission, sqlx::Error> {
        let now = Utc::now();
        Ok(PeerReviewSubmission {
            id: Uuid::new_v4(),
            assignment_id,
            student_id,
            content,
            file_urls,
            submitted_at: now,
            is_late: now > deadline,
        })
    }

    /// Assign peers to review each other's work
    /// Uses a round-robin avoiding self-assignment and friend groups
    pub async fn assign_reviews(
        _pool: &sqlx::PgPool,
        assignment_id: Uuid,
        reviews_per_student: u8,
    ) -> Result<Vec<ReviewAssignment>, sqlx::Error> {
        let _ = (assignment_id, reviews_per_student);
        // TODO: SELECT submissions, shuffle, assign N reviews per student
        //       avoiding: same reviewer reviewing same person twice,
        //       known social connections (if social graph is available)
        Ok(vec![])
    }

    /// Student submits a peer review
    pub async fn submit_review(
        _pool: &sqlx::PgPool,
        review: PeerReview,
    ) -> Result<PeerReview, sqlx::Error> {
        // TODO: INSERT INTO peer_reviews
        //       Mark ReviewAssignment as completed
        Ok(review)
    }

    /// Instructor sets calibration gold standard
    pub async fn set_calibration(
        _pool: &sqlx::PgPool,
        sample: CalibrationSample,
    ) -> Result<CalibrationSample, sqlx::Error> {
        // TODO: INSERT INTO calibration_samples
        Ok(sample)
    }

    /// Run calibration: compare all reviews against gold standard, compute bias
    pub async fn run_calibration(
        _pool: &sqlx::PgPool,
        assignment_id: Uuid,
    ) -> Result<Vec<RaterBiasAnalysis>, sqlx::Error> {
        let _ = assignment_id;
        // Algorithm:
        // 1. For each reviewer who reviewed a calibration sample:
        //    compute Pearson correlation of their scores vs instructor scores
        // 2. Compute mean bias (generous/harsh)
        // 3. Flag reviewers with |correlation| < 0.3 as Random
        // 4. Set adjustment_factor = instructor_avg / reviewer_avg
        Ok(vec![])
    }

    /// Detect random raters (low variance, always max/min, no correlation)
    pub fn detect_random_rater(review_scores: &[f64], calibration_scores: &[f64]) -> bool {
        if review_scores.len() < 3 || calibration_scores.len() < 3 {
            return false;
        }
        let variance: f64 = {
            let mean = review_scores.iter().sum::<f64>() / review_scores.len() as f64;
            review_scores.iter().map(|s| (s - mean).powi(2)).sum::<f64>()
                / review_scores.len() as f64
        };
        // Very low variance (< 0.1) suggests always clicking same score
        if variance < 0.1 {
            return true;
        }
        // Compute Pearson correlation with calibration
        let n = review_scores.len().min(calibration_scores.len()) as f64;
        let mean_r = review_scores.iter().sum::<f64>() / n;
        let mean_c = calibration_scores.iter().sum::<f64>() / n;
        let num: f64 = review_scores
            .iter()
            .zip(calibration_scores.iter())
            .map(|(r, c)| (r - mean_r) * (c - mean_c))
            .sum();
        let denom_r: f64 = review_scores.iter().map(|r| (r - mean_r).powi(2)).sum::<f64>().sqrt();
        let denom_c: f64 = calibration_scores.iter().map(|c| (c - mean_c).powi(2)).sum::<f64>().sqrt();
        if denom_r * denom_c == 0.0 {
            return true;
        }
        let correlation = num / (denom_r * denom_c);
        correlation.abs() < 0.3
    }

    /// Calculate final grades after calibration
    pub async fn calculate_grades(
        _pool: &sqlx::PgPool,
        assignment_id: Uuid,
        weights: GradingWeights,
    ) -> Result<Vec<PeerReviewGrade>, sqlx::Error> {
        let _ = (assignment_id, &weights);
        // TODO: For each student:
        //   1. Collect calibrated peer scores on their submission
        //   2. Average them → submission_score
        //   3. Compute review quality score (compare their reviews vs calibration)
        //   4. final_grade = submission_score * weights.submission_weight
        //                  + review_quality * weights.review_quality_weight
        Ok(vec![])
    }

    /// Register as a peer tutor
    pub async fn register_tutor(
        _pool: &sqlx::PgPool,
        tutor: PeerTutor,
    ) -> Result<PeerTutor, sqlx::Error> {
        // TODO: INSERT INTO peer_tutors
        Ok(tutor)
    }

    /// Search peer tutors by course
    pub async fn find_tutors(
        _pool: &sqlx::PgPool,
        course_id: Uuid,
        institution_id: Uuid,
    ) -> Result<Vec<PeerTutor>, sqlx::Error> {
        let _ = (course_id, institution_id);
        // TODO: SELECT * FROM peer_tutors WHERE $1 = ANY(course_ids)
        Ok(vec![])
    }
}
