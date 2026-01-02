//! Resolver - Standalone Ticketing Application
//!
//! A focused help desk and ticketing solution for individuals and small teams.
//! Part of the PSA Platform ecosystem, can run independently or as part of a suite.

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
                    .add_directive("resolver=debug".parse().unwrap())
                    .add_directive("psa_ticketing=debug".parse().unwrap()),
            )
            .init();

        tracing::info!("Starting Resolver - Ticketing Application");
    }

    // Server-side: Use dioxus::serve with API routes
    #[cfg(feature = "server")]
    dioxus::serve(|| async move {
        use psa_core::{config::AppConfig, db::Database};
        use psa_ticketing::{handlers::TicketingState, ticketing_routes};

        // Load configuration
        let config = AppConfig::from_env().expect("Failed to load configuration");

        // Initialize database
        let db = Database::new(&config.database_url)
            .await
            .expect("Failed to connect to database");

        // Run migrations
        db.run_migrations().await.expect("Failed to run core migrations");

        tracing::info!("Database connected and migrations complete");

        // Create ticketing state
        let ticketing_state = TicketingState { db: db.clone() };

        // Build API routes
        let api_router = axum::Router::new()
            .nest("/tickets", ticketing_routes(ticketing_state));

        // Merge with Dioxus router
        let router = dioxus::server::router(App)
            .nest("/api", api_router);

        tracing::info!("Resolver ready at http://{}:{}", config.host, config.port);

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
