-- Migration 004: Communication Module (Phase 7)
-- Announcements, Messaging, Forums, Notifications

-- ==================== ANNOUNCEMENTS ====================

CREATE TABLE IF NOT EXISTS announcements (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    institution_id UUID NOT NULL REFERENCES institutions(id) ON DELETE CASCADE,
    course_id UUID REFERENCES courses(id) ON DELETE CASCADE,
    title VARCHAR(500) NOT NULL,
    content TEXT NOT NULL,
    target_type VARCHAR(50) NOT NULL DEFAULT 'All', -- All, Role, Course, User, Group
    target_ids UUID[] DEFAULT '{}', -- Target user/course/role IDs
    priority VARCHAR(20) NOT NULL DEFAULT 'Normal', -- Low, Normal, High, Urgent
    is_published BOOLEAN NOT NULL DEFAULT false,
    published_at TIMESTAMPTZ,
    expires_at TIMESTAMPTZ,
    created_by UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_announcements_institution ON announcements(institution_id);
CREATE INDEX idx_announcements_course ON announcements(course_id);
CREATE INDEX idx_announcements_published ON announcements(is_published, published_at);
CREATE INDEX idx_announcements_priority ON announcements(priority DESC);
CREATE INDEX idx_announcements_expires ON announcements(expires_at);

-- ==================== MESSAGES ====================

CREATE TABLE IF NOT EXISTS messages (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    sender_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    receiver_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    subject VARCHAR(500),
    body TEXT NOT NULL,
    is_read BOOLEAN NOT NULL DEFAULT false,
    read_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_messages_sender ON messages(sender_id);
CREATE INDEX idx_messages_receiver ON messages(receiver_id);
CREATE INDEX idx_messages_unread ON messages(receiver_id, is_read) WHERE is_read = false;
CREATE INDEX idx_messages_created ON messages(created_at DESC);

-- ==================== FORUMS ====================

CREATE TABLE IF NOT EXISTS forum_categories (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    institution_id UUID NOT NULL REFERENCES institutions(id) ON DELETE CASCADE,
    name VARCHAR(200) NOT NULL,
    description TEXT,
    position INTEGER NOT NULL DEFAULT 0,
    is_locked BOOLEAN NOT NULL DEFAULT false,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_forum_categories_institution ON forum_categories(institution_id);
CREATE INDEX idx_forum_categories_position ON forum_categories(position);

CREATE TABLE IF NOT EXISTS forums (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    institution_id UUID NOT NULL REFERENCES institutions(id) ON DELETE CASCADE,
    course_id UUID REFERENCES courses(id) ON DELETE CASCADE,
    category_id UUID NOT NULL REFERENCES forum_categories(id) ON DELETE CASCADE,
    name VARCHAR(200) NOT NULL,
    description TEXT,
    is_locked BOOLEAN NOT NULL DEFAULT false,
    created_by UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_forums_institution ON forums(institution_id);
CREATE INDEX idx_forums_course ON forums(course_id);
CREATE INDEX idx_forums_category ON forums(category_id);

CREATE TABLE IF NOT EXISTS forum_threads (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    forum_id UUID NOT NULL REFERENCES forums(id) ON DELETE CASCADE,
    title VARCHAR(500) NOT NULL,
    is_pinned BOOLEAN NOT NULL DEFAULT false,
    is_locked BOOLEAN NOT NULL DEFAULT false,
    is_announcement BOOLEAN NOT NULL DEFAULT false, -- Instructor-only posts
    view_count INTEGER NOT NULL DEFAULT 0,
    post_count INTEGER NOT NULL DEFAULT 0,
    last_post_at TIMESTAMPTZ,
    last_post_by UUID REFERENCES users(id) ON DELETE SET NULL,
    created_by UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_forum_threads_forum ON forum_threads(forum_id);
CREATE INDEX idx_forum_threads_pinned ON forum_threads(is_pinned DESC, last_post_at DESC NULLS LAST);
CREATE INDEX idx_forum_threads_created ON forum_threads(created_at DESC);

CREATE TABLE IF NOT EXISTS forum_posts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    thread_id UUID NOT NULL REFERENCES forum_threads(id) ON DELETE CASCADE,
    parent_post_id UUID REFERENCES forum_posts(id) ON DELETE CASCADE, -- For nested replies
    author_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    body TEXT NOT NULL,
    is_edited BOOLEAN NOT NULL DEFAULT false,
    edited_at TIMESTAMPTZ,
    flag_count INTEGER NOT NULL DEFAULT 0,
    is_hidden BOOLEAN NOT NULL DEFAULT false,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_forum_posts_thread ON forum_posts(thread_id);
CREATE INDEX idx_forum_posts_parent ON forum_posts(parent_post_id);
CREATE INDEX idx_forum_posts_author ON forum_posts(author_id);
CREATE INDEX idx_forum_posts_flags ON forum_posts(flag_count) WHERE flag_count > 0;
CREATE INDEX idx_forum_posts_created ON forum_posts(created_at ASC);

CREATE TABLE IF NOT EXISTS post_reactions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    post_id UUID NOT NULL REFERENCES forum_posts(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    reaction_type VARCHAR(50) NOT NULL, -- like, helpful, disagree, etc.
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(post_id, user_id) -- One reaction per user per post
);

CREATE INDEX idx_post_reactions_post ON post_reactions(post_id);
CREATE INDEX idx_post_reactions_user ON post_reactions(user_id);

-- ==================== NOTIFICATIONS ====================

CREATE TABLE IF NOT EXISTS notifications (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    notification_type VARCHAR(50) NOT NULL, -- announcement, message, assignment, grade, etc.
    title VARCHAR(500) NOT NULL,
    message TEXT NOT NULL,
    action_url VARCHAR(500),
    is_read BOOLEAN NOT NULL DEFAULT false,
    read_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_notifications_user ON notifications(user_id);
CREATE INDEX idx_notifications_unread ON notifications(user_id, is_read) WHERE is_read = false;
CREATE INDEX idx_notifications_type ON notifications(notification_type);
CREATE INDEX idx_notifications_created ON notifications(created_at DESC);

CREATE TABLE IF NOT EXISTS notification_preferences (
    user_id UUID PRIMARY KEY REFERENCES users(id) ON DELETE CASCADE,
    email_enabled BOOLEAN NOT NULL DEFAULT true,
    push_enabled BOOLEAN NOT NULL DEFAULT true,
    sms_enabled BOOLEAN NOT NULL DEFAULT false,
    announcement_email BOOLEAN NOT NULL DEFAULT true,
    announcement_push BOOLEAN NOT NULL DEFAULT true,
    message_email BOOLEAN NOT NULL DEFAULT true,
    message_push BOOLEAN NOT NULL DEFAULT true,
    grade_email BOOLEAN NOT NULL DEFAULT true,
    grade_push BOOLEAN NOT NULL DEFAULT true,
    assignment_email BOOLEAN NOT NULL DEFAULT true,
    assignment_push BOOLEAN NOT NULL DEFAULT true,
    forum_email BOOLEAN NOT NULL DEFAULT true,
    forum_push BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- ==================== COMMENTS ====================

COMMENT ON TABLE announcements IS 'Institution and course-wide announcements with targeting';
COMMENT ON TABLE messages IS 'Direct messaging between users';
COMMENT ON TABLE forum_categories IS 'Forum organization categories';
COMMENT ON TABLE forums IS 'Discussion forums within courses or institution-wide';
COMMENT ON TABLE forum_threads IS 'Discussion threads within forums';
COMMENT ON TABLE forum_posts IS 'Posts in forum threads with nesting support';
COMMENT ON TABLE post_reactions IS 'Reactions to forum posts (like, helpful, etc.)';
COMMENT ON TABLE notifications IS 'User notifications for various events';
COMMENT ON TABLE notification_preferences IS 'Per-user notification delivery preferences';
