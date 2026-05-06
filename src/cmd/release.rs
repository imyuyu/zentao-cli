//! ZenTao Release(发布)命令模块

use crate::cmd::common::{log_command, print_dry_run, print_error, print_json};
use crate::cmd::root::ReleaseSubcommand;
use crate::core::{AppContext, OutputFormat};
use crate::service::release::ReleaseService;

pub async fn run(cmd: &ReleaseSubcommand, ctx: &AppContext) {
    log_command("release", format!("{:?}", cmd));
    match cmd {
        ReleaseSubcommand::List { product, project } => {
            let product_id = ctx.product_id(*product);
            let project_id = ctx.project_id(*project);
            if ctx.dry_run {
                if let Some(pid) = product_id {
                    print_dry_run(
                        "ReleaseService::list() via product",
                        &format!("{}/api.php/v1/products/{}/releases", ctx.config.url, pid),
                    );
                } else if let Some(pid) = project_id {
                    print_dry_run(
                        "ReleaseService::list() via project",
                        &format!("{}/api.php/v1/projects/{}/releases", ctx.config.url, pid),
                    );
                } else {
                    print_dry_run(
                        "ReleaseService::list()",
                        &format!("{}/api.php/v1/releases", ctx.config.url),
                    );
                }
                return;
            }
            match ReleaseService::list(ctx, *product, *project).await {
                Ok(releases) => print_release_list(&releases, ctx.format.clone()),
                Err(e) => print_error(&e),
            }
        }
        ReleaseSubcommand::Get { id } => {
            if ctx.dry_run {
                print_dry_run(
                    "ReleaseService::get()",
                    &format!("{}/api.php/v1/releases/{}", ctx.config.url, id),
                );
                return;
            }
            match ReleaseService::get(ctx, *id).await {
                Ok(release) => print_json(&release),
                Err(e) => print_error(&e),
            }
        }
    }
}

fn print_release_list(releases: &[crate::api::Release], format: OutputFormat) {
    match format {
        OutputFormat::Table => {
            println!("Releases:");
            for item in releases {
                println!("  [{}] {} - {}", item.id, item.name, item.status);
            }
        }
        _ => println!(
            "{}",
            serde_json::to_string_pretty(releases).unwrap_or_default()
        ),
    }
}
