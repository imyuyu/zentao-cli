//! ZenTao 需求(Story)命令模块
//!
//! CLI 命令入口，调用 StoryService 处理用户请求

use crate::api::UpdateStoryRequest;
use crate::cmd::common::{
    log_command, print_dry_run, print_dry_run_with_body, print_error, print_json,
};
use crate::cmd::root::StorySubcommand;
use crate::core::{AppContext, OutputFormat};
use crate::safe_println;
use crate::service::story::StoryService;

/// 执行 Story 相关命令
pub async fn run(cmd: &StorySubcommand, ctx: &AppContext) {
    log_command("story", format!("{:?}", cmd));
    match cmd {
        StorySubcommand::List {
            product,
            project,
            status,
        } => {
            if ctx.dry_run {
                print_dry_run(
                    "StoryService::list()",
                    &format!("{}/api.php/v1/stories", ctx.config.url),
                );
                safe_println("  Params:");
                if let Some(p) = product {
                    println!("    product: {}", p);
                }
                if let Some(p) = project {
                    println!("    project: {}", p);
                }
                if let Some(s) = status {
                    println!("    status: {}", s);
                }
                return;
            }

            match StoryService::list(ctx, *product, *project, status.clone()).await {
                Ok(stories) => print_story_list(&stories, ctx.format.clone()),
                Err(e) => print_error(&e),
            }
        }
        StorySubcommand::Get { id } => {
            if ctx.dry_run {
                print_dry_run(
                    "StoryService::get()",
                    &format!("{}/api.php/v1/stories/{}", ctx.config.url, id),
                );
                return;
            }

            match StoryService::get(ctx, *id).await {
                Ok(story) => print_json(&story),
                Err(e) => print_error(&e),
            }
        }
        StorySubcommand::Create {
            title,
            product,
            pri,
            category,
            spec,
            estimate,
        } => {
            let product_id = match ctx.require_product_id(*product) {
                Ok(id) => id,
                Err(e) => {
                    print_error(&e);
                    return;
                }
            };

            let req = serde_json::json!({
                "title": title,
                "product": product_id,
                "pri": pri,
                "category": category,
                "spec": spec,
                "verify": serde_json::Value::Null,
                "estimate": estimate,
            });

            if ctx.dry_run {
                print_dry_run_with_body(
                    "StoryService::create()",
                    &format!("{}/api.php/v1/stories", ctx.config.url),
                    &req,
                );
                return;
            }

            match StoryService::create(
                ctx,
                title.clone(),
                *product,
                *pri,
                category.clone(),
                spec.clone(),
                *estimate,
            )
            .await
            {
                Ok(story) => print_json(&story),
                Err(e) => print_error(&e),
            }
        }
        StorySubcommand::Update {
            id,
            title,
            status,
            pri,
            assigned_to,
        } => {
            let req = UpdateStoryRequest {
                title: title.clone(),
                module: None,
                source: None,
                sourceNote: None,
                pri: *pri,
                category: None,
                estimate: None,
                keywords: None,
                assigned_to: assigned_to.clone(),
                status: status.clone(),
            };

            if ctx.dry_run {
                print_dry_run_with_body(
                    "StoryService::update()",
                    &format!("{}/api.php/v1/stories/{}", ctx.config.url, id),
                    &req,
                );
                return;
            }

            match StoryService::update(ctx, *id, req).await {
                Ok(story) => print_json(&story),
                Err(e) => print_error(&e),
            }
        }
        StorySubcommand::Change {
            id,
            title,
            status,
            pri,
            assigned_to,
        } => {
            let req = UpdateStoryRequest {
                title: title.clone(),
                module: None,
                source: None,
                sourceNote: None,
                pri: *pri,
                category: None,
                estimate: None,
                keywords: None,
                assigned_to: assigned_to.clone(),
                status: status.clone(),
            };

            if ctx.dry_run {
                print_dry_run_with_body(
                    "StoryService::change()",
                    &format!("{}/api.php/v1/stories/{}/change", ctx.config.url, id),
                    &req,
                );
                return;
            }

            match StoryService::change(ctx, *id, req).await {
                Ok(story) => print_json(&story),
                Err(e) => print_error(&e),
            }
        }
        StorySubcommand::Delete { id } => {
            if ctx.dry_run {
                print_dry_run(
                    "StoryService::delete()",
                    &format!("{}/api.php/v1/stories/{}", ctx.config.url, id),
                );
                return;
            }

            match StoryService::delete(ctx, *id).await {
                Ok(story) => print_json(&story),
                Err(e) => print_error(&e),
            }
        }
        StorySubcommand::Close { id } => {
            if ctx.dry_run {
                print_dry_run(
                    "StoryService::close()",
                    &format!("{}/api.php/v1/stories/{}/close", ctx.config.url, id),
                );
                return;
            }

            match StoryService::close(ctx, *id).await {
                Ok(story) => print_json(&story),
                Err(e) => print_error(&e),
            }
        }
    }
}

fn print_story_list(stories: &[crate::api::Story], format: OutputFormat) {
    match format {
        OutputFormat::Table => {
            println!("Stories:");
            for story in stories {
                println!("  [{}] {} - {}", story.id, story.title, story.status);
            }
        }
        _ => {
            println!(
                "{}",
                serde_json::to_string_pretty(stories).unwrap_or_default()
            );
        }
    }
}
