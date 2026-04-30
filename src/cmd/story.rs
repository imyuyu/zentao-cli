//! ZenTao 需求(Story)命令模块
//!
//! CLI 命令入口，调用 StoryApi 处理用户请求

use crate::api::{ApiClient, CreateStoryRequest, StoryApi, UpdateStoryRequest};
use crate::cmd::root::StorySubcommand;
use crate::core::{Config, OutputFormat};

/// 执行 Story 相关命令
///
/// 根据子命令类型调用对应的 API 并输出结果
pub fn run(cmd: &StorySubcommand, config: &Config, _format: OutputFormat, dry_run: bool) {
    // 创建 Tokio 异步运行时
    // CLI 命令需要手动创建运行时来执行 async 代码
    let rt = tokio::runtime::Runtime::new().expect("Failed to create Tokio runtime");

    rt.block_on(async {
        // 创建 API 客户端，传入 URL 和 Token
        let client = ApiClient::new(&config.url, config.token.clone())
            .with_api_version(config.api_version.as_deref().unwrap_or("v1"));

        match cmd {
            // -------------------- list --------------------
            StorySubcommand::List {
                product,
                project,
                status,
            } => {
                if dry_run {
                    println!("[DRY-RUN] Would call StoryApi::list()");
                    println!("  URL: {}/api.php/v1/stories", config.url);
                    println!("  Params:");
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
                // 调用 StoryApi::list 获取需求列表
                match StoryApi::list(
                    &client,
                    config.product_id(*product),
                    status.clone(),
                    config.project_id(*project),
                )
                .await
                {
                    Ok(stories) => {
                        // 输出 JSON 格式结果
                        println!(
                            "{}",
                            serde_json::to_string_pretty(&stories).unwrap_or_default()
                        );
                    }
                    Err(e) => {
                        eprintln!("Error: {}", e);
                    }
                }
            }

            // -------------------- get --------------------
            StorySubcommand::Get { id } => {
                if dry_run {
                    println!("[DRY-RUN] Would call StoryApi::get()");
                    println!("  URL: {}/api.php/v1/stories/{}", config.url, id);
                    return;
                }
                match StoryApi::get(&client, *id).await {
                    Ok(story) => {
                        println!(
                            "{}",
                            serde_json::to_string_pretty(&story).unwrap_or_default()
                        );
                    }
                    Err(e) => {
                        eprintln!("Error: {}", e);
                    }
                }
            }

            // -------------------- create --------------------
            StorySubcommand::Create {
                title,
                product,
                pri,
                category,
                spec,
                estimate,
            } => {
                // 构建创建请求
                let req = CreateStoryRequest {
                    title: title.clone(),
                    product: *product,
                    pri: *pri,
                    category: category.clone(),
                    spec: spec.clone(),
                    // verify 字段暂不支持
                    verify: None,
                    estimate: *estimate,
                };

                if dry_run {
                    println!("[DRY-RUN] Would call StoryApi::create()");
                    println!("  URL: {}/api.php/v1/stories", config.url);
                    println!(
                        "  Body: {}",
                        serde_json::to_string_pretty(&req).unwrap_or_default()
                    );
                    return;
                }

                match StoryApi::create(&client, &req).await {
                    Ok(story) => {
                        println!(
                            "{}",
                            serde_json::to_string_pretty(&story).unwrap_or_default()
                        );
                    }
                    Err(e) => {
                        eprintln!("Error: {}", e);
                    }
                }
            }

            // -------------------- update --------------------
            StorySubcommand::Update {
                id,
                title,
                status,
                pri,
                assigned_to,
            } => {
                // 构建更新请求
                let req = UpdateStoryRequest {
                    title: title.clone(),
                    status: status.clone(),
                    pri: *pri,
                    assigned_to: assigned_to.clone(),
                };

                if dry_run {
                    println!("[DRY-RUN] Would call StoryApi::update()");
                    println!("  URL: {}/api.php/v1/stories/{}", config.url, id);
                    println!(
                        "  Body: {}",
                        serde_json::to_string_pretty(&req).unwrap_or_default()
                    );
                    return;
                }

                match StoryApi::update(&client, *id, &req).await {
                    Ok(story) => {
                        println!(
                            "{}",
                            serde_json::to_string_pretty(&story).unwrap_or_default()
                        );
                    }
                    Err(e) => {
                        eprintln!("Error: {}", e);
                    }
                }
            }

            // -------------------- change --------------------
            StorySubcommand::Change {
                id,
                title,
                status,
                pri,
                assigned_to,
            } => {
                let req = UpdateStoryRequest {
                    title: title.clone(),
                    status: status.clone(),
                    pri: *pri,
                    assigned_to: assigned_to.clone(),
                };

                if dry_run {
                    println!("[DRY-RUN] Would call StoryApi::change()");
                    println!("  URL: {}/api.php/v1/stories/{}/change", config.url, id);
                    println!(
                        "  Body: {}",
                        serde_json::to_string_pretty(&req).unwrap_or_default()
                    );
                    return;
                }

                match StoryApi::change(&client, *id, &req).await {
                    Ok(story) => {
                        println!(
                            "{}",
                            serde_json::to_string_pretty(&story).unwrap_or_default()
                        );
                    }
                    Err(e) => {
                        eprintln!("Error: {}", e);
                    }
                }
            }

            // -------------------- delete --------------------
            StorySubcommand::Delete { id } => {
                if dry_run {
                    println!("[DRY-RUN] Would call StoryApi::delete()");
                    println!("  URL: {}/api.php/v1/stories/{}", config.url, id);
                    return;
                }

                match StoryApi::delete(&client, *id).await {
                    Ok(story) => {
                        println!(
                            "{}",
                            serde_json::to_string_pretty(&story).unwrap_or_default()
                        );
                    }
                    Err(e) => {
                        eprintln!("Error: {}", e);
                    }
                }
            }

            // -------------------- close --------------------
            StorySubcommand::Close { id } => {
                if dry_run {
                    println!("[DRY-RUN] Would call StoryApi::close()");
                    println!("  URL: {}/api.php/v1/stories/{}/close", config.url, id);
                    return;
                }

                match StoryApi::close(&client, *id).await {
                    Ok(story) => {
                        println!(
                            "{}",
                            serde_json::to_string_pretty(&story).unwrap_or_default()
                        );
                    }
                    Err(e) => {
                        eprintln!("Error: {}", e);
                    }
                }
            }
        }
    });
}
