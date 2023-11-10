use crate::ctx::Ctx;
use crate::error::Result;
use crate::model::{ModelController, Ticket, TicketForCreate};
use axum::extract::Path;
use axum::routing::{delete, post};
use axum::Router;
use axum::{extract::State, Json};

pub fn routes(mc: ModelController) -> Router {
    Router::new()
        .route("/tickets", post(create_ticket).get(list_tickets))
        .route("/tickets/:id", delete(delete_ticket))
        .with_state(mc)
}

async fn create_ticket(
    State(mc): State<ModelController>,
    ctx: Ctx,
    Json(ticket_fc): Json<TicketForCreate>,
) -> Result<Json<Ticket>> {
    println!("--->> {:<12} -  create_ticket", "API_TICKET_HANDLER");
    let ticket = mc.create_ticket(ctx, ticket_fc).await?;

    Ok(Json(ticket))
}

async fn list_tickets(mc: State<ModelController>, ctx: Ctx) -> Result<Json<Vec<Ticket>>> {
    println!("--->> {:<12} -  list_ticket", "API_TICKET_HANDLER");

    let tickets = mc.list_tickets(ctx).await?;
    Ok(Json(tickets))
}

async fn delete_ticket(
    mc: State<ModelController>,
    ctx: Ctx,
    Path(id): Path<u64>,
) -> Result<Json<Ticket>> {
    println!("--->> {:<12} -  delete_ticket", "API_TICKET_HANDLER");

    let ticket = mc.delete_ticket(ctx, id).await?;
    Ok(Json(ticket))
}
