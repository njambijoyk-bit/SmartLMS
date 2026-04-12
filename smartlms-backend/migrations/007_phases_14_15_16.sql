-- Phase 14, 15, 16: Employer Portal, Library, and Compliance
-- Migration file for SmartLMS

-- ============================================================
-- PHASE 14: EMPLOYER & CAREER PORTAL
-- ============================================================

-- Job Postings
CREATE TABLE IF NOT EXISTS job_postings (
    id BIGSERIAL PRIMARY KEY,
    employer_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    title VARCHAR(255) NOT NULL,
    company_name VARCHAR(255) NOT NULL,
    description TEXT NOT NULL,
    requirements TEXT[],
    location VARCHAR(255),
    location_type VARCHAR(50) DEFAULT 'onsite', -- onsite, remote, hybrid
    job_type VARCHAR(50) DEFAULT 'full-time', -- full-time, part-time, contract, internship
    experience_level VARCHAR(50) DEFAULT 'entry', -- entry, mid, senior, executive
    salary_min DECIMAL(12,2),
    salary_max DECIMAL(12,2),
    currency VARCHAR(10) DEFAULT 'USD',
    department VARCHAR(255),
    benefits TEXT[],
    skills_required VARCHAR(255)[],
    application_deadline TIMESTAMP WITH TIME ZONE,
    status VARCHAR(50) DEFAULT 'active', -- active, closed, paused, draft
    views_count INTEGER DEFAULT 0,
    applications_count INTEGER DEFAULT 0,
    company_logo_url VARCHAR(500),
    company_website VARCHAR(500),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_jobs_employer ON job_postings(employer_id);
CREATE INDEX idx_jobs_status ON job_postings(status);
CREATE INDEX idx_jobs_location_type ON job_postings(location_type);
CREATE INDEX idx_jobs_type ON job_postings(job_type);
CREATE INDEX idx_jobs_experience ON job_postings(experience_level);
CREATE INDEX idx_jobs_skills ON job_postings USING GIN(skills_required);

-- Job Applications
CREATE TABLE IF NOT EXISTS job_applications (
    id BIGSERIAL PRIMARY KEY,
    job_id BIGINT NOT NULL REFERENCES job_postings(id) ON DELETE CASCADE,
    candidate_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    resume_url VARCHAR(500),
    cover_letter TEXT,
    status VARCHAR(50) DEFAULT 'submitted', -- submitted, reviewed, shortlisted, interviewed, offered, rejected, withdrawn
    applied_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    reviewed_at TIMESTAMP WITH TIME ZONE,
    reviewed_by BIGINT REFERENCES users(id),
    notes TEXT,
    rating INTEGER CHECK (rating >= 1 AND rating <= 5),
    UNIQUE(job_id, candidate_id)
);

CREATE INDEX idx_applications_job ON job_applications(job_id);
CREATE INDEX idx_applications_candidate ON job_applications(candidate_id);
CREATE INDEX idx_applications_status ON job_applications(status);

-- Candidate Profiles
CREATE TABLE IF NOT EXISTS candidate_profiles (
    id BIGSERIAL PRIMARY KEY,
    user_id BIGINT UNIQUE NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    headline VARCHAR(255),
    summary TEXT,
    desired_roles VARCHAR(255)[],
    desired_locations VARCHAR(255)[],
    remote_preference BOOLEAN DEFAULT true,
    salary_expectation_min DECIMAL(12,2),
    salary_expectation_max DECIMAL(12,2),
    availability_status VARCHAR(50) DEFAULT 'not_available', -- immediately, two_weeks, one_month, not_available
    portfolio_url VARCHAR(500),
    linkedin_url VARCHAR(500),
    github_url VARCHAR(500),
    personal_website VARCHAR(500),
    skills VARCHAR(255)[],
    years_of_experience INTEGER DEFAULT 0,
    profile_visibility VARCHAR(50) DEFAULT 'public', -- public, employers_only, private
    profile_completeness INTEGER DEFAULT 0,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_candidates_user ON candidate_profiles(user_id);
CREATE INDEX idx_candidates_skills ON candidate_profiles USING GIN(skills);
CREATE INDEX idx_candidates_roles ON candidate_profiles USING GIN(desired_roles);

-- Candidate Education History
CREATE TABLE IF NOT EXISTS candidate_education (
    id BIGSERIAL PRIMARY KEY,
    candidate_profile_id BIGINT NOT NULL REFERENCES candidate_profiles(id) ON DELETE CASCADE,
    institution_name VARCHAR(255) NOT NULL,
    degree VARCHAR(255),
    field_of_study VARCHAR(255),
    start_date DATE,
    end_date DATE,
    gpa DECIMAL(3,2),
    gpa_scale DECIMAL(3,2) DEFAULT 4.0,
    achievements TEXT[],
    description TEXT
);

CREATE INDEX idx_education_profile ON candidate_education(candidate_profile_id);

-- Candidate Work Experience
CREATE TABLE IF NOT EXISTS candidate_experience (
    id BIGSERIAL PRIMARY KEY,
    candidate_profile_id BIGINT NOT NULL REFERENCES candidate_profiles(id) ON DELETE CASCADE,
    company_name VARCHAR(255) NOT NULL,
    job_title VARCHAR(255) NOT NULL,
    location VARCHAR(255),
    start_date DATE NOT NULL,
    end_date DATE,
    is_current BOOLEAN DEFAULT false,
    description TEXT,
    achievements TEXT[]
);

CREATE INDEX idx_experience_profile ON candidate_experience(candidate_profile_id);

-- Internship Opportunities
CREATE TABLE IF NOT EXISTS internships (
    id BIGSERIAL PRIMARY KEY,
    employer_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    title VARCHAR(255) NOT NULL,
    company_name VARCHAR(255) NOT NULL,
    description TEXT NOT NULL,
    requirements TEXT[],
    location VARCHAR(255),
    location_type VARCHAR(50) DEFAULT 'onsite',
    stipend_amount DECIMAL(12,2),
    stipend_period VARCHAR(50) DEFAULT 'monthly', -- hourly, weekly, monthly, lump_sum
    duration_months INTEGER,
    start_date DATE,
    end_date DATE,
    learning_objectives TEXT[],
    mentor_assigned BOOLEAN DEFAULT false,
    status VARCHAR(50) DEFAULT 'active',
    applications_count INTEGER DEFAULT 0,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_internships_employer ON internships(employer_id);
CREATE INDEX idx_internships_status ON internships(status);

-- Internship Applications
CREATE TABLE IF NOT EXISTS internship_applications (
    id BIGSERIAL PRIMARY KEY,
    internship_id BIGINT NOT NULL REFERENCES internships(id) ON DELETE CASCADE,
    candidate_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    resume_url VARCHAR(500),
    cover_letter TEXT,
    status VARCHAR(50) DEFAULT 'submitted',
    applied_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    reviewed_at TIMESTAMP WITH TIME ZONE,
    notes TEXT,
    UNIQUE(internship_id, candidate_id)
);

CREATE INDEX idx_intern_apps_internship ON internship_applications(internship_id);
CREATE INDEX idx_intern_apps_candidate ON internship_applications(candidate_id);

-- Recruitment Events
CREATE TABLE IF NOT EXISTS recruitment_events (
    id BIGSERIAL PRIMARY KEY,
    employer_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    title VARCHAR(255) NOT NULL,
    description TEXT,
    event_type VARCHAR(50) NOT NULL, -- career_fair, info_session, workshop, networking, interview_day, hackathon
    format VARCHAR(50) DEFAULT 'in_person', -- in_person, virtual, hybrid
    venue_name VARCHAR(255),
    venue_address TEXT,
    virtual_link VARCHAR(500),
    start_datetime TIMESTAMP WITH TIME ZONE NOT NULL,
    end_datetime TIMESTAMP WITH TIME ZONE NOT NULL,
    capacity INTEGER,
    registered_count INTEGER DEFAULT 0,
    agenda TEXT[],
    target_audience VARCHAR(255)[],
    registration_deadline TIMESTAMP WITH TIME ZONE,
    status VARCHAR(50) DEFAULT 'upcoming', -- upcoming, ongoing, completed, cancelled
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_events_employer ON recruitment_events(employer_id);
CREATE INDEX idx_events_type ON recruitment_events(event_type);
CREATE INDEX idx_events_format ON recruitment_events(format);
CREATE INDEX idx_events_status ON recruitment_events(status);

-- Event Registrations
CREATE TABLE IF NOT EXISTS event_registrations (
    id BIGSERIAL PRIMARY KEY,
    event_id BIGINT NOT NULL REFERENCES recruitment_events(id) ON DELETE CASCADE,
    user_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    registered_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    attendance_status VARCHAR(50) DEFAULT 'registered', -- registered, attended, no_show, cancelled
    notes TEXT,
    UNIQUE(event_id, user_id)
);

CREATE INDEX idx_registrations_event ON event_registrations(event_id);
CREATE INDEX idx_registrations_user ON event_registrations(user_id);

-- Industry Partnerships
CREATE TABLE IF NOT EXISTS industry_partnerships (
    id BIGSERIAL PRIMARY KEY,
    institution_id BIGINT REFERENCES institutions(id) ON DELETE SET NULL,
    partner_company_name VARCHAR(255) NOT NULL,
    partnership_type VARCHAR(50) NOT NULL, -- recruitment, curriculum, research, sponsorship, internship, advisory
    description TEXT,
    contact_person VARCHAR(255),
    contact_email VARCHAR(255),
    contact_phone VARCHAR(50),
    benefits_offered TEXT[],
    benefits_requested TEXT[],
    agreement_terms TEXT,
    start_date DATE,
    end_date DATE,
    status VARCHAR(50) DEFAULT 'active', -- active, pending, expired, terminated
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_partnerships_institution ON industry_partnerships(institution_id);
CREATE INDEX idx_partnerships_type ON industry_partnerships(partnership_type);
CREATE INDEX idx_partnerships_status ON industry_partnerships(status);

-- ============================================================
-- PHASE 15: LIBRARY & CONTENT REPOSITORY
-- ============================================================

-- Digital Resources
CREATE TABLE IF NOT EXISTS library_resources (
    id BIGSERIAL PRIMARY KEY,
    title VARCHAR(500) NOT NULL,
    author VARCHAR(255)[],
    contributor VARCHAR(255)[],
    publisher VARCHAR(255),
    publication_date DATE,
    resource_type VARCHAR(50) NOT NULL, -- book, article, video, audio, dataset, code, image, thesis, report, conference_paper
    format VARCHAR(50), -- pdf, epub, mp4, mp3, html, etc.
    isbn VARCHAR(50),
    issn VARCHAR(50),
    doi VARCHAR(255),
    pmid VARCHAR(100),
    arxiv_id VARCHAR(100),
    language VARCHAR(50) DEFAULT 'en',
    subject_tags VARCHAR(255)[],
    keywords TEXT[],
    abstract TEXT,
    description TEXT,
    file_url VARCHAR(500),
    thumbnail_url VARCHAR(500),
    access_type VARCHAR(50) DEFAULT 'open', -- open, subscription, restricted, embargoed
    license_type VARCHAR(100), -- CC-BY, CC-BY-SA, MIT, Apache, All Rights Reserved, etc.
    copyright_holder VARCHAR(255),
    edition VARCHAR(100),
    volume VARCHAR(100),
    issue VARCHAR(100),
    pages VARCHAR(100),
    series VARCHAR(255),
    open_access BOOLEAN DEFAULT false,
    peer_reviewed BOOLEAN DEFAULT false,
    citation_count INTEGER DEFAULT 0,
    download_count INTEGER DEFAULT 0,
    view_count INTEGER DEFAULT 0,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_resources_type ON library_resources(resource_type);
CREATE INDEX idx_resources_subject ON library_resources USING GIN(subject_tags);
CREATE INDEX idx_resources_keywords ON library_resources USING GIN(keywords);
CREATE INDEX idx_resources_author ON library_resources USING GIN(author);
CREATE INDEX idx_resources_access ON library_resources(access_type);
CREATE INDEX idx_resources_open_access ON library_resources(open_access);

-- Resource Identifiers (multiple per resource)
CREATE TABLE IF NOT EXISTS resource_identifiers (
    id BIGSERIAL PRIMARY KEY,
    resource_id BIGINT NOT NULL REFERENCES library_resources(id) ON DELETE CASCADE,
    identifier_type VARCHAR(50) NOT NULL, -- isbn, issn, doi, pmid, arxiv, uri, local
    identifier_value VARCHAR(255) NOT NULL,
    UNIQUE(resource_id, identifier_type, identifier_value)
);

CREATE INDEX idx_identifiers_resource ON resource_identifiers(resource_id);

-- Library Collections (hierarchical)
CREATE TABLE IF NOT EXISTS library_collections (
    id BIGSERIAL PRIMARY KEY,
    parent_collection_id BIGINT REFERENCES library_collections(id) ON DELETE SET NULL,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    collection_type VARCHAR(50) NOT NULL, -- departmental, course_reserves, special, thematic, institutional, user_created
    curator_id BIGINT REFERENCES users(id) ON DELETE SET NULL,
    visibility VARCHAR(50) DEFAULT 'public', -- public, restricted, private
    cover_image_url VARCHAR(500),
    resource_count INTEGER DEFAULT 0,
    sort_order INTEGER DEFAULT 0,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_collections_parent ON library_collections(parent_collection_id);
CREATE INDEX idx_collections_type ON library_collections(collection_type);
CREATE INDEX idx_collections_curator ON library_collections(curator_id);

-- Collection Resources (many-to-many)
CREATE TABLE IF NOT EXISTS collection_resources (
    id BIGSERIAL PRIMARY KEY,
    collection_id BIGINT NOT NULL REFERENCES library_collections(id) ON DELETE CASCADE,
    resource_id BIGINT NOT NULL REFERENCES library_resources(id) ON DELETE CASCADE,
    added_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    added_by BIGINT REFERENCES users(id),
    sort_order INTEGER DEFAULT 0,
    notes TEXT,
    UNIQUE(collection_id, resource_id)
);

CREATE INDEX idx_collection_res_collection ON collection_resources(collection_id);
CREATE INDEX idx_collection_res_resource ON collection_resources(resource_id);

-- Physical Items (for borrowing)
CREATE TABLE IF NOT EXISTS physical_items (
    id BIGSERIAL PRIMARY KEY,
    resource_id BIGINT REFERENCES library_resources(id) ON DELETE SET NULL,
    barcode VARCHAR(100) UNIQUE NOT NULL,
    call_number VARCHAR(255),
    location VARCHAR(255), -- shelf location
    item_type VARCHAR(50) DEFAULT 'book', -- book, dvd, equipment, etc.
    condition VARCHAR(50) DEFAULT 'good', -- new, good, fair, poor, damaged
    status VARCHAR(50) DEFAULT 'available', -- available, checked_out, reserved, lost, maintenance
    acquisition_date DATE,
    cost DECIMAL(10,2),
    vendor VARCHAR(255),
    notes TEXT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_items_resource ON physical_items(resource_id);
CREATE INDEX idx_items_barcode ON physical_items(barcode);
CREATE INDEX idx_items_status ON physical_items(status);

-- Loans (Borrowing Records)
CREATE TABLE IF NOT EXISTS library_loans (
    id BIGSERIAL PRIMARY KEY,
    item_id BIGINT NOT NULL REFERENCES physical_items(id) ON DELETE CASCADE,
    user_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    checkout_date TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    due_date TIMESTAMP WITH TIME ZONE NOT NULL,
    return_date TIMESTAMP WITH TIME ZONE,
    renewal_count INTEGER DEFAULT 0,
    max_renewals INTEGER DEFAULT 3,
    condition_at_checkout TEXT,
    condition_at_return TEXT,
    fine_amount DECIMAL(10,2) DEFAULT 0,
    fine_paid BOOLEAN DEFAULT false,
    status VARCHAR(50) DEFAULT 'active', -- active, returned, overdue, lost
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_loans_item ON library_loans(item_id);
CREATE INDEX idx_loans_user ON library_loans(user_id);
CREATE INDEX idx_loans_status ON library_loans(status);
CREATE INDEX idx_loans_due_date ON library_loans(due_date);

-- Course Resource Linkages
CREATE TABLE IF NOT EXISTS course_resource_links (
    id BIGSERIAL PRIMARY KEY,
    course_id BIGINT NOT NULL REFERENCES courses(id) ON DELETE CASCADE,
    resource_id BIGINT NOT NULL REFERENCES library_resources(id) ON DELETE CASCADE,
    linkage_type VARCHAR(50) NOT NULL, -- required_reading, supplementary, reference, multimedia, primary_source
    module_id BIGINT REFERENCES course_modules(id) ON DELETE SET NULL,
    week_number INTEGER,
    is_required BOOLEAN DEFAULT false,
    notes TEXT,
    added_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    added_by BIGINT REFERENCES users(id),
    UNIQUE(course_id, resource_id)
);

CREATE INDEX idx_course_res_course ON course_resource_links(course_id);
CREATE INDEX idx_course_res_resource ON course_resource_links(resource_id);
CREATE INDEX idx_course_res_type ON course_resource_links(linkage_type);

-- Citations
CREATE TABLE IF NOT EXISTS resource_citations (
    id BIGSERIAL PRIMARY KEY,
    resource_id BIGINT NOT NULL REFERENCES library_resources(id) ON DELETE CASCADE,
    citation_style VARCHAR(50) NOT NULL, -- apa, mla, chicago, ieee, harvard, vancouver, bibtex
    citation_text TEXT NOT NULL,
    generated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    generated_by BIGINT REFERENCES users(id)
);

CREATE INDEX idx_citations_resource ON resource_citations(resource_id);
CREATE INDEX idx_citations_style ON resource_citations(citation_style);

-- Bulk Upload Jobs
CREATE TABLE IF NOT EXISTS bulk_upload_jobs (
    id BIGSERIAL PRIMARY KEY,
    user_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    collection_id BIGINT REFERENCES library_collections(id) ON DELETE SET NULL,
    course_id BIGINT REFERENCES courses(id) ON DELETE SET NULL,
    total_items INTEGER DEFAULT 0,
    processed_items INTEGER DEFAULT 0,
    failed_items INTEGER DEFAULT 0,
    status VARCHAR(50) DEFAULT 'pending', -- pending, processing, completed, failed
    error_log JSONB,
    started_at TIMESTAMP WITH TIME ZONE,
    completed_at TIMESTAMP WITH TIME ZONE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_upload_jobs_user ON bulk_upload_jobs(user_id);
CREATE INDEX idx_upload_jobs_status ON bulk_upload_jobs(status);

-- ============================================================
-- PHASE 16: SECURITY HARDENING & COMPLIANCE
-- ============================================================

-- Proctoring Sessions
CREATE TABLE IF NOT EXISTS proctoring_sessions (
    id BIGSERIAL PRIMARY KEY,
    assessment_id BIGINT NOT NULL REFERENCES assessments(id) ON DELETE CASCADE,
    user_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    tier VARCHAR(50) NOT NULL, -- basic, standard, advanced, premium
    status VARCHAR(50) DEFAULT 'initialized', -- initialized, started, in_progress, completed, flagged, terminated
    browser_lockdown_enabled BOOLEAN DEFAULT false,
    recording_enabled BOOLEAN DEFAULT false,
    ai_analysis_enabled BOOLEAN DEFAULT false,
    live_proctor_enabled BOOLEAN DEFAULT false,
    biometric_verified BOOLEAN DEFAULT false,
    session_token VARCHAR(500) UNIQUE,
    started_at TIMESTAMP WITH TIME ZONE,
    ended_at TIMESTAMP WITH TIME ZONE,
    expires_at TIMESTAMP WITH TIME ZONE,
    violation_count INTEGER DEFAULT 0,
    critical_violations INTEGER DEFAULT 0,
    evidence_urls TEXT[],
    notes TEXT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_proctoring_assessment ON proctoring_sessions(assessment_id);
CREATE INDEX idx_proctoring_user ON proctoring_sessions(user_id);
CREATE INDEX idx_proctoring_status ON proctoring_sessions(status);
CREATE INDEX idx_proctoring_tier ON proctoring_sessions(tier);

-- Proctoring Violations
CREATE TABLE IF NOT EXISTS proctoring_violations (
    id BIGSERIAL PRIMARY KEY,
    session_id BIGINT NOT NULL REFERENCES proctoring_sessions(id) ON DELETE CASCADE,
    violation_type VARCHAR(100) NOT NULL, -- tab_switch, copy_paste, face_not_detected, multiple_faces, voice_detected, phone_detected, person_left, unauthorized_object
    severity VARCHAR(50) NOT NULL, -- low, medium, high, critical
    description TEXT,
    timestamp TIMESTAMP WITH TIME ZONE NOT NULL,
    evidence_url VARCHAR(500),
    action_taken VARCHAR(100), -- warning, auto_submit, flag_for_review, terminate
    reviewed BOOLEAN DEFAULT false,
    reviewed_by BIGINT REFERENCES users(id),
    review_notes TEXT,
    reviewed_at TIMESTAMP WITH TIME ZONE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_violations_session ON proctoring_violations(session_id);
CREATE INDEX idx_violations_type ON proctoring_violations(violation_type);
CREATE INDEX idx_violations_severity ON proctoring_violations(severity);
CREATE INDEX idx_violations_reviewed ON proctoring_violations(reviewed);

-- Accessibility Audits
CREATE TABLE IF NOT EXISTS accessibility_audits (
    id BIGSERIAL PRIMARY KEY,
    page_url VARCHAR(500) NOT NULL,
    page_title VARCHAR(500),
    wcag_level VARCHAR(50) DEFAULT 'AA', -- A, AA, AAA
    component_type VARCHAR(50), -- page, component, workflow
    component_name VARCHAR(255),
    compliance_score INTEGER DEFAULT 0,
    issues_found INTEGER DEFAULT 0,
    issues_pass INTEGER DEFAULT 0,
    issues_warning INTEGER DEFAULT 0,
    issues_fail INTEGER DEFAULT 0,
    audit_data JSONB,
    recommendations TEXT[],
    audited_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    audited_by BIGINT REFERENCES users(id),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_audits_url ON accessibility_audits(page_url);
CREATE INDEX idx_audits_level ON accessibility_audits(wcag_level);
CREATE INDEX idx_audits_score ON accessibility_audits(compliance_score);

-- Accessibility Issues
CREATE TABLE IF NOT EXISTS accessibility_issues (
    id BIGSERIAL PRIMARY KEY,
    audit_id BIGINT NOT NULL REFERENCES accessibility_audits(id) ON DELETE CASCADE,
    issue_type VARCHAR(100) NOT NULL, -- color_contrast, missing_alt, keyboard_trap, focus_order, aria_label, heading_structure, form_label, link_text
    severity VARCHAR(50) NOT NULL, -- critical, serious, moderate, minor
    wcag_criterion VARCHAR(100),
    element_selector TEXT,
    element_html TEXT,
    description TEXT NOT NULL,
    recommendation TEXT,
    code_snippet TEXT,
    screenshot_url VARCHAR(500),
    status VARCHAR(50) DEFAULT 'open', -- open, in_progress, resolved, wont_fix
    resolved_at TIMESTAMP WITH TIME ZONE,
    resolved_by BIGINT REFERENCES users(id)
);

CREATE INDEX idx_issues_audit ON accessibility_issues(audit_id);
CREATE INDEX idx_issues_type ON accessibility_issues(issue_type);
CREATE INDEX idx_issues_severity ON accessibility_issues(severity);
CREATE INDEX idx_issues_status ON accessibility_issues(status);

-- USSD Sessions
CREATE TABLE IF NOT EXISTS ussd_sessions (
    id BIGSERIAL PRIMARY KEY,
    session_id VARCHAR(100) UNIQUE NOT NULL, -- External USSD gateway session ID
    phone_number VARCHAR(50) NOT NULL,
    user_id BIGINT REFERENCES users(id) ON DELETE SET NULL,
    current_menu VARCHAR(100),
    menu_history TEXT[],
    last_input TEXT,
    state_data JSONB,
    status VARCHAR(50) DEFAULT 'active', -- active, completed, expired, cancelled
    expires_at TIMESTAMP WITH TIME ZONE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_ussd_session ON ussd_sessions(session_id);
CREATE INDEX idx_ussd_phone ON ussd_sessions(phone_number);
CREATE INDEX idx_ussd_user ON ussd_sessions(user_id);
CREATE INDEX idx_ussd_status ON ussd_sessions(status);

-- USSD Logs
CREATE TABLE IF NOT EXISTS ussd_logs (
    id BIGSERIAL PRIMARY KEY,
    session_id BIGINT NOT NULL REFERENCES ussd_sessions(id) ON DELETE CASCADE,
    menu_displayed VARCHAR(100),
    user_input TEXT,
    response_sent TEXT,
    action_taken VARCHAR(100),
    processing_time_ms INTEGER,
    error_message TEXT,
    logged_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_ussd_logs_session ON ussd_logs(session_id);
CREATE INDEX idx_ussd_logs_time ON ussd_logs(logged_at);

-- Sync Devices (Offline-First)
CREATE TABLE IF NOT EXISTS sync_devices (
    id BIGSERIAL PRIMARY KEY,
    device_id VARCHAR(255) UNIQUE NOT NULL,
    user_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    device_name VARCHAR(255),
    device_type VARCHAR(50), -- mobile, tablet, desktop
    platform VARCHAR(50), -- ios, android, windows, macos, linux
    app_version VARCHAR(50),
    last_sync_at TIMESTAMP WITH TIME ZONE,
    pending_changes_count INTEGER DEFAULT 0,
    sync_priority VARCHAR(50) DEFAULT 'low', -- low, medium, high, critical
    status VARCHAR(50) DEFAULT 'active', -- active, inactive, blocked
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_sync_devices_device ON sync_devices(device_id);
CREATE INDEX idx_sync_devices_user ON sync_devices(user_id);
CREATE INDEX idx_sync_devices_priority ON sync_devices(sync_priority);

-- Sync Conflicts
CREATE TABLE IF NOT EXISTS sync_conflicts (
    id BIGSERIAL PRIMARY KEY,
    device_id BIGINT NOT NULL REFERENCES sync_devices(id) ON DELETE CASCADE,
    entity_type VARCHAR(100) NOT NULL, -- note, submission, quiz_answer, etc.
    entity_id BIGINT NOT NULL,
    conflict_type VARCHAR(50) NOT NULL, -- version_mismatch, deleted_vs_modified, dual_modification
    local_version JSONB NOT NULL,
    remote_version JSONB NOT NULL,
    local_timestamp TIMESTAMP WITH TIME ZONE NOT NULL,
    remote_timestamp TIMESTAMP WITH TIME ZONE NOT NULL,
    resolution_strategy VARCHAR(50), -- use_local, use_remote, merge, manual
    resolved_value JSONB,
    status VARCHAR(50) DEFAULT 'unresolved', -- unresolved, auto_resolved, manually_resolved
    resolved_at TIMESTAMP WITH TIME ZONE,
    resolved_by BIGINT REFERENCES users(id),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_conflicts_device ON sync_conflicts(device_id);
CREATE INDEX idx_conflicts_entity ON sync_conflicts(entity_type, entity_id);
CREATE INDEX idx_conflicts_status ON sync_conflicts(status);

-- Deployment Configurations
CREATE TABLE IF NOT EXISTS deployment_configs (
    id BIGSERIAL PRIMARY KEY,
    institution_id BIGINT REFERENCES institutions(id) ON DELETE SET NULL,
    deployment_type VARCHAR(50) NOT NULL, -- docker_compose, systemd, standalone
    config_name VARCHAR(255) NOT NULL,
    configuration JSONB NOT NULL,
    environment_variables JSONB,
    ssl_enabled BOOLEAN DEFAULT true,
    backup_enabled BOOLEAN DEFAULT true,
    monitoring_enabled BOOLEAN DEFAULT true,
    version VARCHAR(50),
    deployed_at TIMESTAMP WITH TIME ZONE,
    deployed_by BIGINT REFERENCES users(id),
    status VARCHAR(50) DEFAULT 'generated', -- generated, deployed, updated, deprecated
    notes TEXT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_deployments_institution ON deployment_configs(institution_id);
CREATE INDEX idx_deployments_type ON deployment_configs(deployment_type);
CREATE INDEX idx_deployments_status ON deployment_configs(status);

-- ============================================================
-- INSERT DEFAULT DATA
-- ============================================================

-- Default Partnership Types
INSERT INTO industry_partnerships (partner_company_name, partnership_type, description, status)
VALUES 
    ('Default Tech Partner', 'recruitment', 'Sample technology recruitment partner', 'active'),
    ('Default Curriculum Partner', 'curriculum', 'Sample curriculum development partner', 'active')
ON CONFLICT DO NOTHING;

-- Default Collection Types
INSERT INTO library_collections (name, collection_type, visibility, description)
VALUES 
    ('General Collection', 'institutional', 'public', 'Main library collection'),
    ('Course Reserves', 'course_reserves', 'restricted', 'Required readings for courses'),
    ('Special Collections', 'special', 'public', 'Rare and unique materials'),
    ('Digital Resources', 'thematic', 'public', 'Online digital resources'),
    ('Open Educational Resources', 'thematic', 'public', 'Free OER materials')
ON CONFLICT DO NOTHING;

-- ============================================================
-- END OF MIGRATION
-- ============================================================
