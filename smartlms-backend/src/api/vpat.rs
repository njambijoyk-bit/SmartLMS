// Phase 16 Enhancement: VPAT (Voluntary Product Accessibility Template) API
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::services::vpat::{VpatService, VpatGenerationRequest, WcagConformanceLevel, ConformanceStatus};
use crate::utils::app_state::AppState;

#[derive(Debug, Deserialize)]
pub struct GenerateVpatRequest {
    pub product_name: String,
    pub product_version: String,
    pub vendor_name: String,
    pub vendor_contact: Option<String>,
    pub wcag_level: String, // "A", "AA", or "AAA"
    pub include_section_508: bool,
    pub include_en_301_549: bool,
}

#[derive(Debug, Serialize)]
pub struct VpatResponse {
    pub id: Uuid,
    pub product_name: String,
    pub product_version: String,
    pub report_date: String,
    pub vendor_name: String,
    pub wcag_level: String,
    pub section_508_compliant: bool,
    pub en_301_549_compliant: bool,
    pub overall_compliance_score: f64,
    pub total_criteria: i32,
    pub passed_criteria: i32,
    pub partially_met_criteria: i32,
    pub not_met_criteria: i32,
    pub not_applicable_criteria: i32,
    pub criteria_count: usize,
}

#[derive(Debug, Deserialize)]
pub struct UpdateCriterionRequest {
    pub conformance_status: String,
    pub user_notes: Option<String>,
    pub evidence_url: Option<String>,
    pub remediation_plan: Option<String>,
    pub target_remediation_date: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct CriterionResponse {
    pub id: Uuid,
    pub criterion_number: String,
    pub criterion_name: String,
    pub wcag_criterion: Option<String>,
    pub conformance_status: String,
    pub user_notes: Option<String>,
    pub evidence_url: Option<String>,
    pub remediation_plan: Option<String>,
    pub target_remediation_date: Option<String>,
}

/// POST /api/vpat/generate - Generate a new VPAT report
pub async fn generate_vpat(
    State(_state): State<AppState>,
    Json(payload): Json<GenerateVpatRequest>,
) -> Result<Json<VpatResponse>, StatusCode> {
    let wcag_level = match payload.wcag_level.as_str() {
        "A" => WcagConformanceLevel::A,
        "AA" => WcagConformanceLevel::AA,
        "AAA" => WcagConformanceLevel::AAA,
        _ => return Err(StatusCode::BAD_REQUEST),
    };

    let request = VpatGenerationRequest {
        product_name: payload.product_name,
        product_version: payload.product_version,
        vendor_name: payload.vendor_name,
        vendor_contact: payload.vendor_contact,
        wcag_level,
        include_section_508: payload.include_section_508,
        include_en_301_549: payload.include_en_301_549,
    };

    let report = VpatService::create_report(request);
    
    Ok(Json(VpatResponse {
        id: report.id,
        product_name: report.product_name,
        product_version: report.product_version,
        report_date: report.report_date.to_string(),
        vendor_name: report.vendor_name,
        wcag_level: format!("{:?}", report.wcag_level),
        section_508_compliant: report.section_508_compliant,
        en_301_549_compliant: report.en_301_549_compliant,
        overall_compliance_score: report.overall_compliance_score,
        total_criteria: report.total_criteria,
        passed_criteria: report.passed_criteria,
        partially_met_criteria: report.partially_met_criteria,
        not_met_criteria: report.not_met_criteria,
        not_applicable_criteria: report.not_applicable_criteria,
        criteria_count: report.criteria.len(),
    }))
}

/// GET /api/vpat/{id} - Get VPAT report details
pub async fn get_vpat_report(
    State(_state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<VpatResponse>, StatusCode> {
    // TODO: Retrieve from database
    // For now, return a placeholder
    Err(StatusCode::NOT_FOUND)
}

/// GET /api/vpat/{id}/criteria - Get all criteria for a VPAT report
pub async fn get_vpat_criteria(
    State(_state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<Vec<CriterionResponse>>, StatusCode> {
    // TODO: Retrieve from database
    Err(StatusCode::NOT_FOUND)
}

/// PUT /api/vpat/{report_id}/criteria/{criterion_id} - Update criterion
pub async fn update_criterion(
    State(_state): State<AppState>,
    Path((report_id, criterion_id)): Path<(Uuid, Uuid)>,
    Json(payload): Json<UpdateCriterionRequest>,
) -> Result<Json<CriterionResponse>, StatusCode> {
    let conformance_status = match payload.conformance_status.as_str() {
        "Supports" => ConformanceStatus::Supports,
        "PartiallySupports" => ConformanceStatus::PartiallySupports,
        "DoesNotSupport" => ConformanceStatus::DoesNotSupport,
        "NotApplicable" => ConformanceStatus::NotApplicable,
        _ => return Err(StatusCode::BAD_REQUEST),
    };

    // TODO: Update in database
    
    Ok(Json(CriterionResponse {
        id: criterion_id,
        criterion_number: "1.1.1".to_string(),
        criterion_name: "Non-text Content".to_string(),
        wcag_criterion: Some("WCAG 2.1 Level A".to_string()),
        conformance_status: payload.conformance_status,
        user_notes: payload.user_notes,
        evidence_url: payload.evidence_url,
        remediation_plan: payload.remediation_plan,
        target_remediation_date: payload.target_remediation_date,
    }))
}

/// POST /api/vpat/{id}/export/pdf - Export VPAT report as PDF
pub async fn export_vpat_pdf(
    State(_state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<(Vec<u8>, &'static str), StatusCode> {
    // TODO: Generate PDF using a PDF library
    // Return binary PDF data with appropriate content type
    Err(StatusCode::NOT_FOUND)
}

/// POST /api/vpat/{id}/export/html - Export VPAT report as HTML
pub async fn export_vpat_html(
    State(_state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<String, StatusCode> {
    // TODO: Generate HTML report
    Err(StatusCode::NOT_FOUND)
}

/// GET /api/vpat/templates - Get available VPAT templates
pub async fn get_vpat_templates() -> Result<Json<Vec<&'static str>>, StatusCode> {
    Ok(Json(vec![
        "WCAG 2.1 AA",
        "WCAG 2.1 AAA",
        "Section 508",
        "EN 301 549",
        "Combined Standards",
    ]))
}

pub fn vpat_router() -> axum::Router {
    axum::Router::new()
        .route("/generate", axum::routing::post(generate_vpat))
        .route("/:id", axum::routing::get(get_vpat_report))
        .route("/:id/criteria", axum::routing::get(get_vpat_criteria))
        .route("/:report_id/criteria/:criterion_id", axum::routing::put(update_criterion))
        .route("/:id/export/pdf", axum::routing::post(export_vpat_pdf))
        .route("/:id/export/html", axum::routing::post(export_vpat_html))
        .route("/templates", axum::routing::get(get_vpat_templates))
}
