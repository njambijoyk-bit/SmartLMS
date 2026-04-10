// Automation Rules Service - Visual rule builder, triggers, and webhooks
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// Automation rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutomationRule {
    pub id: uuid::Uuid,
    pub institution_id: uuid::Uuid,
    pub name: String,
    pub description: Option<String>,
    pub trigger: Trigger,
    pub conditions: Vec<RuleCondition>,
    pub actions: Vec<RuleAction>,
    pub is_active: bool,
    pub execution_count: i64,
    pub last_executed_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Trigger types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Trigger {
    Event {
        event_type: EventType,
    },
    Schedule {
        cron_expression: String,
        timezone: String,
    },
}

/// Event types that can trigger automation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EventType {
    UserRegistered,
    CourseEnrolled,
    CourseCompleted,
    QuizSubmitted,
    GradePosted,
    AssignmentSubmitted,
    LiveSessionStarted,
    LiveSessionEnded,
    PaymentReceived,
    CertificateIssued,
}

/// Condition in a rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleCondition {
    pub field: String,
    pub operator: ConditionOperator,
    pub value: serde_json::Value,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConditionOperator {
    Equals,
    NotEquals,
    GreaterThan,
    LessThan,
    Contains,
    StartsWith,
    IsEmpty,
    IsNotEmpty,
}

/// Action to perform when rule triggers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleAction {
    pub action_type: ActionType,
    pub config: serde_json::Value,
}

/// Action types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ActionType {
    SendEmail,
    SendNotification,
    UpdateRecord,
    AssignToCourse,
    IssueCertificate,
    AddBadge,
    Webhook,
    Delay,
}

/// Webhook configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookConfig {
    pub url: String,
    pub method: String,
    pub headers: std::collections::HashMap<String, String>,
    pub body_template: String,
    pub retry_count: i32,
    pub timeout_seconds: i32,
}

/// Rule execution log
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleExecutionLog {
    pub id: uuid::Uuid,
    pub rule_id: uuid::Uuid,
    pub trigger_event: EventType,
    pub conditions_met: bool,
    pub actions_executed: Vec<String>,
    pub error: Option<String>,
    pub executed_at: DateTime<Utc>,
}

// Service functions
pub mod service {
    use super::*;
    
    /// Create a new automation rule
    pub async fn create_rule(
        pool: &PgPool,
        institution_id: uuid::Uuid,
        name: &str,
        trigger: Trigger,
        conditions: Vec<RuleCondition>,
        actions: Vec<RuleAction>,
    ) -> Result<AutomationRule, String> {
        let id = Uuid::new_v4();
        let now = Utc::now();
        
        sqlx::query!(
            "INSERT INTO automation_rules (id, institution_id, name, trigger, conditions, 
             actions, is_active, execution_count, created_at, updated_at)
             VALUES ($1, $2, $3, $4, $5, $6, false, 0, $7, $8)",
            id, institution_id, name, serde_json::to_string(&trigger).unwrap(),
            serde_json::to_string(&conditions).unwrap(), serde_json::to_string(&actions).unwrap(),
            now, now
        )
        .execute(pool)
        .await
        .map_err(|e| e.to_string())?;
        
        Ok(AutomationRule {
            id,
            institution_id,
            name: name.to_string(),
            description: None,
            trigger,
            conditions,
            actions,
            is_active: false,
            execution_count: 0,
            last_executed_at: None,
            created_at: now,
            updated_at: now,
        })
    }
    
    /// Enable/disable a rule
    pub async fn set_rule_active(
        pool: &PgPool,
        rule_id: uuid::Uuid,
        is_active: bool,
    ) -> Result<(), String> {
        sqlx::query!(
            "UPDATE automation_rules SET is_active = $1, updated_at = $2 WHERE id = $3",
            is_active, Utc::now(), rule_id
        )
        .execute(pool)
        .await
        .map_err(|e| e.to_string())?;
        
        Ok(())
    }
    
    /// Process an event and execute matching rules
    pub async fn process_event(
        pool: &PgPool,
        institution_id: uuid::Uuid,
        event_type: EventType,
        event_data: serde_json::Value,
    ) -> Result<Vec<RuleExecutionLog>, String> {
        // Find all active rules with matching trigger
        let rules = get_rules_for_event(pool, institution_id, event_type).await?;
        
        let mut logs = Vec::new();
        
        for rule in rules {
            // Check conditions
            if evaluate_conditions(&rule.conditions, &event_data) {
                // Execute actions
                let actions_result = execute_actions(pool, &rule, &event_data).await;
                
                let log = RuleExecutionLog {
                    id: Uuid::new_v4(),
                    rule_id: rule.id,
                    trigger_event: event_type,
                    conditions_met: true,
                    actions_executed: vec![],  // Track which actions ran
                    error: actions_result.err(),
                    executed_at: Utc::now(),
                };
                
                // Update rule execution count
                sqlx::query!(
                    "UPDATE automation_rules SET execution_count = execution_count + 1, 
                     last_executed_at = $1 WHERE id = $2",
                    Utc::now(), rule.id
                )
                .execute(pool)
                .await
                .ok();
                
                logs.push(log);
            }
        }
        
        Ok(logs)
    }
    
    /// Execute actions for a rule
    async fn execute_actions(
        pool: &PgPool,
        rule: &AutomationRule,
        event_data: &serde_json::Value,
    ) -> Result<(), String> {
        for action in &rule.actions {
            match action.action_type {
                ActionType::SendEmail => {
                    let config: EmailConfig = serde_json::from_value(action.config.clone())
                        .map_err(|e| e.to_string())?;
                    send_email(&config, event_data).await?;
                }
                ActionType::SendNotification => {
                    // Create notification for user
                }
                ActionType::Webhook => {
                    let config: WebhookConfig = serde_json::from_value(action.config.clone())
                        .map_err(|e| e.to_string())?;
                    call_webhook(&config, event_data).await?;
                }
                ActionType::IssueCertificate => {
                    // Issue certificate to user
                }
                ActionType::AddBadge => {
                    // Add badge to user
                }
                _ => {}
            }
        }
        
        Ok(())
    }
    
    /// Get rules for specific event type
    async fn get_rules_for_event(
        pool: &PgPool,
        institution_id: uuid::Uuid,
        event_type: EventType,
    ) -> Result<Vec<AutomationRule>, String> {
        let rows = sqlx::query!(
            "SELECT id, institution_id, name, description, trigger, conditions, actions,
             is_active, execution_count, last_executed_at, created_at, updated_at
             FROM automation_rules WHERE institution_id = $1 AND is_active = true",
            institution_id
        )
        .fetch_all(pool)
        .await
        .map_err(|e| e.to_string())?;
        
        Ok(rows.into_iter().filter_map(|r| {
            let trigger: Trigger = serde_json::from_str(&r.trigger).ok()?;
            match trigger {
                Trigger::Event { event_type: et } if et == event_type => Some(AutomationRule {
                    id: r.id,
                    institution_id: r.institution_id,
                    name: r.name,
                    description: r.description,
                    trigger,
                    conditions: serde_json::from_str(&r.conditions).unwrap_or_default(),
                    actions: serde_json::from_str(&r.actions).unwrap_or_default(),
                    is_active: r.is_active,
                    execution_count: r.execution_count,
                    last_executed_at: r.last_executed_at,
                    created_at: r.created_at,
                    updated_at: r.updated_at,
                }),
                _ => None,
            }
        }).collect())
    }
    
    /// Get all rules for institution
    pub async fn list_rules(
        pool: &PgPool,
        institution_id: uuid::Uuid,
    ) -> Result<Vec<AutomationRule>, String> {
        let rows = sqlx::query!(
            "SELECT id, institution_id, name, description, trigger, conditions, actions,
             is_active, execution_count, last_executed_at, created_at, updated_at
             FROM automation_rules WHERE institution_id = $1 ORDER BY created_at DESC",
            institution_id
        )
        .fetch_all(pool)
        .await
        .map_err(|e| e.to_string())?;
        
        Ok(rows.into_iter().map(|r| AutomationRule {
            id: r.id,
            institution_id: r.institution_id,
            name: r.name,
            description: r.description,
            trigger: serde_json::from_str(&r.trigger).unwrap_or(Trigger::Event { event_type: EventType::UserRegistered }),
            conditions: serde_json::from_str(&r.conditions).unwrap_or_default(),
            actions: serde_json::from_str(&r.actions).unwrap_or_default(),
            is_active: r.is_active,
            execution_count: r.execution_count,
            last_executed_at: r.last_executed_at,
            created_at: r.created_at,
            updated_at: r.updated_at,
        }).collect())
    }
}

fn evaluate_conditions(conditions: &[RuleCondition], data: &serde_json::Value) -> bool {
    for cond in conditions {
        let field_value = data.get(&cond.field);
        
        match cond.operator {
            ConditionOperator::Equals => {
                if field_value != Some(&cond.value) { return false; }
            }
            ConditionOperator::NotEquals => {
                if field_value == Some(&cond.value) { return false; }
            }
            ConditionOperator::IsEmpty => {
                if field_value.is_some() { return false; }
            }
            ConditionOperator::IsNotEmpty => {
                if field_value.is_none() { return false; }
            }
            _ => {}
        }
    }
    true
}

async fn send_email(config: &EmailConfig, _data: &serde_json::Value) -> Result<(), String> {
    // In production: call email service
    tracing::info!("Sending email to: {}", config.to);
    Ok(())
}

async fn call_webhook(config: &WebhookConfig, data: &serde_json::Value) -> Result<(), String> {
    // In production: make HTTP request to webhook URL
    tracing::info!("Calling webhook: {}", config.url);
    Ok(())
}

#[derive(Debug, Clone, Deserialize)]
pub struct EmailConfig {
    pub to: String,
    pub subject: String,
    pub body_template: String,
}