// Communication Database Layer - Announcements, Messaging, Forums, Notifications
use chrono::{DateTime, Utc};
use sqlx::{PgPool, Postgres, Row, Transaction};
use uuid::Uuid;

use crate::models::communication::*;

/// ==================== ANNOUNCEMENTS ====================

pub async fn create_announcement(
    pool: &PgPool,
    institution_id: Uuid,
    request: CreateAnnouncementRequest,
    created_by: Uuid,
) -> Result<Announcement, sqlx::Error> {
    let id = Uuid::new_v4();
    let now = Utc::now();
    let published_at = if request.publish_now {
        Some(now)
    } else {
        request.scheduled_at
    };

    let row = sqlx::query!(
        r#"INSERT INTO announcements 
           (id, institution_id, course_id, title, content, target_type, target_ids, priority,
            is_published, published_at, expires_at, created_by, created_at, updated_at)
           VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)
           RETURNING *"#,
        id,
        institution_id,
        request.course_id,
        request.title,
        request.content,
        format!("{:?}", request.target_type),
        &request.target_ids,
        format!("{:?}", request.priority),
        request.publish_now || request.scheduled_at.is_some(),
        published_at,
        request.expires_at,
        created_by,
        now,
        now
    )
    .fetch_one(pool)
    .await?;

    Ok(announcement_from_row(row))
}

pub async fn get_announcement(
    pool: &PgPool,
    announcement_id: Uuid,
) -> Result<Option<Announcement>, sqlx::Error> {
    let row = sqlx::query!(
        r#"SELECT * FROM announcements WHERE id = $1"#,
        announcement_id
    )
    .fetch_optional(pool)
    .await?;

    Ok(row.map(announcement_from_row))
}

pub async fn list_announcements(
    pool: &PgPool,
    institution_id: Uuid,
    course_id: Option<Uuid>,
    user_id: Uuid,
    page: i64,
    per_page: i64,
) -> Result<(Vec<Announcement>, i64), sqlx::Error> {
    let offset = (page - 1) * per_page;

    // Get announcements visible to user
    let rows = sqlx::query!(
        r#"SELECT a.* FROM announcements a
           WHERE a.institution_id = $1
             AND a.is_published = true
             AND (a.expires_at IS NULL OR a.expires_at > NOW())
             AND (a.published_at IS NULL OR a.published_at <= NOW())
             AND ($2::uuid IS NULL OR a.course_id = $2 OR a.target_type = 'All')
           ORDER BY a.priority DESC, a.published_at DESC
           LIMIT $3 OFFSET $4"#,
        institution_id,
        course_id,
        per_page,
        offset
    )
    .fetch_all(pool)
    .await?;

    let count = sqlx::query_scalar!(
        r#"SELECT COUNT(*) FROM announcements a
           WHERE a.institution_id = $1
             AND a.is_published = true
             AND (a.expires_at IS NULL OR a.expires_at > NOW())
             AND (a.published_at IS NULL OR a.published_at <= NOW())
             AND ($2::uuid IS NULL OR a.course_id = $2 OR a.target_type = 'All')"#,
        institution_id,
        course_id
    )
    .fetch_one(pool)
    .await?;

    Ok((rows.into_iter().map(announcement_from_row).collect(), count))
}

pub async fn update_announcement(
    pool: &PgPool,
    announcement_id: Uuid,
    request: UpdateAnnouncementRequest,
) -> Result<Announcement, sqlx::Error> {
    let now = Utc::now();

    let row = sqlx::query!(
        r#"UPDATE announcements 
           SET title = COALESCE($1, title),
               content = COALESCE($2, content),
               priority = COALESCE($3, priority),
               expires_at = COALESCE($4, expires_at),
               is_published = COALESCE($5, is_published),
               updated_at = $6
           WHERE id = $7
           RETURNING *"#,
        request.title,
        request.content,
        request.priority.map(|p| format!("{:?}", p)),
        request.expires_at,
        request.is_published,
        now,
        announcement_id
    )
    .fetch_one(pool)
    .await?;

    Ok(announcement_from_row(row))
}

pub async fn delete_announcement(
    pool: &PgPool,
    announcement_id: Uuid,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"DELETE FROM announcements WHERE id = $1"#,
        announcement_id
    )
    .execute(pool)
    .await?;

    Ok(())
}

/// ==================== MESSAGES ====================

pub async fn send_message(
    pool: &PgPool,
    sender_id: Uuid,
    receiver_id: Uuid,
    request: SendMessageRequest,
) -> Result<Message, sqlx::Error> {
    let id = Uuid::new_v4();
    let now = Utc::now();

    let row = sqlx::query!(
        r#"INSERT INTO messages (id, sender_id, receiver_id, subject, body, is_read, created_at)
           VALUES ($1, $2, $3, $4, $5, false, $6)
           RETURNING *"#,
        id,
        sender_id,
        receiver_id,
        request.subject,
        request.body,
        now
    )
    .fetch_one(pool)
    .await?;

    Ok(message_from_row(row))
}

pub async fn get_message_threads(
    pool: &PgPool,
    user_id: Uuid,
    page: i64,
    per_page: i64,
) -> Result<Vec<MessageThread>, sqlx::Error> {
    let offset = (page - 1) * per_page;

    let rows = sqlx::query!(
        r#"SELECT 
               CASE WHEN sender_id = $1 THEN receiver_id ELSE sender_id END as other_user_id,
               MAX(created_at) as last_message_at,
               MAX(body) as last_message_preview,
               COUNT(CASE WHEN is_read = false AND receiver_id = $1 THEN 1 END)::int as unread_count
           FROM messages
           WHERE sender_id = $1 OR receiver_id = $1
           GROUP BY other_user_id
           ORDER BY last_message_at DESC
           LIMIT $2 OFFSET $3"#,
        user_id,
        per_page,
        offset
    )
    .fetch_all(pool)
    .await?;

    // This is simplified - in production you'd build full MessageThread structs
    Ok(rows
        .into_iter()
        .map(|r| MessageThread {
            id: Uuid::new_v4(), // Would need proper thread ID
            participant_1: user_id,
            participant_2: r.other_user_id,
            last_message_at: r.last_message_at,
            last_message_preview: r.last_message_preview.unwrap_or_default(),
            unread_count: r.unread_count,
        })
        .collect())
}

pub async fn get_messages_between_users(
    pool: &PgPool,
    user1_id: Uuid,
    user2_id: Uuid,
    limit: i64,
) -> Result<Vec<Message>, sqlx::Error> {
    let rows = sqlx::query!(
        r#"SELECT * FROM messages
           WHERE (sender_id = $1 AND receiver_id = $2)
              OR (sender_id = $2 AND receiver_id = $1)
           ORDER BY created_at DESC
           LIMIT $3"#,
        user1_id,
        user2_id,
        limit
    )
    .fetch_all(pool)
    .await?;

    Ok(rows.into_iter().map(message_from_row).collect())
}

pub async fn mark_message_read(
    pool: &PgPool,
    message_id: Uuid,
    user_id: Uuid,
) -> Result<(), sqlx::Error> {
    let now = Utc::now();

    sqlx::query!(
        r#"UPDATE messages SET is_read = true, read_at = $1
           WHERE id = $2 AND receiver_id = $3"#,
        now,
        message_id,
        user_id
    )
    .execute(pool)
    .await?;

    Ok(())
}

/// ==================== FORUMS ====================

pub async fn create_forum_category(
    pool: &PgPool,
    institution_id: Uuid,
    request: CreateForumCategoryRequest,
) -> Result<ForumCategory, sqlx::Error> {
    let id = Uuid::new_v4();
    let now = Utc::now();

    let row = sqlx::query!(
        r#"INSERT INTO forum_categories 
           (id, institution_id, name, description, position, is_locked, created_at)
           VALUES ($1, $2, $3, $4, $5, false, $6)
           RETURNING *"#,
        id,
        institution_id,
        request.name,
        request.description,
        request.position,
        now
    )
    .fetch_one(pool)
    .await?;

    Ok(forum_category_from_row(row))
}

pub async fn list_forum_categories(
    pool: &PgPool,
    institution_id: Uuid,
) -> Result<Vec<ForumCategory>, sqlx::Error> {
    let rows = sqlx::query!(
        r#"SELECT * FROM forum_categories
           WHERE institution_id = $1
           ORDER BY position ASC"#,
        institution_id
    )
    .fetch_all(pool)
    .await?;

    Ok(rows.into_iter().map(forum_category_from_row).collect())
}

pub async fn create_forum(
    pool: &PgPool,
    institution_id: Uuid,
    request: CreateForumRequest,
    created_by: Uuid,
) -> Result<Forum, sqlx::Error> {
    let id = Uuid::new_v4();
    let now = Utc::now();

    let row = sqlx::query!(
        r#"INSERT INTO forums 
           (id, institution_id, course_id, category_id, name, description, is_locked, created_by, created_at)
           VALUES ($1, $2, $3, $4, $5, $6, false, $7, $8)
           RETURNING *"#,
        id,
        institution_id,
        request.course_id,
        request.category_id,
        request.name,
        request.description,
        created_by,
        now
    )
    .fetch_one(pool)
    .await?;

    Ok(forum_from_row(row))
}

pub async fn list_forums(
    pool: &PgPool,
    institution_id: Uuid,
    course_id: Option<Uuid>,
) -> Result<Vec<Forum>, sqlx::Error> {
    let rows = sqlx::query!(
        r#"SELECT * FROM forums
           WHERE institution_id = $1 AND ($2::uuid IS NULL OR course_id = $2)
           ORDER BY created_at DESC"#,
        institution_id,
        course_id
    )
    .fetch_all(pool)
    .await?;

    Ok(rows.into_iter().map(forum_from_row).collect())
}

pub async fn create_forum_thread(
    pool: &PgPool,
    forum_id: Uuid,
    request: CreateForumThreadRequest,
    created_by: Uuid,
) -> Result<ForumThread, sqlx::Error> {
    let id = Uuid::new_v4();
    let now = Utc::now();

    let mut tx = pool.begin().await?;

    // Create thread
    let thread_row = sqlx::query!(
        r#"INSERT INTO forum_threads 
           (id, forum_id, title, is_pinned, is_locked, is_announcement, view_count, post_count, created_by, created_at)
           VALUES ($1, $2, $3, false, false, $4, 0, 0, $5, $6)
           RETURNING *"#,
        id,
        forum_id,
        request.title,
        request.is_announcement.unwrap_or(false),
        created_by,
        now
    )
    .fetch_one(&mut *tx)
    .await?;

    // Create first post (OP)
    let _post_row = sqlx::query!(
        r#"INSERT INTO forum_posts 
           (id, thread_id, parent_post_id, author_id, body, is_edited, flag_count, is_hidden, created_at, updated_at)
           VALUES ($1, $2, NULL, $3, $4, false, 0, false, $5, $5)
           RETURNING *"#,
        Uuid::new_v4(),
        id,
        created_by,
        request.body,
        now
    )
    .fetch_one(&mut *tx)
    .await?;

    tx.commit().await?;

    Ok(forum_thread_from_row(thread_row))
}

pub async fn get_forum_thread(
    pool: &PgPool,
    thread_id: Uuid,
) -> Result<Option<ForumThread>, sqlx::Error> {
    // Increment view count
    sqlx::query!(
        r#"UPDATE forum_threads SET view_count = view_count + 1 WHERE id = $1"#,
        thread_id
    )
    .execute(pool)
    .await?;

    let row = sqlx::query!(
        r#"SELECT * FROM forum_threads WHERE id = $1"#,
        thread_id
    )
    .fetch_optional(pool)
    .await?;

    Ok(row.map(forum_thread_from_row))
}

pub async fn list_forum_threads(
    pool: &PgPool,
    forum_id: Uuid,
    page: i64,
    per_page: i64,
) -> Result<(Vec<ForumThread>, i64), sqlx::Error> {
    let offset = (page - 1) * per_page;

    let rows = sqlx::query!(
        r#"SELECT * FROM forum_threads
           WHERE forum_id = $1
           ORDER BY is_pinned DESC, last_post_at DESC NULLS LAST, created_at DESC
           LIMIT $2 OFFSET $3"#,
        forum_id,
        per_page,
        offset
    )
    .fetch_all(pool)
    .await?;

    let count = sqlx::query_scalar!(
        r#"SELECT COUNT(*) FROM forum_threads WHERE forum_id = $1"#,
        forum_id
    )
    .fetch_one(pool)
    .await?;

    Ok((rows.into_iter().map(forum_thread_from_row).collect(), count))
}

pub async fn create_forum_post(
    pool: &PgPool,
    thread_id: Uuid,
    request: CreateForumPostRequest,
    author_id: Uuid,
) -> Result<ForumPost, sqlx::Error> {
    let id = Uuid::new_v4();
    let now = Utc::now();

    let mut tx = pool.begin().await?;

    // Create post
    let post_row = sqlx::query!(
        r#"INSERT INTO forum_posts 
           (id, thread_id, parent_post_id, author_id, body, is_edited, flag_count, is_hidden, created_at, updated_at)
           VALUES ($1, $2, $3, $4, $5, false, 0, false, $6, $6)
           RETURNING *"#,
        id,
        thread_id,
        request.parent_post_id,
        author_id,
        request.body,
        now
    )
    .fetch_one(&mut *tx)
    .await?;

    // Update thread post count and last post info
    sqlx::query!(
        r#"UPDATE forum_threads 
           SET post_count = post_count + 1,
               last_post_at = $1,
               last_post_by = $2
           WHERE id = $3"#,
        now,
        author_id,
        thread_id
    )
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;

    Ok(forum_post_from_row(post_row))
}

pub async fn list_forum_posts(
    pool: &PgPool,
    thread_id: Uuid,
    page: i64,
    per_page: i64,
) -> Result<(Vec<ForumPost>, i64), sqlx::Error> {
    let offset = (page - 1) * per_page;

    let rows = sqlx::query!(
        r#"SELECT * FROM forum_posts
           WHERE thread_id = $1 AND parent_post_id IS NULL
           ORDER BY created_at ASC
           LIMIT $2 OFFSET $3"#,
        thread_id,
        per_page,
        offset
    )
    .fetch_all(pool)
    .await?;

    let count = sqlx::query_scalar!(
        r#"SELECT COUNT(*) FROM forum_posts WHERE thread_id = $1 AND parent_post_id IS NULL"#,
        thread_id
    )
    .fetch_one(pool)
    .await?;

    Ok((rows.into_iter().map(forum_post_from_row).collect(), count))
}

pub async fn add_reaction_to_post(
    pool: &PgPool,
    post_id: Uuid,
    user_id: Uuid,
    reaction_type: String,
) -> Result<PostReaction, sqlx::Error> {
    let id = Uuid::new_v4();
    let now = Utc::now();

    // Remove existing reaction if any
    sqlx::query!(
        r#"DELETE FROM post_reactions WHERE post_id = $1 AND user_id = $2"#,
        post_id,
        user_id
    )
    .execute(pool)
    .await?;

    let row = sqlx::query!(
        r#"INSERT INTO post_reactions (id, post_id, user_id, reaction_type, created_at)
           VALUES ($1, $2, $3, $4, $5)
           RETURNING *"#,
        id,
        post_id,
        user_id,
        reaction_type,
        now
    )
    .fetch_one(pool)
    .await?;

    Ok(post_reaction_from_row(row))
}

/// ==================== NOTIFICATIONS ====================

pub async fn create_notification(
    pool: &PgPool,
    user_id: Uuid,
    notification_type: NotificationType,
    title: &str,
    message: &str,
    action_url: Option<&str>,
) -> Result<Notification, sqlx::Error> {
    let id = Uuid::new_v4();
    let now = Utc::now();

    let row = sqlx::query!(
        r#"INSERT INTO notifications 
           (id, user_id, notification_type, title, message, action_url, is_read, created_at)
           VALUES ($1, $2, $3, $4, $5, $6, false, $7)
           RETURNING *"#,
        id,
        user_id,
        format!("{:?}", notification_type).to_lowercase(),
        title,
        message,
        action_url,
        now
    )
    .fetch_one(pool)
    .await?;

    Ok(notification_from_row(row))
}

pub async fn get_notifications(
    pool: &PgPool,
    user_id: Uuid,
    unread_only: bool,
    limit: i64,
) -> Result<Vec<Notification>, sqlx::Error> {
    let query = if unread_only {
        sqlx::query!(
            r#"SELECT * FROM notifications 
               WHERE user_id = $1 AND is_read = false 
               ORDER BY created_at DESC 
               LIMIT $2"#,
            user_id,
            limit
        )
    } else {
        sqlx::query!(
            r#"SELECT * FROM notifications 
               WHERE user_id = $1 
               ORDER BY created_at DESC 
               LIMIT $2"#,
            user_id,
            limit
        )
    };

    let rows = query.fetch_all(pool).await?;

    Ok(rows.into_iter().map(notification_from_row).collect())
}

pub async fn mark_notification_read(
    pool: &PgPool,
    notification_id: Uuid,
    user_id: Uuid,
) -> Result<(), sqlx::Error> {
    let now = Utc::now();

    sqlx::query!(
        r#"UPDATE notifications SET is_read = true, read_at = $1 
           WHERE id = $2 AND user_id = $3"#,
        now,
        notification_id,
        user_id
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn mark_all_notifications_read(
    pool: &PgPool,
    user_id: Uuid,
) -> Result<(), sqlx::Error> {
    let now = Utc::now();

    sqlx::query!(
        r#"UPDATE notifications SET is_read = true, read_at = $1 
           WHERE user_id = $2 AND is_read = false"#,
        now,
        user_id
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn get_notification_preferences(
    pool: &PgPool,
    user_id: Uuid,
) -> Result<NotificationPreferences, sqlx::Error> {
    let row = sqlx::query!(
        r#"SELECT * FROM notification_preferences WHERE user_id = $1"#,
        user_id
    )
    .fetch_optional(pool)
    .await?;

    match row {
        Some(r) => Ok(notification_preferences_from_row(r)),
        None => {
            // Create default preferences
            create_default_notification_preferences(pool, user_id).await
        }
    }
}

pub async fn create_default_notification_preferences(
    pool: &PgPool,
    user_id: Uuid,
) -> Result<NotificationPreferences, sqlx::Error> {
    let row = sqlx::query!(
        r#"INSERT INTO notification_preferences 
           (user_id, email_enabled, push_enabled, sms_enabled,
            announcement_email, announcement_push, message_email, message_push,
            grade_email, grade_push, assignment_email, assignment_push,
            forum_email, forum_push)
           VALUES ($1, true, true, false, true, true, true, true, true, true, true, true, true, true)
           ON CONFLICT (user_id) DO UPDATE SET
            email_enabled = EXCLUDED.email_enabled,
            push_enabled = EXCLUDED.push_enabled
           RETURNING *"#,
        user_id
    )
    .fetch_one(pool)
    .await?;

    Ok(notification_preferences_from_row(row))
}

pub async fn update_notification_preferences(
    pool: &PgPool,
    user_id: Uuid,
    request: UpdateNotificationPreferencesRequest,
) -> Result<NotificationPreferences, sqlx::Error> {
    let row = sqlx::query!(
        r#"UPDATE notification_preferences 
           SET email_enabled = COALESCE($1, email_enabled),
               push_enabled = COALESCE($2, push_enabled),
               announcement_email = COALESCE($3, announcement_email),
               announcement_push = COALESCE($4, announcement_push),
               message_email = COALESCE($5, message_email),
               message_push = COALESCE($6, message_push),
               grade_email = COALESCE($7, grade_email),
               grade_push = COALESCE($8, grade_push),
               assignment_email = COALESCE($9, assignment_email),
               assignment_push = COALESCE($10, assignment_push),
               forum_email = COALESCE($11, forum_email),
               forum_push = COALESCE($12, forum_push)
           WHERE user_id = $13
           RETURNING *"#,
        request.email_enabled,
        request.push_enabled,
        request.announcement_email,
        request.announcement_push,
        request.message_email,
        request.message_push,
        request.grade_email,
        request.grade_push,
        request.assignment_email,
        request.assignment_push,
        request.forum_email,
        request.forum_push,
        user_id
    )
    .fetch_one(pool)
    .await?;

    Ok(notification_preferences_from_row(row))
}

/// ==================== HELPER FUNCTIONS ====================

fn announcement_from_row(row: sqlx::postgres::PgRow) -> Announcement {
    Announcement {
        id: row.get("id"),
        institution_id: row.get("institution_id"),
        course_id: row.get("course_id"),
        title: row.get("title"),
        content: row.get("content"),
        target_type: serde_json::from_str(row.get::<&str, _>("target_type")).unwrap_or(AnnouncementTarget::All),
        target_ids: row.get("target_ids"),
        priority: serde_json::from_str(row.get::<&str, _>("priority")).unwrap_or(AnnouncementPriority::Normal),
        is_published: row.get("is_published"),
        published_at: row.get("published_at"),
        expires_at: row.get("expires_at"),
        created_by: row.get("created_by"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    }
}

fn message_from_row(row: sqlx::postgres::PgRow) -> Message {
    Message {
        id: row.get("id"),
        sender_id: row.get("sender_id"),
        receiver_id: row.get("receiver_id"),
        subject: row.get("subject"),
        body: row.get("body"),
        is_read: row.get("is_read"),
        read_at: row.get("read_at"),
        created_at: row.get("created_at"),
    }
}

fn forum_category_from_row(row: sqlx::postgres::PgRow) -> ForumCategory {
    ForumCategory {
        id: row.get("id"),
        institution_id: row.get("institution_id"),
        name: row.get("name"),
        description: row.get("description"),
        position: row.get("position"),
        is_locked: row.get("is_locked"),
        created_at: row.get("created_at"),
    }
}

fn forum_from_row(row: sqlx::postgres::PgRow) -> Forum {
    Forum {
        id: row.get("id"),
        institution_id: row.get("institution_id"),
        course_id: row.get("course_id"),
        category_id: row.get("category_id"),
        name: row.get("name"),
        description: row.get("description"),
        is_locked: row.get("is_locked"),
        created_by: row.get("created_by"),
        created_at: row.get("created_at"),
    }
}

fn forum_thread_from_row(row: sqlx::postgres::PgRow) -> ForumThread {
    ForumThread {
        id: row.get("id"),
        forum_id: row.get("forum_id"),
        title: row.get("title"),
        is_pinned: row.get("is_pinned"),
        is_locked: row.get("is_locked"),
        is_announcement: row.get("is_announcement"),
        view_count: row.get("view_count"),
        post_count: row.get("post_count"),
        last_post_at: row.get("last_post_at"),
        last_post_by: row.get("last_post_by"),
        created_by: row.get("created_by"),
        created_at: row.get("created_at"),
    }
}

fn forum_post_from_row(row: sqlx::postgres::PgRow) -> ForumPost {
    ForumPost {
        id: row.get("id"),
        thread_id: row.get("thread_id"),
        parent_post_id: row.get("parent_post_id"),
        author_id: row.get("author_id"),
        body: row.get("body"),
        is_edited: row.get("is_edited"),
        edited_at: row.get("edited_at"),
        flag_count: row.get("flag_count"),
        is_hidden: row.get("is_hidden"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    }
}

fn post_reaction_from_row(row: sqlx::postgres::PgRow) -> PostReaction {
    PostReaction {
        id: row.get("id"),
        post_id: row.get("post_id"),
        user_id: row.get("user_id"),
        reaction_type: row.get("reaction_type"),
        created_at: row.get("created_at"),
    }
}

fn notification_from_row(row: sqlx::postgres::PgRow) -> Notification {
    Notification {
        id: row.get("id"),
        user_id: row.get("user_id"),
        notification_type: NotificationType::System, // Parse from string in production
        title: row.get("title"),
        message: row.get("message"),
        action_url: row.get("action_url"),
        is_read: row.get("is_read"),
        read_at: row.get("read_at"),
        created_at: row.get("created_at"),
    }
}

fn notification_preferences_from_row(row: sqlx::postgres::PgRow) -> NotificationPreferences {
    NotificationPreferences {
        user_id: row.get("user_id"),
        email_enabled: row.get("email_enabled"),
        push_enabled: row.get("push_enabled"),
        sms_enabled: row.get("sms_enabled"),
        announcement_email: row.get("announcement_email"),
        announcement_push: row.get("announcement_push"),
        message_email: row.get("message_email"),
        message_push: row.get("message_push"),
        grade_email: row.get("grade_email"),
        grade_push: row.get("grade_push"),
        assignment_email: row.get("assignment_email"),
        assignment_push: row.get("assignment_push"),
        forum_email: row.get("forum_email"),
        forum_push: row.get("forum_push"),
    }
}
