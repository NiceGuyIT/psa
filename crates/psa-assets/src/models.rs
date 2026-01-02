//! Asset management models

use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use psa_core::models::TenantId;

/// Asset type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AssetType {
    Hardware,
    Software,
    Network,
    Virtual,
    Cloud,
    Other,
}

/// Asset status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum AssetStatus {
    #[default]
    Active,
    InStock,
    InRepair,
    Retired,
    Disposed,
}

/// Asset/Configuration item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Asset {
    pub id: Uuid,
    pub tenant_id: TenantId,
    pub name: String,
    pub asset_tag: Option<String>,
    pub serial_number: Option<String>,
    pub asset_type: AssetType,
    pub status: AssetStatus,
    pub company_id: Option<Uuid>,
    pub location_id: Option<Uuid>,
    pub assigned_to: Option<Uuid>,
    pub manufacturer: Option<String>,
    pub model: Option<String>,
    pub purchase_date: Option<NaiveDate>,
    pub purchase_cost: Option<rust_decimal::Decimal>,
    pub warranty_expires: Option<NaiveDate>,
    pub notes: Option<String>,
    pub custom_fields: serde_json::Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Asset location
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Location {
    pub id: Uuid,
    pub tenant_id: TenantId,
    pub name: String,
    pub address: Option<String>,
    pub parent_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Asset relationship
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetRelationship {
    pub id: Uuid,
    pub parent_asset_id: Uuid,
    pub child_asset_id: Uuid,
    pub relationship_type: String,
    pub created_at: DateTime<Utc>,
}
