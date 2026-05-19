//! ZenTao Ticket(工单)命令模块

use crate::api::ticket::{CreateTicketRequest, UpdateTicketRequest};
use crate::cmd::common::{
    log_command, print_dry_run, print_dry_run_with_body, print_error, print_json,
};
use crate::cmd::root::TicketSubcommand;
use crate::core::{AppContext, OutputFormat};
use crate::service::ticket::TicketService;

pub async fn run(cmd: &TicketSubcommand, ctx: &AppContext) {
    log_command("ticket", format!("{:?}", cmd));
    match cmd {
        TicketSubcommand::List => {
            if ctx.dry_run {
                print_dry_run(
                    "TicketService::list()",
                    &format!("{}/api.php/v1/tickets", ctx.config.url),
                );
                return;
            }
            match TicketService::list(ctx, None, None, 1, 20).await {
                Ok(tickets) => print_ticket_list(&tickets, ctx.format.clone()),
                Err(e) => print_error(&e),
            }
        }
        TicketSubcommand::Get { id } => {
            if ctx.dry_run {
                print_dry_run(
                    "TicketService::get()",
                    &format!("{}/api.php/v1/tickets/{}", ctx.config.url, id),
                );
                return;
            }
            match TicketService::get(ctx, *id).await {
                Ok(ticket) => print_json(&ticket),
                Err(e) => print_error(&e),
            }
        }
        TicketSubcommand::Create {
            product,
            title,
            type_,
            desc,
        } => {
            let product_id = match ctx.require_product_id(*product) {
                Ok(id) => id,
                Err(e) => {
                    print_error(&e);
                    return;
                }
            };
            let req = CreateTicketRequest {
                product: product_id,
                title: title.clone(),
                module: None,
                type_: type_.clone(),
            };
            if ctx.dry_run {
                print_dry_run_with_body(
                    "TicketService::create()",
                    &format!("{}/api.php/v1/tickets", ctx.config.url),
                    &req,
                );
                return;
            }
            match TicketService::create(ctx, req).await {
                Ok(ticket) => print_json(&ticket),
                Err(e) => print_error(&e),
            }
        }
        TicketSubcommand::Update { id, title, desc } => {
            let req = UpdateTicketRequest {
                product: None,
                module: None,
                title: title.clone(),
                type_: None,
                desc: desc.clone(),
            };
            if ctx.dry_run {
                print_dry_run_with_body(
                    "TicketService::update()",
                    &format!("{}/api.php/v1/tickets/{}", ctx.config.url, id),
                    &req,
                );
                return;
            }
            match TicketService::update(ctx, *id, req).await {
                Ok(ticket) => print_json(&ticket),
                Err(e) => print_error(&e),
            }
        }
        TicketSubcommand::Delete { id } => {
            if ctx.dry_run {
                print_dry_run(
                    "TicketService::delete()",
                    &format!("{}/api.php/v1/tickets/{}", ctx.config.url, id),
                );
                return;
            }
            match TicketService::delete(ctx, *id).await {
                Ok(_) => {
                    println!("Ticket [{}] deleted successfully", id);
                }
                Err(e) => print_error(&e),
            }
        }
    }
}

fn print_ticket_list(tickets: &[crate::api::Ticket], format: OutputFormat) {
    match format {
        OutputFormat::Table => {
            println!("Tickets:");
            for item in tickets {
                println!(
                    "  [{}] {} (pri: {}) - {}",
                    item.id, item.title, item.pri, item.status
                );
            }
        }
        _ => println!(
            "{}",
            serde_json::to_string_pretty(tickets).unwrap_or_default()
        ),
    }
}
