// Clearance System — Multi-department student clearance before certificate/transcript release
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// A clearance department (Library, Finance, Hostel, etc.)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClearanceDepartment {
    pub id: Uuid,
    pub institution_id: Uuid,
    pub name: String,
    pub description: String,
    pub officer_role: String,
    pub is_active: bool,
    pub display_order: i32,
    pub created_at: DateTime<Utc>,
}

/// Status of a single department clearance for a student
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum DepartmentClearanceStatus {
    Pending,
    Cleared,
    Blocked,
    NotApplicable,
}

/// A single department's clearance record for a student
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DepartmentClearance {
    pub id: Uuid,
    pub student_clearance_id: Uuid,
    pub department_id: Uuid,
    pub department_name: String,
    pub status: DepartmentClearanceStatus,
    pub reason: Option<String>,
    pub cleared_by: Option<Uuid>,
    pub cleared_at: Option<DateTime<Utc>>,
    pub blocked_by: Option<Uuid>,
    pub blocked_reason: Option<String>,
    pub notes: Option<String>,
    pub updated_at: DateTime<Utc>,
}

/// Overall student clearance record
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ClearanceStatus {
    NotStarted,
    InProgress,
    FullyCleared,
    Blocked,
}

/// Full student clearance record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StudentClearance {
    pub id: Uuid,
    pub student_id: Uuid,
    pub institution_id: Uuid,
    pub academic_year: String,
    pub semester: Option<u8>,
    pub clearance_type: ClearanceType,
    pub overall_status: ClearanceStatus,
    pub department_clearances: Vec<DepartmentClearance>,
    pub initiated_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub certificate_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ClearanceType {
    Graduation,
    Transfer,
    EndOfSemester,
    WithdrawalOrDeferral,
}

/// Request to initiate clearance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InitiateClearanceRequest {
    pub student_id: Uuid,
    pub institution_id: Uuid,
    pub academic_year: String,
    pub semester: Option<u8>,
    pub clearance_type: ClearanceType,
}

/// Officer action on a department clearance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateDepartmentClearanceRequest {
    pub clearance_id: Uuid,
    pub department_id: Uuid,
    pub status: DepartmentClearanceStatus,
    pub reason: Option<String>,
    pub notes: Option<String>,
    pub officer_id: Uuid,
}

/// Student-facing clearance dashboard
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClearanceDashboard {
    pub student_id: Uuid,
    pub overall_status: ClearanceStatus,
    pub departments: Vec<DepartmentClearanceSummary>,
    pub all_cleared: bool,
    pub certificate_available: bool,
    pub certificate_url: Option<String>,
    pub percentage_complete: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DepartmentClearanceSummary {
    pub department_name: String,
    pub status: DepartmentClearanceStatus,
    pub reason: Option<String>,
    pub contact_info: Option<String>,
}

/// Officer view — all students pending clearance in their department
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OfficerClearanceView {
    pub department_id: Uuid,
    pub department_name: String,
    pub pending_students: Vec<PendingStudentClearance>,
    pub cleared_today: i32,
    pub blocked_count: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PendingStudentClearance {
    pub student_id: Uuid,
    pub student_name: String,
    pub reg_number: String,
    pub programme: String,
    pub clearance_type: ClearanceType,
    pub initiated_at: DateTime<Utc>,
    pub department_status: DepartmentClearanceStatus,
    pub notes: Option<String>,
}

/// Generated clearance certificate
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClearanceCertificate {
    pub certificate_id: Uuid,
    pub student_id: Uuid,
    pub student_name: String,
    pub reg_number: String,
    pub programme: String,
    pub institution_name: String,
    pub clearance_type: ClearanceType,
    pub academic_year: String,
    pub issued_at: DateTime<Utc>,
    pub departments_cleared: Vec<String>,
    pub digital_signature: String,
    pub qr_verification_code: String,
    pub pdf_url: String,
}

/// Service implementation
pub struct ClearanceService;

impl ClearanceService {
    /// Initiate a clearance process for a student
    pub async fn initiate_clearance(
        _pool: &sqlx::PgPool,
        req: InitiateClearanceRequest,
    ) -> Result<StudentClearance, sqlx::Error> {
        let id = Uuid::new_v4();
        let now = Utc::now();

        // In production: load active departments for institution and create
        // a DepartmentClearance record for each one
        let clearance = StudentClearance {
            id,
            student_id: req.student_id,
            institution_id: req.institution_id,
            academic_year: req.academic_year,
            semester: req.semester,
            clearance_type: req.clearance_type,
            overall_status: ClearanceStatus::InProgress,
            department_clearances: vec![],
            initiated_at: now,
            completed_at: None,
            certificate_url: None,
        };

        // TODO: sqlx INSERT clearance + department_clearance rows
        Ok(clearance)
    }

    /// Officer marks a department as cleared or blocked
    pub async fn update_department_status(
        _pool: &sqlx::PgPool,
        req: UpdateDepartmentClearanceRequest,
    ) -> Result<DepartmentClearance, sqlx::Error> {
        let now = Utc::now();
        let rec = DepartmentClearance {
            id: Uuid::new_v4(),
            student_clearance_id: req.clearance_id,
            department_id: req.department_id,
            department_name: String::new(), // populated from JOIN in production
            status: req.status.clone(),
            reason: req.reason.clone(),
            cleared_by: if req.status == DepartmentClearanceStatus::Cleared {
                Some(req.officer_id)
            } else {
                None
            },
            cleared_at: if req.status == DepartmentClearanceStatus::Cleared {
                Some(now)
            } else {
                None
            },
            blocked_by: if req.status == DepartmentClearanceStatus::Blocked {
                Some(req.officer_id)
            } else {
                None
            },
            blocked_reason: if req.status == DepartmentClearanceStatus::Blocked {
                req.reason
            } else {
                None
            },
            notes: req.notes,
            updated_at: now,
        };

        // TODO: sqlx UPDATE department_clearances SET status, cleared_by, etc.
        // Then recompute overall_status on parent StudentClearance
        Ok(rec)
    }

    /// Get a student's clearance dashboard
    pub async fn get_student_dashboard(
        _pool: &sqlx::PgPool,
        student_id: Uuid,
        institution_id: Uuid,
        academic_year: &str,
    ) -> Result<ClearanceDashboard, sqlx::Error> {
        // TODO: SELECT FROM clearances + department_clearances WHERE student_id = $1
        let _ = (student_id, institution_id, academic_year);

        // Placeholder — real impl queries DB and builds summary
        Ok(ClearanceDashboard {
            student_id,
            overall_status: ClearanceStatus::InProgress,
            departments: vec![
                DepartmentClearanceSummary {
                    department_name: "Library".to_string(),
                    status: DepartmentClearanceStatus::Cleared,
                    reason: None,
                    contact_info: Some("library@institution.ac".to_string()),
                },
                DepartmentClearanceSummary {
                    department_name: "Finance".to_string(),
                    status: DepartmentClearanceStatus::Pending,
                    reason: None,
                    contact_info: Some("finance@institution.ac".to_string()),
                },
            ],
            all_cleared: false,
            certificate_available: false,
            certificate_url: None,
            percentage_complete: 50.0,
        })
    }

    /// Officer view — pending clearances in their department
    pub async fn get_officer_view(
        _pool: &sqlx::PgPool,
        department_id: Uuid,
        institution_id: Uuid,
    ) -> Result<OfficerClearanceView, sqlx::Error> {
        let _ = (department_id, institution_id);
        // TODO: SELECT pending_clearances WHERE department_id = $1
        Ok(OfficerClearanceView {
            department_id,
            department_name: "Finance".to_string(),
            pending_students: vec![],
            cleared_today: 0,
            blocked_count: 0,
        })
    }

    /// Issue the final clearance certificate (all departments must be Cleared)
    pub async fn issue_certificate(
        _pool: &sqlx::PgPool,
        clearance_id: Uuid,
        student_name: &str,
        reg_number: &str,
        institution_name: &str,
    ) -> Result<ClearanceCertificate, sqlx::Error> {
        let cert_id = Uuid::new_v4();
        let now = Utc::now();
        let sig = format!("sha256:{}", Uuid::new_v4());
        let qr = format!("SLC-{}", cert_id);

        Ok(ClearanceCertificate {
            certificate_id: cert_id,
            student_id: Uuid::new_v4(),
            student_name: student_name.to_string(),
            reg_number: reg_number.to_string(),
            programme: String::new(),
            institution_name: institution_name.to_string(),
            clearance_type: ClearanceType::Graduation,
            academic_year: "2024/2025".to_string(),
            issued_at: now,
            departments_cleared: vec![
                "Library".to_string(),
                "Finance".to_string(),
                "Examination Office".to_string(),
            ],
            digital_signature: sig,
            qr_verification_code: qr,
            pdf_url: format!("/certificates/clearance/{}", cert_id),
        })
    }

    /// Admin: configure departments for an institution
    pub async fn configure_departments(
        _pool: &sqlx::PgPool,
        institution_id: Uuid,
        departments: Vec<String>,
    ) -> Result<Vec<ClearanceDepartment>, sqlx::Error> {
        let now = Utc::now();
        let result = departments
            .into_iter()
            .enumerate()
            .map(|(i, name)| ClearanceDepartment {
                id: Uuid::new_v4(),
                institution_id,
                name: name.clone(),
                description: format!("{} clearance requirement", name),
                officer_role: "clearance_officer".to_string(),
                is_active: true,
                display_order: i as i32,
                created_at: now,
            })
            .collect();
        Ok(result)
    }
}
