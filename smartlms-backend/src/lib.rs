//! SmartLMS Backend
//!
//! Phase 1 surface: multi-tenant router (from PR #53) plus users, RBAC, and
//! auth (this PR). Phase 1 follow-ups will add institutions onboarding,
//! courses, enrollments, and assessments against the master reference spec.

pub mod api;
pub mod db;
pub mod middleware;
pub mod models;
pub mod services;
pub mod tenant;
