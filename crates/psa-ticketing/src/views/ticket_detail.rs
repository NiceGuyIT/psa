//! Ticket detail view

use dioxus::prelude::*;

use psa_ui::{
    Badge, BadgeVariant, Button, ButtonVariant,
    Card, PageHeader, Textarea,
};

use crate::models::{Ticket, TicketComment, TicketPriority, TicketStatus, SlaStatus};

/// Ticket detail page
#[component]
pub fn TicketDetailPage(
    /// The ticket to display
    ticket: Ticket,
    /// Comments on the ticket
    comments: Vec<TicketComment>,
    /// Whether user can edit
    #[props(default = true)]
    can_edit: bool,
    /// Base URL for navigation
    #[props(default = "/tickets")]
    base_url: &'static str,
) -> Element {
    let sla_status = ticket.sla_status();

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

    let sla_variant = match sla_status {
        SlaStatus::OnTrack | SlaStatus::Met => BadgeVariant::Green,
        SlaStatus::Warning => BadgeVariant::Yellow,
        SlaStatus::Breached => BadgeVariant::Red,
        SlaStatus::None => BadgeVariant::Gray,
    };

    let created_at_str = ticket.created_at.format("%b %d, %Y %H:%M").to_string();
    let due_date_str = ticket.due_date.map(|d| d.format("%b %d, %Y %H:%M").to_string());

    let actions = if can_edit {
        rsx! {
            Button {
                variant: ButtonVariant::Secondary,
                onclick: move |_| {},
                "Edit"
            }
            if ticket.status.is_open() {
                Button {
                    onclick: move |_| {},
                    "Resolve"
                }
            }
        }
    } else {
        rsx! {}
    };

    rsx! {
        PageHeader {
            title: format!("#{} - {}", ticket.ticket_number, ticket.subject),
            actions: actions,
        }

        div { class: "grid grid-cols-1 lg:grid-cols-3 gap-6",
            // Main content
            div { class: "lg:col-span-2 space-y-6",
                // Description
                Card {
                    title: "Description".to_string(),
                    div { class: "prose dark:prose-invert max-w-none",
                        p { "{ticket.description}" }
                    }
                }

                // Comments
                Card {
                    title: format!("Comments ({})", comments.len()),
                    div { class: "space-y-4",
                        if comments.is_empty() {
                            p { class: "text-gray-500 dark:text-gray-400 text-sm",
                                "No comments yet"
                            }
                        } else {
                            for comment in comments.iter() {
                                CommentItem { comment: comment.clone() }
                            }
                        }

                        // Add comment form
                        if can_edit {
                            AddCommentForm {}
                        }
                    }
                }
            }

            // Sidebar
            div { class: "space-y-6",
                Card {
                    title: "Details".to_string(),
                    dl { class: "space-y-4",
                        DetailRow {
                            label: "Status",
                            Badge {
                                text: ticket.status.display_name().to_string(),
                                variant: status_variant,
                            }
                        }
                        DetailRow {
                            label: "Priority",
                            Badge {
                                text: ticket.priority.display_name().to_string(),
                                variant: priority_variant,
                            }
                        }
                        DetailRow {
                            label: "SLA",
                            Badge {
                                text: sla_status.as_str().to_string(),
                                variant: sla_variant,
                            }
                        }
                        DetailRow {
                            label: "Created",
                            span { "{created_at_str}" }
                        }
                        if let Some(due_str) = &due_date_str {
                            DetailRow {
                                label: "Due Date",
                                span { "{due_str}" }
                            }
                        }
                    }
                }

                if !ticket.tags.is_empty() {
                    Card {
                        title: "Tags".to_string(),
                        div { class: "flex flex-wrap gap-2",
                            for tag in ticket.tags.iter() {
                                Badge { text: tag.clone() }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Detail row component
#[component]
fn DetailRow(label: &'static str, children: Element) -> Element {
    rsx! {
        div { class: "flex justify-between",
            dt { class: "text-sm font-medium text-gray-500 dark:text-gray-400",
                "{label}"
            }
            dd { class: "text-sm text-gray-900 dark:text-white",
                {children}
            }
        }
    }
}

/// Comment item
#[component]
fn CommentItem(comment: TicketComment) -> Element {
    let bg_class = if comment.is_internal {
        "bg-yellow-50 dark:bg-yellow-900/20 border-yellow-200 dark:border-yellow-800"
    } else {
        "bg-gray-50 dark:bg-gray-800 border-gray-200 dark:border-gray-700"
    };

    let created_str = comment.created_at.format("%b %d, %Y %H:%M").to_string();

    rsx! {
        div { class: "border rounded-lg p-4 {bg_class}",
            div { class: "flex items-center justify-between mb-2",
                span { class: "text-sm font-medium text-gray-900 dark:text-white",
                    "User" // TODO: Get author name
                }
                span { class: "text-xs text-gray-500 dark:text-gray-400",
                    "{created_str}"
                }
            }
            if comment.is_internal {
                Badge {
                    text: "Internal".to_string(),
                    variant: BadgeVariant::Yellow,
                }
            }
            p { class: "text-sm text-gray-700 dark:text-gray-300 mt-2",
                "{comment.content}"
            }
        }
    }
}

/// Add comment form
#[component]
fn AddCommentForm() -> Element {
    let mut content = use_signal(|| String::new());
    let mut is_internal = use_signal(|| false);

    rsx! {
        div { class: "border-t border-gray-200 dark:border-gray-700 pt-4 mt-4",
            Textarea {
                label: "Add a comment".to_string(),
                name: "comment".to_string(),
                value: content,
                rows: 3,
                placeholder: Some("Write your comment...".to_string()),
            }
            div { class: "flex items-center justify-between mt-2",
                label { class: "flex items-center gap-2 text-sm text-gray-600 dark:text-gray-400",
                    input {
                        r#type: "checkbox",
                        class: "rounded border-gray-300",
                        checked: *is_internal.read(),
                        onchange: move |evt| is_internal.set(evt.checked()),
                    }
                    "Internal note (not visible to requester)"
                }
                Button {
                    onclick: move |_| {
                        // TODO: Submit comment
                        content.set(String::new());
                    },
                    "Add Comment"
                }
            }
        }
    }
}
