-- Ticketing module initial migration
-- Prefix: tkt_

-- Ticket status enum
CREATE TYPE ticket_status AS ENUM ('new', 'open', 'in_progress', 'pending', 'resolved', 'closed');

-- Ticket priority enum
CREATE TYPE ticket_priority AS ENUM ('low', 'medium', 'high', 'critical');

-- Ticket queues
CREATE TABLE tkt_queues (
    id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL REFERENCES core_tenants(id) ON DELETE CASCADE,
    name VARCHAR(100) NOT NULL,
    description TEXT,
    default_assignee_id UUID REFERENCES core_users(id) ON DELETE SET NULL,
    default_sla_id UUID,
    is_default BOOLEAN NOT NULL DEFAULT false,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(tenant_id, name)
);

CREATE INDEX idx_tkt_queues_tenant ON tkt_queues(tenant_id);

-- SLA definitions
CREATE TABLE tkt_slas (
    id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL REFERENCES core_tenants(id) ON DELETE CASCADE,
    name VARCHAR(100) NOT NULL,
    description TEXT,
    first_response_minutes INTEGER NOT NULL DEFAULT 60,
    resolution_minutes INTEGER NOT NULL DEFAULT 480,
    business_hours_only BOOLEAN NOT NULL DEFAULT true,
    is_default BOOLEAN NOT NULL DEFAULT false,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(tenant_id, name)
);

CREATE INDEX idx_tkt_slas_tenant ON tkt_slas(tenant_id);

-- Add foreign key for default SLA in queues
ALTER TABLE tkt_queues ADD CONSTRAINT fk_tkt_queues_sla FOREIGN KEY (default_sla_id) REFERENCES tkt_slas(id) ON DELETE SET NULL;

-- Main tickets table
CREATE TABLE tkt_tickets (
    id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL REFERENCES core_tenants(id) ON DELETE CASCADE,
    ticket_number INTEGER NOT NULL,
    subject VARCHAR(500) NOT NULL,
    description TEXT NOT NULL,
    status ticket_status NOT NULL DEFAULT 'new',
    priority ticket_priority NOT NULL DEFAULT 'medium',
    requester_id UUID NOT NULL REFERENCES core_users(id) ON DELETE RESTRICT,
    assignee_id UUID REFERENCES core_users(id) ON DELETE SET NULL,
    queue_id UUID REFERENCES tkt_queues(id) ON DELETE SET NULL,
    sla_id UUID REFERENCES tkt_slas(id) ON DELETE SET NULL,
    due_date TIMESTAMPTZ,
    first_response_at TIMESTAMPTZ,
    resolved_at TIMESTAMPTZ,
    closed_at TIMESTAMPTZ,
    tags TEXT[] NOT NULL DEFAULT '{}',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(tenant_id, ticket_number)
);

CREATE INDEX idx_tkt_tickets_tenant ON tkt_tickets(tenant_id);
CREATE INDEX idx_tkt_tickets_status ON tkt_tickets(status);
CREATE INDEX idx_tkt_tickets_priority ON tkt_tickets(priority);
CREATE INDEX idx_tkt_tickets_assignee ON tkt_tickets(assignee_id);
CREATE INDEX idx_tkt_tickets_requester ON tkt_tickets(requester_id);
CREATE INDEX idx_tkt_tickets_queue ON tkt_tickets(queue_id);
CREATE INDEX idx_tkt_tickets_due ON tkt_tickets(due_date) WHERE due_date IS NOT NULL;
CREATE INDEX idx_tkt_tickets_created ON tkt_tickets(created_at);

-- Ticket comments
CREATE TABLE tkt_ticket_comments (
    id UUID PRIMARY KEY,
    ticket_id UUID NOT NULL REFERENCES tkt_tickets(id) ON DELETE CASCADE,
    author_id UUID NOT NULL REFERENCES core_users(id) ON DELETE RESTRICT,
    content TEXT NOT NULL,
    is_internal BOOLEAN NOT NULL DEFAULT false,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_tkt_comments_ticket ON tkt_ticket_comments(ticket_id);
CREATE INDEX idx_tkt_comments_created ON tkt_ticket_comments(created_at);

-- Ticket attachments
CREATE TABLE tkt_ticket_attachments (
    id UUID PRIMARY KEY,
    ticket_id UUID NOT NULL REFERENCES tkt_tickets(id) ON DELETE CASCADE,
    comment_id UUID REFERENCES tkt_ticket_comments(id) ON DELETE CASCADE,
    filename VARCHAR(255) NOT NULL,
    content_type VARCHAR(100) NOT NULL,
    size_bytes BIGINT NOT NULL,
    storage_path VARCHAR(500) NOT NULL,
    uploaded_by UUID NOT NULL REFERENCES core_users(id) ON DELETE RESTRICT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_tkt_attachments_ticket ON tkt_ticket_attachments(ticket_id);

-- Email mappings (for email-to-ticket integration)
CREATE TABLE tkt_email_mappings (
    id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL REFERENCES core_tenants(id) ON DELETE CASCADE,
    email_address VARCHAR(255) NOT NULL,
    queue_id UUID REFERENCES tkt_queues(id) ON DELETE SET NULL,
    is_active BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(email_address)
);

-- Updated at triggers
CREATE TRIGGER update_tkt_queues_updated_at BEFORE UPDATE ON tkt_queues
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_tkt_slas_updated_at BEFORE UPDATE ON tkt_slas
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_tkt_tickets_updated_at BEFORE UPDATE ON tkt_tickets
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_tkt_comments_updated_at BEFORE UPDATE ON tkt_ticket_comments
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_tkt_email_mappings_updated_at BEFORE UPDATE ON tkt_email_mappings
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
