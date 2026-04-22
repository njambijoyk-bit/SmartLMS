//! SmartLMS Backend
//!
//! Post-reset (2026-04-22): trimmed to the Phase 0 core (tenant router,
//! institutions). Phase 1 will add users, courses, enrollments, assessments.

pub mod api;
pub mod db;
pub mod middleware;
pub mod models;
pub mod services;
pub mod tenant;
