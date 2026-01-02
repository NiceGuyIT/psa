//! Configuration management for PSA Platform

use serde::{Deserialize, Serialize};

/// Application configuration
#[derive(Clone, Debug, Serialize, Deserialize)]
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
    /// Encryption key for sensitive data (32 bytes)
    pub encryption_key: String,
}

impl AppConfig {
    /// Load configuration from environment variables
    pub fn from_env() -> Result<Self, Box<dyn std::error::Error>> {
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

    /// Check if running in development mode
    pub fn is_development(&self) -> bool {
        self.environment == "development"
    }
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            database_url: "postgres://postgres:postgres@localhost:5432/psa".to_string(),
            jwt_secret: "development-secret-change-in-production".to_string(),
            host: "0.0.0.0".to_string(),
            port: 8080,
            environment: "development".to_string(),
            base_url: "http://localhost:8080".to_string(),
            run_migrations: true,
            encryption_key: "32-byte-key-for-dev-only-change!".to_string(),
        }
    }
}
