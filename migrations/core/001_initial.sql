-- Core module initial migration
-- Prefix: core_

-- Tenant status enum
CREATE TYPE tenant_status AS ENUM ('active', 'trial', 'suspended', 'cancelled');

-- Subscription tier enum
CREATE TYPE subscription_tier AS ENUM ('free', 'personal', 'professional', 'enterprise');

-- Tenants table
CREATE TABLE core_tenants (
    id UUID PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    slug VARCHAR(100) NOT NULL UNIQUE,
    status tenant_status NOT NULL DEFAULT 'trial',
    tier subscription_tier NOT NULL DEFAULT 'free',
    trial_ends_at TIMESTAMPTZ,
    settings JSONB NOT NULL DEFAULT '{}',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_core_tenants_slug ON core_tenants(slug);
CREATE INDEX idx_core_tenants_status ON core_tenants(status);

-- Users table
CREATE TABLE core_users (
    id UUID PRIMARY KEY,
    tenant_id UUID REFERENCES core_tenants(id) ON DELETE CASCADE,
    email VARCHAR(255) NOT NULL,
    password_hash VARCHAR(255),
    name VARCHAR(255) NOT NULL,
    role VARCHAR(50) NOT NULL DEFAULT 'viewer',
    is_active BOOLEAN NOT NULL DEFAULT true,
    email_verified_at TIMESTAMPTZ,
    last_login_at TIMESTAMPTZ,
    settings JSONB NOT NULL DEFAULT '{}',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(tenant_id, email)
);

CREATE INDEX idx_core_users_tenant ON core_users(tenant_id);
CREATE INDEX idx_core_users_email ON core_users(email);

-- Refresh tokens for JWT
CREATE TABLE core_refresh_tokens (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES core_users(id) ON DELETE CASCADE,
    token_hash VARCHAR(255) NOT NULL UNIQUE,
    expires_at TIMESTAMPTZ NOT NULL,
    revoked_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_core_refresh_tokens_user ON core_refresh_tokens(user_id);
CREATE INDEX idx_core_refresh_tokens_expires ON core_refresh_tokens(expires_at);

-- SSO configurations per tenant
CREATE TABLE core_sso_configs (
    id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL REFERENCES core_tenants(id) ON DELETE CASCADE,
    provider VARCHAR(50) NOT NULL,
    client_id VARCHAR(255) NOT NULL,
    client_secret_encrypted TEXT NOT NULL,
    redirect_uri VARCHAR(500) NOT NULL,
    metadata JSONB NOT NULL DEFAULT '{}',
    is_enabled BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(tenant_id, provider)
);

-- Audit log
CREATE TABLE core_audit_log (
    id UUID PRIMARY KEY,
    tenant_id UUID REFERENCES core_tenants(id) ON DELETE SET NULL,
    user_id UUID NOT NULL,
    action VARCHAR(50) NOT NULL,
    entity_type VARCHAR(100) NOT NULL,
    entity_id VARCHAR(100) NOT NULL,
    old_values JSONB,
    new_values JSONB,
    ip_address VARCHAR(45),
    user_agent TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_core_audit_tenant ON core_audit_log(tenant_id);
CREATE INDEX idx_core_audit_user ON core_audit_log(user_id);
CREATE INDEX idx_core_audit_entity ON core_audit_log(entity_type, entity_id);
CREATE INDEX idx_core_audit_created ON core_audit_log(created_at);

-- Notifications
CREATE TABLE core_notifications (
    id UUID PRIMARY KEY,
    tenant_id UUID REFERENCES core_tenants(id) ON DELETE CASCADE,
    user_id UUID REFERENCES core_users(id) ON DELETE CASCADE,
    channel VARCHAR(50) NOT NULL,
    priority VARCHAR(20) NOT NULL DEFAULT 'normal',
    subject VARCHAR(500) NOT NULL,
    body TEXT NOT NULL,
    metadata JSONB NOT NULL DEFAULT '{}',
    status VARCHAR(20) NOT NULL DEFAULT 'pending',
    sent_at TIMESTAMPTZ,
    read_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_core_notifications_user ON core_notifications(user_id);
CREATE INDEX idx_core_notifications_status ON core_notifications(status);

-- System settings (key-value store)
CREATE TABLE core_settings (
    id UUID PRIMARY KEY,
    tenant_id UUID REFERENCES core_tenants(id) ON DELETE CASCADE,
    key VARCHAR(255) NOT NULL,
    value JSONB NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(tenant_id, key)
);

-- Updated at trigger function
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';

-- Apply updated_at triggers
CREATE TRIGGER update_core_tenants_updated_at BEFORE UPDATE ON core_tenants
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_core_users_updated_at BEFORE UPDATE ON core_users
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_core_sso_configs_updated_at BEFORE UPDATE ON core_sso_configs
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_core_settings_updated_at BEFORE UPDATE ON core_settings
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
