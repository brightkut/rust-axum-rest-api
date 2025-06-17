use crate::{
    Result,
    model::{CreateTicketDto, Ticket, TicketController},
};
use axum::{
    Json, Router,
    extract::{FromRef, Path, State},
    routing::{delete, post},
};

#[derive(Clone, FromRef)]
struct AppState {
    tc: TicketController,
}

pub fn routes(tc: TicketController) -> Router {
    let app_state = AppState { tc };

    Router::new()
        .route("/tickets", post(create_ticket).get(find_all_tickets))
        .route("/tickets/:id", delete(delete_ticket))
        .with_state(app_state)
}

async fn create_ticket(
    State(tc): State<TicketController>,
    Json(dto): Json<CreateTicketDto>,
) -> Result<Json<Ticket>> {
    println!("Request to Create ticket Handler");

    let ticket = tc.create_ticket(dto).await?;

    Ok(Json(ticket))
}

async fn find_all_tickets(State(tc): State<TicketController>) -> Result<Json<Vec<Ticket>>> {
    println!("Request to Find all tickets Handler");
    let tickets = tc.find_all_tickets().await?;

    Ok(Json(tickets))
}

async fn delete_ticket(
    State(tc): State<TicketController>,
    Path(id): Path<u64>,
) -> Result<Json<Ticket>> {
    println!("Request to Delete ticket Handler");

    let ticket = tc.delete_ticket(id).await?;
    Ok(Json(ticket))
}
