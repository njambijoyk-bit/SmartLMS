// Database operations for ABAC policies
use crate::models::abac::*;
use sqlx::{PgPool, Row};
use uuid::Uuid;

pub async fn create_policy(pool: &PgPool, policy: &AbacPolicy) -> Result<AbacPolicy, sqlx::Error> {
    let conditions_json = policy.conditions.as_ref()
        .map(|c| serde_json::to_string(c).unwrap_or_default());
    
    sqlx::query!(
        "INSERT INTO abac_policies (id, institution_id, name, description, effect, 
         subjects, actions, resources, conditions, priority, enabled, created_at, updated_at)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)",
        policy.id, policy.institution_id, policy.name, policy.description,
        format!("{:?}", policy.effect).to_lowercase(),
        serde_json::to_string(&policy.subjects).unwrap_or_default(),
        serde_json::to_string(&policy.actions).unwrap_or_default(),
        serde_json::to_string(&policy.resources).unwrap_or_default(),
        conditions_json,
        policy.priority, policy.enabled, policy.created_at, policy.updated_at
    )
    .execute(pool)
    .await?;

    Ok(policy.clone())
}

pub async fn get_policy(pool: &PgPool, id: Uuid) -> Result<Option<AbacPolicy>, sqlx::Error> {
    let row = sqlx::query!(
        "SELECT id, institution_id, name, description, effect, subjects, actions, 
         resources, conditions, priority, enabled, created_at, updated_at
         FROM abac_policies WHERE id = $1",
        id
    )
    .fetch_optional(pool)
    .await?;

    Ok(row.map(|r| parse_policy_row(r)))
}

pub async fn list_policies(
    pool: &PgPool,
    institution_id: Uuid,
    enabled: Option<bool>,
) -> Result<Vec<AbacPolicy>, sqlx::Error> {
    let query = if let Some(enabled) = enabled {
        sqlx::query!(
            "SELECT id, institution_id, name, description, effect, subjects, actions, 
             resources, conditions, priority, enabled, created_at, updated_at
             FROM abac_policies WHERE institution_id = $1 AND enabled = $2
             ORDER BY priority DESC",
            institution_id, enabled
        )
    } else {
        sqlx::query!(
            "SELECT id, institution_id, name, description, effect, subjects, actions, 
             resources, conditions, priority, enabled, created_at, updated_at
             FROM abac_policies WHERE institution_id = $1
             ORDER BY priority DESC",
            institution_id
        )
    };

    let rows = query.fetch_all(pool).await?;

    Ok(rows.into_iter().map(parse_policy_row).collect())
}

pub async fn update_policy(pool: &PgPool, policy: &AbacPolicy) -> Result<AbacPolicy, sqlx::Error> {
    let conditions_json = policy.conditions.as_ref()
        .map(|c| serde_json::to_string(c).unwrap_or_default());
    
    sqlx::query!(
        "UPDATE abac_policies SET 
            name = $1, description = $2, effect = $3, subjects = $4, actions = $5,
            resources = $6, conditions = $7, priority = $8, enabled = $9, updated_at = $10
         WHERE id = $11",
        policy.name, policy.description, format!("{:?}", policy.effect).to_lowercase(),
        serde_json::to_string(&policy.subjects).unwrap_or_default(),
        serde_json::to_string(&policy.actions).unwrap_or_default(),
        serde_json::to_string(&policy.resources).unwrap_or_default(),
        conditions_json, policy.priority, policy.enabled, policy.updated_at, policy.id
    )
    .execute(pool)
    .await?;

    Ok(policy.clone())
}

pub async fn delete_policy(pool: &PgPool, id: Uuid) -> Result<(), sqlx::Error> {
    sqlx::query!("DELETE FROM abac_policies WHERE id = $1", id)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn get_enabled_policies(pool: &PgPool, institution_id: Uuid) -> Result<Vec<AbacPolicy>, sqlx::Error> {
    list_policies(pool, institution_id, Some(true)).await
}

fn parse_policy_row(r: sqlx::postgres::PgRow) -> AbacPolicy {
    let subjects: Vec<SubjectSelector> = serde_json::from_str(&r.subjects).unwrap_or_default();
    let actions: Vec<String> = serde_json::from_str(&r.actions).unwrap_or_default();
    let resources: Vec<ResourceSelector> = serde_json::from_str(&r.resources).unwrap_or_default();
    let conditions: Option<ConditionExpression> = r.conditions
        .as_ref()
        .and_then(|c| serde_json::from_str(c).ok());

    AbacPolicy {
        id: r.id,
        institution_id: r.institution_id,
        name: r.name,
        description: r.description,
        effect: if r.effect == "allow" { PolicyEffect::Allow } else { PolicyEffect::Deny },
        subjects,
        actions,
        resources,
        conditions,
        priority: r.priority,
        enabled: r.enabled,
        created_at: r.created_at,
        updated_at: r.updated_at,
    }
}