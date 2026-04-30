//! ZenTao Doc(文档)命令模块
//!
//! CLI 命令入口，调用 DocApi 处理用户请求
//!
//! # 禅道概念解释
//! - Doc（文档）：ZenTao 系统中的文档管理
//! - 文档归属于文档库（lib），可以关联产品或项目

use clap::Subcommand;

use crate::api::{ApiClient, DocApi};
use crate::core::{Config, OutputFormat};
use crate::safe_println;

// ============================================================
// 子命令定义
// ============================================================

/// Doc 子命令枚举
///
/// 定义 doc 命令支持的子命令：
/// - list: 列出所有文档
/// - get: 获取单个文档详情
#[derive(Subcommand, Clone, Debug)]
pub enum DocAction {
    /// 列出所有文档
    #[command(name = "+list")]
    List,
    /// 获取指定文档的详细信息
    #[command(name = "+get")]
    Get {
        /// 文档 ID
        id: u64,
    },
}

// ============================================================
// 命令执行入口
// ============================================================

/// 执行 Doc 相关命令
///
/// # 参数说明
/// - `cmd`: 解析后的子命令
/// - `config`: 全局配置（包含 URL 和 Token）
/// - `_format`: 输出格式（预留参数，当前固定输出 JSON）
pub fn run(cmd: &DocAction, config: &Config, _format: OutputFormat, dry_run: bool) {
    let rt = tokio::runtime::Runtime::new()
        .expect("Failed to create Tokio runtime - system may be out of memory");

    rt.block_on(async {
        let client = ApiClient::new(&config.url, config.token.clone())
            .with_api_version(config.api_version.as_deref().unwrap_or("v1"));

        match cmd {
            // -------------------- list 命令 --------------------
            DocAction::List => {
                if dry_run {
                    safe_println("[DRY-RUN] Would call DocApi::list()");
                    println!("  URL: {}/api.php/v1/docs", config.url);
                    return;
                }
                match DocApi::list(&client).await {
                    Ok(docs) => {
                        println!(
                            "{}",
                            serde_json::to_string_pretty(&docs).unwrap_or_default()
                        );
                    }
                    Err(e) => {
                        eprintln!("Error: {}", e);
                    }
                }
            }

            // -------------------- get 命令 --------------------
            DocAction::Get { id } => {
                if dry_run {
                    safe_println("[DRY-RUN] Would call DocApi::get()");
                    println!("  URL: {}/api.php/v1/docs/{}", config.url, id);
                    return;
                }
                match DocApi::get(&client, *id).await {
                    Ok(doc) => {
                        println!("{}", serde_json::to_string_pretty(&doc).unwrap_or_default());
                    }
                    Err(e) => {
                        eprintln!("Error: {}", e);
                    }
                }
            }
        }
    })
}
