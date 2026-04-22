-- Phase 16 Enhancements: VPAT, LMS Migration, SOC 2 Compliance

-- ============================================================================
-- VPAT (Voluntary Product Accessibility Template) Tables
-- ============================================================================

CREATE TABLE vpat_reports (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    product_name VARCHAR(255) NOT NULL,
    product_version VARCHAR(50) NOT NULL,
    report_date DATE NOT NULL DEFAULT CURRENT_DATE,
    vendor_name VARCHAR(255) NOT NULL,
    vendor_contact VARCHAR(255),
    wcag_level VARCHAR(10) NOT NULL DEFAULT 'AA',
    section_508_compliant BOOLEAN DEFAULT false,
    en_301_549_compliant BOOLEAN DEFAULT false,
    overall_compliance_score DECIMAL(5,2) DEFAULT 0.0,
    total_criteria INTEGER DEFAULT 0,
    passed_criteria INTEGER DEFAULT 0,
    partially_met_criteria INTEGER DEFAULT 0,
    not_met_criteria INTEGER DEFAULT 0,
    not_applicable_criteria INTEGER DEFAULT 0,
    remarks TEXT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE TABLE vpat_criteria (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    report_id UUID NOT NULL REFERENCES vpat_reports(id) ON DELETE CASCADE,
    criterion_number VARCHAR(50) NOT NULL,
    criterion_name VARCHAR(500) NOT NULL,
    wcag_criterion VARCHAR(50),
    conformance_level VARCHAR(10),
    conformance_status VARCHAR(50) NOT NULL, -- Supports, Partially Supports, Does Not Support, Not Applicable
    user_notes TEXT,
    evidence_url TEXT,
    remediation_plan TEXT,
    target_remediation_date DATE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE INDEX idx_vpat_criteria_report_id ON vpat_criteria(report_id);
CREATE INDEX idx_vpat_reports_date ON vpat_reports(report_date);

-- ============================================================================
-- LMS Migration Tables (Moodle, Canvas, etc.)
-- ============================================================================

CREATE TABLE lms_migrations (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    source_lms VARCHAR(50) NOT NULL, -- moodle, canvas, blackboard, etc.
    source_version VARCHAR(50),
    target_institution_id UUID NOT NULL,
    migration_status VARCHAR(50) NOT NULL DEFAULT 'pending', -- pending, running, completed, failed, partial
    total_courses INTEGER DEFAULT 0,
    migrated_courses INTEGER DEFAULT 0,
    total_users INTEGER DEFAULT 0,
    migrated_users INTEGER DEFAULT 0,
    total_quizzes INTEGER DEFAULT 0,
    migrated_quizzes INTEGER DEFAULT 0,
    total_assignments INTEGER DEFAULT 0,
    migrated_assignments INTEGER DEFAULT 0,
    error_log JSONB DEFAULT '[]'::jsonb,
    started_at TIMESTAMP WITH TIME ZONE,
    completed_at TIMESTAMP WITH TIME ZONE,
    created_by UUID NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE TABLE lms_migration_courses (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    migration_id UUID NOT NULL REFERENCES lms_migrations(id) ON DELETE CASCADE,
    source_course_id VARCHAR(255) NOT NULL,
    source_course_name VARCHAR(500) NOT NULL,
    target_course_id UUID,
    migration_status VARCHAR(50) NOT NULL DEFAULT 'pending',
    error_message TEXT,
    metadata JSONB DEFAULT '{}'::jsonb,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE INDEX idx_lms_migration_courses_migration_id ON lms_migration_courses(migration_id);
CREATE INDEX idx_lms_migrations_status ON lms_migrations(migration_status);

CREATE TABLE lms_migration_users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    migration_id UUID NOT NULL REFERENCES lms_migrations(id) ON DELETE CASCADE,
    source_user_id VARCHAR(255) NOT NULL,
    source_username VARCHAR(255) NOT NULL,
    source_email VARCHAR(255),
    target_user_id UUID,
    migration_status VARCHAR(50) NOT NULL DEFAULT 'pending',
    error_message TEXT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE INDEX idx_lms_migration_users_migration_id ON lms_migration_users(migration_id);

-- QTI Package storage
CREATE TABLE qti_packages (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    package_name VARCHAR(255) NOT NULL,
    qti_version VARCHAR(20) NOT NULL, -- 1.2, 2.1, 2.2
    file_path VARCHAR(500) NOT NULL,
    file_size_bytes BIGINT,
    total_questions INTEGER DEFAULT 0,
    question_types JSONB DEFAULT '[]'::jsonb,
    parsed_content JSONB,
    validation_errors JSONB DEFAULT '[]'::jsonb,
    is_valid BOOLEAN DEFAULT false,
    uploaded_by UUID NOT NULL,
    uploaded_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE INDEX idx_qti_packages_version ON qti_packages(qti_version);

-- ============================================================================
-- SOC 2 Compliance Tracking Tables
-- ============================================================================

CREATE TABLE soc2_controls (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    control_id VARCHAR(50) UNIQUE NOT NULL,
    control_name VARCHAR(500) NOT NULL,
    trust_service_category VARCHAR(50) NOT NULL, -- Security, Availability, Processing Integrity, Confidentiality, Privacy
    control_type VARCHAR(50) NOT NULL, -- Design, Operating Effectiveness
    description TEXT NOT NULL,
    implementation_guidance TEXT,
    common_criteria BOOLEAN DEFAULT false,
    point_of_focus JSONB DEFAULT '[]'::jsonb,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE TABLE soc2_assessments (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    assessment_name VARCHAR(255) NOT NULL,
    assessment_period_start DATE NOT NULL,
    assessment_period_end DATE NOT NULL,
    auditor_name VARCHAR(255),
    auditor_firm VARCHAR(255),
    assessment_type VARCHAR(50) NOT NULL, -- Type I, Type II
    overall_status VARCHAR(50) NOT NULL DEFAULT 'in_progress',
    findings_count INTEGER DEFAULT 0,
    deficiencies_count INTEGER DEFAULT 0,
    exceptions_count INTEGER DEFAULT 0,
    report_url TEXT,
    certification_expiry DATE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE TABLE soc2_control_tests (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    assessment_id UUID NOT NULL REFERENCES soc2_assessments(id) ON DELETE CASCADE,
    control_id UUID NOT NULL REFERENCES soc2_controls(id),
    test_date DATE NOT NULL,
    tested_by UUID NOT NULL,
    test_methodology VARCHAR(500),
    sample_size INTEGER,
    test_result VARCHAR(50) NOT NULL, -- Pass, Fail, Not Tested
    evidence_collected JSONB DEFAULT '[]'::jsonb,
    notes TEXT,
    deficiency_severity VARCHAR(50), -- Minor, Moderate, Major, Material Weakness
    remediation_required BOOLEAN DEFAULT false,
    remediation_due_date DATE,
    remediation_completed BOOLEAN DEFAULT false,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE INDEX idx_soc2_control_tests_assessment ON soc2_control_tests(assessment_id);
CREATE INDEX idx_soc2_control_tests_control ON soc2_control_tests(control_id);

CREATE TABLE soc2_audit_trails (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    event_type VARCHAR(100) NOT NULL,
    event_category VARCHAR(100) NOT NULL, -- Access, Change, System, Security
    user_id UUID,
    user_email VARCHAR(255),
    action VARCHAR(255) NOT NULL,
    resource_type VARCHAR(100),
    resource_id UUID,
    old_value JSONB,
    new_value JSONB,
    ip_address INET,
    user_agent TEXT,
    timestamp TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    risk_level VARCHAR(20) DEFAULT 'low' -- low, medium, high, critical
);

CREATE INDEX idx_soc2_audit_trails_timestamp ON soc2_audit_trails(timestamp);
CREATE INDEX idx_soc2_audit_trails_event_type ON soc2_audit_trails(event_type);
CREATE INDEX idx_soc2_audit_trails_user_id ON soc2_audit_trails(user_id);
CREATE INDEX idx_soc2_audit_trails_risk_level ON soc2_audit_trails(risk_level);

CREATE TABLE soc2_risk_assessments (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    assessment_name VARCHAR(255) NOT NULL,
    risk_owner UUID NOT NULL,
    assessment_date DATE NOT NULL,
    next_review_date DATE,
    overall_risk_level VARCHAR(20) NOT NULL, -- low, medium, high, critical
    total_risks INTEGER DEFAULT 0,
    high_risks INTEGER DEFAULT 0,
    medium_risks INTEGER DEFAULT 0,
    low_risks INTEGER DEFAULT 0,
    mitigation_strategies JSONB,
    status VARCHAR(50) NOT NULL DEFAULT 'active',
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE TABLE soc2_risks (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    risk_assessment_id UUID NOT NULL REFERENCES soc2_risk_assessments(id) ON DELETE CASCADE,
    risk_description TEXT NOT NULL,
    risk_category VARCHAR(100) NOT NULL,
    inherent_likelihood VARCHAR(20) NOT NULL, -- rare, unlikely, possible, likely, almost certain
    inherent_impact VARCHAR(20) NOT NULL, -- insignificant, minor, moderate, major, severe
    inherent_risk_level VARCHAR(20) NOT NULL,
    control_description TEXT,
    residual_likelihood VARCHAR(20),
    residual_impact VARCHAR(20),
    residual_risk_level VARCHAR(20),
    mitigation_actions JSONB DEFAULT '[]'::jsonb,
    risk_owner UUID,
    treatment_plan VARCHAR(500), -- Accept, Avoid, Mitigate, Transfer
    status VARCHAR(50) NOT NULL DEFAULT 'identified',
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE INDEX idx_soc2_risks_assessment ON soc2_risks(risk_assessment_id);

-- Insert default SOC 2 controls (Security category - Common Criteria)
INSERT INTO soc2_controls (control_id, control_name, trust_service_category, control_type, description, common_criteria) VALUES
('CC1.1', 'COSO Principle 1: The entity demonstrates a commitment to integrity and ethical values.', 'Security', 'Design', 'Sets tone at the top regarding integrity and ethical values.', true),
('CC1.2', 'COSO Principle 2: Board of Directors oversight responsibility.', 'Security', 'Design', 'Board exercises oversight responsibility for internal control system.', true),
('CC1.3', 'COSO Principle 3: Organizational structure and reporting lines.', 'Security', 'Design', 'Establishes structure, authority, and responsibility.', true),
('CC2.1', 'COSO Principle 4: Commitment to competence.', 'Security', 'Design', 'Maintains commitment to competent personnel.', true),
('CC2.2', 'COSO Principle 5: Accountability for internal control responsibilities.', 'Security', 'Design', 'Enforces accountability through policies and procedures.', true),
('CC3.1', 'COSO Principle 6: Specified objectives support mission.', 'Security', 'Design', 'Defines objectives to identify and manage risks.', true),
('CC3.2', 'COSO Principle 7: Risk identification and analysis.', 'Security', 'Design', 'Identifies and analyzes risks to achievement of objectives.', true),
('CC3.3', 'COSO Principle 8: Fraud risk consideration.', 'Security', 'Design', 'Considers potential for fraud in risk assessment.', true),
('CC3.4', 'COSO Principle 9: Significant change identification.', 'Security', 'Design', 'Identifies and analyzes significant changes.', true),
('CC4.1', 'COSO Principle 10: Selection and development of control activities.', 'Security', 'Design', 'Selects and develops control activities to mitigate risks.', true),
('CC4.2', 'COSO Principle 11: General controls over technology.', 'Security', 'Design', 'Implements general controls over technology infrastructure.', true),
('CC5.1', 'COSO Principle 12: Control activity execution through policies.', 'Security', 'Operating Effectiveness', 'Control activities executed through established policies.', true),
('CC5.2', 'COSO Principle 13: Technological controls deployment.', 'Security', 'Operating Effectiveness', 'Deploys technological controls to achieve objectives.', true),
('CC5.3', 'COSO Principle 14: Segregation of duties.', 'Security', 'Operating Effectiveness', 'Segregates duties to reduce risk of errors or fraud.', true),
('CC6.1', 'Logical and physical access controls.', 'Security', 'Design', 'Implements logical and physical access security controls.', true),
('CC6.2', 'Prior to registration, identity verified.', 'Security', 'Operating Effectiveness', 'Verifies identity before granting system access.', true),
('CC6.3', 'Role-based access authorization.', 'Security', 'Operating Effectiveness', 'Authorizes access based on roles and responsibilities.', true),
('CC6.4', 'Access restriction enforcement.', 'Security', 'Operating Effectiveness', 'Restricts access to authorized personnel only.', true),
('CC6.5', 'Access removal when appropriate.', 'Security', 'Operating Effectiveness', 'Removes access when no longer needed.', true),
('CC6.6', 'Authentication mechanisms.', 'Security', 'Operating Effectiveness', 'Implements authentication mechanisms to verify users.', true),
('CC6.7', 'Transmission encryption.', 'Security', 'Operating Effectiveness', 'Encrypts data during transmission.', true),
('CC6.8', 'Storage encryption.', 'Security', 'Operating Effectiveness', 'Encrypts data at rest.', true),
('CC7.1', 'Intrusion detection and monitoring.', 'Security', 'Design', 'Detects and monitors unauthorized activities.', true),
('CC7.2', 'Security incident identification.', 'Security', 'Operating Effectiveness', 'Identifies security incidents promptly.', true),
('CC7.3', 'Incident response procedures.', 'Security', 'Design', 'Establishes incident response and recovery procedures.', true),
('CC7.4', 'Incident recovery testing.', 'Security', 'Operating Effectiveness', 'Tests recovery procedures regularly.', true),
('CC7.5', 'Incident communication.', 'Security', 'Operating Effectiveness', 'Communicates incidents to relevant parties.', true),
('CC8.1', 'Change management authorization.', 'Security', 'Design', 'Authorizes infrastructure and software changes.', true),
('CC8.2', 'Change testing and approval.', 'Security', 'Operating Effectiveness', 'Tests and approves changes before deployment.', true),
('CC9.1', 'Risk mitigation of third-party providers.', 'Security', 'Design', 'Mitigates risks from third-party service providers.', true),
('CC9.2', 'Third-party service provider agreements.', 'Security', 'Operating Effectiveness', 'Establishes agreements with third-party providers.', true);

