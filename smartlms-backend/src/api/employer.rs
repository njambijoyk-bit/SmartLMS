//! Employer & Career Portal API Endpoints
//! 
//! Provides REST endpoints for employer interactions, job board,
//! internship tracking, and campus recruitment.

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use crate::models::user::UserClaims;
use crate::services::employer::{EmployerService, EmployerError};
use crate::api::ApiResponse;

/// Job search parameters
#[derive(Debug, Deserialize)]
pub struct JobSearchParams {
    pub keyword: Option<String>,
    pub location: Option<String>,
    pub job_type: Option<String>,
    pub experience_level: Option<String>,
    pub remote: Option<bool>,
    pub skills: Option<Vec<String>>,
    pub min_salary: Option<i64>,
    pub max_salary: Option<i64>,
    pub page: Option<i32>,
    pub limit: Option<i32>,
}

/// Application filter parameters
#[derive(Debug, Deserialize)]
pub struct ApplicationFilters {
    pub status: Option<String>,
    pub job_id: Option<i64>,
    pub candidate_id: Option<i64>,
    pub date_from: Option<String>,
    pub date_to: Option<String>,
}

/// Internship search parameters
#[derive(Debug, Deserialize)]
pub struct InternshipSearchParams {
    pub field: Option<String>,
    pub duration_months: Option<i32>,
    pub stipend_min: Option<i64>,
    pub location: Option<String>,
    pub remote: Option<bool>,
    pub page: Option<i32>,
    pub limit: Option<i32>,
}

/// Event filter parameters
#[derive(Debug, Deserialize)]
pub struct EventFilters {
    pub event_type: Option<String>,
    pub status: Option<String>,
    pub date_from: Option<String>,
    pub date_to: Option<String>,
    pub institution_id: Option<i64>,
}

/// Partnership filter parameters
#[derive(Debug, Deserialize)]
pub struct PartnershipFilters {
    pub partnership_type: Option<String>,
    pub status: Option<String>,
    pub industry_sector: Option<String>,
}

/// Skill match request
#[derive(Debug, Deserialize)]
pub struct SkillMatchRequest {
    pub job_id: i64,
    pub candidate_ids: Vec<i64>,
}

/// Create job request
#[derive(Debug, Deserialize)]
pub struct CreateJobRequest {
    pub title: String,
    pub description: String,
    pub requirements: Vec<String>,
    pub responsibilities: Vec<String>,
    pub location: String,
    pub job_type: String,
    pub experience_level: String,
    pub salary_min: Option<i64>,
    pub salary_max: Option<i64>,
    pub salary_period: Option<String>,
    pub remote: bool,
    pub skills_required: Vec<String>,
    pub application_deadline: Option<String>,
    pub openings: i32,
    pub department: Option<String>,
    pub benefits: Option<Vec<String>>,
    pub company_name: Option<String>,
    pub company_logo: Option<String>,
    pub contact_email: String,
}

/// Update job request
#[derive(Debug, Deserialize)]
pub struct UpdateJobRequest {
    pub title: Option<String>,
    pub description: Option<String>,
    pub requirements: Option<Vec<String>>,
    pub responsibilities: Option<Vec<String>>,
    pub location: Option<String>,
    pub job_type: Option<String>,
    pub experience_level: Option<String>,
    pub salary_min: Option<i64>,
    pub salary_max: Option<i64>,
    pub salary_period: Option<String>,
    pub remote: Option<bool>,
    pub skills_required: Option<Vec<String>>,
    pub application_deadline: Option<String>,
    pub openings: Option<i32>,
    pub department: Option<String>,
    pub benefits: Option<Vec<String>>,
    pub status: Option<String>,
}

/// Create internship request
#[derive(Debug, Deserialize)]
pub struct CreateInternshipRequest {
    pub title: String,
    pub description: String,
    pub field: String,
    pub duration_months: i32,
    pub location: String,
    pub remote: bool,
    pub stipend_amount: Option<i64>,
    pub stipend_period: Option<String>,
    pub requirements: Vec<String>,
    pub learning_objectives: Vec<String>,
    pub mentor_name: Option<String>,
    pub mentor_email: Option<String>,
    pub start_date: String,
    pub end_date: String,
    pub openings: i32,
    pub application_deadline: String,
    pub company_name: String,
    pub contact_email: String,
}

/// Update application status request
#[derive(Debug, Deserialize)]
pub struct UpdateApplicationStatusRequest {
    pub status: String,
    pub feedback: Option<String>,
    pub next_steps: Option<String>,
}

/// Create recruitment event request
#[derive(Debug, Deserialize)]
pub struct CreateEventRequest {
    pub title: String,
    pub description: String,
    pub event_type: String,
    pub format: String,
    pub start_datetime: String,
    pub end_datetime: String,
    pub location: Option<String>,
    pub virtual_link: Option<String>,
    pub capacity: i32,
    pub target_audience: Option<String>,
    pub registration_deadline: Option<String>,
    pub company_name: String,
    pub contact_email: String,
    pub agenda: Option<Vec<String>>,
}

/// Register for event request
#[derive(Debug, Deserialize)]
pub struct RegisterEventRequest {
    pub candidate_id: i64,
    pub resume_url: Option<String>,
    pub cover_letter: Option<String>,
}

/// Create partnership request
#[derive(Debug, Deserialize)]
pub struct CreatePartnershipRequest {
    pub partner_name: String,
    pub partnership_type: String,
    pub industry_sector: String,
    pub description: String,
    pub objectives: Vec<String>,
    pub start_date: String,
    pub end_date: Option<String>,
    pub contact_name: String,
    pub contact_email: String,
    pub contact_phone: Option<String>,
    pub benefits_offered: Vec<String>,
    pub benefits_requested: Vec<String>,
    pub agreement_terms: Option<String>,
}

/// Update partnership request
#[derive(Debug, Deserialize)]
pub struct UpdatePartnershipRequest {
    pub description: Option<String>,
    pub objectives: Option<Vec<String>>,
    pub end_date: Option<String>,
    pub contact_email: Option<String>,
    pub benefits_offered: Option<Vec<String>>,
    pub benefits_requested: Option<Vec<String>>,
    pub status: Option<String>,
}

/// Create candidate profile request
#[derive(Debug, Deserialize)]
pub struct CreateCandidateProfileRequest {
    pub user_id: i64,
    pub headline: Option<String>,
    pub summary: Option<String>,
    pub skills: Vec<String>,
    pub desired_roles: Vec<String>,
    pub desired_locations: Vec<String>,
    pub remote_preference: bool,
    pub salary_expectation: Option<i64>,
    pub availability: Option<String>,
    pub portfolio_url: Option<String>,
    pub linkedin_url: Option<String>,
    pub github_url: Option<String>,
    pub resume_url: Option<String>,
}

/// Add education request
#[derive(Debug, Deserialize)]
pub struct AddEducationRequest {
    pub institution_name: String,
    pub degree: String,
    pub field_of_study: String,
    pub start_date: String,
    pub end_date: Option<String>,
    pub gpa: Option<f32>,
    pub activities: Option<Vec<String>>,
    pub description: Option<String>,
}

/// Add experience request
#[derive(Debug, Deserialize)]
pub struct AddExperienceRequest {
    pub company_name: String,
    pub job_title: String,
    pub location: Option<String>,
    pub start_date: String,
    pub end_date: Option<String>,
    pub current: bool,
    pub description: String,
    pub achievements: Option<Vec<String>>,
    pub skills_used: Option<Vec<String>>,
}

// ============================================================================
// API Handlers
// ============================================================================

/// Get all jobs with filtering
pub async fn get_jobs(
    State(pool): State<PgPool>,
    Query(params): Query<JobSearchParams>,
) -> ApiResponse<Vec<serde_json::Value>> {
    let service = EmployerService::new(pool);
    
    match service.search_jobs(
        params.keyword,
        params.location,
        params.job_type,
        params.experience_level,
        params.remote,
        params.skills,
        params.min_salary,
        params.max_salary,
        params.page.unwrap_or(1),
        params.limit.unwrap_or(20),
    ).await {
        Ok(jobs) => ApiResponse::success(jobs),
        Err(e) => ApiResponse::error(e.to_string()),
    }
}

/// Get job by ID
pub async fn get_job(
    State(pool): State<PgPool>,
    Path(job_id): Path<i64>,
) -> ApiResponse<serde_json::Value> {
    let service = EmployerService::new(pool);
    
    match service.get_job_by_id(job_id).await {
        Ok(job) => ApiResponse::success(job),
        Err(EmployerError::NotFound(msg)) => ApiResponse::not_found(msg),
        Err(e) => ApiResponse::error(e.to_string()),
    }
}

/// Create new job posting
pub async fn create_job(
    State(pool): State<PgPool>,
    claims: UserClaims,
    Json(req): Json<CreateJobRequest>,
) -> ApiResponse<serde_json::Value> {
    let service = EmployerService::new(pool);
    
    match service.create_job(
        claims.user_id,
        req.title,
        req.description,
        req.requirements,
        req.responsibilities,
        req.location,
        req.job_type,
        req.experience_level,
        req.salary_min,
        req.salary_max,
        req.salary_period,
        req.remote,
        req.skills_required,
        req.application_deadline,
        req.openings,
        req.department,
        req.benefits,
        req.company_name,
        req.company_logo,
        req.contact_email,
    ).await {
        Ok(job) => ApiResponse::created(job),
        Err(e) => ApiResponse::error(e.to_string()),
    }
}

/// Update job posting
pub async fn update_job(
    State(pool): State<PgPool>,
    Path(job_id): Path<i64>,
    Json(req): Json<UpdateJobRequest>,
) -> ApiResponse<serde_json::Value> {
    let service = EmployerService::new(pool);
    
    match service.update_job(
        job_id,
        req.title,
        req.description,
        req.requirements,
        req.responsibilities,
        req.location,
        req.job_type,
        req.experience_level,
        req.salary_min,
        req.salary_max,
        req.salary_period,
        req.remote,
        req.skills_required,
        req.application_deadline,
        req.openings,
        req.department,
        req.benefits,
        req.status,
    ).await {
        Ok(job) => ApiResponse::success(job),
        Err(EmployerError::NotFound(msg)) => ApiResponse::not_found(msg),
        Err(e) => ApiResponse::error(e.to_string()),
    }
}

/// Delete job posting
pub async fn delete_job(
    State(pool): State<PgPool>,
    Path(job_id): Path<i64>,
) -> ApiResponse<serde_json::Value> {
    let service = EmployerService::new(pool);
    
    match service.delete_job(job_id).await {
        Ok(_) => ApiResponse::success(serde_json::json!({"deleted": true})),
        Err(EmployerError::NotFound(msg)) => ApiResponse::not_found(msg),
        Err(e) => ApiResponse::error(e.to_string()),
    }
}

/// Get skill matches for a job
pub async fn get_skill_matches(
    State(pool): State<PgPool>,
    Path(job_id): Path<i64>,
    Query(candidate_ids): Query<SkillMatchRequest>,
) -> ApiResponse<serde_json::Value> {
    let service = EmployerService::new(pool);
    
    match service.match_candidates_to_job(job_id, candidate_ids.candidate_ids).await {
        Ok(matches) => ApiResponse::success(matches),
        Err(EmployerError::NotFound(msg)) => ApiResponse::not_found(msg),
        Err(e) => ApiResponse::error(e.to_string()),
    }
}

/// Get all job applications
pub async fn get_applications(
    State(pool): State<PgPool>,
    Query(filters): Query<ApplicationFilters>,
) -> ApiResponse<Vec<serde_json::Value>> {
    let service = EmployerService::new(pool);
    
    match service.get_applications(
        filters.status,
        filters.job_id,
        filters.candidate_id,
        filters.date_from,
        filters.date_to,
    ).await {
        Ok(apps) => ApiResponse::success(apps),
        Err(e) => ApiResponse::error(e.to_string()),
    }
}

/// Submit job application
pub async fn submit_application(
    State(pool): State<PgPool>,
    claims: UserClaims,
    Json(req): Json<serde_json::Value>,
) -> ApiResponse<serde_json::Value> {
    let service = EmployerService::new(pool);
    
    let resume_url = req.get("resume_url").and_then(|v| v.as_str()).map(String::from);
    let cover_letter = req.get("cover_letter").and_then(|v| v.as_str()).map(String::from);
    let job_id = req.get("job_id").and_then(|v| v.as_i64()).unwrap_or(0);
    
    if job_id == 0 {
        return ApiResponse::bad_request("job_id is required");
    }
    
    match service.apply_to_job(claims.user_id, job_id, resume_url, cover_letter).await {
        Ok(app) => ApiResponse::created(app),
        Err(e) => ApiResponse::error(e.to_string()),
    }
}

/// Update application status
pub async fn update_application_status(
    State(pool): State<PgPool>,
    Path(app_id): Path<i64>,
    Json(req): Json<UpdateApplicationStatusRequest>,
) -> ApiResponse<serde_json::Value> {
    let service = EmployerService::new(pool);
    
    match service.update_application_status(app_id, req.status, req.feedback, req.next_steps).await {
        Ok(app) => ApiResponse::success(app),
        Err(EmployerError::NotFound(msg)) => ApiResponse::not_found(msg),
        Err(e) => ApiResponse::error(e.to_string()),
    }
}

/// Get all internships
pub async fn get_internships(
    State(pool): State<PgPool>,
    Query(params): Query<InternshipSearchParams>,
) -> ApiResponse<Vec<serde_json::Value>> {
    let service = EmployerService::new(pool);
    
    match service.search_internships(
        params.field,
        params.duration_months,
        params.stipend_min,
        params.location,
        params.remote,
        params.page.unwrap_or(1),
        params.limit.unwrap_or(20),
    ).await {
        Ok(internships) => ApiResponse::success(internships),
        Err(e) => ApiResponse::error(e.to_string()),
    }
}

/// Get internship by ID
pub async fn get_internship(
    State(pool): State<PgPool>,
    Path(internship_id): Path<i64>,
) -> ApiResponse<serde_json::Value> {
    let service = EmployerService::new(pool);
    
    match service.get_internship_by_id(internship_id).await {
        Ok(internship) => ApiResponse::success(internship),
        Err(EmployerError::NotFound(msg)) => ApiResponse::not_found(msg),
        Err(e) => ApiResponse::error(e.to_string()),
    }
}

/// Create internship posting
pub async fn create_internship(
    State(pool): State<PgPool>,
    claims: UserClaims,
    Json(req): Json<CreateInternshipRequest>,
) -> ApiResponse<serde_json::Value> {
    let service = EmployerService::new(pool);
    
    match service.create_internship(
        claims.user_id,
        req.title,
        req.description,
        req.field,
        req.duration_months,
        req.location,
        req.remote,
        req.stipend_amount,
        req.stipend_period,
        req.requirements,
        req.learning_objectives,
        req.mentor_name,
        req.mentor_email,
        req.start_date,
        req.end_date,
        req.openings,
        req.application_deadline,
        req.company_name,
        req.contact_email,
    ).await {
        Ok(internship) => ApiResponse::created(internship),
        Err(e) => ApiResponse::error(e.to_string()),
    }
}

/// Apply to internship
pub async fn apply_to_internship(
    State(pool): State<PgPool>,
    claims: UserClaims,
    Path(internship_id): Path<i64>,
    Json(req): Json<serde_json::Value>,
) -> ApiResponse<serde_json::Value> {
    let service = EmployerService::new(pool);
    
    let resume_url = req.get("resume_url").and_then(|v| v.as_str()).map(String::from);
    let cover_letter = req.get("cover_letter").and_then(|v| v.as_str()).map(String::from);
    
    match service.apply_to_internship(claims.user_id, internship_id, resume_url, cover_letter).await {
        Ok(app) => ApiResponse::created(app),
        Err(e) => ApiResponse::error(e.to_string()),
    }
}

/// Get recruitment events
pub async fn get_events(
    State(pool): State<PgPool>,
    Query(filters): Query<EventFilters>,
) -> ApiResponse<Vec<serde_json::Value>> {
    let service = EmployerService::new(pool);
    
    match service.get_recruitment_events(
        filters.event_type,
        filters.status,
        filters.date_from,
        filters.date_to,
        filters.institution_id,
    ).await {
        Ok(events) => ApiResponse::success(events),
        Err(e) => ApiResponse::error(e.to_string()),
    }
}

/// Get event by ID
pub async fn get_event(
    State(pool): State<PgPool>,
    Path(event_id): Path<i64>,
) -> ApiResponse<serde_json::Value> {
    let service = EmployerService::new(pool);
    
    match service.get_event_by_id(event_id).await {
        Ok(event) => ApiResponse::success(event),
        Err(EmployerError::NotFound(msg)) => ApiResponse::not_found(msg),
        Err(e) => ApiResponse::error(e.to_string()),
    }
}

/// Create recruitment event
pub async fn create_event(
    State(pool): State<PgPool>,
    claims: UserClaims,
    Json(req): Json<CreateEventRequest>,
) -> ApiResponse<serde_json::Value> {
    let service = EmployerService::new(pool);
    
    match service.create_recruitment_event(
        claims.user_id,
        req.title,
        req.description,
        req.event_type,
        req.format,
        req.start_datetime,
        req.end_datetime,
        req.location,
        req.virtual_link,
        req.capacity,
        req.target_audience,
        req.registration_deadline,
        req.company_name,
        req.contact_email,
        req.agenda,
    ).await {
        Ok(event) => ApiResponse::created(event),
        Err(e) => ApiResponse::error(e.to_string()),
    }
}

/// Register for event
pub async fn register_for_event(
    State(pool): State<PgPool>,
    Path(event_id): Path<i64>,
    Json(req): Json<RegisterEventRequest>,
) -> ApiResponse<serde_json::Value> {
    let service = EmployerService::new(pool);
    
    match service.register_for_event(event_id, req.candidate_id, req.resume_url, req.cover_letter).await {
        Ok(registration) => ApiResponse::created(registration),
        Err(e) => ApiResponse::error(e.to_string()),
    }
}

/// Get partnerships
pub async fn get_partnerships(
    State(pool): State<PgPool>,
    Query(filters): Query<PartnershipFilters>,
) -> ApiResponse<Vec<serde_json::Value>> {
    let service = EmployerService::new(pool);
    
    match service.get_partnerships(
        filters.partnership_type,
        filters.status,
        filters.industry_sector,
    ).await {
        Ok(partnerships) => ApiResponse::success(partnerships),
        Err(e) => ApiResponse::error(e.to_string()),
    }
}

/// Get partnership by ID
pub async fn get_partnership(
    State(pool): State<PgPool>,
    Path(partnership_id): Path<i64>,
) -> ApiResponse<serde_json::Value> {
    let service = EmployerService::new(pool);
    
    match service.get_partnership_by_id(partnership_id).await {
        Ok(partnership) => ApiResponse::success(partnership),
        Err(EmployerError::NotFound(msg)) => ApiResponse::not_found(msg),
        Err(e) => ApiResponse::error(e.to_string()),
    }
}

/// Create partnership
pub async fn create_partnership(
    State(pool): State<PgPool>,
    claims: UserClaims,
    Json(req): Json<CreatePartnershipRequest>,
) -> ApiResponse<serde_json::Value> {
    let service = EmployerService::new(pool);
    
    match service.create_partnership(
        claims.user_id,
        req.partner_name,
        req.partnership_type,
        req.industry_sector,
        req.description,
        req.objectives,
        req.start_date,
        req.end_date,
        req.contact_name,
        req.contact_email,
        req.contact_phone,
        req.benefits_offered,
        req.benefits_requested,
        req.agreement_terms,
    ).await {
        Ok(partnership) => ApiResponse::created(partnership),
        Err(e) => ApiResponse::error(e.to_string()),
    }
}

/// Update partnership
pub async fn update_partnership(
    State(pool): State<PgPool>,
    Path(partnership_id): Path<i64>,
    Json(req): Json<UpdatePartnershipRequest>,
) -> ApiResponse<serde_json::Value> {
    let service = EmployerService::new(pool);
    
    match service.update_partnership(
        partnership_id,
        req.description,
        req.objectives,
        req.end_date,
        req.contact_email,
        req.benefits_offered,
        req.benefits_requested,
        req.status,
    ).await {
        Ok(partnership) => ApiResponse::success(partnership),
        Err(EmployerError::NotFound(msg)) => ApiResponse::not_found(msg),
        Err(e) => ApiResponse::error(e.to_string()),
    }
}

/// Get candidate profile
pub async fn get_candidate_profile(
    State(pool): State<PgPool>,
    Path(user_id): Path<i64>,
) -> ApiResponse<serde_json::Value> {
    let service = EmployerService::new(pool);
    
    match service.get_candidate_profile(user_id).await {
        Ok(profile) => ApiResponse::success(profile),
        Err(EmployerError::NotFound(msg)) => ApiResponse::not_found(msg),
        Err(e) => ApiResponse::error(e.to_string()),
    }
}

/// Create/Update candidate profile
pub async fn create_candidate_profile(
    State(pool): State<PgPool>,
    claims: UserClaims,
    Json(req): Json<CreateCandidateProfileRequest>,
) -> ApiResponse<serde_json::Value> {
    let service = EmployerService::new(pool);
    
    match service.create_or_update_profile(
        req.user_id,
        req.headline,
        req.summary,
        req.skills,
        req.desired_roles,
        req.desired_locations,
        req.remote_preference,
        req.salary_expectation,
        req.availability,
        req.portfolio_url,
        req.linkedin_url,
        req.github_url,
        req.resume_url,
    ).await {
        Ok(profile) => ApiResponse::success(profile),
        Err(e) => ApiResponse::error(e.to_string()),
    }
}

/// Add education to profile
pub async fn add_education(
    State(pool): State<PgPool>,
    Path(user_id): Path<i64>,
    Json(req): Json<AddEducationRequest>,
) -> ApiResponse<serde_json::Value> {
    let service = EmployerService::new(pool);
    
    match service.add_education(
        user_id,
        req.institution_name,
        req.degree,
        req.field_of_study,
        req.start_date,
        req.end_date,
        req.gpa,
        req.activities,
        req.description,
    ).await {
        Ok(edu) => ApiResponse::created(edu),
        Err(e) => ApiResponse::error(e.to_string()),
    }
}

/// Add experience to profile
pub async fn add_experience(
    State(pool): State<PgPool>,
    Path(user_id): Path<i64>,
    Json(req): Json<AddExperienceRequest>,
) -> ApiResponse<serde_json::Value> {
    let service = EmployerService::new(pool);
    
    match service.add_experience(
        user_id,
        req.company_name,
        req.job_title,
        req.location,
        req.start_date,
        req.end_date,
        req.current,
        req.description,
        req.achievements,
        req.skills_used,
    ).await {
        Ok(exp) => ApiResponse::created(exp),
        Err(e) => ApiResponse::error(e.to_string()),
    }
}

/// Get employer dashboard stats
pub async fn get_employer_dashboard(
    State(pool): State<PgPool>,
    claims: UserClaims,
) -> ApiResponse<serde_json::Value> {
    let service = EmployerService::new(pool);
    
    match service.get_employer_dashboard(claims.user_id).await {
        Ok(stats) => ApiResponse::success(stats),
        Err(e) => ApiResponse::error(e.to_string()),
    }
}

// ============================================================================
// Router Configuration
// ============================================================================

use axum::Router;
use tower_http::auth::AsyncRequireAuthorizationLayer;
use crate::middleware::auth::AuthMiddleware;

/// Create employer portal router
pub fn employer_router() -> Router {
    Router::new()
        // Job endpoints
        .route("/jobs", axum::routing::get(get_jobs))
        .route("/jobs", axum::routing::post(create_job))
        .route("/jobs/:job_id", axum::routing::get(get_job))
        .route("/jobs/:job_id", axum::routing::put(update_job))
        .route("/jobs/:job_id", axum::routing::delete(delete_job))
        .route("/jobs/:job_id/matches", axum::routing::get(get_skill_matches))
        // Application endpoints
        .route("/applications", axum::routing::get(get_applications))
        .route("/applications", axum::routing::post(submit_application))
        .route("/applications/:app_id", axum::routing::put(update_application_status))
        // Internship endpoints
        .route("/internships", axum::routing::get(get_internships))
        .route("/internships/:internship_id", axum::routing::get(get_internship))
        .route("/internships", axum::routing::post(create_internship))
        .route("/internships/:internship_id/apply", axum::routing::post(apply_to_internship))
        // Event endpoints
        .route("/events", axum::routing::get(get_events))
        .route("/events/:event_id", axum::routing::get(get_event))
        .route("/events", axum::routing::post(create_event))
        .route("/events/:event_id/register", axum::routing::post(register_for_event))
        // Partnership endpoints
        .route("/partnerships", axum::routing::get(get_partnerships))
        .route("/partnerships/:partnership_id", axum::routing::get(get_partnership))
        .route("/partnerships", axum::routing::post(create_partnership))
        .route("/partnerships/:partnership_id", axum::routing::put(update_partnership))
        // Candidate profile endpoints
        .route("/candidates/:user_id/profile", axum::routing::get(get_candidate_profile))
        .route("/candidates/profile", axum::routing::post(create_candidate_profile))
        .route("/candidates/:user_id/education", axum::routing::post(add_education))
        .route("/candidates/:user_id/experience", axum::routing::post(add_experience))
        // Dashboard
        .route("/dashboard", axum::routing::get(get_employer_dashboard))
        .layer(AsyncRequireAuthorizationLayer::new(AuthMiddleware::new()))
}
