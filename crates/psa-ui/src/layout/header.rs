//! Header component

use dioxus::prelude::*;

/// Application header
#[component]
pub fn Header(
    app_name: String,
    user_name: Option<String>,
) -> Element {
    rsx! {
        header { class: "bg-white dark:bg-gray-800 shadow",
            div { class: "mx-auto max-w-7xl px-4 sm:px-6 lg:px-8",
                div { class: "flex h-16 justify-between items-center",
                    // Mobile menu button
                    div { class: "flex items-center lg:hidden",
                        button {
                            class: "text-gray-500 hover:text-gray-900 dark:text-gray-400 dark:hover:text-white",
                            r#type: "button",
                            "☰"
                        }
                    }

                    // Search (placeholder)
                    div { class: "flex-1 px-4 flex justify-center lg:justify-start",
                        div { class: "w-full max-w-lg",
                            label { class: "sr-only", "Search" }
                            div { class: "relative",
                                input {
                                    r#type: "search",
                                    class: "block w-full rounded-md border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-700 py-2 pl-10 pr-3 text-sm placeholder-gray-500 dark:placeholder-gray-400 focus:border-primary-500 focus:ring-primary-500",
                                    placeholder: "Search...",
                                }
                            }
                        }
                    }

                    // User menu
                    div { class: "flex items-center gap-4",
                        // Notifications
                        button {
                            class: "text-gray-500 hover:text-gray-900 dark:text-gray-400 dark:hover:text-white",
                            "🔔"
                        }

                        // User info
                        if let Some(name) = user_name {
                            div { class: "flex items-center gap-2",
                                div { class: "h-8 w-8 rounded-full bg-gray-300 dark:bg-gray-600 flex items-center justify-center text-sm font-medium text-gray-700 dark:text-gray-200",
                                    {name.chars().next().unwrap_or('U').to_string()}
                                }
                                span { class: "hidden md:block text-sm font-medium text-gray-700 dark:text-gray-200",
                                    "{name}"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Page header with title and actions
#[component]
pub fn PageHeader(
    title: String,
    subtitle: Option<String>,
    actions: Option<Element>,
) -> Element {
    rsx! {
        div { class: "md:flex md:items-center md:justify-between mb-6",
            div { class: "min-w-0 flex-1",
                h2 { class: "text-2xl font-bold leading-7 text-gray-900 dark:text-white sm:truncate sm:text-3xl sm:tracking-tight",
                    "{title}"
                }
                if let Some(subtitle) = subtitle {
                    p { class: "mt-1 text-sm text-gray-500 dark:text-gray-400",
                        "{subtitle}"
                    }
                }
            }
            if let Some(actions) = actions {
                div { class: "mt-4 flex md:ml-4 md:mt-0 gap-3",
                    {actions}
                }
            }
        }
    }
}
