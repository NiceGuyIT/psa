//! Billing models

use chrono::{DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use psa_core::models::TenantId;

/// Invoice status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum InvoiceStatus {
    #[default]
    Draft,
    Sent,
    Viewed,
    Paid,
    PartiallyPaid,
    Overdue,
    Cancelled,
}

/// Invoice
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Invoice {
    pub id: Uuid,
    pub tenant_id: TenantId,
    pub invoice_number: String,
    pub company_id: Uuid,
    pub status: InvoiceStatus,
    pub issue_date: NaiveDate,
    pub due_date: NaiveDate,
    pub subtotal: Decimal,
    pub tax_amount: Decimal,
    pub total: Decimal,
    pub amount_paid: Decimal,
    pub currency: String,
    pub notes: Option<String>,
    pub sent_at: Option<DateTime<Utc>>,
    pub paid_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Invoice line item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvoiceItem {
    pub id: Uuid,
    pub invoice_id: Uuid,
    pub description: String,
    pub quantity: Decimal,
    pub unit_price: Decimal,
    pub tax_rate: Decimal,
    pub amount: Decimal,
    pub time_entry_id: Option<Uuid>,
    pub sort_order: i32,
}

/// Payment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Payment {
    pub id: Uuid,
    pub tenant_id: TenantId,
    pub invoice_id: Uuid,
    pub amount: Decimal,
    pub payment_date: NaiveDate,
    pub payment_method: String,
    pub reference: Option<String>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
}

/// Contract/Agreement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contract {
    pub id: Uuid,
    pub tenant_id: TenantId,
    pub company_id: Uuid,
    pub name: String,
    pub start_date: NaiveDate,
    pub end_date: Option<NaiveDate>,
    pub value: Decimal,
    pub billing_frequency: String,
    pub is_active: bool,
    pub auto_renew: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
