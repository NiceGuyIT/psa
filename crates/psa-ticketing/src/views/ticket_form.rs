//! Ticket create/edit form

use dioxus::prelude::*;

use psa_ui::{
    Button, ButtonVariant, Card, Input, Select, SelectOption, Textarea,
};

use crate::models::{TicketPriority, TicketStatus, CreateTicketRequest};

/// Ticket form for create/edit
#[component]
pub fn TicketForm(
    /// Title for the form
    #[props(default = "New Ticket")]
    title: &'static str,
    /// Existing ticket data for editing
    initial_subject: Option<String>,
    initial_description: Option<String>,
    initial_priority: Option<TicketPriority>,
    initial_status: Option<TicketStatus>,
    /// Whether this is an edit form
    #[props(default = false)]
    is_edit: bool,
    /// Submit handler
    on_submit: EventHandler<CreateTicketRequest>,
    /// Cancel handler
    on_cancel: EventHandler<()>,
) -> Element {
    let mut subject = use_signal(|| initial_subject.unwrap_or_default());
    let mut description = use_signal(|| initial_description.unwrap_or_default());
    let mut priority = use_signal(|| {
        initial_priority.unwrap_or_default().as_str().to_string()
    });
    let mut status = use_signal(|| {
        initial_status.map(|s| s.as_str().to_string()).unwrap_or_default()
    });

    let mut errors = use_signal(|| Vec::<String>::new());
    let mut submitting = use_signal(|| false);

    let priority_options: Vec<SelectOption> = TicketPriority::all()
        .into_iter()
        .map(|p| SelectOption {
            value: p.as_str().to_string(),
            label: p.display_name().to_string(),
        })
        .collect();

    let status_options: Vec<SelectOption> = TicketStatus::all()
        .into_iter()
        .map(|s| SelectOption {
            value: s.as_str().to_string(),
            label: s.display_name().to_string(),
        })
        .collect();

    rsx! {
        Card { title: title.to_string(),
            form {
                onsubmit: move |evt| {
                    evt.prevent_default();

                    let mut validation_errors = Vec::new();

                    if subject.read().trim().is_empty() {
                        validation_errors.push("Subject is required".to_string());
                    }
                    if description.read().trim().is_empty() {
                        validation_errors.push("Description is required".to_string());
                    }

                    if !validation_errors.is_empty() {
                        errors.set(validation_errors);
                        return;
                    }

                    errors.set(Vec::new());
                    submitting.set(true);

                    let request = CreateTicketRequest {
                        subject: subject.read().clone(),
                        description: description.read().clone(),
                        priority: priority.read().parse().ok(),
                        assignee_id: None,
                        queue_id: None,
                        tags: None,
                    };

                    on_submit.call(request);
                },

                // Error display
                if !errors.read().is_empty() {
                    div { class: "mb-4 p-4 bg-red-50 dark:bg-red-900/20 rounded-md",
                        ul { class: "list-disc list-inside text-sm text-red-600 dark:text-red-400",
                            for error in errors.read().iter() {
                                li { "{error}" }
                            }
                        }
                    }
                }

                Input {
                    label: "Subject".to_string(),
                    name: "subject".to_string(),
                    value: subject,
                    required: true,
                    placeholder: Some("Brief description of the issue".to_string()),
                }

                Textarea {
                    label: "Description".to_string(),
                    name: "description".to_string(),
                    value: description,
                    required: true,
                    rows: 6,
                    placeholder: Some("Provide detailed information about the issue...".to_string()),
                }

                div { class: "grid grid-cols-1 md:grid-cols-2 gap-4",
                    Select {
                        label: "Priority".to_string(),
                        name: "priority".to_string(),
                        value: priority,
                        options: priority_options,
                    }

                    if is_edit {
                        Select {
                            label: "Status".to_string(),
                            name: "status".to_string(),
                            value: status,
                            options: status_options,
                        }
                    }
                }

                // Actions
                div { class: "flex justify-end gap-3 mt-6",
                    Button {
                        variant: ButtonVariant::Secondary,
                        r#type: "button",
                        onclick: move |_| on_cancel.call(()),
                        "Cancel"
                    }
                    Button {
                        r#type: "submit",
                        loading: *submitting.read(),
                        disabled: *submitting.read(),
                        if is_edit { "Save Changes" } else { "Create Ticket" }
                    }
                }
            }
        }
    }
}
