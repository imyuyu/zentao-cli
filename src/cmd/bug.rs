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
            project,
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
            attachments,
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
                *project,
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
                Ok(bug) => {
                    // 附件必须在 bug 创建后上传——上传接口要求关联对象已存在
                    if let Some(files) = attachments {
                        if !files.is_empty() {
                            if let Err(e) = upload_attachments(ctx, bug.id, files).await {
                                // bug 已创建成功，附件失败不能当成整体失败
                                eprintln!("warning: bug {} 已创建，但附件上传失败: {}", bug.id, e);
                            }
                        }
                    }
                    print_json(&bug)
                }
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
        BugSubcommand::Confirm {
            id,
            assigned_to,
            type_,
            pri,
            mailto,
            comment,
        } => {
            if ctx.dry_run {
                print_dry_run(
                    "BugService::confirm()",
                    &format!("{}/api.php/v1/bugs/{}/confirm", ctx.config.url, id),
                );
                return;
            }

            match BugService::confirm(
                ctx,
                *id,
                assigned_to.as_deref(),
                type_.as_deref(),
                *pri,
                mailto.clone(),
                comment.as_deref(),
            )
            .await
            {
                Ok(bug) => print_json(&bug),
                Err(e) => print_error(&e),
            }
        }
        BugSubcommand::Close { id, comment } => {
            if ctx.dry_run {
                print_dry_run(
                    "BugService::close()",
                    &format!("{}/api.php/v1/bugs/{}/close", ctx.config.url, id),
                );
                println!("  Body: {{ comment: {:?} }}", comment);
                return;
            }

            match BugService::close(ctx, *id, comment.as_deref()).await {
                Ok(bug) => print_json(&bug),
                Err(e) => print_error(&e),
            }
        }
        BugSubcommand::Activate {
            id,
            assigned_to,
            opened_build,
            comment,
        } => {
            if ctx.dry_run {
                print_dry_run(
                    "BugService::activate()",
                    &format!("{}/api.php/v1/bugs/{}/active", ctx.config.url, id),
                );
                println!(
                    "  Body: {{ assignedTo: {:?}, openedBuild: {:?}, comment: {:?} }}",
                    assigned_to, opened_build, comment
                );
                return;
            }

            match BugService::activate(
                ctx,
                *id,
                assigned_to.as_deref(),
                opened_build.clone(),
                comment.as_deref(),
            )
            .await
            {
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

/// 上传附件到指定 bug
///
/// 禅道的附件上传只存在于 v2 API（ZenTao ≥ v22.0）。老版本没有该路由，
/// 请求会被兜底路由吃掉并返回 200 空 body——所以先探测，再决定是否上传。
async fn upload_attachments(ctx: &AppContext, bug_id: u64, files: &[String]) -> anyhow::Result<()> {
    if !ctx.supports_file_upload().await {
        anyhow::bail!(
            "当前 ZenTao 版本不支持 API 上传附件（需 v22.0+ 的 v2 文件接口）。\
             bug 已创建，附件请到网页端补传。"
        );
    }

    let client = ctx.client();
    for path in files {
        let content = tokio::fs::read(path)
            .await
            .map_err(|e| anyhow::anyhow!("读取附件失败 {}: {}", path, e))?;
        let filename = std::path::Path::new(path)
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("attachment");

        let file_id = client
            .upload_file("bug", bug_id, filename, content)
            .await
            .map_err(|e| anyhow::anyhow!("上传附件失败 {}: {}", path, e))?;
        eprintln!("上传成功: {} (file id: {})", path, file_id);
    }
    Ok(())
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
