// Webhook Delivery Worker
// Background worker for processing and delivering webhooks
// In production, this would use a message queue like Redis/RabbitMQ

use chrono::{DateTime, Utc};
use sqlx::PgPool;
use std::time::Duration;
use tokio::time;
use tracing::{info, warn, error};

/// Webhook delivery job
#[derive(Debug, Clone)]
pub struct WebhookDeliveryJob {
    pub id: uuid::Uuid,
    pub webhook_id: uuid::Uuid,
    pub url: String,
    pub event_type: String,
    pub payload: String,
    pub secret: String,
    pub attempts: i32,
    pub max_attempts: i32,
}

/// Webhook delivery result
#[derive(Debug)]
pub struct DeliveryResult {
    pub success: bool,
    pub response_code: Option<i32>,
    pub response_body: Option<String>,
    pub error_message: Option<String>,
}

/// Start the webhook delivery worker
pub async fn run_webhook_worker(pool: PgPool, shutdown: tokio::sync::broadcast::Receiver<()>) {
    info!("Starting webhook delivery worker");
    
    let mut interval = time::interval(Duration::from_secs(5)); // Check every 5 seconds
    
    loop {
        tokio::select! {
            _ = interval.tick() => {
                if let Err(e) = process_pending_deliveries(&pool).await {
                    error!("Error processing webhook deliveries: {}", e);
                }
            }
            _ = shutdown.recv() => {
                info!("Shutting down webhook worker");
                break;
            }
        }
    }
}

/// Process pending webhook deliveries
async fn process_pending_deliveries(pool: &PgPool) -> Result<(), String> {
    // Fetch pending deliveries that are ready to be processed
    let deliveries = sqlx::query!(
        r#"SELECT 
            wd.id, wd.webhook_id, wd.event_type, wd.payload, wd.attempts, wd.max_attempts,
            we.url, we.secret
           FROM webhook_deliveries wd
           JOIN webhook_endpoints we ON wd.webhook_id = we.id
           WHERE wd.status = 'pending'
             AND (wd.next_retry_at IS NULL OR wd.next_retry_at <= $1)
             AND wd.attempts < wd.max_attempts
           LIMIT 50"#,
        Utc::now()
    )
    .fetch_all(pool)
    .await
    .map_err(|e| e.to_string())?;

    if deliveries.is_empty() {
        return Ok(());
    }

    info!("Processing {} pending webhook deliveries", deliveries.len());

    for delivery in deliveries {
        let job = WebhookDeliveryJob {
            id: delivery.id,
            webhook_id: delivery.webhook_id,
            url: delivery.url,
            event_type: delivery.event_type,
            payload: delivery.payload,
            secret: delivery.secret,
            attempts: delivery.attempts,
            max_attempts: delivery.max_attempts,
        };

        // Process delivery
        match deliver_webhook(job.clone()).await {
            Ok(result) => {
                // Update delivery status
                update_delivery_status(
                    pool,
                    job.id,
                    &result,
                    job.attempts + 1,
                ).await?;

                if result.success {
                    info!("Webhook delivered successfully: {}", job.id);
                } else {
                    warn!("Webhook delivery failed: {} - {:?}", job.id, result.error_message);
                }
            }
            Err(e) => {
                error!("Error delivering webhook {}: {}", job.id, e);
                
                // Mark as failed with error
                let result = DeliveryResult {
                    success: false,
                    response_code: None,
                    response_body: None,
                    error_message: Some(e),
                };
                
                update_delivery_status(pool, job.id, &result, job.attempts + 1).await?;
            }
        }
    }

    Ok(())
}

/// Deliver a webhook to its endpoint
async fn deliver_webhook(job: WebhookDeliveryJob) -> Result<DeliveryResult, String> {
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(30))
        .build()
        .map_err(|e| e.to_string())?;

    // Generate HMAC signature for payload verification
    let signature = generate_hmac_signature(&job.secret, &job.payload);

    let response = client
        .post(&job.url)
        .header("Content-Type", "application/json")
        .header("X-Webhook-Signature", signature)
        .header("X-Webhook-Event", &job.event_type)
        .body(job.payload.clone())
        .send()
        .await
        .map_err(|e| format!("Request failed: {}", e))?;

    let status = response.status().as_u16() as i32;
    let body = response.text().await.unwrap_or_default();

    let success = status >= 200 && status < 300;

    Ok(DeliveryResult {
        success,
        response_code: Some(status),
        response_body: Some(body),
        error_message: if success { None } else { Some(format!("HTTP {}", status)) },
    })
}

/// Update delivery status in database
async fn update_delivery_status(
    pool: &PgPool,
    delivery_id: uuid::Uuid,
    result: &DeliveryResult,
    attempts: i32,
) -> Result<(), String> {
    let now = Utc::now();
    
    let (status, next_retry_at, delivered_at) = if result.success {
        ("success", None, Some(now))
    } else if attempts >= 5 {
        ("failed", None, None)
    } else {
        // Schedule retry with exponential backoff
        let delay = Duration::from_secs(60 * 2u64.pow(attempts as u32)); // 2min, 4min, 8min, 16min, 32min
        let retry_at = now + chrono::Duration::from_std(delay).unwrap();
        ("pending", Some(retry_at), None)
    };

    sqlx::query!(
        r#"UPDATE webhook_deliveries
           SET status = $1,
               response_code = $2,
               response_body = $3,
               error_message = $4,
               attempts = $5,
               next_retry_at = $6,
               delivered_at = $7,
               updated_at = $8
           WHERE id = $9"#,
        status,
        result.response_code,
        result.response_body,
        result.error_message,
        attempts,
        next_retry_at,
        delivered_at,
        now,
        delivery_id
    )
    .execute(pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok(())
}

/// Generate HMAC signature for webhook payload
fn generate_hmac_signature(secret: &str, payload: &str) -> String {
    use hmac::{Hmac, Mac};
    use sha2::Sha256;

    type HmacSha256 = Hmac<Sha256>;

    let mut mac = HmacSha256::new_from_slice(secret.as_bytes())
        .expect("HMAC can take key of any size");
    mac.update(payload.as_bytes());
    let result = mac.finalize();
    
    format!("sha256={}", hex::encode(result.into_bytes()))
}

/// Trigger a webhook for an event
pub async fn trigger_webhook(
    pool: &PgPool,
    event_type: &str,
    event_id: uuid::Uuid,
    payload: serde_json::Value,
) -> Result<(), String> {
    // Find active webhooks subscribed to this event
    let webhooks = sqlx::query!(
        "SELECT id FROM webhook_endpoints 
         WHERE is_active = true AND events ? $1",
        event_type
    )
    .fetch_all(pool)
    .await
    .map_err(|e| e.to_string())?;

    if webhooks.is_empty() {
        return Ok(()); // No webhooks to trigger
    }

    let payload_str = serde_json::to_string(&payload).unwrap_or_default();

    // Create delivery records for each webhook
    for webhook in webhooks {
        let delivery_id = uuid::Uuid::new_v4();

        sqlx::query!(
            r#"INSERT INTO webhook_deliveries 
               (id, webhook_id, event_type, event_id, payload, status, attempts, created_at, updated_at)
               VALUES ($1, $2, $3, $4, $5, 'pending', 0, $6, $7)"#,
            delivery_id,
            webhook.id,
            event_type,
            event_id,
            payload_str,
            Utc::now(),
            Utc::now()
        )
        .execute(pool)
        .await
        .map_err(|e| e.to_string())?;
    }

    info!("Triggered {} webhook(s) for event {}", webhooks.len(), event_type);

    Ok(())
}
