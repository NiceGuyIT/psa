//! Application shell layout

use dioxus::prelude::*;

use super::{Header, Sidebar};

/// Navigation item for sidebar
#[derive(Clone, PartialEq)]
pub struct NavItem {
    pub label: &'static str,
    pub href: &'static str,
    pub icon: &'static str,
}

/// Application shell with sidebar and main content area
#[component]
pub fn AppShell(
    /// Navigation items for sidebar
    nav_items: Vec<NavItem>,
    /// Current route path for active highlighting
    current_path: String,
    /// Application name for header
    app_name: String,
    /// User name for header
    user_name: Option<String>,
    /// Main content
    children: Element,
) -> Element {
    rsx! {
        div { class: "min-h-screen bg-gray-50 dark:bg-gray-900",
            // Sidebar
            Sidebar {
                nav_items: nav_items,
                current_path: current_path.clone(),
                app_name: app_name.clone(),
            }

            // Main content area
            div { class: "lg:pl-64",
                // Header
                Header {
                    app_name: app_name,
                    user_name: user_name,
                }

                // Page content
                main { class: "py-6",
                    div { class: "mx-auto max-w-7xl px-4 sm:px-6 lg:px-8",
                        {children}
                    }
                }
            }
        }
    }
}

/// Simple page layout without sidebar
#[component]
pub fn SimplePage(
    /// Page title
    title: Option<String>,
    /// Main content
    children: Element,
) -> Element {
    rsx! {
        div { class: "min-h-screen bg-gray-50 dark:bg-gray-900",
            div { class: "mx-auto max-w-7xl px-4 py-8 sm:px-6 lg:px-8",
                if let Some(title) = title {
                    h1 { class: "text-2xl font-bold text-gray-900 dark:text-white mb-6",
                        "{title}"
                    }
                }
                {children}
            }
        }
    }
}

/// Card container
#[component]
pub fn Card(
    /// Optional title
    title: Option<String>,
    /// Additional CSS classes
    class: Option<String>,
    /// Card content
    children: Element,
) -> Element {
    let class = format!(
        "bg-white dark:bg-gray-800 shadow rounded-lg {}",
        class.unwrap_or_default()
    );

    rsx! {
        div { class: "{class}",
            if let Some(title) = title {
                div { class: "border-b border-gray-200 dark:border-gray-700 px-4 py-3",
                    h3 { class: "text-lg font-medium text-gray-900 dark:text-white",
                        "{title}"
                    }
                }
            }
            div { class: "p-4",
                {children}
            }
        }
    }
}
