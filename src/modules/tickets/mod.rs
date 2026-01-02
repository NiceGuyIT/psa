//! Ticketing Module
//!
//! Core service desk functionality for issue tracking and resolution.

mod models;
#[cfg(feature = "server")]
mod service;
#[cfg(feature = "server")]
mod routes;
#[cfg(feature = "server")]
mod automation;

pub use models::*;
#[cfg(feature = "server")]
pub use service::TicketService;
#[cfg(feature = "server")]
pub use routes::ticket_routes;
#[cfg(feature = "server")]
pub use automation::AutomationEngine;
