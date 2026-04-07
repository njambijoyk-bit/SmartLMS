//! Application state utilities

use std::sync::Arc;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::db::DbPool;

/// Application state shared across all requests
#[derive(Clone)]
pub struct AppState {
    pub db: DbPool,
    pub institution_cache: Arc<dashmap::DashMap<String, InstitutionContext>>,
    pub jwt_secret: String,
    pub jwt_expiry_hours: i64,
}

impl AppState {
    pub fn new(db: DbPool) -> Self {
        Self {
            db,
            institution_cache: Arc::new(dashmap::DashMap::new()),
            jwt_secret: std::env::var("JWT_SECRET")
                .unwrap_or_else(|_| "smartlms-dev-secret-change-in-production".to_string()),
            jwt_expiry_hours: std::env::var("JWT_EXPIRY_HOURS")
                .unwrap_or_else(|_| "24".to_string())
                .parse()
                .unwrap_or(24),
        }
    }
}

/// Institution context - extracted from host header on each request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstitutionContext {
    pub id: uuid::Uuid,
    pub slug: String,
    pub name: String,
    pub domain: Option<String>,
    pub plan_tier: PlanTier,
    pub feature_flags: Vec<String>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum PlanTier {
    Starter,
    Growth,
    Enterprise,
}

impl PlanTier {
    pub fn has_feature(&self, feature: &str) -> bool {
        match self {
            PlanTier::Starter => matches!(feature, "courses" | "assessments" | "communication"),
            PlanTier::Growth => matches!(
                feature,
                "courses"
                    | "assessments"
                    | "communication"
                    | "live_classes"
                    | "library"
                    | "exam_bank"
                    | "attendance"
                    | "fees"
            ),
            PlanTier::Enterprise => true, // All features
        }
    }
}

impl Default for PlanTier {
    fn default() -> Self {
        PlanTier::Starter
    }
}

/// Request-scoped institution context
#[derive(Debug, Clone)]
pub struct RequestInstitutionContext {
    pub institution: InstitutionContext,
    pub user_id: Option<uuid::Uuid>,
    pub user_role: Option<UserRole>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum UserRole {
    Admin,
    Instructor,
    Learner,
    Parent,
    Advisor,
    Observer,
    Alumni,
}

impl std::fmt::Display for UserRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserRole::Admin => write!(f, "admin"),
            UserRole::Instructor => write!(f, "instructor"),
            UserRole::Learner => write!(f, "learner"),
            UserRole::Parent => write!(f, "parent"),
            UserRole::Advisor => write!(f, "advisor"),
            UserRole::Observer => write!(f, "observer"),
            UserRole::Alumni => write!(f, "alumni"),
        }
    }
}

/// Middleware trait for request processing
#[async_trait]
pub trait Middleware {
    async fn handle(&self, request: axum::extract::Request, next: axum::middleware::Next) -> axum::response::Response;
}