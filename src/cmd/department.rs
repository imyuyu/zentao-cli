//! ZenTao Department(部门)命令模块

use crate::cmd::common::{log_command, print_dry_run, print_error, print_json};
use crate::cmd::root::DepartmentSubcommand;
use crate::core::{AppContext, OutputFormat};
use crate::service::department::DepartmentService;

pub async fn run(cmd: &DepartmentSubcommand, ctx: &AppContext) {
    log_command("department", format!("{:?}", cmd));
    match cmd {
        DepartmentSubcommand::List => {
            if ctx.dry_run {
                print_dry_run(
                    "DepartmentService::list()",
                    &format!("{}/api.php/v1/departments", ctx.config.url),
                );
                return;
            }
            match DepartmentService::list(ctx).await {
                Ok(departments) => print_department_list(&departments, ctx.format.clone()),
                Err(e) => print_error(&e),
            }
        }
        DepartmentSubcommand::Get { id } => {
            if ctx.dry_run {
                print_dry_run(
                    "DepartmentService::get()",
                    &format!("{}/api.php/v1/departments/{}", ctx.config.url, id),
                );
                return;
            }
            match DepartmentService::get(ctx, *id).await {
                Ok(department) => print_json(&department),
                Err(e) => print_error(&e),
            }
        }
    }
}

fn print_department_list(departments: &[crate::api::Department], format: OutputFormat) {
    match format {
        OutputFormat::Table => {
            println!("Departments:");
            for item in departments {
                let parent = item
                    .parent
                    .map(|p| p.to_string())
                    .unwrap_or_else(|| "N/A".to_string());
                println!("  [{}] {} (parent: {})", item.id, item.name, parent);
            }
        }
        _ => println!(
            "{}",
            serde_json::to_string_pretty(departments).unwrap_or_default()
        ),
    }
}
