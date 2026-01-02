//! Authentication middleware for Axum

use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::Response,
};
use std::sync::Arc;

use super::{AuthService, AuthState, CurrentUser, JwtClaims, UserRole};
use crate::utils::error::AppError;

/// Extension to hold the current auth state
#[derive(Clone)]
pub struct AuthMiddleware {
    pub auth_service: Arc<AuthService>,
}

impl AuthMiddleware {
    pub fn new(auth_service: AuthService) -> Self {
        Self {
            auth_service: Arc::new(auth_service),
        }
    }
}

/// Extract auth state from request
pub async fn auth_middleware(
    State(auth_middleware): State<AuthMiddleware>,
    mut request: Request,
    next: Next,
) -> Response {
    // Try to extract token from Authorization header
    let auth_state = if let Some(auth_header) = request.headers().get("Authorization") {
        if let Ok(auth_str) = auth_header.to_str() {
            if let Some(token) = auth_str.strip_prefix("Bearer ") {
                match auth_middleware.auth_service.decode_token(token) {
                    Ok(claims) => {
                        // Fetch user to get current info
                        match auth_middleware
                            .auth_service
                            .get_user_by_id(claims.sub)
                            .await
                        {
                            Ok(user) => AuthState::authenticated(
                                user.to_current_user(),
                                claims.tid,
                            ),
                            Err(_) => AuthState::default(),
                        }
                    }
                    Err(_) => AuthState::default(),
                }
            } else {
                AuthState::default()
            }
        } else {
            AuthState::default()
        }
    } else {
        AuthState::default()
    };

    // Insert auth state into request extensions
    request.extensions_mut().insert(auth_state);

    next.run(request).await
}

/// Middleware that requires authentication
pub async fn require_auth(
    request: Request,
    next: Next,
) -> Result<Response, (StatusCode, String)> {
    let auth_state = request
        .extensions()
        .get::<AuthState>()
        .cloned()
        .unwrap_or_default();

    if !auth_state.is_authenticated {
        return Err((StatusCode::UNAUTHORIZED, "Authentication required".to_string()));
    }

    Ok(next.run(request).await)
}

/// Extractor for requiring authentication
#[derive(Clone)]
pub struct RequireAuth(pub CurrentUser);

impl<S> axum::extract::FromRequestParts<S> for RequireAuth
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        _state: &S,
    ) -> Result<Self, Self::Rejection> {
        let auth_state = parts
            .extensions
            .get::<AuthState>()
            .cloned()
            .unwrap_or_default();

        match auth_state.user {
            Some(user) => Ok(RequireAuth(user)),
            None => Err(AppError::Unauthorized),
        }
    }
}

/// Extractor for requiring a specific role
#[derive(Clone)]
pub struct RequireRole<const ROLES: &'static [&'static str]>(pub CurrentUser);

impl<S, const ROLES: &'static [&'static str]> axum::extract::FromRequestParts<S>
    for RequireRole<ROLES>
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        _state: &S,
    ) -> Result<Self, Self::Rejection> {
        let auth_state = parts
            .extensions
            .get::<AuthState>()
            .cloned()
            .unwrap_or_default();

        match auth_state.user {
            Some(user) => {
                let user_role = user.role.as_str();
                if ROLES.contains(&user_role) {
                    Ok(RequireRole(user))
                } else {
                    Err(AppError::Forbidden(
                        "Insufficient permissions".to_string(),
                    ))
                }
            }
            None => Err(AppError::Unauthorized),
        }
    }
}

/// Helper type aliases for common role requirements
pub type RequireAdmin = RequireRole<{ &["super_admin", "admin"] }>;
pub type RequireManager = RequireRole<{ &["super_admin", "admin", "manager"] }>;
pub type RequireFinance = RequireRole<{ &["super_admin", "admin", "finance"] }>;

/// Get the current user's tenant ID from the request
pub fn get_tenant_id(request: &Request) -> Option<uuid::Uuid> {
    request
        .extensions()
        .get::<AuthState>()
        .and_then(|state| state.tenant_id)
}

/// Get the current user from the request
pub fn get_current_user(request: &Request) -> Option<CurrentUser> {
    request
        .extensions()
        .get::<AuthState>()
        .and_then(|state| state.user.clone())
}
