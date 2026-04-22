//! Data structures used across the API/service layer. Phase 0: institutions
//! only. Phase 1 adds users/courses/enrollments/assessments.

pub mod institution;

pub use institution::{
    CreateInstitutionRequest, Institution, InstitutionListResponse, UpdateInstitutionRequest,
};
