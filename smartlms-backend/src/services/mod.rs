//! Business-logic layer.
//!
//! Phase 1: password hashing, JWT issuance, register/login/refresh/logout,
//! institution onboarding, and course / module / lesson / enrollment flows.

pub mod auth;
pub mod course;
pub mod jwt;
pub mod onboarding;
pub mod password;
