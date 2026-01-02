//! Resolver application routes

use dioxus::prelude::*;

use psa_ui::{AppShell, NavItem, SimplePage};
use psa_ticketing::views::{TicketListPage, TicketDetailPage, TicketForm};

/// Application routes
#[derive(Clone, Routable, Debug, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(Layout)]
        #[route("/")]
        Dashboard {},

        #[route("/tickets")]
        Tickets {},

        #[route("/tickets/new")]
        NewTicket {},

        #[route("/tickets/:id")]
        TicketDetail { id: String },

        #[route("/settings")]
        Settings {},
    #[end_layout]

    #[route("/login")]
    Login {},

    #[route("/:..route")]
    NotFound { route: Vec<String> },
}

/// Navigation items for the sidebar
fn nav_items() -> Vec<NavItem> {
    vec![
        NavItem {
            label: "Dashboard",
            href: "/",
            icon: "📊",
        },
        NavItem {
            label: "Tickets",
            href: "/tickets",
            icon: "🎫",
        },
        NavItem {
            label: "Settings",
            href: "/settings",
            icon: "⚙️",
        },
    ]
}

/// Main layout with sidebar
#[component]
fn Layout() -> Element {
    let current_route = use_route::<Route>();
    let current_path = match current_route {
        Route::Dashboard {} => "/",
        Route::Tickets {} => "/tickets",
        Route::NewTicket {} => "/tickets/new",
        Route::TicketDetail { .. } => "/tickets",
        Route::Settings {} => "/settings",
        _ => "/",
    };

    rsx! {
        AppShell {
            nav_items: nav_items(),
            current_path: current_path.to_string(),
            app_name: "Resolver".to_string(),
            user_name: Some("Demo User".to_string()),
            Outlet::<Route> {}
        }
    }
}

/// Dashboard page
#[component]
fn Dashboard() -> Element {
    rsx! {
        div { class: "space-y-6",
            h1 { class: "text-2xl font-bold text-gray-900 dark:text-white",
                "Welcome to Resolver"
            }
            p { class: "text-gray-600 dark:text-gray-400",
                "Your personal help desk and ticketing solution."
            }

            // Quick stats
            div { class: "grid grid-cols-1 md:grid-cols-3 gap-6",
                QuickStatCard {
                    title: "Open Tickets",
                    value: "12",
                    href: "/tickets?status=open",
                }
                QuickStatCard {
                    title: "Pending Response",
                    value: "3",
                    href: "/tickets?status=pending",
                }
                QuickStatCard {
                    title: "Resolved Today",
                    value: "5",
                    href: "/tickets?status=resolved",
                }
            }

            // Recent tickets
            div { class: "bg-white dark:bg-gray-800 rounded-lg shadow p-6",
                h2 { class: "text-lg font-medium text-gray-900 dark:text-white mb-4",
                    "Recent Tickets"
                }
                p { class: "text-gray-500 dark:text-gray-400 text-sm",
                    "Your most recent tickets will appear here."
                }
            }
        }
    }
}

/// Quick stat card
#[component]
fn QuickStatCard(title: &'static str, value: &'static str, href: &'static str) -> Element {
    rsx! {
        a {
            href: "{href}",
            class: "bg-white dark:bg-gray-800 rounded-lg shadow p-6 hover:shadow-md transition-shadow",
            p { class: "text-sm font-medium text-gray-500 dark:text-gray-400",
                "{title}"
            }
            p { class: "text-3xl font-bold text-gray-900 dark:text-white mt-2",
                "{value}"
            }
        }
    }
}

/// Tickets list page
#[component]
fn Tickets() -> Element {
    // TODO: Fetch tickets from API
    let tickets = vec![];
    let stats = None;

    rsx! {
        TicketListPage {
            tickets: tickets,
            stats: stats,
            base_url: "/tickets",
        }
    }
}

/// New ticket page
#[component]
fn NewTicket() -> Element {
    let nav = use_navigator();

    rsx! {
        TicketForm {
            title: "Create New Ticket",
            on_submit: move |request: psa_ticketing::CreateTicketRequest| {
                // TODO: Submit to API
                tracing::info!("Creating ticket: {:?}", request.subject);
                nav.push(Route::Tickets {});
            },
            on_cancel: move |_: ()| {
                nav.push(Route::Tickets {});
            },
        }
    }
}

/// Ticket detail page
#[component]
fn TicketDetail(id: String) -> Element {
    // TODO: Fetch ticket from API
    rsx! {
        div { class: "text-center py-12",
            p { class: "text-gray-500 dark:text-gray-400",
                "Loading ticket {id}..."
            }
        }
    }
}

/// Settings page
#[component]
fn Settings() -> Element {
    rsx! {
        div { class: "space-y-6",
            h1 { class: "text-2xl font-bold text-gray-900 dark:text-white",
                "Settings"
            }

            div { class: "bg-white dark:bg-gray-800 rounded-lg shadow divide-y divide-gray-200 dark:divide-gray-700",
                SettingsSection {
                    title: "Profile",
                    description: "Manage your account settings",
                }
                SettingsSection {
                    title: "Notifications",
                    description: "Configure how you receive notifications",
                }
                SettingsSection {
                    title: "Email Integration",
                    description: "Set up email-to-ticket conversion",
                }
                SettingsSection {
                    title: "SLA Policies",
                    description: "Define service level agreements",
                }
            }
        }
    }
}

#[component]
fn SettingsSection(title: &'static str, description: &'static str) -> Element {
    rsx! {
        div { class: "p-6 flex items-center justify-between",
            div {
                h3 { class: "text-sm font-medium text-gray-900 dark:text-white",
                    "{title}"
                }
                p { class: "text-sm text-gray-500 dark:text-gray-400",
                    "{description}"
                }
            }
            button { class: "text-primary-600 hover:text-primary-800 text-sm font-medium",
                "Configure"
            }
        }
    }
}

/// Login page
#[component]
fn Login() -> Element {
    rsx! {
        SimplePage {
            div { class: "min-h-screen flex items-center justify-center",
                div { class: "max-w-md w-full space-y-8",
                    div { class: "text-center",
                        h1 { class: "text-3xl font-bold text-gray-900 dark:text-white",
                            "Resolver"
                        }
                        p { class: "mt-2 text-gray-600 dark:text-gray-400",
                            "Sign in to your account"
                        }
                    }
                    // TODO: Login form
                }
            }
        }
    }
}

/// 404 page
#[component]
fn NotFound(route: Vec<String>) -> Element {
    let path = route.join("/");
    rsx! {
        SimplePage {
            div { class: "text-center py-12",
                h1 { class: "text-4xl font-bold text-gray-900 dark:text-white mb-4",
                    "404"
                }
                p { class: "text-gray-600 dark:text-gray-400 mb-6",
                    "Page not found: /{path}"
                }
                a {
                    href: "/",
                    class: "text-primary-600 hover:text-primary-800 font-medium",
                    "← Back to Dashboard"
                }
            }
        }
    }
}
