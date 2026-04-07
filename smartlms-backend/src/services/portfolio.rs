// Student Portfolio System — Academic, competency, project, and career portfolios
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Portfolio type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PortfolioType {
    Academic,   // curated best work across all courses
    Competency, // evidence mapped to CBE nodes
    Project,    // showcase of major projects
    Career,     // public-facing, employer-ready
}

/// A student portfolio
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Portfolio {
    pub id: Uuid,
    pub student_id: Uuid,
    pub institution_id: Uuid,
    pub portfolio_type: PortfolioType,
    pub title: String,
    pub summary: String,
    pub items: Vec<PortfolioItem>,
    pub sharing_mode: SharingMode,
    pub public_url_slug: Option<String>,
    pub is_verified: bool,
    pub verification_seal_url: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// How the portfolio is shared
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SharingMode {
    Private,
    ShareWithAdvisor,
    SharedWithEmployer {
        employer_id: Uuid,
        expires_at: DateTime<Utc>,
    },
    Public,
}

/// An item in a portfolio
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortfolioItem {
    pub id: Uuid,
    pub portfolio_id: Uuid,
    pub item_type: PortfolioItemType,
    pub title: String,
    pub description: String,
    pub reflection: Option<String>, // what they learned, what they'd do differently
    pub source: ItemSource,
    pub file_urls: Vec<String>,
    pub external_url: Option<String>,
    pub competency_node_ids: Vec<Uuid>, // CBE nodes this item evidences
    pub grade: Option<String>,
    pub instructor_endorsement: Option<InstructorEndorsement>,
    pub date_created: DateTime<Utc>,
    pub added_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PortfolioItemType {
    Assignment,
    Project,
    Certificate,
    ExternalWork,
    GithubRepo,
    ResearchPaper,
    Presentation,
    Video,
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ItemSource {
    CourseSubmission {
        course_id: Uuid,
        assignment_id: Uuid,
    },
    External,
    Manual,
}

/// Instructor endorsement on a portfolio item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstructorEndorsement {
    pub instructor_id: Uuid,
    pub instructor_name: String,
    pub endorsement_text: String,
    pub endorsed_at: DateTime<Utc>,
}

/// A time-limited share link for employers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortfolioShareLink {
    pub id: Uuid,
    pub portfolio_id: Uuid,
    pub employer_id: Option<Uuid>,
    pub share_token: String,
    pub expires_at: DateTime<Utc>,
    pub view_count: i32,
    pub created_at: DateTime<Utc>,
}

/// Verified portfolio PDF (institution-sealed)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifiedPortfolioPdf {
    pub portfolio_id: Uuid,
    pub student_name: String,
    pub institution_name: String,
    pub generated_at: DateTime<Utc>,
    pub digital_signature: String,
    pub pdf_url: String,
    pub items_count: i32,
    pub competencies_evidenced: Vec<String>,
}

/// Request to create a portfolio
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePortfolioRequest {
    pub student_id: Uuid,
    pub institution_id: Uuid,
    pub portfolio_type: PortfolioType,
    pub title: String,
    pub summary: String,
}

/// Request to add an item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddPortfolioItemRequest {
    pub portfolio_id: Uuid,
    pub item_type: PortfolioItemType,
    pub title: String,
    pub description: String,
    pub reflection: Option<String>,
    pub source: ItemSource,
    pub file_urls: Vec<String>,
    pub external_url: Option<String>,
    pub competency_node_ids: Vec<Uuid>,
    pub grade: Option<String>,
}

pub struct PortfolioService;

impl PortfolioService {
    /// Create a new portfolio for a student
    pub async fn create_portfolio(
        _pool: &sqlx::PgPool,
        req: CreatePortfolioRequest,
    ) -> Result<Portfolio, sqlx::Error> {
        let now = Utc::now();
        Ok(Portfolio {
            id: Uuid::new_v4(),
            student_id: req.student_id,
            institution_id: req.institution_id,
            portfolio_type: req.portfolio_type,
            title: req.title,
            summary: req.summary,
            items: vec![],
            sharing_mode: SharingMode::Private,
            public_url_slug: None,
            is_verified: false,
            verification_seal_url: None,
            created_at: now,
            updated_at: now,
        })
    }

    /// Add an item to a portfolio
    pub async fn add_item(
        _pool: &sqlx::PgPool,
        req: AddPortfolioItemRequest,
    ) -> Result<PortfolioItem, sqlx::Error> {
        let now = Utc::now();
        Ok(PortfolioItem {
            id: Uuid::new_v4(),
            portfolio_id: req.portfolio_id,
            item_type: req.item_type,
            title: req.title,
            description: req.description,
            reflection: req.reflection,
            source: req.source,
            file_urls: req.file_urls,
            external_url: req.external_url,
            competency_node_ids: req.competency_node_ids,
            grade: req.grade,
            instructor_endorsement: None,
            date_created: now,
            added_at: now,
        })
    }

    /// Instructor endorses a portfolio item
    pub async fn endorse_item(
        _pool: &sqlx::PgPool,
        item_id: Uuid,
        instructor_id: Uuid,
        instructor_name: &str,
        endorsement_text: &str,
    ) -> Result<InstructorEndorsement, sqlx::Error> {
        let endorsement = InstructorEndorsement {
            instructor_id,
            instructor_name: instructor_name.to_string(),
            endorsement_text: endorsement_text.to_string(),
            endorsed_at: Utc::now(),
        };
        // TODO: UPDATE portfolio_items SET instructor_endorsement = $1 WHERE id = $2
        let _ = item_id;
        Ok(endorsement)
    }

    /// Update portfolio sharing mode
    pub async fn update_sharing(
        _pool: &sqlx::PgPool,
        portfolio_id: Uuid,
        student_id: Uuid,
        sharing_mode: SharingMode,
    ) -> Result<(), sqlx::Error> {
        let _ = (portfolio_id, student_id, sharing_mode);
        // TODO: UPDATE portfolios SET sharing_mode = $1 WHERE id = $2 AND student_id = $3
        Ok(())
    }

    /// Create a time-limited share link for a specific employer
    pub async fn create_share_link(
        _pool: &sqlx::PgPool,
        portfolio_id: Uuid,
        employer_id: Option<Uuid>,
        validity_days: u32,
    ) -> Result<PortfolioShareLink, sqlx::Error> {
        let token = format!("pf-{}", Uuid::new_v4());
        Ok(PortfolioShareLink {
            id: Uuid::new_v4(),
            portfolio_id,
            employer_id,
            share_token: token,
            expires_at: Utc::now() + chrono::Duration::days(validity_days as i64),
            view_count: 0,
            created_at: Utc::now(),
        })
    }

    /// Generate a verified, institution-sealed portfolio PDF
    pub async fn generate_verified_pdf(
        _pool: &sqlx::PgPool,
        portfolio_id: Uuid,
        student_name: &str,
        institution_name: &str,
    ) -> Result<VerifiedPortfolioPdf, sqlx::Error> {
        // TODO: Render PDF with institution seal + digital signature
        let sig = format!("sha256:{}", Uuid::new_v4());
        Ok(VerifiedPortfolioPdf {
            portfolio_id,
            student_name: student_name.to_string(),
            institution_name: institution_name.to_string(),
            generated_at: Utc::now(),
            digital_signature: sig,
            pdf_url: format!("/portfolios/{}/verified.pdf", portfolio_id),
            items_count: 0,
            competencies_evidenced: vec![],
        })
    }

    /// Get all portfolios for a student
    pub async fn list_student_portfolios(
        _pool: &sqlx::PgPool,
        student_id: Uuid,
    ) -> Result<Vec<Portfolio>, sqlx::Error> {
        let _ = student_id;
        // TODO: SELECT * FROM portfolios WHERE student_id = $1
        Ok(vec![])
    }

    /// Verify a share token (used by employers)
    pub async fn verify_share_token(
        _pool: &sqlx::PgPool,
        token: &str,
    ) -> Result<Option<Portfolio>, sqlx::Error> {
        let _ = token;
        // TODO: SELECT portfolio via share_links WHERE share_token = $1 AND expires_at > NOW()
        Ok(None)
    }
}
