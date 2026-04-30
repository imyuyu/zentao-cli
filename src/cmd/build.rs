//! ZenTao Build(版本)命令模块
//!
//! CLI 命令入口，调用 BuildApi 处理用户请求

use clap::Subcommand;

use crate::api::{ApiClient, BuildApi};
use crate::core::{Config, OutputFormat};

// ============================================================
// 子命令定义
// ============================================================

/// Build 子命令枚举
///
/// 定义 build 命令支持的子命令：
/// - list: 列出所有版本
/// - get: 获取单个版本详情
#[derive(Subcommand, Clone, Debug)]
pub enum BuildAction {
    /// 列出所有版本
    #[command(name = "+list")]
    List {
        /// 按项目 ID 筛选
        #[arg(long)]
        project: Option<u64>,
        /// 按产品 ID 筛选
        #[arg(long)]
        product: Option<u64>,
        /// 按执行 ID 筛选
        #[arg(long)]
        execution: Option<u64>,
    },
    /// 获取指定版本的详细信息
    #[command(name = "+get")]
    Get {
        /// 版本 ID
        id: u64,
    },
}

// ============================================================
// 命令执行入口
// ============================================================

/// 执行 Build 相关命令
pub fn run(cmd: &BuildAction, config: &Config, _format: OutputFormat, dry_run: bool) {
    let rt = tokio::runtime::Runtime::new()
        .expect("Failed to create Tokio runtime - system may be out of memory");

    rt.block_on(async {
        let client = ApiClient::new(&config.url, config.token.clone())
            .with_api_version(config.api_version.as_deref().unwrap_or("v1"));

        match cmd {
            BuildAction::List { project, product, execution } => {
                if dry_run {
                    if let Some(eid) = execution {
                        println!("[DRY-RUN] Would call BuildApi::list_by_execution()");
                        println!("  URL: {}/api.php/v1/executions/{}/builds", config.url, eid);
                    } else {
                        println!("[DRY-RUN] Would call BuildApi::list()");
                        println!("  URL: {}/api.php/v1/builds", config.url);
                        println!("  Params:");
                        if let Some(p) = project {
                            println!("    project: {}", p);
                        }
                        if let Some(p) = product {
                            println!("    product: {}", p);
                        }
                    }
                    return;
                }

                let builds = if let Some(eid) = execution {
                    BuildApi::list_by_execution(&client, *eid).await
                } else {
                    BuildApi::list(&client, *project, *product).await
                };

                match builds {
                    Ok(builds) => {
                        println!("{}", serde_json::to_string_pretty(&builds).unwrap_or_default());
                    }
                    Err(e) => {
                        eprintln!("Error: {}", e);
                    }
                }
            }

            BuildAction::Get { id } => {
                if dry_run {
                    println!("[DRY-RUN] Would call BuildApi::get()");
                    println!("  URL: {}/api.php/v1/builds/{}", config.url, id);
                    return;
                }
                match BuildApi::get(&client, *id).await {
                    Ok(build) => {
                        println!(
                            "{}",
                            serde_json::to_string_pretty(&build).unwrap_or_default()
                        );
                    }
                    Err(e) => {
                        eprintln!("Error: {}", e);
                    }
                }
            }
        }
    })
}
