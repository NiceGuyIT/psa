//! PSA CRM Module
//!
//! Provides customer relationship management:
//! - Contact management
//! - Company/organization management
//! - Opportunity tracking
//! - Activity logging
//! - Notes and communication history
//!
//! # Database Tables
//! All tables use the `crm_` prefix.

pub mod models;

pub use models::*;
