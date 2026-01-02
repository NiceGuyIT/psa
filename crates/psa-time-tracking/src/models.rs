//! Time tracking models

use chrono::{DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use psa_core::models::{TenantId, UserId};

/// Time entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeEntry {
    pub id: Uuid,
    pub tenant_id: TenantId,
    pub user_id: UserId,
    pub date: NaiveDate,
    pub hours: Decimal,
    pub description: String,
    pub is_billable: bool,
    pub ticket_id: Option<Uuid>,
    pub project_id: Option<Uuid>,
    pub task_id: Option<Uuid>,
    pub hourly_rate: Option<Decimal>,
    pub approved_at: Option<DateTime<Utc>>,
    pub approved_by: Option<UserId>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Timesheet status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TimesheetStatus {
    Draft,
    Submitted,
    Approved,
    Rejected,
}

/// Weekly timesheet
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Timesheet {
    pub id: Uuid,
    pub tenant_id: TenantId,
    pub user_id: UserId,
    pub week_start: NaiveDate,
    pub status: TimesheetStatus,
    pub total_hours: Decimal,
    pub billable_hours: Decimal,
    pub submitted_at: Option<DateTime<Utc>>,
    pub approved_at: Option<DateTime<Utc>>,
    pub approved_by: Option<UserId>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
