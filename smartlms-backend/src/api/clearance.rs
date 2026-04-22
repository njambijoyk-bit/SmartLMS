// Clearance System API — Multi-department student clearance management
use crate::services::clearance::*;
use axum::{
    extract::{Extension, Json, Path, Query, State},
    http::{HeaderMap, HeaderValue, StatusCode},
    routing::{get, post, put},
    Router,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

// ============================================
// REQUEST/RESPONSE STRUCTURES
// ============================================

#[derive(Debug, Deserialize)]
pub struct InitiateClearanceQuery {
    pub student_id: Uuid,
    pub institution_id: Uuid,
    pub academic_year: String,
    pub semester: Option<u8>,
    pub clearance_type: String, // graduation, transfer, end_of_semester, withdrawal
}

#[derive(Debug, Deserialize)]
pub struct UpdateDepartmentClearanceBody {
    pub status: String, // cleared, blocked, pending
    pub reason: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct OfficerViewParams {
    pub department_id: Uuid,
    pub institution_id: Uuid,
}

// ============================================
// API HANDLERS
// ============================================

/// Get student's clearance dashboard
pub async fn get_clearance_dashboard_handler(
    Extension(user): Extension<crate::models::user::User>,
    State(pool): State<PgPool>,
) -> Result<Json<ClearanceDashboard>, (StatusCode, String)> {
    let institution_id = user.institution_id.unwrap_or_default();
    
    let dashboard = ClearanceService::get_student_dashboard(
        &pool,
        user.id,
        institution_id,
        "2025/2026",
    )
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    Ok(Json(dashboard))
}

/// Initiate clearance process (admin/student)
pub async fn initiate_clearance_handler(
    Extension(user): Extension<crate::models::user::User>,
    State(pool): State<PgPool>,
    Json(req): Json<InitiateClearanceRequest>,
) -> Result<Json<StudentClearance>, (StatusCode, String)> {
    if user.role != "admin" && user.role != "exams_officer" {
        return Err((StatusCode::FORBIDDEN, "Insufficient permissions".to_string()));
    }
    
    let clearance = ClearanceService::initiate_clearance(&pool, req)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    Ok(Json(clearance))
}

/// Update department clearance status (officer endpoint)
pub async fn update_department_clearance_handler(
    Extension(user): Extension<crate::models::user::User>,
    State(pool): State<PgPool>,
    Path(clearance_id): Path<Uuid>,
    Json(req): Json<UpdateDepartmentClearanceBody>,
) -> Result<Json<DepartmentClearance>, (StatusCode, String)> {
    if user.role != "admin" && user.role != "clearance_officer" {
        return Err((StatusCode::FORBIDDEN, "Insufficient permissions".to_string()));
    }
    
    let status = match req.status.as_str() {
        "cleared" => DepartmentClearanceStatus::Cleared,
        "blocked" => DepartmentClearanceStatus::Blocked,
        "pending" => DepartmentClearanceStatus::Pending,
        _ => return Err((StatusCode::BAD_REQUEST, "Invalid status".to_string())),
    };
    
    let update_req = UpdateDepartmentClearanceRequest {
        clearance_id,
        department_id: Uuid::nil(), // TODO: Get from context or request
        status,
        reason: req.reason,
        notes: req.notes,
        officer_id: user.id,
    };
    
    let record = ClearanceService::update_department_status(&pool, update_req)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    Ok(Json(record))
}

/// Get officer view - pending clearances
pub async fn get_officer_view_handler(
    Extension(user): Extension<crate::models::user::User>,
    State(pool): State<PgPool>,
    Query(params): Query<OfficerViewParams>,
) -> Result<Json<OfficerClearanceView>, (StatusCode, String)> {
    if user.role != "admin" && user.role != "clearance_officer" {
        return Err((StatusCode::FORBIDDEN, "Insufficient permissions".to_string()));
    }
    
    let view = ClearanceService::get_officer_view(&pool, params.department_id, params.institution_id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    Ok(Json(view))
}

/// Issue clearance certificate
pub async fn issue_clearance_certificate_handler(
    Extension(user): Extension<crate::models::user::User>,
    State(pool): State<PgPool>,
    Path(clearance_id): Path<Uuid>,
) -> Result<Json<ClearanceCertificate>, (StatusCode, String)> {
    if user.role != "admin" && user.role != "exams_officer" {
        return Err((StatusCode::FORBIDDEN, "Insufficient permissions".to_string()));
    }
    
    // TODO: Get student details from clearance record
    let cert = ClearanceService::issue_certificate(
        &pool,
        clearance_id,
        "Student Name",
        "REG/2024/001",
        "SmartLMS University",
    )
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    Ok(Json(cert))
}

/// Download clearance certificate as PDF
pub async fn download_clearance_certificate_handler(
    Extension(user): Extension<crate::models::user::User>,
    State(pool): State<PgPool>,
    Path(clearance_id): Path<Uuid>,
) -> Result<(HeaderMap, Vec<u8>), (StatusCode, String)> {
    // TODO: Generate PDF certificate
    // For now, return placeholder
    let pdf_content = b"%PDF-1.4\nPlaceholder PDF for clearance certificate";
    
    let mut headers = HeaderMap::new();
    headers.insert(
        "Content-Type",
        HeaderValue::from_static("application/pdf"),
    );
    headers.insert(
        "Content-Disposition",
        HeaderValue::from_str(&format!("attachment; filename=\"clearance_certificate_{}.pdf\"", clearance_id)).unwrap(),
    );
    
    Ok((headers, pdf_content.to_vec()))
}

/// Configure clearance departments (admin)
pub async fn configure_departments_handler(
    Extension(user): Extension<crate::models::user::User>,
    State(pool): State<PgPool>,
    Json(req): Json<ConfigureDepartmentsRequest>,
) -> Result<Json<Vec<ClearanceDepartment>>, (StatusCode, String)> {
    if user.role != "admin" {
        return Err((StatusCode::FORBIDDEN, "Admin access required".to_string()));
    }
    
    let departments = ClearanceService::configure_departments(&pool, req.institution_id, req.departments)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    Ok(Json(departments))
}

#[derive(Debug, Deserialize)]
pub struct ConfigureDepartmentsRequest {
    pub institution_id: Uuid,
    pub departments: Vec<String>,
}

// ============================================
// ROUTER CREATION
// ============================================

pub fn clearance_router() -> Router {
    Router::new()
        .route("/dashboard", get(get_clearance_dashboard_handler))
        .route("/initiate", post(initiate_clearance_handler))
        .route("/:clearance_id/departments", put(update_department_clearance_handler))
        .route("/officer/view", get(get_officer_view_handler))
        .route("/:clearance_id/certificate", post(issue_clearance_certificate_handler))
        .route("/:clearance_id/certificate/download", get(download_clearance_certificate_handler))
        .route("/departments/configure", post(configure_departments_handler))
}
