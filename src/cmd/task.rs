//! ZenTao Task(任务)命令模块

use clap::Subcommand;

use crate::api::{
    BatchEstimateRequest, CloseTaskRequest, CreateTaskRequest, FinishTaskRequest, PauseTaskRequest,
    RestartTaskRequest, StartTaskRequest, UpdateTaskRequest,
};
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
        assigned_to: String,
        #[arg(long)]
        type_: String,
        #[arg(long)]
        est_started: String,
        #[arg(long)]
        deadline: String,
        #[arg(long)]
        project: Option<u64>,
        #[arg(long)]
        pri: Option<u8>,
        #[arg(long)]
        estimate: Option<f64>,
        #[arg(long)]
        module: Option<u64>,
        #[arg(long)]
        story: Option<u64>,
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
        #[arg(long)]
        est_started: Option<String>,
        #[arg(long)]
        deadline: Option<String>,
        #[arg(long)]
        from_bug: Option<u64>,
    },
    #[command(name = "delete")]
    Delete { id: u64 },
    #[command(name = "start")]
    Start {
        id: u64,
        /// 剩余工时（必填）
        #[arg(long)]
        left: f64,
        /// 已消耗工时（可选）
        #[arg(long)]
        consumed: Option<f64>,
        /// 指派人（可选）
        #[arg(long)]
        assigned_to: Option<String>,
        /// 实际开始时间（可选）
        #[arg(long)]
        real_started: Option<String>,
        /// 备注（可选）
        #[arg(long)]
        comment: Option<String>,
    },
    #[command(name = "pause")]
    Pause {
        id: u64,
        /// 备注（可选）
        #[arg(long)]
        comment: Option<String>,
    },
    #[command(name = "restart")]
    Restart {
        id: u64,
        /// 剩余工时（必填）
        #[arg(long)]
        left: f64,
        /// 已消耗工时（可选）
        #[arg(long)]
        consumed: Option<f64>,
        /// 指派人（可选）
        #[arg(long)]
        assigned_to: Option<String>,
        /// 实际开始时间（可选）
        #[arg(long)]
        real_started: Option<String>,
        /// 备注（可选）
        #[arg(long)]
        comment: Option<String>,
    },
    #[command(name = "finish")]
    Finish {
        id: u64,
        /// 当前消耗工时（必填）
        #[arg(long)]
        current_consumed: f64,
        /// 完成日期（必填）
        #[arg(long)]
        finished_date: String,
        /// 指派人（可选）
        #[arg(long)]
        assigned_to: Option<String>,
        /// 实际开始时间（可选）
        #[arg(long)]
        real_started: Option<String>,
        /// 备注（可选）
        #[arg(long)]
        comment: Option<String>,
    },
    #[command(name = "close")]
    Close {
        id: u64,
        /// 备注（可选）
        #[arg(long)]
        comment: Option<String>,
    },
    #[command(name = "estimate")]
    Estimate {
        id: u64,
        /// 日期列表（多个用逗号分隔）
        #[arg(long)]
        dates: String,
        /// 工作内容列表（多个用逗号分隔）
        #[arg(long)]
        work: String,
        /// 消耗工时列表（多个用逗号分隔）
        #[arg(long)]
        consumed: String,
        /// 剩余工时列表（多个用逗号分隔）
        #[arg(long)]
        left: String,
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
            assigned_to,
            type_,
            est_started,
            deadline,
            project,
            pri,
            estimate,
            module,
            story,
        } => {
            let project_id = ctx.project_id(*project);
            let req = CreateTaskRequest {
                name: name.clone(),
                assigned_to: assigned_to.clone(),
                type_: type_.clone(),
                est_started: est_started.clone(),
                deadline: deadline.clone(),
                project: project_id,
                execution: None,
                pri: *pri,
                module: *module,
                story: *story,
                from_bug: None,
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
            est_started,
            deadline,
            from_bug,
        } => {
            let req = UpdateTaskRequest {
                name: name.clone(),
                status: status.clone(),
                pri: *pri,
                assigned_to: assigned_to.clone(),
                module: None,
                story: None,
                from_bug: *from_bug,
                type_: None,
                est_started: est_started.clone(),
                deadline: deadline.clone(),
                estimate: None,
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
        TaskAction::Start {
            id,
            left,
            consumed,
            assigned_to,
            real_started,
            comment,
        } => {
            let req = StartTaskRequest {
                left: *left,
                consumed: *consumed,
                assigned_to: assigned_to.clone(),
                real_started: real_started.clone(),
                comment: comment.clone(),
            };
            if ctx.dry_run {
                print_dry_run_with_body(
                    "TaskService::start()",
                    &format!("{}/api.php/v1/tasks/{}/start", ctx.config.url, id),
                    &req,
                );
                return;
            }
            match TaskService::start(ctx, *id, req).await {
                Ok(task) => print_json(&task),
                Err(e) => print_error(&e),
            }
        }
        TaskAction::Pause {
            id,
            comment,
        } => {
            let req = PauseTaskRequest { comment: comment.clone() };
            if ctx.dry_run {
                print_dry_run_with_body(
                    "TaskService::pause()",
                    &format!("{}/api.php/v1/tasks/{}/pause", ctx.config.url, id),
                    &req,
                );
                return;
            }
            match TaskService::pause(ctx, *id, req).await {
                Ok(task) => print_json(&task),
                Err(e) => print_error(&e),
            }
        }
        TaskAction::Restart {
            id,
            left,
            consumed,
            assigned_to,
            real_started,
            comment,
        } => {
            let req = RestartTaskRequest {
                left: *left,
                consumed: *consumed,
                assigned_to: assigned_to.clone(),
                real_started: real_started.clone(),
                comment: comment.clone(),
            };
            if ctx.dry_run {
                print_dry_run_with_body(
                    "TaskService::restart()",
                    &format!("{}/api.php/v1/tasks/{}/restart", ctx.config.url, id),
                    &req,
                );
                return;
            }
            match TaskService::restart(ctx, *id, req).await {
                Ok(task) => print_json(&task),
                Err(e) => print_error(&e),
            }
        }
        TaskAction::Finish {
            id,
            current_consumed,
            finished_date,
            assigned_to,
            real_started,
            comment,
        } => {
            let req = FinishTaskRequest {
                current_consumed: *current_consumed,
                finished_date: finished_date.clone(),
                assigned_to: assigned_to.clone(),
                real_started: real_started.clone(),
                comment: comment.clone(),
            };
            if ctx.dry_run {
                print_dry_run_with_body(
                    "TaskService::finish()",
                    &format!("{}/api.php/v1/tasks/{}/finish", ctx.config.url, id),
                    &req,
                );
                return;
            }
            match TaskService::finish(ctx, *id, req).await {
                Ok(task) => print_json(&task),
                Err(e) => print_error(&e),
            }
        }
        TaskAction::Close {
            id,
            comment,
        } => {
            let req = CloseTaskRequest { comment: comment.clone() };
            if ctx.dry_run {
                print_dry_run_with_body(
                    "TaskService::close()",
                    &format!("{}/api.php/v1/tasks/{}/close", ctx.config.url, id),
                    &req,
                );
                return;
            }
            match TaskService::close(ctx, *id, req).await {
                Ok(task) => print_json(&task),
                Err(e) => print_error(&e),
            }
        }
        TaskAction::Estimate {
            id,
            dates,
            work,
            consumed,
            left,
        } => {
            let dates_vec: Vec<String> = dates.split(',').map(|s| s.trim().to_string()).collect();
            let work_vec: Vec<f64> = work
                .split(',')
                .map(|s| s.trim().parse().unwrap_or(0.0))
                .collect();
            let consumed_vec: Vec<f64> = consumed
                .split(',')
                .map(|s| s.trim().parse().unwrap_or(0.0))
                .collect();
            let left_vec: Vec<f64> = left
                .split(',')
                .map(|s| s.trim().parse().unwrap_or(0.0))
                .collect();

            if ctx.dry_run {
                safe_println("[DRY-RUN] Would call TaskService::add_estimate()");
                println!("  URL: {}/api.php/v1/tasks/{}/estimate", ctx.config.url, id);
                println!("  dates: {:?}", dates_vec);
                println!("  work: {:?}", work_vec);
                println!("  consumed: {:?}", consumed_vec);
                println!("  left: {:?}", left_vec);
                return;
            }
            match TaskService::add_estimate(ctx, *id, dates_vec, work_vec, consumed_vec, left_vec).await {
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
