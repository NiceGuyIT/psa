//! Server functions for the PSA Platform
//!
//! These functions use Dioxus server functions to provide API endpoints
//! that work with both `dx serve` (development) and the production binary.

use dioxus::prelude::*;

/// Simple health check endpoint
#[server(HealthCheck)]
pub async fn health_check() -> Result<String, ServerFnError> {
    Ok("OK".to_string())
}

/// Get the server version
#[server(GetVersion)]
pub async fn get_version() -> Result<String, ServerFnError> {
    Ok(env!("CARGO_PKG_VERSION").to_string())
}
