// Communication API - Announcements, Messaging, Forums, Notifications
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    db::{self},
    models::communication::*,
    tenant::InstitutionCtx,
};

/// Create announcement
pub async fn create_announcement(
    State(ctx): State<InstitutionCtx>,
    Json(request): Json<CreateAnnouncementRequest>,
) -> Result<Json<AnnouncementResponse>, StatusCode> {
    // In production: get user_id from JWT token
    let created_by = Uuid::nil(); // Placeholder

    let announcement = db::create_announcement(&ctx.db_pool, ctx.id, request, created_by)
        .await
        .map_err(|e| {
            eprintln!("Error creating announcement: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(AnnouncementResponse {
        announcement,
        creator_name: "Current User".to_string(), // Would fetch from DB
        target_count: 0, // Would calculate based on target
    }))
}

/// List announcements
#[derive(Debug, Deserialize)]
pub struct ListAnnouncementsParams {
    course_id: Option<Uuid>,
    page: Option<i64>,
    per_page: Option<i64>,
}

pub async fn list_announcements(
    State(ctx): State<InstitutionCtx>,
    Query(params): Query<ListAnnouncementsParams>,
) -> Result<Json<Vec<AnnouncementResponse>>, StatusCode> {
    let page = params.page.unwrap_or(1);
    let per_page = params.per_page.unwrap_or(20);
    let user_id = Uuid::nil(); // From JWT in production

    let (announcements, _count) = db::list_announcements(
        &ctx.db_pool,
        ctx.id,
        params.course_id,
        user_id,
        page,
        per_page,
    )
    .await
    .map_err(|e| {
        eprintln!("Error listing announcements: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(
        announcements
            .into_iter()
            .map(|a| AnnouncementResponse {
                announcement: a,
                creator_name: "Creator".to_string(),
                target_count: 0,
            })
            .collect(),
    ))
}

/// Get single announcement
pub async fn get_announcement(
    Path(id): Path<Uuid>,
    State(ctx): State<InstitutionCtx>,
) -> Result<Json<AnnouncementResponse>, StatusCode> {
    let announcement = db::get_announcement(&ctx.db_pool, id)
        .await
        .map_err(|e| {
            eprintln!("Error getting announcement: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(AnnouncementResponse {
        announcement,
        creator_name: "Creator".to_string(),
        target_count: 0,
    }))
}

/// Update announcement
pub async fn update_announcement(
    Path(id): Path<Uuid>,
    State(ctx): State<InstitutionCtx>,
    Json(request): Json<UpdateAnnouncementRequest>,
) -> Result<Json<AnnouncementResponse>, StatusCode> {
    let announcement = db::update_announcement(&ctx.db_pool, id, request)
        .await
        .map_err(|e| {
            eprintln!("Error updating announcement: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(AnnouncementResponse {
        announcement,
        creator_name: "Creator".to_string(),
        target_count: 0,
    }))
}

/// Delete announcement
pub async fn delete_announcement(
    Path(id): Path<Uuid>,
    State(_ctx): State<InstitutionCtx>,
) -> Result<StatusCode, StatusCode> {
    db::delete_announcement(&_ctx.db_pool, id)
        .await
        .map_err(|e| {
            eprintln!("Error deleting announcement: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(StatusCode::NO_CONTENT)
}

/// Send message
pub async fn send_message(
    State(ctx): State<InstitutionCtx>,
    Json(request): Json<SendMessageRequest>,
) -> Result<Json<MessageResponse>, StatusCode> {
    let sender_id = Uuid::nil(); // From JWT in production

    let message = db::send_message(&ctx.db_pool, sender_id, request.receiver_id, request)
        .await
        .map_err(|e| {
            eprintln!("Error sending message: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    // Create notification for receiver
    let _ = db::create_notification(
        &ctx.db_pool,
        request.receiver_id,
        NotificationType::Message,
        "New Message",
        "You have received a new message",
        Some("/messages"),
    )
    .await;

    Ok(Json(MessageResponse {
        message,
        sender_name: "Sender".to_string(),
    }))
}

/// Get message threads
#[derive(Debug, Deserialize)]
pub struct ListMessagesParams {
    page: Option<i64>,
    per_page: Option<i64>,
}

pub async fn get_message_threads(
    State(ctx): State<InstitutionCtx>,
    Query(params): Query<ListMessagesParams>,
) -> Result<Json<Vec<MessageThreadResponse>>, StatusCode> {
    let page = params.page.unwrap_or(1);
    let per_page = params.per_page.unwrap_or(20);
    let user_id = Uuid::nil(); // From JWT in production

    let threads = db::get_message_threads(&ctx.db_pool, user_id, page, per_page)
        .await
        .map_err(|e| {
            eprintln!("Error getting message threads: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(
        threads
            .into_iter()
            .map(|t| MessageThreadResponse {
                thread: t,
                other_participant_name: "User".to_string(),
                other_participant_avatar: None,
            })
            .collect(),
    ))
}

/// Get messages between users
pub async fn get_conversation(
    Path(other_user_id): Path<Uuid>,
    State(ctx): State<InstitutionCtx>,
) -> Result<Json<Vec<MessageResponse>>, StatusCode> {
    let user_id = Uuid::nil(); // From JWT in production

    let messages = db::get_messages_between_users(&ctx.db_pool, user_id, other_user_id, 50)
        .await
        .map_err(|e| {
            eprintln!("Error getting conversation: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(
        messages
            .into_iter()
            .map(|m| MessageResponse {
                message: m,
                sender_name: "Sender".to_string(),
            })
            .collect(),
    ))
}

/// Mark message as read
pub async fn mark_message_read(
    Path(message_id): Path<Uuid>,
    State(ctx): State<InstitutionCtx>,
) -> Result<StatusCode, StatusCode> {
    let user_id = Uuid::nil(); // From JWT in production

    db::mark_message_read(&ctx.db_pool, message_id, user_id)
        .await
        .map_err(|e| {
            eprintln!("Error marking message read: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(StatusCode::OK)
}

/// Create forum category (admin only)
pub async fn create_forum_category(
    State(ctx): State<InstitutionCtx>,
    Json(request): Json<CreateForumCategoryRequest>,
) -> Result<Json<ForumCategory>, StatusCode> {
    let category = db::create_forum_category(&ctx.db_pool, ctx.id, request)
        .await
        .map_err(|e| {
            eprintln!("Error creating forum category: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(category))
}

/// List forum categories
pub async fn list_forum_categories(
    State(ctx): State<InstitutionCtx>,
) -> Result<Json<Vec<ForumCategory>>, StatusCode> {
    let categories = db::list_forum_categories(&ctx.db_pool, ctx.id)
        .await
        .map_err(|e| {
            eprintln!("Error listing forum categories: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(categories))
}

/// Create forum
pub async fn create_forum(
    State(ctx): State<InstitutionCtx>,
    Json(request): Json<CreateForumRequest>,
) -> Result<Json<Forum>, StatusCode> {
    let created_by = Uuid::nil(); // From JWT in production

    let forum = db::create_forum(&ctx.db_pool, ctx.id, request, created_by)
        .await
        .map_err(|e| {
            eprintln!("Error creating forum: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(forum))
}

/// List forums
pub async fn list_forums(
    State(ctx): State<InstitutionCtx>,
    Query(course_id): Query<Option<Uuid>>,
) -> Result<Json<Vec<Forum>>, StatusCode> {
    let forums = db::list_forums(&ctx.db_pool, ctx.id, course_id)
        .await
        .map_err(|e| {
            eprintln!("Error listing forums: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(forums))
}

/// Create forum thread
pub async fn create_forum_thread(
    Path(forum_id): Path<Uuid>,
    State(ctx): State<InstitutionCtx>,
    Json(request): Json<CreateForumThreadRequest>,
) -> Result<Json<ForumThread>, StatusCode> {
    let created_by = Uuid::nil(); // From JWT in production

    let thread = db::create_forum_thread(&ctx.db_pool, forum_id, request, created_by)
        .await
        .map_err(|e| {
            eprintln!("Error creating forum thread: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(thread))
}

/// Get forum thread with posts
#[derive(Debug, Deserialize)]
pub struct ListPostsParams {
    page: Option<i64>,
    per_page: Option<i64>,
}

pub async fn get_forum_thread(
    Path(thread_id): Path<Uuid>,
    State(ctx): State<InstitutionCtx>,
    Query(params): Query<ListPostsParams>,
) -> Result<Json<ForumThreadDetailResponse>, StatusCode> {
    let page = params.page.unwrap_or(1);
    let per_page = params.per_page.unwrap_or(20);

    let thread = db::get_forum_thread(&ctx.db_pool, thread_id)
        .await
        .map_err(|e| {
            eprintln!("Error getting forum thread: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .ok_or(StatusCode::NOT_FOUND)?;

    let (posts, _count) = db::list_forum_posts(&ctx.db_pool, thread_id, page, per_page)
        .await
        .map_err(|e| {
            eprintln!("Error listing forum posts: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(ForumThreadDetailResponse {
        thread,
        posts: posts
            .into_iter()
            .map(|p| ForumPostWithAuthor {
                post: p,
                author_name: "Author".to_string(),
                author_role: "User".to_string(),
                reactions: vec![],
                reply_count: 0,
            })
            .collect(),
        author_name: "OP".to_string(),
    }))
}

/// List forum threads
pub async fn list_forum_threads(
    Path(forum_id): Path<Uuid>,
    State(ctx): State<InstitutionCtx>,
    Query(params): Query<ListPostsParams>,
) -> Result<Json<Vec<ForumThread>>, StatusCode> {
    let page = params.page.unwrap_or(1);
    let per_page = params.per_page.unwrap_or(20);

    let (threads, _count) = db::list_forum_threads(&ctx.db_pool, forum_id, page, per_page)
        .await
        .map_err(|e| {
            eprintln!("Error listing forum threads: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(threads))
}

/// Create forum post
pub async fn create_forum_post(
    Path(thread_id): Path<Uuid>,
    State(ctx): State<InstitutionCtx>,
    Json(request): Json<CreateForumPostRequest>,
) -> Result<Json<ForumPost>, StatusCode> {
    let author_id = Uuid::nil(); // From JWT in production

    let post = db::create_forum_post(&ctx.db_pool, thread_id, request, author_id)
        .await
        .map_err(|e| {
            eprintln!("Error creating forum post: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    // Notify thread participants (simplified)
    let _ = db::create_notification(
        &ctx.db_pool,
        author_id,
        NotificationType::ForumReply,
        "New Reply",
        "Someone replied to your thread",
        Some(&format!("/forums/thread/{}", thread_id)),
    )
    .await;

    Ok(Json(post))
}

/// Add reaction to post
pub async fn add_reaction(
    Path((post_id, reaction_type)): Path<(Uuid, String)>,
    State(ctx): State<InstitutionCtx>,
) -> Result<Json<PostReaction>, StatusCode> {
    let user_id = Uuid::nil(); // From JWT in production

    let reaction = db::add_reaction_to_post(&ctx.db_pool, post_id, user_id, reaction_type)
        .await
        .map_err(|e| {
            eprintln!("Error adding reaction: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(reaction))
}

/// Get notifications
#[derive(Debug, Deserialize)]
pub struct ListNotificationsParams {
    unread_only: Option<bool>,
    limit: Option<i64>,
}

pub async fn get_notifications(
    State(ctx): State<InstitutionCtx>,
    Query(params): Query<ListNotificationsParams>,
) -> Result<Json<NotificationListResponse>, StatusCode> {
    let user_id = Uuid::nil(); // From JWT in production
    let unread_only = params.unread_only.unwrap_or(false);
    let limit = params.limit.unwrap_or(50);

    let notifications = db::get_notifications(&ctx.db_pool, user_id, unread_only, limit)
        .await
        .map_err(|e| {
            eprintln!("Error getting notifications: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    let unread_count = notifications.iter().filter(|n| !n.is_read).count() as i32;
    let total_count = notifications.len() as i32;

    Ok(Json(NotificationListResponse {
        notifications: notifications
            .into_iter()
            .map(|n| NotificationResponse { notification: n })
            .collect(),
        unread_count,
        total_count,
    }))
}

/// Mark notification as read
pub async fn mark_notification_read(
    Path(notification_id): Path<Uuid>,
    State(ctx): State<InstitutionCtx>,
) -> Result<StatusCode, StatusCode> {
    let user_id = Uuid::nil(); // From JWT in production

    db::mark_notification_read(&ctx.db_pool, notification_id, user_id)
        .await
        .map_err(|e| {
            eprintln!("Error marking notification read: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(StatusCode::OK)
}

/// Mark all notifications as read
pub async fn mark_all_notifications_read(
    State(ctx): State<InstitutionCtx>,
) -> Result<StatusCode, StatusCode> {
    let user_id = Uuid::nil(); // From JWT in production

    db::mark_all_notifications_read(&ctx.db_pool, user_id)
        .await
        .map_err(|e| {
            eprintln!("Error marking all notifications read: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(StatusCode::OK)
}

/// Get notification preferences
pub async fn get_notification_preferences(
    State(ctx): State<InstitutionCtx>,
) -> Result<Json<NotificationPreferencesResponse>, StatusCode> {
    let user_id = Uuid::nil(); // From JWT in production

    let preferences = db::get_notification_preferences(&ctx.db_pool, user_id)
        .await
        .map_err(|e| {
            eprintln!("Error getting notification preferences: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(NotificationPreferencesResponse { preferences }))
}

/// Update notification preferences
pub async fn update_notification_preferences(
    State(ctx): State<InstitutionCtx>,
    Json(request): Json<UpdateNotificationPreferencesRequest>,
) -> Result<Json<NotificationPreferencesResponse>, StatusCode> {
    let user_id = Uuid::nil(); // From JWT in production

    let preferences = db::update_notification_preferences(&ctx.db_pool, user_id, request)
        .await
        .map_err(|e| {
            eprintln!("Error updating notification preferences: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(NotificationPreferencesResponse { preferences }))
}

/// Create communication router
pub fn communication_router() -> axum::Router {
    axum::Router::new()
        // Announcements
        .route("/", axum::routing::post(create_announcement))
        .route("/", axum::routing::get(list_announcements))
        .route("/:id", axum::routing::get(get_announcement))
        .route("/:id", axum::routing::put(update_announcement))
        .route("/:id", axum::routing::delete(delete_announcement))
        // Messages
        .route("/messages/send", axum::routing::post(send_message))
        .route("/messages/threads", axum::routing::get(get_message_threads))
        .route("/messages/:user_id", axum::routing::get(get_conversation))
        .route("/messages/read/:id", axum::routing::post(mark_message_read))
        // Forums
        .route("/forum/categories", axum::routing::post(create_forum_category))
        .route("/forum/categories", axum::routing::get(list_forum_categories))
        .route("/forum", axum::routing::post(create_forum))
        .route("/forum", axum::routing::get(list_forums))
        .route("/forum/:forum_id/threads", axum::routing::post(create_forum_thread))
        .route("/forum/:forum_id/threads", axum::routing::get(list_forum_threads))
        .route(
            "/forum/thread/:thread_id",
            axum::routing::get(get_forum_thread),
        )
        .route("/forum/post/:thread_id", axum::routing::post(create_forum_post))
        .route(
            "/forum/reaction/:post_id/:reaction_type",
            axum::routing::post(add_reaction),
        )
        // Notifications
        .route("/notifications", axum::routing::get(get_notifications))
        .route(
            "/notifications/read-all",
            axum::routing::post(mark_all_notifications_read),
        )
        .route(
            "/notifications/:id/read",
            axum::routing::post(mark_notification_read),
        )
        .route(
            "/notifications/preferences",
            axum::routing::get(get_notification_preferences),
        )
        .route(
            "/notifications/preferences",
            axum::routing::put(update_notification_preferences),
        )
}
