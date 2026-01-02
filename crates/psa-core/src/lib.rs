//! PSA Core - Shared functionality for all PSA modules
//!
//! This crate provides core functionality shared across all PSA modules:
//! - Authentication (local + SSO)
//! - Multi-tenancy support
//! - Notification services (email, SMS, webhooks)
//! - Audit logging
//! - Database utilities
//! - Configuration management
//!
//! # Feature Flags
//!
//! - `server` - Server-side functionality (database, auth, etc.)
//! - `web` - Client-side WASM functionality
//! - `fullstack` - Both server and web
//! - `saas` - Multi-tenant SaaS deployment
//! - `single-tenant` - Self-hosted single tenant
//! - `standalone` - Individual standalone app mode

pub mod auth;
pub mod config;
pub mod error;
pub mod models;

#[cfg(feature = "server")]
pub mod db;

#[cfg(feature = "server")]
pub mod tenants;

#[cfg(feature = "server")]
pub mod notifications;

#[cfg(feature = "server")]
pub mod audit;

#[cfg(feature = "server")]
pub mod middleware;

// Re-export commonly used types
pub use error::{CoreError, Result};
pub use models::*;

#[cfg(feature = "server")]
pub use db::Database;

/// Check if running in SaaS mode
#[inline]
pub fn is_saas() -> bool {
    cfg!(feature = "saas")
}

/// Check if running in single-tenant mode
#[inline]
pub fn is_single_tenant() -> bool {
    cfg!(feature = "single-tenant")
}

/// Check if running in standalone mode
#[inline]
pub fn is_standalone() -> bool {
    cfg!(feature = "standalone")
}
