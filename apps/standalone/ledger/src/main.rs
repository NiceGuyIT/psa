//! Ledger - Standalone Billing Application

use dioxus::prelude::*;

fn main() {
    #[cfg(not(target_arch = "wasm32"))]
    {
        tracing_subscriber::fmt().init();
        tracing::info!("Starting Ledger - Billing");
    }

    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        div { class: "min-h-screen bg-gray-100 p-8",
            h1 { class: "text-3xl font-bold text-amber-600", "Ledger" }
            p { class: "text-gray-600 mt-2", "Simple invoicing and payments." }
        }
    }
}
