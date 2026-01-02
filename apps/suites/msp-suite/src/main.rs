//! MSP Suite - Complete Professional Services Automation
//!
//! A comprehensive PSA solution for Managed Service Providers, combining:
//! - Ticketing & Help Desk
//! - Time Tracking & Timesheets
//! - Project Management
//! - CRM & Contact Management
//! - Billing & Invoicing
//! - Asset Management (CMDB)
//! - Knowledge Base
//! - Calendar & Scheduling

use dioxus::prelude::*;

mod routes;

use routes::Route;

fn main() {
    // Initialize tracing (server-side only)
    #[cfg(not(target_arch = "wasm32"))]
    {
        tracing_subscriber::fmt()
            .with_env_filter(
                tracing_subscriber::EnvFilter::from_default_env()
                    .add_directive("msp_suite=debug".parse().unwrap())
                    .add_directive("psa_core=debug".parse().unwrap()),
            )
            .init();

        tracing::info!("Starting MSP Suite - Professional Services Automation");
    }

    // Server-side: Use dioxus::serve with API routes
    #[cfg(feature = "server")]
    dioxus::serve(|| async move {
        use psa_core::{config::AppConfig, db::Database};

        // Load configuration
        let config = AppConfig::from_env().expect("Failed to load configuration");

        // Initialize database
        let db = Database::new(&config.database_url)
            .await
            .expect("Failed to connect to database");

        // Run core migrations
        db.run_migrations().await.expect("Failed to run core migrations");

        tracing::info!("Database connected and migrations complete");

        // Build API router with all enabled modules
        let mut api_router = axum::Router::new();

        // Mount ticketing routes
        #[cfg(feature = "ticketing")]
        {
            use psa_ticketing::{handlers::TicketingState, ticketing_routes};
            let ticketing_state = TicketingState { db: db.clone() };
            api_router = api_router.nest("/tickets", ticketing_routes(ticketing_state));
            tracing::info!("Ticketing module enabled");
        }

        // Mount time tracking routes
        #[cfg(feature = "time-tracking")]
        {
            // TODO: Add time tracking routes when module is implemented
            tracing::info!("Time Tracking module enabled");
        }

        // Mount project routes
        #[cfg(feature = "projects")]
        {
            // TODO: Add project routes when module is implemented
            tracing::info!("Projects module enabled");
        }

        // Mount CRM routes
        #[cfg(feature = "crm")]
        {
            // TODO: Add CRM routes when module is implemented
            tracing::info!("CRM module enabled");
        }

        // Mount billing routes
        #[cfg(feature = "billing")]
        {
            // TODO: Add billing routes when module is implemented
            tracing::info!("Billing module enabled");
        }

        // Mount assets routes
        #[cfg(feature = "assets")]
        {
            // TODO: Add assets routes when module is implemented
            tracing::info!("Assets module enabled");
        }

        // Mount knowledge base routes
        #[cfg(feature = "knowledge-base")]
        {
            // TODO: Add knowledge base routes when module is implemented
            tracing::info!("Knowledge Base module enabled");
        }

        // Mount calendar routes
        #[cfg(feature = "calendar")]
        {
            // TODO: Add calendar routes when module is implemented
            tracing::info!("Calendar module enabled");
        }

        // Merge with Dioxus router
        let router = dioxus::server::router(App)
            .nest("/api", api_router);

        tracing::info!("MSP Suite ready at http://{}:{}", config.host, config.port);

        Ok(router)
    });

    // Client-side (WASM): Use dioxus::launch
    #[cfg(not(feature = "server"))]
    dioxus::launch(App);
}

/// Root application component
#[component]
fn App() -> Element {
    rsx! {
        document::Stylesheet { href: asset!("/assets/styles.css") }
        Router::<Route> {}
    }
}
