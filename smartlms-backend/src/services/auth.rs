// Authentication service - login, registration, password management
use crate::models::user::{User, LoginRequest, LoginResponse, RegisterRequest};
use crate::services::jwt;
use sqlx::PgPool;
use bcrypt::{hash, verify, DEFAULT_COST};

/// Login user with email and password
pub async fn login(pool: &PgPool, req: &LoginRequest) -> Result<Option<LoginResponse>, String> {
    // Find user by email
    let user = crate::db::user::find_by_email(pool, &req.email)
        .await
        .map_err(|e| e.to_string())?;
    
    let user = match user {
        Some(u) => u,
        None => return Ok(None), // Invalid credentials
    };
    
    // Verify password
    if !verify(&req.password, &user.password_hash).map_err(|e| e.to_string())? {
        return Ok(None); // Invalid credentials
    }
    
    // Create JWT token
    let token = jwt::create_token(
        user.id,
        user.email.clone(),
        user.first_name.clone(),
        user.last_name.clone(),
        user.role.clone(),
        uuid::Uuid::new_v4(), // TODO: Get from institution context
    ).map_err(|e| e.to_string())?;
    
    let expires_in = jwt::get_expiration().num_seconds();
    
    Ok(Some(LoginResponse {
        token,
        user,
        expires_in,
    }))
}

/// Register new user
pub async fn register(pool: &PgPool, req: &RegisterRequest) -> Result<User, String> {
    // Check if email already exists
    if crate::db::user::find_by_email(pool, &req.email)
        .await
        .map_err(|e| e.to_string())?
        .is_some() 
    {
        return Err("Email already registered".to_string());
    }
    
    // Hash password
    let password_hash = hash(&req.password, DEFAULT_COST)
        .map_err(|e| e.to_string())?;
    
    // Create user
    let user = crate::db::user::create(
        pool,
        &req.email,
        &password_hash,
        &req.first_name,
        &req.last_name,
        &req.role,
    ).await.map_err(|e| e.to_string())?;
    
    Ok(user)
}

/// Change password
pub async fn change_password(
    pool: &PgPool,
    user_id: uuid::Uuid,
    old_password: &str,
    new_password: &str,
) -> Result<bool, String> {
    // Get user
    let user = crate::db::user::find_by_id(pool, user_id)
        .await
        .map_err(|e| e.to_string())?
        .ok_or("User not found")?;
    
    // Verify old password
    if !verify(old_password, &user.password_hash).map_err(|e| e.to_string())? {
        return Err("Invalid current password".to_string());
    }
    
    // Hash new password
    let new_hash = hash(new_password, DEFAULT_COST)
        .map_err(|e| e.to_string())?;
    
    // Update password
    crate::db::user::update_password(pool, user_id, &new_hash)
        .await
        .map_err(|e| e.to_string())?;
    
    Ok(true)
}

/// Request password reset (generate reset token)
pub async fn request_password_reset(pool: &PgPool, email: &str) -> Result<Option<String>, String> {
    let user = crate::db::user::find_by_email(pool, email)
        .await
        .map_err(|e| e.to_string())?;
    
    if user.is_none() {
        // Don't reveal if email exists
        return Ok(None);
    }
    
    // Generate reset token (in production, store in DB with expiration)
    let reset_token = uuid::Uuid::new_v4().to_string();
    
    // TODO: Store token in DB with expiration
    
    Ok(Some(reset_token))
}

/// Reset password with token
pub async fn reset_password(pool: &PgPool, token: &str, new_password: &str) -> Result<bool, String> {
    // TODO: Validate token from DB
    // TODO: Look up user by token, verify expiration
    
    Err("Not implemented".to_string())
}