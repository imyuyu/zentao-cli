//! ZenTao Testtask(测试单)命令模块

use crate::cmd::common::{log_command, print_dry_run, print_error, print_json};
use crate::cmd::root::TesttaskSubcommand;
use crate::core::{AppContext, OutputFormat};
use crate::service::testtask::TesttaskService;

pub async fn run(cmd: &TesttaskSubcommand, ctx: &AppContext) {
    log_command("testtask", format!("{:?}", cmd));
    match cmd {
        TesttaskSubcommand::List { project } => {
            if ctx.dry_run {
                if let Some(pid) = project {
                    print_dry_run(
                        "TesttaskService::list_by_project()",
                        &format!("{}/api.php/v1/projects/{}/testtasks", ctx.config.url, pid),
                    );
                } else {
                    print_dry_run(
                        "TesttaskService::list()",
                        &format!("{}/api.php/v1/testtasks", ctx.config.url),
                    );
                }
                return;
            }
            match if let Some(pid) = project {
                TesttaskService::list_by_project(ctx, *pid, 1, 100).await
            } else {
                TesttaskService::list(ctx, 1, 100).await
            } {
                Ok(testtasks) => print_testtask_list(&testtasks, ctx.format.clone()),
                Err(e) => print_error(&e),
            }
        }
        TesttaskSubcommand::Get { id } => {
            if ctx.dry_run {
                print_dry_run(
                    "TesttaskService::get()",
                    &format!("{}/api.php/v1/testtasks/{}", ctx.config.url, id),
                );
                return;
            }
            match TesttaskService::get(ctx, *id).await {
                Ok(testtask) => print_json(&testtask),
                Err(e) => print_error(&e),
            }
        }
    }
}

fn print_testtask_list(testtasks: &[crate::api::Testtask], format: OutputFormat) {
    match format {
        OutputFormat::Table => {
            println!("Test Tasks:");
            for item in testtasks {
                println!("  [{}] {} - {}", item.id, item.name, item.status);
            }
        }
        _ => println!(
            "{}",
            serde_json::to_string_pretty(testtasks).unwrap_or_default()
        ),
    }
}
