//! Table components

use dioxus::prelude::*;

/// Simple table without DataTable complexity
#[component]
pub fn SimpleTable(
    /// Header cells
    headers: Vec<String>,
    /// Table content (rows)
    children: Element,
) -> Element {
    rsx! {
        div { class: "overflow-hidden shadow ring-1 ring-black ring-opacity-5 rounded-lg",
            table { class: "min-w-full divide-y divide-gray-300 dark:divide-gray-700",
                thead { class: "bg-gray-50 dark:bg-gray-800",
                    tr {
                        for header in headers.iter() {
                            th {
                                scope: "col",
                                class: "px-3 py-3.5 text-left text-sm font-semibold text-gray-900 dark:text-white",
                                "{header}"
                            }
                        }
                    }
                }
                tbody { class: "divide-y divide-gray-200 dark:divide-gray-700 bg-white dark:bg-gray-900",
                    {children}
                }
            }
        }
    }
}

/// Table row
#[component]
pub fn TableRow(
    children: Element,
    #[props(default = false)]
    highlighted: bool,
) -> Element {
    let class = if highlighted {
        "bg-primary-50 dark:bg-primary-900/20"
    } else {
        "hover:bg-gray-50 dark:hover:bg-gray-800"
    };

    rsx! {
        tr { class: "{class}",
            {children}
        }
    }
}

/// Table cell
#[component]
pub fn TableCell(
    children: Element,
    #[props(default = false)]
    numeric: bool,
) -> Element {
    let align = if numeric { "text-right" } else { "text-left" };

    rsx! {
        td { class: "whitespace-nowrap px-3 py-4 text-sm text-gray-700 dark:text-gray-300 {align}",
            {children}
        }
    }
}

/// Pagination component
#[component]
pub fn Pagination(
    current_page: u32,
    total_pages: u32,
    on_page_change: EventHandler<u32>,
) -> Element {
    if total_pages <= 1 {
        return rsx! {};
    }

    rsx! {
        nav { class: "flex items-center justify-between border-t border-gray-200 dark:border-gray-700 px-4 py-3 sm:px-6",
            div { class: "hidden sm:flex sm:flex-1 sm:items-center sm:justify-between",
                div {
                    p { class: "text-sm text-gray-700 dark:text-gray-300",
                        "Page "
                        span { class: "font-medium", "{current_page}" }
                        " of "
                        span { class: "font-medium", "{total_pages}" }
                    }
                }
                div { class: "flex gap-2",
                    button {
                        class: "relative inline-flex items-center rounded-md px-3 py-2 text-sm font-semibold text-gray-900 dark:text-white ring-1 ring-inset ring-gray-300 dark:ring-gray-600 hover:bg-gray-50 dark:hover:bg-gray-800 disabled:opacity-50 disabled:cursor-not-allowed",
                        disabled: current_page == 1,
                        onclick: move |_| on_page_change.call(current_page - 1),
                        "Previous"
                    }
                    button {
                        class: "relative inline-flex items-center rounded-md px-3 py-2 text-sm font-semibold text-gray-900 dark:text-white ring-1 ring-inset ring-gray-300 dark:ring-gray-600 hover:bg-gray-50 dark:hover:bg-gray-800 disabled:opacity-50 disabled:cursor-not-allowed",
                        disabled: current_page == total_pages,
                        onclick: move |_| on_page_change.call(current_page + 1),
                        "Next"
                    }
                }
            }
        }
    }
}
