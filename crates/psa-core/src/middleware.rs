//! HTTP middleware for PSA Platform
//!
//! Provides authentication, tenant resolution, and other cross-cutting concerns.

use axum::{
    extract::{Request, State},
    http::{header, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
    Json,
};

use crate::{
    auth::{server::validate_token, Claims},
    error::CoreError,
    models::{TenantId, UserContext, UserRole},
};

/// Application state shared across handlers
#[derive(Clone)]
pub struct AppState {
    pub db: crate::db::Database,
    pub jwt_secret: String,
}

/// Extract bearer token from Authorization header
fn extract_bearer_token(req: &Request) -> Option<&str> {
    req.headers()
        .get(header::AUTHORIZATION)?
        .to_str()
        .ok()?
        .strip_prefix("Bearer ")
}

/// Authentication middleware
pub async fn auth_middleware(
    State(state): State<AppState>,
    mut req: Request,
    next: Next,
) -> Result<Response, CoreError> {
    let token = extract_bearer_token(&req)
        .ok_or_else(|| CoreError::AuthenticationFailed("Missing authorization header".to_string()))?;

    let claims = validate_token(token, &state.jwt_secret)?;

    if claims.is_expired() {
        return Err(CoreError::AuthenticationFailed("Token expired".to_string()));
    }

    // Build user context from claims
    let user_context = UserContext {
        user_id: claims.user_id().ok_or_else(|| {
            CoreError::AuthenticationFailed("Invalid user ID in token".to_string())
        })?,
        tenant_id: claims.tenant_id(),
        email: claims.email.clone(),
        name: claims.email.clone(), // TODO: Get from database
        role: claims.role.parse().unwrap_or(UserRole::Viewer),
    };

    // Insert user context into request extensions
    req.extensions_mut().insert(user_context);
    req.extensions_mut().insert(claims);

    Ok(next.run(req).await)
}

/// Optional authentication middleware (allows unauthenticated requests)
pub async fn optional_auth_middleware(
    State(state): State<AppState>,
    mut req: Request,
    next: Next,
) -> Response {
    if let Some(token) = extract_bearer_token(&req) {
        if let Ok(claims) = validate_token(token, &state.jwt_secret) {
            if !claims.is_expired() {
                if let Some(user_id) = claims.user_id() {
                    let user_context = UserContext {
                        user_id,
                        tenant_id: claims.tenant_id(),
                        email: claims.email.clone(),
                        name: claims.email.clone(),
                        role: claims.role.parse().unwrap_or(UserRole::Viewer),
                    };
                    req.extensions_mut().insert(user_context);
                    req.extensions_mut().insert(claims);
                }
            }
        }
    }

    next.run(req).await
}

/// Tenant resolution middleware (must come after auth_middleware)
pub async fn tenant_middleware(
    State(state): State<AppState>,
    mut req: Request,
    next: Next,
) -> Result<Response, CoreError> {
    let user_context = req
        .extensions()
        .get::<UserContext>()
        .ok_or_else(|| CoreError::AuthenticationFailed("User context not found".to_string()))?
        .clone();

    // Super admins can access without tenant
    if user_context.role == UserRole::SuperAdmin {
        return Ok(next.run(req).await);
    }

    // Non-super-admins must have a tenant
    let tenant_id = user_context.tenant_id.ok_or_else(|| {
        CoreError::AuthorizationDenied("Tenant context required".to_string())
    })?;

    // Verify tenant exists and is active
    let tenant_repo = crate::tenants::TenantRepository::new(state.db.pool());
    let tenant = tenant_repo
        .find_by_id(tenant_id)
        .await
        .map_err(|e| CoreError::Database(e.to_string()))?
        .ok_or_else(|| CoreError::TenantNotFound(tenant_id.to_string()))?;

    if !tenant.status.is_accessible() {
        return Err(CoreError::TenantSuspended(tenant.name));
    }

    // Insert tenant into request extensions
    req.extensions_mut().insert(tenant);

    Ok(next.run(req).await)
}

/// Role requirement middleware
pub fn require_role(required_roles: &'static [&'static str]) -> impl Fn(Request, Next) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<Response, CoreError>> + Send>> + Clone {
    move |req: Request, next: Next| {
        Box::pin(async move {
            let user_context = req
                .extensions()
                .get::<UserContext>()
                .ok_or_else(|| CoreError::AuthenticationFailed("User context not found".to_string()))?;

            let user_role = user_context.role.as_str();

            if !required_roles.contains(&user_role) && user_context.role != UserRole::SuperAdmin {
                return Err(CoreError::AuthorizationDenied(format!(
                    "Required role: {:?}, got: {}",
                    required_roles,
                    user_role
                )));
            }

            Ok(next.run(req).await)
        })
    }
}
