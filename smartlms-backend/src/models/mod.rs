// Models module - data structures
pub mod user;
pub mod institution;

pub use user::{User, LoginRequest, RegisterRequest, LoginResponse};
pub use institution::{Institution, CreateInstitutionRequest, UpdateInstitutionRequest, InstitutionListResponse};