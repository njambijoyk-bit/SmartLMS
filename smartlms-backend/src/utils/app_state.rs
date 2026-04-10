#[derive(Clone)]
pub struct AppState {
    pub jwt_secret: String,
    pub db_pool: u32,
}

impl AppState {
    pub fn new(db: u32) -> Self {
        Self {
            jwt_secret: std::env::var("JWT_SECRET").unwrap_or_else(|_| "dev-secret".to_string()),
            db_pool: db,
        }
    }
}