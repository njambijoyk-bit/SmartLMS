// Models for Module 18 (Parents Portal), Module 23 (ID Cards), Module 24 (Alumni Portal)
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// ============================================
// MODULE 18: PARENTS PORTAL MODELS
// ============================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParentStudentLink {
    pub id: Uuid,
    pub parent_id: Uuid,
    pub student_id: Uuid,
    pub linkage_type: String, // admin_csv, self_service, registration
    pub status: String, // pending, active, revoked
    pub student_approved: bool,
    pub approved_at: Option<chrono::DateTime<chrono::Utc>>,
    pub revoked_at: Option<chrono::DateTime<chrono::Utc>>,
    pub revoked_by: Option<Uuid>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParentVisibilitySettings {
    pub id: Uuid,
    pub link_id: Uuid,
    pub enrolled_courses: bool,
    pub grades_and_results: bool,
    pub attendance_records: bool,
    pub exam_timetable: bool,
    pub fee_balance: bool,
    pub disciplinary_records: bool,
    pub coursework_submissions: bool,
    pub direct_messaging: bool,
    pub hidden_courses: Vec<Uuid>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParentNotificationPreferences {
    pub id: Uuid,
    pub link_id: Uuid,
    pub critical_notifications: bool,
    pub important_notifications: bool,
    pub optional_notifications: bool,
    pub email_enabled: bool,
    pub sms_enabled: bool,
    pub push_enabled: bool,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParentFeePayment {
    pub id: Uuid,
    pub parent_id: Uuid,
    pub student_id: Uuid,
    pub amount: rust_decimal::Decimal,
    pub currency: String,
    pub payment_method: String, // mpesa, stripe, bank_transfer
    pub payment_reference: Option<String>,
    pub mpesa_receipt_number: Option<String>,
    pub stripe_charge_id: Option<String>,
    pub status: String, // pending, completed, failed, refunded
    pub receipt_url: Option<String>,
    pub processed_at: Option<chrono::DateTime<chrono::Utc>>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub metadata: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParentAccessLog {
    pub id: Uuid,
    pub parent_id: Uuid,
    pub student_id: Uuid,
    pub action: String,
    pub resource_type: Option<String>,
    pub resource_id: Option<Uuid>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

// Request/Response types for Parents Portal
#[derive(Debug, Deserialize)]
pub struct CreateParentLinkRequest {
    pub student_email: String,
    pub linkage_type: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ApproveParentLinkRequest {
    pub link_id: Uuid,
    pub approved: bool,
}

#[derive(Debug, Deserialize)]
pub struct UpdateVisibilitySettingsRequest {
    pub enrolled_courses: Option<bool>,
    pub grades_and_results: Option<bool>,
    pub attendance_records: Option<bool>,
    pub exam_timetable: Option<bool>,
    pub fee_balance: Option<bool>,
    pub disciplinary_records: Option<bool>,
    pub coursework_submissions: Option<bool>,
    pub direct_messaging: Option<bool>,
    pub hidden_courses: Option<Vec<Uuid>>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateNotificationPreferencesRequest {
    pub critical_notifications: Option<bool>,
    pub important_notifications: Option<bool>,
    pub optional_notifications: Option<bool>,
    pub email_enabled: Option<bool>,
    pub sms_enabled: Option<bool>,
    pub push_enabled: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct MakeFeePaymentRequest {
    pub student_id: Uuid,
    pub amount: rust_decimal::Decimal,
    pub payment_method: String,
    pub mpesa_phone: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ParentDashboardResponse {
    pub links: Vec<ParentStudentLink>,
    pub students: Vec<StudentSummary>,
}

#[derive(Debug, Serialize)]
pub struct StudentSummary {
    pub id: Uuid,
    pub name: String,
    pub reg_number: String,
    pub programme: String,
    pub year_of_study: i32,
    pub gpa: Option<rust_decimal::Decimal>,
    pub status: String,
}

// ============================================
// MODULE 23: STUDENT & ALUMNI ID CARDS MODELS
// ============================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StudentIdCard {
    pub id: Uuid,
    pub user_id: Uuid,
    pub card_number: String,
    pub card_type: String, // student, alumni, former_student
    pub status: String, // active, expired, suspended, alumni, pending
    pub qr_code_hash: String,
    pub photo_url: Option<String>,
    pub issued_date: chrono::NaiveDate,
    pub expiry_date: Option<chrono::NaiveDate>,
    pub last_verified_at: Option<chrono::DateTime<chrono::Utc>>,
    pub verification_count: i32,
    pub printed: bool,
    pub print_batch_id: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdCardVerification {
    pub id: Uuid,
    pub card_id: Uuid,
    pub verified_by: Option<Uuid>,
    pub verification_method: String,
    pub verification_result: String,
    pub verification_context: Option<String>,
    pub ip_address: Option<String>,
    pub notes: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardTransition {
    pub id: Uuid,
    pub card_id: Uuid,
    pub from_status: String,
    pub to_status: String,
    pub transition_reason: String,
    pub triggered_by: Option<Uuid>,
    pub effective_date: chrono::NaiveDate,
    pub notes: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

// Request/Response types for ID Cards
#[derive(Debug, Deserialize)]
pub struct IssueIdCardRequest {
    pub user_id: Uuid,
    pub card_type: Option<String>,
    pub expiry_date: Option<chrono::NaiveDate>,
}

#[derive(Debug, Deserialize)]
pub struct VerifyIdCardRequest {
    pub qr_code: String,
    pub verification_context: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateCardStatusRequest {
    pub status: String,
    pub reason: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct IdCardVerificationResponse {
    pub valid: bool,
    pub card: Option<StudentIdCard>,
    pub student_name: Option<String>,
    pub programme: Option<String>,
    pub message: String,
}

// ============================================
// MODULE 24: ALUMNI PORTAL MODELS
// ============================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlumniProfile {
    pub id: Uuid,
    pub user_id: Uuid,
    pub graduation_year: i32,
    pub programme: String,
    pub degree_type: Option<String>,
    pub final_gpa: Option<rust_decimal::Decimal>,
    pub honours: Option<String>,
    pub current_company: Option<String>,
    pub current_role: Option<String>,
    pub industry: Option<String>,
    pub location_city: Option<String>,
    pub location_country: Option<String>,
    pub linkedin_url: Option<String>,
    pub bio: Option<String>,
    pub skills: Vec<String>,
    pub willing_to_mentor: bool,
    pub available_for_networking: bool,
    pub profile_visibility: String,
    pub transcript_downloaded_count: i32,
    pub last_transcript_download: Option<chrono::DateTime<chrono::Utc>>,
    pub employment_updated_at: Option<chrono::DateTime<chrono::Utc>>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlumniConnection {
    pub id: Uuid,
    pub alumni_id_1: Uuid,
    pub alumni_id_2: Uuid,
    pub connection_type: String,
    pub introduced_by: Option<Uuid>,
    pub status: String,
    pub message: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlumniJob {
    pub id: Uuid,
    pub institution_id: Option<Uuid>,
    pub employer_id: Option<Uuid>,
    pub title: String,
    pub company: String,
    pub description: String,
    pub requirements: Vec<String>,
    pub location: Option<String>,
    pub remote_option: bool,
    pub job_type: Option<String>,
    pub salary_min: Option<rust_decimal::Decimal>,
    pub salary_max: Option<rust_decimal::Decimal>,
    pub salary_currency: String,
    pub application_deadline: Option<chrono::NaiveDate>,
    pub status: String,
    pub application_url: Option<String>,
    pub application_email: Option<String>,
    pub views_count: i32,
    pub applications_count: i32,
    pub featured: bool,
    pub featured_until: Option<chrono::DateTime<chrono::Utc>>,
    pub created_by: Uuid,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlumniJobApplication {
    pub id: Uuid,
    pub job_id: Uuid,
    pub applicant_id: Uuid,
    pub cover_letter: Option<String>,
    pub resume_url: Option<String>,
    pub status: String,
    pub applied_at: chrono::DateTime<chrono::Utc>,
    pub reviewed_at: Option<chrono::DateTime<chrono::Utc>>,
    pub reviewed_by: Option<Uuid>,
    pub review_notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlumniCpdEnrollment {
    pub id: Uuid,
    pub alumni_id: Uuid,
    pub course_id: Uuid,
    pub enrollment_type: String,
    pub payment_status: String,
    pub amount_paid: Option<rust_decimal::Decimal>,
    pub certificate_issued: bool,
    pub certificate_id: Option<Uuid>,
    pub started_at: chrono::DateTime<chrono::Utc>,
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,
    pub progress_percent: rust_decimal::Decimal,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlumniDonation {
    pub id: Uuid,
    pub alumni_id: Uuid,
    pub amount: rust_decimal::Decimal,
    pub currency: String,
    pub donation_type: String,
    pub fund_designation: Option<String>,
    pub payment_method: Option<String>,
    pub payment_reference: Option<String>,
    pub status: String,
    pub receipt_url: Option<String>,
    pub anonymous: bool,
    pub message: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub processed_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraduateOutcome {
    pub id: Uuid,
    pub alumni_id: Uuid,
    pub survey_year: i32,
    pub employed: Option<bool>,
    pub employment_status: Option<String>,
    pub company_name: Option<String>,
    pub job_title: Option<String>,
    pub salary_range: Option<String>,
    pub further_study_programme: Option<String>,
    pub further_study_institution: Option<String>,
    pub skills_utilized: Vec<String>,
    pub satisfaction_score: Option<i32>,
    pub would_recommend: Option<bool>,
    pub comments: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

// Request/Response types for Alumni Portal
#[derive(Debug, Deserialize)]
pub struct CreateAlumniProfileRequest {
    pub graduation_year: i32,
    pub programme: String,
    pub degree_type: Option<String>,
    pub final_gpa: Option<rust_decimal::Decimal>,
    pub honours: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateAlumniProfileRequest {
    pub current_company: Option<String>,
    pub current_role: Option<String>,
    pub industry: Option<String>,
    pub location_city: Option<String>,
    pub location_country: Option<String>,
    pub linkedin_url: Option<String>,
    pub bio: Option<String>,
    pub skills: Option<Vec<String>>,
    pub willing_to_mentor: Option<bool>,
    pub available_for_networking: Option<bool>,
    pub profile_visibility: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateAlumniJobRequest {
    pub title: String,
    pub company: String,
    pub description: String,
    pub requirements: Option<Vec<String>>,
    pub location: Option<String>,
    pub remote_option: Option<bool>,
    pub job_type: Option<String>,
    pub salary_min: Option<rust_decimal::Decimal>,
    pub salary_max: Option<rust_decimal::Decimal>,
    pub salary_currency: Option<String>,
    pub application_deadline: Option<chrono::NaiveDate>,
    pub application_url: Option<String>,
    pub application_email: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ApplyToJobRequest {
    pub job_id: Uuid,
    pub cover_letter: Option<String>,
    pub resume_url: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ConnectAlumniRequest {
    pub alumni_id: Uuid,
    pub connection_type: Option<String>,
    pub message: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct MakeDonationRequest {
    pub amount: rust_decimal::Decimal,
    pub donation_type: Option<String>,
    pub fund_designation: Option<String>,
    pub payment_method: String,
    pub anonymous: Option<bool>,
    pub message: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct AlumniDashboardResponse {
    pub profile: AlumniProfile,
    pub stats: AlumniStats,
    pub recent_jobs: Vec<AlumniJob>,
    pub suggested_connections: Vec<AlumniProfile>,
}

#[derive(Debug, Serialize)]
pub struct AlumniStats {
    pub total_alumni: i64,
    pub alumni_in_network: i64,
    pub jobs_posted: i64,
    pub cpd_courses_available: i64,
    pub mentorship_connections: i64,
}
