//! PSA Assets Module
//!
//! Provides asset and configuration management:
//! - Asset inventory (CMDB)
//! - Hardware and software tracking
//! - Warranty management
//! - Location tracking
//! - Relationship mapping
//!
//! # Database Tables
//! All tables use the `ast_` prefix.

pub mod models;

pub use models::*;
