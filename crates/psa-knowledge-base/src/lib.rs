//! PSA Knowledge Base Module
//!
//! Provides knowledge management functionality:
//! - Article creation and editing
//! - Markdown support
//! - Categories and tags
//! - Full-text search
//! - Version history
//!
//! # Database Tables
//! All tables use the `kb_` prefix.

pub mod models;

pub use models::*;
