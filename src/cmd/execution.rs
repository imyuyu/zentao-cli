//! ZenTao Execution(执行)命令模块

use clap::Subcommand;

use crate::api::execution::{CreateExecutionRequest, UpdateExecutionRequest};
use crate::cmd::common::{
    log_command, print_deleted, print_dry_run, print_dry_run_with_body, print_error, print_json,
};
use crate::core::{AppContext, OutputFormat};
use crate::service::execution::ExecutionService;

#[derive(Subcommand, Clone, Debug)]
pub enum ExecutionAction {
    #[command(name = "list")]
    List {
        #[arg(long)]
        project: Option<u64>,
    },
    #[command(name = "get")]
    Get { id: u64 },
    #[command(name = "create")]
    Create {
        #[arg(long)]
        name: String,
        #[arg(long)]
        project: Option<u64>,
        #[arg(long)]
        type_: Option<String>,
        #[arg(long)]
        begin: Option<String>,
        #[arg(long)]
        end: Option<String>,
        #[arg(long)]
        days: Option<u64>,
        #[arg(long)]
        desc: Option<String>,
    },
    #[command(name = "update")]
    Update {
        id: u64,
        #[arg(long)]
        name: Option<String>,
        #[arg(long)]
        status: Option<String>,
        #[arg(long)]
        begin: Option<String>,
        #[arg(long)]
        end: Option<String>,
        #[arg(long)]
        days: Option<u64>,
        #[arg(long)]
        desc: Option<String>,
    },
    #[command(name = "delete")]
    Delete { id: u64 },
}

pub async fn run(cmd: &ExecutionAction, ctx: &AppContext) {
    log_command("execution", format!("{:?}", cmd));
    match cmd {
        ExecutionAction::List { project } => {
            if ctx.dry_run {
                print_dry_run(
                    "ExecutionService::list()",
                    &format!("{}/api.php/v1/executions", ctx.config.url),
                );
                if let Some(p) = project {
                    println!("  project: {}", p);
                }
                return;
            }
            match ExecutionService::list(ctx, *project).await {
                Ok(executions) => print_execution_list(&executions, ctx.format.clone()),
                Err(e) => print_error(&e),
            }
        }
        ExecutionAction::Get { id } => {
            if ctx.dry_run {
                print_dry_run(
                    "ExecutionService::get()",
                    &format!("{}/api.php/v1/executions/{}", ctx.config.url, id),
                );
                return;
            }
            match ExecutionService::get(ctx, *id).await {
                Ok(execution) => print_json(&execution),
                Err(e) => print_error(&e),
            }
        }
        ExecutionAction::Create {
            name,
            project,
            type_,
            begin,
            end,
            days,
            desc,
        } => {
            let project_id = match ctx.require_project_id(*project) {
                Ok(id) => id,
                Err(e) => {
                    print_error(&e);
                    return;
                }
            };
            let req = CreateExecutionRequest {
                name: name.clone(),
                project: project_id,
                type_: type_.clone(),
                begin: begin.clone(),
                end: end.clone(),
                days: *days,
                desc: desc.clone(),
            };
            if ctx.dry_run {
                print_dry_run_with_body(
                    "ExecutionService::create()",
                    &format!(
                        "{}/api.php/v1/projects/{}/executions",
                        ctx.config.url, project_id
                    ),
                    &req,
                );
                return;
            }
            match ExecutionService::create(ctx, *project, req).await {
                Ok(execution) => print_json(&execution),
                Err(e) => print_error(&e),
            }
        }
        ExecutionAction::Update {
            id,
            name,
            status,
            begin,
            end,
            days,
            desc,
        } => {
            let req = UpdateExecutionRequest {
                name: name.clone(),
                status: status.clone(),
                begin: begin.clone(),
                end: end.clone(),
                days: *days,
                desc: desc.clone(),
            };
            if ctx.dry_run {
                print_dry_run_with_body(
                    "ExecutionService::update()",
                    &format!("{}/api.php/v1/executions/{}", ctx.config.url, id),
                    &req,
                );
                return;
            }
            match ExecutionService::update(ctx, *id, req).await {
                Ok(execution) => print_json(&execution),
                Err(e) => print_error(&e),
            }
        }
        ExecutionAction::Delete { id } => {
            if ctx.dry_run {
                print_dry_run(
                    "ExecutionService::delete()",
                    &format!("{}/api.php/v1/executions/{}", ctx.config.url, id),
                );
                return;
            }
            match ExecutionService::delete(ctx, *id).await {
                Ok(_) => print_deleted("Execution", *id),
                Err(e) => print_error(&e),
            }
        }
    }
}

fn print_execution_list(items: &[crate::api::Execution], format: OutputFormat) {
    match format {
        OutputFormat::Table => {
            println!("Executions:");
            for item in items {
                println!("  [{}] {} - {}", item.id, item.name, item.status);
            }
        }
        _ => println!(
            "{}",
            serde_json::to_string_pretty(items).unwrap_or_default()
        ),
    }
}
