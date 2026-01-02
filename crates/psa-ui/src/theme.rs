//! Theme configuration for PSA UI
//!
//! Provides theming support for standalone apps and suites.

use serde::{Deserialize, Serialize};

/// Theme configuration for an application
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Theme {
    /// Application name
    pub app_name: String,
    /// Primary brand color (hex)
    pub primary_color: String,
    /// Secondary brand color (hex)
    pub secondary_color: String,
    /// Logo URL
    pub logo_url: Option<String>,
    /// Favicon URL
    pub favicon_url: Option<String>,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            app_name: "PSA Platform".to_string(),
            primary_color: "#3b82f6".to_string(),
            secondary_color: "#1e40af".to_string(),
            logo_url: None,
            favicon_url: None,
        }
    }
}

/// Predefined themes for standalone apps
pub mod standalone {
    use super::Theme;

    pub fn resolver() -> Theme {
        Theme {
            app_name: "Resolver".to_string(),
            primary_color: "#ef4444".to_string(),  // Red
            secondary_color: "#b91c1c".to_string(),
            ..Default::default()
        }
    }

    pub fn tempo() -> Theme {
        Theme {
            app_name: "Tempo".to_string(),
            primary_color: "#22c55e".to_string(),  // Green
            secondary_color: "#15803d".to_string(),
            ..Default::default()
        }
    }

    pub fn milestones() -> Theme {
        Theme {
            app_name: "Milestones".to_string(),
            primary_color: "#3b82f6".to_string(),  // Blue
            secondary_color: "#1d4ed8".to_string(),
            ..Default::default()
        }
    }

    pub fn rapport() -> Theme {
        Theme {
            app_name: "Rapport".to_string(),
            primary_color: "#a855f7".to_string(),  // Purple
            secondary_color: "#7e22ce".to_string(),
            ..Default::default()
        }
    }

    pub fn ledger() -> Theme {
        Theme {
            app_name: "Ledger".to_string(),
            primary_color: "#f59e0b".to_string(),  // Amber
            secondary_color: "#d97706".to_string(),
            ..Default::default()
        }
    }

    pub fn registry() -> Theme {
        Theme {
            app_name: "Registry".to_string(),
            primary_color: "#06b6d4".to_string(),  // Cyan
            secondary_color: "#0891b2".to_string(),
            ..Default::default()
        }
    }

    pub fn codex() -> Theme {
        Theme {
            app_name: "Codex".to_string(),
            primary_color: "#ec4899".to_string(),  // Pink
            secondary_color: "#be185d".to_string(),
            ..Default::default()
        }
    }

    pub fn cadence() -> Theme {
        Theme {
            app_name: "Cadence".to_string(),
            primary_color: "#8b5cf6".to_string(),  // Violet
            secondary_color: "#6d28d9".to_string(),
            ..Default::default()
        }
    }
}

/// Predefined themes for suite apps
pub mod suites {
    use super::Theme;

    pub fn msp_suite() -> Theme {
        Theme {
            app_name: "MSP Suite".to_string(),
            primary_color: "#2563eb".to_string(),  // Blue
            secondary_color: "#1e40af".to_string(),
            ..Default::default()
        }
    }

    pub fn vet_suite() -> Theme {
        Theme {
            app_name: "Vet Suite".to_string(),
            primary_color: "#059669".to_string(),  // Emerald
            secondary_color: "#047857".to_string(),
            ..Default::default()
        }
    }

    pub fn legal_suite() -> Theme {
        Theme {
            app_name: "Legal Suite".to_string(),
            primary_color: "#7c3aed".to_string(),  // Violet
            secondary_color: "#5b21b6".to_string(),
            ..Default::default()
        }
    }
}
