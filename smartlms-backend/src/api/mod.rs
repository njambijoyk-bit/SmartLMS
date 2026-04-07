pub mod routes {
    pub fn auth_router() -> axum::Router { axum::Router::new() }
    pub fn institutions_router() -> axum::Router { axum::Router::new() }
    pub fn users_router() -> axum::Router { axum::Router::new() }
    pub fn courses_router() -> axum::Router { axum::Router::new() }
    pub fn assessments_router() -> axum::Router { axum::Router::new() }
    pub fn enrollments_router() -> axum::Router { axum::Router::new() }
}