// Research & Postgraduate Supervision — Thesis management, milestones, chapter reviews
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Research project types
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResearchLevel {
    Masters,
    Doctoral,
    PostDoctoral,
    Undergraduate, // Honours/Final Year
}

/// Supervision relationship
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SupervisionRelationship {
    pub id: Uuid,
    pub institution_id: Uuid,
    pub student_id: Uuid,
    pub primary_supervisor_id: Uuid,
    pub co_supervisor_ids: Vec<Uuid>,
    pub programme: String,
    pub research_level: ResearchLevel,
    pub start_date: DateTime<Utc>,
    pub expected_completion: DateTime<Utc>,
    pub status: SupervisionStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SupervisionStatus {
    Active,
    OnLeave,
    SubmittedForExamination,
    UnderCorrections,
    Completed,
    Withdrawn,
}

/// Research profile / thesis record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResearchProfile {
    pub id: Uuid,
    pub supervision_id: Uuid,
    pub student_id: Uuid,
    pub institution_id: Uuid,
    pub title: String,
    pub abstract_text: Option<String>,
    pub keywords: Vec<String>,
    pub proposal_url: Option<String>,
    pub proposal_approved: bool,
    pub ethics_clearance_status: EthicsClearanceStatus,
    pub ethics_clearance_ref: Option<String>,
    pub milestones: Vec<ResearchMilestone>,
    pub chapters: Vec<ChapterDraft>,
    pub publications: Vec<Publication>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EthicsClearanceStatus {
    NotRequired,
    Pending,
    Approved,
    Rejected,
    Expired,
}

/// A research milestone (e.g. "Submit Chapter 1", "Proposal Approval")
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResearchMilestone {
    pub id: Uuid,
    pub research_profile_id: Uuid,
    pub title: String,
    pub description: String,
    pub due_date: DateTime<Utc>,
    pub status: MilestoneStatus,
    pub supervisor_sign_off: bool,
    pub signed_off_by: Option<Uuid>,
    pub signed_off_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum MilestoneStatus {
    Upcoming,
    InProgress,
    Overdue,
    PendingSignOff,
    Completed,
}

/// A chapter draft submission
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChapterDraft {
    pub id: Uuid,
    pub research_profile_id: Uuid,
    pub chapter_number: u8,
    pub chapter_title: String,
    pub version: u32,
    pub file_url: String,
    pub word_count: Option<u32>,
    pub submitted_at: DateTime<Utc>,
    pub feedback: Vec<ChapterFeedback>,
    pub status: ChapterStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ChapterStatus {
    Submitted,
    UnderReview,
    FeedbackProvided,
    Accepted,
}

/// Supervisor feedback on a chapter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChapterFeedback {
    pub id: Uuid,
    pub chapter_draft_id: Uuid,
    pub supervisor_id: Uuid,
    pub general_comments: String,
    pub annotated_file_url: Option<String>, // tracked-changes document
    pub decision: FeedbackDecision,
    pub provided_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FeedbackDecision {
    AcceptAsIs,
    MinorRevisions,
    MajorRevisions,
    Reject,
}

/// Quarterly progress report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgressReport {
    pub id: Uuid,
    pub research_profile_id: Uuid,
    pub student_id: Uuid,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub progress_summary: String,
    pub plan_for_next_period: String,
    pub challenges: String,
    pub supervisor_endorsement: Option<SupervisorEndorsement>,
    pub submitted_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SupervisorEndorsement {
    pub supervisor_id: Uuid,
    pub comments: String,
    pub endorsed_at: DateTime<Utc>,
}

/// Viva / defence record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VivaRecord {
    pub id: Uuid,
    pub research_profile_id: Uuid,
    pub scheduled_at: DateTime<Utc>,
    pub external_examiner_id: Option<Uuid>,
    pub internal_examiner_id: Option<Uuid>,
    pub outcome: Option<VivaOutcome>,
    pub corrections_deadline: Option<DateTime<Utc>>,
    pub corrections_submitted_at: Option<DateTime<Utc>>,
    pub examiner_report_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VivaOutcome {
    Pass,
    PassWithMinorCorrections,
    MajorCorrections,
    Fail,
    Resubmit,
}

/// Publication linked to a student's research
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Publication {
    pub id: Uuid,
    pub title: String,
    pub authors: Vec<String>,
    pub journal_or_conference: String,
    pub year: u16,
    pub doi: Option<String>,
    pub status: PublicationStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PublicationStatus {
    InPreparation,
    Submitted,
    UnderReview,
    Accepted,
    Published,
}

pub struct ResearchService;

impl ResearchService {
    /// Create a new research profile for a student
    pub async fn create_profile(
        _pool: &sqlx::PgPool,
        supervision_id: Uuid,
        student_id: Uuid,
        institution_id: Uuid,
        title: &str,
    ) -> Result<ResearchProfile, sqlx::Error> {
        let now = Utc::now();
        Ok(ResearchProfile {
            id: Uuid::new_v4(),
            supervision_id,
            student_id,
            institution_id,
            title: title.to_string(),
            abstract_text: None,
            keywords: vec![],
            proposal_url: None,
            proposal_approved: false,
            ethics_clearance_status: EthicsClearanceStatus::Pending,
            ethics_clearance_ref: None,
            milestones: vec![],
            chapters: vec![],
            publications: vec![],
            created_at: now,
            updated_at: now,
        })
    }

    /// Add a milestone to a research profile
    pub async fn add_milestone(
        _pool: &sqlx::PgPool,
        profile_id: Uuid,
        title: &str,
        description: &str,
        due_date: DateTime<Utc>,
        requires_sign_off: bool,
    ) -> Result<ResearchMilestone, sqlx::Error> {
        Ok(ResearchMilestone {
            id: Uuid::new_v4(),
            research_profile_id: profile_id,
            title: title.to_string(),
            description: description.to_string(),
            due_date,
            status: MilestoneStatus::Upcoming,
            supervisor_sign_off: requires_sign_off,
            signed_off_by: None,
            signed_off_at: None,
            completed_at: None,
            notes: None,
        })
    }

    /// Supervisor signs off a milestone
    pub async fn sign_off_milestone(
        _pool: &sqlx::PgPool,
        milestone_id: Uuid,
        supervisor_id: Uuid,
        notes: Option<String>,
    ) -> Result<ResearchMilestone, sqlx::Error> {
        let _ = milestone_id;
        // TODO: UPDATE milestones SET status='completed', signed_off_by=$1, signed_off_at=NOW()
        // If overdue: alert student, supervisor, and coordinator
        Ok(ResearchMilestone {
            id: milestone_id,
            research_profile_id: Uuid::new_v4(),
            title: String::new(),
            description: String::new(),
            due_date: Utc::now(),
            status: MilestoneStatus::Completed,
            supervisor_sign_off: true,
            signed_off_by: Some(supervisor_id),
            signed_off_at: Some(Utc::now()),
            completed_at: Some(Utc::now()),
            notes,
        })
    }

    /// Student submits a chapter draft
    pub async fn submit_chapter(
        _pool: &sqlx::PgPool,
        profile_id: Uuid,
        chapter_number: u8,
        chapter_title: &str,
        file_url: &str,
        word_count: Option<u32>,
    ) -> Result<ChapterDraft, sqlx::Error> {
        // TODO: Check if a prior version exists — increment version number
        Ok(ChapterDraft {
            id: Uuid::new_v4(),
            research_profile_id: profile_id,
            chapter_number,
            chapter_title: chapter_title.to_string(),
            version: 1,
            file_url: file_url.to_string(),
            word_count,
            submitted_at: Utc::now(),
            feedback: vec![],
            status: ChapterStatus::Submitted,
        })
    }

    /// Supervisor provides feedback on a chapter
    pub async fn provide_chapter_feedback(
        _pool: &sqlx::PgPool,
        chapter_id: Uuid,
        supervisor_id: Uuid,
        comments: &str,
        annotated_file_url: Option<String>,
        decision: FeedbackDecision,
    ) -> Result<ChapterFeedback, sqlx::Error> {
        let feedback = ChapterFeedback {
            id: Uuid::new_v4(),
            chapter_draft_id: chapter_id,
            supervisor_id,
            general_comments: comments.to_string(),
            annotated_file_url,
            decision,
            provided_at: Utc::now(),
        };
        // TODO: UPDATE chapter_drafts SET status = FeedbackProvided
        // Notify student
        Ok(feedback)
    }

    /// Student submits quarterly progress report
    pub async fn submit_progress_report(
        _pool: &sqlx::PgPool,
        profile_id: Uuid,
        student_id: Uuid,
        period_start: DateTime<Utc>,
        period_end: DateTime<Utc>,
        progress: &str,
        plan: &str,
        challenges: &str,
    ) -> Result<ProgressReport, sqlx::Error> {
        Ok(ProgressReport {
            id: Uuid::new_v4(),
            research_profile_id: profile_id,
            student_id,
            period_start,
            period_end,
            progress_summary: progress.to_string(),
            plan_for_next_period: plan.to_string(),
            challenges: challenges.to_string(),
            supervisor_endorsement: None,
            submitted_at: Utc::now(),
        })
    }

    /// Record a viva outcome
    pub async fn record_viva_outcome(
        _pool: &sqlx::PgPool,
        viva_id: Uuid,
        outcome: VivaOutcome,
        corrections_deadline: Option<DateTime<Utc>>,
        examiner_report_url: Option<String>,
    ) -> Result<VivaRecord, sqlx::Error> {
        let _ = viva_id;
        Ok(VivaRecord {
            id: viva_id,
            research_profile_id: Uuid::new_v4(),
            scheduled_at: Utc::now(),
            external_examiner_id: None,
            internal_examiner_id: None,
            outcome: Some(outcome),
            corrections_deadline,
            corrections_submitted_at: None,
            examiner_report_url,
        })
    }

    /// Get overdue milestones across all active supervision relationships
    pub async fn get_overdue_milestones(
        _pool: &sqlx::PgPool,
        institution_id: Uuid,
    ) -> Result<Vec<ResearchMilestone>, sqlx::Error> {
        let _ = institution_id;
        // TODO: SELECT * FROM milestones WHERE status = 'in_progress'
        //       AND due_date < NOW() → status becomes Overdue
        //       Alert supervisor, student, and postgrad coordinator
        Ok(vec![])
    }

    /// Check for stale supervision (no interaction in N days) and escalate
    pub async fn check_stale_supervision(
        _pool: &sqlx::PgPool,
        institution_id: Uuid,
        stale_days: u32,
    ) -> Result<Vec<Uuid>, sqlx::Error> {
        let _ = (institution_id, stale_days);
        // TODO: SELECT supervision_id WHERE last_interaction < NOW() - INTERVAL '$1 days'
        Ok(vec![])
    }
}
