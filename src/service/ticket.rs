use crate::api::{Ticket, TicketApi};
use crate::core::logging::{log, LogLevel};
use crate::core::AppContext;
use anyhow::Result;

pub struct TicketService;

impl TicketService {
    pub async fn list(ctx: &AppContext, page: u32, limit: u32) -> Result<Vec<Ticket>> {
        log(LogLevel::Info, "TicketService", "list");
        let client = ctx.client();
        TicketApi::list(&client, page, limit).await
    }

    pub async fn get(ctx: &AppContext, id: u64) -> Result<Ticket> {
        log(LogLevel::Info, "TicketService", format!("get id={}", id));
        let client = ctx.client();
        TicketApi::get(&client, id).await
    }
}
