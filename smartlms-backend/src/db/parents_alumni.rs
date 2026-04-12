// Database operations for Module 18 (Parents Portal), Module 23 (ID Cards), Module 24 (Alumni Portal)
use crate::models::parents_alumni::*;
use sqlx::{PgPool, Row};
use uuid::Uuid;

// ============================================
// PARENTS PORTAL DATABASE OPERATIONS
// ============================================

/// Create parent-student link
pub async fn create_parent_link(
    pool: &PgPool,
    parent_id: Uuid,
    student_id: Uuid,
    linkage_type: &str,
) -> Result<ParentStudentLink, sqlx::Error> {
    let id = Uuid::new_v4();
    
    let row = sqlx::query!(
        r#"INSERT INTO parent_student_links (id, parent_id, student_id, linkage_type, status)
           VALUES ($1, $2, $3, $4, 'pending') RETURNING *"#,
        id, parent_id, student_id, linkage_type
    )
    .fetch_one(pool)
    .await?;
    
    Ok(parent_link_from_row(row))
}

/// Get parent links by parent ID
pub async fn get_parent_links_by_parent(
    pool: &PgPool,
    parent_id: Uuid,
) -> Result<Vec<ParentStudentLink>, sqlx::Error> {
    let rows = sqlx::query!(
        r#"SELECT * FROM parent_student_links WHERE parent_id = $1 ORDER BY created_at DESC"#,
        parent_id
    )
    .fetch_all(pool)
    .await?;
    
    Ok(rows.into_iter().map(parent_link_from_row).collect())
}

/// Get parent links by student ID
pub async fn get_parent_links_by_student(
    pool: &PgPool,
    student_id: Uuid,
) -> Result<Vec<ParentStudentLink>, sqlx::Error> {
    let rows = sqlx::query!(
        r#"SELECT * FROM parent_student_links WHERE student_id = $1 ORDER BY created_at DESC"#,
        student_id
    )
    .fetch_all(pool)
    .await?;
    
    Ok(rows.into_iter().map(parent_link_from_row).collect())
}

/// Approve/revoke parent link
pub async fn update_parent_link_status(
    pool: &PgPool,
    link_id: Uuid,
    status: &str,
    revoked_by: Option<Uuid>,
) -> Result<bool, sqlx::Error> {
    let now = chrono::Utc::now();
    
    let rows = sqlx::query!(
        r#"UPDATE parent_student_links 
           SET status = $1, 
               revoked_by = $2,
               revoked_at = CASE WHEN $1 = 'revoked' THEN $3 ELSE revoked_at END,
               student_approved = CASE WHEN $1 = 'active' THEN true ELSE student_approved END,
               approved_at = CASE WHEN $1 = 'active' THEN $3 ELSE approved_at END,
               updated_at = NOW()
           WHERE id = $4"#,
        status, revoked_by, now, link_id
    )
    .execute(pool)
    .await?;
    
    Ok(rows.rows_affected() > 0)
}

/// Get or create visibility settings
pub async fn get_visibility_settings(
    pool: &PgPool,
    link_id: Uuid,
) -> Result<Option<ParentVisibilitySettings>, sqlx::Error> {
    let row = sqlx::query!(
        r#"SELECT * FROM parent_visibility_settings WHERE link_id = $1"#,
        link_id
    )
    .fetch_optional(pool)
    .await?;
    
    Ok(row.map(|r| ParentVisibilitySettings {
        id: r.id,
        link_id: r.link_id,
        enrolled_courses: r.enrolled_courses,
        grades_and_results: r.grades_and_results,
        attendance_records: r.attendance_records,
        exam_timetable: r.exam_timetable,
        fee_balance: r.fee_balance,
        disciplinary_records: r.disciplinary_records,
        coursework_submissions: r.coursework_submissions,
        direct_messaging: r.direct_messaging,
        hidden_courses: r.hidden_courses.as_ref()
            .and_then(|j| serde_json::from_value(j.clone()).ok())
            .unwrap_or_default(),
        updated_at: r.updated_at,
    }))
}

/// Update visibility settings
pub async fn update_visibility_settings(
    pool: &PgPool,
    link_id: Uuid,
    settings: &UpdateVisibilitySettingsRequest,
) -> Result<ParentVisibilitySettings, sqlx::Error> {
    let hidden_courses_json = settings.hidden_courses.as_ref()
        .map(|c| serde_json::to_value(c).unwrap());
    
    let row = sqlx::query!(
        r#"INSERT INTO parent_visibility_settings (link_id, enrolled_courses, grades_and_results,
                   attendance_records, exam_timetable, fee_balance, disciplinary_records,
                   coursework_submissions, direct_messaging, hidden_courses)
           VALUES ($1, COALESCE($2, true), COALESCE($3, true), COALESCE($4, true),
                   COALESCE($5, true), COALESCE($6, true), COALESCE($7, false),
                   COALESCE($8, false), COALESCE($9, false), COALESCE($10, '[]'::jsonb))
           ON CONFLICT (link_id) DO UPDATE SET
               enrolled_courses = EXCLUDED.enrolled_courses,
               grades_and_results = EXCLUDED.grades_and_results,
               attendance_records = EXCLUDED.attendance_records,
               exam_timetable = EXCLUDED.exam_timetable,
               fee_balance = EXCLUDED.fee_balance,
               disciplinary_records = EXCLUDED.disciplinary_records,
               coursework_submissions = EXCLUDED.coursework_submissions,
               direct_messaging = EXCLUDED.direct_messaging,
               hidden_courses = EXCLUDED.hidden_courses,
               updated_at = NOW()
           RETURNING *"#,
        link_id,
        settings.enrolled_courses,
        settings.grades_and_results,
        settings.attendance_records,
        settings.exam_timetable,
        settings.fee_balance,
        settings.disciplinary_records,
        settings.coursework_submissions,
        settings.direct_messaging,
        hidden_courses_json
    )
    .fetch_one(pool)
    .await?;
    
    Ok(ParentVisibilitySettings {
        id: row.id,
        link_id: row.link_id,
        enrolled_courses: row.enrolled_courses,
        grades_and_results: row.grades_and_results,
        attendance_records: row.attendance_records,
        exam_timetable: row.exam_timetable,
        fee_balance: row.fee_balance,
        disciplinary_records: row.disciplinary_records,
        coursework_submissions: row.coursework_submissions,
        direct_messaging: row.direct_messaging,
        hidden_courses: row.hidden_courses.as_ref()
            .and_then(|j| serde_json::from_value(j.clone()).ok())
            .unwrap_or_default(),
        updated_at: row.updated_at,
    })
}

/// Log parent access
pub async fn log_parent_access(
    pool: &PgPool,
    parent_id: Uuid,
    student_id: Uuid,
    action: &str,
    resource_type: Option<&str>,
    resource_id: Option<Uuid>,
    ip_address: Option<&str>,
    user_agent: Option<&str>,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"INSERT INTO parent_access_log (parent_id, student_id, action, resource_type, resource_id, ip_address, user_agent)
           VALUES ($1, $2, $3, $4, $5, $6::inet, $7)"#,
        parent_id, student_id, action, resource_type, resource_id, ip_address, user_agent
    )
    .execute(pool)
    .await?;
    
    Ok(())
}

/// Create fee payment record
pub async fn create_fee_payment(
    pool: &PgPool,
    payment: &MakeFeePaymentRequest,
    parent_id: Uuid,
) -> Result<ParentFeePayment, sqlx::Error> {
    let id = Uuid::new_v4();
    
    let row = sqlx::query!(
        r#"INSERT INTO parent_fee_payments (id, parent_id, student_id, amount, payment_method, status)
           VALUES ($1, $2, $3, $4, $5, 'pending') RETURNING *"#,
        id, parent_id, payment.student_id, payment.amount.to_string(), payment.payment_method
    )
    .fetch_one(pool)
    .await?;
    
    Ok(parent_fee_payment_from_row(row))
}

// ============================================
// ID CARDS DATABASE OPERATIONS
// ============================================

/// Issue new ID card
pub async fn issue_id_card(
    pool: &PgPool,
    user_id: Uuid,
    card_number: &str,
    card_type: &str,
    expiry_date: Option<chrono::NaiveDate>,
    qr_code_hash: &str,
) -> Result<StudentIdCard, sqlx::Error> {
    let id = Uuid::new_v4();
    let issued_date = chrono::Utc::now().date_naive();
    
    let row = sqlx::query!(
        r#"INSERT INTO student_id_cards (id, user_id, card_number, card_type, status, qr_code_hash, issued_date, expiry_date)
           VALUES ($1, $2, $3, $4, 'active', $5, $6, $7) RETURNING *"#,
        id, user_id, card_number, card_type, qr_code_hash, issued_date, expiry_date
    )
    .fetch_one(pool)
    .await?;
    
    Ok(student_id_card_from_row(row))
}

/// Get ID card by user ID
pub async fn get_id_card_by_user(
    pool: &PgPool,
    user_id: Uuid,
) -> Result<Option<StudentIdCard>, sqlx::Error> {
    let row = sqlx::query!(
        r#"SELECT * FROM student_id_cards WHERE user_id = $1 ORDER BY created_at DESC LIMIT 1"#,
        user_id
    )
    .fetch_optional(pool)
    .await?;
    
    Ok(row.map(student_id_card_from_row))
}

/// Get ID card by card number
pub async fn get_id_card_by_number(
    pool: &PgPool,
    card_number: &str,
) -> Result<Option<StudentIdCard>, sqlx::Error> {
    let row = sqlx::query!(
        r#"SELECT * FROM student_id_cards WHERE card_number = $1"#,
        card_number
    )
    .fetch_optional(pool)
    .await?;
    
    Ok(row.map(student_id_card_from_row))
}

/// Verify ID card
pub async fn verify_id_card(
    pool: &PgPool,
    card_id: Uuid,
    verified_by: Option<Uuid>,
    method: &str,
    result: &str,
    context: Option<&str>,
    ip_address: Option<&str>,
) -> Result<IdCardVerification, sqlx::Error> {
    let id = Uuid::new_v4();
    
    // Update verification count and last verified timestamp
    if result == "success" {
        sqlx::query!(
            r#"UPDATE student_id_cards 
               SET verification_count = verification_count + 1,
                   last_verified_at = NOW()
               WHERE id = $1"#,
            card_id
        )
        .execute(pool)
        .await?;
    }
    
    let row = sqlx::query!(
        r#"INSERT INTO id_card_verifications (id, card_id, verified_by, verification_method, verification_result, verification_context, ip_address)
           VALUES ($1, $2, $3, $4, $5, $6, $7::inet) RETURNING *"#,
        id, card_id, verified_by, method, result, context, ip_address
    )
    .fetch_one(pool)
    .await?;
    
    Ok(id_card_verification_from_row(row))
}

/// Update card status
pub async fn update_card_status(
    pool: &PgPool,
    card_id: Uuid,
    new_status: &str,
    reason: Option<&str>,
    triggered_by: Option<Uuid>,
) -> Result<bool, sqlx::Error> {
    // Get current status
    let current = sqlx::query!(
        r#"SELECT status FROM student_id_cards WHERE id = $1"#,
        card_id
    )
    .fetch_optional(pool)
    .await?;
    
    if let Some(current_row) = current {
        // Record transition
        sqlx::query!(
            r#"INSERT INTO card_transitions (card_id, from_status, to_status, transition_reason, triggered_by)
               VALUES ($1, $2, $3, $4, $5)"#,
            card_id, current_row.status, new_status, reason, triggered_by
        )
        .execute(pool)
        .await?;
        
        // Update card status
        let rows = sqlx::query!(
            r#"UPDATE student_id_cards SET status = $1, updated_at = NOW() WHERE id = $2"#,
            new_status, card_id
        )
        .execute(pool)
        .await?;
        
        Ok(rows.rows_affected() > 0)
    } else {
        Ok(false)
    }
}

// ============================================
// ALUMNI PORTAL DATABASE OPERATIONS
// ============================================

/// Create alumni profile
pub async fn create_alumni_profile(
    pool: &PgPool,
    user_id: Uuid,
    req: &CreateAlumniProfileRequest,
) -> Result<AlumniProfile, sqlx::Error> {
    let id = Uuid::new_v4();
    
    let row = sqlx::query!(
        r#"INSERT INTO alumni_profiles (id, user_id, graduation_year, programme, degree_type, final_gpa, honours)
           VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING *"#,
        id, user_id, req.graduation_year, req.programme, req.degree_type, 
        req.final_gpa.as_ref().map(|d| d.to_string()), req.honours
    )
    .fetch_one(pool)
    .await?;
    
    Ok(alumni_profile_from_row(row))
}

/// Get alumni profile by user ID
pub async fn get_alumni_profile(
    pool: &PgPool,
    user_id: Uuid,
) -> Result<Option<AlumniProfile>, sqlx::Error> {
    let row = sqlx::query!(
        r#"SELECT * FROM alumni_profiles WHERE user_id = $1"#,
        user_id
    )
    .fetch_optional(pool)
    .await?;
    
    Ok(row.map(alumni_profile_from_row))
}

/// Update alumni profile
pub async fn update_alumni_profile(
    pool: &PgPool,
    user_id: Uuid,
    req: &UpdateAlumniProfileRequest,
) -> Result<AlumniProfile, sqlx::Error> {
    let skills_json = req.skills.as_ref().map(|s| serde_json::to_value(s).unwrap());
    
    let row = sqlx::query!(
        r#"UPDATE alumni_profiles SET
               current_company = COALESCE($1, current_company),
               current_role = COALESCE($2, current_role),
               industry = COALESCE($3, industry),
               location_city = COALESCE($4, location_city),
               location_country = COALESCE($5, location_country),
               linkedin_url = COALESCE($6, linkedin_url),
               bio = COALESCE($7, bio),
               skills = COALESCE($8::jsonb, skills),
               willing_to_mentor = COALESCE($9, willing_to_mentor),
               available_for_networking = COALESCE($10, available_for_networking),
               profile_visibility = COALESCE($11, profile_visibility),
               employment_updated_at = NOW(),
               updated_at = NOW()
           WHERE user_id = $12 RETURNING *"#,
        req.current_company, req.current_role, req.industry,
        req.location_city, req.location_country, req.linkedin_url,
        req.bio, skills_json, req.willing_to_mentor,
        req.available_for_networking, req.profile_visibility, user_id
    )
    .fetch_one(pool)
    .await?;
    
    Ok(alumni_profile_from_row(row))
}

/// Search alumni
pub async fn search_alumni(
    pool: &PgPool,
    query: Option<&str>,
    graduation_year: Option<i32>,
    programme: Option<&str>,
    location: Option<&str>,
    limit: i64,
) -> Result<Vec<AlumniProfile>, sqlx::Error> {
    let rows = sqlx::query_as!(
        AlumniProfile,
        r#"SELECT * FROM alumni_profiles 
           WHERE ($1::text IS NULL OR name ILIKE '%' || $1 || '%')
             AND ($2::int IS NULL OR graduation_year = $2)
             AND ($3::text IS NULL OR programme ILIKE '%' || $3 || '%')
             AND ($4::text IS NULL OR location_country ILIKE '%' || $4 || '%')
           AND profile_visibility != 'private'
           ORDER BY graduation_year DESC
           LIMIT $5"#,
        query, graduation_year, programme, location, limit
    )
    .fetch_all(pool)
    .await?;
    
    Ok(rows)
}

/// Create alumni job
pub async fn create_alumni_job(
    pool: &PgPool,
    created_by: Uuid,
    req: &CreateAlumniJobRequest,
) -> Result<AlumniJob, sqlx::Error> {
    let id = Uuid::new_v4();
    let requirements_json = serde_json::to_value(&req.requirements).unwrap_or_default();
    
    let row = sqlx::query!(
        r#"INSERT INTO alumni_jobs (id, title, company, description, requirements, location, remote_option,
                                    job_type, salary_min, salary_max, salary_currency, application_deadline,
                                    application_url, application_email, status, created_by)
           VALUES ($1, $2, $3, $4, $5, $6, COALESCE($7, false), $8, $9, $10, $11, $12, $13, $14, 'active', $15)
           RETURNING *"#,
        id, req.title, req.company, req.description, requirements_json,
        req.location, req.remote_option, req.job_type,
        req.salary_min.as_ref().map(|d| d.to_string()),
        req.salary_max.as_ref().map(|d| d.to_string()),
        req.salary_currency.as_deref().unwrap_or("KES"),
        req.application_deadline, req.application_url, req.application_email, created_by
    )
    .fetch_one(pool)
    .await?;
    
    Ok(alumni_job_from_row(row))
}

/// Get active jobs
pub async fn get_active_jobs(
    pool: &PgPool,
    limit: i64,
) -> Result<Vec<AlumniJob>, sqlx::Error> {
    let rows = sqlx::query!(
        r#"SELECT * FROM alumni_jobs WHERE status = 'active' ORDER BY created_at DESC LIMIT $1"#,
        limit
    )
    .fetch_all(pool)
    .await?;
    
    Ok(rows.into_iter().map(alumni_job_from_row).collect())
}

/// Apply to job
pub async fn apply_to_job(
    pool: &PgPool,
    job_id: Uuid,
    applicant_id: Uuid,
    cover_letter: Option<&str>,
    resume_url: Option<&str>,
) -> Result<AlumniJobApplication, sqlx::Error> {
    let id = Uuid::new_v4();
    
    let row = sqlx::query!(
        r#"INSERT INTO alumni_job_applications (id, job_id, applicant_id, cover_letter, resume_url)
           VALUES ($1, $2, $3, $4, $5) RETURNING *"#,
        id, job_id, applicant_id, cover_letter, resume_url
    )
    .fetch_one(pool)
    .await?;
    
    // Increment applications count
    sqlx::query!(
        r#"UPDATE alumni_jobs SET applications_count = applications_count + 1 WHERE id = $1"#,
        job_id
    )
    .execute(pool)
    .await?;
    
    Ok(alumni_job_application_from_row(row))
}

/// Create alumni connection request
pub async fn create_connection_request(
    pool: &PgPool,
    alumni_id_1: Uuid,
    alumni_id_2: Uuid,
    connection_type: &str,
    message: Option<&str>,
) -> Result<AlumniConnection, sqlx::Error> {
    let id = Uuid::new_v4();
    
    let row = sqlx::query!(
        r#"INSERT INTO alumni_connections (id, alumni_id_1, alumni_id_2, connection_type, message)
           VALUES ($1, $2, $3, $4, $5) RETURNING *"#,
        id, alumni_id_1, alumni_id_2, connection_type, message
    )
    .fetch_one(pool)
    .await?;
    
    Ok(alumni_connection_from_row(row))
}

/// Create donation record
pub async fn create_donation(
    pool: &PgPool,
    alumni_id: Uuid,
    req: &MakeDonationRequest,
) -> Result<AlumniDonation, sqlx::Error> {
    let id = Uuid::new_v4();
    
    let row = sqlx::query!(
        r#"INSERT INTO alumni_donations (id, alumni_id, amount, donation_type, fund_designation,
                                         payment_method, anonymous, message, status)
           VALUES ($1, $2, $3, $4, $5, $6, COALESCE($7, false), $8, 'pending') RETURNING *"#,
        id, alumni_id, req.amount.to_string(), req.donation_type.as_deref().unwrap_or("general"),
        req.fund_designation, req.payment_method, req.anonymous, req.message
    )
    .fetch_one(pool)
    .await?;
    
    Ok(alumni_donation_from_row(row))
}

// Helper functions to convert SQLx rows to models

fn parent_link_from_row(row: sqlx::postgres::PgRow) -> ParentStudentLink {
    ParentStudentLink {
        id: row.get("id"),
        parent_id: row.get("parent_id"),
        student_id: row.get("student_id"),
        linkage_type: row.get("linkage_type"),
        status: row.get("status"),
        student_approved: row.get("student_approved"),
        approved_at: row.get("approved_at"),
        revoked_at: row.get("revoked_at"),
        revoked_by: row.get("revoked_by"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    }
}

fn parent_fee_payment_from_row(row: sqlx::postgres::PgRow) -> ParentFeePayment {
    ParentFeePayment {
        id: row.get("id"),
        parent_id: row.get("parent_id"),
        student_id: row.get("student_id"),
        amount: row.get::<String, _>("amount").parse().unwrap_or_default(),
        currency: row.get("currency"),
        payment_method: row.get("payment_method"),
        payment_reference: row.get("payment_reference"),
        mpesa_receipt_number: row.get("mpesa_receipt_number"),
        stripe_charge_id: row.get("stripe_charge_id"),
        status: row.get("status"),
        receipt_url: row.get("receipt_url"),
        processed_at: row.get("processed_at"),
        created_at: row.get("created_at"),
        metadata: row.get("metadata"),
    }
}

fn student_id_card_from_row(row: sqlx::postgres::PgRow) -> StudentIdCard {
    StudentIdCard {
        id: row.get("id"),
        user_id: row.get("user_id"),
        card_number: row.get("card_number"),
        card_type: row.get("card_type"),
        status: row.get("status"),
        qr_code_hash: row.get("qr_code_hash"),
        photo_url: row.get("photo_url"),
        issued_date: row.get("issued_date"),
        expiry_date: row.get("expiry_date"),
        last_verified_at: row.get("last_verified_at"),
        verification_count: row.get("verification_count"),
        printed: row.get("printed"),
        print_batch_id: row.get("print_batch_id"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    }
}

fn id_card_verification_from_row(row: sqlx::postgres::PgRow) -> IdCardVerification {
    IdCardVerification {
        id: row.get("id"),
        card_id: row.get("card_id"),
        verified_by: row.get("verified_by"),
        verification_method: row.get("verification_method"),
        verification_result: row.get("verification_result"),
        verification_context: row.get("verification_context"),
        ip_address: row.get("ip_address"),
        notes: row.get("notes"),
        created_at: row.get("created_at"),
    }
}

fn alumni_profile_from_row(row: sqlx::postgres::PgRow) -> AlumniProfile {
    AlumniProfile {
        id: row.get("id"),
        user_id: row.get("user_id"),
        graduation_year: row.get("graduation_year"),
        programme: row.get("programme"),
        degree_type: row.get("degree_type"),
        final_gpa: row.get::<Option<String>, _>("final_gpa")
            .and_then(|s| s.parse().ok()),
        honours: row.get("honours"),
        current_company: row.get("current_company"),
        current_role: row.get("current_role"),
        industry: row.get("industry"),
        location_city: row.get("location_city"),
        location_country: row.get("location_country"),
        linkedin_url: row.get("linkedin_url"),
        bio: row.get("bio"),
        skills: row.get::<serde_json::Value, _>("skills")
            .as_array()
            .map(|arr| arr.iter().filter_map(|v| v.as_str().map(String::from)).collect())
            .unwrap_or_default(),
        willing_to_mentor: row.get("willing_to_mentor"),
        available_for_networking: row.get("available_for_networking"),
        profile_visibility: row.get("profile_visibility"),
        transcript_downloaded_count: row.get("transcript_downloaded_count"),
        last_transcript_download: row.get("last_transcript_download"),
        employment_updated_at: row.get("employment_updated_at"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    }
}

fn alumni_job_from_row(row: sqlx::postgres::PgRow) -> AlumniJob {
    AlumniJob {
        id: row.get("id"),
        institution_id: row.get("institution_id"),
        employer_id: row.get("employer_id"),
        title: row.get("title"),
        company: row.get("company"),
        description: row.get("description"),
        requirements: row.get::<serde_json::Value, _>("requirements")
            .as_array()
            .map(|arr| arr.iter().filter_map(|v| v.as_str().map(String::from)).collect())
            .unwrap_or_default(),
        location: row.get("location"),
        remote_option: row.get("remote_option"),
        job_type: row.get("job_type"),
        salary_min: row.get::<Option<String>, _>("salary_min")
            .and_then(|s| s.parse().ok()),
        salary_max: row.get::<Option<String>, _>("salary_max")
            .and_then(|s| s.parse().ok()),
        salary_currency: row.get("salary_currency"),
        application_deadline: row.get("application_deadline"),
        status: row.get("status"),
        application_url: row.get("application_url"),
        application_email: row.get("application_email"),
        views_count: row.get("views_count"),
        applications_count: row.get("applications_count"),
        featured: row.get("featured"),
        featured_until: row.get("featured_until"),
        created_by: row.get("created_by"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    }
}

fn alumni_job_application_from_row(row: sqlx::postgres::PgRow) -> AlumniJobApplication {
    AlumniJobApplication {
        id: row.get("id"),
        job_id: row.get("job_id"),
        applicant_id: row.get("applicant_id"),
        cover_letter: row.get("cover_letter"),
        resume_url: row.get("resume_url"),
        status: row.get("status"),
        applied_at: row.get("applied_at"),
        reviewed_at: row.get("reviewed_at"),
        reviewed_by: row.get("reviewed_by"),
        review_notes: row.get("review_notes"),
    }
}

fn alumni_connection_from_row(row: sqlx::postgres::PgRow) -> AlumniConnection {
    AlumniConnection {
        id: row.get("id"),
        alumni_id_1: row.get("alumni_id_1"),
        alumni_id_2: row.get("alumni_id_2"),
        connection_type: row.get("connection_type"),
        introduced_by: row.get("introduced_by"),
        status: row.get("status"),
        message: row.get("message"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    }
}

fn alumni_donation_from_row(row: sqlx::postgres::PgRow) -> AlumniDonation {
    AlumniDonation {
        id: row.get("id"),
        alumni_id: row.get("alumni_id"),
        amount: row.get::<String, _>("amount").parse().unwrap_or_default(),
        currency: row.get("currency"),
        donation_type: row.get("donation_type"),
        fund_designation: row.get("fund_designation"),
        payment_method: row.get("payment_method"),
        payment_reference: row.get("payment_reference"),
        status: row.get("status"),
        receipt_url: row.get("receipt_url"),
        anonymous: row.get("anonymous"),
        message: row.get("message"),
        created_at: row.get("created_at"),
        processed_at: row.get("processed_at"),
    }
}
