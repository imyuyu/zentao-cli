//! ZenTao Bug(缺陷)命令模块
//!
//! CLI 命令入口，调用 BugService 处理用户请求

use crate::api::UpdateBugRequest;
use crate::cmd::common::{
    log_command, print_deleted, print_dry_run, print_dry_run_with_body, print_error, print_json,
};
use crate::cmd::root::BugSubcommand;
use crate::core::{AppContext, OutputFormat};
use crate::safe_println;
use crate::service::bug::BugService;

/// 执行 Bug 相关命令
pub async fn run(cmd: &BugSubcommand, ctx: &AppContext) {
    log_command("bug", format!("{:?}", cmd));
    match cmd {
        BugSubcommand::List {
            product,
            status,
            assigned_to,
        } => {
            if ctx.dry_run {
                print_dry_run(
                    "BugService::list()",
                    &format!("{}/api.php/v1/bugs", ctx.config.url),
                );
                safe_println("  Params:");
                println!("    product: {:?}", product);
                if let Some(s) = status {
                    println!("    status: {}", s);
                }
                if let Some(a) = assigned_to {
                    println!("    assigned_to: {}", a);
                }
                return;
            }

            match BugService::list(ctx, *product, status.clone(), assigned_to.clone()).await {
                Ok(bugs) => print_bug_list(&bugs, ctx.format.clone()),
                Err(e) => print_error(&e),
            }
        }
        BugSubcommand::Get { id } => {
            if ctx.dry_run {
                print_dry_run(
                    "BugService::get()",
                    &format!("{}/api.php/v1/bugs/{}", ctx.config.url, id),
                );
                return;
            }

            match BugService::get(ctx, *id).await {
                Ok(bug) => print_json(&bug),
                Err(e) => print_error(&e),
            }
        }
        BugSubcommand::Create {
            title,
            product,
            severity,
            pri,
            type_,
            steps,
            story,
            branch,
            module,
            execution,
            keywords,
            os,
            browser,
            deadline,
            opened_build,
        } => {
            let product_id = match ctx.require_product_id(*product) {
                Ok(id) => id,
                Err(e) => {
                    print_error(&e);
                    return;
                }
            };

            if ctx.dry_run {
                let req = serde_json::json!({
                    "title": title,
                    "product": product_id,
                    "severity": severity,
                    "pri": pri,
                    "type": type_,
                    "steps": steps,
                    "story": story,
                    "branch": branch,
                    "module": module,
                    "execution": execution,
                    "keywords": keywords,
                    "os": os,
                    "browser": browser,
                    "deadline": deadline,
                    "openedBuild": opened_build,
                });
                print_dry_run_with_body(
                    "BugService::create()",
                    &format!("{}/api.php/v1/bugs", ctx.config.url),
                    &req,
                );
                return;
            }

            match BugService::create(
                ctx,
                title.clone(),
                *product,
                *severity,
                *pri,
                type_.clone(),
                steps.clone(),
                *story,
                *branch,
                *module,
                *execution,
                keywords.clone(),
                os.clone(),
                browser.clone(),
                deadline.clone(),
                opened_build.clone(),
            )
            .await
            {
                Ok(bug) => print_json(&bug),
                Err(e) => print_error(&e),
            }
        }
        BugSubcommand::Update {
            id,
            title,
            keywords,
            severity,
            pri,
            type_,
            os,
            browser,
            steps,
            task,
            story,
            deadline,
            opened_build,
            branch,
            module,
            execution,
        } => {
            let req = UpdateBugRequest {
                title: title.clone(),
                keywords: keywords.clone(),
                severity: *severity,
                pri: *pri,
                type_: type_.clone(),
                os: os.clone(),
                browser: browser.clone(),
                steps: steps.clone(),
                task: *task,
                story: *story,
                deadline: deadline.clone(),
                opened_build: opened_build.clone(),
                branch: *branch,
                module: *module,
                execution: *execution,
            };

            if ctx.dry_run {
                print_dry_run_with_body(
                    "BugService::update()",
                    &format!("{}/api.php/v1/bugs/{}", ctx.config.url, id),
                    &req,
                );
                return;
            }

            match BugService::update(ctx, *id, req).await {
                Ok(bug) => print_json(&bug),
                Err(e) => print_error(&e),
            }
        }
        BugSubcommand::Resolve {
            id,
            resolution,
            resolved_build,
            assigned_to,
            duplicate_bug,
            resolved_date,
            comment,
        } => {
            if ctx.dry_run {
                print_dry_run(
                    "BugService::resolve()",
                    &format!("{}/api.php/v1/bugs/{}/resolve", ctx.config.url, id),
                );
                println!(
                    "  Body: {{ resolution: {}, resolvedBuild: {}, assignedTo: {:?} }}",
                    resolution, resolved_build, assigned_to
                );
                return;
            }

            match BugService::resolve(
                ctx,
                *id,
                resolution,
                resolved_build,
                assigned_to.as_deref(),
                *duplicate_bug,
                resolved_date.as_deref(),
                comment.as_deref(),
            )
            .await
            {
                Ok(bug) => print_json(&bug),
                Err(e) => print_error(&e),
            }
        }
        BugSubcommand::Confirm { id } => {
            if ctx.dry_run {
                print_dry_run(
                    "BugService::confirm()",
                    &format!("{}/api.php/v1/bugs/{}/confirm", ctx.config.url, id),
                );
                return;
            }

            match BugService::confirm(ctx, *id).await {
                Ok(bug) => print_json(&bug),
                Err(e) => print_error(&e),
            }
        }
        BugSubcommand::Close { id } => {
            if ctx.dry_run {
                print_dry_run(
                    "BugService::close()",
                    &format!("{}/api.php/v1/bugs/{}/close", ctx.config.url, id),
                );
                return;
            }

            match BugService::close(ctx, *id).await {
                Ok(bug) => print_json(&bug),
                Err(e) => print_error(&e),
            }
        }
        BugSubcommand::Activate { id } => {
            if ctx.dry_run {
                print_dry_run(
                    "BugService::activate()",
                    &format!("{}/api.php/v1/bugs/{}/activate", ctx.config.url, id),
                );
                return;
            }

            match BugService::activate(ctx, *id).await {
                Ok(bug) => print_json(&bug),
                Err(e) => print_error(&e),
            }
        }
        BugSubcommand::Delete { id } => {
            if ctx.dry_run {
                print_dry_run(
                    "BugService::delete()",
                    &format!("{}/api.php/v1/bugs/{}", ctx.config.url, id),
                );
                return;
            }

            match BugService::delete(ctx, *id).await {
                Ok(_) => print_deleted("Bug", *id),
                Err(e) => print_error(&e),
            }
        }
    }
}

fn print_bug_list(bugs: &[crate::api::Bug], format: OutputFormat) {
    match format {
        OutputFormat::Table => {
            println!("Bugs:");
            for bug in bugs {
                println!(
                    "  [{}] {} - {} (Severity: {})",
                    bug.id, bug.title, bug.status, bug.severity
                );
            }
        }
        _ => {
            println!("{}", serde_json::to_string_pretty(bugs).unwrap_or_default());
        }
    }
}
