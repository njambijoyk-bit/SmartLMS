-- Per-institution schema — Phase 1.3 (assessments + question bank + attempts).
-- Runs after 002_courses_and_enrollments.sql. Idempotent.
--
-- Scope per master reference §4 module 7 (Assessment Engine):
--   * questions             — reusable question bank
--   * assessments           — course-scoped test definitions
--   * assessment_questions  — M:N link with per-assessment position + points override
--   * attempts              — a user's sitting of an assessment
--   * attempt_answers       — per-question response + grading result
--
-- Auto-grading is handled in Rust for MCQ / true-false / short-answer / numeric.
-- Essay + code questions are stored here but graded out-of-band (manual or ML).

-- ---------------------------------------------------------------------------
-- questions — reusable across assessments
-- ---------------------------------------------------------------------------
CREATE TABLE IF NOT EXISTS questions (
    id              UUID        PRIMARY KEY DEFAULT gen_random_uuid(),
    author_id       UUID        NOT NULL REFERENCES users(id) ON DELETE RESTRICT,
    -- mcq | multi | true_false | short_answer | numeric | essay | code
    kind            TEXT        NOT NULL
                    CHECK (kind IN ('mcq', 'multi', 'true_false',
                                    'short_answer', 'numeric', 'essay', 'code')),
    stem            TEXT        NOT NULL,
    -- Kind-specific static content:
    --   mcq/multi       : { "options": [{"id":"a","text":"..."}, ...] }
    --   numeric         : { "tolerance": 0.01 }
    --   code            : { "language": "python", "starter": "..." }
    body            JSONB       NOT NULL DEFAULT '{}'::jsonb,
    -- Authoritative answer (never returned to learners):
    --   mcq             : { "option_id": "a" }
    --   multi           : { "option_ids": ["a","b"] }
    --   true_false      : { "value": true }
    --   short_answer    : { "accepted": ["paris", "Paris, France"], "ci": true }
    --   numeric         : { "value": 3.14 }
    --   essay/code      : { "rubric": "..." }
    answer          JSONB       NOT NULL DEFAULT '{}'::jsonb,
    default_points  NUMERIC(6,2) NOT NULL DEFAULT 1
                    CHECK (default_points >= 0),
    explanation     TEXT,
    tags            TEXT[]      NOT NULL DEFAULT ARRAY[]::TEXT[],
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS questions_author_idx ON questions (author_id);
CREATE INDEX IF NOT EXISTS questions_kind_idx   ON questions (kind);
CREATE INDEX IF NOT EXISTS questions_tags_idx   ON questions USING GIN (tags);

DROP TRIGGER IF EXISTS set_questions_updated_at ON questions;
CREATE TRIGGER set_questions_updated_at
    BEFORE UPDATE ON questions
    FOR EACH ROW EXECUTE FUNCTION set_updated_at();

-- ---------------------------------------------------------------------------
-- assessments — course-scoped quiz / exam / survey
-- ---------------------------------------------------------------------------
CREATE TABLE IF NOT EXISTS assessments (
    id                   UUID        PRIMARY KEY DEFAULT gen_random_uuid(),
    course_id            UUID        NOT NULL REFERENCES courses(id) ON DELETE CASCADE,
    title                TEXT        NOT NULL,
    description          TEXT,
    -- quiz | exam | survey | practice
    kind                 TEXT        NOT NULL DEFAULT 'quiz'
                         CHECK (kind IN ('quiz', 'exam', 'survey', 'practice')),
    status               TEXT        NOT NULL DEFAULT 'draft'
                         CHECK (status IN ('draft', 'published', 'archived')),
    time_limit_minutes   INT         CHECK (time_limit_minutes IS NULL OR time_limit_minutes > 0),
    -- NULL = unlimited, 0 is not allowed.
    max_attempts         INT         CHECK (max_attempts IS NULL OR max_attempts > 0),
    passing_score_pct    NUMERIC(5,2) NOT NULL DEFAULT 60
                         CHECK (passing_score_pct BETWEEN 0 AND 100),
    shuffle_questions    BOOLEAN     NOT NULL DEFAULT false,
    show_results_policy  TEXT        NOT NULL DEFAULT 'after_submit'
                         CHECK (show_results_policy IN
                                ('never', 'after_submit', 'after_close', 'immediately')),
    available_from       TIMESTAMPTZ,
    available_until      TIMESTAMPTZ,
    created_at           TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at           TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS assessments_course_idx ON assessments (course_id);
CREATE INDEX IF NOT EXISTS assessments_status_idx ON assessments (status);

DROP TRIGGER IF EXISTS set_assessments_updated_at ON assessments;
CREATE TRIGGER set_assessments_updated_at
    BEFORE UPDATE ON assessments
    FOR EACH ROW EXECUTE FUNCTION set_updated_at();

-- ---------------------------------------------------------------------------
-- assessment_questions — M:N link
-- ---------------------------------------------------------------------------
CREATE TABLE IF NOT EXISTS assessment_questions (
    assessment_id  UUID        NOT NULL REFERENCES assessments(id) ON DELETE CASCADE,
    question_id    UUID        NOT NULL REFERENCES questions(id)   ON DELETE RESTRICT,
    position       INT         NOT NULL DEFAULT 0,
    -- NULL → use questions.default_points.
    points_override NUMERIC(6,2) CHECK (points_override IS NULL OR points_override >= 0),
    added_at       TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (assessment_id, question_id)
);

CREATE INDEX IF NOT EXISTS assessment_questions_ordered
    ON assessment_questions (assessment_id, position);

-- ---------------------------------------------------------------------------
-- attempts — a user's sitting of an assessment
-- ---------------------------------------------------------------------------
CREATE TABLE IF NOT EXISTS attempts (
    id                UUID        PRIMARY KEY DEFAULT gen_random_uuid(),
    assessment_id     UUID        NOT NULL REFERENCES assessments(id) ON DELETE CASCADE,
    user_id           UUID        NOT NULL REFERENCES users(id)       ON DELETE CASCADE,
    -- in_progress | submitted | graded | expired
    state             TEXT        NOT NULL DEFAULT 'in_progress'
                      CHECK (state IN ('in_progress', 'submitted', 'graded', 'expired')),
    started_at        TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    submitted_at      TIMESTAMPTZ,
    due_at            TIMESTAMPTZ,
    -- Denormalised totals recomputed on grading.
    score_points      NUMERIC(8,2) NOT NULL DEFAULT 0,
    max_points        NUMERIC(8,2) NOT NULL DEFAULT 0,
    score_pct         NUMERIC(5,2),
    passed            BOOLEAN,
    requires_manual   BOOLEAN     NOT NULL DEFAULT false,
    attempt_no        INT         NOT NULL DEFAULT 1
                      CHECK (attempt_no > 0),
    created_at        TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at        TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS attempts_user_idx      ON attempts (user_id);
CREATE INDEX IF NOT EXISTS attempts_assessment_idx ON attempts (assessment_id);
CREATE INDEX IF NOT EXISTS attempts_state_idx     ON attempts (state);

-- Only one in_progress attempt per (user, assessment).
CREATE UNIQUE INDEX IF NOT EXISTS attempts_one_open_per_user
    ON attempts (assessment_id, user_id)
    WHERE state = 'in_progress';

DROP TRIGGER IF EXISTS set_attempts_updated_at ON attempts;
CREATE TRIGGER set_attempts_updated_at
    BEFORE UPDATE ON attempts
    FOR EACH ROW EXECUTE FUNCTION set_updated_at();

-- ---------------------------------------------------------------------------
-- attempt_answers — one row per answered question
-- ---------------------------------------------------------------------------
CREATE TABLE IF NOT EXISTS attempt_answers (
    attempt_id     UUID        NOT NULL REFERENCES attempts(id)  ON DELETE CASCADE,
    question_id    UUID        NOT NULL REFERENCES questions(id) ON DELETE RESTRICT,
    response       JSONB       NOT NULL DEFAULT '{}'::jsonb,
    is_correct     BOOLEAN,
    points_earned  NUMERIC(6,2) NOT NULL DEFAULT 0,
    -- auto | manual | pending
    graded_by      TEXT        NOT NULL DEFAULT 'pending'
                   CHECK (graded_by IN ('auto', 'manual', 'pending')),
    grader_id      UUID REFERENCES users(id) ON DELETE SET NULL,
    graded_at      TIMESTAMPTZ,
    feedback       TEXT,
    created_at     TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at     TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (attempt_id, question_id)
);

CREATE INDEX IF NOT EXISTS attempt_answers_attempt_idx ON attempt_answers (attempt_id);
CREATE INDEX IF NOT EXISTS attempt_answers_grader_idx  ON attempt_answers (grader_id);

DROP TRIGGER IF EXISTS set_attempt_answers_updated_at ON attempt_answers;
CREATE TRIGGER set_attempt_answers_updated_at
    BEFORE UPDATE ON attempt_answers
    FOR EACH ROW EXECUTE FUNCTION set_updated_at();
