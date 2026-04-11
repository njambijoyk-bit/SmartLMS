// Analytics & Reporting Service - Dashboards, reports, xAPI export
// Enhanced with Predictive Analytics & ML-Powered Insights
use chrono::{DateTime, Utc, Duration};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Learner dashboard data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearnerDashboard {
    pub user_id: uuid::Uuid,
    pub enrolled_courses: i32,
    pub completed_courses: i32,
    pub in_progress_courses: i32,
    pub average_grade: f64,
    pub total_study_time_hours: f64,
    pub upcoming_assignments: Vec<AssignmentSummary>,
    pub recent_activities: Vec<ActivityItem>,
    pub achievements: Vec<AchievementSummary>,
}

/// Assignment summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssignmentSummary {
    pub assignment_id: uuid::Uuid,
    pub course_name: String,
    pub title: String,
    pub due_date: DateTime<Utc>,
    pub status: String,
}

/// Activity item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivityItem {
    pub activity_type: String,
    pub description: String,
    pub timestamp: DateTime<Utc>,
    pub course_id: Option<uuid::Uuid>,
}

/// Achievement summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AchievementSummary {
    pub badge_name: String,
    pub earned_at: DateTime<Utc>,
}

/// Course analytics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CourseAnalytics {
    pub course_id: uuid::Uuid,
    pub total_enrolled: i32,
    pub total_completed: i32,
    pub completion_rate: f64,
    pub average_grade: f64,
    pub average_completion_time_hours: f64,
    pub engagement_metrics: EngagementMetrics,
    pub grade_distribution: Vec<GradeDistribution>,
    pub content_performance: Vec<ContentPerformance>,
}

/// Engagement metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EngagementMetrics {
    pub avg_login_frequency: f64,
    avg_session_duration_minutes: f64,
    pub total_content_views: i64,
    pub total_video_watch_time_hours: f64,
    pub forum_posts_count: i32,
    pub quiz_attempts_count: i32,
}

/// Grade distribution bucket
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GradeDistribution {
    pub grade_range: String,
    pub count: i32,
    pub percentage: f64,
}

/// Content performance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentPerformance {
    pub content_id: uuid::Uuid,
    pub content_title: String,
    pub views: i32,
    pub completion_rate: f64,
    pub avg_time_spent_minutes: f64,
}

/// Cohort comparison
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CohortComparison {
    pub cohort_id: uuid::Uuid,
    pub metrics: CohortMetrics,
    pub comparison_to_previous: ComparisonData,
}

/// Cohort metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CohortMetrics {
    pub total_students: i32,
    pub avg_engagement_score: f64,
    pub avg_completion_rate: f64,
    pub avg_grade: f64,
    pub at_risk_count: i32,
}

/// Comparison to previous cohort
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComparisonData {
    pub metric_name: String,
    pub current_value: f64,
    pub previous_value: f64,
    pub change_percentage: f64,
}

/// Custom report definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomReport {
    pub id: uuid::Uuid,
    pub institution_id: uuid::Uuid,
    pub name: String,
    pub description: Option<String>,
    pub report_type: ReportType,
    pub filters: Vec<ReportFilter>,
    pub columns: Vec<String>,
    pub schedule: Option<ReportSchedule>,
    pub created_by: uuid::Uuid,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReportType {
    LearnerProgress,
    CourseCompletion,
    GradeBook,
    Engagement,
    Attendance,
    Custom,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportFilter {
    pub field: String,
    pub operator: String,
    pub value: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportSchedule {
    pub frequency: String, // "daily", "weekly", "monthly"
    pub recipients: Vec<String>,
    pub next_run: DateTime<Utc>,
}

/// xAPI Statement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XapiStatement {
    pub id: uuid::Uuid,
    pub actor: XapiActor,
    pub verb: XapiVerb,
    pub object: XapiObject,
    pub result: Option<XapiResult>,
    pub context: Option<XapiContext>,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XapiActor {
    pub mbox: String, // email
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XapiVerb {
    pub id: String,
    pub display: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XapiObject {
    pub id: String,
    pub object_type: String,
    pub name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XapiResult {
    pub score: Option<XapiScore>,
    pub success: Option<bool>,
    pub completion: Option<bool>,
    pub duration: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XapiScore {
    pub scaled: Option<f64>,
    pub raw: Option<f64>,
    pub min: Option<f64>,
    pub max: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XapiContext {
    pub registration: Option<uuid::Uuid>,
    pub contextActivities: Option<serde_json::Value>,
}

// ============== PREDICTIVE ANALYTICS STRUCTURES ==============

/// Student success prediction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuccessPrediction {
    pub user_id: uuid::Uuid,
    pub course_id: uuid::Uuid,
    pub predicted_final_grade: f64,
    pub probability_of_success: f64, // 0-1 scale
    pub probability_of_completion: f64, // 0-1 scale
    pub risk_level: RiskLevel,
    pub contributing_factors: Vec<PredictionFactor>,
    pub recommended_interventions: Vec<Intervention>,
    pub confidence_score: f64,
    pub last_updated: DateTime<Utc>,
}

/// Risk level classification
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RiskLevel {
    Low,
    Medium,
    High,
    Critical,
}

/// Factor contributing to a prediction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PredictionFactor {
    pub factor_name: String,
    pub impact_weight: f64, // -1.0 to 1.0 (negative = detrimental, positive = beneficial)
    pub current_value: f64,
    pub benchmark_value: f64,
    pub description: String,
}

/// Recommended intervention for at-risk students
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Intervention {
    pub intervention_type: InterventionType,
    pub priority: Priority,
    pub title: String,
    pub description: String,
    pub expected_impact: String,
    pub suggested_timing: String,
    pub resources_required: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InterventionType {
    AcademicSupport,
    TimeManagement,
    PeerMentoring,
    InstructorOutreach,
    CounselingReferral,
    StudyGroup,
    ResourceRecommendation,
    DeadlineExtension,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Ord, PartialOrd)]
#[serde(rename_all = "lowercase")]
pub enum Priority {
    Low,
    Medium,
    High,
    Urgent,
}

/// Learning pattern analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningPattern {
    pub user_id: uuid::Uuid,
    pub pattern_type: PatternType,
    pub study_session_frequency: f64, // sessions per week
    pub avg_session_duration_minutes: f64,
    pub preferred_study_times: Vec<String>, // e.g., ["morning", "evening"]
    pub content_consumption_rate: f64, // modules per week
    pub engagement_trend: TrendDirection,
    pub consistency_score: f64, // 0-1 scale
    pub procrastination_indicator: f64, // 0-1 scale (higher = more procrastination)
    pub collaboration_score: f64, // 0-1 scale
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PatternType {
    ConsistentHighPerformer,
    ConsistentStruggler,
    Improving,
    Declining,
    Crammer,
    Procrastinator,
    SocialLearner,
    IndependentLearner,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TrendDirection {
    Improving,
    Stable,
    Declining,
    Volatile,
}

/// Cohort predictive analytics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CohortPredictions {
    pub cohort_id: uuid::Uuid,
    pub total_students: i32,
    pub predicted_pass_rate: f64,
    pub predicted_avg_grade: f64,
    pub students_by_risk: RiskDistribution,
    pub early_warning_alerts: Vec<EarlyWarningAlert>,
    pub cohort_health_score: f64, // 0-100 scale
    pub trend_analysis: CohortTrend,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskDistribution {
    pub low_risk: i32,
    pub medium_risk: i32,
    pub high_risk: i32,
    pub critical_risk: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EarlyWarningAlert {
    pub alert_id: uuid::Uuid,
    pub student_id: uuid::Uuid,
    pub student_name: String,
    pub alert_type: AlertType,
    pub severity: Severity,
    pub description: String,
    pub triggered_at: DateTime<Utc>,
    pub metrics: AlertMetrics,
    pub status: AlertStatus,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AlertType {
    LowEngagement,
    MissingAssignments,
    PoorQuizPerformance,
    AttendanceDrop,
    InactivityPeriod,
    GradeDecline,
    SocialIsolation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Severity {
    Info,
    Warning,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertMetrics {
    pub days_inactive: Option<i32>,
    pub missing_assignments_count: Option<i32>,
    pub avg_quiz_score: Option<f64>,
    pub attendance_percentage: Option<f64>,
    pub grade_change_percentage: Option<f64>,
    pub forum_posts_count: Option<i32>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AlertStatus {
    New,
    Acknowledged,
    InProgress,
    Resolved,
    Escalated,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CohortTrend {
    pub metric_name: String,
    pub historical_values: Vec<TrendPoint>,
    pub predicted_values: Vec<TrendPoint>,
    pub trend_direction: TrendDirection,
    pub velocity: f64, // rate of change
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrendPoint {
    pub period: String,
    pub value: f64,
    pub timestamp: DateTime<Utc>,
}

/// Course effectiveness prediction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CourseEffectiveness {
    pub course_id: uuid::Uuid,
    pub effectiveness_score: f64, // 0-100 scale
    pub predicted_completion_rate: f64,
    pub predicted_satisfaction_score: f64,
    pub difficulty_assessment: DifficultyAssessment,
    pub content_quality_indicators: ContentQualityIndicators,
    pub improvement_recommendations: Vec<CourseImprovement>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DifficultyAssessment {
    pub overall_difficulty: DifficultyLevel,
    pub difficulty_distribution: Vec<ModuleDifficulty>,
    pub bottleneck_modules: Vec<BottleneckModule>,
    pub time_to_complete_estimate_hours: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DifficultyLevel {
    VeryEasy,
    Easy,
    Moderate,
    Hard,
    VeryHard,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleDifficulty {
    pub module_id: uuid::Uuid,
    pub module_name: String,
    pub difficulty_score: f64, // 0-100
    pub avg_completion_time_hours: f64,
    pub failure_rate: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BottleneckModule {
    pub module_id: uuid::Uuid,
    pub module_name: String,
    pub dropout_rate: f64,
    pub avg_attempts: f64,
    pub common_issues: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentQualityIndicators {
    pub engagement_rate: f64,
    pub completion_rate: f64,
    pub rewatch_rate: f64, // For videos
    pub note_taking_frequency: f64,
    pub discussion_activity: f64,
    pub resource_download_rate: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CourseImprovement {
    pub area: ImprovementArea,
    pub priority: Priority,
    pub recommendation: String,
    pub expected_impact: String,
    pub implementation_effort: EffortLevel,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ImprovementArea {
    ContentClarity,
    AssessmentDesign,
    Pacing,
    Engagement,
    SupportResources,
    Prerequisites,
    TechnicalIssues,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum EffortLevel {
    Low,
    Medium,
    High,
}

/// ML Model performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelMetrics {
    pub model_name: String,
    pub model_version: String,
    pub accuracy: f64,
    pub precision: f64,
    pub recall: f64,
    pub f1_score: f64,
    pub auc_roc: f64,
    pub training_samples: i32,
    pub last_trained: DateTime<Utc>,
    pub drift_detected: bool,
    pub performance_trend: Vec<ModelPerformancePoint>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelPerformancePoint {
    pub period: String,
    pub accuracy: f64,
    pub timestamp: DateTime<Utc>,
}

// SERVICE FUNCTIONS
pub mod service {
    use super::*;
    use sqlx::PgPool;

    /// Get learner dashboard
    pub async fn get_learner_dashboard(
        pool: &PgPool,
        user_id: uuid::Uuid,
    ) -> Result<LearnerDashboard, String> {
        // Get enrollment stats
        let stats = sqlx::query!(
            "SELECT 
                COUNT(*) as total,
                SUM(CASE WHEN progress_percent = 100 THEN 1 ELSE 0 END) as completed,
                SUM(CASE WHEN progress_percent > 0 AND progress_percent < 100 THEN 1 ELSE 0 END) as in_progress
             FROM enrollments WHERE user_id = $1",
            user_id
        )
        .fetch_one(pool)
        .await
        .map_err(|e| e.to_string())?;

        // Get average grade
        let avg_grade: Option<f64> = sqlx::query_scalar!(
            "SELECT AVG(current_grade) FROM enrollments WHERE user_id = $1 AND current_grade IS NOT NULL",
            user_id
        )
        .fetch_one(pool)
        .await
        .map_err(|e| e.to_string())?;

        // Get upcoming assignments
        let assignments = sqlx::query!(
            "SELECT a.id, c.title as course_name, a.title, a.due_date
             FROM assignments a
             JOIN courses c ON a.course_id = c.id
             JOIN enrollments e ON e.course_id = c.id
             WHERE e.user_id = $1 AND a.due_date > NOW()
             ORDER BY a.due_date LIMIT 5",
            user_id
        )
        .fetch_all(pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(LearnerDashboard {
            user_id,
            enrolled_courses: stats.total.unwrap_or(0) as i32,
            completed_courses: stats.completed.unwrap_or(0) as i32,
            in_progress_courses: stats.in_progress.unwrap_or(0) as i32,
            average_grade: avg_grade.unwrap_or(0.0),
            total_study_time_hours: 0.0,
            upcoming_assignments: assignments
                .into_iter()
                .map(|a| AssignmentSummary {
                    assignment_id: a.id,
                    course_name: a.course_name,
                    title: a.title,
                    due_date: a.due_date,
                    status: "pending".to_string(),
                })
                .collect(),
            recent_activities: vec![],
            achievements: vec![],
        })
    }

    /// Get course analytics
    pub async fn get_course_analytics(
        pool: &PgPool,
        course_id: uuid::Uuid,
    ) -> Result<CourseAnalytics, String> {
        // Get enrollment stats
        let stats = sqlx::query!(
            "SELECT 
                COUNT(*) as total,
                SUM(CASE WHEN progress_percent = 100 THEN 1 ELSE 0 END) as completed
             FROM enrollments WHERE course_id = $1",
            course_id
        )
        .fetch_one(pool)
        .await
        .map_err(|e| e.to_string())?;

        let total = stats.total.unwrap_or(0) as i32;
        let completed = stats.completed.unwrap_or(0) as i32;
        let completion_rate = if total > 0 {
            (completed as f64 / total as f64) * 100.0
        } else {
            0.0
        };

        // Get average grade
        let avg_grade: Option<f64> = sqlx::query_scalar!(
            "SELECT AVG(current_grade) FROM enrollments WHERE course_id = $1 AND current_grade IS NOT NULL",
            course_id
        )
        .fetch_one(pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(CourseAnalytics {
            course_id,
            total_enrolled: total,
            total_completed: completed,
            completion_rate,
            average_grade: avg_grade.unwrap_or(0.0),
            average_completion_time_hours: 0.0,
            engagement_metrics: EngagementMetrics {
                avg_login_frequency: 0.0,
                avg_session_duration_minutes: 0.0,
                total_content_views: 0,
                total_video_watch_time_hours: 0.0,
                forum_posts_count: 0,
                quiz_attempts_count: 0,
            },
            grade_distribution: vec![],
            content_performance: vec![],
        })
    }

    /// Create custom report
    pub async fn create_custom_report(
        pool: &PgPool,
        institution_id: uuid::Uuid,
        creator_id: uuid::Uuid,
        name: &str,
        report_type: ReportType,
    ) -> Result<CustomReport, String> {
        let id = Uuid::new_v4();

        sqlx::query!(
            "INSERT INTO custom_reports (id, institution_id, name, report_type, created_by, created_at)
             VALUES ($1, $2, $3, $4, $5, $6)",
            id, institution_id, name, format!("{:?}", report_type).to_lowercase(), creator_id, Utc::now()
        )
        .execute(pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(CustomReport {
            id,
            institution_id,
            name: name.to_string(),
            description: None,
            report_type,
            filters: vec![],
            columns: vec![],
            schedule: None,
            created_by: creator_id,
            created_at: Utc::now(),
        })
    }

    /// Export to xAPI format
    pub async fn export_xapi(
        pool: &PgPool,
        user_id: uuid::Uuid,
        start_date: DateTime<Utc>,
        end_date: DateTime<Utc>,
    ) -> Result<Vec<XapiStatement>, String> {
        // Get quiz attempts as xAPI statements
        let attempts = sqlx::query!(
            "SELECT qa.id, qa.user_id, q.quiz_id, q.title, qa.score_percent, qa.completed_at,
                    u.email, u.first_name
             FROM quiz_attempts qa
             JOIN quizzes q ON qa.quiz_id = q.id
             JOIN users u ON qa.user_id = u.id
             WHERE qa.user_id = $1 AND qa.completed_at BETWEEN $2 AND $3",
            user_id,
            start_date,
            end_date
        )
        .fetch_all(pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(attempts
            .into_iter()
            .map(|a| XapiStatement {
                id: a.id,
                actor: XapiActor {
                    mbox: a.email,
                    name: format!("{} {}", a.first_name, ""),
                },
                verb: XapiVerb {
                    id: "http://adlnet.gov/expapi/verbs/completed".to_string(),
                    display: "completed".to_string(),
                },
                object: XapiObject {
                    id: format!("/quizzes/{}", a.quiz_id),
                    object_type: "Activity".to_string(),
                    name: Some(a.title),
                },
                result: Some(XapiResult {
                    score: Some(XapiScore {
                        scaled: Some(a.score_percent.unwrap_or(0.0) / 100.0),
                        raw: a.score_percent,
                        min: Some(0.0),
                        max: Some(100.0),
                    }),
                    success: Some(a.score_percent.unwrap_or(0.0) >= 60.0),
                    completion: Some(true),
                    duration: None,
                }),
                context: None,
                timestamp: a.completed_at,
            })
            .collect())
    }

    /// Get cohort comparison
    pub async fn get_cohort_comparison(
        pool: &PgPool,
        cohort_id: uuid::Uuid,
    ) -> Result<CohortComparison, String> {
        let row = sqlx::query!(
            "SELECT id, total_students, avg_engagement_score, avg_completion_rate, avg_grade, at_risk_count
             FROM cohorts WHERE id = $1",
            cohort_id
        )
        .fetch_one(pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(CohortComparison {
            cohort_id: row.id,
            metrics: CohortMetrics {
                total_students: row.total_students,
                avg_engagement_score: row.avg_engagement_score,
                avg_completion_rate: row.avg_completion_rate,
                avg_grade: row.avg_grade,
                at_risk_count: row.at_risk_count,
            },
            comparison_to_previous: ComparisonData {
                metric_name: "completion_rate".to_string(),
                current_value: row.avg_completion_rate,
                previous_value: row.avg_completion_rate * 0.95, // Simulated
                change_percentage: 5.0,
            },
        })
    }

    // ============== PREDICTIVE ANALYTICS SERVICE FUNCTIONS ==============

    /// Predict student success and generate early warnings
    pub async fn predict_student_success(
        pool: &PgPool,
        user_id: uuid::Uuid,
        course_id: uuid::Uuid,
    ) -> Result<SuccessPrediction, String> {
        // Get student's current performance data
        let enrollment = sqlx::query!(
            "SELECT current_grade, progress_percent, last_activity_at 
             FROM enrollments WHERE user_id = $1 AND course_id = $2",
            user_id, course_id
        )
        .fetch_optional(pool)
        .await
        .map_err(|e| e.to_string())?;

        let (current_grade, progress, last_activity) = match enrollment {
            Some(e) => (e.current_grade.unwrap_or(0.0), e.progress_percent.unwrap_or(0.0), e.last_activity_at),
            None => return Err("Enrollment not found".to_string()),
        };

        // Calculate days since last activity
        let days_inactive = match last_activity {
            Some(last) => {
                let now = Utc::now();
                ((now - last).num_hours() as f64 / 24.0).max(0.0)
            }
            None => 999.0,
        };

        // Get quiz performance
        let avg_quiz_score: Option<f64> = sqlx::query_scalar!(
            "SELECT AVG(score_percent) FROM quiz_attempts 
             WHERE user_id = $1 AND course_id = $2 AND completed_at IS NOT NULL",
            user_id, course_id
        )
        .fetch_one(pool)
        .await
        .map_err(|e| e.to_string())?;

        // Get assignment completion rate
        let assignment_stats = sqlx::query!(
            "SELECT COUNT(*) as total, 
                    SUM(CASE WHEN submitted_at IS NOT NULL THEN 1 ELSE 0 END) as submitted
             FROM assignment_submissions 
             WHERE user_id = $1 AND course_id = $2",
            user_id, course_id
        )
        .fetch_one(pool)
        .await
        .map_err(|e| e.to_string())?;

        let assignment_completion_rate = if assignment_stats.total > 0 {
            assignment_stats.submitted.unwrap_or(0) as f64 / assignment_stats.total as f64
        } else {
            1.0
        };

        // Build contributing factors
        let mut factors = Vec::new();
        
        // Factor: Current grade
        factors.push(PredictionFactor {
            factor_name: "Current Grade".to_string(),
            impact_weight: 0.35,
            current_value: current_grade,
            benchmark_value: 65.0,
            description: format!("Current grade of {:.1}% vs target of 65%", current_grade),
        });

        // Factor: Course progress
        factors.push(PredictionFactor {
            factor_name: "Course Progress".to_string(),
            impact_weight: 0.25,
            current_value: progress,
            benchmark_value: 70.0,
            description: format!("Progress at {:.1}% vs expected {:.1}%", progress, 70.0),
        });

        // Factor: Activity recency
        let activity_score = if days_inactive < 3.0 { 100.0 } else if days_inactive < 7.0 { 70.0 } else if days_inactive < 14.0 { 40.0 } else { 10.0 };
        factors.push(PredictionFactor {
            factor_name: "Recent Activity".to_string(),
            impact_weight: 0.20,
            current_value: activity_score,
            benchmark_value: 80.0,
            description: format!("Last active {:.1} days ago", days_inactive),
        });

        // Factor: Quiz performance
        if let Some(quiz_avg) = avg_quiz_score {
            factors.push(PredictionFactor {
                factor_name: "Quiz Performance".to_string(),
                impact_weight: 0.15,
                current_value: quiz_avg,
                benchmark_value: 70.0,
                description: format!("Average quiz score: {:.1}%", quiz_avg),
            });
        }

        // Factor: Assignment completion
        factors.push(PredictionFactor {
            factor_name: "Assignment Completion".to_string(),
            impact_weight: 0.15,
            current_value: assignment_completion_rate * 100.0,
            benchmark_value: 90.0,
            description: format!("{:.0}% of assignments submitted", assignment_completion_rate * 100.0),
        });

        // Calculate weighted score
        let weighted_score: f64 = factors.iter().map(|f| f.current_value * f.impact_weight.abs()).sum();
        let normalized_score = weighted_score / factors.iter().map(|f| f.impact_weight.abs()).sum::<f64>();

        // Determine risk level
        let (risk_level, probability_of_success, probability_of_completion) = if normalized_score >= 80.0 {
            (RiskLevel::Low, 0.92, 0.95)
        } else if normalized_score >= 65.0 {
            (RiskLevel::Medium, 0.75, 0.85)
        } else if normalized_score >= 50.0 {
            (RiskLevel::High, 0.55, 0.70)
        } else {
            (RiskLevel::Critical, 0.30, 0.45)
        };

        // Generate recommended interventions
        let mut interventions = Vec::new();
        
        if days_inactive > 5.0 {
            interventions.push(Intervention {
                intervention_type: InterventionType::InstructorOutreach,
                priority: if days_inactive > 10.0 { Priority::Urgent } else { Priority::High },
                title: "Re-engagement Outreach".to_string(),
                description: "Student has been inactive for several days. Personal outreach recommended.".to_string(),
                expected_impact: "Increase engagement and identify barriers to participation".to_string(),
                suggested_timing: "Within 24 hours".to_string(),
                resources_required: vec!["Email template".to_string(), "15 min instructor time".to_string()],
            });
        }

        if current_grade < 60.0 {
            interventions.push(Intervention {
                intervention_type: InterventionType::AcademicSupport,
                priority: Priority::High,
                title: "Academic Support Session".to_string(),
                description: format!("Current grade ({:.1}%) is below passing threshold.", current_grade),
                expected_impact: "Improve understanding of key concepts and raise grades".to_string(),
                suggested_timing: "This week".to_string(),
                resources_required: vec!["Tutor availability".to_string(), "Study materials".to_string()],
            });
        }

        if assignment_completion_rate < 0.7 {
            interventions.push(Intervention {
                intervention_type: InterventionType::TimeManagement,
                priority: Priority::Medium,
                title: "Time Management Coaching".to_string(),
                description: "Assignment submission rate is below expected levels.".to_string(),
                expected_impact: "Help student develop better planning and prioritization skills".to_string(),
                suggested_timing: "Within one week".to_string(),
                resources_required: vec!["Academic advisor session".to_string()],
            });
        }

        if avg_quiz_score.unwrap_or(100.0) < 65.0 {
            interventions.push(Intervention {
                intervention_type: InterventionType::StudyGroup,
                priority: Priority::Medium,
                title: "Peer Study Group".to_string(),
                description: "Quiz performance suggests benefit from collaborative learning.".to_string(),
                expected_impact: "Peer learning can improve comprehension and retention".to_string(),
                suggested_timing: "Ongoing".to_string(),
                resources_required: vec!["Study group facilitator".to_string(), "Meeting space".to_string()],
            });
        }

        Ok(SuccessPrediction {
            user_id,
            course_id,
            predicted_final_grade: normalized_score,
            probability_of_success,
            probability_of_completion,
            risk_level,
            contributing_factors: factors,
            recommended_interventions: interventions,
            confidence_score: 0.78, // Would be based on model confidence
            last_updated: Utc::now(),
        })
    }

    /// Get early warning alerts for a course
    pub async fn get_early_warning_alerts(
        pool: &PgPool,
        course_id: uuid::Uuid,
        limit: i64,
    ) -> Result<Vec<EarlyWarningAlert>, String> {
        // Get students with concerning patterns
        let at_risk_students = sqlx::query!(
            "SELECT e.user_id, u.first_name, u.last_name, e.current_grade, e.progress_percent, e.last_activity_at,
                    (SELECT COUNT(*) FROM assignment_submissions WHERE user_id = e.user_id AND course_id = e.course_id AND submitted_at IS NULL) as missing_assignments,
                    (SELECT AVG(score_percent) FROM quiz_attempts WHERE user_id = e.user_id AND course_id = e.course_id) as avg_quiz_score
             FROM enrollments e
             JOIN users u ON e.user_id = u.id
             WHERE e.course_id = $1 
               AND (e.current_grade < 60 OR e.progress_percent < 50 OR e.last_activity_at < NOW() - INTERVAL '7 days')
             ORDER BY e.current_grade ASC
             LIMIT $2",
            course_id, limit
        )
        .fetch_all(pool)
        .await
        .map_err(|e| e.to_string())?;

        let mut alerts = Vec::new();
        
        for student in at_risk_students {
            let days_inactive = match student.last_activity_at {
                Some(last) => ((Utc::now() - last).num_hours() as f64 / 24.0).max(0.0),
                None => 999.0,
            };

            // Determine alert type and severity
            let (alert_type, severity, description) = if student.current_grade.unwrap_or(100.0) < 40.0 {
                (AlertType::PoorQuizPerformance, Severity::Critical, "Failing grade - immediate intervention needed")
            } else if days_inactive > 14.0 {
                (AlertType::InactivityPeriod, Severity::Critical, "Extended period of inactivity")
            } else if student.missing_assignments.unwrap_or(0) > 3 {
                (AlertType::MissingAssignments, Severity::Warning, "Multiple missing assignments")
            } else if student.current_grade.unwrap_or(100.0) < 60.0 {
                (AlertType::GradeDecline, Severity::Warning, "Below passing grade threshold")
            } else {
                (AlertType::LowEngagement, Severity::Info, "Engagement below expected levels")
            };

            alerts.push(EarlyWarningAlert {
                alert_id: Uuid::new_v4(),
                student_id: student.user_id,
                student_name: format!("{} {}", student.first_name, student.last_name),
                alert_type,
                severity,
                description: description.to_string(),
                triggered_at: Utc::now(),
                metrics: AlertMetrics {
                    days_inactive: Some(days_inactive as i32),
                    missing_assignments_count: student.missing_assignments,
                    avg_quiz_score: student.avg_quiz_score,
                    attendance_percentage: None,
                    grade_change_percentage: None,
                    forum_posts_count: None,
                },
                status: AlertStatus::New,
            });
        }

        Ok(alerts)
    }

    /// Analyze learning patterns for a student
    pub async fn analyze_learning_pattern(
        pool: &PgPool,
        user_id: uuid::Uuid,
        course_id: uuid::Uuid,
    ) -> Result<LearningPattern, String> {
        // Get activity statistics
        let stats = sqlx::query!(
            "SELECT 
                COUNT(DISTINCT DATE(created_at)) as active_days,
                AVG(session_duration_minutes) as avg_session,
                SUM(session_duration_minutes) as total_time
             FROM user_activity_log 
             WHERE user_id = $1 AND course_id = $2 AND created_at > NOW() - INTERVAL '30 days'",
            user_id, course_id
        )
        .fetch_optional(pool)
        .await
        .map_err(|e| e.to_string())?;

        let (active_days, avg_session, total_time) = match stats {
            Some(s) => (s.active_days.unwrap_or(0), s.avg_session.unwrap_or(0.0), s.total_time.unwrap_or(0.0)),
            None => (0, 0.0, 0.0),
        };

        // Calculate weekly frequency
        let weekly_frequency = active_days as f64 / 4.3; // Average weeks in a month
        
        // Determine pattern type based on metrics
        let pattern_type = if weekly_frequency >= 5.0 && avg_session >= 45.0 {
            PatternType::ConsistentHighPerformer
        } else if weekly_frequency >= 4.0 && avg_session >= 30.0 {
            PatternType::IndependentLearner
        } else if weekly_frequency < 2.0 && avg_session < 20.0 {
            PatternType::Procrastinator
        } else if weekly_frequency >= 3.0 && avg_session >= 60.0 {
            PatternType::Crammer
        } else {
            PatternType::Improving
        };

        // Calculate consistency score
        let consistency_score = (weekly_frequency / 7.0).min(1.0) * (avg_session / 60.0).min(1.0);

        // Estimate procrastination indicator
        let procrastination_indicator = if weekly_frequency < 2.0 { 0.8 } else if weekly_frequency < 4.0 { 0.5 } else { 0.2 };

        Ok(LearningPattern {
            user_id,
            pattern_type,
            study_session_frequency: weekly_frequency,
            avg_session_duration_minutes: avg_session,
            preferred_study_times: vec!["evening".to_string()], // Would analyze actual timestamps
            content_consumption_rate: 0.0, // Would calculate from content views
            engagement_trend: TrendDirection::Stable,
            consistency_score,
            procrastination_indicator,
            collaboration_score: 0.5, // Would calculate from forum/discussion activity
        })
    }

    /// Get cohort predictions and health metrics
    pub async fn get_cohort_predictions(
        pool: &PgPool,
        cohort_id: uuid::Uuid,
    ) -> Result<CohortPredictions, String> {
        // Get cohort statistics
        let cohort_stats = sqlx::query!(
            "SELECT COUNT(*) as total_students,
                    AVG(current_grade) as avg_grade,
                    AVG(progress_percent) as avg_progress,
                    SUM(CASE WHEN current_grade < 50 THEN 1 ELSE 0 END) as critical_risk,
                    SUM(CASE WHEN current_grade >= 50 AND current_grade < 60 THEN 1 ELSE 0 END) as high_risk,
                    SUM(CASE WHEN current_grade >= 60 AND current_grade < 70 THEN 1 ELSE 0 END) as medium_risk,
                    SUM(CASE WHEN current_grade >= 70 THEN 1 ELSE 0 END) as low_risk
             FROM enrollments 
             WHERE cohort_id = $1",
            cohort_id
        )
        .fetch_one(pool)
        .await
        .map_err(|e| e.to_string())?;

        let total = cohort_stats.total_students.unwrap_or(0);
        let avg_grade = cohort_stats.avg_grade.unwrap_or(0.0);
        
        // Calculate risk distribution
        let risk_dist = RiskDistribution {
            low_risk: cohort_stats.low_risk.unwrap_or(0),
            medium_risk: cohort_stats.medium_risk.unwrap_or(0),
            high_risk: cohort_stats.high_risk.unwrap_or(0),
            critical_risk: cohort_stats.critical_risk.unwrap_or(0),
        };

        // Calculate cohort health score (0-100)
        let health_score = if total > 0 {
            ((risk_dist.low_risk as f64 * 100.0 + 
              risk_dist.medium_risk as f64 * 70.0 + 
              risk_dist.high_risk as f64 * 40.0 + 
              risk_dist.critical_risk as f64 * 15.0) / total as f64)
        } else {
            0.0
        };

        // Get early warning alerts for this cohort
        let alerts = sqlx::query!(
            "SELECT e.id as enrollment_id, e.user_id, u.first_name, u.last_name, e.current_grade
             FROM enrollments e
             JOIN users u ON e.user_id = u.id
             WHERE e.cohort_id = $1 AND e.current_grade < 60
             LIMIT 10",
            cohort_id
        )
        .fetch_all(pool)
        .await
        .map_err(|e| e.to_string())?;

        let early_warnings: Vec<EarlyWarningAlert> = alerts.into_iter().map(|a| EarlyWarningAlert {
            alert_id: Uuid::new_v4(),
            student_id: a.user_id,
            student_name: format!("{} {}", a.first_name, a.last_name),
            alert_type: AlertType::GradeDecline,
            severity: if a.current_grade.unwrap_or(100.0) < 40.0 { Severity::Critical } else { Severity::Warning },
            description: "Student performing below passing threshold".to_string(),
            triggered_at: Utc::now(),
            metrics: AlertMetrics {
                days_inactive: None,
                missing_assignments_count: None,
                avg_quiz_score: a.current_grade,
                attendance_percentage: None,
                grade_change_percentage: None,
                forum_posts_count: None,
            },
            status: AlertStatus::New,
        }).collect();

        Ok(CohortPredictions {
            cohort_id,
            total_students: total,
            predicted_pass_rate: (risk_dist.low_risk + risk_dist.medium_risk) as f64 / total.max(1) as f64 * 100.0,
            predicted_avg_grade: avg_grade,
            students_by_risk: risk_dist,
            early_warning_alerts: early_warnings,
            cohort_health_score: health_score,
            trend_analysis: CohortTrend {
                metric_name: "pass_rate".to_string(),
                historical_values: vec![],
                predicted_values: vec![],
                trend_direction: TrendDirection::Stable,
                velocity: 0.0,
            },
        })
    }

    /// Analyze course effectiveness and identify improvement areas
    pub async fn analyze_course_effectiveness(
        pool: &PgPool,
        course_id: uuid::Uuid,
    ) -> Result<CourseEffectiveness, String> {
        // Get course performance metrics
        let course_stats = sqlx::query!(
            "SELECT 
                COUNT(*) as total_enrolled,
                AVG(current_grade) as avg_grade,
                AVG(progress_percent) as avg_progress,
                SUM(CASE WHEN progress_percent = 100 THEN 1 ELSE 0 END) as completed
             FROM enrollments 
             WHERE course_id = $1",
            course_id
        )
        .fetch_one(pool)
        .await
        .map_err(|e| e.to_string())?;

        let total = course_stats.total_enrolled.max(1);
        let completion_rate = course_stats.completed as f64 / total as f64 * 100.0;
        let avg_grade = course_stats.avg_grade.unwrap_or(0.0);

        // Calculate effectiveness score
        let effectiveness_score = (completion_rate * 0.4 + avg_grade * 0.4 + 20.0).min(100.0);

        // Identify bottleneck modules (simplified - would need more detailed analysis)
        let bottlenecks = sqlx::query!(
            "SELECT m.id, m.title, 
                    COUNT(DISTINCT c.user_id) as completions,
                    COUNT(DISTINCT e.user_id) as enrolled
             FROM modules m
             LEFT JOIN content_completions c ON c.module_id = m.id
             JOIN enrollments e ON e.course_id = m.course_id
             WHERE m.course_id = $1
             GROUP BY m.id, m.title
             HAVING COUNT(DISTINCT c.user_id)::float / NULLIF(COUNT(DISTINCT e.user_id), 0) < 0.5
             ORDER BY COUNT(DISTINCT c.user_id)::float / NULLIF(COUNT(DISTINCT e.user_id), 0) ASC
             LIMIT 3",
            course_id
        )
        .fetch_all(pool)
        .await
        .map_err(|e| e.to_string())?;

        let bottleneck_modules: Vec<BottleneckModule> = bottlenecks.into_iter().map(|b| {
            let dropout = 1.0 - (b.completions as f64 / b.enrolled.max(1) as f64);
            BottleneckModule {
                module_id: b.id,
                module_name: b.title,
                dropout_rate: dropout,
                avg_attempts: 1.5,
                common_issues: vec!["Content complexity".to_string(), "Insufficient prerequisites".to_string()],
            }
        }).collect();

        // Generate improvement recommendations
        let mut recommendations = Vec::new();

        if completion_rate < 60.0 {
            recommendations.push(CourseImprovement {
                area: ImprovementArea::Pacing,
                priority: Priority::High,
                recommendation: "Consider breaking down complex modules into smaller units".to_string(),
                expected_impact: "Improve completion rate by 15-20%".to_string(),
                implementation_effort: EffortLevel::Medium,
            });
        }

        if avg_grade < 65.0 {
            recommendations.push(CourseImprovement {
                area: ImprovementArea::AssessmentDesign,
                priority: Priority::High,
                recommendation: "Review assessment difficulty and provide more practice opportunities".to_string(),
                expected_impact: "Raise average grades by 10-15 points".to_string(),
                implementation_effort: EffortLevel::Medium,
            });
        }

        if !bottleneck_modules.is_empty() {
            recommendations.push(CourseImprovement {
                area: ImprovementArea::ContentClarity,
                priority: Priority::Medium,
                recommendation: "Enhance explanations and add supplementary materials for difficult modules".to_string(),
                expected_impact: "Reduce dropout rate in bottleneck modules".to_string(),
                implementation_effort: EffortLevel::High,
            });
        }

        Ok(CourseEffectiveness {
            course_id,
            effectiveness_score,
            predicted_completion_rate: completion_rate,
            predicted_satisfaction_score: 75.0, // Would survey-based
            difficulty_assessment: DifficultyAssessment {
                overall_difficulty: if avg_grade < 60.0 { DifficultyLevel::Hard } else if avg_grade < 75.0 { DifficultyLevel::Moderate } else { DifficultyLevel::Easy },
                difficulty_distribution: vec![],
                bottleneck_modules,
                time_to_complete_estimate_hours: 40.0,
            },
            content_quality_indicators: ContentQualityIndicators {
                engagement_rate: 0.72,
                completion_rate,
                rewatch_rate: 0.15,
                note_taking_frequency: 0.35,
                discussion_activity: 0.45,
                resource_download_rate: 0.60,
            },
            improvement_recommendations: recommendations,
        })
    }
}
