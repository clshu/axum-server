use axum::Json;
use axum::extract::{FromRef, Path, State};
use axum::routing::{Router, delete, get, post};

use crate::model::{ModelConstroller, Ticket, TicketForCreate};
use crate::{Error, Result};

// #[derive(Clone, FromRef)]
// struct AppState {
//     mc: ModelConstroller,
// }

pub fn routes(mc: ModelConstroller) -> Router {
    // let app_state = AppState { mc };

    Router::new()
        .route("/tickets", post(create_ticket).get(list_tickets))
        .route("/tickets/{id}", delete(delete_ticket))
        .with_state(mc)
}

// region:    --- REST Handlers

async fn create_ticket(
    State(mc): State<ModelConstroller>,
    Json(ticket_fc): Json<TicketForCreate>,
) -> Result<Json<Ticket>> {
    println!("->> {:<12} - create_ticket", "HANDLER");

    let ticket = mc.create_ticket(ticket_fc).await?;

    Ok(Json(ticket))
}

async fn list_tickets(State(mc): State<ModelConstroller>) -> Result<Json<Vec<Ticket>>> {
    println!("->> {:<12} - list_ticket", "HANDLER");

    let tickets = mc.list_tickets().await?;

    Ok(Json(tickets))
}

async fn delete_ticket(
    State(mc): State<ModelConstroller>,
    Path(id): Path<u64>,
) -> Result<Json<Ticket>> {
    println!("->> {:<12} - delete_ticket", "HANDLER");

    let ticket = mc.delete_ticket(id).await?;

    Ok(Json(ticket))
}
// endregion: --- REST Handlers
