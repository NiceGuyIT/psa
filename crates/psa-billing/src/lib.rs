//! PSA Billing Module
//!
//! Provides billing and invoicing functionality:
//! - Invoice generation
//! - Payment tracking
//! - Recurring billing
//! - Contract management
//! - Payment gateway integration
//!
//! # Database Tables
//! All tables use the `bill_` prefix.

pub mod models;

pub use models::*;
