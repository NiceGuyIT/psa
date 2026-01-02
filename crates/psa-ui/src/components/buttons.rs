//! Button components

use dioxus::prelude::*;

/// Button variant
#[derive(Clone, Copy, Default, PartialEq)]
pub enum ButtonVariant {
    #[default]
    Primary,
    Secondary,
    Danger,
    Ghost,
}

impl ButtonVariant {
    fn classes(&self) -> &'static str {
        match self {
            ButtonVariant::Primary => "bg-primary-600 text-white hover:bg-primary-700 focus:ring-primary-500",
            ButtonVariant::Secondary => "bg-white text-gray-700 border border-gray-300 hover:bg-gray-50 focus:ring-primary-500 dark:bg-gray-700 dark:text-gray-200 dark:border-gray-600 dark:hover:bg-gray-600",
            ButtonVariant::Danger => "bg-red-600 text-white hover:bg-red-700 focus:ring-red-500",
            ButtonVariant::Ghost => "text-gray-700 hover:bg-gray-100 dark:text-gray-200 dark:hover:bg-gray-700",
        }
    }
}

/// Button size
#[derive(Clone, Copy, Default, PartialEq)]
pub enum ButtonSize {
    Small,
    #[default]
    Medium,
    Large,
}

impl ButtonSize {
    fn classes(&self) -> &'static str {
        match self {
            ButtonSize::Small => "px-2.5 py-1.5 text-xs",
            ButtonSize::Medium => "px-4 py-2 text-sm",
            ButtonSize::Large => "px-6 py-3 text-base",
        }
    }
}

/// Primary button component
#[component]
pub fn Button(
    /// Button text
    children: Element,
    /// Click handler
    onclick: Option<EventHandler<MouseEvent>>,
    /// Button variant
    #[props(default)]
    variant: ButtonVariant,
    /// Button size
    #[props(default)]
    size: ButtonSize,
    /// Disabled state
    #[props(default = false)]
    disabled: bool,
    /// Loading state
    #[props(default = false)]
    loading: bool,
    /// Additional CSS classes
    class: Option<String>,
    /// Button type
    #[props(default = "button")]
    r#type: &'static str,
) -> Element {
    let base_classes = "inline-flex items-center justify-center rounded-md font-medium shadow-sm focus:outline-none focus:ring-2 focus:ring-offset-2 disabled:opacity-50 disabled:cursor-not-allowed transition-colors";
    let variant_classes = variant.classes();
    let size_classes = size.classes();
    let extra_classes = class.unwrap_or_default();

    let full_class = format!("{} {} {} {}", base_classes, variant_classes, size_classes, extra_classes);

    rsx! {
        button {
            r#type: r#type,
            class: "{full_class}",
            disabled: disabled || loading,
            onclick: move |evt| {
                if let Some(handler) = &onclick {
                    handler.call(evt);
                }
            },
            if loading {
                span { class: "mr-2 animate-spin", "⟳" }
            }
            {children}
        }
    }
}

/// Link styled as a button
#[component]
pub fn ButtonLink(
    /// Link text
    children: Element,
    /// Link href
    href: String,
    /// Button variant
    #[props(default)]
    variant: ButtonVariant,
    /// Button size
    #[props(default)]
    size: ButtonSize,
    /// Additional CSS classes
    class: Option<String>,
) -> Element {
    let base_classes = "inline-flex items-center justify-center rounded-md font-medium shadow-sm focus:outline-none focus:ring-2 focus:ring-offset-2 transition-colors no-underline";
    let variant_classes = variant.classes();
    let size_classes = size.classes();
    let extra_classes = class.unwrap_or_default();

    let full_class = format!("{} {} {} {}", base_classes, variant_classes, size_classes, extra_classes);

    rsx! {
        a {
            href: "{href}",
            class: "{full_class}",
            {children}
        }
    }
}

/// Icon button
#[component]
pub fn IconButton(
    /// Icon (emoji or text)
    icon: String,
    /// Click handler
    onclick: Option<EventHandler<MouseEvent>>,
    /// Aria label for accessibility
    label: String,
    /// Button variant
    #[props(default)]
    variant: ButtonVariant,
    /// Disabled state
    #[props(default = false)]
    disabled: bool,
) -> Element {
    let variant_classes = variant.classes();

    rsx! {
        button {
            r#type: "button",
            class: "p-2 rounded-md focus:outline-none focus:ring-2 focus:ring-offset-2 disabled:opacity-50 disabled:cursor-not-allowed {variant_classes}",
            disabled: disabled,
            "aria-label": "{label}",
            onclick: move |evt| {
                if let Some(handler) = &onclick {
                    handler.call(evt);
                }
            },
            "{icon}"
        }
    }
}
