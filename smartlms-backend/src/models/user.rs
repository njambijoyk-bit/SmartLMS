//! User + RBAC models for the per-institution schema.
//!
//! `User` mirrors `public.users` in `migrations_institution/001_users_and_rbac.sql`.
//! `password_hash` is intentionally not serialised out of the API.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Role codes mirror `roles.code` in the institution DB and the 8-role
/// catalogue in master ref §4 module 5. The `code` values are the canonical
/// strings used in JWT claims and RBAC checks.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RoleCode {
    Admin,
    Instructor,
    Learner,
    Observer,
    Parent,
    Advisor,
    Counsellor,
    Alumni,
}

impl RoleCode {
    pub fn as_str(self) -> &'static str {
        match self {
            RoleCode::Admin => "admin",
            RoleCode::Instructor => "instructor",
            RoleCode::Learner => "learner",
            RoleCode::Observer => "observer",
            RoleCode::Parent => "parent",
            RoleCode::Advisor => "advisor",
            RoleCode::Counsellor => "counsellor",
            RoleCode::Alumni => "alumni",
        }
    }

    pub fn from_code(s: &str) -> Option<Self> {
        match s {
            "admin" => Some(RoleCode::Admin),
            "instructor" => Some(RoleCode::Instructor),
            "learner" => Some(RoleCode::Learner),
            "observer" => Some(RoleCode::Observer),
            "parent" => Some(RoleCode::Parent),
            "advisor" => Some(RoleCode::Advisor),
            "counsellor" => Some(RoleCode::Counsellor),
            "alumni" => Some(RoleCode::Alumni),
            _ => None,
        }
    }
}

/// User record (minus password_hash — that never leaves the DB layer).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: uuid::Uuid,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub display_name: Option<String>,
    pub phone: Option<String>,
    pub avatar_url: Option<String>,
    pub locale: String,
    pub timezone: String,
    pub is_active: bool,
    pub is_verified: bool,
    pub last_login_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// User record enriched with role codes. Returned from /users/me etc.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserWithRoles {
    #[serde(flatten)]
    pub user: User,
    pub roles: Vec<String>,
}

/// Internal DB row that includes password_hash and lockout state. Never
/// serialised — only used inside the auth service.
#[derive(Debug, Clone)]
pub struct UserRecord {
    pub user: User,
    pub password_hash: Option<String>,
    pub failed_login_count: i32,
    pub locked_until: Option<DateTime<Utc>>,
    pub deleted_at: Option<DateTime<Utc>>,
}

/// Request body for admin-initiated user creation.
#[derive(Debug, Deserialize)]
pub struct CreateUserRequest {
    pub email: String,
    pub password: Option<String>,
    pub first_name: String,
    pub last_name: String,
    pub roles: Vec<RoleCode>,
}

/// Partial update.
#[derive(Debug, Deserialize, Default)]
pub struct UpdateUserRequest {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub display_name: Option<String>,
    pub phone: Option<String>,
    pub locale: Option<String>,
    pub timezone: Option<String>,
    pub is_active: Option<bool>,
}
