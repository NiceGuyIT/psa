//! HTTP handlers for ticketing API

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use uuid::Uuid;

use psa_core::{
    error::CoreError,
    models::{Pagination, PaginatedResponse, UserContext},
};

use crate::{
    models::*,
    repository::{CommentRepository, TicketRepository},
};

/// Application state for ticketing handlers
#[derive(Clone)]
pub struct TicketingState {
    pub db: psa_core::db::Database,
}

/// List tickets
pub async fn list_tickets(
    State(state): State<TicketingState>,
    user: axum::Extension<UserContext>,
    Query(filters): Query<TicketFilters>,
    Query(pagination): Query<Pagination>,
) -> Result<Json<PaginatedResponse<Ticket>>, CoreError> {
    let tenant_id = user.tenant_id.ok_or_else(|| {
        CoreError::AuthorizationDenied("Tenant context required".to_string())
    })?;

    let repo = TicketRepository::new(state.db.pool());
    let (tickets, total) = repo.list(tenant_id, &filters, &pagination).await?;

    Ok(Json(PaginatedResponse::new(tickets, total as u64, &pagination)))
}

/// Get a single ticket
pub async fn get_ticket(
    State(state): State<TicketingState>,
    user: axum::Extension<UserContext>,
    Path(id): Path<Uuid>,
) -> Result<Json<Ticket>, CoreError> {
    let tenant_id = user.tenant_id.ok_or_else(|| {
        CoreError::AuthorizationDenied("Tenant context required".to_string())
    })?;

    let repo = TicketRepository::new(state.db.pool());
    let ticket = repo
        .find_by_id(tenant_id, id)
        .await?
        .ok_or_else(|| CoreError::NotFound(format!("Ticket {} not found", id)))?;

    Ok(Json(ticket))
}

/// Create a new ticket
pub async fn create_ticket(
    State(state): State<TicketingState>,
    user: axum::Extension<UserContext>,
    Json(request): Json<CreateTicketRequest>,
) -> Result<(StatusCode, Json<Ticket>), CoreError> {
    let tenant_id = user.tenant_id.ok_or_else(|| {
        CoreError::AuthorizationDenied("Tenant context required".to_string())
    })?;

    let repo = TicketRepository::new(state.db.pool());
    let ticket = repo.create(tenant_id, user.user_id, request).await?;

    Ok((StatusCode::CREATED, Json(ticket)))
}

/// Update a ticket
pub async fn update_ticket(
    State(state): State<TicketingState>,
    user: axum::Extension<UserContext>,
    Path(id): Path<Uuid>,
    Json(request): Json<UpdateTicketRequest>,
) -> Result<Json<Ticket>, CoreError> {
    let tenant_id = user.tenant_id.ok_or_else(|| {
        CoreError::AuthorizationDenied("Tenant context required".to_string())
    })?;

    let repo = TicketRepository::new(state.db.pool());
    let ticket = repo
        .update(tenant_id, id, request)
        .await?
        .ok_or_else(|| CoreError::NotFound(format!("Ticket {} not found", id)))?;

    Ok(Json(ticket))
}

/// Delete a ticket
pub async fn delete_ticket(
    State(state): State<TicketingState>,
    user: axum::Extension<UserContext>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, CoreError> {
    let tenant_id = user.tenant_id.ok_or_else(|| {
        CoreError::AuthorizationDenied("Tenant context required".to_string())
    })?;

    let repo = TicketRepository::new(state.db.pool());
    let deleted = repo.delete(tenant_id, id).await?;

    if deleted {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(CoreError::NotFound(format!("Ticket {} not found", id)))
    }
}

/// Get ticket statistics
pub async fn ticket_stats(
    State(state): State<TicketingState>,
    user: axum::Extension<UserContext>,
) -> Result<Json<TicketStats>, CoreError> {
    let tenant_id = user.tenant_id.ok_or_else(|| {
        CoreError::AuthorizationDenied("Tenant context required".to_string())
    })?;

    let repo = TicketRepository::new(state.db.pool());
    let stats = repo.stats(tenant_id).await?;

    Ok(Json(stats))
}

/// Add comment request
#[derive(serde::Deserialize)]
pub struct AddCommentRequest {
    pub content: String,
    #[serde(default)]
    pub is_internal: bool,
}

/// List ticket comments
pub async fn list_comments(
    State(state): State<TicketingState>,
    user: axum::Extension<UserContext>,
    Path(ticket_id): Path<Uuid>,
) -> Result<Json<Vec<TicketComment>>, CoreError> {
    let include_internal = user.role.can_write();

    let repo = CommentRepository::new(state.db.pool());
    let comments = repo.list_for_ticket(ticket_id, include_internal).await?;

    Ok(Json(comments))
}

/// Add a comment to a ticket
pub async fn add_comment(
    State(state): State<TicketingState>,
    user: axum::Extension<UserContext>,
    Path(ticket_id): Path<Uuid>,
    Json(request): Json<AddCommentRequest>,
) -> Result<(StatusCode, Json<TicketComment>), CoreError> {
    let repo = CommentRepository::new(state.db.pool());
    let comment = repo
        .create(ticket_id, user.user_id, request.content, request.is_internal)
        .await?;

    Ok((StatusCode::CREATED, Json(comment)))
}
