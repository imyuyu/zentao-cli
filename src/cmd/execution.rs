//! ZenTao Execution(执行)命令模块
//!
//! CLI 命令入口，调用 ExecutionApi 处理用户请求
//!
//! # 禅道概念解释
//! - Execution（执行）：也称为迭代或里程碑，是项目中的具体执行单元
//! - 执行类型包括：iteration（迭代）、milestone（里程碑）

use clap::Subcommand;

use crate::api::{ApiClient, ExecutionApi};
use crate::core::{Config, OutputFormat};

// ============================================================
// 子命令定义
// ============================================================

/// Execution 子命令枚举
///
/// 定义 execution 命令支持的子命令：
/// - list: 列出执行
/// - get: 获取执行详情
#[derive(Subcommand, Clone, Debug)]
pub enum ExecutionAction {
    /// 列出项目下的执行
    #[command(name = "+list")]
    List {
        /// 项目 ID（必填）
        #[arg(long)]
        project: Option<u64>,
    },
    /// 获取执行详情
    #[command(name = "+get")]
    Get {
        /// 执行 ID
        id: u64,
    },
}

// ============================================================
// 命令执行入口
// ============================================================

/// 执行 Execution 相关命令
pub fn run(cmd: &ExecutionAction, config: &Config, _format: OutputFormat) {
    let rt = tokio::runtime::Runtime::new()
        .expect("Failed to create Tokio runtime - system may be out of memory");

    rt.block_on(async {
        let client = ApiClient::new(&config.url, config.token.clone())
            .with_api_version(config.api_version.as_deref().unwrap_or("v1"));

        match cmd {
            // -------------------- list 命令 --------------------
            ExecutionAction::List { project } => {
                match ExecutionApi::list(&client, *project).await {
                    Ok(executions) => {
                        println!("{}", serde_json::to_string_pretty(&executions).unwrap_or_default());
                    }
                    Err(e) => {
                        eprintln!("Error: {}", e);
                    }
                }
            }

            // -------------------- get 命令 --------------------
            ExecutionAction::Get { id } => {
                match ExecutionApi::get(&client, *id).await {
                    Ok(execution) => {
                        println!("{}", serde_json::to_string_pretty(&execution).unwrap_or_default());
                    }
                    Err(e) => {
                        eprintln!("Error: {}", e);
                    }
                }
            }
        }
    })
}
