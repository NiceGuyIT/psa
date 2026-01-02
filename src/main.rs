//! PSA Platform - Professional Services Automation for MSPs
//!
//! This application provides a complete PSA solution including:
//! - Ticketing system with email integration
//! - Time tracking and billing
//! - Project and task management
//! - Contact and company management
//! - Calendar and scheduling
//! - Contract and SLA management
//! - Asset management (CMDB)
//! - Knowledge base
//! - Client portal
//! - RMM integration (Tactical RMM)
//! - Multi-channel notifications

use dioxus::prelude::*;
use psa_platform::Route;

/// Application configuration loaded from environment
#[derive(Clone, Debug)]
pub struct AppConfig {
    /// Database connection URL
    pub database_url: String,
    /// JWT secret for token signing
    pub jwt_secret: String,
    /// Server host
    pub host: String,
    /// Server port
    pub port: u16,
    /// Environment (development, staging, production)
    pub environment: String,
    /// Base URL for the application
    pub base_url: String,
    /// Whether to run database migrations on startup
    pub run_migrations: bool,
    /// Encryption key for sensitive data
    pub encryption_key: String,
}

impl AppConfig {
    /// Load configuration from environment variables
    pub fn from_env() -> Result<Self, Box<dyn std::error::Error>> {
        dotenvy::dotenv().ok();

        Ok(Self {
            database_url: std::env::var("DATABASE_URL")
                .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/psa".to_string()),
            jwt_secret: std::env::var("JWT_SECRET")
                .unwrap_or_else(|_| "development-secret-change-in-production".to_string()),
            host: std::env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
            port: std::env::var("PORT")
                .unwrap_or_else(|_| "8080".to_string())
                .parse()
                .unwrap_or(8080),
            environment: std::env::var("ENVIRONMENT").unwrap_or_else(|_| "development".to_string()),
            base_url: std::env::var("BASE_URL")
                .unwrap_or_else(|_| "http://localhost:8080".to_string()),
            run_migrations: std::env::var("RUN_MIGRATIONS")
                .unwrap_or_else(|_| "true".to_string())
                .parse()
                .unwrap_or(true),
            encryption_key: std::env::var("ENCRYPTION_KEY")
                .unwrap_or_else(|_| "32-byte-key-for-dev-only-change!".to_string()),
        })
    }

    /// Check if running in production mode
    pub fn is_production(&self) -> bool {
        self.environment == "production"
    }

    /// Check if running in multi-tenant mode
    #[cfg(feature = "multi-tenant")]
    pub fn is_multi_tenant(&self) -> bool {
        true
    }

    #[cfg(feature = "single-tenant")]
    pub fn is_multi_tenant(&self) -> bool {
        false
    }
}

fn main() {
    // Initialize tracing/logging (server-side only - not available in WASM)
    #[cfg(not(target_arch = "wasm32"))]
    {
        tracing_subscriber::fmt()
            .with_env_filter(
                tracing_subscriber::EnvFilter::from_default_env()
                    .add_directive("psa_platform=debug".parse().unwrap())
                    .add_directive("tower_http=debug".parse().unwrap()),
            )
            .init();

        tracing::info!("Starting PSA Platform");

        #[cfg(feature = "multi-tenant")]
        tracing::info!("Running in multi-tenant mode");

        #[cfg(feature = "single-tenant")]
        tracing::info!("Running in single-tenant mode");
    }

    // Server-side: Use dioxus::serve with custom API routes
    #[cfg(feature = "server")]
    dioxus::serve(|| async move {
        use psa_platform::{api::create_api_router, Database};

        // Load configuration
        let config = AppConfig::from_env().expect("Failed to load configuration");

        // Try to initialize database (optional for development)
        let db_result = Database::new(&config.database_url).await;

        // Create router based on database availability
        let router = match db_result {
            Ok(db) => {
                // Run migrations if enabled and database is available
                if config.run_migrations {
                    if let Err(e) = db.run_migrations().await {
                        tracing::warn!("Failed to run migrations: {}", e);
                    } else {
                        tracing::info!("Database migrations complete");
                    }
                }

                tracing::info!("Database connected");

                // Create the API router with database and JWT secret
                let api_router = create_api_router(db, config.jwt_secret);

                // Merge with Dioxus router
                dioxus::server::router(App).merge(api_router)
            }
            Err(e) => {
                tracing::warn!("Database not available: {}. API routes will not be mounted.", e);
                tracing::warn!("To enable API routes, start PostgreSQL and restart the server.");

                // Return just the Dioxus router without API routes
                dioxus::server::router(App)
            }
        };

        tracing::info!("Server ready");

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
