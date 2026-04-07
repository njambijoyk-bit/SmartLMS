// Communication Service - Announcements, messaging, notifications
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// Announcement entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Announcement {
    pub id: uuid::Uuid,
    pub institution_id: uuid::Uuid,
    pub title: String,
    pub content: String,
    pub target_type: AnnouncementTarget,
    pub target_ids: Vec<uuid::Uuid>,  // course_ids, role names, or user_ids
    pub priority: AnnouncementPriority,
    pub published_at: Option<DateTime<Utc>>,
    pub expires_at: Option<DateTime<Utc>>,
    pub created_by: uuid::Uuid,
    pub created_at: DateTime<Utc>,
}

/// Announcement targeting
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AnnouncementTarget {
    All,           // All users
    Role,          // Specific roles
    Course,        // Users enrolled in specific courses
    User,          // Specific users
}

/// Announcement priority
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AnnouncementPriority {
    Low,
    Normal,
    High,
    Urgent,
}

/// Direct message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub id: uuid::Uuid,
    pub sender_id: uuid::Uuid,
    pub receiver_id: uuid::Uuid,
    pub subject: Option<String>,
    pub body: String,
    pub is_read: bool,
    pub read_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

/// Notification entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Notification {
    pub id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub notification_type: NotificationType,
    pub title: String,
    pub message: String,
    pub action_url: Option<String>,
    pub is_read: bool,
    pub created_at: DateTime<Utc>,
}

/// Notification types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NotificationType {
    Announcement,
    Message,
    Assignment,
    Grade,
    CourseUpdate,
    LiveSession,
    System,
}

/// Notification preferences
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationPreferences {
    pub user_id: uuid::Uuid,
    pub email_enabled: bool,
    pub push_enabled: bool,
    pub announcement_email: bool,
    pub message_email: bool,
    pub grade_email: bool,
    pub assignment_email: bool,
}

// Request types
#[derive(Debug, Deserialize)]
pub struct CreateAnnouncementRequest {
    pub title: String,
    pub content: String,
    pub target_type: AnnouncementTarget,
    pub target_ids: Vec<uuid::Uuid>,
    pub priority: AnnouncementPriority,
    pub publish_now: bool,
    pub scheduled_at: Option<DateTime<Utc>>,
    pub expires_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
pub struct SendMessageRequest {
    pub receiver_id: uuid::Uuid,
    pub subject: Option<String>,
    pub body: String,
}

#[derive(Debug, Deserialize)]
pub struct NotificationPreferencesRequest {
    pub email_enabled: bool,
    pub push_enabled: bool,
    pub announcement_email: bool,
    pub message_email: bool,
    pub grade_email: bool,
    pub assignment_email: bool,
}

// Service functions
pub mod service {
    use super::*;
    
    /// Create an announcement
    pub async fn create_announcement(
        pool: &PgPool,
        institution_id: uuid::Uuid,
        creator_id: uuid::Uuid,
        req: &CreateAnnouncementRequest,
    ) -> Result<Announcement, String> {
        if req.title.is_empty() {
            return Err("Title is required".to_string());
        }
        
        let id = Uuid::new_v4();
        let now = Utc::now();
        
        let published_at = if req.publish_now {
            Some(now)
        } else {
            req.scheduled_at
        };
        
        sqlx::query!(
            "INSERT INTO announcements (id, institution_id, title, content, target_type, 
             target_ids, priority, published_at, expires_at, created_by, created_at)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)",
            id, institution_id, req.title, req.content, format!("{:?}", req.target_type).to_lowercase(),
            serde_json::to_string(&req.target_ids).unwrap_or_default(),
            format!("{:?}", req.priority).to_lowercase(), published_at, req.expires_at, creator_id, now
        )
        .execute(pool)
        .await
        .map_err(|e| e.to_string())?;
        
        // Create notifications for target users
        let target_user_ids = get_target_user_ids(pool, institution_id, &req.target_type, &req.target_ids).await?;
        
        for user_id in target_user_ids {
            create_notification(
                pool,
                user_id,
                NotificationType::Announcement,
                &req.title,
                &req.content,
                Some(&format!("/announcements/{}", id)),
            ).await?;
        }
        
        Ok(Announcement {
            id,
            institution_id,
            title: req.title.clone(),
            content: req.content.clone(),
            target_type: req.target_type,
            target_ids: req.target_ids.clone(),
            priority: req.priority,
            published_at,
            expires_at: req.expires_at,
            created_by: creator_id,
            created_at: now,
        })
    }
    
    /// Send a direct message
    pub async fn send_message(
        pool: &PgPool,
        sender_id: uuid::Uuid,
        req: &SendMessageRequest,
    ) -> Result<Message, String> {
        let id = Uuid::new_v4();
        let now = Utc::now();
        
        sqlx::query!(
            "INSERT INTO messages (id, sender_id, receiver_id, subject, body, is_read, created_at)
             VALUES ($1, $2, $3, $4, $5, false, $6)",
            id, sender_id, req.receiver_id, req.subject, req.body, now
        )
        .execute(pool)
        .await
        .map_err(|e| e.to_string())?;
        
        // Create notification for receiver
        create_notification(
            pool,
            req.receiver_id,
            NotificationType::Message,
            "New Message",
            req.subject.as_deref().unwrap_or("You have a new message"),
            Some("/messages"),
        ).await?;
        
        Ok(Message {
            id,
            sender_id,
            receiver_id: req.receiver_id,
            subject: req.subject.clone(),
            body: req.body.clone(),
            is_read: false,
            read_at: None,
            created_at: now,
        })
    }
    
    /// Get user notifications
    pub async fn get_notifications(
        pool: &PgPool,
        user_id: uuid::Uuid,
        unread_only: bool,
        limit: i64,
    ) -> Result<Vec<Notification>, String> {
        let query = if unread_only {
            sqlx::query!(
                "SELECT id, user_id, notification_type, title, message, action_url, is_read, created_at
                 FROM notifications WHERE user_id = $1 AND is_read = false ORDER BY created_at DESC LIMIT $2",
                user_id, limit
            )
        } else {
            sqlx::query!(
                "SELECT id, user_id, notification_type, title, message, action_url, is_read, created_at
                 FROM notifications WHERE user_id = $1 ORDER BY created_at DESC LIMIT $2",
                user_id, limit
            )
        };
        
        let rows = query.fetch_all(pool).await.map_err(|e| e.to_string())?;
        
        Ok(rows.into_iter().map(|r| Notification {
            id: r.id,
            user_id: r.user_id,
            notification_type: NotificationType::System,  // Parse from string
            title: r.title,
            message: r.message,
            action_url: r.action_url,
            is_read: r.is_read,
            created_at: r.created_at,
        }).collect())
    }
    
    /// Mark notification as read
    pub async fn mark_notification_read(
        pool: &PgPool,
        notification_id: uuid::Uuid,
        user_id: uuid::Uuid,
    ) -> Result<(), String> {
        let now = Utc::now();
        
        sqlx::query!(
            "UPDATE notifications SET is_read = true, read_at = $1 WHERE id = $2 AND user_id = $3",
            now, notification_id, user_id
        )
        .execute(pool)
        .await
        .map_err(|e| e.to_string())?;
        
        Ok(())
    }
    
    async fn create_notification(
        pool: &PgPool,
        user_id: uuid::Uuid,
        notification_type: NotificationType,
        title: &str,
        message: &str,
        action_url: Option<&str>,
    ) -> Result<Notification, String> {
        let id = Uuid::new_v4();
        let now = Utc::now();
        
        sqlx::query!(
            "INSERT INTO notifications (id, user_id, notification_type, title, message, action_url, is_read, created_at)
             VALUES ($1, $2, $3, $4, $5, $6, false, $7)",
            id, user_id, format!("{:?}", notification_type).to_lowercase(), title, message, action_url, now
        )
        .execute(pool)
        .await
        .map_err(|e| e.to_string())?;
        
        Ok(Notification {
            id,
            user_id,
            notification_type,
            title: title.to_string(),
            message: message.to_string(),
            action_url: action_url.map(String::from),
            is_read: false,
            created_at: now,
        })
    }
    
    async fn get_target_user_ids(
        pool: &PgPool,
        institution_id: uuid::Uuid,
        target_type: &AnnouncementTarget,
        target_ids: &[uuid::Uuid],
    ) -> Result<Vec<uuid::Uuid>, String> {
        // Simplified - return empty for now
        // In production: query based on target type
        Ok(vec![])
    }
}