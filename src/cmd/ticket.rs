//! ZenTao Ticket(工单)命令模块

use crate::cmd::common::{log_command, print_dry_run, print_error, print_json};
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
