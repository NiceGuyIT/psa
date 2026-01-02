//! Authentication and Authorization Module
//!
//! Handles user authentication, session management, and authorization.

mod models;
mod service;
#[cfg(feature = "server")]
mod routes;
#[cfg(feature = "server")]
pub mod middleware;

pub use models::*;
#[cfg(feature = "server")]
pub use routes::auth_routes;
#[cfg(feature = "server")]
pub use service::AuthService;
#[cfg(feature = "server")]
pub use middleware::{AuthMiddleware, RequireAuth, RequireRole, RoleRequirement, AdminRoles, ManagerRoles, FinanceRoles, RequireAdmin, RequireManager, RequireFinance};
