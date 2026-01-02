//! PSA Ticketing Module
//!
//! Provides service desk ticketing functionality:
//! - Ticket creation, updates, and lifecycle management
//! - SLA definitions and tracking
//! - Email integration (inbound/outbound)
//! - Ticket queues and assignment
//! - Time tracking on tickets
//!
//! # Usage
//!
//! This module can be used:
//! - As a standalone app (Resolver)
//! - As part of a PSA suite (MSP Suite, etc.)
//!
//! # Database Tables
//!
//! All tables use the `tkt_` prefix:
//! - `tkt_tickets` - Main tickets table
//! - `tkt_ticket_comments` - Ticket comments/notes
//! - `tkt_slas` - SLA definitions
//! - `tkt_queues` - Ticket queues
//! - `tkt_statuses` - Custom statuses

pub mod models;
pub mod views;

#[cfg(feature = "server")]
pub mod handlers;

#[cfg(feature = "server")]
pub mod repository;

#[cfg(feature = "server")]
pub mod routes;

// Re-export commonly used types
pub use models::*;

#[cfg(feature = "server")]
pub use routes::ticketing_routes;
