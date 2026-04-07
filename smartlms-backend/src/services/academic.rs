// Advanced Academic Features - CBE, Micro-credentials, Wellbeing, Advising, etc.
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

// ============================================================================
// COMPETENCY-BASED EDUCATION (CBE)
// ============================================================================

/// Competency framework
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompetencyFramework {
    pub id: uuid::Uuid,
    pub institution_id: uuid::Uuid,
    pub name: String,
    pub description: Option<String>,
    pub competencies: Vec<Competency>,
    pub created_at: DateTime<Utc>,
}

/// Individual competency
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Competency {
    pub id: uuid::Uuid,
    pub framework_id: uuid::Uuid,
    pub code: String,           // e.g., "MATH-101"
    pub name: String,
    pub description: String,
    pub level: i32,             // 1-5 mastery levels
    pub parent_id: Option<uuid::Uuid>,
    pub prerequisites: Vec<uuid::Uuid>,
}

/// Student competency record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompetencyRecord {
    pub id: uuid::Uuid,
    pub student_id: uuid::Uuid,
    pub competency_id: uuid::Uuid,
    pub mastery_level: i32,
    pub evidence: Vec<CompetencyEvidence>,
    pub assessed_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
}

/// Evidence for competency
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompetencyEvidence {
    pub id: uuid::Uuid,
    pub record_id: uuid::Uuid,
    pub evidence_type: String,  // "quiz", "assignment", "project", "portfolio"
    pub reference_id: uuid::Uuid,
    pub score: Option<f64>,
    pub notes: Option<String>,
    pub submitted_at: DateTime<Utc>,
}

// ============================================================================
// MICRO-CREDENTIALS & DIGITAL BADGES
// ============================================================================

/// Micro-credential definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MicroCredential {
    pub id: uuid::Uuid,
    pub institution_id: uuid::Uuid,
    pub name: String,
    pub description: String,
    pub competencies: Vec<uuid::Uuid>,
    pub requirements: CredentialRequirement,
    pub validity_months: i32,
    pub badge_template: BadgeTemplate,
    pub created_at: DateTime<Utc>,
}

/// Requirements to earn credential
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CredentialRequirement {
    pub min_competencies: i32,
    pub min_mastery_level: i32,
    pub required_courses: Vec<uuid::Uuid>,
    pub practice_hours: Option<i32>,
}

/// Badge template
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BadgeTemplate {
    pub name: String,
    pub image_url: String,
    pub criteria_url: String,
    pub issuer_name: String,
    pub issuer_url: String,
}

/// Issued credential
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IssuedCredential {
    pub id: uuid::Uuid,
    pub credential_id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub badge_id: String,          // Open Badges 3.0 ID
    pub issued_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
    pub revocation_reason: Option<String>,
    pub is_revoked: bool,
}

// ============================================================================
// STUDENT WELLBEING
// ============================================================================

/// Wellbeing check-in
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WellbeingCheckin {
    pub id: uuid::Uuid,
    pub student_id: uuid::Uuid,
    pub emotional_state: i32,      // 1-10
    pub stress_level: i32,         // 1-10
    pub sleep_quality: i32,       // 1-10
    pub social_connection: i32,   // 1-10
    pub comments: Option<String>,
    pub flag_for_followup: bool,
    pub submitted_at: DateTime<Utc>,
}

/// Wellbeing alert
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WellbeingAlert {
    pub id: uuid::Uuid,
    pub student_id: uuid::Uuid,
    pub alert_type: WellbeingAlertType,
    pub severity: AlertSeverity,
    pub description: String,
    pub assigned_counsellor: Option<uuid::Uuid>,
    pub is_resolved: bool,
    pub resolved_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WellbeingAlertType {
    MentalHealth,
    AcademicStress,
    FinancialConcern,
    Safety,
    SocialIsolation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AlertSeverity {
    Low,
    Medium,
    High,
    Critical,
}

// ============================================================================
// ACADEMIC ADVISING
// ============================================================================

/// Academic plan
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AcademicPlan {
    pub id: uuid::Uuid,
    pub student_id: uuid::Uuid,
    pub advisor_id: uuid::Uuid,
    pub semester_id: uuid::Uuid,
    pub courses: Vec<PlannedCourse>,
    pub status: PlanStatus,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub approved_at: Option<DateTime<Utc>>,
}

/// Course in academic plan
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlannedCourse {
    pub course_id: uuid::Uuid,
    pub is_required: bool,
    pub priority: i32,
    pub status: CoursePlanStatus,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PlanStatus {
    Draft,
    PendingApproval,
    Approved,
    Rejected,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CoursePlanStatus {
    Planned,
    Enrolled,
    Completed,
    Dropped,
    Deferred,
}

// ============================================================================
// RESEARCH & POSTGRADUATE SUPERVISION
// ============================================================================

/// Research proposal
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResearchProposal {
    pub id: uuid::Uuid,
    pub student_id: uuid::Uuid,
    pub title: String,
    pub abstract_text: String,
    pub supervisor_id: uuid::Uuid,
    pub committee_members: Vec<uuid::Uuid>,
    pub status: ProposalStatus,
    pub milestones: Vec<ResearchMilestone>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProposalStatus {
    Draft,
    Submitted,
    UnderReview,
    Approved,
    RevisionRequired,
    Rejected,
}

/// Research milestone
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResearchMilestone {
    pub id: uuid::Uuid,
    pub proposal_id: uuid::Uuid,
    pub title: String,
    pub description: String,
    pub due_date: DateTime<Utc>,
    pub status: MilestoneStatus,
    pub submitted_at: Option<DateTime<Utc>>,
    pub feedback: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MilestoneStatus {
    Pending,
    Submitted,
    Approved,
    RevisionRequested,
}

// ============================================================================
// PEER LEARNING & PEER REVIEW
// ============================================================================

/// Peer review assignment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeerReview {
    pub id: uuid::Uuid,
    pub assignment_id: uuid::Uuid,
    pub reviewer_id: uuid::Uuid,
    pub reviewee_id: uuid::Uuid,
    pub criteria: Vec<ReviewCriterion>,
    pub feedback: Option<String>,
    pub score: Option<f64>,
    pub status: ReviewStatus,
    pub due_date: DateTime<Utc>,
    pub submitted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReviewCriterion {
    pub criterion: String,
    pub max_score: i32,
    pub score: Option<i32>,
    pub comment: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReviewStatus {
    Pending,
    InProgress,
    Submitted,
    Late,
}

// ============================================================================
// STUDENT PORTFOLIO
// ============================================================================

/// Student portfolio
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Portfolio {
    pub id: uuid::Uuid,
    pub student_id: uuid::Uuid,
    pub title: String,
    pub description: Option<String>,
    pub items: Vec<PortfolioItem>,
    pub is_public: bool,
    pub share_url: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Portfolio item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortfolioItem {
    pub id: uuid::Uuid,
    pub portfolio_id: uuid::Uuid,
    pub title: String,
    pub item_type: PortfolioItemType,
    pub content_url: Option<String>,
    pub description: Option<String>,
    pub tags: Vec<String>,
    pub is_featured: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PortfolioItemType {
    Document,
    Image,
    Video,
    Code,
    Project,
    Certificate,
    Reflection,
}

// ============================================================================
// RECOGNITION OF PRIOR LEARNING (RPL)
// ============================================================================

/// RPL application
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RplApplication {
    pub id: uuid::Uuid,
    pub student_id: uuid::Uuid,
    pub target_credential_id: uuid::Uuid,
    pub experiences: Vec<RplExperience>,
    pub evidence: Vec<RplEvidence>,
    pub status: RplStatus,
    pub assessed_by: Option<uuid::Uuid>,
    pub outcome: Option<RplOutcome>,
    pub created_at: DateTime<Utc>,
}

/// Prior learning experience
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RplExperience {
    pub id: uuid::Uuid,
    pub application_id: uuid::Uuid,
    pub title: String,
    pub description: String,
    pub hours: i32,
    pub competencies_mapped: Vec<uuid::Uuid>,
}

/// Evidence for RPL
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RplEvidence {
    pub id: uuid::Uuid,
    pub application_id: uuid::Uuid,
    pub evidence_type: String,
    pub title: String,
    pub file_url: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RplStatus {
    Draft,
    Submitted,
    UnderReview,
    EvidenceRequested,
    Approved,
    PartialApproval,
    Rejected,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RplOutcome {
    pub status: RplStatus,
    pub credits_recognized: i32,
    pub competencies_awarded: Vec<uuid::Uuid>,
    pub remaining_requirements: Vec<String>,
    pub notes: Option<String>,
}

// ============================================================================
// ALUMNI PORTAL
// ============================================================================

/// Alumni record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlumniRecord {
    pub id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub graduation_year: i32,
    pub degree: String,
    pub major: String,
    pub employment_status: EmploymentStatus,
    pub current_employer: Option<String>,
    pub linkedin_url: Option<String>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EmploymentStatus {
    Employed,
    SelfEmployed,
    FurtherStudy,
    Seeking,
    NotSeeking,
}

// SERVICE FUNCTIONS
pub mod service {
    use super::*;
    
    /// Create competency framework
    pub async fn create_framework(
        pool: &PgPool,
        institution_id: uuid::Uuid,
        name: &str,
        description: Option<&str>,
    ) -> Result<CompetencyFramework, String> {
        let id = Uuid::new_v4();
        
        sqlx::query!(
            "INSERT INTO competency_frameworks (id, institution_id, name, description, created_at)
             VALUES ($1, $2, $3, $4, $5)",
            id, institution_id, name, description, Utc::now()
        )
        .execute(pool)
        .await
        .map_err(|e| e.to_string())?;
        
        Ok(CompetencyFramework {
            id,
            institution_id,
            name: name.to_string(),
            description: description.map(String::from),
            competencies: vec![],
            created_at: Utc::now(),
        })
    }
    
    /// Create micro-credential
    pub async fn create_micro_credential(
        pool: &PgPool,
        institution_id: uuid::Uuid,
        name: &str,
        description: &str,
        competencies: Vec<uuid::Uuid>,
    ) -> Result<MicroCredential, String> {
        let id = Uuid::new_v4();
        
        sqlx::query!(
            "INSERT INTO micro_credentials (id, institution_id, name, description, competencies, created_at)
             VALUES ($1, $2, $3, $4, $5, $6)",
            id, institution_id, name, description, serde_json::to_string(&competencies).unwrap(), Utc::now()
        )
        .execute(pool)
        .await
        .map_err(|e| e.to_string())?;
        
        Ok(MicroCredential {
            id,
            institution_id,
            name: name.to_string(),
            description: description.to_string(),
            competencies,
            requirements: CredentialRequirement {
                min_competencies: 3,
                min_mastery_level: 3,
                required_courses: vec![],
                practice_hours: None,
            },
            validity_months: 24,
            badge_template: BadgeTemplate {
                name: name.to_string(),
                image_url: String::new(),
                criteria_url: String::new(),
                issuer_name: String::new(),
                issuer_url: String::new(),
            },
            created_at: Utc::now(),
        })
    }
    
    /// Submit wellbeing check-in
    pub async fn submit_wellbeing_checkin(
        pool: &PgPool,
        student_id: uuid::Uuid,
        emotional: i32,
        stress: i32,
        sleep: i32,
        social: i32,
        comments: Option<&str>,
    ) -> Result<WellbeingCheckin, String> {
        let id = Uuid::new_v4();
        
        // Flag if any metric is low
        let flag = emotional < 4 || stress > 7 || sleep < 4 || social < 4;
        
        sqlx::query!(
            "INSERT INTO wellbeing_checkins (id, student_id, emotional_state, stress_level, 
             sleep_quality, social_connection, comments, flag_for_followup, submitted_at)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)",
            id, student_id, emotional, stress, sleep, social, comments, flag, Utc::now()
        )
        .execute(pool)
        .await
        .map_err(|e| e.to_string())?;
        
        // Create alert if flagged
        if flag {
            let alert_id = Uuid::new_v4();
            sqlx::query!(
                "INSERT INTO wellbeing_alerts (id, student_id, alert_type, severity, description, created_at)
                 VALUES ($1, $2, 'academic_stress', 'medium', 'Student flagged from check-in', $3)",
                alert_id, student_id, Utc::now()
            )
            .execute(pool)
            .await
            .ok();
        }
        
        Ok(WellbeingCheckin {
            id,
            student_id,
            emotional_state: emotional,
            stress_level: stress,
            sleep_quality: sleep,
            social_connection: social,
            comments: comments.map(String::from),
            flag_for_followup: flag,
            submitted_at: Utc::now(),
        })
    }
    
    /// Create peer review assignment
    pub async fn create_peer_review(
        pool: &PgPool,
        assignment_id: uuid::Uuid,
        reviewer_id: uuid::Uuid,
        reviewee_id: uuid::Uuid,
        due_date: DateTime<Utc>,
    ) -> Result<PeerReview, String> {
        let id = Uuid::new_v4();
        
        sqlx::query!(
            "INSERT INTO peer_reviews (id, assignment_id, reviewer_id, reviewee_id, status, due_date)
             VALUES ($1, $2, $3, $4, 'pending', $5)",
            id, assignment_id, reviewer_id, reviewee_id, due_date
        )
        .execute(pool)
        .await
        .map_err(|e| e.to_string())?;
        
        Ok(PeerReview {
            id,
            assignment_id,
            reviewer_id,
            reviewee_id,
            criteria: vec![],
            feedback: None,
            score: None,
            status: ReviewStatus::Pending,
            due_date,
            submitted_at: None,
        })
    }
    
    /// Create student portfolio
    pub async fn create_portfolio(
        pool: &PgPool,
        student_id: uuid::Uuid,
        title: &str,
        description: Option<&str>,
    ) -> Result<Portfolio, String> {
        let id = Uuid::new_v4();
        
        sqlx::query!(
            "INSERT INTO portfolios (id, student_id, title, description, is_public, created_at, updated_at)
             VALUES ($1, $2, $3, $4, false, $5, $6)",
            id, student_id, title, description, Utc::now(), Utc::now()
        )
        .execute(pool)
        .await
        .map_err(|e| e.to_string())?;
        
        Ok(Portfolio {
            id,
            student_id,
            title: title.to_string(),
            description: description.map(String::from),
            items: vec![],
            is_public: false,
            share_url: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        })
    }
    
    /// Submit RPL application
    pub async fn submit_rpl_application(
        pool: &PgPool,
        student_id: uuid::Uuid,
        target_credential_id: uuid::Uuid,
    ) -> Result<RplApplication, String> {
        let id = Uuid::new_v4();
        
        sqlx::query!(
            "INSERT INTO rpl_applications (id, student_id, target_credential_id, status, created_at)
             VALUES ($1, $2, $3, 'draft', $4)",
            id, student_id, target_credential_id, Utc::now()
        )
        .execute(pool)
        .await
        .map_err(|e| e.to_string())?;
        
        Ok(RplApplication {
            id,
            student_id,
            target_credential_id,
            experiences: vec![],
            evidence: vec![],
            status: RplStatus::Draft,
            assessed_by: None,
            outcome: None,
            created_at: Utc::now(),
        })
    }
}