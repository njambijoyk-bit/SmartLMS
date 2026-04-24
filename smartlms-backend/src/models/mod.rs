//! Data structures used across the API/service layer.
//!
//! Phase 1: institutions (master DB) + users/roles/auth + courses/modules/
//! lessons/enrollments + assessments/questions/attempts + onboarding DTOs.

pub mod assessment;
pub mod auth;
pub mod course;
pub mod institution;
pub mod onboarding;
pub mod user;

pub use institution::{
    CreateInstitutionRequest, Institution, InstitutionListResponse, UpdateInstitutionRequest,
};
pub use user::{CreateUserRequest, RoleCode, UpdateUserRequest, User, UserRecord, UserWithRoles};
