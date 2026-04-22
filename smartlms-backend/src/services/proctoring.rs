// Proctoring Service - Browser lockdown, video recording, AI proctoring
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Proctoring session status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProctoringStatus {
    NotStarted,
    InProgress,
    Paused,
    Completed,
    Flagged,
}

/// Proctoring session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProctoringSession {
    pub id: uuid::Uuid,
    pub exam_id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub status: ProctoringStatus,

    // Recording URLs
    pub webcam_url: Option<String>,
    pub screen_url: Option<String>,
    pub audio_url: Option<String>,

    // Settings
    pub browser_lockdown: bool,
    pub fullscreen_required: bool,
    pub tab_switch_alerts: bool,
    pub webcam_required: bool,
    pub screen_recording: bool,

    // Security flags
    pub face_detected: bool,
    pub multiple_faces: bool,
    pub no_face_duration_seconds: i32,
    pub tab_switch_count: i32,
    pub copy_paste_attempts: i32,

    // Timing
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub duration_minutes: Option<i32>,

    pub created_at: DateTime<Utc>,
}

/// Violation event during proctoring
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ViolationEvent {
    pub id: uuid::Uuid,
    pub session_id: uuid::Uuid,
    pub violation_type: ViolationType,
    pub severity: ViolationSeverity,
    pub description: String,
    pub evidence_url: Option<String>,
    pub timestamp: DateTime<Utc>,
}

/// Types of violations
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ViolationType {
    TabSwitch,
    WindowBlur,
    FullscreenExit,
    MultipleFacesDetected,
    NoFaceDetected,
    SuspiciousMovement,
    AudioAnomaly,
    CopyPasteAttempt,
    RightClickAttempt,
    KeyboardShortcut,
    ExternalMonitor,
    DeviceChange,
    BrowserExtension,
}

/// Violation severity
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ViolationSeverity {
    Low,
    Medium,
    High,
    Critical,
}

/// Review status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReviewStatus {
    Pending,
    UnderReview,
    Approved,
    Rejected,
    FlaggedForAppeal,
}

/// Proctoring review
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProctoringReview {
    pub id: uuid::Uuid,
    pub session_id: uuid::Uuid,
    pub reviewer_id: Option<uuid::Uuid>,
    pub review_status: ReviewStatus,
    pub notes: Option<String>,
    pub violation_count: i32,
    pub overall_score: Option<f64>,
    pub reviewed_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

/// Identity verification record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentityVerification {
    pub id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub verification_type: IdentityType,
    pub id_photo_url: Option<String>,
    pub selfie_photo_url: Option<String>,
    pub match_score: Option<f64>,
    pub is_verified: bool,
    pub liveness_passed: bool,
    pub verified_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

/// Identity verification type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum IdentityType {
    PhotoID,
    Selfie,
    Liveness,
}

// ============================================================================
// SERVICE FUNCTIONS
// ============================================================================

pub mod service {
    use super::*;

    /// Create a new proctoring session for an exam
    pub async fn create_session(
        pool: &PgPool,
        exam_id: uuid::Uuid,
        user_id: uuid::Uuid,
        settings: &ProctoringSettings,
    ) -> Result<ProctoringSession, String> {
        let id = Uuid::new_v4();
        let now = Utc::now();

        sqlx::query!(
            "INSERT INTO proctoring_sessions (id, exam_id, user_id, status, 
             browser_lockdown, fullscreen_required, tab_switch_alerts,
             webcam_required, screen_recording, created_at)
             VALUES ($1, $2, $3, 'not_started', $4, $5, $6, $7, $8, $9)",
            id,
            exam_id,
            user_id,
            settings.browser_lockdown,
            settings.fullscreen_required,
            settings.tab_switch_alerts,
            settings.webcam_required,
            settings.screen_recording,
            now
        )
        .execute(pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(ProctoringSession {
            id,
            exam_id,
            user_id,
            status: ProctoringStatus::NotStarted,
            webcam_url: None,
            screen_url: None,
            audio_url: None,
            browser_lockdown: settings.browser_lockdown,
            fullscreen_required: settings.fullscreen_required,
            tab_switch_alerts: settings.tab_switch_alerts,
            webcam_required: settings.webcam_required,
            screen_recording: settings.screen_recording,
            face_detected: false,
            multiple_faces: false,
            no_face_duration_seconds: 0,
            tab_switch_count: 0,
            copy_paste_attempts: 0,
            started_at: None,
            completed_at: None,
            duration_minutes: None,
            created_at: now,
        })
    }

    /// Start proctoring session
    pub async fn start_session(
        pool: &PgPool,
        session_id: uuid::Uuid,
    ) -> Result<ProctoringSession, String> {
        let now = Utc::now();

        sqlx::query!(
            "UPDATE proctoring_sessions SET status = 'in_progress', started_at = $1 WHERE id = $2",
            now,
            session_id
        )
        .execute(pool)
        .await
        .map_err(|e| e.to_string())?;

        get_session(pool, session_id)
            .await?
            .ok_or("Session not found".to_string())
    }

    /// End proctoring session
    pub async fn end_session(
        pool: &PgPool,
        session_id: uuid::Uuid,
    ) -> Result<ProctoringSession, String> {
        let now = Utc::now();

        // Get start time to calculate duration
        let start: Option<DateTime<Utc>> = sqlx::query_scalar!(
            "SELECT started_at FROM proctoring_sessions WHERE id = $1",
            session_id
        )
        .fetch_one(pool)
        .await
        .map_err(|e| e.to_string())?;

        let duration = start.map(|s| ((now - s).num_minutes()) as i32);

        // Determine if flagged based on violations
        let violation_count: i64 = sqlx::query_scalar!(
            "SELECT COUNT(*) FROM violation_events WHERE session_id = $1 AND severity IN ('high', 'critical')",
            session_id
        )
        .fetch_one(pool)
        .await
        .map_err(|e| e.to_string())?;

        let status = if violation_count > 0 {
            ProctoringStatus::Flagged
        } else {
            ProctoringStatus::Completed
        };

        sqlx::query!(
            "UPDATE proctoring_sessions SET status = $1, completed_at = $2, duration_minutes = $3 WHERE id = $4",
            format!("{:?}", status).to_lowercase(), now, duration, session_id
        )
        .execute(pool)
        .await
        .map_err(|e| e.to_string())?;

        get_session(pool, session_id)
            .await?
            .ok_or("Session not found".to_string())
    }

    /// Record a violation event
    pub async fn record_violation(
        pool: &PgPool,
        session_id: uuid::Uuid,
        violation_type: ViolationType,
        severity: ViolationSeverity,
        description: &str,
    ) -> Result<ViolationEvent, String> {
        let id = Uuid::new_v4();

        sqlx::query!(
            "INSERT INTO violation_events (id, session_id, violation_type, severity, description, timestamp)
             VALUES ($1, $2, $3, $4, $5, $6)",
            id, session_id, format!("{:?}", violation_type).to_lowercase(),
            format!("{:?}", severity).to_lowercase(), description, Utc::now()
        )
        .execute(pool)
        .await
        .map_err(|e| e.to_string())?;

        // Update session violation count
        sqlx::query!(
            "UPDATE proctoring_sessions SET 
             tab_switch_count = tab_switch_count + 1
             WHERE id = $1 AND $2 IN ('tab_switch', 'window_blur')",
            session_id,
            format!("{:?}", violation_type).to_lowercase()
        )
        .execute(pool)
        .await
        .ok();

        Ok(ViolationEvent {
            id,
            session_id,
            violation_type,
            severity,
            description: description.to_string(),
            evidence_url: None,
            timestamp: Utc::now(),
        })
    }

    /// Get violations for a session
    pub async fn get_violations(
        pool: &PgPool,
        session_id: uuid::Uuid,
    ) -> Result<Vec<ViolationEvent>, String> {
        let rows = sqlx::query!(
            "SELECT id, session_id, violation_type, severity, description, evidence_url, timestamp
             FROM violation_events WHERE session_id = $1 ORDER BY timestamp",
            session_id
        )
        .fetch_all(pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(rows
            .into_iter()
            .map(|r| ViolationEvent {
                id: r.id,
                session_id: r.session_id,
                violation_type: ViolationType::TabSwitch,
                severity: ViolationSeverity::Low,
                description: r.description,
                evidence_url: r.evidence_url,
                timestamp: r.timestamp,
            })
            .collect())
    }

    /// Get session by ID
    pub async fn get_session(
        pool: &PgPool,
        session_id: uuid::Uuid,
    ) -> Result<Option<ProctoringSession>, String> {
        let row = sqlx::query!(
            "SELECT id, exam_id, user_id, status, webcam_url, screen_url, audio_url,
             browser_lockdown, fullscreen_required, tab_switch_alerts, webcam_required,
             screen_recording, face_detected, multiple_faces, no_face_duration_seconds,
             tab_switch_count, copy_paste_attempts, started_at, completed_at, 
             duration_minutes, created_at
             FROM proctoring_sessions WHERE id = $1",
            session_id
        )
        .fetch_optional(pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(row.map(|r| ProctoringSession {
            id: r.id,
            exam_id: r.exam_id,
            user_id: r.user_id,
            status: ProctoringStatus::InProgress,
            webcam_url: r.webcam_url,
            screen_url: r.screen_url,
            audio_url: r.audio_url,
            browser_lockdown: r.browser_lockdown,
            fullscreen_required: r.fullscreen_required,
            tab_switch_alerts: r.tab_switch_alerts,
            webcam_required: r.webcam_required,
            screen_recording: r.screen_recording,
            face_detected: r.face_detected,
            multiple_faces: r.multiple_faces,
            no_face_duration_seconds: r.no_face_duration_seconds,
            tab_switch_count: r.tab_switch_count as i32,
            copy_paste_attempts: r.copy_paste_attempts as i32,
            started_at: r.started_at,
            completed_at: r.completed_at,
            duration_minutes: r.duration_minutes,
            created_at: r.created_at,
        }))
    }

    /// Get flagged sessions for review
    pub async fn get_review_queue(
        pool: &PgPool,
        institution_id: uuid::Uuid,
        limit: i64,
    ) -> Result<Vec<ReviewQueueItem>, String> {
        let rows = sqlx::query!(
            "SELECT ps.id, ps.exam_id, ps.user_id, ps.status, ps.tab_switch_count,
                    ps.face_detected, ps.multiple_faces, ps.started_at, 
                    COUNT(ve.id) as violation_count
             FROM proctoring_sessions ps
             LEFT JOIN violation_events ve ON ps.id = ve.session_id
             WHERE ps.status IN ('flagged', 'completed')
             AND ps.completed_at > NOW() - INTERVAL '7 days'
             GROUP BY ps.id
             HAVING ps.status = 'flagged' OR COUNT(ve.id) > 0
             ORDER BY violation_count DESC, ps.completed_at DESC
             LIMIT $1",
            limit
        )
        .fetch_all(pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(rows
            .into_iter()
            .map(|r| ReviewQueueItem {
                session_id: r.id,
                exam_id: r.exam_id,
                user_id: r.user_id,
                violation_count: r.violation_count as i32,
                tab_switch_count: r.tab_switch_count as i32,
                multiple_faces: r.multiple_faces,
                status: "pending".to_string(),
            })
            .collect())
    }

    /// Submit review for a session
    pub async fn submit_review(
        pool: &PgPool,
        session_id: uuid::Uuid,
        reviewer_id: uuid::Uuid,
        review_status: ReviewStatus,
        notes: Option<&str>,
    ) -> Result<ProctoringReview, String> {
        let now = Utc::now();
        let review_id = Uuid::new_v4();

        sqlx::query!(
            "INSERT INTO proctoring_reviews (id, session_id, reviewer_id, review_status, 
             notes, violation_count, reviewed_at, created_at)
             VALUES ($1, $2, $3, $4, $5, 0, $6, $7)",
            review_id,
            session_id,
            reviewer_id,
            format!("{:?}", review_status).to_lowercase(),
            notes,
            now,
            now
        )
        .execute(pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(ProctoringReview {
            id: review_id,
            session_id,
            reviewer_id: Some(reviewer_id),
            review_status,
            notes: notes.map(String::from),
            violation_count: 0,
            overall_score: None,
            reviewed_at: Some(now),
            created_at: now,
        })
    }

    /// Create identity verification
    pub async fn verify_identity(
        pool: &PgPool,
        user_id: uuid::Uuid,
        id_photo_url: Option<&str>,
        selfie_url: Option<&str>,
    ) -> Result<IdentityVerification, String> {
        let id = Uuid::new_v4();

        // In production: call face recognition service
        // For now, simulate verification
        let is_verified = true;
        let liveness_passed = true;
        let match_score = 0.95;

        sqlx::query!(
            "INSERT INTO identity_verifications (id, user_id, verification_type, 
             id_photo_url, selfie_photo_url, match_score, is_verified, liveness_passed, 
             verified_at, created_at)
             VALUES ($1, $2, 'photo_id', $3, $4, $5, $6, $7, $8, $9)",
            id,
            user_id,
            id_photo_url,
            selfie_url,
            match_score,
            is_verified,
            liveness_passed,
            if is_verified { Some(Utc::now()) } else { None },
            Utc::now()
        )
        .execute(pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(IdentityVerification {
            id,
            user_id,
            verification_type: IdentityType::PhotoID,
            id_photo_url: id_photo_url.map(String::from),
            selfie_photo_url: selfie_url.map(String::from),
            match_score: Some(match_score),
            is_verified,
            liveness_passed,
            verified_at: if is_verified { Some(Utc::now()) } else { None },
            created_at: Utc::now(),
        })
    }
}

/// Request/Response types
#[derive(Debug, Clone, Deserialize)]
pub struct ProctoringSettings {
    pub browser_lockdown: bool,
    pub fullscreen_required: bool,
    pub tab_switch_alerts: bool,
    pub webcam_required: bool,
    pub screen_recording: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct ReviewQueueItem {
    pub session_id: uuid::Uuid,
    pub exam_id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub violation_count: i32,
    pub tab_switch_count: i32,
    pub multiple_faces: bool,
    pub status: String,
}
