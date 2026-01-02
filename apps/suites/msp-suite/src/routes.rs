//! MSP Suite application routes

use dioxus::prelude::*;

use psa_ui::{AppShell, NavItem, SimplePage};

/// Build navigation items based on enabled modules
fn nav_items() -> Vec<NavItem> {
    let mut items = vec![
        NavItem {
            label: "Dashboard",
            href: "/",
            icon: "📊",
        },
    ];

    #[cfg(feature = "ticketing")]
    items.push(NavItem {
        label: "Tickets",
        href: "/tickets",
        icon: "🎫",
    });

    #[cfg(feature = "time-tracking")]
    items.push(NavItem {
        label: "Time",
        href: "/time",
        icon: "⏱️",
    });

    #[cfg(feature = "projects")]
    items.push(NavItem {
        label: "Projects",
        href: "/projects",
        icon: "📋",
    });

    #[cfg(feature = "crm")]
    items.push(NavItem {
        label: "Contacts",
        href: "/contacts",
        icon: "👥",
    });

    #[cfg(feature = "billing")]
    items.push(NavItem {
        label: "Billing",
        href: "/billing",
        icon: "💰",
    });

    #[cfg(feature = "assets")]
    items.push(NavItem {
        label: "Assets",
        href: "/assets",
        icon: "🖥️",
    });

    #[cfg(feature = "knowledge-base")]
    items.push(NavItem {
        label: "Knowledge",
        href: "/knowledge",
        icon: "📚",
    });

    #[cfg(feature = "calendar")]
    items.push(NavItem {
        label: "Calendar",
        href: "/calendar",
        icon: "📅",
    });

    items.push(NavItem {
        label: "Settings",
        href: "/settings",
        icon: "⚙️",
    });

    items
}

/// Application routes
#[derive(Clone, Routable, Debug, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(Layout)]
        #[route("/")]
        Dashboard {},

        // Ticketing routes
        #[route("/tickets")]
        Tickets {},
        #[route("/tickets/new")]
        NewTicket {},
        #[route("/tickets/:id")]
        TicketDetail { id: String },

        // Time tracking routes
        #[route("/time")]
        TimeTracking {},
        #[route("/time/new")]
        NewTimeEntry {},

        // Project routes
        #[route("/projects")]
        Projects {},
        #[route("/projects/:id")]
        ProjectDetail { id: String },

        // CRM routes
        #[route("/contacts")]
        Contacts {},
        #[route("/contacts/:id")]
        ContactDetail { id: String },
        #[route("/companies")]
        Companies {},
        #[route("/companies/:id")]
        CompanyDetail { id: String },

        // Billing routes
        #[route("/billing")]
        Billing {},
        #[route("/invoices")]
        Invoices {},
        #[route("/invoices/:id")]
        InvoiceDetail { id: String },

        // Assets routes
        #[route("/assets")]
        Assets {},
        #[route("/assets/:id")]
        AssetDetail { id: String },

        // Knowledge base routes
        #[route("/knowledge")]
        KnowledgeBase {},
        #[route("/knowledge/:id")]
        ArticleDetail { id: String },

        // Calendar routes
        #[route("/calendar")]
        Calendar {},

        // Settings
        #[route("/settings")]
        Settings {},
    #[end_layout]

    #[route("/login")]
    Login {},

    #[route("/:..route")]
    NotFound { route: Vec<String> },
}

/// Main layout with sidebar
#[component]
fn Layout() -> Element {
    let current_route = use_route::<Route>();
    let current_path = match &current_route {
        Route::Dashboard {} => "/",
        Route::Tickets {} | Route::NewTicket {} | Route::TicketDetail { .. } => "/tickets",
        Route::TimeTracking {} | Route::NewTimeEntry {} => "/time",
        Route::Projects {} | Route::ProjectDetail { .. } => "/projects",
        Route::Contacts {} | Route::ContactDetail { .. } => "/contacts",
        Route::Companies {} | Route::CompanyDetail { .. } => "/companies",
        Route::Billing {} | Route::Invoices {} | Route::InvoiceDetail { .. } => "/billing",
        Route::Assets {} | Route::AssetDetail { .. } => "/assets",
        Route::KnowledgeBase {} | Route::ArticleDetail { .. } => "/knowledge",
        Route::Calendar {} => "/calendar",
        Route::Settings {} => "/settings",
        _ => "/",
    };

    rsx! {
        AppShell {
            nav_items: nav_items(),
            current_path: current_path.to_string(),
            app_name: "MSP Suite".to_string(),
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
                "MSP Suite Dashboard"
            }
            p { class: "text-gray-600 dark:text-gray-400",
                "Your complete professional services automation platform."
            }

            // Module quick access cards
            div { class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6",
                ModuleCard {
                    title: "Tickets",
                    description: "12 open tickets",
                    href: "/tickets",
                    icon: "🎫",
                }

                ModuleCard {
                    title: "Time",
                    description: "32h this week",
                    href: "/time",
                    icon: "⏱️",
                }

                ModuleCard {
                    title: "Projects",
                    description: "5 active projects",
                    href: "/projects",
                    icon: "📋",
                }

                ModuleCard {
                    title: "Billing",
                    description: "$12,450 pending",
                    href: "/billing",
                    icon: "💰",
                }
            }
        }
    }
}

/// Module quick access card
#[component]
fn ModuleCard(
    title: &'static str,
    description: &'static str,
    href: &'static str,
    icon: &'static str,
) -> Element {
    rsx! {
        a {
            href: "{href}",
            class: "bg-white dark:bg-gray-800 rounded-lg shadow p-6 hover:shadow-md transition-shadow",
            div { class: "flex items-center",
                span { class: "text-3xl mr-4", "{icon}" }
                div {
                    h3 { class: "text-lg font-medium text-gray-900 dark:text-white",
                        "{title}"
                    }
                    p { class: "text-sm text-gray-500 dark:text-gray-400",
                        "{description}"
                    }
                }
            }
        }
    }
}

// Placeholder components for each module
// These will be replaced with actual module views

#[component]
fn Tickets() -> Element {
    use psa_ticketing::views::TicketListPage;
    rsx! {
        TicketListPage {
            tickets: vec![],
            stats: None,
            base_url: "/tickets",
        }
    }
}

#[component]
fn NewTicket() -> Element {
    rsx! { Placeholder { title: "New Ticket" } }
}

#[component]
fn TicketDetail(id: String) -> Element {
    rsx! { Placeholder { title: "Ticket Detail" } }
}

#[component]
fn TimeTracking() -> Element {
    rsx! { Placeholder { title: "Time Tracking" } }
}

#[component]
fn NewTimeEntry() -> Element {
    rsx! { Placeholder { title: "New Time Entry" } }
}

#[component]
fn Projects() -> Element {
    rsx! { Placeholder { title: "Projects" } }
}

#[component]
fn ProjectDetail(id: String) -> Element {
    rsx! { Placeholder { title: "Project Detail" } }
}

#[component]
fn Contacts() -> Element {
    rsx! { Placeholder { title: "Contacts" } }
}

#[component]
fn ContactDetail(id: String) -> Element {
    rsx! { Placeholder { title: "Contact Detail" } }
}

#[component]
fn Companies() -> Element {
    rsx! { Placeholder { title: "Companies" } }
}

#[component]
fn CompanyDetail(id: String) -> Element {
    rsx! { Placeholder { title: "Company Detail" } }
}

#[component]
fn Billing() -> Element {
    rsx! { Placeholder { title: "Billing" } }
}

#[component]
fn Invoices() -> Element {
    rsx! { Placeholder { title: "Invoices" } }
}

#[component]
fn InvoiceDetail(id: String) -> Element {
    rsx! { Placeholder { title: "Invoice Detail" } }
}

#[component]
fn Assets() -> Element {
    rsx! { Placeholder { title: "Assets" } }
}

#[component]
fn AssetDetail(id: String) -> Element {
    rsx! { Placeholder { title: "Asset Detail" } }
}

#[component]
fn KnowledgeBase() -> Element {
    rsx! { Placeholder { title: "Knowledge Base" } }
}

#[component]
fn ArticleDetail(id: String) -> Element {
    rsx! { Placeholder { title: "Article" } }
}

#[component]
fn Calendar() -> Element {
    rsx! { Placeholder { title: "Calendar" } }
}

#[component]
fn Settings() -> Element {
    rsx! {
        div { class: "space-y-6",
            h1 { class: "text-2xl font-bold text-gray-900 dark:text-white",
                "Settings"
            }
            div { class: "bg-white dark:bg-gray-800 rounded-lg shadow p-6",
                p { class: "text-gray-500 dark:text-gray-400",
                    "Settings panel coming soon..."
                }
            }
        }
    }
}

#[component]
fn Login() -> Element {
    rsx! {
        SimplePage {
            div { class: "min-h-screen flex items-center justify-center",
                div { class: "max-w-md w-full",
                    h1 { class: "text-3xl font-bold text-center text-gray-900 dark:text-white mb-8",
                        "MSP Suite"
                    }
                    // TODO: Login form
                }
            }
        }
    }
}

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

/// Placeholder for unimplemented pages
#[component]
fn Placeholder(title: &'static str) -> Element {
    rsx! {
        div { class: "space-y-6",
            h1 { class: "text-2xl font-bold text-gray-900 dark:text-white",
                "{title}"
            }
            div { class: "bg-white dark:bg-gray-800 rounded-lg shadow p-12 text-center",
                p { class: "text-gray-500 dark:text-gray-400",
                    "This page is coming soon..."
                }
            }
        }
    }
}

/// Message for disabled modules
#[component]
fn ModuleDisabled(name: &'static str) -> Element {
    rsx! {
        div { class: "bg-yellow-50 dark:bg-yellow-900/20 rounded-lg p-8 text-center",
            h2 { class: "text-xl font-bold text-yellow-800 dark:text-yellow-200 mb-2",
                "{name} Module Disabled"
            }
            p { class: "text-yellow-700 dark:text-yellow-300",
                "This module is not enabled in your current build. Enable the '{name}' feature to use this functionality."
            }
        }
    }
}
