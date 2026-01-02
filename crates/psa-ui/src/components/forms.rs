//! Form components

use dioxus::prelude::*;

/// Text input field
#[component]
pub fn Input(
    /// Input label
    label: String,
    /// Input name/id
    name: String,
    /// Current value
    value: Signal<String>,
    /// Input type
    #[props(default = "text")]
    r#type: &'static str,
    /// Placeholder text
    placeholder: Option<String>,
    /// Required field
    #[props(default = false)]
    required: bool,
    /// Disabled state
    #[props(default = false)]
    disabled: bool,
    /// Error message
    error: Option<String>,
    /// Help text
    help: Option<String>,
) -> Element {
    let has_error = error.is_some();
    let input_class = if has_error {
        "block w-full rounded-md border-red-300 text-red-900 placeholder-red-300 focus:border-red-500 focus:ring-red-500 sm:text-sm"
    } else {
        "block w-full rounded-md border-gray-300 dark:border-gray-600 dark:bg-gray-700 dark:text-white shadow-sm focus:border-primary-500 focus:ring-primary-500 sm:text-sm"
    };

    rsx! {
        div { class: "mb-4",
            label {
                r#for: "{name}",
                class: "block text-sm font-medium text-gray-700 dark:text-gray-200 mb-1",
                "{label}"
                if required {
                    span { class: "text-red-500 ml-1", "*" }
                }
            }
            input {
                r#type: r#type,
                name: "{name}",
                id: "{name}",
                class: "{input_class}",
                placeholder: placeholder.unwrap_or_default(),
                required: required,
                disabled: disabled,
                value: "{value}",
                oninput: move |evt| value.set(evt.value()),
            }
            if let Some(error) = error {
                p { class: "mt-1 text-sm text-red-600 dark:text-red-400", "{error}" }
            }
            if let Some(help) = help {
                p { class: "mt-1 text-sm text-gray-500 dark:text-gray-400", "{help}" }
            }
        }
    }
}

/// Textarea field
#[component]
pub fn Textarea(
    /// Input label
    label: String,
    /// Input name/id
    name: String,
    /// Current value
    value: Signal<String>,
    /// Number of rows
    #[props(default = 4)]
    rows: u32,
    /// Placeholder text
    placeholder: Option<String>,
    /// Required field
    #[props(default = false)]
    required: bool,
    /// Disabled state
    #[props(default = false)]
    disabled: bool,
    /// Error message
    error: Option<String>,
) -> Element {
    let has_error = error.is_some();
    let textarea_class = if has_error {
        "block w-full rounded-md border-red-300 text-red-900 placeholder-red-300 focus:border-red-500 focus:ring-red-500 sm:text-sm"
    } else {
        "block w-full rounded-md border-gray-300 dark:border-gray-600 dark:bg-gray-700 dark:text-white shadow-sm focus:border-primary-500 focus:ring-primary-500 sm:text-sm"
    };

    rsx! {
        div { class: "mb-4",
            label {
                r#for: "{name}",
                class: "block text-sm font-medium text-gray-700 dark:text-gray-200 mb-1",
                "{label}"
                if required {
                    span { class: "text-red-500 ml-1", "*" }
                }
            }
            textarea {
                name: "{name}",
                id: "{name}",
                rows: "{rows}",
                class: "{textarea_class}",
                placeholder: placeholder.unwrap_or_default(),
                required: required,
                disabled: disabled,
                value: "{value}",
                oninput: move |evt| value.set(evt.value()),
            }
            if let Some(error) = error {
                p { class: "mt-1 text-sm text-red-600 dark:text-red-400", "{error}" }
            }
        }
    }
}

/// Select option
#[derive(Clone, PartialEq)]
pub struct SelectOption {
    pub value: String,
    pub label: String,
}

/// Select dropdown
#[component]
pub fn Select(
    /// Input label
    label: String,
    /// Input name/id
    name: String,
    /// Current value
    value: Signal<String>,
    /// Available options
    options: Vec<SelectOption>,
    /// Placeholder text
    placeholder: Option<String>,
    /// Required field
    #[props(default = false)]
    required: bool,
    /// Disabled state
    #[props(default = false)]
    disabled: bool,
    /// Error message
    error: Option<String>,
) -> Element {
    let has_error = error.is_some();
    let select_class = if has_error {
        "block w-full rounded-md border-red-300 focus:border-red-500 focus:ring-red-500 sm:text-sm"
    } else {
        "block w-full rounded-md border-gray-300 dark:border-gray-600 dark:bg-gray-700 dark:text-white shadow-sm focus:border-primary-500 focus:ring-primary-500 sm:text-sm"
    };

    rsx! {
        div { class: "mb-4",
            label {
                r#for: "{name}",
                class: "block text-sm font-medium text-gray-700 dark:text-gray-200 mb-1",
                "{label}"
                if required {
                    span { class: "text-red-500 ml-1", "*" }
                }
            }
            select {
                name: "{name}",
                id: "{name}",
                class: "{select_class}",
                required: required,
                disabled: disabled,
                value: "{value}",
                onchange: move |evt| value.set(evt.value()),
                if let Some(placeholder) = placeholder {
                    option { value: "", disabled: true, selected: value.read().is_empty(),
                        "{placeholder}"
                    }
                }
                for option in options.iter() {
                    option {
                        value: "{option.value}",
                        selected: *value.read() == option.value,
                        "{option.label}"
                    }
                }
            }
            if let Some(error) = error {
                p { class: "mt-1 text-sm text-red-600 dark:text-red-400", "{error}" }
            }
        }
    }
}

/// Checkbox
#[component]
pub fn Checkbox(
    /// Checkbox label
    label: String,
    /// Input name/id
    name: String,
    /// Checked state
    checked: Signal<bool>,
    /// Disabled state
    #[props(default = false)]
    disabled: bool,
    /// Help text
    help: Option<String>,
) -> Element {
    rsx! {
        div { class: "mb-4 flex items-start",
            div { class: "flex h-5 items-center",
                input {
                    r#type: "checkbox",
                    name: "{name}",
                    id: "{name}",
                    class: "h-4 w-4 rounded border-gray-300 text-primary-600 focus:ring-primary-500",
                    checked: *checked.read(),
                    disabled: disabled,
                    onchange: move |evt| checked.set(evt.checked()),
                }
            }
            div { class: "ml-3 text-sm",
                label {
                    r#for: "{name}",
                    class: "font-medium text-gray-700 dark:text-gray-200",
                    "{label}"
                }
                if let Some(help) = help {
                    p { class: "text-gray-500 dark:text-gray-400", "{help}" }
                }
            }
        }
    }
}
