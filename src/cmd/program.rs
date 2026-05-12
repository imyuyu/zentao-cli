//! ZenTao Program(项目集)命令模块

use crate::cmd::common::{log_command, print_dry_run, print_error, print_json};
use crate::cmd::root::ProgramSubcommand;
use crate::core::{AppContext, OutputFormat};
use crate::service::program::ProgramService;

pub async fn run(cmd: &ProgramSubcommand, ctx: &AppContext) {
    log_command("program", format!("{:?}", cmd));
    match cmd {
        ProgramSubcommand::List => {
            if ctx.dry_run {
                print_dry_run(
                    "ProgramService::list()",
                    &format!("{}/api.php/v1/programs", ctx.config.url),
                );
                return;
            }
            match ProgramService::list(ctx).await {
                Ok(programs) => print_program_list(&programs, ctx.format.clone()),
                Err(e) => print_error(&e),
            }
        }
        ProgramSubcommand::Get { id } => {
            if ctx.dry_run {
                print_dry_run(
                    "ProgramService::get()",
                    &format!("{}/api.php/v1/programs/{}", ctx.config.url, id),
                );
                return;
            }
            match ProgramService::get(ctx, *id).await {
                Ok(program) => print_json(&program),
                Err(e) => print_error(&e),
            }
        }
    }
}

fn print_program_list(programs: &[crate::api::Program], format: OutputFormat) {
    match format {
        OutputFormat::Table => {
            println!("Programs:");
            for item in programs {
                println!("  [{}] {} ({}) - {}", item.id, item.name, item.code, item.status);
            }
        }
        _ => println!(
            "{}",
            serde_json::to_string_pretty(programs).unwrap_or_default()
        ),
    }
}
