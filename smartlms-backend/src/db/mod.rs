//! Database connection pool

use std::sync::Arc;
use std::time::Duration;

use sqlx::{
    mysql::{MySqlPool, MySqlPoolOptions},
    postgres::{PgPool, PgPoolOptions},
    ConnectOptions,
};

/// Database pool type - wraps MySQL and PostgreSQL pools
pub type DbPool = Arc<tokio::sync::RwLock<Option<AnyPool>>>;

/// Any pool that can be either MySQL or PostgreSQL
pub enum AnyPool {
    Postgres(PgPool),
    MySql(MySqlPool),
}

impl AnyPool {
    pub fn postgres() -> Self {
        AnyPool::Postgres(PgPool::lazy_empty())
    }
    
    pub fn mysql() -> Self {
        AnyPool::MySql(MySqlPool::lazy_empty())
    }
    
    pub fn is_postgres(&self) -> bool {
        matches!(self, AnyPool::Postgres(_))
    }
}

/// Database configuration
#[derive(Debug, Clone)]
pub struct DbConfig {
    pub url: String,
    pub max_connections: u32,
    pub min_connections: u32,
    pub connect_timeout: Duration,
    pub idle_timeout: Duration,
}

impl Default for DbConfig {
    fn default() -> Self {
        Self {
            url: std::env::var("DATABASE_URL")
                .unwrap_or_else(|_| "postgres://postgres:postgres@localhost/smartlms".to_string()),
            max_connections: 10,
            min_connections: 2,
            connect_timeout: Duration::from_secs(5),
            idle_timeout: Duration::from_secs(600),
        }
    }
}

impl DbPool {
    /// Create a new database pool
    pub async fn new() -> Result<Self, sqlx::Error> {
        let config = DbConfig::default();
        Self::with_config(config).await
    }
    
    /// Create a pool with custom configuration
    pub async fn with_config(config: DbConfig) -> Result<Self, sqlx::Error> {
        let url = config.url.clone();
        
        // Detect database type from URL
        let pool: AnyPool = if url.starts_with("postgres://") || url.starts_with("postgresql://") {
            let options = PgPoolOptions::new()
                .max_connections(config.max_connections)
                .min_connections(config.min_connections)
                .connect_timeout(config.connect_timeout)
                .idle_timeout(config.idle_timeout);
            
            let pool = PgPool::connect_with(
                ConnectOptions::new(url)
                    .connect_timeout(config.connect_timeout)
            ).await?;
            
            AnyPool::Postgres(pool)
        } else if url.starts_with("mysql://") {
            let options = MySqlPoolOptions::new()
                .max_connections(config.max_connections)
                .min_connections(config.min_connections)
                .connect_timeout(config.connect_timeout)
                .idle_timeout(config.idle_timeout);
            
            let pool = MySqlPool::connect_with(
                ConnectOptions::new(url)
                    .connect_timeout(config.connect_timeout)
            ).await?;
            
            AnyPool::MySql(pool)
        } else {
            // Default to PostgreSQL
            let pool = PgPool::connect_with(
                ConnectOptions::new(url)
                    .connect_timeout(config.connect_timeout)
            ).await?;
            
            AnyPool::Postgres(pool)
        };
        
        Ok(Arc::new(tokio::sync::RwLock::new(Some(pool))))
    }
    
    /// Get the underlying PostgreSQL pool
    pub async fn postgres(&self) -> Option<PgPool> {
        let pool = self.read().await;
        match &*pool {
            Some(AnyPool::Postgres(p)) => Some(p.clone()),
            _ => None,
        }
    }
    
    /// Get the underlying MySQL pool
    pub async fn mysql(&self) -> Option<MySqlPool> {
        let pool = self.read().await;
        match &*pool {
            Some(AnyPool::MySql(p)) => Some(p.clone()),
            _ => None,
        }
    }
}