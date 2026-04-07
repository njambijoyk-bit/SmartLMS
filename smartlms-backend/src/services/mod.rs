// Services module - business logic layer
pub mod auth;
pub mod jwt;
pub mod rbac;
pub mod security;
pub mod onboarding;
pub mod license;
pub mod whitelabel;
pub mod sso;
pub mod courses;
pub mod assessments;
pub mod live;
pub mod attendance;

pub use auth::{login, register, change_password, request_password_reset, reset_password};