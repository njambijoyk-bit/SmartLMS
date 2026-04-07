// Models module - data structures
pub mod user;
pub mod institution;
pub mod course;
pub mod assessment;
pub mod live;
pub mod attendance;
pub mod backup;
pub mod abac;

pub use user::{User, LoginRequest, RegisterRequest, LoginResponse};
pub use institution::{Institution, CreateInstitutionRequest, UpdateInstitutionRequest, InstitutionListResponse};
pub use course::*;
pub use assessment::*;
pub use live::*;
pub use attendance::*;
pub use backup::*;
pub use abac::*;