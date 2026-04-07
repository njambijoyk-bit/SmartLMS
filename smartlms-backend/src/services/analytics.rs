// Analytics & Reporting Service - Dashboards, reports, xAPI export
use chrono::{DateTime, Utc};
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

// SERVICE FUNCTIONS
pub mod service {
    use super::*;

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
}
