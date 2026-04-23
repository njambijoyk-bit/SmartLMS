//! Database access layer.
//!
//! Phase 1:
//!   * institution         (master DB)
//!   * user / role /
//!     refresh_token       (per-institution DB, from PR #54)
//!   * course / module_db /
//!     enrollment          (per-institution DB, from PR #56)

pub mod course;
pub mod enrollment;
pub mod institution;
pub mod module_db;
pub mod refresh_token;
pub mod role;
pub mod user;
