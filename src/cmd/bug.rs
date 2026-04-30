//! ZenTao Bug(缺陷)命令模块
//!
//! CLI 命令入口，调用 BugApi 处理用户请求

use crate::api::{ApiClient, BugApi, CreateBugRequest, UpdateBugRequest};
use crate::cmd::root::BugSubcommand;
use crate::core::{Config, OutputFormat};

/// 执行 Bug 相关命令
///
/// 根据子命令类型调用对应的 API 并输出结果
pub fn run(cmd: &BugSubcommand, config: &Config, _format: OutputFormat, dry_run: bool) {
    let rt = tokio::runtime::Runtime::new().expect("Failed to create Tokio runtime");

    rt.block_on(async {
        let client = ApiClient::new(&config.url, config.token.clone())
            .with_api_version(config.api_version.as_deref().unwrap_or("v1"));

        match cmd {
            // -------------------- list --------------------
            BugSubcommand::List {
                product,
                status,
                assigned_to,
            } => {
                if dry_run {
                    println!("[DRY-RUN] Would call BugApi::list()");
                    println!("  URL: {}/api.php/v1/bugs", config.url);
                    println!("  Params:");
                    println!("    product: {:?}", product);
                    if let Some(s) = status {
                        println!("    status: {}", s);
                    }
                    if let Some(a) = assigned_to {
                        println!("    assigned_to: {}", a);
                    }
                    return;
                }
                match BugApi::list(&client, config.product_id(*product).unwrap_or_else(|| {
                    eprintln!("Error: product ID is required. Provide via --product or set ZENTAO_PRODUCT_ID");
                    std::process::exit(1);
                }), status.clone(), assigned_to.clone()).await {
                    Ok(bugs) => {
                        println!(
                            "{}",
                            serde_json::to_string_pretty(&bugs).unwrap_or_default()
                        );
                    }
                    Err(e) => {
                        eprintln!("Error: {}", e);
                    }
                }
            }

            // -------------------- get --------------------
            BugSubcommand::Get { id } => {
                if dry_run {
                    println!("[DRY-RUN] Would call BugApi::get()");
                    println!("  URL: {}/api.php/v1/bugs/{}", config.url, id);
                    return;
                }
                match BugApi::get(&client, *id).await {
                    Ok(bug) => {
                        println!("{}", serde_json::to_string_pretty(&bug).unwrap_or_default());
                    }
                    Err(e) => {
                        eprintln!("Error: {}", e);
                    }
                }
            }

            // -------------------- create --------------------
            BugSubcommand::Create {
                title,
                product,
                severity,
                pri,
                type_,
                steps,
                story,
            } => {
                let product_id = config.product_id(*product).unwrap_or_else(|| {
                    eprintln!("Error: product ID is required. Provide via --product or set ZENTAO_PRODUCT_ID");
                    std::process::exit(1);
                });
                let req = CreateBugRequest {
                    title: title.clone(),
                    product: product_id,
                    severity: *severity,
                    pri: *pri,
                    type_: type_.clone(),
                    steps: steps.clone(),
                    story: *story,
                    assigned_to: None,
                };

                if dry_run {
                    println!("[DRY-RUN] Would call BugApi::create()");
                    println!("  URL: {}/api.php/v1/bugs", config.url);
                    println!(
                        "  Body: {}",
                        serde_json::to_string_pretty(&req).unwrap_or_default()
                    );
                    return;
                }

                match BugApi::create(&client, &req).await {
                    Ok(bug) => {
                        println!("{}", serde_json::to_string_pretty(&bug).unwrap_or_default());
                    }
                    Err(e) => {
                        eprintln!("Error: {}", e);
                    }
                }
            }

            // -------------------- update --------------------
            BugSubcommand::Update {
                id,
                title,
                status,
                resolution,
                resolved_build,
                assigned_to,
            } => {
                let req = UpdateBugRequest {
                    title: title.clone(),
                    status: status.clone(),
                    resolution: resolution.clone(),
                    resolved_build: *resolved_build,
                    assigned_to: assigned_to.clone(),
                };

                if dry_run {
                    println!("[DRY-RUN] Would call BugApi::update()");
                    println!("  URL: {}/api.php/v1/bugs/{}", config.url, id);
                    println!(
                        "  Body: {}",
                        serde_json::to_string_pretty(&req).unwrap_or_default()
                    );
                    return;
                }

                match BugApi::update(&client, *id, &req).await {
                    Ok(bug) => {
                        println!("{}", serde_json::to_string_pretty(&bug).unwrap_or_default());
                    }
                    Err(e) => {
                        eprintln!("Error: {}", e);
                    }
                }
            }

            // -------------------- resolve --------------------
            BugSubcommand::Resolve {
                id,
                resolution,
                resolved_build,
            } => {
                if dry_run {
                    println!("[DRY-RUN] Would call BugApi::resolve()");
                    println!("  URL: {}/api.php/v1/bugs/{}/resolve", config.url, id);
                    println!(
                        "  Body: {{ resolution: {}, resolved_build: {} }}",
                        resolution, resolved_build
                    );
                    return;
                }

                match BugApi::resolve(&client, *id, resolution, resolved_build).await {
                    Ok(bug) => {
                        println!("{}", serde_json::to_string_pretty(&bug).unwrap_or_default());
                    }
                    Err(e) => {
                        eprintln!("Error: {}", e);
                    }
                }
            }

            // -------------------- confirm --------------------
            BugSubcommand::Confirm { id } => {
                if dry_run {
                    println!("[DRY-RUN] Would call BugApi::confirm()");
                    println!("  URL: {}/api.php/v1/bugs/{}/confirm", config.url, id);
                    return;
                }

                match BugApi::confirm(&client, *id).await {
                    Ok(bug) => {
                        println!("{}", serde_json::to_string_pretty(&bug).unwrap_or_default());
                    }
                    Err(e) => {
                        eprintln!("Error: {}", e);
                    }
                }
            }

            // -------------------- close --------------------
            BugSubcommand::Close { id } => {
                if dry_run {
                    println!("[DRY-RUN] Would call BugApi::close()");
                    println!("  URL: {}/api.php/v1/bugs/{}/close", config.url, id);
                    return;
                }

                match BugApi::close(&client, *id).await {
                    Ok(bug) => {
                        println!("{}", serde_json::to_string_pretty(&bug).unwrap_or_default());
                    }
                    Err(e) => {
                        eprintln!("Error: {}", e);
                    }
                }
            }

            // -------------------- activate --------------------
            BugSubcommand::Activate { id } => {
                if dry_run {
                    println!("[DRY-RUN] Would call BugApi::activate()");
                    println!("  URL: {}/api.php/v1/bugs/{}/activate", config.url, id);
                    return;
                }

                match BugApi::activate(&client, *id).await {
                    Ok(bug) => {
                        println!("{}", serde_json::to_string_pretty(&bug).unwrap_or_default());
                    }
                    Err(e) => {
                        eprintln!("Error: {}", e);
                    }
                }
            }

            // -------------------- delete --------------------
            BugSubcommand::Delete { id } => {
                if dry_run {
                    println!("[DRY-RUN] Would call BugApi::delete()");
                    println!("  URL: {}/api.php/v1/bugs/{}", config.url, id);
                    return;
                }

                match BugApi::delete(&client, *id).await {
                    Ok(_) => {
                        println!("Bug {} deleted successfully", id);
                    }
                    Err(e) => {
                        eprintln!("Error: {}", e);
                    }
                }
            }
        }
    });
}
