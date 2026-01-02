//! Tenant Management Module (Multi-tenant mode only)
//!
//! Handles tenant provisioning, configuration, and management.

mod models;
#[cfg(feature = "server")]
mod service;
#[cfg(feature = "server")]
mod routes;

pub use models::*;
#[cfg(feature = "server")]
pub use service::TenantService;
#[cfg(feature = "server")]
pub use routes::tenant_routes;
