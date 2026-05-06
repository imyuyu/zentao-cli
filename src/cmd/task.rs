//! ZenTao Task(任务)命令模块

use clap::Subcommand;

use crate::api::{CreateTaskRequest, UpdateTaskRequest};
use crate::cmd::common::{
    log_command, print_deleted, print_dry_run, print_dry_run_with_body, print_error, print_json,
};
use crate::core::{AppContext, OutputFormat};
use crate::safe_println;
use crate::service::task::TaskService;

#[derive(Subcommand, Clone, Debug)]
pub enum TaskAction {
    #[command(name = "list")]
    List {
        #[arg(long)]
        project: Option<u64>,
        #[arg(long)]
        assigned_to: Option<String>,
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
        pri: u8,
        #[arg(long)]
        type_: Option<String>,
        #[arg(long)]
        assigned_to: Option<String>,
        #[arg(long)]
        estimate: Option<f64>,
    },
    #[command(name = "update")]
    Update {
        id: u64,
        #[arg(long)]
        name: Option<String>,
        #[arg(long)]
        status: Option<String>,
        #[arg(long)]
        pri: Option<u8>,
        #[arg(long)]
        assigned_to: Option<String>,
    },
    #[command(name = "delete")]
    Delete { id: u64 },
    #[command(name = "start")]
    Start { id: u64 },
    #[command(name = "pause")]
    Pause { id: u64 },
    #[command(name = "restart")]
    Restart { id: u64 },
    #[command(name = "finish")]
    Finish { id: u64 },
    #[command(name = "close")]
    Close { id: u64 },
    #[command(name = "estimate")]
    Estimate {
        id: u64,
        #[arg(long)]
        consumed: f64,
        #[arg(long)]
        left: f64,
        #[arg(long)]
        notes: Option<String>,
    },
    #[command(name = "get-estimate")]
    GetEstimate { id: u64 },
}

pub async fn run(cmd: &TaskAction, ctx: &AppContext) {
    log_command("task", format!("{:?}", cmd));
    match cmd {
        TaskAction::List {
            project,
            assigned_to,
        } => {
            if ctx.dry_run {
                safe_println("[DRY-RUN] Would call TaskService::list()");
                safe_println("  Step 1: GET /api.php/v1/projects/{{project_id}}/executions");
                safe_println(
                    "  Step 2: For each execution, GET /api.php/v1/executions/{{id}}/tasks",
                );
                if let Some(a) = assigned_to {
                    println!("  Filter: assignedTo={}", a);
                }
                return;
            }
            match TaskService::list(ctx, *project, assigned_to.clone()).await {
                Ok(tasks) => print_task_list(&tasks, ctx.format.clone()),
                Err(e) => print_error(&e),
            }
        }
        TaskAction::Get { id } => handle_json(
            ctx,
            "TaskService::get()",
            &format!("{}/api.php/v1/tasks/{}", ctx.config.url, id),
            TaskService::get(ctx, *id).await,
        ),
        TaskAction::Create {
            name,
            project,
            pri,
            type_,
            assigned_to,
            estimate,
        } => {
            let project_id = match ctx.require_project_id(*project) {
                Ok(id) => id,
                Err(e) => {
                    print_error(&e);
                    return;
                }
            };
            let req = CreateTaskRequest {
                name: name.clone(),
                project: project_id,
                pri: *pri,
                type_: type_.clone(),
                assigned_to: assigned_to.clone(),
                estimate: *estimate,
            };
            if ctx.dry_run {
                print_dry_run_with_body(
                    "TaskService::create()",
                    &format!("{}/api.php/v1/tasks", ctx.config.url),
                    &req,
                );
                return;
            }
            match TaskService::create(ctx, req).await {
                Ok(task) => print_json(&task),
                Err(e) => print_error(&e),
            }
        }
        TaskAction::Update {
            id,
            name,
            status,
            pri,
            assigned_to,
        } => {
            let req = UpdateTaskRequest {
                name: name.clone(),
                status: status.clone(),
                pri: *pri,
                assigned_to: assigned_to.clone(),
            };
            if ctx.dry_run {
                print_dry_run_with_body(
                    "TaskService::update()",
                    &format!("{}/api.php/v1/tasks/{}", ctx.config.url, id),
                    &req,
                );
                return;
            }
            match TaskService::update(ctx, *id, req).await {
                Ok(task) => print_json(&task),
                Err(e) => print_error(&e),
            }
        }
        TaskAction::Delete { id } => handle_unit(
            ctx,
            "TaskService::delete()",
            &format!("{}/api.php/v1/tasks/{}", ctx.config.url, id),
            *id,
            "Task",
            TaskService::delete(ctx, *id).await,
        ),
        TaskAction::Start { id } => handle_json(
            ctx,
            "TaskService::start()",
            &format!("{}/api.php/v1/tasks/{}/start", ctx.config.url, id),
            TaskService::start(ctx, *id).await,
        ),
        TaskAction::Pause { id } => handle_json(
            ctx,
            "TaskService::pause()",
            &format!("{}/api.php/v1/tasks/{}/pause", ctx.config.url, id),
            TaskService::pause(ctx, *id).await,
        ),
        TaskAction::Restart { id } => handle_json(
            ctx,
            "TaskService::restart()",
            &format!("{}/api.php/v1/tasks/{}/restart", ctx.config.url, id),
            TaskService::restart(ctx, *id).await,
        ),
        TaskAction::Finish { id } => handle_json(
            ctx,
            "TaskService::finish()",
            &format!("{}/api.php/v1/tasks/{}/finish", ctx.config.url, id),
            TaskService::finish(ctx, *id).await,
        ),
        TaskAction::Close { id } => handle_json(
            ctx,
            "TaskService::close()",
            &format!("{}/api.php/v1/tasks/{}/close", ctx.config.url, id),
            TaskService::close(ctx, *id).await,
        ),
        TaskAction::Estimate {
            id,
            consumed,
            left,
            notes,
        } => {
            if ctx.dry_run {
                print_dry_run(
                    "TaskService::add_estimate()",
                    &format!("{}/api.php/v1/tasks/{}/estimate", ctx.config.url, id),
                );
                println!("  consumed: {}, left: {}", consumed, left);
                if let Some(n) = notes {
                    println!("  notes: {}", n);
                }
                return;
            }
            match TaskService::add_estimate(ctx, *id, *consumed, *left, notes.clone()).await {
                Ok(estimate) => print_json(&estimate),
                Err(e) => print_error(&e),
            }
        }
        TaskAction::GetEstimate { id } => {
            if ctx.dry_run {
                safe_println("[DRY-RUN] Would call TaskService::get_estimates()");
                println!("  URL: {}/api.php/v1/tasks/{}/estimate", ctx.config.url, id);
                return;
            }
            match TaskService::get_estimates(ctx, *id).await {
                Ok(estimates) => print_json(&estimates),
                Err(e) => print_error(&e),
            }
        }
    }
}

fn print_task_list(tasks: &[crate::api::Task], format: OutputFormat) {
    match format {
        OutputFormat::Table => {
            println!("Tasks:");
            for item in tasks {
                println!("  [{}] {} - {}", item.id, item.name, item.status);
            }
        }
        _ => println!(
            "{}",
            serde_json::to_string_pretty(tasks).unwrap_or_default()
        ),
    }
}

fn handle_json<T: serde::Serialize>(
    ctx: &AppContext,
    action: &str,
    url: &str,
    result: anyhow::Result<T>,
) {
    if ctx.dry_run {
        print_dry_run(action, url);
        return;
    }
    match result {
        Ok(value) => print_json(&value),
        Err(e) => print_error(&e),
    }
}

fn handle_unit(
    ctx: &AppContext,
    action: &str,
    url: &str,
    id: u64,
    label: &str,
    result: anyhow::Result<()>,
) {
    if ctx.dry_run {
        print_dry_run(action, url);
        return;
    }
    match result {
        Ok(_) => print_deleted(label, id),
        Err(e) => print_error(&e),
    }
}
