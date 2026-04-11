// Gamification Service - Badges, XP, levels, leaderboards, quests, virtual economy, and achievements
use chrono::{DateTime, Duration, Utc};
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
    pub coins: i64,
    pub gems: i64,
    pub current_quest_ids: Vec<Uuid>,
    pub completed_quests_count: i32,
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
    pub coin_reward: i64,
    pub gem_reward: i64,
    pub criteria: BadgeCriteria,
    pub rarity: BadgeRarity,
}

/// Badge rarity
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BadgeRarity {
    Common,
    Uncommon,
    Rare,
    Epic,
    Legendary,
}

/// Badge type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BadgeType {
    Achievement,
    Milestone,
    Streak,
    Completion,
    Special,
    Secret,
    Event,
}

/// Badge criteria for earning
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BadgeCriteria {
    pub criteria_type: String, // "courses_completed", "quiz_score", "login_streak", etc.
    pub threshold: i32,
    pub course_id: Option<uuid::Uuid>,
    pub additional_data: Option<serde_json::Value>,
}

/// User's earned badge
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EarnedBadge {
    pub id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub badge_id: uuid::Uuid,
    pub earned_at: DateTime<Utc>,
    pub certificate_id: Option<uuid::Uuid>,
    pub is_public: bool,
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
    pub badge_count: i32,
}

/// Level configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LevelConfig {
    pub level: i32,
    pub xp_required: i64,
    pub title: String,
    pub perks: Vec<String>,
}

// ==================== QUEST SYSTEM ====================

/// Quest definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Quest {
    pub id: uuid::Uuid,
    pub title: String,
    pub description: String,
    pub quest_type: QuestType,
    pub difficulty: QuestDifficulty,
    pub xp_reward: i64,
    pub coin_reward: i64,
    pub gem_reward: i64,
    pub badge_reward: Option<uuid::Uuid>,
    pub requirements: Vec<QuestRequirement>,
    pub time_limit_days: Option<i32>,
    pub is_repeatable: bool,
    pub category: String,
}

/// Quest type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum QuestType {
    Daily,
    Weekly,
    Monthly,
    OneTime,
    Event,
    Challenge,
}

/// Quest difficulty
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum QuestDifficulty {
    Easy,
    Medium,
    Hard,
    Expert,
}

/// Quest requirement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestRequirement {
    pub requirement_type: String, // "complete_quiz", "watch_video", "post_discussion", etc.
    pub target_value: i32,
    pub current_value: i32,
    pub course_id: Option<uuid::Uuid>,
    pub is_completed: bool,
}

/// User quest progress
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserQuest {
    pub id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub quest_id: uuid::Uuid,
    pub status: QuestStatus,
    pub started_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub progress: Vec<QuestRequirement>,
    pub claimed: bool,
}

/// Quest status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum QuestStatus {
    Active,
    Completed,
    Failed,
    Claimed,
    Expired,
}

// ==================== VIRTUAL ECONOMY ====================

/// Virtual currency transaction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurrencyTransaction {
    pub id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub currency_type: CurrencyType,
    pub amount: i64,
    pub transaction_type: TransactionType,
    pub reason: String,
    pub balance_after: i64,
    pub source_id: Option<uuid::Uuid>,
    pub created_at: DateTime<Utc>,
}

/// Currency type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CurrencyType {
    Coins,
    Gems,
    Tokens,
}

/// Transaction type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TransactionType {
    Earn,
    Spend,
    Bonus,
    Penalty,
    Transfer,
    Refund,
}

/// Shop item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShopItem {
    pub id: uuid::Uuid,
    pub name: String,
    pub description: String,
    pub item_type: ShopItemType,
    pub cost_coins: i64,
    pub cost_gems: i64,
    pub discount_percent: Option<i32>,
    pub stock_quantity: Option<i32>,
    pub is_limited: bool,
    pub available_from: Option<DateTime<Utc>>,
    pub available_until: Option<DateTime<Utc>>,
    pub image_url: String,
}

/// Shop item type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShopItemType {
    pub category: String, // "avatar", "powerup", "decoration", "certificate", "course_discount"
    pub subtype: String,
    pub data: Option<serde_json::Value>,
}

/// User inventory item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventoryItem {
    pub id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub item_id: uuid::Uuid,
    pub quantity: i32,
    pub acquired_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
    pub is_used: bool,
}

// ==================== SPIN THE WHEEL ====================

/// Spin wheel configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpinWheel {
    pub id: uuid::Uuid,
    pub name: String,
    pub segments: Vec<WheelSegment>,
    pub spin_cost_coins: i64,
    pub spin_cost_gems: i64,
    pub daily_free_spins: i32,
    pub cooldown_hours: i32,
    pub is_active: bool,
}

/// Wheel segment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WheelSegment {
    pub id: uuid::Uuid,
    pub label: String,
    pub reward_type: RewardType,
    pub reward_value: i64,
    pub probability: f64, // 0.0 to 1.0
    pub color: String,
    pub is_jackpot: bool,
}

/// Reward type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RewardType {
    XP,
    Coins,
    Gems,
    Badge,
    PowerUp,
    Discount,
    Jackpot,
    Nothing,
}

/// User spin history
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpinHistory {
    pub id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub wheel_id: uuid::Uuid,
    pub result_segment_id: uuid::Uuid,
    pub reward_type: RewardType,
    pub reward_value: i64,
    pub spun_at: DateTime<Utc>,
    pub was_free_spin: bool,
}

// ==================== ACHIEVEMENTS ====================

/// Achievement definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Achievement {
    pub id: uuid::Uuid,
    pub title: String,
    pub description: String,
    pub achievement_type: AchievementType,
    pub tier: AchievementTier,
    pub xp_reward: i64,
    pub coin_reward: i64,
    pub criteria: AchievementCriteria,
    pub parent_achievement_id: Option<uuid::Uuid>,
    pub is_secret: bool,
    pub icon_url: String,
}

/// Achievement type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AchievementType {
    Progress,
    Skill,
    Social,
    Collection,
    Challenge,
    TimeBased,
}

/// Achievement tier
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AchievementTier {
    Bronze,
    Silver,
    Gold,
    Platinum,
    Diamond,
}

/// Achievement criteria
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AchievementCriteria {
    pub criteria_type: String,
    pub threshold: i64,
    pub timeframe_days: Option<i32>,
    pub additional_conditions: Option<serde_json::Value>,
}

/// User achievement progress
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserAchievement {
    pub id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub achievement_id: uuid::Uuid,
    pub current_progress: i64,
    pub is_completed: bool,
    pub completed_at: Option<DateTime<Utc>>,
    pub claimed: bool,
}

// ==================== POWER-UPS & BOOSTS ====================

/// Power-up definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PowerUp {
    pub id: uuid::Uuid,
    pub name: String,
    pub description: String,
    pub power_type: PowerType,
    pub effect_multiplier: f64,
    pub duration_hours: i32,
    pub cost_coins: i64,
    pub cost_gems: i64,
    pub icon_url: String,
}

/// Power type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PowerType {
    XPMultiplier,
    CoinBooster,
    StreakFreeze,
    HintUnlock,
    TimeExtension,
    DoubleRewards,
}

/// Active power-up
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivePowerUp {
    pub id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub power_up_id: uuid::Uuid,
    pub activated_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub is_active: bool,
}

// ==================== LEADERBOARDS & COMPETITIONS ====================

/// Competition/Leaderboard season
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Competition {
    pub id: uuid::Uuid,
    pub name: String,
    pub description: String,
    pub competition_type: CompetitionType,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub prize_pool_coins: i64,
    pub prize_pool_gems: i64,
    pub participant_limit: Option<i32>,
    pub is_team_based: bool,
    pub rules: String,
}

/// Competition type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CompetitionType {
    WeeklyXP,
    MonthlyBadges,
    CourseCompletion,
    QuizMastery,
    StreakChallenge,
    Custom,
}

/// Competition participant
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompetitionParticipant {
    pub id: uuid::Uuid,
    pub competition_id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub team_id: Option<uuid::Uuid>,
    pub score: i64,
    pub rank: Option<i32>,
    pub prize_awarded: bool,
    pub joined_at: DateTime<Utc>,
}

/// Team for team-based competitions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Team {
    pub id: uuid::Uuid,
    pub name: String,
    pub captain_id: uuid::Uuid,
    pub member_ids: Vec<uuid::Uuid>,
    pub total_score: i64,
    pub created_at: DateTime<Utc>,
}

// ==================== SERVICE FUNCTIONS ====================

pub mod service {
    use super::*;
    use sqlx::PgPool;

    /// Award XP to user with full gamification update
    pub async fn award_xp(
        pool: &PgPool,
        user_id: uuid::Uuid,
        amount: i64,
        reason: &str,
        source_type: &str,
        source_id: Option<uuid::Uuid>,
    ) -> Result<GamificationProfile, String> {
        let tx = pool.begin().await.map_err(|e| e.to_string())?;
        
        let tx_id = Uuid::new_v4();

        sqlx::query!(
            "INSERT INTO xp_transactions (id, user_id, amount, reason, source_type, source_id, created_at)
             VALUES ($1, $2, $3, $4, $5, $6, $7)",
            tx_id, user_id, amount, reason, source_type, source_id, Utc::now()
        )
        .execute(&tx)
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
        .execute(&tx)
        .await
        .map_err(|e| e.to_string())?;

        // Check for level up
        let profile = check_level_up(&tx, user_id).await?;

        // Check for badge eligibility
        check_badges(&tx, user_id).await?;

        // Check for achievement progress
        update_achievements(&tx, user_id, "xp_earned", amount).await?;

        // Update quest progress
        update_quest_progress(&tx, user_id, "earn_xp", amount).await?;

        tx.commit().await.map_err(|e| e.to_string())?;

        Ok(profile)
    }

    /// Award coins to user
    pub async fn award_coins(
        pool: &PgPool,
        user_id: uuid::Uuid,
        amount: i64,
        reason: &str,
        source_type: &str,
        source_id: Option<uuid::Uuid>,
    ) -> Result<i64, String> {
        let tx = pool.begin().await.map_err(|e| e.to_string())?;

        // Get current balance
        let current_balance: i64 = sqlx::query_scalar!(
            "SELECT coins FROM gamification_profiles WHERE user_id = $1",
            user_id
        )
        .fetch_one(&tx)
        .await
        .map_err(|e| e.to_string())?;

        let new_balance = current_balance + amount;

        // Update balance
        sqlx::query!(
            "UPDATE gamification_profiles SET coins = $1 WHERE user_id = $2",
            new_balance,
            user_id
        )
        .execute(&tx)
        .await
        .map_err(|e| e.to_string())?;

        // Record transaction
        let tx_id = Uuid::new_v4();
        sqlx::query!(
            "INSERT INTO currency_transactions (id, user_id, currency_type, amount, transaction_type, reason, balance_after, source_id, created_at)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)",
            tx_id,
            user_id,
            "Coins" as _,
            amount,
            "Earn" as _,
            reason,
            new_balance,
            source_id,
            Utc::now()
        )
        .execute(&tx)
        .await
        .map_err(|e| e.to_string())?;

        tx.commit().await.map_err(|e| e.to_string())?;

        Ok(new_balance)
    }

    /// Spend coins from user balance
    pub async fn spend_coins(
        pool: &PgPool,
        user_id: uuid::Uuid,
        amount: i64,
        reason: &str,
        item_id: Option<uuid::Uuid>,
    ) -> Result<bool, String> {
        let tx = pool.begin().await.map_err(|e| e.to_string())?;

        // Get current balance
        let current_balance: i64 = sqlx::query_scalar!(
            "SELECT coins FROM gamification_profiles WHERE user_id = $1",
            user_id
        )
        .fetch_one(&tx)
        .await
        .map_err(|e| e.to_string())?;

        if current_balance < amount {
            return Err("Insufficient coins".to_string());
        }

        let new_balance = current_balance - amount;

        // Update balance
        sqlx::query!(
            "UPDATE gamification_profiles SET coins = $1 WHERE user_id = $2",
            new_balance,
            user_id
        )
        .execute(&tx)
        .await
        .map_err(|e| e.to_string())?;

        // Record transaction
        let tx_id = Uuid::new_v4();
        sqlx::query!(
            "INSERT INTO currency_transactions (id, user_id, currency_type, amount, transaction_type, reason, balance_after, source_id, created_at)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)",
            tx_id,
            user_id,
            "Coins" as _,
            amount,
            "Spend" as _,
            reason,
            new_balance,
            item_id,
            Utc::now()
        )
        .execute(&tx)
        .await
        .map_err(|e| e.to_string())?;

        tx.commit().await.map_err(|e| e.to_string())?;

        Ok(true)
    }

    /// Check and update level
    async fn check_level_up(
        pool: &sqlx::Transaction<'_, sqlx::Postgres>,
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
            
            // Award level-up bonus
            award_coins(pool, user_id, (new_level as i64) * 10, "Level up bonus", "level_up", None).await.ok();
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
            52000, 66000, 82000, 100000, 120000, 145000, 175000, 210000, 250000, 300000,
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

        // Get badge details for rewards
        let badge = sqlx::query!("SELECT xp_reward, coin_reward, gem_reward FROM badges WHERE id = $1", badge_id)
            .fetch_one(pool)
            .await
            .map_err(|e| e.to_string())?;

        let id = Uuid::new_v4();

        sqlx::query!(
            "INSERT INTO earned_badges (id, user_id, badge_id, earned_at, is_public)
             VALUES ($1, $2, $3, $4, $5)",
            id,
            user_id,
            badge_id,
            Utc::now(),
            true
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

        // Award coins
        if badge.coin_reward > 0 {
            award_coins(pool, user_id, badge.coin_reward, "Badge reward", "badge", Some(badge_id)).await.ok();
        }

        Ok(EarnedBadge {
            id,
            user_id,
            badge_id,
            earned_at: Utc::now(),
            certificate_id: None,
            is_public: true,
        })
    }

    /// Check badge eligibility
    async fn check_badges(pool: &sqlx::Transaction<'_, sqlx::Postgres>, user_id: uuid::Uuid) -> Result<(), String> {
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
                "quiz_perfect_score" => {
                    let count: i32 = sqlx::query_scalar!(
                        "SELECT COUNT(*) FROM quiz_attempts WHERE user_id = $1 AND score_percent = 100",
                        user_id
                    )
                    .fetch_one(pool)
                    .await
                    .unwrap_or(0);
                    count >= badge.threshold
                }
                _ => false,
            };

            if eligible {
                award_badge(pool, user_id, badge.id).await.ok();
            }
        }

        Ok(())
    }

    async fn get_login_streak(pool: &sqlx::Transaction<'_, sqlx::Postgres>, user_id: uuid::Uuid) -> Result<i32, String> {
        // Check consecutive days
        let count: Option<i32> = sqlx::query_scalar!(
            "SELECT COUNT(DISTINCT DATE(created_at)) FROM login_events 
             WHERE user_id = $1 AND created_at > NOW() - INTERVAL '30 days'",
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
            "SELECT user_id, total_xp, level, streak_days, last_activity_at, coins, gems, completed_quests_count
             FROM gamification_profiles WHERE user_id = $1",
            user_id
        )
        .fetch_one(pool)
        .await
        .map_err(|e| e.to_string())?;

        // Get badges
        let badge_rows = sqlx::query!(
            "SELECT b.id, b.name, b.description, b.icon_url, b.badge_type, b.xp_reward, b.coin_reward, b.gem_reward, b.rarity
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
                badge_type: b.badge_type,
                xp_reward: b.xp_reward,
                coin_reward: b.coin_reward,
                gem_reward: b.gem_reward,
                criteria: BadgeCriteria {
                    criteria_type: String::new(),
                    threshold: 0,
                    course_id: None,
                    additional_data: None,
                },
                rarity: b.rarity,
            })
            .collect();

        // Get active quest IDs
        let quest_ids: Vec<Uuid> = sqlx::query_scalar!(
            "SELECT quest_id FROM user_quests WHERE user_id = $1 AND status = 'Active'",
            user_id
        )
        .fetch_all(pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(GamificationProfile {
            user_id: row.user_id,
            total_xp: row.total_xp,
            level: row.level,
            streak_days: row.streak_days,
            last_activity_at: row.last_activity_at,
            badges,
            coins: row.coins,
            gems: row.gems,
            current_quest_ids: quest_ids,
            completed_quests_count: row.completed_quests_count,
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
                    u.avatar_url, gp.total_xp, gp.level, gp.streak_days,
                    (SELECT COUNT(*) FROM earned_badges eb WHERE eb.user_id = gp.user_id) as badge_count
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
                badge_count: r.badge_count,
            })
            .collect())
    }

    /// Update achievement progress
    async fn update_achievements(
        pool: &sqlx::Transaction<'_, sqlx::Postgres>,
        user_id: uuid::Uuid,
        event_type: &str,
        value: i64,
    ) -> Result<(), String> {
        // Find relevant achievements
        let achievements = sqlx::query!(
            "SELECT id, criteria_type, threshold FROM achievements 
             WHERE criteria_type LIKE $1",
            format!("%{}%", event_type)
        )
        .fetch_all(pool)
        .await
        .map_err(|e| e.to_string())?;

        for achievement in achievements {
            // Update or create user achievement progress
            let current: Option<i64> = sqlx::query_scalar!(
                "SELECT current_progress FROM user_achievements 
                 WHERE user_id = $1 AND achievement_id = $2",
                user_id,
                achievement.id
            )
            .fetch_optional(pool)
            .await
            .map_err(|e| e.to_string())?;

            let new_progress = current.unwrap_or(0) + value;

            if current.is_some() {
                sqlx::query!(
                    "UPDATE user_achievements SET current_progress = $1 WHERE user_id = $2 AND achievement_id = $3",
                    new_progress,
                    user_id,
                    achievement.id
                )
                .execute(pool)
                .await
                .map_err(|e| e.to_string())?;
            } else {
                sqlx::query!(
                    "INSERT INTO user_achievements (id, user_id, achievement_id, current_progress, is_completed, claimed)
                     VALUES ($1, $2, $3, $4, $5, $6)",
                    Uuid::new_v4(),
                    user_id,
                    achievement.id,
                    new_progress,
                    false,
                    false
                )
                .execute(pool)
                .await
                .map_err(|e| e.to_string())?;
            }

            // Check if completed
            if new_progress >= achievement.threshold {
                sqlx::query!(
                    "UPDATE user_achievements SET is_completed = true, completed_at = $1 
                     WHERE user_id = $2 AND achievement_id = $3",
                    Utc::now(),
                    user_id,
                    achievement.id
                )
                .execute(pool)
                .await
                .map_err(|e| e.to_string())?;
            }
        }

        Ok(())
    }

    /// Update quest progress
    async fn update_quest_progress(
        pool: &sqlx::Transaction<'_, sqlx::Postgres>,
        user_id: uuid::Uuid,
        event_type: &str,
        value: i64,
    ) -> Result<(), String> {
        // Get active quests
        let user_quests = sqlx::query!(
            "SELECT id, quest_id FROM user_quests WHERE user_id = $1 AND status = 'Active'",
            user_id
        )
        .fetch_all(pool)
        .await
        .map_err(|e| e.to_string())?;

        for uq in user_quests {
            // Update progress based on quest requirements
            // This is simplified - in production would parse JSON requirements
            sqlx::query!(
                "UPDATE user_quests SET progress = jsonb_set(progress, '{0, current_value}', 
                         (progress->0->>'current_value')::int + $1::text::jsonb)
                 WHERE id = $2",
                value as i32,
                uq.id
            )
            .execute(pool)
            .await
            .ok();
        }

        Ok(())
    }

    /// Spin the wheel
    pub async fn spin_wheel(
        pool: &PgPool,
        user_id: uuid::Uuid,
        wheel_id: uuid::Uuid,
        use_free_spin: bool,
    ) -> Result<SpinHistory, String> {
        let tx = pool.begin().await.map_err(|e| e.to_string())?;

        // Get wheel config
        let wheel = sqlx::query!(
            "SELECT id, spin_cost_coins, spin_cost_gems, daily_free_spins, cooldown_hours 
             FROM spin_wheels WHERE id = $1 AND is_active = true",
            wheel_id
        )
        .fetch_one(&tx)
        .await
        .map_err(|e| e.to_string())?;

        // Check if user can spin
        if !use_free_spin {
            // Deduct cost
            if wheel.spin_cost_gems > 0 {
                // Check and deduct gems
                let gems: i64 = sqlx::query_scalar!(
                    "SELECT gems FROM gamification_profiles WHERE user_id = $1",
                    user_id
                )
                .fetch_one(&tx)
                .await
                .map_err(|e| e.to_string())?;

                if gems < wheel.spin_cost_gems {
                    return Err("Insufficient gems".to_string());
                }

                sqlx::query!(
                    "UPDATE gamification_profiles SET gems = gems - $1 WHERE user_id = $2",
                    wheel.spin_cost_gems,
                    user_id
                )
                .execute(&tx)
                .await
                .map_err(|e| e.to_string())?;
            } else if wheel.spin_cost_coins > 0 {
                spend_coins(&tx, user_id, wheel.spin_cost_coins, "Wheel spin", None).await?;
            }
        }

        // Determine result based on probability
        let segments = sqlx::query!(
            "SELECT id, label, reward_type, reward_value, probability, color, is_jackpot 
             FROM wheel_segments WHERE wheel_id = $1",
            wheel_id
        )
        .fetch_all(&tx)
        .await
        .map_err(|e| e.to_string())?;

        let result = select_weighted_random(&segments);

        // Create spin history
        let spin_id = Uuid::new_v4();
        sqlx::query!(
            "INSERT INTO spin_history (id, user_id, wheel_id, result_segment_id, reward_type, reward_value, spun_at, was_free_spin)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8)",
            spin_id,
            user_id,
            wheel_id,
            result.id,
            result.reward_type,
            result.reward_value,
            Utc::now(),
            use_free_spin
        )
        .execute(&tx)
        .await
        .map_err(|e| e.to_string())?;

        // Award reward
        match result.reward_type.as_str() {
            "XP" => {
                award_xp(&tx, user_id, result.reward_value, "Wheel spin", "wheel", Some(wheel_id)).await.ok();
            }
            "Coins" => {
                award_coins(&tx, user_id, result.reward_value, "Wheel spin", "wheel", Some(wheel_id)).await.ok();
            }
            "Gems" => {
                sqlx::query!(
                    "UPDATE gamification_profiles SET gems = gems + $1 WHERE user_id = $2",
                    result.reward_value,
                    user_id
                )
                .execute(&tx)
                .await
                .ok();
            }
            _ => {}
        }

        tx.commit().await.map_err(|e| e.to_string())?;

        Ok(SpinHistory {
            id: spin_id,
            user_id,
            wheel_id,
            result_segment_id: result.id,
            reward_type: result.reward_type,
            reward_value: result.reward_value,
            spun_at: Utc::now(),
            was_free_spin: use_free_spin,
        })
    }

    fn select_weighted_random(segments: &[sqlx::postgres::PgRow]) -> &sqlx::postgres::PgRow {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let roll: f64 = rng.gen();
        
        let mut cumulative = 0.0;
        for segment in segments {
            let prob: f64 = segment.get("probability");
            cumulative += prob;
            if roll <= cumulative {
                return segment;
            }
        }
        
        segments.last().unwrap()
    }

    /// Purchase shop item
    pub async fn purchase_item(
        pool: &PgPool,
        user_id: uuid::Uuid,
        item_id: uuid::Uuid,
        quantity: i32,
    ) -> Result<InventoryItem, String> {
        let tx = pool.begin().await.map_err(|e| e.to_string())?;

        // Get item details
        let item = sqlx::query!(
            "SELECT id, name, cost_coins, cost_gems, stock_quantity, is_limited 
             FROM shop_items WHERE id = $1",
            item_id
        )
        .fetch_one(&tx)
        .await
        .map_err(|e| e.to_string())?;

        // Check stock
        if let Some(stock) = item.stock_quantity {
            if stock < quantity {
                return Err("Out of stock".to_string());
            }
        }

        // Calculate total cost
        let total_coins = item.cost_coins * quantity as i64;
        let total_gems = item.cost_gems * quantity as i64;

        // Deduct payment
        if total_coins > 0 {
            spend_coins(&tx, user_id, total_coins, &format!("Purchase: {}", item.name), Some(item_id)).await?;
        }
        if total_gems > 0 {
            let gems: i64 = sqlx::query_scalar!(
                "SELECT gems FROM gamification_profiles WHERE user_id = $1",
                user_id
            )
            .fetch_one(&tx)
            .await
            .map_err(|e| e.to_string())?;

            if gems < total_gems {
                return Err("Insufficient gems".to_string());
            }

            sqlx::query!(
                "UPDATE gamification_profiles SET gems = gems - $1 WHERE user_id = $2",
                total_gems,
                user_id
            )
            .execute(&tx)
            .await
            .map_err(|e| e.to_string())?;
        }

        // Add to inventory
        let inv_id = Uuid::new_v4();
        sqlx::query!(
            "INSERT INTO inventory_items (id, user_id, item_id, quantity, acquired_at, is_used)
             VALUES ($1, $2, $3, $4, $5, $6)",
            inv_id,
            user_id,
            item_id,
            quantity,
            Utc::now(),
            false
        )
        .execute(&tx)
        .await
        .map_err(|e| e.to_string())?;

        // Update stock if limited
        if item.is_limited {
            sqlx::query!(
                "UPDATE shop_items SET stock_quantity = stock_quantity - $1 WHERE id = $2",
                quantity,
                item_id
            )
            .execute(&tx)
            .await
            .map_err(|e| e.to_string())?;
        }

        tx.commit().await.map_err(|e| e.to_string())?;

        Ok(InventoryItem {
            id: inv_id,
            user_id,
            item_id,
            quantity,
            acquired_at: Utc::now(),
            expires_at: None,
            is_used: false,
        })
    }

    /// Activate power-up
    pub async fn activate_power_up(
        pool: &PgPool,
        user_id: uuid::Uuid,
        power_up_id: uuid::Uuid,
    ) -> Result<ActivePowerUp, String> {
        let tx = pool.begin().await.map_err(|e| e.to_string())?;

        // Get power-up details
        let power = sqlx::query!(
            "SELECT id, name, duration_hours FROM power_ups WHERE id = $1",
            power_up_id
        )
        .fetch_one(&tx)
        .await
        .map_err(|e| e.to_string())?;

        // Check if user has this item in inventory
        let has_item = sqlx::query!(
            "SELECT id, quantity FROM inventory_items 
             WHERE user_id = $1 AND item_id = $2 AND is_used = false",
            user_id,
            power_up_id
        )
        .fetch_optional(&tx)
        .await
        .map_err(|e| e.to_string())?;

        if has_item.is_none() {
            return Err("Power-up not in inventory".to_string());
        }

        let item = has_item.unwrap();

        // Mark item as used
        sqlx::query!(
            "UPDATE inventory_items SET is_used = true WHERE id = $1",
            item.id
        )
        .execute(&tx)
        .await
        .map_err(|e| e.to_string())?;

        // Create active power-up
        let active_id = Uuid::new_v4();
        let expires_at = Utc::now() + Duration::hours(power.duration_hours as i64);

        sqlx::query!(
            "INSERT INTO active_power_ups (id, user_id, power_up_id, activated_at, expires_at, is_active)
             VALUES ($1, $2, $3, $4, $5, $6)",
            active_id,
            user_id,
            power_up_id,
            Utc::now(),
            expires_at,
            true
        )
        .execute(&tx)
        .await
        .map_err(|e| e.to_string())?;

        tx.commit().await.map_err(|e| e.to_string())?;

        Ok(ActivePowerUp {
            id: active_id,
            user_id,
            power_up_id,
            activated_at: Utc::now(),
            expires_at,
            is_active: true,
        })
    }

    /// Join competition
    pub async fn join_competition(
        pool: &PgPool,
        user_id: uuid::Uuid,
        competition_id: uuid::Uuid,
    ) -> Result<CompetitionParticipant, String> {
        let tx = pool.begin().await.map_err(|e| e.to_string())?;

        // Check competition exists and is active
        let comp = sqlx::query!(
            "SELECT id, name, start_date, end_date, participant_limit FROM competitions 
             WHERE id = $1",
            competition_id
        )
        .fetch_one(&tx)
        .await
        .map_err(|e| e.to_string())?;

        let now = Utc::now();
        if now < comp.start_date || now > comp.end_date {
            return Err("Competition is not active".to_string());
        }

        // Check participant limit
        if let Some(limit) = comp.participant_limit {
            let count: i64 = sqlx::query_scalar!(
                "SELECT COUNT(*) FROM competition_participants WHERE competition_id = $1",
                competition_id
            )
            .fetch_one(&tx)
            .await
            .map_err(|e| e.to_string())?;

            if count >= limit as i64 {
                return Err("Competition is full".to_string());
            }
        }

        // Check if already joined
        let existing = sqlx::query!(
            "SELECT id FROM competition_participants WHERE competition_id = $1 AND user_id = $2",
            competition_id,
            user_id
        )
        .fetch_optional(&tx)
        .await
        .map_err(|e| e.to_string())?;

        if existing.is_some() {
            return Err("Already joined competition".to_string());
        }

        // Add participant
        let part_id = Uuid::new_v4();
        sqlx::query!(
            "INSERT INTO competition_participants (id, competition_id, user_id, score, joined_at, prize_awarded)
             VALUES ($1, $2, $3, $4, $5, $6)",
            part_id,
            competition_id,
            user_id,
            0,
            Utc::now(),
            false
        )
        .execute(&tx)
        .await
        .map_err(|e| e.to_string())?;

        tx.commit().await.map_err(|e| e.to_string())?;

        Ok(CompetitionParticipant {
            id: part_id,
            competition_id,
            user_id,
            score: 0,
            rank: None,
            prize_awarded: false,
            joined_at: Utc::now(),
        })
    }

    /// Accept quest
    pub async fn accept_quest(
        pool: &PgPool,
        user_id: uuid::Uuid,
        quest_id: uuid::Uuid,
    ) -> Result<UserQuest, String> {
        let tx = pool.begin().await.map_err(|e| e.to_string())?;

        // Get quest details
        let quest = sqlx::query!(
            "SELECT id, title, quest_type, time_limit_days, is_repeatable FROM quests WHERE id = $1",
            quest_id
        )
        .fetch_one(&tx)
        .await
        .map_err(|e| e.to_string())?;

        // Check if repeatable or not already active/completed
        if !quest.is_repeatable {
            let existing = sqlx::query!(
                "SELECT id FROM user_quests WHERE user_id = $1 AND quest_id = $2",
                user_id,
                quest_id
            )
            .fetch_optional(&tx)
            .await
            .map_err(|e| e.to_string())?;

            if existing.is_some() {
                return Err("Quest already accepted".to_string());
            }
        }

        // Create user quest
        let uq_id = Uuid::new_v4();
        sqlx::query!(
            "INSERT INTO user_quests (id, user_id, quest_id, status, started_at, claimed, progress)
             VALUES ($1, $2, $3, $4, $5, $6, $7)",
            uq_id,
            user_id,
            quest_id,
            "Active" as _,
            Utc::now(),
            false,
            serde_json::Value::Array(vec![])
        )
        .execute(&tx)
        .await
        .map_err(|e| e.to_string())?;

        tx.commit().await.map_err(|e| e.to_string())?;

        Ok(UserQuest {
            id: uq_id,
            user_id,
            quest_id,
            status: QuestStatus::Active,
            started_at: Utc::now(),
            completed_at: None,
            progress: vec![],
            claimed: false,
        })
    }
}
