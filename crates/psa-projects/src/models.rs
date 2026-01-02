//! Project management models

use chrono::{DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use psa_core::models::{TenantId, UserId};

/// Project status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum ProjectStatus {
    #[default]
    Planning,
    Active,
    OnHold,
    Completed,
    Cancelled,
}

/// Project
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub id: Uuid,
    pub tenant_id: TenantId,
    pub name: String,
    pub description: Option<String>,
    pub status: ProjectStatus,
    pub client_id: Option<Uuid>,
    pub manager_id: Option<UserId>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub budget_hours: Option<Decimal>,
    pub budget_amount: Option<Decimal>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Task status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum TaskStatus {
    #[default]
    Todo,
    InProgress,
    Review,
    Done,
}

/// Project task
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: Uuid,
    pub project_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub status: TaskStatus,
    pub assignee_id: Option<UserId>,
    pub due_date: Option<NaiveDate>,
    pub estimated_hours: Option<Decimal>,
    pub parent_task_id: Option<Uuid>,
    pub sort_order: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Project milestone
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Milestone {
    pub id: Uuid,
    pub project_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub due_date: NaiveDate,
    pub completed_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
