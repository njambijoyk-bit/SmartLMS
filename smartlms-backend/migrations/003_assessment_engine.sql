-- Assessment Engine Migration
-- Adds support for advanced assessments, question banks, attempts, and grading

-- Enable UUID extension if not already enabled
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Questions table (question bank)
CREATE TABLE IF NOT EXISTS questions (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    course_id UUID NOT NULL REFERENCES courses(id) ON DELETE CASCADE,
    created_by UUID NOT NULL REFERENCES users(id),
    question_type VARCHAR(50) NOT NULL,
    text TEXT NOT NULL,
    media_url TEXT,
    options JSONB,
    correct_answer_hint TEXT,
    points INTEGER NOT NULL DEFAULT 1,
    tags TEXT,
    is_public BOOLEAN DEFAULT false,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Assessments table (exams/quizzes/assignments)
CREATE TABLE IF NOT EXISTS assessments (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    course_group_id UUID REFERENCES course_groups(id) ON DELETE SET NULL,
    course_id UUID NOT NULL REFERENCES courses(id) ON DELETE CASCADE,
    created_by UUID NOT NULL REFERENCES users(id),
    title VARCHAR(500) NOT NULL,
    description TEXT,
    assessment_type VARCHAR(50) NOT NULL,
    start_time TIMESTAMPTZ,
    end_time TIMESTAMPTZ,
    due_date TIMESTAMPTZ,
    time_limit_minutes INTEGER,
    passing_score INTEGER NOT NULL DEFAULT 60,
    shuffle_questions BOOLEAN DEFAULT false,
    shuffle_options BOOLEAN DEFAULT false,
    show_results BOOLEAN DEFAULT true,
    show_results_immediately BOOLEAN DEFAULT false,
    allow_retries BOOLEAN DEFAULT false,
    max_retries INTEGER,
    require_lockdown_browser BOOLEAN DEFAULT false,
    allow_late_submission BOOLEAN DEFAULT false,
    late_penalty_percent INTEGER DEFAULT 0,
    is_published BOOLEAN DEFAULT false,
    status VARCHAR(50) DEFAULT 'draft',
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Assessment-Question linkage (ordering and points override)
CREATE TABLE IF NOT EXISTS assessment_questions (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    assessment_id UUID NOT NULL REFERENCES assessments(id) ON DELETE CASCADE,
    question_id UUID NOT NULL REFERENCES questions(id) ON DELETE CASCADE,
    order_index INTEGER NOT NULL,
    points_override INTEGER,
    UNIQUE(assessment_id, question_id)
);

-- Student attempts at assessments
CREATE TABLE IF NOT EXISTS assessment_attempts (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    assessment_id UUID NOT NULL REFERENCES assessments(id) ON DELETE CASCADE,
    student_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    started_at TIMESTAMPTZ DEFAULT NOW(),
    submitted_at TIMESTAMPTZ,
    status VARCHAR(50) DEFAULT 'in_progress',
    total_score DECIMAL(5,2),
    percentage DECIMAL(5,2),
    passed BOOLEAN,
    time_spent_seconds INTEGER DEFAULT 0,
    is_late BOOLEAN DEFAULT false,
    lockdown_session_id TEXT,
    ip_address INET,
    attempt_number INTEGER DEFAULT 1,
    UNIQUE(assessment_id, student_id, attempt_number)
);

-- Individual answers within an attempt
CREATE TABLE IF NOT EXISTS attempt_answers (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    attempt_id UUID NOT NULL REFERENCES assessment_attempts(id) ON DELETE CASCADE,
    question_id UUID NOT NULL REFERENCES questions(id),
    answer_text TEXT,
    selected_options JSONB,
    code_content TEXT,
    file_url TEXT,
    is_correct BOOLEAN,
    points_earned INTEGER,
    auto_grade_score DECIMAL(5,2),
    manual_grade_score DECIMAL(5,2),
    feedback TEXT,
    graded_by UUID REFERENCES users(id),
    graded_at TIMESTAMPTZ,
    UNIQUE(attempt_id, question_id)
);

-- Gradebook entries (aggregated grades)
CREATE TABLE IF NOT EXISTS gradebook_entries (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    course_group_id UUID NOT NULL REFERENCES course_groups(id) ON DELETE CASCADE,
    student_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    assessment_id UUID REFERENCES assessments(id) ON DELETE SET NULL,
    category VARCHAR(100),
    score DECIMAL(5,2) NOT NULL,
    max_score DECIMAL(5,2) NOT NULL,
    percent DECIMAL(5,2) NOT NULL,
    letter_grade VARCHAR(10),
    feedback TEXT,
    graded_by UUID REFERENCES users(id),
    graded_at TIMESTAMPTZ DEFAULT NOW(),
    created_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(course_group_id, student_id, assessment_id)
);

-- Grade categories for weighted grading
CREATE TABLE IF NOT EXISTS grade_categories (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    course_id UUID NOT NULL REFERENCES courses(id) ON DELETE CASCADE,
    name VARCHAR(100) NOT NULL,
    weight DECIMAL(5,2) NOT NULL DEFAULT 100.00,
    drop_lowest INTEGER,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(course_id, name)
);

-- Indexes for performance optimization
CREATE INDEX IF NOT EXISTS idx_questions_course ON questions(course_id);
CREATE INDEX IF NOT EXISTS idx_questions_created_by ON questions(created_by);
CREATE INDEX IF NOT EXISTS idx_questions_type ON questions(question_type);
CREATE INDEX IF NOT EXISTS idx_questions_is_public ON questions(is_public);

CREATE INDEX IF NOT EXISTS idx_assessments_course ON assessments(course_id);
CREATE INDEX IF NOT EXISTS idx_assessments_group ON assessments(course_group_id);
CREATE INDEX IF NOT EXISTS idx_assessments_created_by ON assessments(created_by);
CREATE INDEX IF NOT EXISTS idx_assessments_status ON assessments(status);
CREATE INDEX IF NOT EXISTS idx_assessments_due_date ON assessments(due_date);

CREATE INDEX IF NOT EXISTS idx_assessment_questions_assessment ON assessment_questions(assessment_id);
CREATE INDEX IF NOT EXISTS idx_assessment_questions_question ON assessment_questions(question_id);

CREATE INDEX IF NOT EXISTS idx_attempts_assessment ON assessment_attempts(assessment_id);
CREATE INDEX IF NOT EXISTS idx_attempts_student ON assessment_attempts(student_id);
CREATE INDEX IF NOT EXISTS idx_attempts_status ON assessment_attempts(status);

CREATE INDEX IF NOT EXISTS idx_answers_attempt ON attempt_answers(attempt_id);
CREATE INDEX IF NOT EXISTS idx_answers_question ON attempt_answers(question_id);

CREATE INDEX IF NOT EXISTS idx_gradebook_course_group ON gradebook_entries(course_group_id);
CREATE INDEX IF NOT EXISTS idx_gradebook_student ON gradebook_entries(student_id);
CREATE INDEX IF NOT EXISTS idx_gradebook_assessment ON gradebook_entries(assessment_id);

-- Comments for documentation
COMMENT ON TABLE questions IS 'Question bank storing all question types';
COMMENT ON TABLE assessments IS 'Assessments (quizzes, exams, assignments) linked to courses or groups';
COMMENT ON TABLE assessment_questions IS 'Links questions to assessments with ordering';
COMMENT ON TABLE assessment_attempts IS 'Student attempts at assessments';
COMMENT ON TABLE attempt_answers IS 'Individual answers within an attempt';
COMMENT ON TABLE gradebook_entries IS 'Aggregated gradebook entries';
COMMENT ON TABLE grade_categories IS 'Weighted grading categories';

COMMENT ON COLUMN assessments.course_group_id IS 'NULL means assessment is for entire course, otherwise specific to lecturer group';
COMMENT ON COLUMN assessments.require_lockdown_browser IS 'Enforce exam mode with browser lockdown';
COMMENT ON COLUMN assessments.late_penalty_percent IS 'Percentage penalty for late submission (0-100)';
COMMENT ON COLUMN assessment_attempts.lockdown_session_id IS 'Track lockdown browser session for proctoring';
COMMENT ON COLUMN attempt_answers.code_content IS 'Source code for programming questions';
COMMENT ON COLUMN attempt_answers.auto_grade_score IS 'Score from automatic grading (MCQ, code)';
COMMENT ON COLUMN attempt_answers.manual_grade_score IS 'Score from manual grading (essays)';
