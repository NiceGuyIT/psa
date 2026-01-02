//! Calendar models

use chrono::{DateTime, NaiveTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use psa_core::models::{TenantId, UserId};

/// Event type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum EventType {
    #[default]
    Meeting,
    Appointment,
    Task,
    Reminder,
    BlockedTime,
}

/// Calendar event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub id: Uuid,
    pub tenant_id: TenantId,
    pub title: String,
    pub description: Option<String>,
    pub event_type: EventType,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub all_day: bool,
    pub location: Option<String>,
    pub organizer_id: UserId,
    pub recurrence_rule: Option<String>,
    pub recurrence_end: Option<DateTime<Utc>>,
    pub ticket_id: Option<Uuid>,
    pub project_id: Option<Uuid>,
    pub company_id: Option<Uuid>,
    pub contact_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Event attendee
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventAttendee {
    pub id: Uuid,
    pub event_id: Uuid,
    pub user_id: Option<UserId>,
    pub email: String,
    pub status: AttendeeStatus,
    pub created_at: DateTime<Utc>,
}

/// Attendee response status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum AttendeeStatus {
    #[default]
    Pending,
    Accepted,
    Declined,
    Tentative,
}

/// User availability schedule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Availability {
    pub id: Uuid,
    pub user_id: UserId,
    pub day_of_week: i16,
    pub start_time: NaiveTime,
    pub end_time: NaiveTime,
    pub is_available: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Booking slot for appointments
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BookingSlot {
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub available: bool,
}
