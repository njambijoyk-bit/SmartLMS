//! Data structures used across the API/service layer. Phase 1: institutions
//! (master DB) + users/roles/auth (per-institution DB) + onboarding DTOs.

pub mod auth;
pub mod institution;
pub mod onboarding;
pub mod user;

pub use institution::{
    CreateInstitutionRequest, Institution, InstitutionListResponse, UpdateInstitutionRequest,
};
pub use user::{CreateUserRequest, RoleCode, UpdateUserRequest, User, UserRecord, UserWithRoles};
