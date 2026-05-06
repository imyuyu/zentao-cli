//! ZenTao Doc(文档)命令模块

use clap::Subcommand;

use crate::cmd::common::{log_command, print_dry_run, print_error, print_json};
use crate::core::{AppContext, OutputFormat};
use crate::service::doc::DocService;

#[derive(Subcommand, Clone, Debug)]
pub enum DocAction {
    #[command(name = "list")]
    List,
    #[command(name = "get")]
    Get { id: u64 },
}

pub async fn run(cmd: &DocAction, ctx: &AppContext) {
    log_command("doc", format!("{:?}", cmd));
    match cmd {
        DocAction::List => {
            if ctx.dry_run {
                print_dry_run(
                    "DocService::list()",
                    &format!("{}/api.php/v1/docs", ctx.config.url),
                );
                return;
            }
            match DocService::list(ctx).await {
                Ok(docs) => print_doc_list(&docs, ctx.format.clone()),
                Err(e) => print_error(&e),
            }
        }
        DocAction::Get { id } => {
            if ctx.dry_run {
                print_dry_run(
                    "DocService::get()",
                    &format!("{}/api.php/v1/docs/{}", ctx.config.url, id),
                );
                return;
            }
            match DocService::get(ctx, *id).await {
                Ok(doc) => print_json(&doc),
                Err(e) => print_error(&e),
            }
        }
    }
}

fn print_doc_list(items: &[crate::api::Doc], format: OutputFormat) {
    match format {
        OutputFormat::Table => {
            println!("Docs:");
            for item in items {
                println!("  [{}] {}", item.id, item.title);
            }
        }
        _ => println!(
            "{}",
            serde_json::to_string_pretty(items).unwrap_or_default()
        ),
    }
}
