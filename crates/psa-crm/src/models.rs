//! CRM models

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use psa_core::models::{TenantId, UserId};

/// Contact
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contact {
    pub id: Uuid,
    pub tenant_id: TenantId,
    pub first_name: String,
    pub last_name: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub mobile: Option<String>,
    pub job_title: Option<String>,
    pub company_id: Option<Uuid>,
    pub is_primary: bool,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Contact {
    pub fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }
}

/// Company/Organization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Company {
    pub id: Uuid,
    pub tenant_id: TenantId,
    pub name: String,
    pub website: Option<String>,
    pub phone: Option<String>,
    pub address_line1: Option<String>,
    pub address_line2: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub postal_code: Option<String>,
    pub country: Option<String>,
    pub industry: Option<String>,
    pub notes: Option<String>,
    pub account_manager_id: Option<UserId>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Opportunity status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum OpportunityStatus {
    #[default]
    Lead,
    Qualified,
    Proposal,
    Negotiation,
    Won,
    Lost,
}

/// Sales opportunity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Opportunity {
    pub id: Uuid,
    pub tenant_id: TenantId,
    pub name: String,
    pub company_id: Option<Uuid>,
    pub contact_id: Option<Uuid>,
    pub status: OpportunityStatus,
    pub value: Option<rust_decimal::Decimal>,
    pub probability: Option<i32>,
    pub expected_close_date: Option<chrono::NaiveDate>,
    pub owner_id: Option<UserId>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
