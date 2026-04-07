#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct User {
    pub id: uuid::Uuid,
    pub email: String,
    pub password_hash: String,
    pub first_name: String,
    pub last_name: String,
    pub role: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, serde::Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub user: User,
    pub expires_in: i64,
}