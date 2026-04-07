//! User models

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// User entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub institution_id: Uuid,
    pub email: String,
    pub password_hash: String,
    pub first_name: String,
    pub last_name: String,
    pub role: UserRole,
    pub status: UserStatus,
    pub avatar_url: Option<String>,
    pub phone: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub last_login_at: Option<DateTime<Utc>>,
}

impl User {
    pub fn new(
        institution_id: Uuid,
        email: String,
        password_hash: String,
        first_name: String,
        last_name: String,
        role: UserRole,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            institution_id,
            email,
            password_hash,
            first_name,
            last_name,
            role,
            status: UserStatus::Active,
            avatar_url: None,
            phone: None,
            created_at: now,
            updated_at: now,
            last_login_at: None,
        }
    }

    pub fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }
}

/// User role enum
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

impl std::str::FromStr for UserRole {
    type Err = String;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "admin" => Ok(UserRole::Admin),
            "instructor" => Ok(UserRole::Instructor),
            "learner" => Ok(UserRole::Learner),
            "parent" => Ok(UserRole::Parent),
            "advisor" => Ok(UserRole::Advisor),
            "observer" => Ok(UserRole::Observer),
            "alumni" => Ok(UserRole::Alumni),
            _ => Err(format!("Unknown role: {}", s)),
        }
    }
}

/// User status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum UserStatus {
    Active,
    Suspended,
    Pending,
    Deleted,
}

/// Create user request
#[derive(Debug, Deserialize, Serialize)]
pub struct CreateUserRequest {
    pub email: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
    pub role: UserRole,
    pub phone: Option<String>,
}

/// Update user request
#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateUserRequest {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub phone: Option<String>,
    pub avatar_url: Option<String>,
    pub role: Option<UserRole>,
    pub status: Option<UserStatus>,
}

/// User response (without password)
#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub institution_id: Uuid,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub full_name: String,
    pub role: UserRole,
    pub status: UserStatus,
    pub avatar_url: Option<String>,
    pub phone: Option<String>,
    pub created_at: DateTime<Utc>,
    pub last_login_at: Option<DateTime<Utc>>,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            institution_id: user.institution_id,
            email: user.email,
            first_name: user.first_name,
            last_name: user.last_name,
            full_name: user.full_name(),
            role: user.role,
            status: user.status,
            avatar_url: user.avatar_url,
            phone: user.phone,
            created_at: user.created_at,
            last_login_at: user.last_login_at,
        }
    }
}

/// Login request
#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

/// Login response
#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub user: UserResponse,
    pub expires_in: i64,
}

/// User list filter
#[derive(Debug, Deserialize, Default)]
pub struct UserFilter {
    pub role: Option<UserRole>,
    pub status: Option<UserStatus>,
    pub search: Option<String>,
    pub page: Option<usize>,
    pub page_size: Option<usize>,
}