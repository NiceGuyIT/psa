//! Ticket list view

use dioxus::prelude::*;

use psa_ui::{
    Badge, BadgeVariant, Button, ButtonLink, ButtonVariant,
    Card, PageHeader, SimpleTable, TableCell, TableRow,
};

use crate::models::{Ticket, TicketPriority, TicketStatus, TicketStats};

/// Ticket list page
#[component]
pub fn TicketListPage(
    /// Tickets to display
    tickets: Vec<Ticket>,
    /// Ticket statistics
    stats: Option<TicketStats>,
    /// Loading state
    #[props(default = false)]
    loading: bool,
    /// Base URL for ticket links (e.g., "/tickets" or "/")
    #[props(default = "/tickets")]
    base_url: &'static str,
) -> Element {
    rsx! {
        PageHeader {
            title: "Tickets".to_string(),
            subtitle: stats.as_ref().map(|s| format!("{} open, {} unassigned", s.open, s.unassigned)),
            actions: rsx! {
                ButtonLink {
                    href: format!("{}/new", base_url),
                    "New Ticket"
                }
            },
        }

        // Stats cards
        if let Some(stats) = stats {
            div { class: "grid grid-cols-1 gap-4 sm:grid-cols-2 lg:grid-cols-4 mb-6",
                StatCard { label: "Open", value: stats.open.to_string(), color: "blue" }
                StatCard { label: "Unassigned", value: stats.unassigned.to_string(), color: "yellow" }
                StatCard { label: "Overdue", value: stats.overdue.to_string(), color: "red" }
                StatCard { label: "Resolved Today", value: stats.resolved_today.to_string(), color: "green" }
            }
        }

        // Ticket table
        Card {
            SimpleTable {
                headers: vec![
                    "#".to_string(),
                    "Subject".to_string(),
                    "Status".to_string(),
                    "Priority".to_string(),
                    "Created".to_string(),
                ],
                if loading {
                    TableRow {
                        TableCell {
                            div { class: "animate-pulse h-4 bg-gray-200 rounded" }
                        }
                    }
                } else if tickets.is_empty() {
                    TableRow {
                        td {
                            colspan: "5",
                            class: "px-3 py-8 text-center text-sm text-gray-500",
                            "No tickets found"
                        }
                    }
                } else {
                    for ticket in tickets.iter() {
                        TicketRow {
                            ticket: ticket.clone(),
                            base_url: base_url,
                        }
                    }
                }
            }
        }
    }
}

/// Single ticket row
#[component]
fn TicketRow(ticket: Ticket, base_url: &'static str) -> Element {
    let status_variant = match ticket.status {
        TicketStatus::New => BadgeVariant::Blue,
        TicketStatus::Open | TicketStatus::InProgress => BadgeVariant::Green,
        TicketStatus::Pending => BadgeVariant::Yellow,
        TicketStatus::Resolved | TicketStatus::Closed => BadgeVariant::Gray,
    };

    let priority_variant = match ticket.priority {
        TicketPriority::Low => BadgeVariant::Gray,
        TicketPriority::Medium => BadgeVariant::Blue,
        TicketPriority::High => BadgeVariant::Yellow,
        TicketPriority::Critical => BadgeVariant::Red,
    };

    let created = ticket.created_at.format("%b %d, %Y").to_string();

    rsx! {
        TableRow {
            TableCell {
                a {
                    href: "{base_url}/{ticket.id}",
                    class: "text-primary-600 hover:text-primary-800 font-medium",
                    "#{ticket.ticket_number}"
                }
            }
            TableCell {
                a {
                    href: "{base_url}/{ticket.id}",
                    class: "hover:text-primary-600",
                    "{ticket.subject}"
                }
            }
            TableCell {
                Badge {
                    text: ticket.status.display_name().to_string(),
                    variant: status_variant,
                }
            }
            TableCell {
                Badge {
                    text: ticket.priority.display_name().to_string(),
                    variant: priority_variant,
                }
            }
            TableCell {
                "{created}"
            }
        }
    }
}

/// Statistics card
#[component]
fn StatCard(
    label: &'static str,
    value: String,
    color: &'static str,
) -> Element {
    let bg_class = match color {
        "blue" => "bg-blue-500",
        "yellow" => "bg-yellow-500",
        "red" => "bg-red-500",
        "green" => "bg-green-500",
        _ => "bg-gray-500",
    };

    rsx! {
        div { class: "bg-white dark:bg-gray-800 rounded-lg shadow p-4",
            div { class: "flex items-center",
                div { class: "flex-shrink-0 p-3 rounded-full {bg_class}",
                    span { class: "text-white text-xl font-bold", "{value}" }
                }
                div { class: "ml-4",
                    p { class: "text-sm font-medium text-gray-500 dark:text-gray-400",
                        "{label}"
                    }
                }
            }
        }
    }
}
