//! Ticketing models

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use psa_core::models::{TenantId, UserId};

/// Unique identifier for a ticket
pub type TicketId = Uuid;

/// Ticket status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(sqlx::Type))]
#[cfg_attr(feature = "server", sqlx(type_name = "ticket_status", rename_all = "snake_case"))]
#[serde(rename_all = "snake_case")]
pub enum TicketStatus {
    #[default]
    New,
    Open,
    InProgress,
    Pending,
    Resolved,
    Closed,
}

impl TicketStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            TicketStatus::New => "new",
            TicketStatus::Open => "open",
            TicketStatus::InProgress => "in_progress",
            TicketStatus::Pending => "pending",
            TicketStatus::Resolved => "resolved",
            TicketStatus::Closed => "closed",
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            TicketStatus::New => "New",
            TicketStatus::Open => "Open",
            TicketStatus::InProgress => "In Progress",
            TicketStatus::Pending => "Pending",
            TicketStatus::Resolved => "Resolved",
            TicketStatus::Closed => "Closed",
        }
    }

    pub fn is_open(&self) -> bool {
        !matches!(self, TicketStatus::Resolved | TicketStatus::Closed)
    }

    pub fn all() -> Vec<TicketStatus> {
        vec![
            TicketStatus::New,
            TicketStatus::Open,
            TicketStatus::InProgress,
            TicketStatus::Pending,
            TicketStatus::Resolved,
            TicketStatus::Closed,
        ]
    }
}

impl std::str::FromStr for TicketStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "new" => Ok(TicketStatus::New),
            "open" => Ok(TicketStatus::Open),
            "in_progress" => Ok(TicketStatus::InProgress),
            "pending" => Ok(TicketStatus::Pending),
            "resolved" => Ok(TicketStatus::Resolved),
            "closed" => Ok(TicketStatus::Closed),
            _ => Err(format!("Unknown ticket status: {}", s)),
        }
    }
}

/// Ticket priority
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(sqlx::Type))]
#[cfg_attr(feature = "server", sqlx(type_name = "ticket_priority", rename_all = "snake_case"))]
#[serde(rename_all = "snake_case")]
pub enum TicketPriority {
    Low,
    #[default]
    Medium,
    High,
    Critical,
}

impl TicketPriority {
    pub fn as_str(&self) -> &'static str {
        match self {
            TicketPriority::Low => "low",
            TicketPriority::Medium => "medium",
            TicketPriority::High => "high",
            TicketPriority::Critical => "critical",
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            TicketPriority::Low => "Low",
            TicketPriority::Medium => "Medium",
            TicketPriority::High => "High",
            TicketPriority::Critical => "Critical",
        }
    }

    /// Get SLA response time multiplier
    pub fn sla_multiplier(&self) -> f64 {
        match self {
            TicketPriority::Low => 2.0,
            TicketPriority::Medium => 1.0,
            TicketPriority::High => 0.5,
            TicketPriority::Critical => 0.25,
        }
    }

    pub fn all() -> Vec<TicketPriority> {
        vec![
            TicketPriority::Low,
            TicketPriority::Medium,
            TicketPriority::High,
            TicketPriority::Critical,
        ]
    }
}

impl std::str::FromStr for TicketPriority {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "low" => Ok(TicketPriority::Low),
            "medium" => Ok(TicketPriority::Medium),
            "high" => Ok(TicketPriority::High),
            "critical" => Ok(TicketPriority::Critical),
            _ => Err(format!("Unknown priority: {}", s)),
        }
    }
}

/// A support ticket
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(sqlx::FromRow))]
pub struct Ticket {
    pub id: TicketId,
    pub tenant_id: TenantId,
    pub ticket_number: i32,
    pub subject: String,
    pub description: String,
    pub status: TicketStatus,
    pub priority: TicketPriority,
    pub requester_id: UserId,
    pub assignee_id: Option<UserId>,
    pub queue_id: Option<Uuid>,
    pub sla_id: Option<Uuid>,
    pub due_date: Option<DateTime<Utc>>,
    pub first_response_at: Option<DateTime<Utc>>,
    pub resolved_at: Option<DateTime<Utc>>,
    pub closed_at: Option<DateTime<Utc>>,
    pub tags: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Ticket {
    /// Calculate SLA status
    pub fn sla_status(&self) -> SlaStatus {
        let now = Utc::now();

        if let Some(due_date) = self.due_date {
            if self.status == TicketStatus::Resolved || self.status == TicketStatus::Closed {
                if let Some(resolved_at) = self.resolved_at {
                    if resolved_at <= due_date {
                        return SlaStatus::Met;
                    } else {
                        return SlaStatus::Breached;
                    }
                }
            }

            if now > due_date {
                return SlaStatus::Breached;
            }

            // Warning if within 20% of due time
            let created = self.created_at;
            let total_time = (due_date - created).num_seconds() as f64;
            let elapsed = (now - created).num_seconds() as f64;

            if elapsed / total_time > 0.8 {
                return SlaStatus::Warning;
            }

            SlaStatus::OnTrack;
        }

        SlaStatus::None
    }
}

/// SLA compliance status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SlaStatus {
    None,
    OnTrack,
    Warning,
    Breached,
    Met,
}

impl SlaStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            SlaStatus::None => "none",
            SlaStatus::OnTrack => "on_track",
            SlaStatus::Warning => "warning",
            SlaStatus::Breached => "breached",
            SlaStatus::Met => "met",
        }
    }
}

/// Create ticket request
#[derive(Debug, Deserialize, Validate)]
pub struct CreateTicketRequest {
    #[validate(length(min = 1, max = 500))]
    pub subject: String,

    #[validate(length(min = 1))]
    pub description: String,

    pub priority: Option<TicketPriority>,

    pub assignee_id: Option<UserId>,

    pub queue_id: Option<Uuid>,

    #[validate(length(max = 10))]
    pub tags: Option<Vec<String>>,
}

/// Update ticket request
#[derive(Debug, Deserialize, Validate)]
pub struct UpdateTicketRequest {
    #[validate(length(min = 1, max = 500))]
    pub subject: Option<String>,

    pub description: Option<String>,

    pub status: Option<TicketStatus>,

    pub priority: Option<TicketPriority>,

    pub assignee_id: Option<UserId>,

    pub queue_id: Option<Uuid>,

    pub tags: Option<Vec<String>>,
}

/// Ticket comment/note
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(sqlx::FromRow))]
pub struct TicketComment {
    pub id: Uuid,
    pub ticket_id: TicketId,
    pub author_id: UserId,
    pub content: String,
    pub is_internal: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// SLA definition
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(sqlx::FromRow))]
pub struct Sla {
    pub id: Uuid,
    pub tenant_id: TenantId,
    pub name: String,
    pub description: Option<String>,
    /// First response time in minutes
    pub first_response_minutes: i32,
    /// Resolution time in minutes
    pub resolution_minutes: i32,
    /// Business hours only
    pub business_hours_only: bool,
    pub is_default: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Ticket queue
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(sqlx::FromRow))]
pub struct TicketQueue {
    pub id: Uuid,
    pub tenant_id: TenantId,
    pub name: String,
    pub description: Option<String>,
    pub default_assignee_id: Option<UserId>,
    pub default_sla_id: Option<Uuid>,
    pub is_default: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Ticket list filters
#[derive(Debug, Clone, Default, Deserialize)]
pub struct TicketFilters {
    pub status: Option<TicketStatus>,
    pub priority: Option<TicketPriority>,
    pub assignee_id: Option<UserId>,
    pub queue_id: Option<Uuid>,
    pub requester_id: Option<UserId>,
    pub search: Option<String>,
}

/// Ticket statistics
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TicketStats {
    pub total: i64,
    pub open: i64,
    pub unassigned: i64,
    pub overdue: i64,
    pub resolved_today: i64,
}
