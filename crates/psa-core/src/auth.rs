//! Authentication module for PSA Platform
//!
//! Supports both local authentication (email/password) and SSO.

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

use crate::models::{TenantId, UserId, UserRole};

/// JWT claims for authentication tokens
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    /// Subject (user ID)
    pub sub: String,
    /// Tenant ID (if applicable)
    pub tenant_id: Option<String>,
    /// User email
    pub email: String,
    /// User role
    pub role: String,
    /// Token expiration time (Unix timestamp)
    pub exp: i64,
    /// Token issued at (Unix timestamp)
    pub iat: i64,
    /// Token ID for revocation
    pub jti: String,
}

impl Claims {
    /// Create new claims for a user
    pub fn new(
        user_id: UserId,
        tenant_id: Option<TenantId>,
        email: String,
        role: UserRole,
        expires_in_hours: i64,
    ) -> Self {
        let now = Utc::now();
        Self {
            sub: user_id.to_string(),
            tenant_id: tenant_id.map(|t| t.to_string()),
            email,
            role: role.as_str().to_string(),
            exp: (now + chrono::Duration::hours(expires_in_hours)).timestamp(),
            iat: now.timestamp(),
            jti: Uuid::new_v4().to_string(),
        }
    }

    /// Check if the token is expired
    pub fn is_expired(&self) -> bool {
        Utc::now().timestamp() > self.exp
    }

    /// Get user ID from claims
    pub fn user_id(&self) -> Option<UserId> {
        Uuid::parse_str(&self.sub).ok()
    }

    /// Get tenant ID from claims
    pub fn tenant_id(&self) -> Option<TenantId> {
        self.tenant_id.as_ref().and_then(|t| Uuid::parse_str(t).ok())
    }
}

/// Login request payload
#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
    /// Optional tenant slug for multi-tenant login
    pub tenant: Option<String>,
}

/// Login response
#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub token_type: String,
    pub expires_in: i64,
    pub user: AuthUser,
}

/// Authenticated user info
#[derive(Debug, Serialize, Deserialize)]
pub struct AuthUser {
    pub id: UserId,
    pub email: String,
    pub name: String,
    pub role: UserRole,
    pub tenant_id: Option<TenantId>,
    pub tenant_name: Option<String>,
}

/// SSO provider configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SsoConfig {
    pub provider: SsoProvider,
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uri: String,
    pub enabled: bool,
}

/// Supported SSO providers
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SsoProvider {
    Google,
    Microsoft,
    Okta,
    Auth0,
    Saml,
}

impl SsoProvider {
    pub fn as_str(&self) -> &'static str {
        match self {
            SsoProvider::Google => "google",
            SsoProvider::Microsoft => "microsoft",
            SsoProvider::Okta => "okta",
            SsoProvider::Auth0 => "auth0",
            SsoProvider::Saml => "saml",
        }
    }
}

#[cfg(feature = "server")]
pub mod server {
    //! Server-side authentication utilities

    use super::*;
    use argon2::{
        password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
        Argon2,
    };
    use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};

    use crate::error::{CoreError, Result};

    /// Hash a password using Argon2
    pub fn hash_password(password: &str) -> Result<String> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();

        argon2
            .hash_password(password.as_bytes(), &salt)
            .map(|hash| hash.to_string())
            .map_err(|e| CoreError::Internal(format!("Password hashing failed: {}", e)))
    }

    /// Verify a password against a hash
    pub fn verify_password(password: &str, hash: &str) -> Result<bool> {
        let parsed_hash = PasswordHash::new(hash)
            .map_err(|e| CoreError::Internal(format!("Invalid password hash: {}", e)))?;

        Ok(Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok())
    }

    /// Generate a JWT token
    pub fn generate_token(claims: &Claims, secret: &str) -> Result<String> {
        encode(
            &Header::default(),
            claims,
            &EncodingKey::from_secret(secret.as_bytes()),
        )
        .map_err(|e| CoreError::Internal(format!("Token generation failed: {}", e)))
    }

    /// Validate and decode a JWT token
    pub fn validate_token(token: &str, secret: &str) -> Result<Claims> {
        decode::<Claims>(
            token,
            &DecodingKey::from_secret(secret.as_bytes()),
            &Validation::default(),
        )
        .map(|data| data.claims)
        .map_err(|e| CoreError::AuthenticationFailed(format!("Invalid token: {}", e)))
    }
}
