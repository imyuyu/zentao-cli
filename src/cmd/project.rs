//! ZenTao Project(项目)命令模块

use clap::Subcommand;

use crate::api::{CreateProjectRequest, UpdateProjectRequest};
use crate::cmd::common::{
    log_command, print_deleted, print_dry_run, print_dry_run_with_body, print_error, print_json,
};
use crate::core::{AppContext, OutputFormat};
use crate::service::project::ProjectService;

#[derive(Subcommand, Clone, Debug)]
pub enum ProjectAction {
    #[command(name = "list")]
    List,
    #[command(name = "get")]
    Get { id: u64 },
    #[command(name = "create")]
    Create {
        #[arg(long)]
        name: String,
        #[arg(long)]
        code: String,
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
        desc: Option<String>,
    },
    #[command(name = "delete")]
    Delete { id: u64 },
}

pub async fn run(cmd: &ProjectAction, ctx: &AppContext) {
    log_command("project", format!("{:?}", cmd));
    match cmd {
        ProjectAction::List => {
            if ctx.dry_run {
                print_dry_run(
                    "ProjectService::list()",
                    &format!("{}/api.php/v1/projects", ctx.config.url),
                );
                return;
            }
            match ProjectService::list(ctx).await {
                Ok(projects) => print_list(&projects, ctx.format.clone()),
                Err(e) => print_error(&e),
            }
        }
        ProjectAction::Get { id } => {
            if ctx.dry_run {
                print_dry_run(
                    "ProjectService::get()",
                    &format!("{}/api.php/v1/projects/{}", ctx.config.url, id),
                );
                return;
            }
            match ProjectService::get(ctx, *id).await {
                Ok(project) => print_json(&project),
                Err(e) => print_error(&e),
            }
        }
        ProjectAction::Create { name, code, desc } => {
            let req = CreateProjectRequest {
                name: name.clone(),
                code: code.clone(),
                desc: desc.clone(),
            };
            if ctx.dry_run {
                print_dry_run_with_body(
                    "ProjectService::create()",
                    &format!("{}/api.php/v1/projects", ctx.config.url),
                    &req,
                );
                return;
            }
            match ProjectService::create(ctx, req).await {
                Ok(project) => print_json(&project),
                Err(e) => print_error(&e),
            }
        }
        ProjectAction::Update {
            id,
            name,
            status,
            desc,
        } => {
            let req = UpdateProjectRequest {
                name: name.clone(),
                status: status.clone(),
                desc: desc.clone(),
            };
            if ctx.dry_run {
                print_dry_run_with_body(
                    "ProjectService::update()",
                    &format!("{}/api.php/v1/projects/{}", ctx.config.url, id),
                    &req,
                );
                return;
            }
            match ProjectService::update(ctx, *id, req).await {
                Ok(project) => print_json(&project),
                Err(e) => print_error(&e),
            }
        }
        ProjectAction::Delete { id } => {
            if ctx.dry_run {
                print_dry_run(
                    "ProjectService::delete()",
                    &format!("{}/api.php/v1/projects/{}", ctx.config.url, id),
                );
                return;
            }
            match ProjectService::delete(ctx, *id).await {
                Ok(_) => print_deleted("Project", *id),
                Err(e) => print_error(&e),
            }
        }
    }
}

fn print_list(projects: &[crate::api::Project], format: OutputFormat) {
    match format {
        OutputFormat::Table => {
            println!("Projects:");
            for item in projects {
                println!("  [{}] {} - {}", item.id, item.name, item.status);
            }
        }
        _ => println!(
            "{}",
            serde_json::to_string_pretty(projects).unwrap_or_default()
        ),
    }
}
