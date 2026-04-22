// ML Engine Service - Julia-based ML for adaptive learning
// This service provides ML predictions via API, with actual ML models in Julia

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Learning analytics data point
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningEvent {
    pub user_id: uuid::Uuid,
    pub course_id: uuid::Uuid,
    pub event_type: String,
    pub timestamp: DateTime<Utc>,
    pub metadata: std::collections::HashMap<String, String>,
}

/// Dropout prediction result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DropoutPrediction {
    pub user_id: uuid::Uuid,
    pub course_id: uuid::Uuid,
    pub risk_score: f64, // 0.0 - 1.0
    pub risk_level: RiskLevel,
    pub contributing_factors: Vec<RiskFactor>,
    pub recommended_actions: Vec<String>,
    pub predicted_at: DateTime<Utc>,
}

/// Risk levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RiskLevel {
    Low,
    Medium,
    High,
    Critical,
}

/// Risk factor contributing to dropout
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskFactor {
    pub factor: String,
    pub impact: f64, // Contribution to overall risk
    pub description: String,
}

/// Content recommendation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentRecommendation {
    pub user_id: uuid::Uuid,
    pub course_id: uuid::Uuid,
    pub recommendations: Vec<RecommendedContent>,
    pub reason: String,
    pub generated_at: DateTime<Utc>,
}

/// Recommended content item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecommendedContent {
    pub content_id: uuid::Uuid,
    pub content_type: String,
    pub title: String,
    pub relevance_score: f64,
    pub reason: String,
}

/// Performance prediction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformancePrediction {
    pub user_id: uuid::Uuid,
    pub course_id: uuid::Uuid,
    pub predicted_score: f64,
    pub confidence_interval: (f64, f64),
    pub factors: std::collections::HashMap<String, f64>,
    pub predicted_at: DateTime<Utc>,
}

/// Knowledge gap analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeGap {
    pub user_id: uuid::Uuid,
    pub course_id: uuid::Uuid,
    pub topic_id: uuid::Uuid,
    pub topic_name: String,
    pub mastery_level: f64, // 0.0 - 1.0
    pub recommended_resources: Vec<uuid::Uuid>,
}

/// Adaptive learning path
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningPath {
    pub user_id: uuid::Uuid,
    pub course_id: uuid::Uuid,
    pub path: Vec<PathNode>,
    pub estimated_completion_hours: f64,
    pub generated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PathNode {
    pub content_id: uuid::Uuid,
    pub content_type: String,
    pub title: String,
    pub order: i32,
    pub estimated_minutes: i32,
    pub is_required: bool,
}

// Service functions
pub mod service {
    use super::*;

    /// Predict dropout risk for a user in a course
    pub async fn predict_dropout_risk(
        pool: &PgPool,
        user_id: uuid::Uuid,
        course_id: uuid::Uuid,
    ) -> Result<DropoutPrediction, String> {
        // Collect user features for prediction
        let features = collect_learning_features(pool, user_id, course_id).await?;

        // In production: call Julia ML service
        // For now, calculate heuristic-based risk
        let risk_score = calculate_dropout_risk(&features);

        let risk_level = match risk_score {
            s if s < 0.2 => RiskLevel::Low,
            s if s < 0.5 => RiskLevel::Medium,
            s if s < 0.8 => RiskLevel::High,
            _ => RiskLevel::Critical,
        };

        let factors = identify_risk_factors(&features);

        let actions = generate_interventions(risk_level, &factors);

        Ok(DropoutPrediction {
            user_id,
            course_id,
            risk_score,
            risk_level,
            contributing_factors: factors,
            recommended_actions: actions,
            predicted_at: Utc::now(),
        })
    }

    fn collect_learning_features(
        pool: &PgPool,
        user_id: uuid::Uuid,
        course_id: uuid::Uuid,
    ) -> Result<LearningFeatures, String> {
        // Get engagement metrics
        let login_count: i64 = sqlx::query_scalar!(
            "SELECT COUNT(*) FROM login_events WHERE user_id = $1 AND created_at > NOW() - INTERVAL '30 days'",
            user_id
        )
        .fetch_one(pool)
        .await
        .map_err(|e| e.to_string())?;

        // Get course progress
        let progress: Option<f64> = sqlx::query_scalar!(
            "SELECT progress_percent FROM enrollments WHERE user_id = $1 AND course_id = $2",
            user_id,
            course_id
        )
        .fetch_one(pool)
        .await
        .map_err(|e| e.to_string())?;

        // Get assignment submissions
        let late_submissions: i64 = sqlx::query_scalar!(
            "SELECT COUNT(*) FROM assignment_submissions 
             WHERE user_id = $1 AND course_id = $2 AND submitted_at > due_date",
            user_id,
            course_id
        )
        .fetch_one(pool)
        .await
        .map_err(|e| e.to_string())?;

        // Get quiz scores
        let avg_quiz_score: Option<f64> = sqlx::query_scalar!(
            "SELECT AVG(score_percent) FROM quiz_attempts 
             WHERE user_id = $1 AND course_id = $2",
            user_id,
            course_id
        )
        .fetch_one(pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(LearningFeatures {
            login_count: login_count as i32,
            progress_percent: progress.unwrap_or(0.0),
            late_submissions: late_submissions as i32,
            avg_quiz_score: avg_quiz_score.unwrap_or(0.0),
        })
    }

    fn calculate_dropout_risk(features: &LearningFeatures) -> f64 {
        let mut risk = 0.0;

        // Low engagement = higher risk
        if features.login_count < 3 {
            risk += 0.3;
        } else if features.login_count < 7 {
            risk += 0.15;
        }

        // Low progress = higher risk
        if features.progress_percent < 10.0 {
            risk += 0.35;
        } else if features.progress_percent < 30.0 {
            risk += 0.2;
        }

        // Late submissions = higher risk
        if features.late_submissions > 3 {
            risk += 0.2;
        }

        // Low quiz scores = higher risk
        if features.avg_quiz_score < 50.0 {
            risk += 0.25;
        } else if features.avg_quiz_score < 70.0 {
            risk += 0.1;
        }

        risk.min(1.0)
    }

    fn identify_risk_factors(features: &LearningFeatures) -> Vec<RiskFactor> {
        let mut factors = Vec::new();

        if features.login_count < 5 {
            factors.push(RiskFactor {
                factor: "low_engagement".to_string(),
                impact: 0.3,
                description: "Low login frequency in past 30 days".to_string(),
            });
        }

        if features.progress_percent < 30.0 {
            factors.push(RiskFactor {
                factor: "low_progress".to_string(),
                impact: 0.35,
                description: "Course progress below expected".to_string(),
            });
        }

        if features.late_submissions > 0 {
            factors.push(RiskFactor {
                factor: "late_assignments".to_string(),
                impact: 0.2,
                description: "Multiple late assignment submissions".to_string(),
            });
        }

        if features.avg_quiz_score < 60.0 {
            factors.push(RiskFactor {
                factor: "struggling_quizzes".to_string(),
                impact: 0.25,
                description: "Below average quiz performance".to_string(),
            });
        }

        factors
    }

    fn generate_interventions(risk_level: RiskLevel, factors: &[RiskFactor]) -> Vec<String> {
        let mut actions = Vec::new();

        actions.push("Schedule one-on-one check-in with student".to_string());

        if factors.iter().any(|f| f.factor == "low_engagement") {
            actions.push("Send engagement reminder email".to_string());
            actions.push("Recommend joining study group".to_string());
        }

        if factors.iter().any(|f| f.factor == "low_progress") {
            actions.push("Offer additional tutoring sessions".to_string());
            actions.push("Provide supplementary learning materials".to_string());
        }

        if risk_level == RiskLevel::Critical {
            actions.push("Flag for academic advisor review".to_string());
            actions.push("Consider extending assignment deadlines".to_string());
        }

        actions
    }

    /// Get content recommendations for user
    pub async fn get_recommendations(
        pool: &PgPool,
        user_id: uuid::Uuid,
        course_id: uuid::Uuid,
    ) -> Result<ContentRecommendation, String> {
        // Get user's completed content
        let completed: Vec<uuid::Uuid> = sqlx::query_scalar!(
            "SELECT content_id FROM lesson_progress WHERE user_id = $1 AND is_completed = true",
            user_id
        )
        .fetch_all(pool)
        .await
        .map_err(|e| e.to_string())?;

        // Get user's quiz performance by topic
        let weak_topics = sqlx::query!(
            "SELECT topic_id, AVG(score_percent) as avg_score 
             FROM quiz_attempts WHERE user_id = $1 AND score_percent < 70
             GROUP BY topic_id ORDER BY avg_score ASC LIMIT 5",
            user_id
        )
        .fetch_all(pool)
        .await
        .map_err(|e| e.to_string())?;

        // Find content for weak topics
        let mut recommendations = Vec::new();

        for topic in weak_topics {
            let content = sqlx::query!(
                "SELECT id, title, content_type FROM course_content 
                 WHERE topic_id = $1 AND id NOT IN (SELECT content_id FROM lesson_progress WHERE user_id = $2 AND is_completed = true)
                 ORDER BY created_at DESC LIMIT 3",
                topic.topic_id, user_id
            )
            .fetch_all(pool)
            .await
            .map_err(|e| e.to_string())?;

            for c in content {
                recommendations.push(RecommendedContent {
                    content_id: c.id,
                    content_type: c.content_type,
                    title: c.title,
                    relevance_score: 0.8,
                    reason: format!("Strengthen {} topic understanding", topic.topic_id),
                });
            }
        }

        Ok(ContentRecommendation {
            user_id,
            course_id,
            recommendations,
            reason: "Based on your quiz performance and learning history".to_string(),
            generated_at: Utc::now(),
        })
    }

    /// Predict final exam score
    pub async fn predict_performance(
        pool: &PgPool,
        user_id: uuid::Uuid,
        course_id: uuid::Uuid,
    ) -> Result<PerformancePrediction, String> {
        // Get current grade
        let current_grade: Option<f64> = sqlx::query_scalar!(
            "SELECT current_grade FROM enrollments WHERE user_id = $1 AND course_id = $2",
            user_id,
            course_id
        )
        .fetch_one(pool)
        .await
        .map_err(|e| e.to_string())?;

        // Calculate predicted score based on trends
        let base_score = current_grade.unwrap_or(70.0);

        // Adjust based on engagement
        let recent_activity: i64 = sqlx::query_scalar!(
            "SELECT COUNT(*) FROM learning_events 
             WHERE user_id = $1 AND course_id = $2 AND created_at > NOW() - INTERVAL '7 days'",
            user_id,
            course_id
        )
        .fetch_one(pool)
        .await
        .map_err(|e| e.to_string())?;

        let adjustment = if recent_activity > 10 { 5.0 } else { -5.0 };
        let predicted = (base_score + adjustment).min(100.0).max(0.0);

        Ok(PerformancePrediction {
            user_id,
            course_id,
            predicted_score: predicted,
            confidence_interval: (predicted - 10.0, predicted + 10.0),
            factors: std::collections::HashMap::from([
                ("current_grade".to_string(), base_score),
                ("recent_activity".to_string(), recent_activity as f64),
            ]),
            predicted_at: Utc::now(),
        })
    }

    /// Generate adaptive learning path
    pub async fn generate_learning_path(
        pool: &PgPool,
        user_id: uuid::Uuid,
        course_id: uuid::Uuid,
    ) -> Result<LearningPath, String> {
        // Get course content in order
        let content = sqlx::query!(
            "SELECT id, title, content_type, order_index, estimated_minutes
             FROM course_content WHERE course_id = $1 ORDER BY order_index",
            course_id
        )
        .fetch_all(pool)
        .await
        .map_err(|e| e.to_string())?;

        // Get user's completed content
        let completed: Vec<uuid::Uuid> = sqlx::query_scalar!(
            "SELECT content_id FROM lesson_progress WHERE user_id = $1 AND is_completed = true",
            user_id
        )
        .fetch_all(pool)
        .await
        .map_err(|e| e.to_string())?;

        // Build path - skip completed, add recommended
        let mut path = Vec::new();
        let mut total_minutes = 0;
        let mut order = 1;

        for c in content {
            if !completed.contains(&c.id) {
                path.push(PathNode {
                    content_id: c.id,
                    content_type: c.content_type,
                    title: c.title,
                    order,
                    estimated_minutes: c.estimated_minutes.unwrap_or(30),
                    is_required: true,
                });
                total_minutes += c.estimated_minutes.unwrap_or(30);
                order += 1;
            }
        }

        Ok(LearningPath {
            user_id,
            course_id,
            path,
            estimated_completion_hours: total_minutes as f64 / 60.0,
            generated_at: Utc::now(),
        })
    }
}

#[derive(Debug, Clone)]
struct LearningFeatures {
    login_count: i32,
    progress_percent: f64,
    late_submissions: i32,
    avg_quiz_score: f64,
}
