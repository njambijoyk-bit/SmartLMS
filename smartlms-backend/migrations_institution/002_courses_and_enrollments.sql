-- Per-institution schema — Phase 1.2 (courses + modules + enrollments).
-- Runs against each tenant's isolated Postgres DB after
-- 001_users_and_rbac.sql. Idempotent (IF NOT EXISTS, ON CONFLICT).
--
-- Scope per master reference §4 modules 6 + 7:
--   * courses       — unit of teaching, owned by an instructor user
--   * course_staff  — additional instructors/TAs per course (M:N)
--   * modules       — ordered groups within a course
--   * lessons       — leaf content items within a module (video/text/file/quiz)
--   * enrollments   — learner ↔ course assignment with status + progress
--   * lesson_progress — per-learner completion tracking for each lesson
--
-- Assessments/attempts land in PR #57 (migration 003).

-- ---------------------------------------------------------------------------
-- courses
-- ---------------------------------------------------------------------------
CREATE TABLE IF NOT EXISTS courses (
    id            UUID        PRIMARY KEY DEFAULT gen_random_uuid(),
    slug          TEXT        NOT NULL,
    title         TEXT        NOT NULL,
    subtitle      TEXT,
    description   TEXT,
    cover_url     TEXT,
    instructor_id UUID        NOT NULL REFERENCES users(id) ON DELETE RESTRICT,
    -- draft | published | archived
    status        TEXT        NOT NULL DEFAULT 'draft'
                  CHECK (status IN ('draft', 'published', 'archived')),
    visibility    TEXT        NOT NULL DEFAULT 'private'
                  CHECK (visibility IN ('private', 'institution', 'public')),
    language      TEXT        NOT NULL DEFAULT 'en',
    -- Free-form per-course config (grading scale, completion rule, etc.)
    config        JSONB       NOT NULL DEFAULT '{}'::jsonb,
    published_at  TIMESTAMPTZ,
    archived_at   TIMESTAMPTZ,
    created_at    TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at    TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE UNIQUE INDEX IF NOT EXISTS courses_slug_unique
    ON courses (lower(slug))
    WHERE archived_at IS NULL;

CREATE INDEX IF NOT EXISTS courses_instructor_idx   ON courses (instructor_id);
CREATE INDEX IF NOT EXISTS courses_status_idx       ON courses (status);

DROP TRIGGER IF EXISTS set_courses_updated_at ON courses;
CREATE TRIGGER set_courses_updated_at
    BEFORE UPDATE ON courses
    FOR EACH ROW EXECUTE FUNCTION set_updated_at();

-- ---------------------------------------------------------------------------
-- course_staff — extra instructors / TAs beyond the primary instructor
-- ---------------------------------------------------------------------------
CREATE TABLE IF NOT EXISTS course_staff (
    course_id   UUID        NOT NULL REFERENCES courses(id) ON DELETE CASCADE,
    user_id     UUID        NOT NULL REFERENCES users(id)   ON DELETE CASCADE,
    -- instructor | assistant | observer
    role_code   TEXT        NOT NULL DEFAULT 'assistant'
                CHECK (role_code IN ('instructor', 'assistant', 'observer')),
    granted_at  TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    granted_by  UUID REFERENCES users(id) ON DELETE SET NULL,
    PRIMARY KEY (course_id, user_id)
);

CREATE INDEX IF NOT EXISTS course_staff_user_idx ON course_staff (user_id);

-- ---------------------------------------------------------------------------
-- modules
-- ---------------------------------------------------------------------------
CREATE TABLE IF NOT EXISTS modules (
    id          UUID        PRIMARY KEY DEFAULT gen_random_uuid(),
    course_id   UUID        NOT NULL REFERENCES courses(id) ON DELETE CASCADE,
    title       TEXT        NOT NULL,
    summary     TEXT,
    position    INT         NOT NULL DEFAULT 0,
    unlock_rule JSONB       NOT NULL DEFAULT '{}'::jsonb,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at  TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS modules_course_idx ON modules (course_id, position);

DROP TRIGGER IF EXISTS set_modules_updated_at ON modules;
CREATE TRIGGER set_modules_updated_at
    BEFORE UPDATE ON modules
    FOR EACH ROW EXECUTE FUNCTION set_updated_at();

-- ---------------------------------------------------------------------------
-- lessons — leaf content within a module
-- ---------------------------------------------------------------------------
CREATE TABLE IF NOT EXISTS lessons (
    id          UUID        PRIMARY KEY DEFAULT gen_random_uuid(),
    module_id   UUID        NOT NULL REFERENCES modules(id) ON DELETE CASCADE,
    title       TEXT        NOT NULL,
    -- video | text | file | link | quiz | live
    kind        TEXT        NOT NULL
                CHECK (kind IN ('video', 'text', 'file', 'link', 'quiz', 'live')),
    -- Kind-specific payload (video URL, markdown body, file URL, quiz id, ...).
    content     JSONB       NOT NULL DEFAULT '{}'::jsonb,
    position    INT         NOT NULL DEFAULT 0,
    duration_s  INT,
    is_required BOOLEAN     NOT NULL DEFAULT true,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at  TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS lessons_module_idx ON lessons (module_id, position);

DROP TRIGGER IF EXISTS set_lessons_updated_at ON lessons;
CREATE TRIGGER set_lessons_updated_at
    BEFORE UPDATE ON lessons
    FOR EACH ROW EXECUTE FUNCTION set_updated_at();

-- ---------------------------------------------------------------------------
-- enrollments
-- ---------------------------------------------------------------------------
CREATE TABLE IF NOT EXISTS enrollments (
    id            UUID        PRIMARY KEY DEFAULT gen_random_uuid(),
    course_id     UUID        NOT NULL REFERENCES courses(id) ON DELETE CASCADE,
    user_id       UUID        NOT NULL REFERENCES users(id)   ON DELETE CASCADE,
    -- pending (invited) | active | completed | dropped | suspended
    status        TEXT        NOT NULL DEFAULT 'active'
                  CHECK (status IN ('pending', 'active', 'completed', 'dropped', 'suspended')),
    enrolled_at   TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    completed_at  TIMESTAMPTZ,
    -- 0-100, integer percent, updated by lesson_progress triggers/services.
    progress_pct  INT         NOT NULL DEFAULT 0
                  CHECK (progress_pct BETWEEN 0 AND 100),
    last_seen_at  TIMESTAMPTZ,
    metadata      JSONB       NOT NULL DEFAULT '{}'::jsonb,
    created_at    TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at    TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE UNIQUE INDEX IF NOT EXISTS enrollments_user_course_unique
    ON enrollments (user_id, course_id);
CREATE INDEX IF NOT EXISTS enrollments_course_idx ON enrollments (course_id);
CREATE INDEX IF NOT EXISTS enrollments_status_idx ON enrollments (status);

DROP TRIGGER IF EXISTS set_enrollments_updated_at ON enrollments;
CREATE TRIGGER set_enrollments_updated_at
    BEFORE UPDATE ON enrollments
    FOR EACH ROW EXECUTE FUNCTION set_updated_at();

-- ---------------------------------------------------------------------------
-- lesson_progress — per-learner per-lesson completion
-- ---------------------------------------------------------------------------
CREATE TABLE IF NOT EXISTS lesson_progress (
    user_id       UUID        NOT NULL REFERENCES users(id)   ON DELETE CASCADE,
    lesson_id     UUID        NOT NULL REFERENCES lessons(id) ON DELETE CASCADE,
    enrollment_id UUID        NOT NULL REFERENCES enrollments(id) ON DELETE CASCADE,
    -- started | completed | failed (failed is for quizzes graded elsewhere)
    status        TEXT        NOT NULL DEFAULT 'started'
                  CHECK (status IN ('started', 'completed', 'failed')),
    started_at    TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    completed_at  TIMESTAMPTZ,
    seconds_spent INT         NOT NULL DEFAULT 0,
    PRIMARY KEY (user_id, lesson_id)
);

CREATE INDEX IF NOT EXISTS lesson_progress_enrollment_idx
    ON lesson_progress (enrollment_id);
