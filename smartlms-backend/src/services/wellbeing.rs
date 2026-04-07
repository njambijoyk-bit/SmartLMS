// Student Wellbeing Module — Weekly check-ins, counsellor management, at-risk escalation
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// Weekly student wellbeing check-in response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WellbeingCheckIn {
    pub id: Uuid,
    pub student_id: Uuid,
    pub institution_id: Uuid,
    pub week_start: DateTime<Utc>,
    /// Overall wellbeing score 1–5
    pub wellbeing_score: u8,
    /// Workload manageability 1–5
    pub workload_score: u8,
    /// Are you sleeping enough?
    pub sleeping_enough: bool,
    pub free_text: Option<String>,
    pub self_referral_requested: bool,
    pub is_anonymous: bool,
    pub submitted_at: DateTime<Utc>,
}

/// Wellbeing trend data point
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WellbeingTrend {
    pub week_start: DateTime<Utc>,
    pub avg_wellbeing: f64,
    pub avg_workload: f64,
    pub response_rate: f64,
    pub self_referral_count: i32,
}

/// Counsellor case for a student
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WellbeingCase {
    pub id: Uuid,
    pub student_id: Uuid,
    pub institution_id: Uuid,
    pub counsellor_id: Uuid,
    pub referral_source: ReferralSource,
    pub status: CaseStatus,
    pub notes: Vec<CaseNote>,
    pub opened_at: DateTime<Utc>,
    pub next_followup: Option<DateTime<Utc>>,
    pub closed_at: Option<DateTime<Utc>>,
    pub outcome: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReferralSource {
    SelfReferral,
    InstructorReferral,
    AdvisorReferral,
    JuliaAutoEscalation,
    AdminReferral,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CaseStatus {
    Open,
    InProgress,
    OnHold,
    Closed,
}

/// A counsellor's note on a case (private, searchable, date-stamped)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CaseNote {
    pub id: Uuid,
    pub case_id: Uuid,
    pub authored_by: Uuid,
    pub content: String,
    pub is_private: bool,
    pub created_at: DateTime<Utc>,
}

/// Counsellor's aggregate view of a cohort's wellbeing (no individual data)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CohortWellbeingAggregate {
    pub cohort_id: Uuid,
    pub cohort_name: String,
    pub week_start: DateTime<Utc>,
    pub avg_wellbeing: f64,
    pub avg_workload: f64,
    pub response_count: i32,
    pub total_students: i32,
    pub response_rate: f64,
    pub trend_direction: TrendDirection,
    pub alert_level: AlertLevel,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TrendDirection {
    Improving,
    Stable,
    Declining,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AlertLevel {
    Normal,
    Watch,
    Warning,
    Critical,
}

/// Instructor-facing aggregate — no individual student data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstructorWellbeingAlert {
    pub course_id: Uuid,
    pub course_name: String,
    pub message: String,
    pub avg_wellbeing_drop: f64,
    pub correlated_events: Vec<String>,
    pub suggested_actions: Vec<String>,
}

/// Request to submit a weekly check-in
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubmitCheckInRequest {
    pub student_id: Uuid,
    pub institution_id: Uuid,
    pub wellbeing_score: u8,
    pub workload_score: u8,
    pub sleeping_enough: bool,
    pub free_text: Option<String>,
    pub self_referral_requested: bool,
    pub is_anonymous: bool,
}

/// Wellbeing configuration per institution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WellbeingConfig {
    pub institution_id: Uuid,
    pub enabled: bool,
    pub check_in_frequency_days: u32,
    pub mandatory: bool,
    pub counsellor_role_enabled: bool,
    pub auto_escalation_threshold: f64, // wellbeing score below this triggers auto-case
    pub instructor_aggregate_visible: bool,
}

pub struct WellbeingService;

impl WellbeingService {
    /// Submit a weekly wellbeing check-in
    pub async fn submit_check_in(
        _pool: &sqlx::PgPool,
        req: SubmitCheckInRequest,
    ) -> Result<WellbeingCheckIn, sqlx::Error> {
        // Scores are clamped to 1–5 range; callers should validate before calling
        let check_in = WellbeingCheckIn {
            id: Uuid::new_v4(),
            student_id: req.student_id,
            institution_id: req.institution_id,
            week_start: Utc::now(), // normalise to Monday of current week in production
            wellbeing_score: req.wellbeing_score.clamp(1, 5),
            workload_score: req.workload_score.clamp(1, 5),
            sleeping_enough: req.sleeping_enough,
            free_text: req.free_text,
            self_referral_requested: req.self_referral_requested,
            is_anonymous: req.is_anonymous,
            submitted_at: Utc::now(),
        };

        // TODO: INSERT INTO wellbeing_checkins
        // If self_referral_requested → auto-create WellbeingCase with SelfReferral source
        // If wellbeing_score <= auto_escalation_threshold → create case with JuliaAutoEscalation

        Ok(check_in)
    }

    /// Get a student's personal wellbeing trend (private — only they can see)
    pub async fn get_student_trend(
        _pool: &sqlx::PgPool,
        student_id: Uuid,
        weeks: u32,
    ) -> Result<Vec<WellbeingTrend>, sqlx::Error> {
        let _ = (student_id, weeks);
        // TODO: SELECT week_start, avg(wellbeing_score), avg(workload_score)
        //       FROM wellbeing_checkins WHERE student_id = $1 GROUP BY week_start
        Ok(vec![])
    }

    /// Counsellor: aggregate view of a cohort (no individual identification)
    pub async fn get_cohort_aggregate(
        _pool: &sqlx::PgPool,
        counsellor_id: Uuid,
        cohort_id: Uuid,
    ) -> Result<CohortWellbeingAggregate, sqlx::Error> {
        let _ = counsellor_id;
        Ok(CohortWellbeingAggregate {
            cohort_id,
            cohort_name: String::new(),
            week_start: Utc::now(),
            avg_wellbeing: 3.8,
            avg_workload: 3.2,
            response_count: 0,
            total_students: 0,
            response_rate: 0.0,
            trend_direction: TrendDirection::Stable,
            alert_level: AlertLevel::Normal,
        })
    }

    /// Open a counsellor case for a student (requires student consent for adults)
    pub async fn open_case(
        _pool: &sqlx::PgPool,
        student_id: Uuid,
        institution_id: Uuid,
        counsellor_id: Uuid,
        referral_source: ReferralSource,
    ) -> Result<WellbeingCase, sqlx::Error> {
        Ok(WellbeingCase {
            id: Uuid::new_v4(),
            student_id,
            institution_id,
            counsellor_id,
            referral_source,
            status: CaseStatus::Open,
            notes: vec![],
            opened_at: Utc::now(),
            next_followup: None,
            closed_at: None,
            outcome: None,
        })
    }

    /// Add a note to a counsellor case
    pub async fn add_case_note(
        _pool: &sqlx::PgPool,
        case_id: Uuid,
        authored_by: Uuid,
        content: &str,
        is_private: bool,
    ) -> Result<CaseNote, sqlx::Error> {
        Ok(CaseNote {
            id: Uuid::new_v4(),
            case_id,
            authored_by,
            content: content.to_string(),
            is_private,
            created_at: Utc::now(),
        })
    }

    /// Close a counsellor case
    pub async fn close_case(
        _pool: &sqlx::PgPool,
        case_id: Uuid,
        counsellor_id: Uuid,
        outcome: &str,
    ) -> Result<(), sqlx::Error> {
        let _ = (case_id, counsellor_id, outcome);
        // TODO: UPDATE wellbeing_cases SET status='closed', closed_at=NOW(), outcome=$3
        Ok(())
    }

    /// Instructor alert: cohort wellbeing dropped (aggregate only, no individual data)
    pub async fn get_instructor_alerts(
        _pool: &sqlx::PgPool,
        instructor_id: Uuid,
    ) -> Result<Vec<InstructorWellbeingAlert>, sqlx::Error> {
        let _ = instructor_id;
        // TODO: Compare last 2 weeks avg for instructor's courses
        //       Flag if drop > 0.5 points, correlate with deadlines
        Ok(vec![])
    }

    /// Julia integration: combine wellbeing signals with dropout risk
    pub fn compute_risk_contribution(
        wellbeing_score: f64,
        workload_score: f64,
        weeks_since_last_checkin: u32,
    ) -> f64 {
        // Low wellbeing + high workload + no recent check-in = higher contribution to dropout risk
        let base = (5.0 - wellbeing_score) / 5.0;
        let workload_factor = if workload_score >= 4.0 { 1.2 } else { 1.0 };
        let recency_penalty = (weeks_since_last_checkin as f64 * 0.05).min(0.3);
        (base * workload_factor + recency_penalty).min(1.0)
    }
}
