//! Database access layer. Phase 1: institution registry (master DB) plus
//! per-institution users/roles/refresh_tokens. Phase 1 PRs #56–#57 will add
//! courses/enrollments/assessments.

pub mod institution;
pub mod refresh_token;
pub mod role;
pub mod user;
