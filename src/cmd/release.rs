//! ZenTao Release(发布)命令模块
//!
//! CLI 命令入口，调用 ReleaseApi 处理用户请求

use crate::api::{ApiClient, ReleaseApi};
use crate::cmd::root::ReleaseSubcommand;
use crate::core::{Config, OutputFormat};
use crate::safe_println;

/// 执行 Release 相关命令
///
/// 根据子命令类型调用对应的 API 并输出结果
pub fn run(cmd: &ReleaseSubcommand, config: &Config, _format: OutputFormat, dry_run: bool) {
    let rt = tokio::runtime::Runtime::new().expect("Failed to create Tokio runtime");

    rt.block_on(async {
        let client = ApiClient::new(&config.url, config.token.clone())
            .with_api_version(config.api_version.as_deref().unwrap_or("v1"));

        match cmd {
            // -------------------- list --------------------
            ReleaseSubcommand::List { product, project } => {
                let product_id = config.product_id(*product);
                let project_id = config.project_id(*project);

                if dry_run {
                    if let Some(pid) = product_id {
                        safe_println("[DRY-RUN] Would call ReleaseApi::list_by_product()");
                        println!("  URL: {}/api.php/v1/products/{}/releases", config.url, pid);
                    } else if let Some(pid) = project_id {
                        safe_println("[DRY-RUN] Would call ReleaseApi::list_by_project()");
                        println!("  URL: {}/api.php/v1/projects/{}/releases", config.url, pid);
                    } else {
                        safe_println("[DRY-RUN] Would call ReleaseApi::list()");
                        println!("  URL: {}/api.php/v1/releases", config.url);
                    }
                    return;
                }

                let releases = if let Some(pid) = product_id {
                    ReleaseApi::list_by_product(&client, pid).await
                } else if let Some(pid) = project_id {
                    ReleaseApi::list_by_project(&client, pid).await
                } else {
                    ReleaseApi::list(&client).await
                };

                match releases {
                    Ok(releases) => {
                        println!(
                            "{}",
                            serde_json::to_string_pretty(&releases).unwrap_or_default()
                        );
                    }
                    Err(e) => {
                        eprintln!("Error: {}", e);
                    }
                }
            }

            // -------------------- get --------------------
            ReleaseSubcommand::Get { id } => {
                if dry_run {
                    safe_println("[DRY-RUN] Would call ReleaseApi::get()");
                    println!("  URL: {}/api.php/v1/releases/{}", config.url, id);
                    return;
                }
                match ReleaseApi::get(&client, *id).await {
                    Ok(release) => {
                        println!(
                            "{}",
                            serde_json::to_string_pretty(&release).unwrap_or_default()
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
