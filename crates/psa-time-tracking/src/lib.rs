//! PSA Time Tracking Module
//!
//! Provides time tracking and timesheet functionality:
//! - Time entry recording
//! - Billable/non-billable tracking
//! - Timesheet submission and approval
//! - Timer functionality
//! - Integration with tickets and projects
//!
//! # Database Tables
//! All tables use the `tt_` prefix.

pub mod models;

pub use models::*;
