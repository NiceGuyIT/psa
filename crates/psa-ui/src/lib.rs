//! PSA UI - Shared UI components for PSA Platform
//!
//! This crate provides reusable Dioxus components used across all PSA apps:
//! - Layout components (shell, sidebar, header)
//! - Form components (inputs, selects, buttons)
//! - Display components (tables, cards, badges)
//! - Feedback components (alerts, toasts, modals)
//! - Navigation components (tabs, breadcrumbs)

pub mod components;
pub mod layout;
pub mod theme;

// Re-export commonly used components
pub use components::*;
pub use layout::*;
