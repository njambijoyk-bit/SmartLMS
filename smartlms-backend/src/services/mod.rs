//! Business-logic layer. Phase 1: password hashing, JWT issuance, the
//! register/login/refresh/logout flows, and self-service institution
//! onboarding.

pub mod auth;
pub mod jwt;
pub mod onboarding;
pub mod password;
