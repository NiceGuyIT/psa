//! Core shared models used across all PSA modules

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Unique identifier for a tenant
pub type TenantId = Uuid;

/// Unique identifier for a user
pub type UserId = Uuid;

/// User role within the system
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UserRole {
    /// System administrator (cross-tenant)
    SuperAdmin,
    /// Tenant administrator
    Admin,
    /// Regular technician/employee
    Technician,
    /// Read-only user
    Viewer,
    /// External client with portal access
    Client,
}

impl UserRole {
    pub fn as_str(&self) -> &'static str {
        match self {
            UserRole::SuperAdmin => "super_admin",
            UserRole::Admin => "admin",
            UserRole::Technician => "technician",
            UserRole::Viewer => "viewer",
            UserRole::Client => "client",
        }
    }

    /// Check if this role has admin privileges
    pub fn is_admin(&self) -> bool {
        matches!(self, UserRole::SuperAdmin | UserRole::Admin)
    }

    /// Check if this role can modify data
    pub fn can_write(&self) -> bool {
        !matches!(self, UserRole::Viewer | UserRole::Client)
    }
}

impl std::str::FromStr for UserRole {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "super_admin" => Ok(UserRole::SuperAdmin),
            "admin" => Ok(UserRole::Admin),
            "technician" => Ok(UserRole::Technician),
            "viewer" => Ok(UserRole::Viewer),
            "client" => Ok(UserRole::Client),
            _ => Err(format!("Unknown role: {}", s)),
        }
    }
}

/// Current user context for request handling
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserContext {
    pub user_id: UserId,
    pub tenant_id: Option<TenantId>,
    pub email: String,
    pub name: String,
    pub role: UserRole,
}

impl UserContext {
    /// Check if user has access to a specific tenant
    pub fn has_tenant_access(&self, tenant_id: TenantId) -> bool {
        match self.role {
            UserRole::SuperAdmin => true,
            _ => self.tenant_id == Some(tenant_id),
        }
    }
}

/// Pagination parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pagination {
    pub page: u32,
    pub per_page: u32,
}

impl Default for Pagination {
    fn default() -> Self {
        Self {
            page: 1,
            per_page: 25,
        }
    }
}

impl Pagination {
    pub fn offset(&self) -> u32 {
        (self.page.saturating_sub(1)) * self.per_page
    }
}

/// Paginated response wrapper
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedResponse<T> {
    pub items: Vec<T>,
    pub total: u64,
    pub page: u32,
    pub per_page: u32,
    pub total_pages: u32,
}

impl<T> PaginatedResponse<T> {
    pub fn new(items: Vec<T>, total: u64, pagination: &Pagination) -> Self {
        let total_pages = ((total as f64) / (pagination.per_page as f64)).ceil() as u32;
        Self {
            items,
            total,
            page: pagination.page,
            per_page: pagination.per_page,
            total_pages,
        }
    }
}

/// Audit record for tracking changes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditRecord {
    pub id: Uuid,
    pub tenant_id: Option<TenantId>,
    pub user_id: UserId,
    pub action: String,
    pub entity_type: String,
    pub entity_id: String,
    pub old_values: Option<serde_json::Value>,
    pub new_values: Option<serde_json::Value>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub created_at: DateTime<Utc>,
}

/// Sort direction
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum SortDirection {
    #[default]
    Asc,
    Desc,
}

impl SortDirection {
    pub fn as_sql(&self) -> &'static str {
        match self {
            SortDirection::Asc => "ASC",
            SortDirection::Desc => "DESC",
        }
    }
}
