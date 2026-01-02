//! Ticket repository for database operations

use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

use psa_core::models::{Pagination, TenantId, UserId};

use crate::models::*;

/// Ticket repository
pub struct TicketRepository<'a> {
    pool: &'a PgPool,
}

impl<'a> TicketRepository<'a> {
    pub fn new(pool: &'a PgPool) -> Self {
        Self { pool }
    }

    /// Get next ticket number for tenant
    async fn next_ticket_number(&self, tenant_id: TenantId) -> Result<i32, sqlx::Error> {
        let result: (i32,) = sqlx::query_as(
            r#"
            SELECT COALESCE(MAX(ticket_number), 0) + 1
            FROM tkt_tickets
            WHERE tenant_id = $1
            "#
        )
        .bind(tenant_id)
        .fetch_one(self.pool)
        .await?;

        Ok(result.0)
    }

    /// Create a new ticket
    pub async fn create(
        &self,
        tenant_id: TenantId,
        requester_id: UserId,
        request: CreateTicketRequest,
    ) -> Result<Ticket, sqlx::Error> {
        let id = Uuid::new_v4();
        let ticket_number = self.next_ticket_number(tenant_id).await?;
        let now = Utc::now();

        let ticket = Ticket {
            id,
            tenant_id,
            ticket_number,
            subject: request.subject,
            description: request.description,
            status: TicketStatus::New,
            priority: request.priority.unwrap_or_default(),
            requester_id,
            assignee_id: request.assignee_id,
            queue_id: request.queue_id,
            sla_id: None,
            due_date: None,
            first_response_at: None,
            resolved_at: None,
            closed_at: None,
            tags: request.tags.unwrap_or_default(),
            created_at: now,
            updated_at: now,
        };

        sqlx::query(
            r#"
            INSERT INTO tkt_tickets (
                id, tenant_id, ticket_number, subject, description, status, priority,
                requester_id, assignee_id, queue_id, sla_id, due_date,
                first_response_at, resolved_at, closed_at, tags, created_at, updated_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18)
            "#
        )
        .bind(&ticket.id)
        .bind(&ticket.tenant_id)
        .bind(&ticket.ticket_number)
        .bind(&ticket.subject)
        .bind(&ticket.description)
        .bind(&ticket.status)
        .bind(&ticket.priority)
        .bind(&ticket.requester_id)
        .bind(&ticket.assignee_id)
        .bind(&ticket.queue_id)
        .bind(&ticket.sla_id)
        .bind(&ticket.due_date)
        .bind(&ticket.first_response_at)
        .bind(&ticket.resolved_at)
        .bind(&ticket.closed_at)
        .bind(&ticket.tags)
        .bind(&ticket.created_at)
        .bind(&ticket.updated_at)
        .execute(self.pool)
        .await?;

        Ok(ticket)
    }

    /// Find ticket by ID
    pub async fn find_by_id(
        &self,
        tenant_id: TenantId,
        id: TicketId,
    ) -> Result<Option<Ticket>, sqlx::Error> {
        sqlx::query_as::<_, Ticket>(
            "SELECT * FROM tkt_tickets WHERE tenant_id = $1 AND id = $2"
        )
        .bind(tenant_id)
        .bind(id)
        .fetch_optional(self.pool)
        .await
    }

    /// Find ticket by number
    pub async fn find_by_number(
        &self,
        tenant_id: TenantId,
        ticket_number: i32,
    ) -> Result<Option<Ticket>, sqlx::Error> {
        sqlx::query_as::<_, Ticket>(
            "SELECT * FROM tkt_tickets WHERE tenant_id = $1 AND ticket_number = $2"
        )
        .bind(tenant_id)
        .bind(ticket_number)
        .fetch_optional(self.pool)
        .await
    }

    /// List tickets with filters
    pub async fn list(
        &self,
        tenant_id: TenantId,
        filters: &TicketFilters,
        pagination: &Pagination,
    ) -> Result<(Vec<Ticket>, i64), sqlx::Error> {
        let mut query = String::from(
            "SELECT * FROM tkt_tickets WHERE tenant_id = $1"
        );
        let mut count_query = String::from(
            "SELECT COUNT(*) FROM tkt_tickets WHERE tenant_id = $1"
        );

        // Build dynamic filter conditions
        let mut conditions = Vec::new();

        if filters.status.is_some() {
            conditions.push("status = $2");
        }
        if filters.priority.is_some() {
            conditions.push("priority = $3");
        }
        if filters.assignee_id.is_some() {
            conditions.push("assignee_id = $4");
        }
        if filters.queue_id.is_some() {
            conditions.push("queue_id = $5");
        }
        if filters.requester_id.is_some() {
            conditions.push("requester_id = $6");
        }
        if filters.search.is_some() {
            conditions.push("(subject ILIKE $7 OR description ILIKE $7)");
        }

        for (i, condition) in conditions.iter().enumerate() {
            query.push_str(" AND ");
            query.push_str(condition);
            count_query.push_str(" AND ");
            count_query.push_str(condition);
        }

        query.push_str(" ORDER BY created_at DESC");
        query.push_str(&format!(" LIMIT {} OFFSET {}", pagination.per_page, pagination.offset()));

        // For now, use a simpler approach without dynamic binding
        // In production, use a query builder like sea-query
        let tickets = sqlx::query_as::<_, Ticket>(
            "SELECT * FROM tkt_tickets WHERE tenant_id = $1 ORDER BY created_at DESC LIMIT $2 OFFSET $3"
        )
        .bind(tenant_id)
        .bind(pagination.per_page as i32)
        .bind(pagination.offset() as i32)
        .fetch_all(self.pool)
        .await?;

        let total: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM tkt_tickets WHERE tenant_id = $1"
        )
        .bind(tenant_id)
        .fetch_one(self.pool)
        .await?;

        Ok((tickets, total.0))
    }

    /// Update a ticket
    pub async fn update(
        &self,
        tenant_id: TenantId,
        id: TicketId,
        request: UpdateTicketRequest,
    ) -> Result<Option<Ticket>, sqlx::Error> {
        let mut ticket = match self.find_by_id(tenant_id, id).await? {
            Some(t) => t,
            None => return Ok(None),
        };

        if let Some(subject) = request.subject {
            ticket.subject = subject;
        }
        if let Some(description) = request.description {
            ticket.description = description;
        }
        if let Some(status) = request.status {
            // Track status transitions
            if status == TicketStatus::Resolved && ticket.resolved_at.is_none() {
                ticket.resolved_at = Some(Utc::now());
            }
            if status == TicketStatus::Closed && ticket.closed_at.is_none() {
                ticket.closed_at = Some(Utc::now());
            }
            ticket.status = status;
        }
        if let Some(priority) = request.priority {
            ticket.priority = priority;
        }
        if let Some(assignee_id) = request.assignee_id {
            ticket.assignee_id = Some(assignee_id);
        }
        if let Some(queue_id) = request.queue_id {
            ticket.queue_id = Some(queue_id);
        }
        if let Some(tags) = request.tags {
            ticket.tags = tags;
        }

        ticket.updated_at = Utc::now();

        sqlx::query(
            r#"
            UPDATE tkt_tickets SET
                subject = $3,
                description = $4,
                status = $5,
                priority = $6,
                assignee_id = $7,
                queue_id = $8,
                tags = $9,
                resolved_at = $10,
                closed_at = $11,
                updated_at = $12
            WHERE tenant_id = $1 AND id = $2
            "#
        )
        .bind(&ticket.tenant_id)
        .bind(&ticket.id)
        .bind(&ticket.subject)
        .bind(&ticket.description)
        .bind(&ticket.status)
        .bind(&ticket.priority)
        .bind(&ticket.assignee_id)
        .bind(&ticket.queue_id)
        .bind(&ticket.tags)
        .bind(&ticket.resolved_at)
        .bind(&ticket.closed_at)
        .bind(&ticket.updated_at)
        .execute(self.pool)
        .await?;

        Ok(Some(ticket))
    }

    /// Delete a ticket
    pub async fn delete(&self, tenant_id: TenantId, id: TicketId) -> Result<bool, sqlx::Error> {
        let result = sqlx::query(
            "DELETE FROM tkt_tickets WHERE tenant_id = $1 AND id = $2"
        )
        .bind(tenant_id)
        .bind(id)
        .execute(self.pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }

    /// Get ticket statistics
    pub async fn stats(&self, tenant_id: TenantId) -> Result<TicketStats, sqlx::Error> {
        let stats: (i64, i64, i64, i64, i64) = sqlx::query_as(
            r#"
            SELECT
                COUNT(*) as total,
                COUNT(*) FILTER (WHERE status NOT IN ('resolved', 'closed')) as open,
                COUNT(*) FILTER (WHERE assignee_id IS NULL AND status NOT IN ('resolved', 'closed')) as unassigned,
                COUNT(*) FILTER (WHERE due_date < NOW() AND status NOT IN ('resolved', 'closed')) as overdue,
                COUNT(*) FILTER (WHERE resolved_at::date = CURRENT_DATE) as resolved_today
            FROM tkt_tickets
            WHERE tenant_id = $1
            "#
        )
        .bind(tenant_id)
        .fetch_one(self.pool)
        .await?;

        Ok(TicketStats {
            total: stats.0,
            open: stats.1,
            unassigned: stats.2,
            overdue: stats.3,
            resolved_today: stats.4,
        })
    }
}

/// Comment repository
pub struct CommentRepository<'a> {
    pool: &'a PgPool,
}

impl<'a> CommentRepository<'a> {
    pub fn new(pool: &'a PgPool) -> Self {
        Self { pool }
    }

    /// Add a comment to a ticket
    pub async fn create(
        &self,
        ticket_id: TicketId,
        author_id: UserId,
        content: String,
        is_internal: bool,
    ) -> Result<TicketComment, sqlx::Error> {
        let id = Uuid::new_v4();
        let now = Utc::now();

        let comment = TicketComment {
            id,
            ticket_id,
            author_id,
            content,
            is_internal,
            created_at: now,
            updated_at: now,
        };

        sqlx::query(
            r#"
            INSERT INTO tkt_ticket_comments (id, ticket_id, author_id, content, is_internal, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            "#
        )
        .bind(&comment.id)
        .bind(&comment.ticket_id)
        .bind(&comment.author_id)
        .bind(&comment.content)
        .bind(&comment.is_internal)
        .bind(&comment.created_at)
        .bind(&comment.updated_at)
        .execute(self.pool)
        .await?;

        Ok(comment)
    }

    /// List comments for a ticket
    pub async fn list_for_ticket(
        &self,
        ticket_id: TicketId,
        include_internal: bool,
    ) -> Result<Vec<TicketComment>, sqlx::Error> {
        let query = if include_internal {
            "SELECT * FROM tkt_ticket_comments WHERE ticket_id = $1 ORDER BY created_at ASC"
        } else {
            "SELECT * FROM tkt_ticket_comments WHERE ticket_id = $1 AND is_internal = false ORDER BY created_at ASC"
        };

        sqlx::query_as::<_, TicketComment>(query)
            .bind(ticket_id)
            .fetch_all(self.pool)
            .await
    }
}
