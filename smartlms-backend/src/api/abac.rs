// ABAC Policy API routes - CRUD for policies
use axum::{
    extract::{State, Json, Path, Query, Extension},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post, put, delete},
    Router,
};
use crate::models::abac::*;
use crate::services::abac as abac_service;
use crate::tenant::InstitutionCtx;
use serde::Deserialize;

/// List policies for an institution
pub async fn list_policies(
    Extension(ctx): Extension<InstitutionCtx>,
    Query(_query): Query<PoliciesQuery>,
) -> Result<Json<Vec<AbacPolicy>>, (StatusCode, String)> {
    // In production, fetch from DB
    Ok(Json(vec![]))
}

/// Get a specific policy
pub async fn get_policy(
    Extension(ctx): Extension<InstitutionCtx>,
    Path(policy_id): Path<uuid::Uuid>,
) -> Result<Json<AbacPolicy>, (StatusCode, String)> {
    // In production, fetch from DB
    Err((StatusCode::NOT_FOUND, "Policy not found".to_string()))
}

/// Create a new policy
pub async fn create_policy(
    Extension(ctx): Extension<InstitutionCtx>,
    Json(req): Json<CreatePolicyRequest>,
) -> Result<Json<AbacPolicy>, (StatusCode, String)> {
    let policy = AbacPolicy {
        id: uuid::Uuid::new_v4(),
        institution_id: ctx.id,
        name: req.name,
        description: req.description,
        effect: req.effect,
        subjects: req.subjects,
        actions: req.actions,
        resources: req.resources,
        conditions: req.conditions,
        priority: req.priority.unwrap_or(0),
        enabled: true,
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };
    
    // In production, save to DB
    Ok(Json(policy))
}

/// Update an existing policy
pub async fn update_policy(
    Extension(ctx): Extension<InstitutionCtx>,
    Path(policy_id): Path<uuid::Uuid>,
    Json(req): Json<UpdatePolicyRequest>,
) -> Result<Json<AbacPolicy>, (StatusCode, String)> {
    // In production, update in DB
    let policy = AbacPolicy {
        id: policy_id,
        institution_id: ctx.id,
        name: req.name.unwrap_or_default(),
        description: req.description,
        effect: req.effect.unwrap_or(PolicyEffect::Allow),
        subjects: req.subjects.unwrap_or_default(),
        actions: req.actions.unwrap_or_default(),
        resources: req.resources.unwrap_or_default(),
        conditions: req.conditions,
        priority: req.priority.unwrap_or(0),
        enabled: req.enabled.unwrap_or(true),
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };
    
    Ok(Json(policy))
}

/// Delete a policy
pub async fn delete_policy(
    Extension(ctx): Extension<InstitutionCtx>,
    Path(policy_id): Path<uuid::Uuid>,
) -> Result<StatusCode, (StatusCode, String)> {
    // In production, delete from DB
    Ok(StatusCode::NO_CONTENT)
}

/// Test a policy against a sample request
pub async fn test_policy(
    Extension(ctx): Extension<InstitutionCtx>,
    Json(req): Json<TestPolicyRequest>,
) -> Result<Json<abac_service::EvaluationResult>, (StatusCode, String)> {
    // Build the access request from the test input
    let request = abac_service::AccessRequest {
        user_id: req.user_id,
        user_roles: req.user_roles,
        user_attributes: req.user_attributes,
        action: req.action,
        resource_type: req.resource_type,
        resource_id: req.resource_id,
        resource_attributes: req.resource_attributes,
        environment: req.environment,
    };
    
    // If policy_id provided, test that specific policy
    if let Some(policy_id) = req.policy_id {
        // Fetch policy and evaluate
        let result = abac_service::EvaluationResult {
            decision: abac_service::AccessDecision::Allow,
            matched_policies: vec![policy_id],
            evaluated_conditions: vec![],
        };
        return Ok(Json(result));
    }
    
    // Otherwise evaluate against all enabled policies
    let result = abac_service::evaluate_access(&request, &[]);
    Ok(Json(result))
}

/// Get available policy templates
pub async fn get_templates() -> Json<Vec<PolicyTemplate>> {
    Json(vec![
        PolicyTemplate {
            id: "time-based".to_string(),
            name: "Time-Based Access".to_string(),
            description: "Restrict access to specific hours and days".to_string(),
            icon: "clock".to_string(),
        },
        PolicyTemplate {
            id: "department".to_string(),
            name: "Department Access".to_string(),
            description: "Restrict access based on department membership".to_string(),
            icon: "building".to_string(),
        },
        PolicyTemplate {
            id: "enrollment-status".to_string(),
            name: "Enrollment Status".to_string(),
            description: "Allow access based on enrollment status".to_string(),
            icon: "user-check".to_string(),
        },
        PolicyTemplate {
            id: "ip-restriction".to_string(),
            name: "IP Restriction".to_string(),
            description: "Allow access only from specific IP ranges".to_string(),
            icon: "globe".to_string(),
        },
        PolicyTemplate {
            id: "course-ownership".to_string(),
            name: "Course Ownership".to_string(),
            description: "Instructors can only modify their own courses".to_string(),
            icon: "book".to_string(),
        },
    ])
}

// Request/Response types
#[derive(Debug, Deserialize)]
pub struct PoliciesQuery {
    pub enabled: Option<bool>,
    pub effect: Option<PolicyEffect>,
}

#[derive(Debug, Deserialize)]
pub struct CreatePolicyRequest {
    pub name: String,
    pub description: Option<String>,
    pub effect: PolicyEffect,
    pub subjects: Vec<SubjectSelector>,
    pub actions: Vec<String>,
    pub resources: Vec<ResourceSelector>,
    pub conditions: Option<ConditionExpression>,
    pub priority: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct UpdatePolicyRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub effect: Option<PolicyEffect>,
    pub subjects: Option<Vec<SubjectSelector>>,
    pub actions: Option<Vec<String>>,
    pub resources: Option<Vec<ResourceSelector>>,
    pub conditions: Option<ConditionExpression>,
    pub priority: Option<i32>,
    pub enabled: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct TestPolicyRequest {
    pub policy_id: Option<uuid::Uuid>,
    pub user_id: uuid::Uuid,
    pub user_roles: Vec<String>,
    pub user_attributes: std::collections::HashMap<String, abac_service::AttributeValue>,
    pub action: String,
    pub resource_type: String,
    pub resource_id: Option<uuid::Uuid>,
    pub resource_attributes: std::collections::HashMap<String, abac_service::AttributeValue>,
    pub environment: abac_service::EnvironmentAttributes,
}

#[derive(Debug, serde::Serialize)]
pub struct PolicyTemplate {
    pub id: String,
    pub name: String,
    pub description: String,
    pub icon: String,
}

/// Create ABAC router
pub fn abac_router() -> Router {
    Router::new()
        .route("/policies", get(list_policies))
        .route("/policies", post(create_policy))
        .route("/policies/:id", get(get_policy))
        .route("/policies/:id", put(update_policy))
        .route("/policies/:id", delete(delete_policy))
        .route("/policies/:id/test", post(test_policy))
        .route("/templates", get(get_templates))
}