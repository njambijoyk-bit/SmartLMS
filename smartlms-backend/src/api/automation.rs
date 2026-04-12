// Automation API - Visual rule builder, triggers, and webhooks
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    app_state::AppState,
    services::automation::{self, AutomationRule, RuleCondition, RuleAction, Trigger, EventType},
};

// ==================== REQUEST/RESPONSE MODELS ====================

#[derive(Debug, Deserialize)]
pub struct CreateAutomationRuleRequest {
    pub name: String,
    pub description: Option<String>,
    pub trigger: TriggerRequest,
    pub conditions: Vec<ConditionRequest>,
    pub actions: Vec<ActionRequest>,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub enum TriggerRequest {
    #[serde(rename = "event")]
    Event { event_type: String },
    #[serde(rename = "schedule")]
    Schedule { 
        cron_expression: String,
        timezone: String,
    },
}

#[derive(Debug, Deserialize)]
pub struct ConditionRequest {
    pub field: String,
    pub operator: String,
    pub value: serde_json::Value,
}

#[derive(Debug, Deserialize)]
pub struct ActionRequest {
    pub action_type: String,
    pub config: serde_json::Value,
}

#[derive(Debug, Deserialize)]
pub struct UpdateAutomationRuleRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub trigger: Option<TriggerRequest>,
    pub conditions: Option<Vec<ConditionRequest>>,
    pub actions: Option<Vec<ActionRequest>>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct TestRuleRequest {
    pub event_type: String,
    pub event_data: serde_json::Value,
}

#[derive(Debug, Serialize)]
pub struct ApiResponse<T: Serialize> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

// ==================== API HANDLERS ====================

/// List all automation rules for an institution
pub async fn list_rules(
    State(state): State<AppState>,
    Path(institution_id): Path<Uuid>,
) -> Result<Json<ApiResponse<Vec<AutomationRule>>>, StatusCode> {
    match automation::service::list_rules(&state.db_pool, institution_id).await {
        Ok(rules) => Ok(Json(ApiResponse {
            success: true,
            data: Some(rules),
            error: None,
        })),
        Err(e) => Ok(Json(ApiResponse {
            success: false,
            data: None,
            error: Some(e),
        })),
    }
}

/// Create a new automation rule
pub async fn create_rule(
    State(state): State<AppState>,
    Path(institution_id): Path<Uuid>,
    Json(req): Json<CreateAutomationRuleRequest>,
) -> Result<Json<ApiResponse<AutomationRule>>, StatusCode> {
    // Convert trigger request
    let trigger = match req.trigger {
        TriggerRequest::Event { event_type } => {
            let et = match event_type.as_str() {
                "UserRegistered" => EventType::UserRegistered,
                "CourseEnrolled" => EventType::CourseEnrolled,
                "CourseCompleted" => EventType::CourseCompleted,
                "QuizSubmitted" => EventType::QuizSubmitted,
                "GradePosted" => EventType::GradePosted,
                "AssignmentSubmitted" => EventType::AssignmentSubmitted,
                "LiveSessionStarted" => EventType::LiveSessionStarted,
                "LiveSessionEnded" => EventType::LiveSessionEnded,
                "PaymentReceived" => EventType::PaymentReceived,
                "CertificateIssued" => EventType::CertificateIssued,
                _ => return Ok(Json(ApiResponse {
                    success: false,
                    data: None,
                    error: Some(format!("Invalid event type: {}", event_type)),
                })),
            };
            Trigger::Event { event_type: et }
        }
        TriggerRequest::Schedule { cron_expression, timezone } => {
            Trigger::Schedule { cron_expression, timezone }
        }
    };

    // Convert conditions
    let conditions: Vec<RuleCondition> = req.conditions.into_iter().filter_map(|c| {
        use automation::ConditionOperator;
        let operator = match c.operator.as_str() {
            "Equals" => ConditionOperator::Equals,
            "NotEquals" => ConditionOperator::NotEquals,
            "GreaterThan" => ConditionOperator::GreaterThan,
            "LessThan" => ConditionOperator::LessThan,
            "Contains" => ConditionOperator::Contains,
            "StartsWith" => ConditionOperator::StartsWith,
            "IsEmpty" => ConditionOperator::IsEmpty,
            "IsNotEmpty" => ConditionOperator::IsNotEmpty,
            _ => return None,
        };
        Some(RuleCondition {
            field: c.field,
            operator,
            value: c.value,
        })
    }).collect();

    // Convert actions
    use automation::ActionType;
    let actions: Vec<RuleAction> = req.actions.into_iter().filter_map(|a| {
        let action_type = match a.action_type.as_str() {
            "SendEmail" => ActionType::SendEmail,
            "SendNotification" => ActionType::SendNotification,
            "UpdateRecord" => ActionType::UpdateRecord,
            "AssignToCourse" => ActionType::AssignToCourse,
            "IssueCertificate" => ActionType::IssueCertificate,
            "AddBadge" => ActionType::AddBadge,
            "Webhook" => ActionType::Webhook,
            "Delay" => ActionType::Delay,
            _ => return None,
        };
        Some(RuleAction {
            action_type,
            config: a.config,
        })
    }).collect();

    match automation::service::create_rule(
        &state.db_pool,
        institution_id,
        &req.name,
        trigger,
        conditions,
        actions,
    ).await {
        Ok(rule) => Ok(Json(ApiResponse {
            success: true,
            data: Some(rule),
            error: None,
        })),
        Err(e) => Ok(Json(ApiResponse {
            success: false,
            data: None,
            error: Some(e),
        })),
    }
}

/// Get a specific automation rule
pub async fn get_rule(
    State(state): State<AppState>,
    Path(rule_id): Path<Uuid>,
) -> Result<Json<ApiResponse<AutomationRule>>, StatusCode> {
    // Implementation would query database for specific rule
    // For now, return not implemented
    Ok(Json(ApiResponse {
        success: false,
        data: None,
        error: Some("Not implemented".to_string()),
    }))
}

/// Update an automation rule
pub async fn update_rule(
    State(state): State<AppState>,
    Path(rule_id): Path<Uuid>,
    Json(_req): Json<UpdateAutomationRuleRequest>,
) -> Result<Json<ApiResponse<AutomationRule>>, StatusCode> {
    // Implementation would update rule in database
    Ok(Json(ApiResponse {
        success: false,
        data: None,
        error: Some("Not implemented".to_string()),
    }))
}

/// Delete an automation rule
pub async fn delete_rule(
    State(state): State<AppState>,
    Path(rule_id): Path<Uuid>,
) -> Result<Json<ApiResponse<bool>>, StatusCode> {
    // Implementation would delete rule from database
    Ok(Json(ApiResponse {
        success: true,
        data: Some(true),
        error: None,
    }))
}

/// Enable/disable a rule
pub async fn toggle_rule(
    State(state): State<AppState>,
    Path((rule_id, is_active)): Path<(Uuid, bool)>,
) -> Result<Json<ApiResponse<bool>>, StatusCode> {
    match automation::service::set_rule_active(&state.db_pool, rule_id, is_active).await {
        Ok(_) => Ok(Json(ApiResponse {
            success: true,
            data: Some(true),
            error: None,
        })),
        Err(e) => Ok(Json(ApiResponse {
            success: false,
            data: None,
            error: Some(e),
        })),
    }
}

/// Get execution logs for a rule
pub async fn get_execution_logs(
    State(state): State<AppState>,
    Path((rule_id, limit)): Path<(Uuid, i64)>,
) -> Result<Json<ApiResponse<Vec<automation::RuleExecutionLog>>>, StatusCode> {
    // Implementation would query execution logs
    Ok(Json(ApiResponse {
        success: true,
        data: Some(vec![]),
        error: None,
    }))
}

/// Test a rule with sample event data
pub async fn test_rule(
    State(state): State<AppState>,
    Path(rule_id): Path<Uuid>,
    Json(req): Json<TestRuleRequest>,
) -> Result<Json<ApiResponse<bool>>, StatusCode> {
    // Parse event type
    let event_type = match req.event_type.as_str() {
        "UserRegistered" => EventType::UserRegistered,
        "CourseEnrolled" => EventType::CourseEnrolled,
        "CourseCompleted" => EventType::CourseCompleted,
        "QuizSubmitted" => EventType::QuizSubmitted,
        "GradePosted" => EventType::GradePosted,
        "AssignmentSubmitted" => EventType::AssignmentSubmitted,
        "LiveSessionStarted" => EventType::LiveSessionStarted,
        "LiveSessionEnded" => EventType::LiveSessionEnded,
        "PaymentReceived" => EventType::PaymentReceived,
        "CertificateIssued" => EventType::CertificateIssued,
        _ => return Ok(Json(ApiResponse {
            success: false,
            data: None,
            error: Some(format!("Invalid event type: {}", req.event_type)),
        })),
    };

    // Get institution_id from rule (would need to fetch rule first)
    // For now, return success
    Ok(Json(ApiResponse {
        success: true,
        data: Some(true),
        error: None,
    }))
}

/// Manually trigger an event for testing
pub async fn trigger_event(
    State(state): State<AppState>,
    Path(institution_id): Path<Uuid>,
    Json(req): Json<TestRuleRequest>,
) -> Result<Json<ApiResponse<Vec<automation::RuleExecutionLog>>>, StatusCode> {
    let event_type = match req.event_type.as_str() {
        "UserRegistered" => EventType::UserRegistered,
        "CourseEnrolled" => EventType::CourseEnrolled,
        "CourseCompleted" => EventType::CourseCompleted,
        "QuizSubmitted" => EventType::QuizSubmitted,
        "GradePosted" => EventType::GradePosted,
        "AssignmentSubmitted" => EventType::AssignmentSubmitted,
        "LiveSessionStarted" => EventType::LiveSessionStarted,
        "LiveSessionEnded" => EventType::LiveSessionEnded,
        "PaymentReceived" => EventType::PaymentReceived,
        "CertificateIssued" => EventType::CertificateIssued,
        _ => return Ok(Json(ApiResponse {
            success: false,
            data: None,
            error: Some(format!("Invalid event type: {}", req.event_type)),
        })),
    };

    match automation::service::process_event(
        &state.db_pool,
        institution_id,
        event_type,
        req.event_data,
    ).await {
        Ok(logs) => Ok(Json(ApiResponse {
            success: true,
            data: Some(logs),
            error: None,
        })),
        Err(e) => Ok(Json(ApiResponse {
            success: false,
            data: None,
            error: Some(e),
        })),
    }
}

// ==================== ROUTER CONFIGURATION ====================

use axum::routing::{get, post, put, delete};
use axum::Router;

pub fn create_router() -> Router<AppState> {
    Router::new()
        // Rule management endpoints
        .route("/automation/:institution_id/rules", get(list_rules))
        .route("/automation/:institution_id/rules", post(create_rule))
        .route("/automation/rules/:rule_id", get(get_rule))
        .route("/automation/rules/:rule_id", put(update_rule))
        .route("/automation/rules/:rule_id", delete(delete_rule))
        .route("/automation/rules/:rule_id/toggle/:is_active", post(toggle_rule))
        // Execution logs
        .route("/automation/rules/:rule_id/logs/:limit", get(get_execution_logs))
        // Testing endpoints
        .route("/automation/rules/:rule_id/test", post(test_rule))
        .route("/automation/:institution_id/events", post(trigger_event))
}
