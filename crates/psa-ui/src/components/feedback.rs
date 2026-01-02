//! Feedback components (alerts, toasts, modals, badges)

use dioxus::prelude::*;

/// Alert variant
#[derive(Clone, Copy, Default, PartialEq)]
pub enum AlertVariant {
    #[default]
    Info,
    Success,
    Warning,
    Error,
}

impl AlertVariant {
    fn classes(&self) -> (&'static str, &'static str) {
        match self {
            AlertVariant::Info => ("bg-blue-50 dark:bg-blue-900/20", "text-blue-800 dark:text-blue-200"),
            AlertVariant::Success => ("bg-green-50 dark:bg-green-900/20", "text-green-800 dark:text-green-200"),
            AlertVariant::Warning => ("bg-yellow-50 dark:bg-yellow-900/20", "text-yellow-800 dark:text-yellow-200"),
            AlertVariant::Error => ("bg-red-50 dark:bg-red-900/20", "text-red-800 dark:text-red-200"),
        }
    }

    fn icon(&self) -> &'static str {
        match self {
            AlertVariant::Info => "ℹ️",
            AlertVariant::Success => "✓",
            AlertVariant::Warning => "⚠️",
            AlertVariant::Error => "✕",
        }
    }
}

/// Alert component
#[component]
pub fn Alert(
    /// Alert message
    message: String,
    /// Alert variant
    #[props(default)]
    variant: AlertVariant,
    /// Optional title
    title: Option<String>,
    /// Dismissible
    #[props(default = false)]
    dismissible: bool,
    /// On dismiss callback
    on_dismiss: Option<EventHandler<()>>,
) -> Element {
    let (bg_class, text_class) = variant.classes();
    let icon = variant.icon();

    rsx! {
        div { class: "rounded-md p-4 {bg_class}",
            div { class: "flex",
                div { class: "flex-shrink-0 text-lg",
                    "{icon}"
                }
                div { class: "ml-3",
                    if let Some(title) = title {
                        h3 { class: "text-sm font-medium {text_class}",
                            "{title}"
                        }
                    }
                    div { class: "text-sm {text_class}",
                        "{message}"
                    }
                }
                if dismissible {
                    div { class: "ml-auto pl-3",
                        button {
                            class: "-mx-1.5 -my-1.5 rounded-lg p-1.5 inline-flex {text_class} hover:bg-black/10",
                            onclick: move |_| {
                                if let Some(handler) = &on_dismiss {
                                    handler.call(());
                                }
                            },
                            "✕"
                        }
                    }
                }
            }
        }
    }
}

/// Badge variant
#[derive(Clone, Copy, Default, PartialEq)]
pub enum BadgeVariant {
    #[default]
    Gray,
    Red,
    Yellow,
    Green,
    Blue,
    Purple,
}

impl BadgeVariant {
    fn classes(&self) -> &'static str {
        match self {
            BadgeVariant::Gray => "bg-gray-100 text-gray-800 dark:bg-gray-700 dark:text-gray-200",
            BadgeVariant::Red => "bg-red-100 text-red-800 dark:bg-red-900 dark:text-red-200",
            BadgeVariant::Yellow => "bg-yellow-100 text-yellow-800 dark:bg-yellow-900 dark:text-yellow-200",
            BadgeVariant::Green => "bg-green-100 text-green-800 dark:bg-green-900 dark:text-green-200",
            BadgeVariant::Blue => "bg-blue-100 text-blue-800 dark:bg-blue-900 dark:text-blue-200",
            BadgeVariant::Purple => "bg-purple-100 text-purple-800 dark:bg-purple-900 dark:text-purple-200",
        }
    }
}

/// Badge component
#[component]
pub fn Badge(
    /// Badge text
    text: String,
    /// Badge variant
    #[props(default)]
    variant: BadgeVariant,
) -> Element {
    let variant_class = variant.classes();

    rsx! {
        span { class: "inline-flex items-center rounded-full px-2.5 py-0.5 text-xs font-medium {variant_class}",
            "{text}"
        }
    }
}

/// Modal component
#[component]
pub fn Modal(
    /// Modal title
    title: String,
    /// Whether modal is open
    is_open: Signal<bool>,
    /// Modal content
    children: Element,
    /// Footer actions
    footer: Option<Element>,
) -> Element {
    if !*is_open.read() {
        return rsx! {};
    }

    rsx! {
        // Backdrop
        div {
            class: "fixed inset-0 z-50 overflow-y-auto",
            div {
                class: "fixed inset-0 bg-gray-500 bg-opacity-75 transition-opacity",
                onclick: move |_| is_open.set(false),
            }

            div { class: "flex min-h-full items-end justify-center p-4 text-center sm:items-center sm:p-0",
                // Modal panel
                div { class: "relative transform overflow-hidden rounded-lg bg-white dark:bg-gray-800 text-left shadow-xl transition-all sm:my-8 sm:w-full sm:max-w-lg",
                    // Header
                    div { class: "flex items-center justify-between px-4 py-3 border-b border-gray-200 dark:border-gray-700",
                        h3 { class: "text-lg font-medium text-gray-900 dark:text-white",
                            "{title}"
                        }
                        button {
                            class: "text-gray-400 hover:text-gray-500 dark:hover:text-gray-300",
                            onclick: move |_| is_open.set(false),
                            "✕"
                        }
                    }

                    // Content
                    div { class: "px-4 py-4",
                        {children}
                    }

                    // Footer
                    if let Some(footer) = footer {
                        div { class: "px-4 py-3 bg-gray-50 dark:bg-gray-900 flex justify-end gap-3",
                            {footer}
                        }
                    }
                }
            }
        }
    }
}

/// Empty state component
#[component]
pub fn EmptyState(
    /// Icon (emoji or text)
    icon: String,
    /// Title
    title: String,
    /// Description
    description: String,
    /// Action element
    action: Option<Element>,
) -> Element {
    rsx! {
        div { class: "text-center py-12",
            div { class: "text-4xl mb-4", "{icon}" }
            h3 { class: "text-lg font-medium text-gray-900 dark:text-white mb-2",
                "{title}"
            }
            p { class: "text-sm text-gray-500 dark:text-gray-400 mb-6 max-w-sm mx-auto",
                "{description}"
            }
            if let Some(action) = action {
                {action}
            }
        }
    }
}

/// Loading spinner
#[component]
pub fn Spinner(
    /// Size (sm, md, lg)
    #[props(default = "md")]
    size: &'static str,
) -> Element {
    let size_class = match size {
        "sm" => "h-4 w-4",
        "lg" => "h-8 w-8",
        _ => "h-6 w-6",
    };

    rsx! {
        div { class: "animate-spin rounded-full border-2 border-gray-300 border-t-primary-600 {size_class}" }
    }
}
