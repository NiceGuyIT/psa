//! Multi-tenancy support for PSA Platform

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use crate::models::TenantId;

/// Tenant status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "tenant_status", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum TenantStatus {
    Active,
    Trial,
    Suspended,
    Cancelled,
}

impl TenantStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            TenantStatus::Active => "active",
            TenantStatus::Trial => "trial",
            TenantStatus::Suspended => "suspended",
            TenantStatus::Cancelled => "cancelled",
        }
    }

    pub fn is_accessible(&self) -> bool {
        matches!(self, TenantStatus::Active | TenantStatus::Trial)
    }
}

/// Subscription tier
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "subscription_tier", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum SubscriptionTier {
    /// Free tier with limited features
    Free,
    /// Individual/Personal tier (standalone apps)
    Personal,
    /// Small business tier (suite apps)
    Professional,
    /// Enterprise tier with all features
    Enterprise,
}

impl SubscriptionTier {
    pub fn as_str(&self) -> &'static str {
        match self {
            SubscriptionTier::Free => "free",
            SubscriptionTier::Personal => "personal",
            SubscriptionTier::Professional => "professional",
            SubscriptionTier::Enterprise => "enterprise",
        }
    }
}

/// Tenant (organization) in the system
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Tenant {
    pub id: TenantId,
    pub name: String,
    pub slug: String,
    pub status: TenantStatus,
    pub tier: SubscriptionTier,
    pub trial_ends_at: Option<DateTime<Utc>>,
    pub settings: serde_json::Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Tenant settings
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TenantSettings {
    /// Custom branding
    pub branding: Option<BrandingSettings>,
    /// Enabled modules for this tenant
    pub enabled_modules: Vec<String>,
    /// SSO configuration
    pub sso_enabled: bool,
    /// Default timezone
    pub timezone: String,
    /// Date format preference
    pub date_format: String,
    /// Currency for billing
    pub currency: String,
}

/// Branding settings for white-labeling
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrandingSettings {
    pub logo_url: Option<String>,
    pub primary_color: Option<String>,
    pub company_name: Option<String>,
}

/// Create tenant request
#[derive(Debug, Deserialize)]
pub struct CreateTenantRequest {
    pub name: String,
    pub slug: String,
    pub tier: SubscriptionTier,
    pub admin_email: String,
    pub admin_name: String,
    pub admin_password: String,
}

/// Tenant repository for database operations
pub struct TenantRepository<'a> {
    pool: &'a sqlx::PgPool,
}

impl<'a> TenantRepository<'a> {
    pub fn new(pool: &'a sqlx::PgPool) -> Self {
        Self { pool }
    }

    /// Find tenant by ID
    pub async fn find_by_id(&self, id: TenantId) -> Result<Option<Tenant>, sqlx::Error> {
        sqlx::query_as::<_, Tenant>(
            "SELECT * FROM core_tenants WHERE id = $1"
        )
        .bind(id)
        .fetch_optional(self.pool)
        .await
    }

    /// Find tenant by slug
    pub async fn find_by_slug(&self, slug: &str) -> Result<Option<Tenant>, sqlx::Error> {
        sqlx::query_as::<_, Tenant>(
            "SELECT * FROM core_tenants WHERE slug = $1"
        )
        .bind(slug)
        .fetch_optional(self.pool)
        .await
    }

    /// List all active tenants
    pub async fn list_active(&self) -> Result<Vec<Tenant>, sqlx::Error> {
        sqlx::query_as::<_, Tenant>(
            "SELECT * FROM core_tenants WHERE status IN ('active', 'trial') ORDER BY name"
        )
        .fetch_all(self.pool)
        .await
    }

    /// Create a new tenant
    pub async fn create(&self, tenant: &Tenant) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            INSERT INTO core_tenants (id, name, slug, status, tier, trial_ends_at, settings, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            "#
        )
        .bind(&tenant.id)
        .bind(&tenant.name)
        .bind(&tenant.slug)
        .bind(&tenant.status)
        .bind(&tenant.tier)
        .bind(&tenant.trial_ends_at)
        .bind(&tenant.settings)
        .bind(&tenant.created_at)
        .bind(&tenant.updated_at)
        .execute(self.pool)
        .await?;

        Ok(())
    }

    /// Update tenant status
    pub async fn update_status(&self, id: TenantId, status: TenantStatus) -> Result<(), sqlx::Error> {
        sqlx::query(
            "UPDATE core_tenants SET status = $2, updated_at = NOW() WHERE id = $1"
        )
        .bind(id)
        .bind(status)
        .execute(self.pool)
        .await?;

        Ok(())
    }
}
