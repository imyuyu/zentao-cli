//! ZenTao Build(版本)命令模块

use clap::Subcommand;

use crate::api::{CreateBuildRequest, UpdateBuildRequest};
use crate::cmd::common::{
    log_command, print_deleted, print_dry_run, print_dry_run_with_body, print_error, print_json,
};
use crate::core::{AppContext, OutputFormat};
use crate::service::build::BuildService;

#[derive(Subcommand, Clone, Debug)]
pub enum BuildAction {
    #[command(name = "list")]
    List {
        #[arg(long)]
        project: Option<u64>,
        #[arg(long)]
        product: Option<u64>,
        #[arg(long)]
        execution: Option<u64>,
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
        product: Option<u64>,
        #[arg(long)]
        branch: Option<u64>,
        #[arg(long)]
        scm_path: Option<String>,
        #[arg(long)]
        ci: Option<String>,
        #[arg(long)]
        pkg: Option<String>,
    },
    #[command(name = "update")]
    Update {
        id: u64,
        #[arg(long)]
        name: Option<String>,
        #[arg(long)]
        scm_path: Option<String>,
        #[arg(long)]
        ci: Option<String>,
        #[arg(long)]
        pkg: Option<String>,
    },
    #[command(name = "delete")]
    Delete { id: u64 },
}

pub async fn run(cmd: &BuildAction, ctx: &AppContext) {
    log_command("build", format!("{:?}", cmd));
    match cmd {
        BuildAction::List {
            project,
            product,
            execution,
        } => {
            let project_id = ctx.project_id(*project);
            let product_id = ctx.product_id(*product);
            if ctx.dry_run {
                if let Some(eid) = execution {
                    print_dry_run(
                        "BuildService::list() via execution",
                        &format!("{}/api.php/v1/executions/{}/builds", ctx.config.url, eid),
                    );
                } else {
                    print_dry_run(
                        "BuildService::list()",
                        &format!("{}/api.php/v1/builds", ctx.config.url),
                    );
                    println!("  Params:");
                    if let Some(p) = project_id {
                        println!("    project: {}", p);
                    }
                    if let Some(p) = product_id {
                        println!("    product: {}", p);
                    }
                }
                return;
            }
            match BuildService::list(ctx, *project, *product, *execution).await {
                Ok(builds) => print_build_list(&builds, ctx.format.clone()),
                Err(e) => print_error(&e),
            }
        }
        BuildAction::Get { id } => {
            if ctx.dry_run {
                print_dry_run(
                    "BuildService::get()",
                    &format!("{}/api.php/v1/builds/{}", ctx.config.url, id),
                );
                return;
            }
            match BuildService::get(ctx, *id).await {
                Ok(build) => print_json(&build),
                Err(e) => print_error(&e),
            }
        }
        BuildAction::Create {
            name,
            project,
            product,
            branch,
            scm_path,
            ci,
            pkg,
        } => {
            let project_id = match ctx.require_project_id(*project) {
                Ok(id) => id,
                Err(e) => {
                    print_error(&e);
                    return;
                }
            };
            let product_id = match ctx.require_product_id(*product) {
                Ok(id) => id,
                Err(e) => {
                    print_error(&e);
                    return;
                }
            };
            let req = CreateBuildRequest {
                name: name.clone(),
                project: project_id,
                product: product_id,
                branch: *branch,
                scm_path: scm_path.clone(),
                ci: ci.clone(),
                pkg: pkg.clone(),
            };
            if ctx.dry_run {
                print_dry_run_with_body(
                    "BuildService::create()",
                    &format!(
                        "{}/api.php/v1/projects/{}/builds",
                        ctx.config.url, project_id
                    ),
                    &req,
                );
                return;
            }
            match BuildService::create(ctx, *project, req).await {
                Ok(build) => print_json(&build),
                Err(e) => print_error(&e),
            }
        }
        BuildAction::Update {
            id,
            name,
            scm_path,
            ci,
            pkg,
        } => {
            let req = UpdateBuildRequest {
                name: name.clone(),
                scm_path: scm_path.clone(),
                ci: ci.clone(),
                pkg: pkg.clone(),
                file_size: None,
                generated_at: None,
            };
            if ctx.dry_run {
                print_dry_run_with_body(
                    "BuildService::update()",
                    &format!("{}/api.php/v1/builds/{}", ctx.config.url, id),
                    &req,
                );
                return;
            }
            match BuildService::update(ctx, *id, req).await {
                Ok(build) => print_json(&build),
                Err(e) => print_error(&e),
            }
        }
        BuildAction::Delete { id } => {
            if ctx.dry_run {
                print_dry_run(
                    "BuildService::delete()",
                    &format!("{}/api.php/v1/builds/{}", ctx.config.url, id),
                );
                return;
            }
            match BuildService::delete(ctx, *id).await {
                Ok(_) => print_deleted("Build", *id),
                Err(e) => print_error(&e),
            }
        }
    }
}

fn print_build_list(builds: &[crate::api::Build], format: OutputFormat) {
    match format {
        OutputFormat::Table => {
            println!("Builds:");
            for item in builds {
                println!("  [{}] {}", item.id, item.name);
            }
        }
        _ => println!(
            "{}",
            serde_json::to_string_pretty(builds).unwrap_or_default()
        ),
    }
}
