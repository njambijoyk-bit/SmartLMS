// Gamification Service - Badges, XP, levels, and leaderboards
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// User gamification profile
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GamificationProfile {
    pub user_id: uuid::Uuid,
    pub total_xp: i64,
    pub level: i32,
    pub streak_days: i32,
    pub last_activity_at: Option<DateTime<Utc>>,
    pub badges: Vec<Badge>,
}

/// Badge definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Badge {
    pub id: uuid::Uuid,
    pub name: String,
    pub description: String,
    pub icon_url: String,
    pub badge_type: BadgeType,
    pub xp_reward: i64,
    pub criteria: BadgeCriteria,
}

/// Badge type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BadgeType {
    Achievement,
    Milestone,
    Streak,
    Completion,
    Special,
}

/// Badge criteria for earning
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BadgeCriteria {
    pub criteria_type: String, // "courses_completed", "quiz_score", "login_streak", etc.
    pub threshold: i32,
    pub course_id: Option<uuid::Uuid>,
}

/// User's earned badge
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EarnedBadge {
    pub id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub badge_id: uuid::Uuid,
    pub earned_at: DateTime<Utc>,
    pub certificate_id: Option<uuid::Uuid>,
}

/// XP transaction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XpTransaction {
    pub id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub amount: i64,
    pub reason: String,
    pub source_type: String, // "course_completed", "quiz_passed", "login_bonus"
    pub source_id: Option<uuid::Uuid>,
    pub created_at: DateTime<Utc>,
}

/// Leaderboard entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeaderboardEntry {
    pub rank: i32,
    pub user_id: uuid::Uuid,
    pub user_name: String,
    pub avatar_url: Option<String>,
    pub total_xp: i64,
    pub level: i32,
    pub streak_days: i32,
}

/// Level configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LevelConfig {
    pub level: i32,
    pub xp_required: i64,
    pub title: String,
}

// Service functions
pub mod service {
    use super::*;

    /// Award XP to user
    pub async fn award_xp(
        pool: &PgPool,
        user_id: uuid::Uuid,
        amount: i64,
        reason: &str,
        source_type: &str,
        source_id: Option<uuid::Uuid>,
    ) -> Result<GamificationProfile, String> {
        let tx_id = Uuid::new_v4();

        sqlx::query!(
            "INSERT INTO xp_transactions (id, user_id, amount, reason, source_type, source_id, created_at)
             VALUES ($1, $2, $3, $4, $5, $6, $7)",
            tx_id, user_id, amount, reason, source_type, source_id, Utc::now()
        )
        .execute(pool)
        .await
        .map_err(|e| e.to_string())?;

        // Update user profile total XP
        sqlx::query!(
            "UPDATE gamification_profiles SET total_xp = total_xp + $1, last_activity_at = $2 
             WHERE user_id = $3",
            amount,
            Utc::now(),
            user_id
        )
        .execute(pool)
        .await
        .map_err(|e| e.to_string())?;

        // Check for level up
        let profile = check_level_up(pool, user_id).await?;

        // Check for badge eligibility
        check_badges(pool, user_id).await?;

        Ok(profile)
    }

    /// Check and update level
    async fn check_level_up(
        pool: &PgPool,
        user_id: uuid::Uuid,
    ) -> Result<GamificationProfile, String> {
        let row = sqlx::query!(
            "SELECT total_xp, level FROM gamification_profiles WHERE user_id = $1",
            user_id
        )
        .fetch_one(pool)
        .await
        .map_err(|e| e.to_string())?;

        let new_level = calculate_level(row.total_xp);

        if new_level > row.level {
            sqlx::query!(
                "UPDATE gamification_profiles SET level = $1 WHERE user_id = $2",
                new_level,
                user_id
            )
            .execute(pool)
            .await
            .map_err(|e| e.to_string())?;

            tracing::info!("User {} leveled up to {}", user_id, new_level);
        }

        // Return updated profile
        get_profile(pool, user_id).await
    }

    /// Calculate level from XP
    fn calculate_level(xp: i64) -> i32 {
        // Level formula: each level requires more XP
        // Level 1: 0, Level 2: 100, Level 3: 250, etc.
        let levels = vec![
            0, 100, 250, 500, 1000, 2000, 3500, 5500, 8000, 12000, 17000, 23000, 30000, 40000,
            52000, 66000, 82000, 100000,
        ];

        for (i, req) in levels.iter().enumerate() {
            if xp < *req {
                return i as i32;
            }
        }

        levels.len() as i32
    }

    /// Award badge to user
    pub async fn award_badge(
        pool: &PgPool,
        user_id: uuid::Uuid,
        badge_id: uuid::Uuid,
    ) -> Result<EarnedBadge, String> {
        // Check if already earned
        let existing = sqlx::query!(
            "SELECT id FROM earned_badges WHERE user_id = $1 AND badge_id = $2",
            user_id,
            badge_id
        )
        .fetch_optional(pool)
        .await
        .map_err(|e| e.to_string())?;

        if existing.is_some() {
            return Err("Badge already earned".to_string());
        }

        // Get badge details for XP reward
        let badge = sqlx::query!("SELECT xp_reward FROM badges WHERE id = $1", badge_id)
            .fetch_one(pool)
            .await
            .map_err(|e| e.to_string())?;

        let id = Uuid::new_v4();

        sqlx::query!(
            "INSERT INTO earned_badges (id, user_id, badge_id, earned_at)
             VALUES ($1, $2, $3, $4)",
            id,
            user_id,
            badge_id,
            Utc::now()
        )
        .execute(pool)
        .await
        .map_err(|e| e.to_string())?;

        // Award XP for badge
        if badge.xp_reward > 0 {
            award_xp(
                pool,
                user_id,
                badge.xp_reward,
                "Badge earned",
                "badge",
                Some(badge_id),
            )
            .await
            .ok();
        }

        Ok(EarnedBadge {
            id,
            user_id,
            badge_id,
            earned_at: Utc::now(),
            certificate_id: None,
        })
    }

    /// Check badge eligibility
    async fn check_badges(pool: &PgPool, user_id: uuid::Uuid) -> Result<(), String> {
        // Get all badges not yet earned
        let rows = sqlx::query!(
            "SELECT b.id, b.criteria_type, b.threshold, b.course_id
             FROM badges b
             WHERE b.id NOT IN (SELECT badge_id FROM earned_badges WHERE user_id = $1)",
            user_id
        )
        .fetch_all(pool)
        .await
        .map_err(|e| e.to_string())?;

        for badge in rows {
            let eligible = match badge.criteria_type.as_str() {
                "courses_completed" => {
                    let count: i32 = sqlx::query_scalar!(
                        "SELECT COUNT(*) FROM enrollments WHERE user_id = $1 AND progress_percent = 100",
                        user_id
                    )
                    .fetch_one(pool)
                    .await
                    .unwrap_or(0);
                    count >= badge.threshold
                }
                "login_streak" => {
                    let streak = get_login_streak(pool, user_id).await?;
                    streak >= badge.threshold
                }
                _ => false,
            };

            if eligible {
                award_badge(pool, user_id, badge.id).await.ok();
            }
        }

        Ok(())
    }

    async fn get_login_streak(pool: &PgPool, user_id: uuid::Uuid) -> Result<i32, String> {
        // Check consecutive days
        let count: Option<i32> = sqlx::query_scalar!(
            "SELECT COUNT(DISTINCT DATE(created_at)) FROM login_events 
             WHERE user_id = $1 AND created_at > NOW() - INTERVAL '30 days'
             ORDER BY created_at DESC",
            user_id
        )
        .fetch_one(pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(count.unwrap_or(0))
    }

    /// Get user profile
    pub async fn get_profile(
        pool: &PgPool,
        user_id: uuid::Uuid,
    ) -> Result<GamificationProfile, String> {
        let row = sqlx::query!(
            "SELECT user_id, total_xp, level, streak_days, last_activity_at
             FROM gamification_profiles WHERE user_id = $1",
            user_id
        )
        .fetch_one(pool)
        .await
        .map_err(|e| e.to_string())?;

        // Get badges
        let badge_rows = sqlx::query!(
            "SELECT b.id, b.name, b.description, b.icon_url, b.badge_type, b.xp_reward
             FROM badges b
             JOIN earned_badges eb ON b.id = eb.badge_id
             WHERE eb.user_id = $1",
            user_id
        )
        .fetch_all(pool)
        .await
        .map_err(|e| e.to_string())?;

        let badges: Vec<Badge> = badge_rows
            .into_iter()
            .map(|b| Badge {
                id: b.id,
                name: b.name,
                description: b.description,
                icon_url: b.icon_url,
                badge_type: BadgeType::Achievement,
                xp_reward: b.xp_reward,
                criteria: BadgeCriteria {
                    criteria_type: String::new(),
                    threshold: 0,
                    course_id: None,
                },
            })
            .collect();

        Ok(GamificationProfile {
            user_id: row.user_id,
            total_xp: row.total_xp,
            level: row.level,
            streak_days: row.streak_days,
            last_activity_at: row.last_activity_at,
            badges,
        })
    }

    /// Get leaderboard
    pub async fn get_leaderboard(
        pool: &PgPool,
        institution_id: uuid::Uuid,
        limit: i64,
    ) -> Result<Vec<LeaderboardEntry>, String> {
        let rows = sqlx::query!(
            "SELECT gp.user_id, u.first_name || ' ' || u.last_name as user_name, 
                    u.avatar_url, gp.total_xp, gp.level, gp.streak_days
             FROM gamification_profiles gp
             JOIN users u ON gp.user_id = u.id
             WHERE u.institution_id = $1
             ORDER BY gp.total_xp DESC
             LIMIT $2",
            institution_id,
            limit
        )
        .fetch_all(pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(rows
            .into_iter()
            .enumerate()
            .map(|(i, r)| LeaderboardEntry {
                rank: (i + 1) as i32,
                user_id: r.user_id,
                user_name: r.user_name,
                avatar_url: r.avatar_url,
                total_xp: r.total_xp,
                level: r.level,
                streak_days: r.streak_days,
            })
            .collect())
    }
}
