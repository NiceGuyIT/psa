//! Audit logging for PSA Platform
//!
//! Tracks all changes to data for compliance and debugging.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use crate::models::{AuditRecord, TenantId, UserId};

/// Audit action types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuditAction {
    Create,
    Read,
    Update,
    Delete,
    Login,
    Logout,
    Export,
    Import,
}

impl AuditAction {
    pub fn as_str(&self) -> &'static str {
        match self {
            AuditAction::Create => "create",
            AuditAction::Read => "read",
            AuditAction::Update => "update",
            AuditAction::Delete => "delete",
            AuditAction::Login => "login",
            AuditAction::Logout => "logout",
            AuditAction::Export => "export",
            AuditAction::Import => "import",
        }
    }
}

/// Audit entry stored in database
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct AuditEntry {
    pub id: Uuid,
    pub tenant_id: Option<TenantId>,
    pub user_id: UserId,
    pub action: String,
    pub entity_type: String,
    pub entity_id: String,
    pub old_values: Option<serde_json::Value>,
    pub new_values: Option<serde_json::Value>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub created_at: DateTime<Utc>,
}

/// Builder for creating audit entries
pub struct AuditBuilder {
    tenant_id: Option<TenantId>,
    user_id: UserId,
    action: AuditAction,
    entity_type: String,
    entity_id: String,
    old_values: Option<serde_json::Value>,
    new_values: Option<serde_json::Value>,
    ip_address: Option<String>,
    user_agent: Option<String>,
}

impl AuditBuilder {
    pub fn new(user_id: UserId, action: AuditAction, entity_type: &str, entity_id: &str) -> Self {
        Self {
            tenant_id: None,
            user_id,
            action,
            entity_type: entity_type.to_string(),
            entity_id: entity_id.to_string(),
            old_values: None,
            new_values: None,
            ip_address: None,
            user_agent: None,
        }
    }

    pub fn tenant(mut self, tenant_id: TenantId) -> Self {
        self.tenant_id = Some(tenant_id);
        self
    }

    pub fn old_values<T: Serialize>(mut self, values: &T) -> Self {
        self.old_values = serde_json::to_value(values).ok();
        self
    }

    pub fn new_values<T: Serialize>(mut self, values: &T) -> Self {
        self.new_values = serde_json::to_value(values).ok();
        self
    }

    pub fn ip_address(mut self, ip: &str) -> Self {
        self.ip_address = Some(ip.to_string());
        self
    }

    pub fn user_agent(mut self, ua: &str) -> Self {
        self.user_agent = Some(ua.to_string());
        self
    }

    pub fn build(self) -> AuditEntry {
        AuditEntry {
            id: Uuid::new_v4(),
            tenant_id: self.tenant_id,
            user_id: self.user_id,
            action: self.action.as_str().to_string(),
            entity_type: self.entity_type,
            entity_id: self.entity_id,
            old_values: self.old_values,
            new_values: self.new_values,
            ip_address: self.ip_address,
            user_agent: self.user_agent,
            created_at: Utc::now(),
        }
    }
}

/// Audit repository for database operations
pub struct AuditRepository<'a> {
    pool: &'a sqlx::PgPool,
}

impl<'a> AuditRepository<'a> {
    pub fn new(pool: &'a sqlx::PgPool) -> Self {
        Self { pool }
    }

    /// Record an audit entry
    pub async fn record(&self, entry: &AuditEntry) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            INSERT INTO core_audit_log (id, tenant_id, user_id, action, entity_type, entity_id, old_values, new_values, ip_address, user_agent, created_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
            "#
        )
        .bind(&entry.id)
        .bind(&entry.tenant_id)
        .bind(&entry.user_id)
        .bind(&entry.action)
        .bind(&entry.entity_type)
        .bind(&entry.entity_id)
        .bind(&entry.old_values)
        .bind(&entry.new_values)
        .bind(&entry.ip_address)
        .bind(&entry.user_agent)
        .bind(&entry.created_at)
        .execute(self.pool)
        .await?;

        Ok(())
    }

    /// Find audit entries by entity
    pub async fn find_by_entity(
        &self,
        entity_type: &str,
        entity_id: &str,
    ) -> Result<Vec<AuditEntry>, sqlx::Error> {
        sqlx::query_as::<_, AuditEntry>(
            r#"
            SELECT * FROM core_audit_log
            WHERE entity_type = $1 AND entity_id = $2
            ORDER BY created_at DESC
            LIMIT 100
            "#
        )
        .bind(entity_type)
        .bind(entity_id)
        .fetch_all(self.pool)
        .await
    }

    /// Find audit entries by user
    pub async fn find_by_user(&self, user_id: UserId) -> Result<Vec<AuditEntry>, sqlx::Error> {
        sqlx::query_as::<_, AuditEntry>(
            r#"
            SELECT * FROM core_audit_log
            WHERE user_id = $1
            ORDER BY created_at DESC
            LIMIT 100
            "#
        )
        .bind(user_id)
        .fetch_all(self.pool)
        .await
    }

    /// Find audit entries by tenant
    pub async fn find_by_tenant(&self, tenant_id: TenantId) -> Result<Vec<AuditEntry>, sqlx::Error> {
        sqlx::query_as::<_, AuditEntry>(
            r#"
            SELECT * FROM core_audit_log
            WHERE tenant_id = $1
            ORDER BY created_at DESC
            LIMIT 100
            "#
        )
        .bind(tenant_id)
        .fetch_all(self.pool)
        .await
    }
}
