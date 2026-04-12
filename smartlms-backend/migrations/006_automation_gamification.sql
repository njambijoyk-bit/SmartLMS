-- Phase 10: Automation & Gamification Database Schema
-- Migration 006: Automation rules, gamification tables

-- ==================== AUTOMATION RULES ====================

CREATE TABLE IF NOT EXISTS automation_rules (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    institution_id UUID NOT NULL REFERENCES institutions(id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    trigger JSONB NOT NULL, -- { "event_type": "UserRegistered" } or { "cron_expression": "...", "timezone": "UTC" }
    conditions JSONB NOT NULL DEFAULT '[]', -- Array of rule conditions
    actions JSONB NOT NULL DEFAULT '[]', -- Array of actions to execute
    is_active BOOLEAN DEFAULT false,
    execution_count BIGINT DEFAULT 0,
    last_executed_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_automation_rules_institution ON automation_rules(institution_id);
CREATE INDEX idx_automation_rules_active ON automation_rules(is_active) WHERE is_active = true;
CREATE INDEX idx_automation_rules_trigger ON automation_rules USING GIN(trigger);

CREATE TABLE IF NOT EXISTS automation_execution_logs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    rule_id UUID NOT NULL REFERENCES automation_rules(id) ON DELETE CASCADE,
    trigger_event VARCHAR(100) NOT NULL,
    conditions_met BOOLEAN DEFAULT false,
    actions_executed JSONB DEFAULT '[]',
    error TEXT,
    event_data JSONB,
    executed_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_automation_logs_rule ON automation_execution_logs(rule_id);
CREATE INDEX idx_automation_logs_executed ON automation_execution_logs(executed_at);

-- ==================== GAMIFICATION - XP & LEVELS ====================

CREATE TABLE IF NOT EXISTS gamification_profiles (
    user_id UUID PRIMARY KEY REFERENCES users(id) ON DELETE CASCADE,
    total_xp BIGINT DEFAULT 0,
    level INTEGER DEFAULT 1,
    streak_days INTEGER DEFAULT 0,
    last_activity_at TIMESTAMPTZ,
    coins BIGINT DEFAULT 0,
    gems BIGINT DEFAULT 0,
    completed_quests_count INTEGER DEFAULT 0,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_gamification_profiles_total_xp ON gamification_profiles(total_xp DESC);
CREATE INDEX idx_gamification_profiles_level ON gamification_profiles(level DESC);

CREATE TABLE IF NOT EXISTS xp_transactions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    amount BIGINT NOT NULL,
    reason VARCHAR(255) NOT NULL,
    source_type VARCHAR(100) NOT NULL, -- course_completed, quiz_passed, login_bonus, etc.
    source_id UUID,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_xp_transactions_user ON xp_transactions(user_id);
CREATE INDEX idx_xp_transactions_created ON xp_transactions(created_at);

-- ==================== GAMIFICATION - BADGES ====================

CREATE TABLE IF NOT EXISTS badges (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    institution_id UUID REFERENCES institutions(id) ON DELETE SET NULL,
    name VARCHAR(255) NOT NULL,
    description TEXT NOT NULL,
    icon_url VARCHAR(500),
    badge_type VARCHAR(50) NOT NULL, -- achievement, milestone, streak, completion, special, secret, event
    xp_reward BIGINT DEFAULT 0,
    coin_reward BIGINT DEFAULT 0,
    gem_reward BIGINT DEFAULT 0,
    criteria JSONB NOT NULL, -- { "criteria_type": "courses_completed", "threshold": 10 }
    rarity VARCHAR(50) DEFAULT 'common', -- common, uncommon, rare, epic, legendary
    is_active BOOLEAN DEFAULT true,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_badges_institution ON badges(institution_id);
CREATE INDEX idx_badges_type ON badges(badge_type);
CREATE INDEX idx_badges_rarity ON badges(rarity);

CREATE TABLE IF NOT EXISTS earned_badges (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    badge_id UUID NOT NULL REFERENCES badges(id) ON DELETE CASCADE,
    earned_at TIMESTAMPTZ DEFAULT NOW(),
    certificate_id UUID,
    is_public BOOLEAN DEFAULT true,
    UNIQUE(user_id, badge_id)
);

CREATE INDEX idx_earned_badges_user ON earned_badges(user_id);
CREATE INDEX idx_earned_badges_badge ON earned_badges(badge_id);

-- ==================== GAMIFICATION - LEADERBOARDS ====================

CREATE TABLE IF NOT EXISTS leaderboard_entries (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    institution_id UUID NOT NULL REFERENCES institutions(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    period VARCHAR(50) NOT NULL, -- daily, weekly, monthly, all_time
    rank INTEGER NOT NULL,
    total_xp BIGINT NOT NULL,
    level INTEGER NOT NULL,
    streak_days INTEGER NOT NULL,
    badge_count INTEGER DEFAULT 0,
    period_start DATE NOT NULL,
    period_end DATE,
    calculated_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(institution_id, user_id, period, period_start)
);

CREATE INDEX idx_leaderboard_institution_period ON leaderboard_entries(institution_id, period, period_start);
CREATE INDEX idx_leaderboard_rank ON leaderboard_entries(institution_id, period, period_start, rank);

-- ==================== GAMIFICATION - QUESTS ====================

CREATE TABLE IF NOT EXISTS quests (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    institution_id UUID REFERENCES institutions(id) ON DELETE SET NULL,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    quest_type VARCHAR(50) NOT NULL, -- daily, weekly, one_time, recurring
    objectives JSONB NOT NULL, -- Array of objectives to complete
    xp_reward BIGINT DEFAULT 0,
    coin_reward BIGINT DEFAULT 0,
    gem_reward BIGINT DEFAULT 0,
    badge_id UUID REFERENCES badges(id),
    difficulty VARCHAR(50) DEFAULT 'medium', -- easy, medium, hard, expert
    time_limit_hours INTEGER, -- NULL means no time limit
    is_active BOOLEAN DEFAULT true,
    repeatable BOOLEAN DEFAULT false,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_quests_institution ON quests(institution_id);
CREATE INDEX idx_quests_type ON quests(quest_type);
CREATE INDEX idx_quests_active ON quests(is_active) WHERE is_active = true;

CREATE TABLE IF NOT EXISTS user_quests (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    quest_id UUID NOT NULL REFERENCES quests(id) ON DELETE CASCADE,
    status VARCHAR(50) DEFAULT 'active', -- active, completed, failed, expired
    progress JSONB DEFAULT '[]', -- Progress on each objective
    started_at TIMESTAMPTZ DEFAULT NOW(),
    completed_at TIMESTAMPTZ,
    expires_at TIMESTAMPTZ,
    UNIQUE(user_id, quest_id) WHERE status = 'active'
);

CREATE INDEX idx_user_quests_user ON user_quests(user_id);
CREATE INDEX idx_user_quests_status ON user_quests(status);
CREATE INDEX idx_user_quests_expires ON user_quests(expires_at) WHERE status = 'active';

-- ==================== GAMIFICATION - VIRTUAL ECONOMY ====================

CREATE TABLE IF NOT EXISTS shop_items (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    institution_id UUID REFERENCES institutions(id) ON DELETE SET NULL,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    item_type VARCHAR(50) NOT NULL, -- power_up, avatar, theme, course_discount, certificate_frame, etc.
    coin_price BIGINT DEFAULT 0,
    gem_price BIGINT DEFAULT 0,
    stock_quantity INTEGER, -- NULL means unlimited
    max_per_user INTEGER, -- NULL means no limit
    metadata JSONB, -- Additional item-specific data
    is_active BOOLEAN DEFAULT true,
    available_from TIMESTAMPTZ,
    available_until TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_shop_items_institution ON shop_items(institution_id);
CREATE INDEX idx_shop_items_type ON shop_items(item_type);
CREATE INDEX idx_shop_items_active ON shop_items(is_active) WHERE is_active = true;

CREATE TABLE IF NOT EXISTS user_inventory (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    item_id UUID NOT NULL REFERENCES shop_items(id) ON DELETE CASCADE,
    quantity INTEGER DEFAULT 1,
    acquired_at TIMESTAMPTZ DEFAULT NOW(),
    expires_at TIMESTAMPTZ, -- For items with expiration
    is_used BOOLEAN DEFAULT false,
    metadata JSONB
);

CREATE INDEX idx_user_inventory_user ON user_inventory(user_id);
CREATE INDEX idx_user_inventory_item ON user_inventory(item_id);

CREATE TABLE IF NOT EXISTS coin_transactions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    amount BIGINT NOT NULL, -- Positive for earning, negative for spending
    transaction_type VARCHAR(50) NOT NULL, -- award, spend, purchase, reward, penalty
    reason VARCHAR(255) NOT NULL,
    source_type VARCHAR(100), -- quest_completed, badge_earned, shop_purchase, etc.
    source_id UUID,
    balance_after BIGINT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_coin_transactions_user ON coin_transactions(user_id);
CREATE INDEX idx_coin_transactions_created ON coin_transactions(created_at);

-- ==================== GAMIFICATION - POWER-UPS ====================

CREATE TABLE IF NOT EXISTS power_ups (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) NOT NULL,
    description TEXT,
    effect_type VARCHAR(100) NOT NULL, -- double_xp, streak_freeze, hint, time_extension, etc.
    effect_value JSONB, -- Configuration for the effect
    duration_minutes INTEGER, -- How long the power-up lasts when activated
    coin_cost BIGINT DEFAULT 0,
    gem_cost BIGINT DEFAULT 0,
    is_active BOOLEAN DEFAULT true,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS active_power_ups (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    power_up_id UUID NOT NULL REFERENCES power_ups(id) ON DELETE CASCADE,
    activated_at TIMESTAMPTZ DEFAULT NOW(),
    expires_at TIMESTAMPTZ NOT NULL,
    is_active BOOLEAN DEFAULT true
);

CREATE INDEX idx_active_power_ups_user ON active_power_ups(user_id);
CREATE INDEX idx_active_power_ups_expires ON active_power_ups(expires_at);

-- ==================== GAMIFICATION - SPIN WHEEL ====================

CREATE TABLE IF NOT EXISTS spin_wheels (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    institution_id UUID REFERENCES institutions(id) ON DELETE SET NULL,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    segments JSONB NOT NULL, -- Array of { prize_type, prize_value, probability, weight }
    cost_coins BIGINT DEFAULT 0,
    cost_gems BIGINT DEFAULT 0,
    free_spins_per_day INTEGER DEFAULT 1,
    is_active BOOLEAN DEFAULT true,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS spin_history (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    wheel_id UUID NOT NULL REFERENCES spin_wheels(id) ON DELETE CASCADE,
    result_segment INTEGER NOT NULL, -- Index of the segment landed on
    prize_type VARCHAR(50) NOT NULL, -- coins, gems, xp, badge, item, nothing
    prize_value JSONB, -- Details of the prize won
    used_free_spin BOOLEAN DEFAULT false,
    spun_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_spin_history_user ON spin_history(user_id);
CREATE INDEX idx_spin_history_wheel ON spin_history(wheel_id);
CREATE INDEX idx_spin_history_spun ON spin_history(spun_at);

-- ==================== GAMIFICATION - COMPETITIONS ====================

CREATE TABLE IF NOT EXISTS competitions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    institution_id UUID NOT NULL REFERENCES institutions(id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    competition_type VARCHAR(50) NOT NULL, -- xp_race, badge_collection, quest_marathon, etc.
    start_date TIMESTAMPTZ NOT NULL,
    end_date TIMESTAMPTZ NOT NULL,
    prize_pool JSONB, -- { coins: 1000, gems: 100, badges: [...] }
    ranking_criteria VARCHAR(100) DEFAULT 'total_xp', -- total_xp, badges_earned, quests_completed
    min_level INTEGER DEFAULT 1,
    max_participants INTEGER,
    is_team_based BOOLEAN DEFAULT false,
    is_active BOOLEAN DEFAULT true,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_competitions_institution ON competitions(institution_id);
CREATE INDEX idx_competitions_dates ON competitions(start_date, end_date);
CREATE INDEX idx_competitions_active ON competitions(is_active) WHERE is_active = true;

CREATE TABLE IF NOT EXISTS competition_participants (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    competition_id UUID NOT NULL REFERENCES competitions(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    team_id UUID, -- For team-based competitions
    current_score BIGINT DEFAULT 0,
    rank INTEGER,
    joined_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(competition_id, user_id)
);

CREATE INDEX idx_competition_participants_competition ON competition_participants(competition_id);
CREATE INDEX idx_competition_participants_user ON competition_participants(user_id);
CREATE INDEX idx_competition_participants_rank ON competition_participants(competition_id, rank);

-- ==================== GAMIFICATION - ACHIEVEMENTS ====================

CREATE TABLE IF NOT EXISTS achievements (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    institution_id UUID REFERENCES institutions(id) ON DELETE SET NULL,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    achievement_type VARCHAR(50) NOT NULL, -- learning, social, consistency, mastery, special
    criteria JSONB NOT NULL, -- Conditions to unlock
    xp_reward BIGINT DEFAULT 0,
    coin_reward BIGINT DEFAULT 0,
    badge_id UUID REFERENCES badges(id),
    is_hidden BOOLEAN DEFAULT false, -- Hidden until earned
    is_active BOOLEAN DEFAULT true,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_achievements_institution ON achievements(institution_id);
CREATE INDEX idx_achievements_type ON achievements(achievement_type);

CREATE TABLE IF NOT EXISTS user_achievements (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    achievement_id UUID NOT NULL REFERENCES achievements(id) ON DELETE CASCADE,
    earned_at TIMESTAMPTZ DEFAULT NOW(),
    progress_percentage INTEGER DEFAULT 100,
    UNIQUE(user_id, achievement_id)
);

CREATE INDEX idx_user_achievements_user ON user_achievements(user_id);
CREATE INDEX idx_user_achievements_earned ON user_achievements(earned_at);

-- ==================== GAMIFICATION - STREAK TRACKING ====================

CREATE TABLE IF NOT EXISTS user_streaks (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    streak_type VARCHAR(50) NOT NULL, -- login, study, exercise, etc.
    current_streak INTEGER DEFAULT 0,
    longest_streak INTEGER DEFAULT 0,
    last_activity_date DATE NOT NULL,
    streak_freezes_available INTEGER DEFAULT 0,
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(user_id, streak_type)
);

CREATE INDEX idx_user_streaks_user ON user_streaks(user_id);
CREATE INDEX idx_user_streaks_current ON user_streaks(current_streak DESC);

-- ==================== INSERT DEFAULT DATA ====================

-- Default badges
INSERT INTO badges (name, description, badge_type, xp_reward, coin_reward, rarity, criteria) VALUES
('First Steps', 'Complete your first course', 'milestone', 100, 50, 'common', '{"criteria_type": "courses_completed", "threshold": 1}'),
('Quick Learner', 'Complete 5 courses', 'milestone', 500, 200, 'uncommon', '{"criteria_type": "courses_completed", "threshold": 5}'),
('Scholar', 'Complete 20 courses', 'milestone', 2000, 1000, 'rare', '{"criteria_type": "courses_completed", "threshold": 20}'),
('Perfect Score', 'Get 100% on a quiz', 'achievement', 200, 100, 'uncommon', '{"criteria_type": "quiz_score", "threshold": 100}'),
('Week Warrior', '7-day login streak', 'streak', 300, 150, 'rare', '{"criteria_type": "login_streak", "threshold": 7}'),
('Month Master', '30-day login streak', 'streak', 1500, 750, 'epic', '{"criteria_type": "login_streak", "threshold": 30}'),
('Year Legend', '365-day login streak', 'streak', 10000, 5000, 'legendary', '{"criteria_type": "login_streak", "threshold": 365}'),
('Quiz Champion', 'Complete 50 quizzes', 'completion', 1000, 500, 'rare', '{"criteria_type": "quizzes_completed", "threshold": 50}'),
('Assignment Ace', 'Submit 25 assignments on time', 'achievement', 800, 400, 'uncommon', '{"criteria_type": "assignments_on_time", "threshold": 25}'),
('Social Butterfly', 'Participate in 10 discussions', 'social', 400, 200, 'common', '{"criteria_type": "discussions_participated", "threshold": 10}')
ON CONFLICT DO NOTHING;

-- Default power-ups
INSERT INTO power_ups (name, description, effect_type, duration_minutes, coin_cost, gem_cost) VALUES
('Double XP', 'Earn double XP for 30 minutes', 'double_xp', 30, 500, 50),
('Streak Freeze', 'Protect your streak for one day', 'streak_freeze', 1440, 200, 20),
('Hint Master', 'Get 5 hints for quizzes', 'hint', null, 100, 10),
('Time Extension', 'Add 30 minutes to quiz timer', 'time_extension', null, 150, 15),
('Focus Mode', 'Block distractions for 60 minutes', 'focus', 60, 300, 30)
ON CONFLICT DO NOTHING;

-- Default spin wheel
INSERT INTO spin_wheels (name, description, segments, cost_coins, cost_gems, free_spins_per_day) VALUES
('Daily Luck Wheel', 'Spin daily for chance to win prizes!', 
 '[{"prize_type": "coins", "prize_value": 100, "probability": 0.3, "weight": 30},
   {"prize_type": "coins", "prize_value": 500, "probability": 0.15, "weight": 15},
   {"prize_type": "gems", "prize_value": 10, "probability": 0.2, "weight": 20},
   {"prize_type": "xp", "prize_value": 200, "probability": 0.2, "weight": 20},
   {"prize_type": "nothing", "prize_value": 0, "probability": 0.1, "weight": 10},
   {"prize_type": "badge", "prize_value": "random_common", "probability": 0.05, "weight": 5}]',
 100, 10, 1)
ON CONFLICT DO NOTHING;

-- Default shop items
INSERT INTO shop_items (name, description, item_type, coin_price, gem_price, metadata) VALUES
('Avatar Frame - Gold', 'Exclusive gold frame for your avatar', 'avatar', 1000, 100, '{"frame_color": "gold"}'),
('Avatar Frame - Silver', 'Silver frame for your avatar', 'avatar', 500, 50, '{"frame_color": "silver"}'),
('Dark Theme', 'Premium dark theme for the platform', 'theme', 2000, 200, '{"theme_id": "dark_premium"}'),
('Course Discount 10%', '10% discount on any course', 'course_discount', 1500, 150, '{"discount_percent": 10}'),
('Certificate Frame - Premium', 'Premium frame for certificates', 'certificate_frame', 800, 80, '{"frame_style": "premium"}'),
('XP Boost (1 hour)', 'Temporary XP boost for 1 hour', 'power_up', 600, 60, '{"boost_multiplier": 1.5, "duration_minutes": 60}')
ON CONFLICT DO NOTHING;

-- Sample quests
INSERT INTO quests (name, description, quest_type, objectives, xp_reward, coin_reward, difficulty, time_limit_hours) VALUES
('Daily Login', 'Log in today', 'daily', '[{"type": "login", "target": 1}]', 50, 25, 'easy', 24),
('Weekly Learner', 'Complete 3 lessons this week', 'weekly', '[{"type": "lessons_completed", "target": 3}]', 300, 150, 'medium', 168),
('Quiz Master', 'Score above 80% on 5 quizzes', 'one_time', '[{"type": "quiz_score_above", "target": 5, "min_score": 80}]', 500, 250, 'hard', null),
('Course Finisher', 'Complete a full course', 'one_time', '[{"type": "course_completed", "target": 1}]', 1000, 500, 'medium', null),
('Social Contributor', 'Post 10 discussion messages', 'weekly', '[{"type": "discussion_posts", "target": 10}]', 200, 100, 'easy', 168)
ON CONFLICT DO NOTHING;

COMMENT ON TABLE automation_rules IS 'Stores automation rules with triggers, conditions, and actions';
COMMENT ON TABLE gamification_profiles IS 'User gamification profiles with XP, levels, and currency';
COMMENT ON TABLE badges IS 'Available badges that users can earn';
COMMENT ON TABLE earned_badges IS 'Badges earned by users';
COMMENT ON TABLE quests IS 'Available quests for users to complete';
COMMENT ON TABLE user_quests IS 'User progress on quests';
COMMENT ON TABLE shop_items IS 'Items available for purchase in the virtual shop';
COMMENT ON TABLE user_inventory IS 'Items owned by users';
COMMENT ON TABLE competitions IS 'Gamification competitions and challenges';
COMMENT ON TABLE spin_wheels IS 'Luck wheels for random prize distribution';
