-- Phase 11: Advanced Academic Features
-- Competency-Based Education, Micro-Credentials, Student Wellbeing, Academic Advising, Research Supervision

CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- COMPETENCY-BASED EDUCATION
CREATE TABLE competency_frameworks (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR(255) NOT NULL,
    description TEXT,
    version VARCHAR(50) DEFAULT '1.0',
    institution_id UUID REFERENCES institutions(id),
    is_active BOOLEAN DEFAULT true,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE competencies (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    framework_id UUID REFERENCES competency_frameworks(id) ON DELETE CASCADE,
    parent_competency_id UUID REFERENCES competencies(id) ON DELETE SET NULL,
    code VARCHAR(50) NOT NULL,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    level INTEGER DEFAULT 1,
    category VARCHAR(100),
    is_assessable BOOLEAN DEFAULT true,
    sort_order INTEGER DEFAULT 0,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE course_competency_mappings (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    course_id UUID REFERENCES courses(id) ON DELETE CASCADE,
    competency_id UUID REFERENCES competencies(id) ON DELETE CASCADE,
    proficiency_level_required INTEGER DEFAULT 1,
    is_primary BOOLEAN DEFAULT false,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(course_id, competency_id)
);

CREATE TABLE student_competency_progress (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    student_id UUID REFERENCES users(id) ON DELETE CASCADE,
    competency_id UUID REFERENCES competencies(id) ON DELETE CASCADE,
    course_id UUID REFERENCES courses(id),
    proficiency_level_achieved INTEGER DEFAULT 0,
    evidence_count INTEGER DEFAULT 0,
    last_assessed_at TIMESTAMPTZ,
    assessed_by UUID REFERENCES users(id),
    status VARCHAR(50) DEFAULT 'in_progress',
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(student_id, competency_id, course_id)
);

CREATE TABLE competency_evidence (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    student_id UUID REFERENCES users(id) ON DELETE CASCADE,
    competency_id UUID REFERENCES competencies(id) ON DELETE CASCADE,
    submission_id UUID REFERENCES submissions(id),
    evidence_type VARCHAR(50),
    description TEXT,
    file_urls TEXT[],
    external_url VARCHAR(500),
    submitted_at TIMESTAMPTZ DEFAULT NOW(),
    assessed_at TIMESTAMPTZ,
    assessed_by UUID REFERENCES users(id),
    assessment_notes TEXT,
    is_approved BOOLEAN DEFAULT false,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- MICRO-CREDENTIALS
CREATE TABLE micro_credentials (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR(255) NOT NULL,
    description TEXT,
    short_description VARCHAR(500),
    credential_type VARCHAR(50),
    issuer_id UUID REFERENCES institutions(id),
    image_url VARCHAR(500),
    criteria_description TEXT,
    estimated_hours INTEGER,
    difficulty_level VARCHAR(50),
    is_stackable BOOLEAN DEFAULT true,
    expiry_months INTEGER,
    is_active BOOLEAN DEFAULT true,
    metadata JSONB DEFAULT '{}',
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE micro_credential_requirements (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    micro_credential_id UUID REFERENCES micro_credentials(id) ON DELETE CASCADE,
    requirement_type VARCHAR(50),
    target_id UUID,
    target_type VARCHAR(50),
    minimum_score DECIMAL(5,2),
    is_mandatory BOOLEAN DEFAULT true,
    sort_order INTEGER DEFAULT 0,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE awarded_micro_credentials (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    student_id UUID REFERENCES users(id) ON DELETE CASCADE,
    micro_credential_id UUID REFERENCES micro_credentials(id) ON DELETE CASCADE,
    awarded_at TIMESTAMPTZ DEFAULT NOW(),
    expires_at TIMESTAMPTZ,
    credential_hash VARCHAR(100) UNIQUE,
    blockchain_tx_id VARCHAR(100),
    verification_url VARCHAR(500),
    metadata JSONB DEFAULT '{}',
    is_revoked BOOLEAN DEFAULT false,
    revoked_at TIMESTAMPTZ,
    revoked_reason TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE badge_showcases (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID REFERENCES users(id) ON DELETE CASCADE,
    is_public BOOLEAN DEFAULT true,
    custom_title VARCHAR(255),
    custom_description TEXT,
    featured_badge_ids UUID[],
    layout_template VARCHAR(50),
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- STUDENT WELLBEING
CREATE TABLE wellbeing_checkins (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    student_id UUID REFERENCES users(id) ON DELETE CASCADE,
    mood_score INTEGER CHECK (mood_score BETWEEN 1 AND 10),
    stress_level INTEGER CHECK (stress_level BETWEEN 1 AND 10),
    energy_level INTEGER CHECK (energy_level BETWEEN 1 AND 10),
    sleep_hours DECIMAL(3,1),
    notes TEXT,
    tags VARCHAR(100)[],
    is_anonymous BOOLEAN DEFAULT false,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE wellbeing_resources (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    title VARCHAR(255) NOT NULL,
    description TEXT,
    resource_type VARCHAR(50),
    category VARCHAR(100),
    content_url VARCHAR(500),
    thumbnail_url VARCHAR(500),
    duration_minutes INTEGER,
    target_audience VARCHAR(100),
    is_featured BOOLEAN DEFAULT false,
    view_count INTEGER DEFAULT 0,
    helpful_count INTEGER DEFAULT 0,
    created_by UUID REFERENCES users(id),
    is_active BOOLEAN DEFAULT true,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE counseling_appointments (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    student_id UUID REFERENCES users(id) ON DELETE CASCADE,
    counselor_id UUID REFERENCES users(id),
    appointment_type VARCHAR(50),
    scheduled_at TIMESTAMPTZ NOT NULL,
    duration_minutes INTEGER DEFAULT 30,
    location VARCHAR(255),
    video_conference_url VARCHAR(500),
    status VARCHAR(50) DEFAULT 'scheduled',
    notes TEXT,
    follow_up_required BOOLEAN DEFAULT false,
    follow_up_date TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE early_alerts (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    student_id UUID REFERENCES users(id) ON DELETE CASCADE,
    alert_type VARCHAR(50),
    severity VARCHAR(50) DEFAULT 'medium',
    trigger_reason TEXT,
    triggered_by UUID REFERENCES users(id),
    assigned_to UUID REFERENCES users(id),
    status VARCHAR(50) DEFAULT 'new',
    action_plan TEXT,
    resolved_at TIMESTAMPTZ,
    resolved_by UUID REFERENCES users(id),
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- ACADEMIC ADVISING
CREATE TABLE advisor_assignments (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    advisor_id UUID REFERENCES users(id) ON DELETE CASCADE,
    student_id UUID REFERENCES users(id) ON DELETE CASCADE,
    assignment_type VARCHAR(50),
    department_id UUID REFERENCES departments(id),
    start_date DATE NOT NULL,
    end_date DATE,
    is_active BOOLEAN DEFAULT true,
    notes TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(advisor_id, student_id, start_date)
);

CREATE TABLE advising_sessions (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    advisor_id UUID REFERENCES users(id) ON DELETE CASCADE,
    student_id UUID REFERENCES users(id) ON DELETE CASCADE,
    session_type VARCHAR(50),
    scheduled_at TIMESTAMPTZ,
    completed_at TIMESTAMPTZ,
    location VARCHAR(255),
    video_conference_url VARCHAR(500),
    agenda TEXT,
    notes TEXT,
    action_items JSONB DEFAULT '[]',
    follow_up_scheduled BOOLEAN DEFAULT false,
    follow_up_date TIMESTAMPTZ,
    student_feedback_rating INTEGER CHECK (student_feedback_rating BETWEEN 1 AND 5),
    student_feedback_comments TEXT,
    status VARCHAR(50) DEFAULT 'scheduled',
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE degree_requirements (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    program_id UUID REFERENCES programs(id) ON DELETE CASCADE,
    requirement_type VARCHAR(50),
    category VARCHAR(100),
    minimum_value DECIMAL(10,2),
    description TEXT,
    is_mandatory BOOLEAN DEFAULT true,
    sort_order INTEGER DEFAULT 0,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE degree_audits (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    student_id UUID REFERENCES users(id) ON DELETE CASCADE,
    program_id UUID REFERENCES programs(id),
    audit_date TIMESTAMPTZ DEFAULT NOW(),
    total_credits_required DECIMAL(10,2),
    total_credits_earned DECIMAL(10,2),
    gpa_required DECIMAL(5,2),
    gpa_current DECIMAL(5,2),
    requirements_met JSONB DEFAULT '[]',
    requirements_pending JSONB DEFAULT '[]',
    estimated_graduation DATE,
    is_on_track BOOLEAN DEFAULT true,
    warnings TEXT[],
    recommendations TEXT[],
    generated_by UUID REFERENCES users(id),
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE course_plans (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    student_id UUID REFERENCES users(id) ON DELETE CASCADE,
    plan_name VARCHAR(255),
    plan_type VARCHAR(50),
    target_graduation DATE,
    semesters JSONB DEFAULT '[]',
    total_planned_credits DECIMAL(10,2),
    is_submitted BOOLEAN DEFAULT false,
    approved_by UUID REFERENCES users(id),
    approved_at TIMESTAMPTZ,
    comments TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- RESEARCH & SUPERVISION
CREATE TABLE research_projects (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    title VARCHAR(500) NOT NULL,
    description TEXT,
    abstract TEXT,
    principal_investigator_id UUID REFERENCES users(id),
    department_id UUID REFERENCES departments(id),
    funding_source VARCHAR(255),
    funding_amount DECIMAL(15,2),
    start_date DATE,
    end_date DATE,
    status VARCHAR(50) DEFAULT 'proposed',
    keywords VARCHAR(200)[],
    field_of_study VARCHAR(200),
    methodology VARCHAR(500),
    expected_outcomes TEXT,
    ethical_approval_required BOOLEAN DEFAULT false,
    ethical_approval_status VARCHAR(50),
    website_url VARCHAR(500),
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE research_team_members (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    project_id UUID REFERENCES research_projects(id) ON DELETE CASCADE,
    user_id UUID REFERENCES users(id) ON DELETE CASCADE,
    role VARCHAR(100),
    contribution_percentage DECIMAL(5,2),
    start_date DATE,
    end_date DATE,
    is_active BOOLEAN DEFAULT true,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE theses (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    student_id UUID REFERENCES users(id) ON DELETE CASCADE,
    title VARCHAR(500) NOT NULL,
    abstract TEXT,
    thesis_type VARCHAR(50),
    supervisor_id UUID REFERENCES users(id),
    co_supervisor_ids UUID[],
    submission_date DATE,
    defense_date DATE,
    defense_location VARCHAR(255),
    defense_committee_ids UUID[],
    status VARCHAR(50) DEFAULT 'in_progress',
    grade VARCHAR(10),
    honors_designation VARCHAR(100),
    file_url VARCHAR(500),
    is_published BOOLEAN DEFAULT false,
    publication_url VARCHAR(500),
    keywords VARCHAR(200)[],
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE thesis_milestones (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    thesis_id UUID REFERENCES theses(id) ON DELETE CASCADE,
    milestone_type VARCHAR(100),
    title VARCHAR(255),
    description TEXT,
    due_date DATE,
    completed_date DATE,
    status VARCHAR(50) DEFAULT 'pending',
    notes TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE research_outputs (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    project_id UUID REFERENCES research_projects(id) ON DELETE SET NULL,
    output_type VARCHAR(50),
    title VARCHAR(500) NOT NULL,
    authors JSONB DEFAULT '[]',
    publication_date DATE,
    journal_or_conference VARCHAR(500),
    publisher VARCHAR(255),
    doi VARCHAR(100),
    isbn VARCHAR(50),
    url VARCHAR(500),
    citation_count INTEGER DEFAULT 0,
    is_peer_reviewed BOOLEAN DEFAULT false,
    impact_factor DECIMAL(5,2),
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- INDEXES
CREATE INDEX idx_competencies_framework ON competencies(framework_id);
CREATE INDEX idx_student_competency_student ON student_competency_progress(student_id);
CREATE INDEX idx_awarded_credentials_student ON awarded_micro_credentials(student_id);
CREATE INDEX idx_wellbeing_checkins_student ON wellbeing_checkins(student_id);
CREATE INDEX idx_early_alerts_student ON early_alerts(student_id);
CREATE INDEX idx_advisor_assignments_student ON advisor_assignments(student_id);
CREATE INDEX idx_degree_audits_student ON degree_audits(student_id);
CREATE INDEX idx_research_projects_pi ON research_projects(principal_investigator_id);
CREATE INDEX idx_theses_student ON theses(student_id);

-- DEFAULT WELLBEING RESOURCES
INSERT INTO wellbeing_resources (title, description, resource_type, category, target_audience, is_active) VALUES
('Managing Academic Stress', 'Tips for handling academic pressure', 'article', 'academic_stress', 'students', true),
('Mindfulness for Students', 'Guided meditation exercises', 'video', 'mental_health', 'students', true),
('Time Management Strategies', 'Effective study schedule management', 'article', 'academic_stress', 'students', true),
('Building Resilience', 'Develop coping strategies', 'article', 'mental_health', 'students', true),
('Sleep Hygiene Guide', 'Improve sleep for better performance', 'article', 'physical_health', 'students', true),
('Financial Wellness', 'Managing finances as a student', 'article', 'financial', 'students', true);
