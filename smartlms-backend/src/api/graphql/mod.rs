// GraphQL API for Developer Platform
// Uses async-graphql for schema definition and resolvers

use async_graphql::*;
use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::utils::app_state::AppState;

// ============================================================================
// GRAPHQL SCHEMA TYPES
// ============================================================================

/// API Key object
#[derive(SimpleObject)]
pub struct ApiKey {
    pub id: Uuid,
    pub name: String,
    pub key_prefix: String,
    pub permissions: Vec<String>,
    pub rate_limit: i32,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub last_used_at: Option<DateTime<Utc>>,
}

/// Webhook endpoint
#[derive(SimpleObject)]
pub struct Webhook {
    pub id: Uuid,
    pub name: String,
    pub url: String,
    pub events: Vec<String>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
}

/// Integration with external service
#[derive(SimpleObject)]
pub struct Integration {
    pub id: Uuid,
    pub name: String,
    pub integration_type: String,
    pub is_active: bool,
    pub last_sync_at: Option<DateTime<Utc>>,
    pub sync_status: String,
}

/// SDK Configuration
#[derive(SimpleObject)]
pub struct SdkConfig {
    pub api_key: String,
    pub base_url: String,
    pub version: String,
    pub features: Vec<String>,
}

/// Usage statistics
#[derive(SimpleObject)]
pub struct UsageStats {
    pub total_requests: i64,
    pub successful_requests: i64,
    pub failed_requests: i64,
    pub avg_response_time_ms: f64,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
}

/// User type for GraphQL
#[derive(SimpleObject)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub name: String,
    pub role: String,
    pub institution_id: Uuid,
}

/// Course type for GraphQL
#[derive(SimpleObject)]
pub struct Course {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub institution_id: Uuid,
    pub is_published: bool,
    pub created_at: DateTime<Utc>,
}

/// Enrollment type
#[derive(SimpleObject)]
pub struct Enrollment {
    pub id: Uuid,
    pub user_id: Uuid,
    pub course_id: Uuid,
    pub status: String,
    pub enrolled_at: DateTime<Utc>,
}

/// Grade type
#[derive(SimpleObject)]
pub struct Grade {
    pub id: Uuid,
    pub user_id: Uuid,
    pub course_id: Uuid,
    pub score: f64,
    pub letter_grade: Option<String>,
    pub graded_at: DateTime<Utc>,
}

// ============================================================================
// INPUT TYPES
// ============================================================================

#[derive(InputObject)]
pub struct CreateApiKeyInput {
    pub name: String,
    pub permissions: Vec<String>,
    pub rate_limit: Option<i32>,
    pub expires_in_days: Option<i32>,
}

#[derive(InputObject)]
pub struct CreateWebhookInput {
    pub name: String,
    pub url: String,
    pub events: Vec<String>,
}

#[derive(InputObject)]
pub struct CreateIntegrationInput {
    pub name: String,
    pub integration_type: String,
    pub config: serde_json::Value,
}

// ============================================================================
// QUERY ROOT
// ============================================================================

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    /// Get all API keys for current user
    async fn api_keys(&self, ctx: &Context<'_>) -> Result<Vec<ApiKey>> {
        let pool = ctx.data::<sqlx::PgPool>().map_err(|e| e.to_string())?;
        let user_id = ctx
            .data::<Uuid>()
            .map_err(|_| "User ID not found".to_string())?;

        let rows = sqlx::query!(
            "SELECT id, name, key_prefix, permissions, rate_limit, is_active, 
             last_used_at, created_at
             FROM api_keys WHERE user_id = $1",
            user_id
        )
        .fetch_all(pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(rows
            .into_iter()
            .map(|r| ApiKey {
                id: r.id,
                name: r.name,
                key_prefix: r.key_prefix,
                permissions: serde_json::from_value(r.permissions).unwrap_or_default(),
                rate_limit: r.rate_limit,
                is_active: r.is_active,
                created_at: r.created_at,
                last_used_at: r.last_used_at,
            })
            .collect())
    }

    /// Get webhook by ID
    async fn webhook(&self, ctx: &Context<'_>, id: Uuid) -> Result<Option<Webhook>> {
        let pool = ctx.data::<sqlx::PgPool>().map_err(|e| e.to_string())?;

        let row = sqlx::query!(
            "SELECT id, name, url, events, is_active, created_at 
             FROM webhook_endpoints WHERE id = $1",
            id
        )
        .fetch_optional(pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(row.map(|r| Webhook {
            id: r.id,
            name: r.name,
            url: r.url,
            events: serde_json::from_value(r.events).unwrap_or_default(),
            is_active: r.is_active,
            created_at: r.created_at,
        }))
    }

    /// List all webhooks for institution
    async fn webhooks(&self, ctx: &Context<'_>) -> Result<Vec<Webhook>> {
        let pool = ctx.data::<sqlx::PgPool>().map_err(|e| e.to_string())?;
        let user_id = ctx
            .data::<Uuid>()
            .map_err(|_| "User ID not found".to_string())?;

        // Get institution from user
        let inst_row = sqlx::query_scalar!(
            "SELECT institution_id FROM users WHERE id = $1",
            user_id
        )
        .fetch_one(pool)
        .await
        .map_err(|e| e.to_string())?;

        let rows = sqlx::query!(
            "SELECT id, name, url, events, is_active, created_at 
             FROM webhook_endpoints WHERE institution_id = $1",
            inst_row
        )
        .fetch_all(pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(rows
            .into_iter()
            .map(|r| Webhook {
                id: r.id,
                name: r.name,
                url: r.url,
                events: serde_json::from_value(r.events).unwrap_or_default(),
                is_active: r.is_active,
                created_at: r.created_at,
            })
            .collect())
    }

    /// List integrations
    async fn integrations(&self, ctx: &Context<'_>) -> Result<Vec<Integration>> {
        let pool = ctx.data::<sqlx::PgPool>().map_err(|e| e.to_string())?;
        let user_id = ctx
            .data::<Uuid>()
            .map_err(|_| "User ID not found".to_string())?;

        let inst_row = sqlx::query_scalar!(
            "SELECT institution_id FROM users WHERE id = $1",
            user_id
        )
        .fetch_one(pool)
        .await
        .map_err(|e| e.to_string())?;

        let rows = sqlx::query!(
            "SELECT id, name, integration_type, is_active, last_sync_at, sync_status
             FROM integrations WHERE institution_id = $1",
            inst_row
        )
        .fetch_all(pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(rows
            .into_iter()
            .map(|r| Integration {
                id: r.id,
                name: r.name,
                integration_type: r.integration_type,
                is_active: r.is_active,
                last_sync_at: r.last_sync_at,
                sync_status: r.sync_status.unwrap_or_else(|| "idle".to_string()),
            })
            .collect())
    }

    /// Get SDK configuration
    async fn sdk_config(&self, ctx: &Context<'_>) -> Result<SdkConfig> {
        let pool = ctx.data::<sqlx::PgPool>().map_err(|e| e.to_string())?;
        let user_id = ctx
            .data::<Uuid>()
            .map_err(|_| "User ID not found".to_string())?;

        let inst_row = sqlx::query_scalar!(
            "SELECT institution_id FROM users WHERE id = $1",
            user_id
        )
        .fetch_one(pool)
        .await
        .map_err(|e| e.to_string())?;

        // Generate or fetch SDK config
        let api_key = format!("sk_live_{}", hex::encode(rand::random::<[u8; 32]>()));

        Ok(SdkConfig {
            api_key,
            base_url: "https://api.smartlms.example.com".to_string(),
            version: "v1".to_string(),
            features: vec![
                "courses".to_string(),
                "users".to_string(),
                "enrollments".to_string(),
            ],
        })
    }

    /// Get usage statistics
    async fn usage_stats(
        &self,
        ctx: &Context<'_>,
        days: Option<i32>,
    ) -> Result<UsageStats> {
        let pool = ctx.data::<sqlx::PgPool>().map_err(|e| e.to_string())?;
        let days = days.unwrap_or(30);

        let row = sqlx::query!(
            r#"SELECT 
                COUNT(*) as total,
                COUNT(*) FILTER (WHERE status_code < 400) as successful,
                COUNT(*) FILTER (WHERE status_code >= 400) as failed,
                AVG(response_time_ms) as avg_response_time,
                MIN(created_at) as period_start,
                MAX(created_at) as period_end
               FROM api_usage_logs 
               WHERE created_at >= NOW() - INTERVAL '1 day' * $1"#,
            days
        )
        .fetch_one(pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(UsageStats {
            total_requests: row.total.unwrap_or(0),
            successful_requests: row.successful.unwrap_or(0),
            failed_requests: row.failed.unwrap_or(0),
            avg_response_time_ms: row.avg_response_time.unwrap_or(0.0),
            period_start: row.period_start.unwrap_or_else(Utc::now),
            period_end: row.period_end.unwrap_or_else(Utc::now),
        })
    }

    /// Get user by ID
    async fn user(&self, ctx: &Context<'_>, id: Uuid) -> Result<Option<User>> {
        let pool = ctx.data::<sqlx::PgPool>().map_err(|e| e.to_string())?;

        let row = sqlx::query!(
            "SELECT id, email, name, role, institution_id 
             FROM users WHERE id = $1",
            id
        )
        .fetch_optional(pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(row.map(|r| User {
            id: r.id,
            email: r.email,
            name: r.name,
            role: r.role,
            institution_id: r.institution_id,
        }))
    }

    /// List courses with pagination
    async fn courses(
        &self,
        ctx: &Context<'_>,
        first: Option<i32>,
        after: Option<String>,
    ) -> Result<CourseConnection> {
        let pool = ctx.data::<sqlx::PgPool>().map_err(|e| e.to_string())?;
        let limit = first.unwrap_or(10) as i64;

        let rows = sqlx::query!(
            "SELECT id, name, description, institution_id, is_published, created_at 
             FROM courses LIMIT $1",
            limit
        )
        .fetch_all(pool)
        .await
        .map_err(|e| e.to_string())?;

        let edges = rows
            .into_iter()
            .map(|r| CourseEdge {
                cursor: r.id.to_string(),
                node: Course {
                    id: r.id,
                    name: r.name,
                    description: r.description,
                    institution_id: r.institution_id,
                    is_published: r.is_published,
                    created_at: r.created_at,
                },
            })
            .collect();

        Ok(CourseConnection {
            edges,
            page_info: PageInfo {
                has_next_page: rows.len() as i64 == limit,
                has_previous_page: after.is_some(),
                ..Default::default()
            },
        })
    }
}

// ============================================================================
// MUTATION ROOT
// ============================================================================

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    /// Create a new API key
    async fn create_api_key(
        &self,
        ctx: &Context<'_>,
        input: CreateApiKeyInput,
    ) -> Result<ApiKey> {
        let pool = ctx.data::<sqlx::PgPool>().map_err(|e| e.to_string())?;
        let user_id = ctx
            .data::<Uuid>()
            .map_err(|_| "User ID not found".to_string())?;

        let inst_row = sqlx::query_scalar!(
            "SELECT institution_id FROM users WHERE id = $1",
            user_id
        )
        .fetch_one(pool)
        .await
        .map_err(|e| e.to_string())?;

        let id = Uuid::new_v4();
        let key_prefix = format!("sk_{}", hex::encode(&rand::random::<[u8; 6]>()));
        let key_hash = hex::encode(rand::random::<[u8; 32]>());

        sqlx::query!(
            "INSERT INTO api_keys (id, institution_id, user_id, name, key_hash, key_prefix,
             permissions, rate_limit, is_active, created_at, updated_at)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, true, $9, $10)",
            id,
            inst_row,
            user_id,
            input.name,
            key_hash,
            key_prefix,
            serde_json::to_value(&input.permissions).unwrap(),
            input.rate_limit.unwrap_or(1000),
            Utc::now(),
            Utc::now()
        )
        .execute(pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(ApiKey {
            id,
            name: input.name,
            key_prefix,
            permissions: input.permissions,
            rate_limit: input.rate_limit.unwrap_or(1000),
            is_active: true,
            created_at: Utc::now(),
            last_used_at: None,
        })
    }

    /// Create a new webhook endpoint
    async fn create_webhook(
        &self,
        ctx: &Context<'_>,
        input: CreateWebhookInput,
    ) -> Result<CreateWebhookResult> {
        let pool = ctx.data::<sqlx::PgPool>().map_err(|e| e.to_string())?;
        let user_id = ctx
            .data::<Uuid>()
            .map_err(|_| "User ID not found".to_string())?;

        let inst_row = sqlx::query_scalar!(
            "SELECT institution_id FROM users WHERE id = $1",
            user_id
        )
        .fetch_one(pool)
        .await
        .map_err(|e| e.to_string())?;

        let id = Uuid::new_v4();
        let secret = hex::encode(rand::random::<[u8; 32]>());

        sqlx::query!(
            "INSERT INTO webhook_endpoints (id, institution_id, name, url, events, secret,
             is_active, created_at, updated_at)
             VALUES ($1, $2, $3, $4, $5, $6, true, $7, $8)",
            id,
            inst_row,
            input.name,
            input.url,
            serde_json::to_value(&input.events).unwrap(),
            secret,
            Utc::now(),
            Utc::now()
        )
        .execute(pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(CreateWebhookResult {
            webhook: Webhook {
                id,
                name: input.name,
                url: input.url,
                events: input.events,
                is_active: true,
                created_at: Utc::now(),
            },
            secret,
        })
    }

    /// Create a new integration
    async fn create_integration(
        &self,
        ctx: &Context<'_>,
        input: CreateIntegrationInput,
    ) -> Result<Integration> {
        let pool = ctx.data::<sqlx::PgPool>().map_err(|e| e.to_string())?;
        let user_id = ctx
            .data::<Uuid>()
            .map_err(|_| "User ID not found".to_string())?;

        let inst_row = sqlx::query_scalar!(
            "SELECT institution_id FROM users WHERE id = $1",
            user_id
        )
        .fetch_one(pool)
        .await
        .map_err(|e| e.to_string())?;

        let id = Uuid::new_v4();

        sqlx::query!(
            "INSERT INTO integrations (id, institution_id, name, integration_type, config,
             is_active, created_at, updated_at)
             VALUES ($1, $2, $3, $4, $5, true, $6, $7)",
            id,
            inst_row,
            input.name,
            input.integration_type,
            input.config,
            Utc::now(),
            Utc::now()
        )
        .execute(pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(Integration {
            id,
            name: input.name,
            integration_type: input.integration_type,
            is_active: true,
            last_sync_at: None,
            sync_status: "idle".to_string(),
        })
    }

    /// Revoke an API key
    async fn revoke_api_key(&self, ctx: &Context<'_>, id: Uuid) -> Result<bool> {
        let pool = ctx.data::<sqlx::PgPool>().map_err(|e| e.to_string())?;

        sqlx::query!(
            "UPDATE api_keys SET is_active = false, updated_at = $1 WHERE id = $2",
            Utc::now(),
            id
        )
        .execute(pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(true)
    }

    /// Delete a webhook
    async fn delete_webhook(&self, ctx: &Context<'_>, id: Uuid) -> Result<bool> {
        let pool = ctx.data::<sqlx::PgPool>().map_err(|e| e.to_string())?;

        sqlx::query!("DELETE FROM webhook_endpoints WHERE id = $1", id)
            .execute(pool)
            .await
            .map_err(|e| e.to_string())?;

        Ok(true)
    }
}

// ============================================================================
// CONNECTION TYPES FOR PAGINATION
// ============================================================================

#[derive(SimpleObject)]
pub struct CourseEdge {
    pub cursor: String,
    pub node: Course,
}

#[derive(SimpleObject)]
pub struct CourseConnection {
    pub edges: Vec<CourseEdge>,
    pub page_info: PageInfo,
}

#[derive(SimpleObject, Default)]
pub struct PageInfo {
    pub has_next_page: bool,
    pub has_previous_page: bool,
    pub start_cursor: Option<String>,
    pub end_cursor: Option<String>,
}

// ============================================================================
// RESULT TYPES
// ============================================================================

#[derive(SimpleObject)]
pub struct CreateWebhookResult {
    pub webhook: Webhook,
    pub secret: String,
}

// ============================================================================
// SCHEMA DEFINITION
// ============================================================================

pub type Schema = async_graphql::Schema<QueryRoot, MutationRoot, EmptySubscription>;

pub fn create_schema(pool: sqlx::PgPool) -> Schema {
    Schema::build(QueryRoot, MutationRoot, EmptySubscription)
        .data(pool)
        .finish()
}
