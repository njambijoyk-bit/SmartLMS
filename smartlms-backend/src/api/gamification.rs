// Gamification API - XP, badges, leaderboards, quests, shop, and competitions
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    app_state::AppState,
    services::gamification::{self, service::*},
};

// ==================== REQUEST/RESPONSE MODELS ====================

#[derive(Debug, Deserialize)]
pub struct AwardXpRequest {
    pub amount: i64,
    pub reason: String,
    pub source_type: String,
    pub source_id: Option<Uuid>,
}

#[derive(Debug, Deserialize)]
pub struct AwardCoinsRequest {
    pub amount: i64,
    pub reason: String,
    pub source_type: String,
    pub source_id: Option<Uuid>,
}

#[derive(Debug, Deserialize)]
pub struct SpendCoinsRequest {
    pub amount: i64,
    pub reason: String,
    pub item_id: Option<Uuid>,
}

#[derive(Debug, Deserialize)]
pub struct SpinWheelRequest {
    pub wheel_id: Uuid,
    pub use_free_spin: bool,
}

#[derive(Debug, Deserialize)]
pub struct PurchaseItemRequest {
    pub item_id: Uuid,
    pub quantity: i32,
}

#[derive(Debug, Deserialize)]
pub struct ActivatePowerUpRequest {
    pub power_up_id: Uuid,
}

#[derive(Debug, Deserialize)]
pub struct JoinCompetitionRequest {
    pub competition_id: Uuid,
}

#[derive(Debug, Deserialize)]
pub struct AcceptQuestRequest {
    pub quest_id: Uuid,
}

#[derive(Debug, Serialize)]
pub struct ApiResponse<T: Serialize> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

// ==================== API HANDLERS ====================

/// Get user's gamification profile
pub async fn get_profile(
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>,
) -> Result<Json<ApiResponse<gamification::GamificationProfile>>, StatusCode> {
    match service::get_profile(&state.db_pool, user_id).await {
        Ok(profile) => Ok(Json(ApiResponse {
            success: true,
            data: Some(profile),
            error: None,
        })),
        Err(e) => Ok(Json(ApiResponse {
            success: false,
            data: None,
            error: Some(e),
        })),
    }
}

/// Award XP to user
pub async fn award_xp(
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>,
    Json(req): Json<AwardXpRequest>,
) -> Result<Json<ApiResponse<gamification::GamificationProfile>>, StatusCode> {
    match service::award_xp(
        &state.db_pool,
        user_id,
        req.amount,
        &req.reason,
        &req.source_type,
        req.source_id,
    )
    .await
    {
        Ok(profile) => Ok(Json(ApiResponse {
            success: true,
            data: Some(profile),
            error: None,
        })),
        Err(e) => Ok(Json(ApiResponse {
            success: false,
            data: None,
            error: Some(e),
        })),
    }
}

/// Award coins to user
pub async fn award_coins(
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>,
    Json(req): Json<AwardCoinsRequest>,
) -> Result<Json<ApiResponse<i64>>, StatusCode> {
    match service::award_coins(
        &state.db_pool,
        user_id,
        req.amount,
        &req.reason,
        &req.source_type,
        req.source_id,
    )
    .await
    {
        Ok(balance) => Ok(Json(ApiResponse {
            success: true,
            data: Some(balance),
            error: None,
        })),
        Err(e) => Ok(Json(ApiResponse {
            success: false,
            data: None,
            error: Some(e),
        })),
    }
}

/// Spend coins from user balance
pub async fn spend_coins(
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>,
    Json(req): Json<SpendCoinsRequest>,
) -> Result<Json<ApiResponse<bool>>, StatusCode> {
    match service::spend_coins(&state.db_pool, user_id, req.amount, &req.reason, req.item_id).await {
        Ok(success) => Ok(Json(ApiResponse {
            success: true,
            data: Some(success),
            error: None,
        })),
        Err(e) => Ok(Json(ApiResponse {
            success: false,
            data: None,
            error: Some(e),
        })),
    }
}

/// Award badge to user
pub async fn award_badge(
    State(state): State<AppState>,
    Path((user_id, badge_id)): Path<(Uuid, Uuid)>,
) -> Result<Json<ApiResponse<gamification::EarnedBadge>>, StatusCode> {
    match service::award_badge(&state.db_pool, user_id, badge_id).await {
        Ok(badge) => Ok(Json(ApiResponse {
            success: true,
            data: Some(badge),
            error: None,
        })),
        Err(e) => Ok(Json(ApiResponse {
            success: false,
            data: None,
            error: Some(e),
        })),
    }
}

/// Get leaderboard for institution
pub async fn get_leaderboard(
    State(state): State<AppState>,
    Path((institution_id, limit)): Path<(Uuid, i64)>,
) -> Result<Json<ApiResponse<Vec<gamification::LeaderboardEntry>>>, StatusCode> {
    match service::get_leaderboard(&state.db_pool, institution_id, limit).await {
        Ok(entries) => Ok(Json(ApiResponse {
            success: true,
            data: Some(entries),
            error: None,
        })),
        Err(e) => Ok(Json(ApiResponse {
            success: false,
            data: None,
            error: Some(e),
        })),
    }
}

/// Spin the wheel
pub async fn spin_wheel(
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>,
    Json(req): Json<SpinWheelRequest>,
) -> Result<Json<ApiResponse<gamification::SpinHistory>>, StatusCode> {
    match service::spin_wheel(&state.db_pool, user_id, req.wheel_id, req.use_free_spin).await {
        Ok(result) => Ok(Json(ApiResponse {
            success: true,
            data: Some(result),
            error: None,
        })),
        Err(e) => Ok(Json(ApiResponse {
            success: false,
            data: None,
            error: Some(e),
        })),
    }
}

/// Purchase shop item
pub async fn purchase_item(
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>,
    Json(req): Json<PurchaseItemRequest>,
) -> Result<Json<ApiResponse<gamification::InventoryItem>>, StatusCode> {
    match service::purchase_item(&state.db_pool, user_id, req.item_id, req.quantity).await {
        Ok(item) => Ok(Json(ApiResponse {
            success: true,
            data: Some(item),
            error: None,
        })),
        Err(e) => Ok(Json(ApiResponse {
            success: false,
            data: None,
            error: Some(e),
        })),
    }
}

/// Activate power-up
pub async fn activate_power_up(
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>,
    Json(req): Json<ActivatePowerUpRequest>,
) -> Result<Json<ApiResponse<gamification::ActivePowerUp>>, StatusCode> {
    match service::activate_power_up(&state.db_pool, user_id, req.power_up_id).await {
        Ok(power_up) => Ok(Json(ApiResponse {
            success: true,
            data: Some(power_up),
            error: None,
        })),
        Err(e) => Ok(Json(ApiResponse {
            success: false,
            data: None,
            error: Some(e),
        })),
    }
}

/// Join competition
pub async fn join_competition(
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>,
    Json(req): Json<JoinCompetitionRequest>,
) -> Result<Json<ApiResponse<gamification::CompetitionParticipant>>, StatusCode> {
    match service::join_competition(&state.db_pool, user_id, req.competition_id).await {
        Ok(participant) => Ok(Json(ApiResponse {
            success: true,
            data: Some(participant),
            error: None,
        })),
        Err(e) => Ok(Json(ApiResponse {
            success: false,
            data: None,
            error: Some(e),
        })),
    }
}

/// Accept quest
pub async fn accept_quest(
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>,
    Json(req): Json<AcceptQuestRequest>,
) -> Result<Json<ApiResponse<gamification::UserQuest>>, StatusCode> {
    match service::accept_quest(&state.db_pool, user_id, req.quest_id).await {
        Ok(quest) => Ok(Json(ApiResponse {
            success: true,
            data: Some(quest),
            error: None,
        })),
        Err(e) => Ok(Json(ApiResponse {
            success: false,
            data: None,
            error: Some(e),
        })),
    }
}

/// Get user's active quests
pub async fn get_user_quests(
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>,
) -> Result<Json<ApiResponse<Vec<gamification::UserQuest>>>, StatusCode> {
    // Implementation would query database for user's quests
    Ok(Json(ApiResponse {
        success: true,
        data: Some(vec![]),
        error: None,
    }))
}

/// Get available quests
pub async fn get_available_quests(
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>,
) -> Result<Json<ApiResponse<Vec<gamification::Quest>>>, StatusCode> {
    // Implementation would query database for available quests
    Ok(Json(ApiResponse {
        success: true,
        data: Some(vec![]),
        error: None,
    }))
}

/// Get shop items
pub async fn get_shop_items(
    State(_state): State<AppState>,
) -> Result<Json<ApiResponse<Vec<gamification::ShopItem>>>, StatusCode> {
    // Implementation would query database for shop items
    Ok(Json(ApiResponse {
        success: true,
        data: Some(vec![]),
        error: None,
    }))
}

/// Get user's inventory
pub async fn get_inventory(
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>,
) -> Result<Json<ApiResponse<Vec<gamification::InventoryItem>>>, StatusCode> {
    // Implementation would query database for user's inventory
    Ok(Json(ApiResponse {
        success: true,
        data: Some(vec![]),
        error: None,
    }))
}

/// Get active competitions
pub async fn get_active_competitions(
    State(_state): State<AppState>,
) -> Result<Json<ApiResponse<Vec<gamification::Competition>>>, StatusCode> {
    // Implementation would query database for active competitions
    Ok(Json(ApiResponse {
        success: true,
        data: Some(vec![]),
        error: None,
    }))
}

/// Get user's achievements
pub async fn get_user_achievements(
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>,
) -> Result<Json<ApiResponse<Vec<gamification::UserAchievement>>>, StatusCode> {
    // Implementation would query database for user's achievements
    Ok(Json(ApiResponse {
        success: true,
        data: Some(vec![]),
        error: None,
    }))
}

/// Get available achievements
pub async fn get_available_achievements(
    State(_state): State<AppState>,
) -> Result<Json<ApiResponse<Vec<gamification::Achievement>>>, StatusCode> {
    // Implementation would query database for available achievements
    Ok(Json(ApiResponse {
        success: true,
        data: Some(vec![]),
        error: None,
    }))
}

// ==================== ROUTER CONFIGURATION ====================

use axum::routing::{get, post};
use axum::Router;

pub fn create_router() -> Router<AppState> {
    Router::new()
        // Profile endpoints
        .route("/gamification/profile/:user_id", get(get_profile))
        // XP endpoints
        .route("/gamification/:user_id/xp/award", post(award_xp))
        // Coin endpoints
        .route("/gamification/:user_id/coins/award", post(award_coins))
        .route("/gamification/:user_id/coins/spend", post(spend_coins))
        // Badge endpoints
        .route("/gamification/:user_id/badges/:badge_id/award", post(award_badge))
        // Leaderboard endpoints
        .route("/gamification/leaderboard/:institution_id/:limit", get(get_leaderboard))
        // Spin wheel endpoints
        .route("/gamification/:user_id/wheel/spin", post(spin_wheel))
        // Shop endpoints
        .route("/gamification/shop/items", get(get_shop_items))
        .route("/gamification/:user_id/shop/purchase", post(purchase_item))
        .route("/gamification/:user_id/inventory", get(get_inventory))
        // Power-up endpoints
        .route("/gamification/:user_id/powerups/activate", post(activate_power_up))
        // Competition endpoints
        .route("/gamification/competitions/active", get(get_active_competitions))
        .route("/gamification/:user_id/competitions/join", post(join_competition))
        // Quest endpoints
        .route("/gamification/quests/available", get(get_available_quests))
        .route("/gamification/:user_id/quests", get(get_user_quests))
        .route("/gamification/:user_id/quests/accept", post(accept_quest))
        // Achievement endpoints
        .route("/gamification/achievements/available", get(get_available_achievements))
        .route("/gamification/:user_id/achievements", get(get_user_achievements))
}
