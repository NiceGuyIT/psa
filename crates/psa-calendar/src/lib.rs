//! PSA Calendar Module
//!
//! Provides calendar and scheduling functionality:
//! - Event management
//! - Recurring events
//! - Availability tracking
//! - Appointment booking
//! - Calendar sync
//!
//! # Database Tables
//! All tables use the `cal_` prefix.

pub mod models;

pub use models::*;
