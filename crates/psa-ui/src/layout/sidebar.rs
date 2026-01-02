//! Sidebar component

use dioxus::prelude::*;
use super::shell::NavItem;

/// Sidebar navigation
#[component]
pub fn Sidebar(
    nav_items: Vec<NavItem>,
    current_path: String,
    app_name: String,
) -> Element {
    rsx! {
        // Desktop sidebar
        div { class: "hidden lg:fixed lg:inset-y-0 lg:flex lg:w-64 lg:flex-col",
            div { class: "flex min-h-0 flex-1 flex-col bg-gray-800",
                // Logo area
                div { class: "flex h-16 flex-shrink-0 items-center px-4 bg-gray-900",
                    span { class: "text-xl font-bold text-white",
                        "{app_name}"
                    }
                }

                // Navigation
                nav { class: "mt-5 flex-1 space-y-1 px-2",
                    for item in nav_items.iter() {
                        {
                            let is_active = current_path == item.href || current_path.starts_with(&format!("{}/", item.href));
                            let base_class = if is_active {
                                "bg-gray-900 text-white"
                            } else {
                                "text-gray-300 hover:bg-gray-700 hover:text-white"
                            };

                            rsx! {
                                a {
                                    href: "{item.href}",
                                    class: "group flex items-center px-2 py-2 text-sm font-medium rounded-md {base_class}",
                                    // Icon placeholder (using emoji for now)
                                    span { class: "mr-3 text-lg", "{item.icon}" }
                                    "{item.label}"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Mobile sidebar (slide-over)
#[component]
pub fn MobileSidebar(
    nav_items: Vec<NavItem>,
    current_path: String,
    app_name: String,
    is_open: Signal<bool>,
) -> Element {
    if !*is_open.read() {
        return rsx! {};
    }

    rsx! {
        // Backdrop
        div {
            class: "fixed inset-0 z-40 bg-gray-600 bg-opacity-75 lg:hidden",
            onclick: move |_| is_open.set(false),
        }

        // Sidebar panel
        div { class: "fixed inset-y-0 left-0 z-50 w-64 bg-gray-800 lg:hidden",
            // Close button
            div { class: "absolute top-0 right-0 pt-2 pr-2",
                button {
                    class: "text-gray-300 hover:text-white",
                    onclick: move |_| is_open.set(false),
                    "✕"
                }
            }

            // Logo
            div { class: "flex h-16 items-center px-4 bg-gray-900",
                span { class: "text-xl font-bold text-white",
                    "{app_name}"
                }
            }

            // Navigation
            nav { class: "mt-5 space-y-1 px-2",
                for item in nav_items.iter() {
                    {
                        let is_active = current_path == item.href;
                        let base_class = if is_active {
                            "bg-gray-900 text-white"
                        } else {
                            "text-gray-300 hover:bg-gray-700 hover:text-white"
                        };

                        rsx! {
                            a {
                                href: "{item.href}",
                                class: "group flex items-center px-2 py-2 text-base font-medium rounded-md {base_class}",
                                span { class: "mr-3 text-lg", "{item.icon}" }
                                "{item.label}"
                            }
                        }
                    }
                }
            }
        }
    }
}
