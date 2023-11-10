use axum::{extract::State, Json};
use crate::model::{ModelController, Ticket, TicketForCreate};
use crate::error::{  Error,Result };

async fn create_ticket(
    mc:State<ModelController>,
    ticket_fc:Json<TicketForCreate>
) -> Result<Json<Ticket>> {


}
