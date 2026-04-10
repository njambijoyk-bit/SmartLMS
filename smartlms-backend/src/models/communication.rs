// Communication Models - Announcements, Messaging, Forums, Notifications
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Announcement entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Announcement {
    pub id: Uuid,
    pub institution_id: Uuid,
    pub course_id: Option<Uuid>, // None = institution-wide
    pub title: String,
    pub content: String,
    pub target_type: AnnouncementTarget,
    pub target_ids: Vec<Uuid>, // course_ids, role names, or user_ids
    pub priority: AnnouncementPriority,
    pub is_published: bool,
    pub published_at: Option<DateTime<Utc>>,
    pub expires_at: Option<DateTime<Utc>>,
    pub created_by: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Announcement targeting
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AnnouncementTarget {
    All,      // All users in institution/course
    Role,     // Specific roles (instructor, student, etc.)
    Course,   // Users enrolled in specific courses
    User,     // Specific users
    Group,    // Specific groups/cohorts
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
    pub id: Uuid,
    pub sender_id: Uuid,
    pub receiver_id: Uuid,
    pub subject: Option<String>,
    pub body: String,
    pub is_read: bool,
    pub read_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

/// Message thread (conversation between two users)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageThread {
    pub id: Uuid,
    pub participant_1: Uuid,
    pub participant_2: Uuid,
    pub last_message_at: DateTime<Utc>,
    pub last_message_preview: String,
    pub unread_count: i32,
}

/// Forum category
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForumCategory {
    pub id: Uuid,
    pub institution_id: Uuid,
    pub name: String,
    pub description: String,
    pub position: i32,
    pub is_locked: bool,
    pub created_at: DateTime<Utc>,
}

/// Forum (within a course or institution-wide)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Forum {
    pub id: Uuid,
    pub institution_id: Uuid,
    pub course_id: Option<Uuid>,
    pub category_id: Uuid,
    pub name: String,
    pub description: String,
    pub is_locked: bool,
    pub created_by: Uuid,
    pub created_at: DateTime<Utc>,
}

/// Forum thread
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForumThread {
    pub id: Uuid,
    pub forum_id: Uuid,
    pub title: String,
    pub is_pinned: bool,
    pub is_locked: bool,
    pub is_announcement: bool, // Instructor-only post
    pub view_count: i32,
    pub post_count: i32,
    pub last_post_at: Option<DateTime<Utc>>,
    pub last_post_by: Option<Uuid>,
    pub created_by: Uuid,
    pub created_at: DateTime<Utc>,
}

/// Forum post
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForumPost {
    pub id: Uuid,
    pub thread_id: Uuid,
    pub parent_post_id: Option<Uuid>, // For nested replies (max 2 levels)
    pub author_id: Uuid,
    pub body: String,
    pub is_edited: bool,
    pub edited_at: Option<DateTime<Utc>>,
    pub flag_count: i32,
    pub is_hidden: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Post reaction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostReaction {
    pub id: Uuid,
    pub post_id: Uuid,
    pub user_id: Uuid,
    pub reaction_type: String, // "like", "helpful", "disagree", etc.
    pub created_at: DateTime<Utc>,
}

/// Notification entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Notification {
    pub id: Uuid,
    pub user_id: Uuid,
    pub notification_type: NotificationType,
    pub title: String,
    pub message: String,
    pub action_url: Option<String>,
    pub is_read: bool,
    pub read_at: Option<DateTime<Utc>>,
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
    ForumReply,
    ForumMention,
    System,
}

/// Notification preferences
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationPreferences {
    pub user_id: Uuid,
    pub email_enabled: bool,
    pub push_enabled: bool,
    pub sms_enabled: bool,
    pub announcement_email: bool,
    pub announcement_push: bool,
    pub message_email: bool,
    pub message_push: bool,
    pub grade_email: bool,
    pub grade_push: bool,
    pub assignment_email: bool,
    pub assignment_push: bool,
    pub forum_email: bool,
    pub forum_push: bool,
}

// ==================== Request Types ====================

#[derive(Debug, Deserialize)]
pub struct CreateAnnouncementRequest {
    pub title: String,
    pub content: String,
    pub course_id: Option<Uuid>,
    pub target_type: AnnouncementTarget,
    pub target_ids: Vec<Uuid>,
    pub priority: AnnouncementPriority,
    pub publish_now: bool,
    pub scheduled_at: Option<DateTime<Utc>>,
    pub expires_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateAnnouncementRequest {
    pub title: Option<String>,
    pub content: Option<String>,
    pub priority: Option<AnnouncementPriority>,
    pub expires_at: Option<DateTime<Utc>>,
    pub is_published: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct SendMessageRequest {
    pub receiver_id: Uuid,
    pub subject: Option<String>,
    pub body: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateForumCategoryRequest {
    pub name: String,
    pub description: String,
    pub position: i32,
}

#[derive(Debug, Deserialize)]
pub struct CreateForumRequest {
    pub name: String,
    pub description: String,
    pub category_id: Uuid,
    pub course_id: Option<Uuid>,
}

#[derive(Debug, Deserialize)]
pub struct CreateForumThreadRequest {
    pub title: String,
    pub body: String,
    pub is_announcement: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct CreateForumPostRequest {
    pub body: String,
    pub parent_post_id: Option<Uuid>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateNotificationPreferencesRequest {
    pub email_enabled: Option<bool>,
    pub push_enabled: Option<bool>,
    pub announcement_email: Option<bool>,
    pub announcement_push: Option<bool>,
    pub message_email: Option<bool>,
    pub message_push: Option<bool>,
    pub grade_email: Option<bool>,
    pub grade_push: Option<bool>,
    pub assignment_email: Option<bool>,
    pub assignment_push: Option<bool>,
    pub forum_email: Option<bool>,
    pub forum_push: Option<bool>,
}

// ==================== Response Types ====================

#[derive(Debug, Serialize)]
pub struct AnnouncementResponse {
    pub announcement: Announcement,
    pub creator_name: String,
    pub target_count: i32,
}

#[derive(Debug, Serialize)]
pub struct MessageResponse {
    pub message: Message,
    pub sender_name: String,
}

#[derive(Debug, Serialize)]
pub struct MessageThreadResponse {
    pub thread: MessageThread,
    pub other_participant_name: String,
    pub other_participant_avatar: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ForumDetailResponse {
    pub forum: Forum,
    pub category_name: String,
    pub thread_count: i32,
    pub total_posts: i32,
}

#[derive(Debug, Serialize)]
pub struct ForumThreadDetailResponse {
    pub thread: ForumThread,
    pub posts: Vec<ForumPostWithAuthor>,
    pub author_name: String,
}

#[derive(Debug, Serialize)]
pub struct ForumPostWithAuthor {
    pub post: ForumPost,
    pub author_name: String,
    pub author_role: String,
    pub reactions: Vec<PostReaction>,
    pub reply_count: i32,
}

#[derive(Debug, Serialize)]
pub struct NotificationResponse {
    pub notification: Notification,
}

#[derive(Debug, Serialize)]
pub struct NotificationListResponse {
    pub notifications: Vec<NotificationResponse>,
    pub unread_count: i32,
    pub total_count: i32,
}

#[derive(Debug, Serialize)]
pub struct NotificationPreferencesResponse {
    pub preferences: NotificationPreferences,
}
