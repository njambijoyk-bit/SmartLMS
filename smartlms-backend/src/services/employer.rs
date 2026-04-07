// Employer Portal Service - Career services, job board, employer connections
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

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
    pub created_at: DateTime<Utc>,
}

/// Job posting
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobPosting {
    pub id: uuid::Uuid,
    pub employer_id: uuid::Uuid,
    pub title: String,
    pub description: String,
    pub requirements: Vec<String>,
    pub location: String,
    pub job_type: JobType,
    pub experience_level: String,
    pub salary_range: Option<SalaryRange>,
    pub application_deadline: Option<DateTime<Utc>>,
    pub is_active: bool,
    pub views_count: i64,
    pub applications_count: i64,
    pub created_at: DateTime<Utc>,
}

/// Job type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum JobType {
    FullTime,
    PartTime,
    Contract,
    Internship,
    Freelance,
}

/// Salary range
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SalaryRange {
    pub min: i64,
    pub max: i64,
    pub currency: String,
}

/// Job application
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobApplication {
    pub id: uuid::Uuid,
    pub job_id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub cover_letter: Option<String>,
    pub resume_url: Option<String>,
    pub status: ApplicationStatus,
    pub applied_at: DateTime<Utc>,
    pub reviewed_at: Option<DateTime<Utc>>,
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
}

/// Career resource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CareerResource {
    pub id: uuid::Uuid,
    pub title: String,
    pub content: String,
    pub resource_type: ResourceType,
    pub category: String,
    pub is_published: bool,
    pub created_at: DateTime<Utc>,
}

/// Resource type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResourceType {
    Article,
    Video,
    Template,
    Guide,
}

// Service functions
pub mod service {
    use super::*;
    
    /// Register employer (company)
    pub async fn register_employer(
        pool: &PgPool,
        req: &RegisterEmployerRequest,
    ) -> Result<Employer, String> {
        let id = Uuid::new_v4();
        
        sqlx::query!(
            "INSERT INTO employers (id, name, description, industry, website, logo_url, 
             size_range, location, is_verified, created_at)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, false, $9)",
            id, req.name, req.description, req.industry, req.website, req.logo_url,
            req.size_range, req.location, Utc::now()
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
        
        sqlx::query!(
            "INSERT INTO job_postings (id, employer_id, title, description, requirements, 
             location, job_type, experience_level, salary_min, salary_max, salary_currency,
             application_deadline, is_active, views_count, applications_count, created_at)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, true, 0, 0, $13)",
            id, employer_id, req.title, req.description, serde_json::to_string(&req.requirements).unwrap(),
            req.location, format!("{:?}", req.job_type).to_lowercase(), req.experience_level,
            salary_min, salary_max, salary_currency, req.application_deadline, Utc::now()
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
            location: req.location.clone(),
            job_type: req.job_type,
            experience_level: req.experience_level.clone(),
            salary_range: req.salary_range.clone(),
            application_deadline: req.application_deadline,
            is_active: true,
            views_count: 0,
            applications_count: 0,
            created_at: Utc::now(),
        })
    }
    
    /// Apply for a job
    pub async fn apply_for_job(
        pool: &PgPool,
        job_id: uuid::Uuid,
        user_id: uuid::Uuid,
        cover_letter: Option<&str>,
    ) -> Result<JobApplication, String> {
        // Check if already applied
        let existing = sqlx::query!(
            "SELECT id FROM job_applications WHERE job_id = $1 AND user_id = $2",
            job_id, user_id
        )
        .fetch_optional(pool)
        .await
        .map_err(|e| e.to_string())?;
        
        if existing.is_some() {
            return Err("You have already applied for this job".to_string());
        }
        
        let id = Uuid::new_v4();
        
        sqlx::query!(
            "INSERT INTO job_applications (id, job_id, user_id, cover_letter, status, applied_at)
             VALUES ($1, $2, $3, $4, 'pending', $5)",
            id, job_id, user_id, cover_letter, Utc::now()
        )
        .execute(pool)
        .await
        .map_err(|e| e.to_string())?;
        
        // Increment application count
        sqlx::query!(
            "UPDATE job_posting SET applications_count = applications_count + 1 WHERE id = $1",
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
            resume_url: None,
            status: ApplicationStatus::Pending,
            applied_at: Utc::now(),
            reviewed_at: None,
        })
    }
    
    /// Get jobs for employer
    pub async fn get_employer_jobs(
        pool: &PgPool,
        employer_id: uuid::Uuid,
    ) -> Result<Vec<JobPosting>, String> {
        let rows = sqlx::query!(
            "SELECT id, employer_id, title, description, requirements, location, job_type,
             experience_level, salary_min, salary_max, salary_currency, application_deadline,
             is_active, views_count, applications_count, created_at
             FROM job_posting WHERE employer_id = $1 ORDER BY created_at DESC",
            employer_id
        )
        .fetch_all(pool)
        .await
        .map_err(|e| e.to_string())?;
        
        Ok(rows.into_iter().map(|r| JobPosting {
            id: r.id,
            employer_id: r.employer_id,
            title: r.title,
            description: r.description,
            requirements: serde_json::from_str(&r.requirements).unwrap_or_default(),
            location: r.location,
            job_type: JobType::FullTime,
            experience_level: r.experience_level,
            salary_range: if r.salary_min.is_some() {
                Some(SalaryRange {
                    min: r.salary_min.unwrap(),
                    max: r.salary_max.unwrap_or(0),
                    currency: r.salary_currency.unwrap_or_else(|| "USD".to_string()),
                })
            } else { None },
            application_deadline: r.application_deadline,
            is_active: r.is_active,
            views_count: r.views_count,
            applications_count: r.applications_count,
            created_at: r.created_at,
        }).collect())
    }
    
    /// Search jobs
    pub async fn search_jobs(
        pool: &PgPool,
        query: &str,
        location: Option<&str>,
        job_type: Option<JobType>,
        limit: i64,
    ) -> Result<Vec<JobPosting>, String> {
        let search_pattern = format!("%{}%", query);
        
        let rows = sqlx::query!(
            "SELECT id, employer_id, title, description, requirements, location, job_type,
             experience_level, salary_min, salary_max, salary_currency, application_deadline,
             is_active, views_count, applications_count, created_at
             FROM job_posting 
             WHERE is_active = true AND (title ILIKE $1 OR description ILIKE $1)
             ORDER BY created_at DESC LIMIT $2",
            search_pattern, limit
        )
        .fetch_all(pool)
        .await
        .map_err(|e| e.to_string())?;
        
        Ok(rows.into_iter().map(|r| JobPosting {
            id: r.id,
            employer_id: r.employer_id,
            title: r.title,
            description: r.description,
            requirements: serde_json::from_str(&r.requirements).unwrap_or_default(),
            location: r.location,
            job_type: JobType::FullTime,
            experience_level: r.experience_level,
            salary_range: None,
            application_deadline: r.application_deadline,
            is_active: r.is_active,
            views_count: r.views_count,
            applications_count: r.applications_count,
            created_at: r.created_at,
        }).collect())
    }
    
    /// Update application status
    pub async fn update_application_status(
        pool: &PgPool,
        application_id: uuid::Uuid,
        status: ApplicationStatus,
    ) -> Result<(), String> {
        sqlx::query!(
            "UPDATE job_applications SET status = $1, reviewed_at = $2 WHERE id = $3",
            format!("{:?}", status).to_lowercase(), Utc::now(), application_id
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
    ) -> Result<CareerResource, String> {
        let id = Uuid::new_v4();
        
        sqlx::query!(
            "INSERT INTO career_resources (id, title, content, resource_type, category, is_published, created_at)
             VALUES ($1, $2, $3, $4, $5, true, $6)",
            id, title, content, format!("{:?}", resource_type).to_lowercase(), category, Utc::now()
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
            is_published: true,
            created_at: Utc::now(),
        })
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
}

#[derive(Debug, Deserialize)]
pub struct PostJobRequest {
    pub title: String,
    pub description: String,
    pub requirements: Vec<String>,
    pub location: String,
    pub job_type: JobType,
    pub experience_level: String,
    pub salary_range: Option<SalaryRange>,
    pub application_deadline: Option<DateTime<Utc>>,
}