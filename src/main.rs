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

mod api;
mod components;
mod db;
mod hooks;
mod modules;
mod pages;
mod utils;

// Re-export commonly used types
pub use modules::auth::AuthState;
pub use utils::error::{AppError, AppResult};

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
    // Initialize tracing/logging
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

    // Launch Dioxus application
    dioxus::launch(App);
}

/// Root application component
#[component]
fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}

/// Application routes
#[derive(Clone, Routable, Debug, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    // Public routes
    #[route("/")]
    Home {},

    #[route("/login")]
    Login {},

    #[route("/forgot-password")]
    ForgotPassword {},

    #[route("/reset-password/:token")]
    ResetPassword { token: String },

    // Dashboard
    #[route("/dashboard")]
    Dashboard {},

    // Tickets
    #[route("/tickets")]
    TicketList {},

    #[route("/tickets/new")]
    TicketNew {},

    #[route("/tickets/:id")]
    TicketDetail { id: String },

    // Time Tracking
    #[route("/time")]
    TimeEntryList {},

    #[route("/time/new")]
    TimeEntryNew {},

    #[route("/timesheets")]
    Timesheets {},

    // Projects
    #[route("/projects")]
    ProjectList {},

    #[route("/projects/new")]
    ProjectNew {},

    #[route("/projects/:id")]
    ProjectDetail { id: String },

    #[route("/projects/:id/tasks")]
    ProjectTasks { id: String },

    // Contacts
    #[route("/companies")]
    CompanyList {},

    #[route("/companies/new")]
    CompanyNew {},

    #[route("/companies/:id")]
    CompanyDetail { id: String },

    #[route("/contacts")]
    ContactList {},

    #[route("/contacts/new")]
    ContactNew {},

    #[route("/contacts/:id")]
    ContactDetail { id: String },

    // Calendar
    #[route("/calendar")]
    Calendar {},

    #[route("/dispatch")]
    DispatchBoard {},

    // Contracts
    #[route("/contracts")]
    ContractList {},

    #[route("/contracts/new")]
    ContractNew {},

    #[route("/contracts/:id")]
    ContractDetail { id: String },

    // Billing
    #[route("/invoices")]
    InvoiceList {},

    #[route("/invoices/new")]
    InvoiceNew {},

    #[route("/invoices/:id")]
    InvoiceDetail { id: String },

    #[route("/payments")]
    PaymentList {},

    // Assets
    #[route("/assets")]
    AssetList {},

    #[route("/assets/new")]
    AssetNew {},

    #[route("/assets/:id")]
    AssetDetail { id: String },

    // Knowledge Base
    #[route("/kb")]
    KBHome {},

    #[route("/kb/articles")]
    KBArticleList {},

    #[route("/kb/articles/new")]
    KBArticleNew {},

    #[route("/kb/articles/:id")]
    KBArticleDetail { id: String },

    // Reports
    #[route("/reports")]
    Reports {},

    #[route("/reports/:report_type")]
    ReportDetail { report_type: String },

    // Settings
    #[route("/settings")]
    Settings {},

    #[route("/settings/users")]
    UserManagement {},

    #[route("/settings/teams")]
    TeamManagement {},

    #[route("/settings/notifications")]
    NotificationSettings {},

    #[route("/settings/integrations")]
    IntegrationSettings {},

    #[route("/settings/billing")]
    BillingSettings {},

    // Admin (multi-tenant only)
    #[cfg(feature = "multi-tenant")]
    #[route("/admin/tenants")]
    TenantManagement {},

    // Client Portal Routes (separate layout)
    #[route("/portal")]
    PortalHome {},

    #[route("/portal/tickets")]
    PortalTicketList {},

    #[route("/portal/tickets/new")]
    PortalTicketNew {},

    #[route("/portal/tickets/:id")]
    PortalTicketDetail { id: String },

    #[route("/portal/invoices")]
    PortalInvoiceList {},

    #[route("/portal/invoices/:id")]
    PortalInvoiceDetail { id: String },

    #[route("/portal/kb")]
    PortalKB {},

    // Catch-all 404
    #[route("/:..route")]
    NotFound { route: Vec<String> },
}

// Page component stubs - these will be implemented in the pages module
#[component]
fn Home() -> Element {
    rsx! {
        pages::home::HomePage {}
    }
}

#[component]
fn Login() -> Element {
    rsx! {
        pages::auth::LoginPage {}
    }
}

#[component]
fn ForgotPassword() -> Element {
    rsx! {
        pages::auth::ForgotPasswordPage {}
    }
}

#[component]
fn ResetPassword(token: String) -> Element {
    rsx! {
        pages::auth::ResetPasswordPage { token }
    }
}

#[component]
fn Dashboard() -> Element {
    rsx! {
        pages::dashboard::DashboardPage {}
    }
}

// Ticket pages
#[component]
fn TicketList() -> Element {
    rsx! {
        pages::tickets::TicketListPage {}
    }
}

#[component]
fn TicketNew() -> Element {
    rsx! {
        pages::tickets::TicketNewPage {}
    }
}

#[component]
fn TicketDetail(id: String) -> Element {
    rsx! {
        pages::tickets::TicketDetailPage { id }
    }
}

// Time tracking pages
#[component]
fn TimeEntryList() -> Element {
    rsx! {
        pages::time::TimeEntryListPage {}
    }
}

#[component]
fn TimeEntryNew() -> Element {
    rsx! {
        pages::time::TimeEntryNewPage {}
    }
}

#[component]
fn Timesheets() -> Element {
    rsx! {
        pages::time::TimesheetsPage {}
    }
}

// Project pages
#[component]
fn ProjectList() -> Element {
    rsx! {
        pages::projects::ProjectListPage {}
    }
}

#[component]
fn ProjectNew() -> Element {
    rsx! {
        pages::projects::ProjectNewPage {}
    }
}

#[component]
fn ProjectDetail(id: String) -> Element {
    rsx! {
        pages::projects::ProjectDetailPage { id }
    }
}

#[component]
fn ProjectTasks(id: String) -> Element {
    rsx! {
        pages::projects::ProjectTasksPage { id }
    }
}

// Company/Contact pages
#[component]
fn CompanyList() -> Element {
    rsx! {
        pages::contacts::CompanyListPage {}
    }
}

#[component]
fn CompanyNew() -> Element {
    rsx! {
        pages::contacts::CompanyNewPage {}
    }
}

#[component]
fn CompanyDetail(id: String) -> Element {
    rsx! {
        pages::contacts::CompanyDetailPage { id }
    }
}

#[component]
fn ContactList() -> Element {
    rsx! {
        pages::contacts::ContactListPage {}
    }
}

#[component]
fn ContactNew() -> Element {
    rsx! {
        pages::contacts::ContactNewPage {}
    }
}

#[component]
fn ContactDetail(id: String) -> Element {
    rsx! {
        pages::contacts::ContactDetailPage { id }
    }
}

// Calendar pages
#[component]
fn Calendar() -> Element {
    rsx! {
        pages::calendar::CalendarPage {}
    }
}

#[component]
fn DispatchBoard() -> Element {
    rsx! {
        pages::calendar::DispatchBoardPage {}
    }
}

// Contract pages
#[component]
fn ContractList() -> Element {
    rsx! {
        pages::contracts::ContractListPage {}
    }
}

#[component]
fn ContractNew() -> Element {
    rsx! {
        pages::contracts::ContractNewPage {}
    }
}

#[component]
fn ContractDetail(id: String) -> Element {
    rsx! {
        pages::contracts::ContractDetailPage { id }
    }
}

// Invoice pages
#[component]
fn InvoiceList() -> Element {
    rsx! {
        pages::billing::InvoiceListPage {}
    }
}

#[component]
fn InvoiceNew() -> Element {
    rsx! {
        pages::billing::InvoiceNewPage {}
    }
}

#[component]
fn InvoiceDetail(id: String) -> Element {
    rsx! {
        pages::billing::InvoiceDetailPage { id }
    }
}

#[component]
fn PaymentList() -> Element {
    rsx! {
        pages::billing::PaymentListPage {}
    }
}

// Asset pages
#[component]
fn AssetList() -> Element {
    rsx! {
        pages::assets::AssetListPage {}
    }
}

#[component]
fn AssetNew() -> Element {
    rsx! {
        pages::assets::AssetNewPage {}
    }
}

#[component]
fn AssetDetail(id: String) -> Element {
    rsx! {
        pages::assets::AssetDetailPage { id }
    }
}

// Knowledge Base pages
#[component]
fn KBHome() -> Element {
    rsx! {
        pages::knowledge_base::KBHomePage {}
    }
}

#[component]
fn KBArticleList() -> Element {
    rsx! {
        pages::knowledge_base::KBArticleListPage {}
    }
}

#[component]
fn KBArticleNew() -> Element {
    rsx! {
        pages::knowledge_base::KBArticleNewPage {}
    }
}

#[component]
fn KBArticleDetail(id: String) -> Element {
    rsx! {
        pages::knowledge_base::KBArticleDetailPage { id }
    }
}

// Reports pages
#[component]
fn Reports() -> Element {
    rsx! {
        pages::reports::ReportsPage {}
    }
}

#[component]
fn ReportDetail(report_type: String) -> Element {
    rsx! {
        pages::reports::ReportDetailPage { report_type }
    }
}

// Settings pages
#[component]
fn Settings() -> Element {
    rsx! {
        pages::settings::SettingsPage {}
    }
}

#[component]
fn UserManagement() -> Element {
    rsx! {
        pages::settings::UserManagementPage {}
    }
}

#[component]
fn TeamManagement() -> Element {
    rsx! {
        pages::settings::TeamManagementPage {}
    }
}

#[component]
fn NotificationSettings() -> Element {
    rsx! {
        pages::settings::NotificationSettingsPage {}
    }
}

#[component]
fn IntegrationSettings() -> Element {
    rsx! {
        pages::settings::IntegrationSettingsPage {}
    }
}

#[component]
fn BillingSettings() -> Element {
    rsx! {
        pages::settings::BillingSettingsPage {}
    }
}

// Tenant management (multi-tenant only)
#[cfg(feature = "multi-tenant")]
#[component]
fn TenantManagement() -> Element {
    rsx! {
        pages::admin::TenantManagementPage {}
    }
}

// Client Portal pages
#[component]
fn PortalHome() -> Element {
    rsx! {
        pages::portal::PortalHomePage {}
    }
}

#[component]
fn PortalTicketList() -> Element {
    rsx! {
        pages::portal::PortalTicketListPage {}
    }
}

#[component]
fn PortalTicketNew() -> Element {
    rsx! {
        pages::portal::PortalTicketNewPage {}
    }
}

#[component]
fn PortalTicketDetail(id: String) -> Element {
    rsx! {
        pages::portal::PortalTicketDetailPage { id }
    }
}

#[component]
fn PortalInvoiceList() -> Element {
    rsx! {
        pages::portal::PortalInvoiceListPage {}
    }
}

#[component]
fn PortalInvoiceDetail(id: String) -> Element {
    rsx! {
        pages::portal::PortalInvoiceDetailPage { id }
    }
}

#[component]
fn PortalKB() -> Element {
    rsx! {
        pages::portal::PortalKBPage {}
    }
}

// 404 page
#[component]
fn NotFound(route: Vec<String>) -> Element {
    rsx! {
        pages::not_found::NotFoundPage { route }
    }
}
