// ABAC models - re-export from services for API use
pub use crate::services::abac::{
    AbacPolicy, AccessDecision, AccessRequest, Attribute, AttributeSource, 
    AttributeValue, Condition, ConditionExpression, EnvironmentAttributes, 
    EvaluationResult, LogicalOperator, Operator, PolicyEffect, PolicyTemplate,
    ResourceSelector, SubjectSelector,
};