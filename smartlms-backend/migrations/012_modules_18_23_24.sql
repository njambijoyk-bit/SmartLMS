-- Module 18, 23, 24 Migration
-- Parents Portal, Student & Alumni ID Cards, Alumni Portal

-- Enable UUID extension if not already enabled
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- ============================================
-- MODULE 18: PARENTS PORTAL
-- ============================================

-- Parent-Student linkages table
CREATE TABLE IF NOT EXISTS parent_student_links (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    parent_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    student_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    linkage_type VARCHAR(50) NOT NULL DEFAULT 'self_service', -- admin_csv, self_service, registration
    status VARCHAR(50) NOT NULL DEFAULT 'pending', -- pending, active, revoked
    student_approved BOOLEAN DEFAULT false,
    approved_at TIMESTAMPTZ,
    revoked_at TIMESTAMPTZ,
    revoked_by UUID REFERENCES users(id),
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(parent_id, student_id)
);

-- Parent visibility settings per student
CREATE TABLE IF NOT EXISTS parent_visibility_settings (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    link_id UUID NOT NULL REFERENCES parent_student_links(id) ON DELETE CASCADE,
    enrolled_courses BOOLEAN DEFAULT true,
    grades_and_results BOOLEAN DEFAULT true,
    attendance_records BOOLEAN DEFAULT true,
    exam_timetable BOOLEAN DEFAULT true,
    fee_balance BOOLEAN DEFAULT true,
    disciplinary_records BOOLEAN DEFAULT false,
    coursework_submissions BOOLEAN DEFAULT false,
    direct_messaging BOOLEAN DEFAULT false,
    hidden_courses JSONB DEFAULT '[]', -- Array of course IDs hidden from parent
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(link_id)
);

-- Parent notification preferences
CREATE TABLE IF NOT EXISTS parent_notification_preferences (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    link_id UUID NOT NULL REFERENCES parent_student_links(id) ON DELETE CASCADE,
    critical_notifications BOOLEAN DEFAULT true, -- Always on: exam card, fee payment, emergency
    important_notifications BOOLEAN DEFAULT true, -- Results, attendance warning, missed deadline
    optional_notifications BOOLEAN DEFAULT false, -- Every grade posted, every attendance
    email_enabled BOOLEAN DEFAULT true,
    sms_enabled BOOLEAN DEFAULT false,
    push_enabled BOOLEAN DEFAULT true,
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(link_id)
);

-- Parent fee payments
CREATE TABLE IF NOT EXISTS parent_fee_payments (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    parent_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    student_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    amount DECIMAL(12,2) NOT NULL,
    currency VARCHAR(3) DEFAULT 'KES',
    payment_method VARCHAR(50) NOT NULL, -- mpesa, stripe, bank_transfer
    payment_reference VARCHAR(255),
    mpesa_receipt_number VARCHAR(100),
    stripe_charge_id VARCHAR(255),
    status VARCHAR(50) DEFAULT 'pending', -- pending, completed, failed, refunded
    receipt_url TEXT,
    processed_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    metadata JSONB DEFAULT '{}'
);

-- Parent audit log
CREATE TABLE IF NOT EXISTS parent_access_log (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    parent_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    student_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    action VARCHAR(100) NOT NULL, -- view_grades, view_attendance, view_courses, etc.
    resource_type VARCHAR(50), -- course, grade, attendance, fee, etc.
    resource_id UUID,
    ip_address INET,
    user_agent TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- ============================================
-- MODULE 23: STUDENT & ALUMNI ID CARDS
-- ============================================

-- Student ID cards table
CREATE TABLE IF NOT EXISTS student_id_cards (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    card_number VARCHAR(50) NOT NULL UNIQUE,
    card_type VARCHAR(50) NOT NULL DEFAULT 'student', -- student, alumni, former_student
    status VARCHAR(50) NOT NULL DEFAULT 'active', -- active, expired, suspended, alumni, pending
    qr_code_hash VARCHAR(255) NOT NULL, -- Hash for QR verification
    photo_url TEXT,
    issued_date DATE NOT NULL DEFAULT CURRENT_DATE,
    expiry_date DATE,
    last_verified_at TIMESTAMPTZ,
    verification_count INTEGER DEFAULT 0,
    printed BOOLEAN DEFAULT false,
    print_batch_id VARCHAR(100),
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- ID card verification log
CREATE TABLE IF NOT EXISTS id_card_verifications (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    card_id UUID NOT NULL REFERENCES student_id_cards(id) ON DELETE CASCADE,
    verified_by UUID REFERENCES users(id), -- Who verified (admin, instructor, etc.)
    verification_method VARCHAR(50) DEFAULT 'qr_scan', -- qr_scan, manual, api
    verification_result VARCHAR(50) NOT NULL, -- success, failed, expired, suspended
    verification_context VARCHAR(100), -- exam_entry, library_access, event_checkin
    ip_address INET,
    notes TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- Card transition history
CREATE TABLE IF NOT EXISTS card_transitions (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    card_id UUID NOT NULL REFERENCES student_id_cards(id) ON DELETE CASCADE,
    from_status VARCHAR(50) NOT NULL,
    to_status VARCHAR(50) NOT NULL,
    transition_reason VARCHAR(255), -- graduation, withdrawal, suspension, reactivation
    triggered_by UUID REFERENCES users(id), -- admin who triggered or system
    effective_date DATE NOT NULL DEFAULT CURRENT_DATE,
    notes TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- ============================================
-- MODULE 24: ALUMNI PORTAL
-- ============================================

-- Alumni profiles (extends user data)
CREATE TABLE IF NOT EXISTS alumni_profiles (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE UNIQUE,
    graduation_year INTEGER NOT NULL,
    programme VARCHAR(255) NOT NULL,
    degree_type VARCHAR(100), -- BSc, MSc, PhD, Diploma, Certificate
    final_gpa DECIMAL(3,2),
    honours VARCHAR(100), -- First Class, Second Class Upper, etc.
    current_company VARCHAR(255),
    current_role VARCHAR(255),
    industry VARCHAR(100),
    location_city VARCHAR(100),
    location_country VARCHAR(100),
    linkedin_url TEXT,
    bio TEXT,
    skills JSONB DEFAULT '[]',
    willing_to_mentor BOOLEAN DEFAULT false,
    available_for_networking BOOLEAN DEFAULT true,
    profile_visibility VARCHAR(50) DEFAULT 'alumni_only', -- public, alumni_only, private
    transcript_downloaded_count INTEGER DEFAULT 0,
    last_transcript_download TIMESTAMPTZ,
    employment_updated_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Alumni network connections
CREATE TABLE IF NOT EXISTS alumni_connections (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    alumni_id_1 UUID NOT NULL REFERENCES alumni_profiles(user_id) ON DELETE CASCADE,
    alumni_id_2 UUID NOT NULL REFERENCES alumni_profiles(user_id) ON DELETE CASCADE,
    connection_type VARCHAR(50) DEFAULT 'network', -- network, mentor, mentee
    introduced_by UUID REFERENCES users(id),
    status VARCHAR(50) DEFAULT 'pending', -- pending, accepted, declined
    message TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(alumni_id_1, alumni_id_2)
);

-- Alumni job board
CREATE TABLE IF NOT EXISTS alumni_jobs (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    institution_id UUID REFERENCES institutions(id) ON DELETE SET NULL,
    employer_id UUID REFERENCES users(id) ON DELETE SET NULL, -- External employer account
    title VARCHAR(255) NOT NULL,
    company VARCHAR(255) NOT NULL,
    description TEXT NOT NULL,
    requirements JSONB DEFAULT '[]',
    location VARCHAR(255),
    remote_option BOOLEAN DEFAULT false,
    job_type VARCHAR(50), -- full_time, part_time, contract, internship
    salary_min DECIMAL(12,2),
    salary_max DECIMAL(12,2),
    salary_currency VARCHAR(3) DEFAULT 'KES',
    application_deadline DATE,
    status VARCHAR(50) DEFAULT 'active', -- active, closed, draft
    application_url TEXT,
    application_email VARCHAR(255),
    views_count INTEGER DEFAULT 0,
    applications_count INTEGER DEFAULT 0,
    featured BOOLEAN DEFAULT false,
    featured_until TIMESTAMPTZ,
    created_by UUID NOT NULL REFERENCES users(id),
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Job applications
CREATE TABLE IF NOT EXISTS alumni_job_applications (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    job_id UUID NOT NULL REFERENCES alumni_jobs(id) ON DELETE CASCADE,
    applicant_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    cover_letter TEXT,
    resume_url TEXT,
    status VARCHAR(50) DEFAULT 'submitted', -- submitted, viewed, shortlisted, rejected, hired
    applied_at TIMESTAMPTZ DEFAULT NOW(),
    reviewed_at TIMESTAMPTZ,
    reviewed_by UUID REFERENCES users(id),
    review_notes TEXT,
    UNIQUE(job_id, applicant_id)
);

-- Alumni CPD courses enrollment
CREATE TABLE IF NOT EXISTS alumni_cpd_enrollments (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    alumni_id UUID NOT NULL REFERENCES alumni_profiles(user_id) ON DELETE CASCADE,
    course_id UUID NOT NULL REFERENCES courses(id) ON DELETE CASCADE,
    enrollment_type VARCHAR(50) DEFAULT 'cpd', -- cpd, professional_dev
    payment_status VARCHAR(50) DEFAULT 'pending', -- pending, paid, waived
    amount_paid DECIMAL(12,2),
    certificate_issued BOOLEAN DEFAULT false,
    certificate_id UUID REFERENCES blockchain_certificates(id),
    started_at TIMESTAMPTZ DEFAULT NOW(),
    completed_at TIMESTAMPTZ,
    progress_percent DECIMAL(5,2) DEFAULT 0,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(alumni_id, course_id)
);

-- Alumni donations
CREATE TABLE IF NOT EXISTS alumni_donations (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    alumni_id UUID NOT NULL REFERENCES alumni_profiles(user_id) ON DELETE CASCADE,
    amount DECIMAL(12,2) NOT NULL,
    currency VARCHAR(3) DEFAULT 'KES',
    donation_type VARCHAR(50) DEFAULT 'general', -- general, scholarship, infrastructure, specific_fund
    fund_designation VARCHAR(255),
    payment_method VARCHAR(50), -- mpesa, stripe, bank_transfer
    payment_reference VARCHAR(255),
    status VARCHAR(50) DEFAULT 'pending', -- pending, completed, failed
    receipt_url TEXT,
    anonymous BOOLEAN DEFAULT false,
    message TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    processed_at TIMESTAMPTZ
);

-- Graduate outcomes tracking
CREATE TABLE IF NOT EXISTS graduate_outcomes (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    alumni_id UUID NOT NULL REFERENCES alumni_profiles(user_id) ON DELETE CASCADE,
    survey_year INTEGER NOT NULL,
    employed BOOLEAN,
    employment_status VARCHAR(50), -- employed_full_time, employed_part_time, self_employed, unemployed, further_study
    company_name VARCHAR(255),
    job_title VARCHAR(255),
    salary_range VARCHAR(50),
    further_study_programme VARCHAR(255),
    further_study_institution VARCHAR(255),
    skills_utilized JSONB DEFAULT '[]',
    satisfaction_score INTEGER CHECK (satisfaction_score >= 1 AND satisfaction_score <= 5),
    would_recommend BOOLEAN,
    comments TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(alumni_id, survey_year)
);

-- Indexes for performance
CREATE INDEX idx_parent_student_links_parent ON parent_student_links(parent_id);
CREATE INDEX idx_parent_student_links_student ON parent_student_links(student_id);
CREATE INDEX idx_parent_student_links_status ON parent_student_links(status);
CREATE INDEX idx_parent_fee_payments_parent ON parent_fee_payments(parent_id);
CREATE INDEX idx_parent_fee_payments_student ON parent_fee_payments(student_id);
CREATE INDEX idx_parent_access_log_parent ON parent_access_log(parent_id);
CREATE INDEX idx_student_id_cards_user ON student_id_cards(user_id);
CREATE INDEX idx_student_id_cards_card_number ON student_id_cards(card_number);
CREATE INDEX idx_student_id_cards_status ON student_id_cards(status);
CREATE INDEX idx_id_card_verifications_card ON id_card_verifications(card_id);
CREATE INDEX idx_alumni_profiles_graduation_year ON alumni_profiles(graduation_year);
CREATE INDEX idx_alumni_profiles_programme ON alumni_profiles(programme);
CREATE INDEX idx_alumni_profiles_location ON alumni_profiles(location_country, location_city);
CREATE INDEX idx_alumni_jobs_status ON alumni_jobs(status);
CREATE INDEX idx_alumni_jobs_created_at ON alumni_jobs(created_at DESC);
CREATE INDEX idx_alumni_job_applications_job ON alumni_job_applications(job_id);
CREATE INDEX idx_alumni_cpd_enrollments_alumni ON alumni_cpd_enrollments(alumni_id);
CREATE INDEX idx_alumni_donations_alumni ON alumni_donations(alumni_id);
CREATE INDEX idx_graduate_outcomes_alumni ON graduate_outcomes(alumni_id);

-- Comments for documentation
COMMENT ON TABLE parent_student_links IS 'Links between parent and student accounts with approval workflow';
COMMENT ON TABLE parent_visibility_settings IS 'Configurable data access permissions for each parent-student link';
COMMENT ON TABLE parent_notification_preferences IS 'Tiered notification settings for parents';
COMMENT ON TABLE parent_fee_payments IS 'Fee payments made through parent portal with multiple payment methods';
COMMENT ON TABLE parent_access_log IS 'Audit trail for all parent data access';
COMMENT ON TABLE student_id_cards IS 'Digital and physical student ID cards with QR verification';
COMMENT ON TABLE id_card_verifications IS 'Log of all ID card verification attempts';
COMMENT ON TABLE card_transitions IS 'History of card status changes (student to alumni, etc.)';
COMMENT ON TABLE alumni_profiles IS 'Extended alumni profile data for networking and tracking';
COMMENT ON TABLE alumni_connections IS 'Alumni networking connections and mentorship relationships';
COMMENT ON TABLE alumni_jobs IS 'Job board for alumni and employers';
COMMENT ON TABLE alumni_cpd_enrollments IS 'Continuing professional development course enrollments for alumni';
COMMENT ON TABLE alumni_donations IS 'Donation tracking for alumni giving';
COMMENT ON TABLE graduate_outcomes IS 'Graduate outcome data for accreditation and rankings';
