//! PSA Platform Library
//!
//! This library provides the core functionality for the PSA platform.

pub mod api;
pub mod components;
pub mod db;
pub mod hooks;
pub mod modules;
pub mod pages;
pub mod utils;

// Re-export commonly used types
pub use modules::auth::{AuthState, CurrentUser};
pub use utils::error::{AppError, AppResult};

#[cfg(feature = "server")]
pub use db::Database;

/// Prelude module for common imports
pub mod prelude {
    pub use crate::modules::auth::{AuthState, CurrentUser};
    pub use crate::utils::error::{AppError, AppResult};
    pub use chrono::{DateTime, NaiveDate, NaiveTime, Utc};
    pub use serde::{Deserialize, Serialize};
    pub use uuid::Uuid;
    pub use validator::Validate;
}
