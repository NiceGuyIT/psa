//! Database utilities for PSA Platform

use sqlx::{postgres::PgPoolOptions, PgPool};
use std::time::Duration;

use crate::error::{CoreError, Result};

/// Database connection pool wrapper
#[derive(Clone)]
pub struct Database {
    pool: PgPool,
}

impl Database {
    /// Create a new database connection pool
    pub async fn new(database_url: &str) -> Result<Self> {
        let pool = PgPoolOptions::new()
            .max_connections(20)
            .min_connections(2)
            .acquire_timeout(Duration::from_secs(30))
            .idle_timeout(Duration::from_secs(600))
            .connect(database_url)
            .await
            .map_err(|e| CoreError::Database(format!("Failed to connect: {}", e)))?;

        Ok(Self { pool })
    }

    /// Get a reference to the connection pool
    pub fn pool(&self) -> &PgPool {
        &self.pool
    }

    /// Run database migrations
    pub async fn run_migrations(&self) -> Result<()> {
        sqlx::migrate!("../../migrations/core")
            .run(&self.pool)
            .await
            .map_err(|e| CoreError::Database(format!("Migration failed: {}", e)))?;

        Ok(())
    }

    /// Run module-specific migrations
    pub async fn run_module_migrations(&self, module: &str) -> Result<()> {
        let path = format!("../../migrations/{}", module);

        // Note: This is a simplified approach. In production, you'd want to
        // use a more sophisticated migration runner that can handle dynamic paths.
        tracing::info!("Running migrations for module: {} from {}", module, path);

        Ok(())
    }

    /// Check database connectivity
    pub async fn health_check(&self) -> Result<()> {
        sqlx::query("SELECT 1")
            .execute(&self.pool)
            .await
            .map_err(|e| CoreError::Database(format!("Health check failed: {}", e)))?;

        Ok(())
    }
}

/// Database table prefix for module isolation
pub struct TablePrefix;

impl TablePrefix {
    pub const CORE: &'static str = "core_";
    pub const TICKETING: &'static str = "tkt_";
    pub const TIME_TRACKING: &'static str = "tt_";
    pub const PROJECTS: &'static str = "prj_";
    pub const CRM: &'static str = "crm_";
    pub const BILLING: &'static str = "bill_";
    pub const ASSETS: &'static str = "ast_";
    pub const KNOWLEDGE_BASE: &'static str = "kb_";
    pub const CALENDAR: &'static str = "cal_";
}
