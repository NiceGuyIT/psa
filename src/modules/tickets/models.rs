//! Ticket models and types

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

// ============================================================================
// TICKET SOURCE
// ============================================================================

/// How the ticket was created
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum TicketSource {
    #[default]
    Portal,
    Email,
    Phone,
    Api,
    Chat,
    Rmm,
    Internal,
}

impl TicketSource {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "portal" => Some(Self::Portal),
            "email" => Some(Self::Email),
            "phone" => Some(Self::Phone),
            "api" => Some(Self::Api),
            "chat" => Some(Self::Chat),
            "rmm" => Some(Self::Rmm),
            "internal" => Some(Self::Internal),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Portal => "portal",
            Self::Email => "email",
            Self::Phone => "phone",
            Self::Api => "api",
            Self::Chat => "chat",
            Self::Rmm => "rmm",
            Self::Internal => "internal",
        }
    }
}

// ============================================================================
// BILLING STATUS
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum BillingStatus {
    #[default]
    NotBilled,
    ReadyToBill,
    Billed,
}

impl BillingStatus {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "not_billed" => Some(Self::NotBilled),
            "ready_to_bill" => Some(Self::ReadyToBill),
            "billed" => Some(Self::Billed),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::NotBilled => "not_billed",
            Self::ReadyToBill => "ready_to_bill",
            Self::Billed => "billed",
        }
    }
}

// ============================================================================
// NOTE TYPE
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum NoteType {
    #[default]
    Internal,
    Public,
    Resolution,
    TimeEntry,
}

impl NoteType {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "internal" => Some(Self::Internal),
            "public" => Some(Self::Public),
            "resolution" => Some(Self::Resolution),
            "time_entry" => Some(Self::TimeEntry),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Internal => "internal",
            Self::Public => "public",
            Self::Resolution => "resolution",
            Self::TimeEntry => "time_entry",
        }
    }
}

// ============================================================================
// TICKET STATUS
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TicketStatus {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub name: String,
    pub color: String,
    pub is_closed: bool,
    pub is_default: bool,
    pub sort_order: i32,
}

// ============================================================================
// TICKET PRIORITY
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TicketPriority {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub name: String,
    pub color: String,
    pub icon: Option<String>,
    pub sla_multiplier: f64,
    pub sort_order: i32,
    pub is_default: bool,
}

// ============================================================================
// TICKET TYPE
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TicketType {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub icon: Option<String>,
    pub is_active: bool,
    pub sort_order: i32,
}

// ============================================================================
// TICKET QUEUE
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TicketQueue {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub color: Option<String>,
    pub icon: Option<String>,
    pub is_default: bool,
    pub sort_order: i32,
}

// ============================================================================
// TICKET
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ticket {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub ticket_number: String,
    pub title: String,
    pub description: Option<String>,
    pub status_id: Uuid,
    pub priority_id: Uuid,
    pub type_id: Option<Uuid>,
    pub category_id: Option<Uuid>,
    pub subcategory_id: Option<Uuid>,
    pub queue_id: Uuid,
    pub source: TicketSource,
    pub company_id: Uuid,
    pub contact_id: Option<Uuid>,
    pub site_id: Option<Uuid>,
    pub assigned_to_id: Option<Uuid>,
    pub team_id: Option<Uuid>,
    pub parent_ticket_id: Option<Uuid>,
    pub contract_id: Option<Uuid>,
    pub sla_id: Option<Uuid>,
    pub sla_due_date: Option<DateTime<Utc>>,
    pub first_response_due: Option<DateTime<Utc>>,
    pub first_response_at: Option<DateTime<Utc>>,
    pub resolution_due: Option<DateTime<Utc>>,
    pub resolved_at: Option<DateTime<Utc>>,
    pub closed_at: Option<DateTime<Utc>>,
    pub scheduled_start: Option<DateTime<Utc>>,
    pub scheduled_end: Option<DateTime<Utc>>,
    pub estimated_hours: Option<f64>,
    pub actual_hours: f64,
    pub is_billable: bool,
    pub billing_status: BillingStatus,
    pub asset_id: Option<Uuid>,
    pub custom_fields: serde_json::Value,
    pub tags: Vec<String>,
    pub created_by_id: Uuid,
    pub last_updated_by_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Create ticket request
#[derive(Debug, Clone, Deserialize, Validate)]
pub struct CreateTicketRequest {
    #[validate(length(min = 1, max = 500))]
    pub title: String,
    pub description: Option<String>,
    pub priority_id: Option<Uuid>,
    pub type_id: Option<Uuid>,
    pub category_id: Option<Uuid>,
    pub queue_id: Option<Uuid>,
    #[serde(default)]
    pub source: TicketSource,
    pub company_id: Uuid,
    pub contact_id: Option<Uuid>,
    pub site_id: Option<Uuid>,
    pub assigned_to_id: Option<Uuid>,
    pub team_id: Option<Uuid>,
    pub contract_id: Option<Uuid>,
    pub sla_id: Option<Uuid>,
    pub scheduled_start: Option<DateTime<Utc>>,
    pub scheduled_end: Option<DateTime<Utc>>,
    pub estimated_hours: Option<f64>,
    #[serde(default = "default_true")]
    pub is_billable: bool,
    pub asset_id: Option<Uuid>,
    #[serde(default)]
    pub custom_fields: serde_json::Value,
    #[serde(default)]
    pub tags: Vec<String>,
}

fn default_true() -> bool {
    true
}

/// Update ticket request
#[derive(Debug, Clone, Deserialize, Validate)]
pub struct UpdateTicketRequest {
    #[validate(length(min = 1, max = 500))]
    pub title: Option<String>,
    pub description: Option<String>,
    pub status_id: Option<Uuid>,
    pub priority_id: Option<Uuid>,
    pub type_id: Option<Uuid>,
    pub category_id: Option<Uuid>,
    pub queue_id: Option<Uuid>,
    pub contact_id: Option<Uuid>,
    pub site_id: Option<Uuid>,
    pub assigned_to_id: Option<Uuid>,
    pub team_id: Option<Uuid>,
    pub contract_id: Option<Uuid>,
    pub sla_id: Option<Uuid>,
    pub scheduled_start: Option<DateTime<Utc>>,
    pub scheduled_end: Option<DateTime<Utc>>,
    pub estimated_hours: Option<f64>,
    pub is_billable: Option<bool>,
    pub billing_status: Option<BillingStatus>,
    pub asset_id: Option<Uuid>,
    pub custom_fields: Option<serde_json::Value>,
    pub tags: Option<Vec<String>>,
}

/// Ticket response for API
#[derive(Debug, Clone, Serialize)]
pub struct TicketResponse {
    pub id: Uuid,
    pub ticket_number: String,
    pub title: String,
    pub description: Option<String>,
    pub status: TicketStatusSummary,
    pub priority: TicketPrioritySummary,
    pub type_name: Option<String>,
    pub category_name: Option<String>,
    pub queue_name: String,
    pub source: TicketSource,
    pub company_id: Uuid,
    pub company_name: String,
    pub contact_id: Option<Uuid>,
    pub contact_name: Option<String>,
    pub assigned_to_id: Option<Uuid>,
    pub assigned_to_name: Option<String>,
    pub sla_due_date: Option<DateTime<Utc>>,
    pub sla_status: SlaStatus,
    pub is_billable: bool,
    pub billing_status: BillingStatus,
    pub estimated_hours: Option<f64>,
    pub actual_hours: f64,
    pub tags: Vec<String>,
    pub created_by_name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize)]
pub struct TicketStatusSummary {
    pub id: Uuid,
    pub name: String,
    pub color: String,
    pub is_closed: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct TicketPrioritySummary {
    pub id: Uuid,
    pub name: String,
    pub color: String,
}

/// SLA status indicator
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SlaStatus {
    OnTrack,
    Warning,
    Breached,
    NotApplicable,
}

impl Ticket {
    /// Calculate SLA status
    pub fn sla_status(&self) -> SlaStatus {
        if self.closed_at.is_some() {
            return SlaStatus::NotApplicable;
        }

        let Some(due) = self.sla_due_date else {
            return SlaStatus::NotApplicable;
        };

        let now = Utc::now();
        if now > due {
            SlaStatus::Breached
        } else if (due - now).num_hours() < 2 {
            SlaStatus::Warning
        } else {
            SlaStatus::OnTrack
        }
    }
}

// ============================================================================
// TICKET NOTES
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TicketNote {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub ticket_id: Uuid,
    pub note_type: NoteType,
    pub content: String,
    pub content_html: Option<String>,
    pub is_email_sent: bool,
    pub email_sent_at: Option<DateTime<Utc>>,
    pub created_by_id: Uuid,
    pub created_by_name: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Create note request
#[derive(Debug, Clone, Deserialize, Validate)]
pub struct CreateNoteRequest {
    #[serde(default)]
    pub note_type: NoteType,
    #[validate(length(min = 1))]
    pub content: String,
    /// Send email notification to contact
    #[serde(default)]
    pub send_email: bool,
}

/// Note response
#[derive(Debug, Clone, Serialize)]
pub struct TicketNoteResponse {
    pub id: Uuid,
    pub note_type: NoteType,
    pub content: String,
    pub is_email_sent: bool,
    pub created_by_id: Uuid,
    pub created_by_name: String,
    pub created_at: DateTime<Utc>,
}

// ============================================================================
// TICKET ATTACHMENTS
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TicketAttachment {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub ticket_id: Uuid,
    pub note_id: Option<Uuid>,
    pub file_name: String,
    pub file_size: i64,
    pub mime_type: String,
    pub storage_path: String,
    pub uploaded_by_id: Uuid,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize)]
pub struct TicketAttachmentResponse {
    pub id: Uuid,
    pub file_name: String,
    pub file_size: i64,
    pub mime_type: String,
    pub uploaded_by_name: String,
    pub created_at: DateTime<Utc>,
}

// ============================================================================
// TICKET FILTERS
// ============================================================================

#[derive(Debug, Clone, Deserialize, Default)]
pub struct TicketFilter {
    pub q: Option<String>,
    pub status_id: Option<Uuid>,
    pub priority_id: Option<Uuid>,
    pub type_id: Option<Uuid>,
    pub queue_id: Option<Uuid>,
    pub company_id: Option<Uuid>,
    pub contact_id: Option<Uuid>,
    pub assigned_to_id: Option<Uuid>,
    pub team_id: Option<Uuid>,
    pub is_unassigned: Option<bool>,
    pub is_overdue: Option<bool>,
    pub is_open: Option<bool>,
    pub billing_status: Option<BillingStatus>,
    pub created_from: Option<DateTime<Utc>>,
    pub created_to: Option<DateTime<Utc>>,
    pub tags: Option<String>,
}

// ============================================================================
// TICKET ACTIVITY
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum TicketActivity {
    Created {
        user_id: Uuid,
        user_name: String,
        timestamp: DateTime<Utc>,
    },
    StatusChanged {
        user_id: Uuid,
        user_name: String,
        from_status: String,
        to_status: String,
        timestamp: DateTime<Utc>,
    },
    Assigned {
        user_id: Uuid,
        user_name: String,
        assigned_to_name: String,
        timestamp: DateTime<Utc>,
    },
    NoteAdded {
        user_id: Uuid,
        user_name: String,
        note_type: NoteType,
        timestamp: DateTime<Utc>,
    },
    PriorityChanged {
        user_id: Uuid,
        user_name: String,
        from_priority: String,
        to_priority: String,
        timestamp: DateTime<Utc>,
    },
    TimeLogged {
        user_id: Uuid,
        user_name: String,
        duration_minutes: i32,
        timestamp: DateTime<Utc>,
    },
}

// ============================================================================
// AUTOMATION TYPES
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AutomationTrigger {
    OnCreate,
    OnUpdate,
    OnSchedule,
    OnSlaBreach,
    OnSlaWarning,
    OnAging,
}

impl AutomationTrigger {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "on_create" => Some(Self::OnCreate),
            "on_update" => Some(Self::OnUpdate),
            "on_schedule" => Some(Self::OnSchedule),
            "on_sla_breach" => Some(Self::OnSlaBreach),
            "on_sla_warning" => Some(Self::OnSlaWarning),
            "on_aging" => Some(Self::OnAging),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::OnCreate => "on_create",
            Self::OnUpdate => "on_update",
            Self::OnSchedule => "on_schedule",
            Self::OnSlaBreach => "on_sla_breach",
            Self::OnSlaWarning => "on_sla_warning",
            Self::OnAging => "on_aging",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutomationRule {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub is_active: bool,
    pub trigger_type: AutomationTrigger,
    pub conditions: serde_json::Value,
    pub actions: serde_json::Value,
    pub priority: i32,
    pub last_run_at: Option<DateTime<Utc>>,
    pub run_count: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Automation condition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutomationCondition {
    pub field: String,
    pub operator: String,
    pub value: serde_json::Value,
}

/// Automation action
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutomationAction {
    pub action_type: String,
    pub params: serde_json::Value,
}
