//! Contact Management Module
//!
//! Handles companies (clients), contacts, and sites.

mod models;
#[cfg(feature = "server")]
mod service;
#[cfg(feature = "server")]
mod routes;

pub use models::*;
#[cfg(feature = "server")]
pub use service::ContactService;
#[cfg(feature = "server")]
pub use routes::contact_routes;
