//! Ticketing API routes

use axum::{
    routing::{delete, get, post, put},
    Router,
};

use crate::handlers::*;

/// Create the ticketing router
///
/// This router can be mounted:
/// - Standalone: at root `/api`
/// - In suite: at `/api/tickets`
pub fn ticketing_routes<S>(state: TicketingState) -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    Router::new()
        // Ticket CRUD
        .route("/", get(list_tickets).post(create_ticket))
        .route("/:id", get(get_ticket).put(update_ticket).delete(delete_ticket))

        // Statistics
        .route("/stats", get(ticket_stats))

        // Comments
        .route("/:id/comments", get(list_comments).post(add_comment))

        .with_state(state)
}
