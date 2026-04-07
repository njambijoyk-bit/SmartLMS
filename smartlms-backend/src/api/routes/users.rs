//! Users API routes

use axum::{
    extract::State,
    response::Json,
    routing::{delete, get, post, put},
    Router,
};

use crate::utils::app_state::AppState;

/// Users router
pub fn router() -> Router {
    Router::new()
        .route("/api/users", get(list_users))
        .route("/api/users", post(create_user))
        .route("/api/users/bulk", post(bulk_create_users))
        .route("/api/users/import", post(import_users))
        .route("/api/users/:id", get(get_user))
        .route("/api/users/:id", put(update_user))
        .route("/api/users/:id", delete(delete_user))
        .route("/api/users/:id/activate", post(activate_user))
        .route("/api/users/:id/suspend", post(suspend_user))
        .route("/api/users/:id/reset-password", post(reset_password))
}

/// List users
async fn list_users(
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, AppError> {
    // Mock data for demo
    let users = vec![
        serde_json::json!({
            "id": uuid::Uuid::new_v4(),
            "email": "admin@demo.edu",
            "first_name": "Admin",
            "last_name": "User",
            "role": "admin",
            "status": "active"
        }),
        serde_json::json!({
            "id": uuid::Uuid::new_v4(),
            "email": "instructor@demo.edu",
            "first_name": "John",
            "last_name": "Doe",
            "role": "instructor",
            "status": "active"
        }),
    ];
    
    Ok(Json(serde_json::json!({
        "users": users,
        "total": users.len()
    })))
}

/// Create user
async fn create_user(
    State(state): State<AppState>,
    Json(payload): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, AppError> {
    Ok(Json(serde_json::json!({
        "message": "User created",
        "id": uuid::Uuid::new_v4()
    })))
}

/// Bulk create users
async fn bulk_create_users(
    State(state): State<AppState>,
    Json(payload): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, AppError> {
    Ok(Json(serde_json::json!({
        "message": "Users created",
        "count": 0
    })))
}

/// Import users from CSV
async fn import_users(
    State(state): State<AppState>,
    Json(payload): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, AppError> {
    Ok(Json(serde_json::json!({
        "message": "Users imported",
        "imported": 0,
        "failed": []
    })))
}

/// Get user by ID
async fn get_user(
    State(state): State<AppState>,
    axum::extract::Path(id): axum::extract::Path<uuid::Uuid>,
) -> Result<Json<serde_json::Value>, AppError> {
    Ok(Json(serde_json::json!({
        "id": id,
        "email": "user@example.com",
        "first_name": "Demo",
        "last_name": "User",
        "role": "learner",
        "status": "active"
    })))
}

/// Update user
async fn update_user(
    State(state): State<AppState>,
    axum::extract::Path(id): axum::extract::Path<uuid::Uuid>,
    Json(payload): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, AppError> {
    Ok(Json(serde_json::json!({
        "message": "User updated",
        "id": id
    })))
}

/// Delete user
async fn delete_user(
    State(state): State<AppState>,
    axum::extract::Path(id): axum::extract::Path<uuid::Uuid>,
) -> Result<Json<serde_json::Value>, AppError> {
    Ok(Json(serde_json::json!({
        "message": "User deleted"
    })))
}

/// Activate user
async fn activate_user(
    State(state): State<AppState>,
    axum::extract::Path(id): axum::extract::Path<uuid::Uuid>,
) -> Result<Json<serde_json::Value>, AppError> {
    Ok(Json(serde_json::json!({
        "message": "User activated"
    })))
}

/// Suspend user
async fn suspend_user(
    State(state): State<AppState>,
    axum::extract::Path(id): axum::extract::Path<uuid::Uuid>,
) -> Result<Json<serde_json::Value>, AppError> {
    Ok(Json(serde_json::json!({
        "message": "User suspended"
    })))
}

/// Reset user password
async fn reset_password(
    State(state): State<AppState>,
    axum::extract::Path(id): axum::extract::Path<uuid::Uuid>,
) -> Result<Json<serde_json::Value>, AppError> {
    Ok(Json(serde_json::json!({
        "message": "Password reset email sent"
    })))
}

use crate::api::routes::auth::AppError;