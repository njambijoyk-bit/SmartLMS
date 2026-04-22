// Employer Portal Service - Career services, job board, employer connections, internships, skills matching
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Employer/Company profile
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Employer {
    pub id: uuid::Uuid,
    pub name: String,
    pub description: Option<String>,
    pub industry: String,
    pub website: Option<String>,
    pub logo_url: Option<String>,
    pub size_range: String,
    pub location: String,
    pub is_verified: bool,
    pub contact_email: Option<String>,
    pub contact_phone: Option<String>,
    pub benefits: Vec<String>,
    pub social_links: std::collections::HashMap<String, String>,
    pub created_at: DateTime<Utc>,
}

/// Job posting with enhanced features
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobPosting {
    pub id: uuid::Uuid,
    pub employer_id: uuid::Uuid,
    pub title: String,
    pub description: String,
    pub requirements: Vec<String>,
    pub preferred_skills: Vec<String>,
    pub responsibilities: Vec<String>,
    pub location: String,
    pub remote_option: RemoteOption,
    pub job_type: JobType,
    pub experience_level: String,
    pub salary_range: Option<SalaryRange>,
    pub application_deadline: Option<DateTime<Utc>>,
    pub start_date: Option<DateTime<Utc>>,
    pub positions_available: i32,
    pub is_active: bool,
    pub is_featured: bool,
    pub views_count: i64,
    pub applications_count: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Remote work options
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RemoteOption {
    OnSite,
    Hybrid,
    FullyRemote,
    Flexible,
}

/// Job type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum JobType {
    FullTime,
    PartTime,
    Contract,
    Internship,
    Freelance,
    Apprenticeship,
    CoOp,
}

/// Salary range
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SalaryRange {
    pub min: i64,
    pub max: i64,
    pub currency: String,
    pub period: SalaryPeriod,
}

/// Salary period
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SalaryPeriod {
    Hourly,
    Monthly,
    Yearly,
}

/// Job application with tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobApplication {
    pub id: uuid::Uuid,
    pub job_id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub cover_letter: Option<String>,
    pub resume_url: Option<String>,
    pub portfolio_urls: Vec<String>,
    pub status: ApplicationStatus,
    pub applied_at: DateTime<Utc>,
    pub reviewed_at: Option<DateTime<Utc>>,
    pub interviewed_at: Option<DateTime<Utc>>,
    pub decision_at: Option<DateTime<Utc>>,
    pub employer_notes: Option<String>,
    pub rating: Option<i32>,
}

/// Application status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ApplicationStatus {
    Pending,
    UnderReview,
    Shortlisted,
    Interview,
    Offered,
    Rejected,
    Withdrawn,
    Accepted,
}

/// Internship program
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InternshipProgram {
    pub id: uuid::Uuid,
    pub employer_id: uuid::Uuid,
    pub title: String,
    pub description: String,
    pub department: String,
    pub duration_months: i32,
    pub stipend: Option<SalaryRange>,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub application_deadline: DateTime<Utc>,
    pub positions: i32,
    pub filled_positions: i32,
    pub is_active: bool,
    pub learning_objectives: Vec<String>,
    pub mentorship_provided: bool,
    pub conversion_rate: Option<f64>, // % converted to full-time
    pub created_at: DateTime<Utc>,
}

/// Internship application and tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InternshipApplication {
    pub id: uuid::Uuid,
    pub program_id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub university: Option<String>,
    pub major: Option<String>,
    pub graduation_year: Option<i32>,
    pub gpa: Option<f64>,
    pub cover_letter: Option<String>,
    pub resume_url: Option<String>,
    pub status: ApplicationStatus,
    pub supervisor_id: Option<uuid::Uuid>,
    pub performance_rating: Option<i32>,
    pub completion_status: Option<CompletionStatus>,
    pub applied_at: DateTime<Utc>,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
}

/// Completion status for internships
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CompletionStatus {
    InProgress,
    Completed,
    Failed,
    Discontinued,
}

/// Skills match result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillsMatch {
    pub user_id: uuid::Uuid,
    pub job_id: uuid::Uuid,
    pub match_score: f64, // 0-100
    pub matched_skills: Vec<String>,
    pub missing_skills: Vec<String>,
    pub transferable_skills: Vec<String>,
    pub recommendation: String,
}

/// Candidate profile for employer viewing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CandidateProfile {
    pub user_id: uuid::Uuid,
    pub name: String,
    pub headline: Option<String>,
    pub skills: Vec<String>,
    pub education: Vec<EducationEntry>,
    pub experience: Vec<ExperienceEntry>,
    pub certifications: Vec<String>,
    pub portfolio_url: Option<String>,
    pub availability: AvailabilityStatus,
    pub preferred_roles: Vec<String>,
    pub preferred_locations: Vec<String>,
    pub match_scores: std::collections::HashMap<uuid::Uuid, f64>, // job_id -> score
}

/// Education entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EducationEntry {
    pub institution: String,
    pub degree: String,
    pub field_of_study: String,
    pub graduation_year: Option<i32>,
    pub gpa: Option<f64>,
}

/// Experience entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExperienceEntry {
    pub company: String,
    pub position: String,
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
    pub description: Option<String>,
    pub skills_used: Vec<String>,
}

/// Availability status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AvailabilityStatus {
    Immediately,
    TwoWeeks,
    OneMonth,
    NotAvailable,
}

/// Career resource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CareerResource {
    pub id: uuid::Uuid,
    pub title: String,
    pub content: String,
    pub resource_type: ResourceType,
    pub category: String,
    pub tags: Vec<String>,
    pub author: Option<String>,
    pub is_published: bool,
    pub view_count: i64,
    pub created_at: DateTime<Utc>,
}

/// Resource type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResourceType {
    Article,
    Video,
    Template,
    Guide,
    Webinar,
    Workshop,
    CaseStudy,
}

/// Campus recruitment event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecruitmentEvent {
    pub id: uuid::Uuid,
    pub employer_id: uuid::Uuid,
    pub institution_id: uuid::Uuid,
    pub event_type: EventType,
    pub title: String,
    pub description: String,
    pub venue: String,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub max_attendees: i32,
    pub registered_count: i32,
    pub is_virtual: bool,
    pub meeting_link: Option<String>,
    pub status: EventStatus,
    pub target_majors: Vec<String>,
    pub target_graduation_years: Vec<i32>,
    pub created_at: DateTime<Utc>,
}

/// Event type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EventType {
    InfoSession,
    CareerFair,
    TechnicalWorkshop,
    NetworkingMixer,
    OnCampusInterview,
    Hackathon,
}

/// Event status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EventStatus {
    Scheduled,
    Ongoing,
    Completed,
    Cancelled,
}

/// Industry partnership
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndustryPartnership {
    pub id: uuid::Uuid,
    pub employer_id: uuid::Uuid,
    pub institution_id: uuid::Uuid,
    pub partnership_type: PartnershipType,
    pub title: String,
    pub description: String,
    pub start_date: DateTime<Utc>,
    pub end_date: Option<DateTime<Utc>>,
    pub benefits: Vec<String>,
    pub commitments: Vec<String>,
    pub status: PartnershipStatus,
    pub created_at: DateTime<Utc>,
}

/// Partnership type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PartnershipType {
    ResearchCollaboration,
    CurriculumAdvisory,
    InternshipPipeline,
    Sponsorship,
    JointLab,
    GuestLectures,
}

/// Partnership status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PartnershipStatus {
    Active,
    Pending,
    Expired,
    Terminated,
}

// Service functions
pub mod service {
    use super::*;
    use sqlx::PgPool;

    /// Register employer (company)
    pub async fn register_employer(
        pool: &PgPool,
        req: &RegisterEmployerRequest,
    ) -> Result<Employer, String> {
        let id = Uuid::new_v4();

        sqlx::query!(
            "INSERT INTO employers (id, name, description, industry, website, logo_url, 
             size_range, location, is_verified, contact_email, contact_phone, benefits, 
             social_links, created_at)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, false, $9, $10, $11, $12, $13)",
            id,
            req.name,
            req.description,
            req.industry,
            req.website,
            req.logo_url,
            req.size_range,
            req.location,
            req.contact_email,
            req.contact_phone,
            serde_json::to_string(&req.benefits).unwrap(),
            serde_json::to_string(&req.social_links).unwrap(),
            Utc::now()
        )
        .execute(pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(Employer {
            id,
            name: req.name.clone(),
            description: req.description.clone(),
            industry: req.industry.clone(),
            website: req.website.clone(),
            logo_url: req.logo_url.clone(),
            size_range: req.size_range.clone(),
            location: req.location.clone(),
            is_verified: false,
            contact_email: req.contact_email.clone(),
            contact_phone: req.contact_phone.clone(),
            benefits: req.benefits.clone(),
            social_links: req.social_links.clone(),
            created_at: Utc::now(),
        })
    }

    /// Post a new job
    pub async fn post_job(
        pool: &PgPool,
        employer_id: uuid::Uuid,
        req: &PostJobRequest,
    ) -> Result<JobPosting, String> {
        let id = Uuid::new_v4();

        let salary_min = req.salary_range.as_ref().map(|s| s.min);
        let salary_max = req.salary_range.as_ref().map(|s| s.max);
        let salary_currency = req.salary_range.as_ref().map(|s| s.currency.as_str());
        let salary_period = req.salary_range.as_ref().map(|s| format!("{:?}", s.period).to_lowercase());

        sqlx::query!(
            "INSERT INTO job_postings (id, employer_id, title, description, requirements, 
             preferred_skills, responsibilities, location, remote_option, job_type, 
             experience_level, salary_min, salary_max, salary_currency, salary_period,
             application_deadline, start_date, positions_available, is_active, is_featured, 
             views_count, applications_count, created_at, updated_at)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, true, false, 0, 0, $19, $20)",
            id,
            employer_id,
            req.title,
            req.description,
            serde_json::to_string(&req.requirements).unwrap(),
            serde_json::to_string(&req.preferred_skills).unwrap(),
            serde_json::to_string(&req.responsibilities).unwrap(),
            req.location,
            format!("{:?}", req.remote_option).to_lowercase(),
            format!("{:?}", req.job_type).to_lowercase(),
            req.experience_level,
            salary_min,
            salary_max,
            salary_currency,
            salary_period,
            req.application_deadline,
            req.start_date,
            req.positions_available,
            Utc::now(),
            Utc::now()
        )
        .execute(pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(JobPosting {
            id,
            employer_id,
            title: req.title.clone(),
            description: req.description.clone(),
            requirements: req.requirements.clone(),
            preferred_skills: req.preferred_skills.clone(),
            responsibilities: req.responsibilities.clone(),
            location: req.location.clone(),
            remote_option: req.remote_option,
            job_type: req.job_type,
            experience_level: req.experience_level.clone(),
            salary_range: req.salary_range.clone(),
            application_deadline: req.application_deadline,
            start_date: req.start_date,
            positions_available: req.positions_available,
            is_active: true,
            is_featured: false,
            views_count: 0,
            applications_count: 0,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        })
    }

    /// Apply for a job
    pub async fn apply_for_job(
        pool: &PgPool,
        job_id: uuid::Uuid,
        user_id: uuid::Uuid,
        cover_letter: Option<&str>,
        resume_url: Option<&str>,
        portfolio_urls: Vec<String>,
    ) -> Result<JobApplication, String> {
        // Check if already applied
        let existing = sqlx::query!(
            "SELECT id FROM job_applications WHERE job_id = $1 AND user_id = $2",
            job_id,
            user_id
        )
        .fetch_optional(pool)
        .await
        .map_err(|e| e.to_string())?;

        if existing.is_some() {
            return Err("You have already applied for this job".to_string());
        }

        let id = Uuid::new_v4();

        sqlx::query!(
            "INSERT INTO job_applications (id, job_id, user_id, cover_letter, resume_url, 
             portfolio_urls, status, applied_at)
             VALUES ($1, $2, $3, $4, $5, $6, 'pending', $7)",
            id,
            job_id,
            user_id,
            cover_letter,
            resume_url,
            serde_json::to_string(&portfolio_urls).unwrap(),
            Utc::now()
        )
        .execute(pool)
        .await
        .map_err(|e| e.to_string())?;

        // Increment application count
        sqlx::query!(
            "UPDATE job_postings SET applications_count = applications_count + 1 WHERE id = $1",
            job_id
        )
        .execute(pool)
        .await
        .ok();

        Ok(JobApplication {
            id,
            job_id,
            user_id,
            cover_letter: cover_letter.map(String::from),
            resume_url: resume_url.map(String::from),
            portfolio_urls,
            status: ApplicationStatus::Pending,
            applied_at: Utc::now(),
            reviewed_at: None,
            interviewed_at: None,
            decision_at: None,
            employer_notes: None,
            rating: None,
        })
    }

    /// Calculate skills match between candidate and job
    pub async fn calculate_skills_match(
        pool: &PgPool,
        user_id: uuid::Uuid,
        job_id: uuid::Uuid,
    ) -> Result<SkillsMatch, String> {
        // Get user skills from profile
        let user_skills_row = sqlx::query!(
            "SELECT skills FROM user_profiles WHERE user_id = $1",
            user_id
        )
        .fetch_optional(pool)
        .await
        .map_err(|e| e.to_string())?;

        let user_skills: Vec<String> = user_skills_row
            .and_then(|r| r.skills)
            .map(|s| serde_json::from_str(&s).unwrap_or_default())
            .unwrap_or_default();

        // Get job requirements
        let job_row = sqlx::query!(
            "SELECT requirements, preferred_skills FROM job_postings WHERE id = $1",
            job_id
        )
        .fetch_optional(pool)
        .await
        .map_err(|e| e.to_string())?;

        if job_row.is_none() {
            return Err("Job not found".to_string());
        }

        let job_row = job_row.unwrap();
        let required_skills: Vec<String> = serde_json::from_str(&job_row.requirements).unwrap_or_default();
        let preferred_skills: Vec<String> = serde_json::from_str(&job_row.preferred_skills).unwrap_or_default();

        let all_job_skills: Vec<String> = [required_skills.clone(), preferred_skills].concat();
        
        // Calculate matches
        let mut matched_skills = Vec::new();
        let mut missing_skills = Vec::new();
        let mut transferable_skills = Vec::new();

        for skill in &all_job_skills {
            if user_skills.iter().any(|s| s.to_lowercase() == skill.to_lowercase()) {
                matched_skills.push(skill.clone());
            } else {
                // Check for transferable skills (simplified logic)
                if is_transferable(skill, &user_skills) {
                    transferable_skills.push(skill.clone());
                } else {
                    missing_skills.push(skill.clone());
                }
            }
        }

        // Calculate match score
        let total_skills = all_job_skills.len() as f64;
        let matched_count = matched_skills.len() as f64;
        let transferable_count = transferable_skills.len() as f64 * 0.5; // Partial credit
        
        let match_score = if total_skills > 0.0 {
            ((matched_count + transferable_count) / total_skills * 100.0).min(100.0)
        } else {
            0.0
        };

        // Generate recommendation
        let recommendation = if match_score >= 80.0 {
            "Excellent match! Strong candidate for this role.".to_string()
        } else if match_score >= 60.0 {
            "Good match. Consider highlighting transferable skills.".to_string()
        } else if match_score >= 40.0 {
            "Moderate match. May need additional training or experience.".to_string()
        } else {
            "Low match. Consider building required skills before applying.".to_string()
        };

        Ok(SkillsMatch {
            user_id,
            job_id,
            match_score,
            matched_skills,
            missing_skills,
            transferable_skills,
            recommendation,
        })
    }

    /// Helper function to check transferable skills (simplified)
    fn is_transferable(skill: &str, user_skills: &[String]) -> bool {
        let skill_lower = skill.to_lowercase();
        // Simple keyword-based transferability check
        let transferable_keywords = ["communication", "leadership", "teamwork", "analysis", "problem solving", "project management"];
        
        if transferable_keywords.iter().any(|k| skill_lower.contains(k)) {
            return true;
        }
        
        // Check if similar skill exists
        user_skills.iter().any(|s| {
            let s_lower = s.to_lowercase();
            s_lower.contains(&skill_lower[..skill_lower.len().min(4)])
        })
    }

    /// Create internship program
    pub async fn create_internship_program(
        pool: &PgPool,
        employer_id: uuid::Uuid,
        req: &CreateInternshipRequest,
    ) -> Result<InternshipProgram, String> {
        let id = Uuid::new_v4();

        let stipend_min = req.stipend.as_ref().map(|s| s.min);
        let stipend_max = req.stipend.as_ref().map(|s| s.max);
        let stipend_currency = req.stipend.as_ref().map(|s| s.currency.as_str());
        let stipend_period = req.stipend.as_ref().map(|s| format!("{:?}", s.period).to_lowercase());

        sqlx::query!(
            "INSERT INTO internship_programs (id, employer_id, title, description, department,
             duration_months, stipend_min, stipend_max, stipend_currency, stipend_period,
             start_date, end_date, application_deadline, positions, filled_positions, is_active,
             learning_objectives, mentorship_provided, conversion_rate, created_at)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, 0, true, $15, $16, $17, $18)",
            id,
            employer_id,
            req.title,
            req.description,
            req.department,
            req.duration_months,
            stipend_min,
            stipend_max,
            stipend_currency,
            stipend_period,
            req.start_date,
            req.end_date,
            req.application_deadline,
            req.positions,
            serde_json::to_string(&req.learning_objectives).unwrap(),
            req.mentorship_provided,
            req.conversion_rate,
            Utc::now()
        )
        .execute(pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(InternshipProgram {
            id,
            employer_id,
            title: req.title.clone(),
            description: req.description.clone(),
            department: req.department.clone(),
            duration_months: req.duration_months,
            stipend: req.stipend.clone(),
            start_date: req.start_date,
            end_date: req.end_date,
            application_deadline: req.application_deadline,
            positions: req.positions,
            filled_positions: 0,
            is_active: true,
            learning_objectives: req.learning_objectives.clone(),
            mentorship_provided: req.mentorship_provided,
            conversion_rate: req.conversion_rate,
            created_at: Utc::now(),
        })
    }

    /// Apply for internship
    pub async fn apply_for_internship(
        pool: &PgPool,
        program_id: uuid::Uuid,
        user_id: uuid::Uuid,
        req: &InternshipApplicationRequest,
    ) -> Result<InternshipApplication, String> {
        let id = Uuid::new_v4();

        sqlx::query!(
            "INSERT INTO internship_applications (id, program_id, user_id, university, major,
             graduation_year, gpa, cover_letter, resume_url, status, applied_at)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, 'pending', $10)",
            id,
            program_id,
            user_id,
            req.university,
            req.major,
            req.graduation_year,
            req.gpa,
            req.cover_letter,
            req.resume_url,
            Utc::now()
        )
        .execute(pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(InternshipApplication {
            id,
            program_id,
            user_id,
            university: req.university.clone(),
            major: req.major.clone(),
            graduation_year: req.graduation_year,
            gpa: req.gpa,
            cover_letter: req.cover_letter.clone(),
            resume_url: req.resume_url.clone(),
            status: ApplicationStatus::Pending,
            supervisor_id: None,
            performance_rating: None,
            completion_status: None,
            applied_at: Utc::now(),
            started_at: None,
            completed_at: None,
        })
    }

    /// Create recruitment event
    pub async fn create_recruitment_event(
        pool: &PgPool,
        employer_id: uuid::Uuid,
        institution_id: uuid::Uuid,
        req: &CreateEventRequest,
    ) -> Result<RecruitmentEvent, String> {
        let id = Uuid::new_v4();

        sqlx::query!(
            "INSERT INTO recruitment_events (id, employer_id, institution_id, event_type, title,
             description, venue, start_time, end_time, max_attendees, registered_count, is_virtual,
             meeting_link, status, target_majors, target_graduation_years, created_at)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, 0, $11, $12, 'scheduled', $13, $14, $15)",
            id,
            employer_id,
            institution_id,
            format!("{:?}", req.event_type).to_lowercase(),
            req.title,
            req.description,
            req.venue,
            req.start_time,
            req.end_time,
            req.max_attendees,
            req.is_virtual,
            req.meeting_link,
            serde_json::to_string(&req.target_majors).unwrap(),
            serde_json::to_string(&req.target_graduation_years).unwrap(),
            Utc::now()
        )
        .execute(pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(RecruitmentEvent {
            id,
            employer_id,
            institution_id,
            event_type: req.event_type,
            title: req.title.clone(),
            description: req.description.clone(),
            venue: req.venue.clone(),
            start_time: req.start_time,
            end_time: req.end_time,
            max_attendees: req.max_attendees,
            registered_count: 0,
            is_virtual: req.is_virtual,
            meeting_link: req.meeting_link.clone(),
            status: EventStatus::Scheduled,
            target_majors: req.target_majors.clone(),
            target_graduation_years: req.target_graduation_years.clone(),
            created_at: Utc::now(),
        })
    }

    /// Create industry partnership
    pub async fn create_partnership(
        pool: &PgPool,
        employer_id: uuid::Uuid,
        institution_id: uuid::Uuid,
        req: &CreatePartnershipRequest,
    ) -> Result<IndustryPartnership, String> {
        let id = Uuid::new_v4();

        sqlx::query!(
            "INSERT INTO industry_partnerships (id, employer_id, institution_id, partnership_type,
             title, description, start_date, end_date, benefits, commitments, status, created_at)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, 'active', $11)",
            id,
            employer_id,
            institution_id,
            format!("{:?}", req.partnership_type).to_lowercase(),
            req.title,
            req.description,
            req.start_date,
            req.end_date,
            serde_json::to_string(&req.benefits).unwrap(),
            serde_json::to_string(&req.commitments).unwrap(),
            Utc::now()
        )
        .execute(pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(IndustryPartnership {
            id,
            employer_id,
            institution_id,
            partnership_type: req.partnership_type,
            title: req.title.clone(),
            description: req.description.clone(),
            start_date: req.start_date,
            end_date: req.end_date,
            benefits: req.benefits.clone(),
            commitments: req.commitments.clone(),
            status: PartnershipStatus::Active,
            created_at: Utc::now(),
        })
    }

    /// Get jobs for employer
    pub async fn get_employer_jobs(
        pool: &PgPool,
        employer_id: uuid::Uuid,
    ) -> Result<Vec<JobPosting>, String> {
        let rows = sqlx::query!(
            "SELECT id, employer_id, title, description, requirements, preferred_skills, 
             responsibilities, location, remote_option, job_type, experience_level, 
             salary_min, salary_max, salary_currency, salary_period, application_deadline,
             start_date, positions_available, is_active, is_featured, views_count, 
             applications_count, created_at, updated_at
             FROM job_postings WHERE employer_id = $1 ORDER BY created_at DESC",
            employer_id
        )
        .fetch_all(pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(rows
            .into_iter()
            .map(|r| JobPosting {
                id: r.id,
                employer_id: r.employer_id,
                title: r.title,
                description: r.description,
                requirements: serde_json::from_str(&r.requirements).unwrap_or_default(),
                preferred_skills: serde_json::from_str(&r.preferred_skills).unwrap_or_default(),
                responsibilities: serde_json::from_str(&r.responsibilities).unwrap_or_default(),
                location: r.location,
                remote_option: RemoteOption::OnSite,
                job_type: JobType::FullTime,
                experience_level: r.experience_level,
                salary_range: if r.salary_min.is_some() {
                    Some(SalaryRange {
                        min: r.salary_min.unwrap(),
                        max: r.salary_max.unwrap_or(0),
                        currency: r.salary_currency.unwrap_or_else(|| "USD".to_string()),
                        period: SalaryPeriod::Yearly,
                    })
                } else {
                    None
                },
                application_deadline: r.application_deadline,
                start_date: r.start_date,
                positions_available: r.positions_available,
                is_active: r.is_active,
                is_featured: r.is_featured,
                views_count: r.views_count,
                applications_count: r.applications_count,
                created_at: r.created_at,
                updated_at: r.updated_at,
            })
            .collect())
    }

    /// Search jobs with advanced filters
    pub async fn search_jobs(
        pool: &PgPool,
        query: &str,
        location: Option<&str>,
        job_type: Option<JobType>,
        remote_option: Option<RemoteOption>,
        experience_level: Option<&str>,
        min_salary: Option<i64>,
        limit: i64,
    ) -> Result<Vec<JobPosting>, String> {
        let search_pattern = format!("%{}%", query);

        let rows = sqlx::query!(
            "SELECT id, employer_id, title, description, requirements, preferred_skills, 
             responsibilities, location, remote_option, job_type, experience_level, 
             salary_min, salary_max, salary_currency, salary_period, application_deadline,
             start_date, positions_available, is_active, is_featured, views_count, 
             applications_count, created_at, updated_at
             FROM job_postings 
             WHERE is_active = true 
             AND (title ILIKE $1 OR description ILIKE $1)
             ORDER BY created_at DESC LIMIT $2",
            search_pattern,
            limit
        )
        .fetch_all(pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(rows
            .into_iter()
            .map(|r| JobPosting {
                id: r.id,
                employer_id: r.employer_id,
                title: r.title,
                description: r.description,
                requirements: serde_json::from_str(&r.requirements).unwrap_or_default(),
                preferred_skills: serde_json::from_str(&r.preferred_skills).unwrap_or_default(),
                responsibilities: serde_json::from_str(&r.responsibilities).unwrap_or_default(),
                location: r.location,
                remote_option: RemoteOption::OnSite,
                job_type: JobType::FullTime,
                experience_level: r.experience_level,
                salary_range: None,
                application_deadline: r.application_deadline,
                start_date: r.start_date,
                positions_available: r.positions_available,
                is_active: r.is_active,
                is_featured: r.is_featured,
                views_count: r.views_count,
                applications_count: r.applications_count,
                created_at: r.created_at,
                updated_at: r.updated_at,
            })
            .collect())
    }

    /// Update application status
    pub async fn update_application_status(
        pool: &PgPool,
        application_id: uuid::Uuid,
        status: ApplicationStatus,
        employer_notes: Option<&str>,
        rating: Option<i32>,
    ) -> Result<(), String> {
        let now = Utc::now();
        
        sqlx::query!(
            "UPDATE job_applications SET status = $1, employer_notes = $2, rating = $3, 
             reviewed_at = CASE WHEN $1 != 'pending' THEN $4 ELSE reviewed_at END,
             decision_at = CASE WHEN $1 IN ('offered', 'rejected') THEN $4 ELSE decision_at END
             WHERE id = $5",
            format!("{:?}", status).to_lowercase(),
            employer_notes,
            rating,
            now,
            application_id
        )
        .execute(pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(())
    }

    /// Create career resource
    pub async fn create_resource(
        pool: &PgPool,
        title: &str,
        content: &str,
        resource_type: ResourceType,
        category: &str,
        tags: Vec<String>,
        author: Option<&str>,
    ) -> Result<CareerResource, String> {
        let id = Uuid::new_v4();

        sqlx::query!(
            "INSERT INTO career_resources (id, title, content, resource_type, category, 
             tags, author, is_published, view_count, created_at)
             VALUES ($1, $2, $3, $4, $5, $6, $7, true, 0, $8)",
            id, title, content, format!("{:?}", resource_type).to_lowercase(), 
            category, serde_json::to_string(&tags).unwrap(), author, Utc::now()
        )
        .execute(pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(CareerResource {
            id,
            title: title.to_string(),
            content: content.to_string(),
            resource_type,
            category: category.to_string(),
            tags,
            author: author.map(String::from),
            is_published: true,
            view_count: 0,
            created_at: Utc::now(),
        })
    }

    /// Get candidate profile for employer
    pub async fn get_candidate_profile(
        pool: &PgPool,
        user_id: uuid::Uuid,
    ) -> Result<Option<CandidateProfile>, String> {
        let row = sqlx::query!(
            "SELECT user_id, name, headline, skills, education, experience, certifications,
             portfolio_url, availability, preferred_roles, preferred_locations
             FROM candidate_profiles WHERE user_id = $1",
            user_id
        )
        .fetch_optional(pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(row.map(|r| CandidateProfile {
            user_id: r.user_id,
            name: r.name,
            headline: r.headline,
            skills: serde_json::from_str(&r.skills).unwrap_or_default(),
            education: serde_json::from_str(&r.education).unwrap_or_default(),
            experience: serde_json::from_str(&r.experience).unwrap_or_default(),
            certifications: serde_json::from_str(&r.certifications).unwrap_or_default(),
            portfolio_url: r.portfolio_url,
            availability: AvailabilityStatus::Immediately,
            preferred_roles: serde_json::from_str(&r.preferred_roles).unwrap_or_default(),
            preferred_locations: serde_json::from_str(&r.preferred_locations).unwrap_or_default(),
            match_scores: std::collections::HashMap::new(),
        }))
    }
}

#[derive(Debug, Deserialize)]
pub struct RegisterEmployerRequest {
    pub name: String,
    pub description: Option<String>,
    pub industry: String,
    pub website: Option<String>,
    pub logo_url: Option<String>,
    pub size_range: String,
    pub location: String,
    pub contact_email: Option<String>,
    pub contact_phone: Option<String>,
    pub benefits: Vec<String>,
    pub social_links: std::collections::HashMap<String, String>,
}

#[derive(Debug, Deserialize)]
pub struct PostJobRequest {
    pub title: String,
    pub description: String,
    pub requirements: Vec<String>,
    pub preferred_skills: Vec<String>,
    pub responsibilities: Vec<String>,
    pub location: String,
    pub remote_option: RemoteOption,
    pub job_type: JobType,
    pub experience_level: String,
    pub salary_range: Option<SalaryRange>,
    pub application_deadline: Option<DateTime<Utc>>,
    pub start_date: Option<DateTime<Utc>>,
    pub positions_available: i32,
}

#[derive(Debug, Deserialize)]
pub struct InternshipApplicationRequest {
    pub university: Option<String>,
    pub major: Option<String>,
    pub graduation_year: Option<i32>,
    pub gpa: Option<f64>,
    pub cover_letter: Option<String>,
    pub resume_url: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateInternshipRequest {
    pub title: String,
    pub description: String,
    pub department: String,
    pub duration_months: i32,
    pub stipend: Option<SalaryRange>,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub application_deadline: DateTime<Utc>,
    pub positions: i32,
    pub learning_objectives: Vec<String>,
    pub mentorship_provided: bool,
    pub conversion_rate: Option<f64>,
}

#[derive(Debug, Deserialize)]
pub struct CreateEventRequest {
    pub event_type: EventType,
    pub title: String,
    pub description: String,
    pub venue: String,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub max_attendees: i32,
    pub is_virtual: bool,
    pub meeting_link: Option<String>,
    pub target_majors: Vec<String>,
    pub target_graduation_years: Vec<i32>,
}

#[derive(Debug, Deserialize)]
pub struct CreatePartnershipRequest {
    pub partnership_type: PartnershipType,
    pub title: String,
    pub description: String,
    pub start_date: DateTime<Utc>,
    pub end_date: Option<DateTime<Utc>>,
    pub benefits: Vec<String>,
    pub commitments: Vec<String>,
}
