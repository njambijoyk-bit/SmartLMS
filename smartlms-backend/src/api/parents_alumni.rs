// API routes for Module 18 (Parents Portal), Module 23 (ID Cards), Module 24 (Alumni Portal)
use crate::models::parents_alumni::*;
use crate::db::parents_alumni as db;
use axum::{
    extract::{Extension, Json, Path, Query, State},
    http::StatusCode,
    routing::{get, post, put, delete},
    Router,
};
use sqlx::PgPool;
use uuid::Uuid;

// ============================================
// PARENTS PORTAL API ROUTES
// ============================================

/// Get parent dashboard - all linked students
pub async fn get_parent_dashboard(
    Extension(user): Extension<crate::models::user::User>,
    State(pool): State<PgPool>,
) -> Result<Json<ParentDashboardResponse>, (StatusCode, String)> {
    let links = db::get_parent_links_by_parent(&pool, user.id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    // Filter only active links
    let active_links: Vec<_> = links.into_iter().filter(|l| l.status == "active").collect();
    
    // TODO: Fetch student summaries from users table
    let students = vec![];
    
    Ok(Json(ParentDashboardResponse {
        links: active_links,
        students,
    }))
}

/// Request parent-student link
pub async fn request_parent_link(
    Extension(user): Extension<crate::models::user::User>,
    State(pool): State<PgPool>,
    Json(req): Json<CreateParentLinkRequest>,
) -> Result<StatusCode, (StatusCode, String)> {
    // Find student by email
    let student = crate::db::user::find_by_email(&pool, &req.student_email)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or_else(|| (StatusCode::BAD_REQUEST, "Student not found".to_string()))?;
    
    // Verify student has learner role
    if student.role != "learner" {
        return Err((StatusCode::BAD_REQUEST, "User is not a student".to_string()));
    }
    
    // Create link request
    db::create_parent_link(&pool, user.id, student.id, req.linkage_type.as_deref().unwrap_or("self_service"))
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    // TODO: Send notification to student for approval
    
    Ok(StatusCode::CREATED)
}

/// Approve/revoke parent link (student endpoint)
pub async fn approve_parent_link(
    Extension(user): Extension<crate::models::user::User>,
    State(pool): State<PgPool>,
    Json(req): Json<ApproveParentLinkRequest>,
) -> Result<StatusCode, (StatusCode, String)> {
    // Verify this link belongs to the current user as student
    let links = db::get_parent_links_by_student(&pool, user.id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    if !links.iter().any(|l| l.id == req.link_id) {
        return Err((StatusCode::FORBIDDEN, "Link not found".to_string()));
    }
    
    let new_status = if req.approved { "active" } else { "revoked" };
    
    db::update_parent_link_status(&pool, req.link_id, new_status, Some(user.id))
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    Ok(StatusCode::OK)
}

/// Get visibility settings
pub async fn get_visibility(
    Extension(user): Extension<crate::models::user::User>,
    State(pool): State<PgPool>,
    Path(link_id): Path<Uuid>,
) -> Result<Json<ParentVisibilitySettings>, (StatusCode, String)> {
    // Verify access
    let links = db::get_parent_links_by_parent(&pool, user.id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    if !links.iter().any(|l| l.id == link_id && l.status == "active") {
        return Err((StatusCode::FORBIDDEN, "Access denied".to_string()));
    }
    
    let settings = db::get_visibility_settings(&pool, link_id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or_else(|| (StatusCode::NOT_FOUND, "Settings not found".to_string()))?;
    
    // Log access
    let student_id = links.iter().find(|l| l.id == link_id).unwrap().student_id;
    db::log_parent_access(&pool, user.id, student_id, "view_visibility", None, None, None, None)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    Ok(Json(settings))
}

/// Update visibility settings (student endpoint)
pub async fn update_visibility(
    Extension(user): Extension<crate::models::user::User>,
    State(pool): State<PgPool>,
    Path(link_id): Path<Uuid>,
    Json(req): Json<UpdateVisibilitySettingsRequest>,
) -> Result<Json<ParentVisibilitySettings>, (StatusCode, String)> {
    // Verify this link belongs to the current user as student
    let links = db::get_parent_links_by_student(&pool, user.id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    if !links.iter().any(|l| l.id == link_id) {
        return Err((StatusCode::FORBIDDEN, "Link not found".to_string()));
    }
    
    let settings = db::update_visibility_settings(&pool, link_id, &req)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    Ok(Json(settings))
}

/// Make fee payment
pub async fn make_fee_payment(
    Extension(user): Extension<crate::models::user::User>,
    State(pool): State<PgPool>,
    Json(req): Json<MakeFeePaymentRequest>,
) -> Result<Json<ParentFeePayment>, (StatusCode, String)> {
    // Verify parent has access to this student
    let links = db::get_parent_links_by_parent(&pool, user.id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    if !links.iter().any(|l| l.student_id == req.student_id && l.status == "active") {
        return Err((StatusCode::FORBIDDEN, "Access denied".to_string()));
    }
    
    let payment = db::create_fee_payment(&pool, &req, user.id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    // TODO: Process payment based on payment_method (M-Pesa, Stripe, etc.)
    
    Ok(Json(payment))
}

// ============================================
// ID CARDS API ROUTES
// ============================================

/// Get my ID card (student endpoint)
pub async fn get_my_id_card(
    Extension(user): Extension<crate::models::user::User>,
    State(pool): State<PgPool>,
) -> Result<Json<StudentIdCard>, (StatusCode, String)> {
    let card = db::get_id_card_by_user(&pool, user.id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or_else(|| (StatusCode::NOT_FOUND, "No ID card found".to_string()))?;
    
    Ok(Json(card))
}

/// Issue ID card (admin endpoint)
pub async fn issue_id_card(
    Extension(user): Extension<crate::models::user::User>,
    State(pool): State<PgPool>,
    Json(req): Json<IssueIdCardRequest>,
) -> Result<Json<StudentIdCard>, (StatusCode, String)> {
    // Generate card number
    let card_number = format!("SID-{}-{:06}", chrono::Utc::now().year(), rand::random::<u32>());
    
    // Generate QR code hash
    let qr_code_hash = format!("{:x}", md5::compute(format!("{}-{}", req.user_id, card_number)));
    
    let card = db::issue_id_card(
        &pool,
        req.user_id,
        &card_number,
        req.card_type.as_deref().unwrap_or("student"),
        req.expiry_date,
        &qr_code_hash,
    )
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    Ok(Json(card))
}

/// Verify ID card via QR code
pub async fn verify_id_card(
    State(pool): State<PgPool>,
    Json(req): Json<VerifyIdCardRequest>,
) -> Result<Json<IdCardVerificationResponse>, (StatusCode, String)> {
    // Decode QR code to get card number or hash
    // For now, assume QR contains card number
    let card = db::get_id_card_by_number(&pool, &req.qr_code)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    match card {
        Some(c) => {
            // Check status
            let valid = c.status == "active";
            let message = if valid {
                "Valid student ID".to_string()
            } else {
                format!("Card status: {}", c.status)
            };
            
            // Log verification
            db::verify_id_card(
                &pool,
                c.id,
                None,
                "qr_scan",
                if valid { "success" } else { "failed" },
                req.verification_context.as_deref(),
                None,
            )
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
            
            // TODO: Get student name and programme from users/enrollments
            
            Ok(Json(IdCardVerificationResponse {
                valid,
                card: Some(c),
                student_name: None,
                programme: None,
                message,
            }))
        }
        None => Ok(Json(IdCardVerificationResponse {
            valid: false,
            card: None,
            student_name: None,
            programme: None,
            message: "Card not found".to_string(),
        })),
    }
}

/// Update card status (admin endpoint)
pub async fn update_card_status(
    Extension(_user): Extension<crate::models::user::User>,
    State(pool): State<PgPool>,
    Path(card_id): Path<Uuid>,
    Json(req): Json<UpdateCardStatusRequest>,
) -> Result<StatusCode, (StatusCode, String)> {
    db::update_card_status(&pool, card_id, &req.status, req.reason.as_deref(), Some(_user.id))
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    Ok(StatusCode::OK)
}

// ============================================
// ALUMNI PORTAL API ROUTES
// ============================================

/// Get alumni dashboard
pub async fn get_alumni_dashboard(
    Extension(user): Extension<crate::models::user::User>,
    State(pool): State<PgPool>,
) -> Result<Json<AlumniDashboardResponse>, (StatusCode, String)> {
    let profile = db::get_alumni_profile(&pool, user.id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or_else(|| (StatusCode::NOT_FOUND, "Alumni profile not found".to_string()))?;
    
    let recent_jobs = db::get_active_jobs(&pool, 10)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    // TODO: Calculate stats and get suggested connections
    let stats = AlumniStats {
        total_alumni: 0,
        alumni_in_network: 0,
        jobs_posted: recent_jobs.len() as i64,
        cpd_courses_available: 0,
        mentorship_connections: 0,
    };
    
    Ok(Json(AlumniDashboardResponse {
        profile,
        stats,
        recent_jobs,
        suggested_connections: vec![],
    }))
}

/// Create/update alumni profile
pub async fn update_alumni_profile(
    Extension(user): Extension<crate::models::user::User>,
    State(pool): State<PgPool>,
    Json(req): Json<UpdateAlumniProfileRequest>,
) -> Result<Json<AlumniProfile>, (StatusCode, String)> {
    // Check if profile exists
    let existing = db::get_alumni_profile(&pool, user.id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    let profile = match existing {
        Some(_) => {
            db::update_alumni_profile(&pool, user.id, &req)
                .await
                .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        }
        None => {
            return Err((StatusCode::BAD_REQUEST, "Create alumni profile first through admin".to_string()));
        }
    };
    
    Ok(Json(profile))
}

/// Search alumni directory
#[derive(Debug, Deserialize)]
pub struct AlumniSearchQuery {
    pub q: Option<String>,
    pub year: Option<i32>,
    pub programme: Option<String>,
    pub location: Option<String>,
    pub limit: Option<i64>,
}

pub async fn search_alumni_directory(
    State(pool): State<PgPool>,
    Query(params): Query<AlumniSearchQuery>,
) -> Result<Json<Vec<AlumniProfile>>, (StatusCode, String)> {
    let results = db::search_alumni(
        &pool,
        params.q.as_deref(),
        params.year,
        params.programme.as_deref(),
        params.location.as_deref(),
        params.limit.unwrap_or(20),
    )
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    Ok(Json(results))
}

/// Post a job (alumni/employer endpoint)
pub async fn post_job(
    Extension(user): Extension<crate::models::user::User>,
    State(pool): State<PgPool>,
    Json(req): Json<CreateAlumniJobRequest>,
) -> Result<Json<AlumniJob>, (StatusCode, String)> {
    let job = db::create_alumni_job(&pool, user.id, &req)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    Ok(Json(job))
}

/// Apply to job
pub async fn apply_to_job(
    Extension(user): Extension<crate::models::user::User>,
    State(pool): State<PgPool>,
    Json(req): Json<ApplyToJobRequest>,
) -> Result<Json<AlumniJobApplication>, (StatusCode, String)> {
    let application = db::apply_to_job(
        &pool,
        req.job_id,
        user.id,
        req.cover_letter.as_deref(),
        req.resume_url.as_deref(),
    )
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    Ok(Json(application))
}

/// Connect with alumni
pub async fn connect_with_alumni(
    Extension(user): Extension<crate::models::user::User>,
    State(pool): State<PgPool>,
    Json(req): Json<ConnectAlumniRequest>,
) -> Result<StatusCode, (StatusCode, String)> {
    db::create_connection_request(
        &pool,
        user.id,
        req.alumni_id,
        req.connection_type.as_deref().unwrap_or("network"),
        req.message.as_deref(),
    )
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    Ok(StatusCode::CREATED)
}

/// Make donation
pub async fn make_donation(
    Extension(user): Extension<crate::models::user::User>,
    State(pool): State<PgPool>,
    Json(req): Json<MakeDonationRequest>,
) -> Result<Json<AlumniDonation>, (StatusCode, String)> {
    let donation = db::create_donation(&pool, user.id, &req)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    // TODO: Process payment
    
    Ok(Json(donation))
}

/// Download transcript (alumni endpoint)
pub async fn download_transcript(
    Extension(_user): Extension<crate::models::user::User>,
    State(_pool): State<PgPool>,
) -> Result<StatusCode, (StatusCode, String)> {
    // TODO: Generate PDF transcript
    // TODO: Increment download count
    // TODO: Log download
    
    Ok(StatusCode::OK)
}

// ============================================
// ROUTER CREATION
// ============================================

/// Create parents portal router
pub fn parents_router() -> Router {
    Router::new()
        .route("/dashboard", get(get_parent_dashboard))
        .route("/link", post(request_parent_link))
        .route("/link/approve", post(approve_parent_link))
        .route("/visibility/:link_id", get(get_visibility))
        .route("/visibility/:link_id", put(update_visibility))
        .route("/payment", post(make_fee_payment))
}

/// Create ID cards router
pub fn id_cards_router() -> Router {
    Router::new()
        .route("/my-card", get(get_my_id_card))
        .route("/issue", post(issue_id_card))
        .route("/verify", post(verify_id_card))
        .route("/:card_id/status", put(update_card_status))
}

/// Create alumni portal router
pub fn alumni_router() -> Router {
    Router::new()
        .route("/dashboard", get(get_alumni_dashboard))
        .route("/profile", put(update_alumni_profile))
        .route("/directory", get(search_alumni_directory))
        .route("/jobs", post(post_job))
        .route("/jobs/apply", post(apply_to_job))
        .route("/connect", post(connect_with_alumni))
        .route("/donate", post(make_donation))
        .route("/transcript", get(download_transcript))
}
