// ABAC (Attribute-Based Access Control) Service
// Extends RBAC with fine-grained policies based on user, resource, and environment attributes

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Attribute source - where the attribute comes from
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AttributeSource {
    User,        // From user profile (role, department, enrollment status, etc.)
    Resource,    // From entity metadata (course owner, content visibility, etc.)
    Environment, // From request context (time, IP, device, location, etc.)
}

/// Attribute definition with name and source
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attribute {
    pub name: String,
    pub source: AttributeSource,
}

/// Supported attribute value types
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AttributeValue {
    String(String),
    Number(f64),
    Boolean(bool),
    List(Vec<String>),
    Datetime(DateTime<Utc>),
    Null,
}

impl AttributeValue {
    pub fn as_string(&self) -> Option<&str> {
        match self {
            AttributeValue::String(s) => Some(s),
            _ => None,
        }
    }

    pub fn as_number(&self) -> Option<f64> {
        match self {
            AttributeValue::Number(n) => Some(*n),
            _ => None,
        }
    }

    pub fn as_bool(&self) -> Option<bool> {
        match self {
            AttributeValue::Boolean(b) => Some(*b),
            _ => None,
        }
    }

    pub fn as_list(&self) -> Option<&Vec<String>> {
        match self {
            AttributeValue::List(l) => Some(l),
            _ => None,
        }
    }
}

/// Comparison operators for attribute conditions
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Operator {
    // Equality
    Eq,  // equal
    Neq, // not equal

    // Comparison
    Gt,  // greater than
    Gte, // greater than or equal
    Lt,  // less than
    Lte, // less than or equal

    // Set membership
    In,    // in set
    NotIn, // not in set

    // String operations
    Contains,   // contains substring
    StartsWith, // starts with prefix
    EndsWith,   // ends with suffix

    // Range
    Between, // between (for dates/numbers)
}

/// Logical operators for combining conditions
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LogicalOperator {
    And,
    Or,
    Not,
}

/// Single condition on an attribute
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Condition {
    pub attribute: String,
    pub operator: Operator,
    pub value: AttributeValue,
    // For 'between' operator, this would be a two-element array
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_end: Option<AttributeValue>,
}

/// Complex condition with logical operators
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ConditionExpression {
    Simple(Condition),
    Compound {
        logical_op: LogicalOperator,
        conditions: Vec<ConditionExpression>,
    },
}

impl ConditionExpression {
    pub fn and(conditions: Vec<ConditionExpression>) -> Self {
        ConditionExpression::Compound {
            logical_op: LogicalOperator::And,
            conditions,
        }
    }

    pub fn or(conditions: Vec<ConditionExpression>) -> Self {
        ConditionExpression::Compound {
            logical_op: LogicalOperator::Or,
            conditions,
        }
    }

    pub fn not(condition: ConditionExpression) -> Self {
        ConditionExpression::Compound {
            logical_op: LogicalOperator::Not,
            conditions: vec![condition],
        }
    }
}

/// Policy effect - allow or deny
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PolicyEffect {
    Allow,
    Deny,
}

/// ABAC Policy definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AbacPolicy {
    pub id: uuid::Uuid,
    pub institution_id: uuid::Uuid,
    pub name: String,
    pub description: Option<String>,
    pub effect: PolicyEffect,

    // Subject selectors (who)
    pub subjects: Vec<SubjectSelector>,

    // Action selectors (what operation)
    pub actions: Vec<String>, // e.g., "courses:read", "users:write"

    // Resource selectors (which resources)
    pub resources: Vec<ResourceSelector>,

    // Conditions under which policy applies
    pub conditions: Option<ConditionExpression>,

    // Priority for evaluation order (higher = evaluated first)
    pub priority: i32,

    pub enabled: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Subject selector - defines which users the policy applies to
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubjectSelector {
    // Match by role
    pub roles: Option<Vec<String>>,
    // Match by user attribute
    pub attributes: Option<Vec<Condition>>,
    // Match specific user IDs
    pub user_ids: Option<Vec<uuid::Uuid>>,
}

/// Resource selector - defines which resources the policy applies to
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceSelector {
    // Match by resource type
    pub resource_type: Option<String>, // "course", "assessment", "lesson", etc.
    // Match by resource attribute
    pub attributes: Option<Vec<Condition>>,
    // Match specific resource IDs
    pub resource_ids: Option<Vec<uuid::Uuid>>,
}

/// Access decision result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AccessDecision {
    Allow,
    Deny(String),          // Reason for denial
    Indeterminate(String), // Can't determine - multiple policies conflict
}

/// Request context for authorization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessRequest {
    pub user_id: uuid::Uuid,
    pub user_roles: Vec<String>,
    pub user_attributes: std::collections::HashMap<String, AttributeValue>,

    pub action: String, // e.g., "courses:write"

    pub resource_type: String,
    pub resource_id: Option<uuid::Uuid>,
    pub resource_attributes: std::collections::HashMap<String, AttributeValue>,

    pub environment: EnvironmentAttributes,
}

/// Environment attributes from the request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentAttributes {
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub current_time: DateTime<Utc>,
    pub location: Option<String>,
    pub device_type: Option<String>,
}

/// Policy evaluation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvaluationResult {
    pub decision: AccessDecision,
    pub matched_policies: Vec<uuid::Uuid>,
    pub evaluated_conditions: Vec<ConditionEvaluation>,
}

/// Result of evaluating a single condition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConditionEvaluation {
    pub condition: Condition,
    pub result: bool,
    pub error: Option<String>,
}

// ============================================================================
// POLICY EVALUATION ENGINE
// ============================================================================

impl AbacPolicy {
    /// Check if this policy applies to the given request
    pub fn matches_request(&self, request: &AccessRequest) -> bool {
        // Check subject matches
        if !self.matches_subject(request) {
            return false;
        }

        // Check action matches
        if !self
            .actions
            .iter()
            .any(|a| request.action.starts_with(a) || a == "*")
        {
            return false;
        }

        // Check resource matches
        if !self.matches_resource(request) {
            return false;
        }

        true
    }

    fn matches_subject(&self, request: &AccessRequest) -> bool {
        for selector in &self.subjects {
            // Check role match
            if let Some(roles) = &selector.roles {
                if request.user_roles.iter().any(|r| roles.contains(r)) {
                    return true;
                }
            }

            // Check user ID match
            if let Some(user_ids) = &selector.user_ids {
                if user_ids.contains(&request.user_id) {
                    return true;
                }
            }

            // Check attribute match
            if let Some(attrs) = &selector.attributes {
                let mut attr_match = true;
                for cond in attrs {
                    if !evaluate_attribute_condition(cond, &request.user_attributes) {
                        attr_match = false;
                        break;
                    }
                }
                if attr_match {
                    return true;
                }
            }
        }

        // Empty subjects means match all
        self.subjects.is_empty()
    }

    fn matches_resource(&self, request: &AccessRequest) -> bool {
        for selector in &self.resources {
            // Check resource type match
            if let Some(ref res_type) = selector.resource_type {
                if res_type != &request.resource_type && res_type != "*" {
                    continue;
                }
            }

            // Check resource ID match
            if let Some(ids) = &selector.resource_ids {
                if let Some(rid) = request.resource_id {
                    if ids.contains(&rid) {
                        return true;
                    }
                }
            } else {
                // No specific IDs, check attributes
                if let Some(attrs) = &selector.attributes {
                    let mut attr_match = true;
                    for cond in attrs {
                        if !evaluate_attribute_condition(cond, &request.resource_attributes) {
                            attr_match = false;
                            break;
                        }
                    }
                    if attr_match {
                        return true;
                    }
                } else {
                    // No specific resource constraints - matches all
                    return true;
                }
            }
        }

        // Empty resources means match all
        self.resources.is_empty()
    }
}

/// Evaluate a single attribute condition
pub fn evaluate_attribute_condition(
    cond: &Condition,
    attrs: &std::collections::HashMap<String, AttributeValue>,
) -> bool {
    let attr_value = attrs.get(&cond.attribute);

    match (attr_value, &cond.operator) {
        // Equality checks
        (Some(actual), Operator::Eq) => values_equal(actual, &cond.value),
        (Some(actual), Operator::Neq) => !values_equal(actual, &cond.value),

        // Numeric comparisons
        (Some(AttributeValue::Number(n)), Operator::Gt) => {
            cond.value.as_number().map(|v| n > v).unwrap_or(false)
        }
        (Some(AttributeValue::Number(n)), Operator::Gte) => {
            cond.value.as_number().map(|v| n >= v).unwrap_or(false)
        }
        (Some(AttributeValue::Number(n)), Operator::Lt) => {
            cond.value.as_number().map(|v| n < v).unwrap_or(false)
        }
        (Some(AttributeValue::Number(n)), Operator::Lte) => {
            cond.value.as_number().map(|v| n <= v).unwrap_or(false)
        }

        // Set membership
        (Some(actual), Operator::In) => {
            if let Some(expected_list) = cond.value.as_list() {
                if let Some(actual_str) = actual.as_string() {
                    return expected_list.contains(&actual_str.to_string());
                }
            }
            false
        }
        (Some(actual), Operator::NotIn) => {
            if let Some(expected_list) = cond.value.as_list() {
                if let Some(actual_str) = actual.as_string() {
                    return !expected_list.contains(&actual_str.to_string());
                }
            }
            true
        }

        // String operations
        (Some(AttributeValue::String(s)), Operator::Contains) => cond
            .value
            .as_string()
            .map(|v| s.contains(v))
            .unwrap_or(false),
        (Some(AttributeValue::String(s)), Operator::StartsWith) => cond
            .value
            .as_string()
            .map(|v| s.starts_with(v))
            .unwrap_or(false),
        (Some(AttributeValue::String(s)), Operator::EndsWith) => cond
            .value
            .as_string()
            .map(|v| s.ends_with(v))
            .unwrap_or(false),

        // Boolean
        (Some(actual), Operator::Eq) => cond
            .value
            .as_bool()
            .map(|v| actual.as_bool() == Some(v))
            .unwrap_or(false),

        _ => false,
    }
}

fn values_equal(a: &AttributeValue, b: &AttributeValue) -> bool {
    match (a, b) {
        (AttributeValue::String(s1), AttributeValue::String(s2)) => s1 == s2,
        (AttributeValue::Number(n1), AttributeValue::Number(n2)) => (n1 - n2).abs() < 0.0001,
        (AttributeValue::Boolean(b1), AttributeValue::Boolean(b2)) => b1 == b2,
        (AttributeValue::List(l1), AttributeValue::List(l2)) => l1 == l2,
        _ => false,
    }
}

/// Evaluate a condition expression (simple or compound)
pub fn evaluate_condition_expression(
    expr: &ConditionExpression,
    attrs: &std::collections::HashMap<String, AttributeValue>,
) -> bool {
    match expr {
        ConditionExpression::Simple(cond) => evaluate_attribute_condition(cond, attrs),

        ConditionExpression::Compound {
            logical_op,
            conditions,
        } => match logical_op {
            LogicalOperator::And => conditions
                .iter()
                .all(|c| evaluate_condition_expression(c, attrs)),
            LogicalOperator::Or => conditions
                .iter()
                .any(|c| evaluate_condition_expression(c, attrs)),
            LogicalOperator::Not => {
                if let Some(first) = conditions.first() {
                    !evaluate_condition_expression(first, attrs)
                } else {
                    true
                }
            }
        },
    }
}

/// Evaluate a complete policy against a request
pub fn evaluate_policy(policy: &AbacPolicy, request: &AccessRequest) -> bool {
    // First check if policy matches basic subject/action/resource
    if !policy.matches_request(request) {
        return false;
    }

    // Then evaluate conditions if present
    if let Some(ref conditions) = policy.conditions {
        // Build combined attributes map
        let mut all_attrs = request.user_attributes.clone();
        for (k, v) in &request.resource_attributes {
            all_attrs.insert(k.clone(), v.clone());
        }
        // Add environment attributes
        all_attrs.insert(
            "current_time".to_string(),
            AttributeValue::Datetime(request.environment.current_time),
        );
        if let Some(ip) = &request.environment.ip_address {
            all_attrs.insert("ip_address".to_string(), AttributeValue::String(ip.clone()));
        }
        if let Some(loc) = &request.environment.location {
            all_attrs.insert("location".to_string(), AttributeValue::String(loc.clone()));
        }

        evaluate_condition_expression(conditions, &all_attrs)
    } else {
        // No conditions = always matches
        true
    }
}

/// Evaluate all policies and make access decision
pub fn evaluate_access(request: &AccessRequest, policies: &[AbacPolicy]) -> EvaluationResult {
    let mut allow_policies = Vec::new();
    let mut deny_policies = Vec::new();
    let mut evaluated = Vec::new();

    // Sort by priority (higher first)
    let mut sorted_policies: Vec<&AbacPolicy> = policies.iter().filter(|p| p.enabled).collect();
    sorted_policies.sort_by(|a, b| b.priority.cmp(&a.priority));

    for policy in sorted_policies {
        if evaluate_policy(policy, request) {
            match policy.effect {
                PolicyEffect::Allow => allow_policies.push(policy.id),
                PolicyEffect::Deny => deny_policies.push(policy.id),
            }
        }

        evaluated.push(ConditionEvaluation {
            condition: Condition {
                attribute: "policy_match".to_string(),
                operator: Operator::Eq,
                value: AttributeValue::Boolean(true),
                value_end: None,
            },
            result: true,
            error: None,
        });
    }

    // Decision logic: deny overrides allow
    let decision = if !deny_policies.is_empty() {
        AccessDecision::Deny(format!("Denied by policies: {:?}", deny_policies))
    } else if !allow_policies.is_empty() {
        AccessDecision::Allow
    } else {
        AccessDecision::Indeterminate("No matching policies".to_string())
    };

    EvaluationResult {
        decision,
        matched_policies: allow_policies
            .iter()
            .chain(deny_policies.iter())
            .copied()
            .collect(),
        evaluated_conditions: evaluated,
    }
}

// ============================================================================
// PRE-BUILT POLICY TEMPLATES
// ============================================================================

/// Create a policy for time-based access control
pub fn time_based_policy(
    institution_id: uuid::Uuid,
    name: &str,
    start_hour: i32,
    end_hour: i32,
    days: Vec<i32>, // 0=Sunday, 1=Monday, etc.
    allowed_roles: Vec<String>,
    action: &str,
) -> AbacPolicy {
    AbacPolicy {
        id: Uuid::new_v4(),
        institution_id,
        name: name.to_string(),
        description: Some(format!(
            "Allow {} only during {}:00-{}:00 on days {:?}",
            action, start_hour, end_hour, days
        )),
        effect: PolicyEffect::Allow,
        subjects: vec![SubjectSelector {
            roles: Some(allowed_roles),
            attributes: None,
            user_ids: None,
        }],
        actions: vec![action.to_string()],
        resources: vec![],
        conditions: Some(ConditionExpression::Compound {
            logical_op: LogicalOperator::And,
            conditions: vec![
                // Check day of week
                ConditionExpression::Simple(Condition {
                    attribute: "current_time".to_string(),
                    operator: Operator::In,
                    value: AttributeValue::List(days.iter().map(|d| d.to_string()).collect()),
                    value_end: None,
                }),
            ],
        }),
        priority: 100,
        enabled: true,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    }
}

/// Create a policy for department-based access
pub fn department_access_policy(
    institution_id: uuid::Uuid,
    name: &str,
    department: &str,
    allowed_roles: Vec<String>,
    action: &str,
) -> AbacPolicy {
    AbacPolicy {
        id: Uuid::new_v4(),
        institution_id,
        name: name.to_string(),
        description: Some(format!(
            "Allow {} only for department {}",
            action, department
        )),
        effect: PolicyEffect::Allow,
        subjects: vec![SubjectSelector {
            roles: Some(allowed_roles),
            attributes: Some(vec![Condition {
                attribute: "department".to_string(),
                operator: Operator::Eq,
                value: AttributeValue::String(department.to_string()),
                value_end: None,
            }]),
            user_ids: None,
        }],
        actions: vec![action.to_string()],
        resources: vec![],
        conditions: None,
        priority: 50,
        enabled: true,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    }
}

/// Create a policy for enrollment status-based access
pub fn enrollment_status_policy(
    institution_id: uuid::Uuid,
    name: &str,
    required_status: &str, // "active", "pending", "graduated", etc.
    action: &str,
) -> AbacPolicy {
    AbacPolicy {
        id: Uuid::new_v4(),
        institution_id,
        name: name.to_string(),
        description: Some(format!(
            "Allow {} only for users with {} enrollment status",
            action, required_status
        )),
        effect: PolicyEffect::Allow,
        subjects: vec![SubjectSelector {
            roles: Some(vec!["learner".to_string()]),
            attributes: Some(vec![Condition {
                attribute: "enrollment_status".to_string(),
                operator: Operator::Eq,
                value: AttributeValue::String(required_status.to_string()),
                value_end: None,
            }]),
            user_ids: None,
        }],
        actions: vec![action.to_string()],
        resources: vec![],
        conditions: None,
        priority: 75,
        enabled: true,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    }
}

/// Create a policy for IP-based restrictions
pub fn ip_restriction_policy(
    institution_id: uuid::Uuid,
    name: &str,
    allowed_ips: Vec<String>, // IP ranges or CIDR
    action: &str,
) -> AbacPolicy {
    AbacPolicy {
        id: Uuid::new_v4(),
        institution_id,
        name: name.to_string(),
        description: Some(format!(
            "Allow {} only from IP ranges {:?}",
            action, allowed_ips
        )),
        effect: PolicyEffect::Allow,
        subjects: vec![],
        actions: vec![action.to_string()],
        resources: vec![],
        conditions: Some(ConditionExpression::Compound {
            logical_op: LogicalOperator::Or,
            conditions: allowed_ips
                .iter()
                .map(|ip| {
                    ConditionExpression::Simple(Condition {
                        attribute: "ip_address".to_string(),
                        operator: Operator::StartsWith,
                        value: AttributeValue::String(ip.clone()),
                        value_end: None,
                    })
                })
                .collect(),
        }),
        priority: 90,
        enabled: true,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    }
}

/// Create a policy for course ownership
pub fn course_ownership_policy(
    institution_id: uuid::Uuid,
    name: &str,
    action: &str, // e.g., "courses:write"
) -> AbacPolicy {
    AbacPolicy {
        id: Uuid::new_v4(),
        institution_id,
        name: name.to_string(),
        description: Some("Instructors can only edit their own courses".to_string()),
        effect: PolicyEffect::Allow,
        subjects: vec![SubjectSelector {
            roles: Some(vec!["instructor".to_string()]),
            attributes: None,
            user_ids: None,
        }],
        actions: vec![action.to_string()],
        resources: vec![ResourceSelector {
            resource_type: Some("course".to_string()),
            attributes: Some(vec![Condition {
                attribute: "instructor_id".to_string(),
                operator: Operator::Eq,
                value: AttributeValue::String("${user_id}".to_string()), // Placeholder
                value_end: None,
            }]),
            resource_ids: None,
        }],
        conditions: None,
        priority: 80,
        enabled: true,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    }
}
