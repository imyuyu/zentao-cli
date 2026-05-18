use crate::api::{Ticket, TicketApi};
use crate::core::logging::{log, LogLevel};
use crate::core::AppContext;
use anyhow::Result;

pub struct TicketService;

impl TicketService {
    pub async fn list(
        ctx: &AppContext,
        browse_type: Option<String>,
        order_by: Option<String>,
        page: u32,
        limit: u32,
    ) -> Result<Vec<Ticket>> {
        log(LogLevel::Info, "TicketService", "list");
        let client = ctx.client();
        TicketApi::list(&client, browse_type, order_by, page, limit).await
    }

    pub async fn get(ctx: &AppContext, id: u64) -> Result<Ticket> {
        log(LogLevel::Info, "TicketService", format!("get id={}", id));
        let client = ctx.client();
        TicketApi::get(&client, id).await
    }

    pub async fn create(
        ctx: &AppContext,
        req: crate::api::ticket::CreateTicketRequest,
    ) -> Result<Ticket> {
        log(LogLevel::Info, "TicketService", "create");
        let client = ctx.client();
        TicketApi::create(&client, &req).await
    }

    pub async fn update(
        ctx: &AppContext,
        id: u64,
        req: crate::api::ticket::UpdateTicketRequest,
    ) -> Result<Ticket> {
        log(LogLevel::Info, "TicketService", format!("update id={}", id));
        let client = ctx.client();
        TicketApi::update(&client, id, &req).await
    }

    pub async fn delete(ctx: &AppContext, id: u64) -> Result<()> {
        log(LogLevel::Info, "TicketService", format!("delete id={}", id));
        let client = ctx.client();
        TicketApi::delete(&client, id).await
    }
}
