//! ZenTao Execution(执行)命令模块
//!
//! CLI 命令入口，调用 ExecutionApi 处理用户请求
//!
//! # 禅道概念解释
//! - Execution（执行）：也称为迭代或里程碑，是项目中的具体执行单元
//! - 执行类型包括：iteration（迭代）、milestone（里程碑）

use clap::Subcommand;

use crate::api::{ApiClient, ExecutionApi};
use crate::api::execution::{CreateExecutionRequest, UpdateExecutionRequest};
use crate::core::{Config, OutputFormat};

// ============================================================
// 子命令定义
// ============================================================

/// Execution 子命令枚举
///
/// 定义 execution 命令支持的子命令：
/// - list: 列出执行
/// - get: 获取执行详情
/// - create: 创建执行
/// - update: 更新执行
/// - delete: 删除执行
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
    /// 创建执行
    #[command(name = "+create")]
    Create {
        /// 执行名称（必填）
        #[arg(long)]
        name: String,
        /// 所属项目 ID（必填）
        #[arg(long)]
        project: u64,
        /// 执行类型：iteration/milestone
        #[arg(long)]
        type_: Option<String>,
        /// 开始日期，格式：2024-01-01
        #[arg(long)]
        begin: Option<String>,
        /// 结束日期，格式：2024-01-14
        #[arg(long)]
        end: Option<String>,
        /// 预计工期（天）
        #[arg(long)]
        days: Option<u64>,
        /// 执行描述
        #[arg(long)]
        desc: Option<String>,
    },
    /// 更新执行
    #[command(name = "+update")]
    Update {
        /// 执行 ID（必填）
        id: u64,
        /// 新名称
        #[arg(long)]
        name: Option<String>,
        /// 新状态：wait/doing/closed/suspended
        #[arg(long)]
        status: Option<String>,
        /// 开始日期
        #[arg(long)]
        begin: Option<String>,
        /// 结束日期
        #[arg(long)]
        end: Option<String>,
        /// 预计工期（天）
        #[arg(long)]
        days: Option<u64>,
        /// 执行描述
        #[arg(long)]
        desc: Option<String>,
    },
    /// 删除执行
    #[command(name = "+delete")]
    Delete {
        /// 执行 ID（必填）
        id: u64,
    },
}

// ============================================================
// 命令执行入口
// ============================================================

/// 执行 Execution 相关命令
pub fn run(cmd: &ExecutionAction, config: &Config, _format: OutputFormat, dry_run: bool) {
    let rt = tokio::runtime::Runtime::new()
        .expect("Failed to create Tokio runtime - system may be out of memory");

    rt.block_on(async {
        let client = ApiClient::new(&config.url, config.token.clone())
            .with_api_version(config.api_version.as_deref().unwrap_or("v1"));

        match cmd {
            // -------------------- list 命令 --------------------
            ExecutionAction::List { project } => {
                if dry_run {
                    println!("[DRY-RUN] Would call ExecutionApi::list()");
                    println!("  URL: {}/api.php/v1/executions", config.url);
                    println!("  Params:");
                    if let Some(p) = project {
                        println!("    project: {}", p);
                    }
                    return;
                }
                match ExecutionApi::list(&client, *project).await {
                    Ok(executions) => {
                        println!(
                            "{}",
                            serde_json::to_string_pretty(&executions).unwrap_or_default()
                        );
                    }
                    Err(e) => {
                        eprintln!("Error: {}", e);
                    }
                }
            }

            // -------------------- get 命令 --------------------
            ExecutionAction::Get { id } => {
                if dry_run {
                    println!("[DRY-RUN] Would call ExecutionApi::get()");
                    println!("  URL: {}/api.php/v1/executions/{}", config.url, id);
                    return;
                }
                match ExecutionApi::get(&client, *id).await {
                    Ok(execution) => {
                        println!(
                            "{}",
                            serde_json::to_string_pretty(&execution).unwrap_or_default()
                        );
                    }
                    Err(e) => {
                        eprintln!("Error: {}", e);
                    }
                }
            }

            // -------------------- create 命令 --------------------
            ExecutionAction::Create {
                name,
                project,
                type_,
                begin,
                end,
                days,
                desc,
            } => {
                if dry_run {
                    println!("[DRY-RUN] Would call ExecutionApi::create()");
                    println!("  URL: {}/api.php/v1/projects/{}/executions", config.url, project);
                    println!("  Body:");
                    println!("    name: {}", name);
                    println!("    project: {}", project);
                    if let Some(t) = type_ {
                        println!("    type: {}", t);
                    }
                    if let Some(b) = begin {
                        println!("    begin: {}", b);
                    }
                    if let Some(e) = end {
                        println!("    end: {}", e);
                    }
                    if let Some(d) = days {
                        println!("    days: {}", d);
                    }
                    if let Some(d) = desc {
                        println!("    desc: {}", d);
                    }
                    return;
                }
                let req = CreateExecutionRequest {
                    name: name.clone(),
                    project: *project,
                    type_: type_.clone(),
                    begin: begin.clone(),
                    end: end.clone(),
                    days: *days,
                    desc: desc.clone(),
                };
                match ExecutionApi::create(&client, *project, &req).await {
                    Ok(execution) => {
                        println!(
                            "{}",
                            serde_json::to_string_pretty(&execution).unwrap_or_default()
                        );
                    }
                    Err(e) => {
                        eprintln!("Error: {}", e);
                    }
                }
            }

            // -------------------- update 命令 --------------------
            ExecutionAction::Update {
                id,
                name,
                status,
                begin,
                end,
                days,
                desc,
            } => {
                if dry_run {
                    println!("[DRY-RUN] Would call ExecutionApi::update()");
                    println!("  URL: {}/api.php/v1/executions/{}", config.url, id);
                    println!("  Body:");
                    if let Some(n) = name {
                        println!("    name: {}", n);
                    }
                    if let Some(s) = status {
                        println!("    status: {}", s);
                    }
                    if let Some(b) = begin {
                        println!("    begin: {}", b);
                    }
                    if let Some(e) = end {
                        println!("    end: {}", e);
                    }
                    if let Some(d) = days {
                        println!("    days: {}", d);
                    }
                    if let Some(d) = desc {
                        println!("    desc: {}", d);
                    }
                    return;
                }
                let req = UpdateExecutionRequest {
                    name: name.clone(),
                    status: status.clone(),
                    begin: begin.clone(),
                    end: end.clone(),
                    days: *days,
                    desc: desc.clone(),
                };
                match ExecutionApi::update(&client, *id, &req).await {
                    Ok(execution) => {
                        println!(
                            "{}",
                            serde_json::to_string_pretty(&execution).unwrap_or_default()
                        );
                    }
                    Err(e) => {
                        eprintln!("Error: {}", e);
                    }
                }
            }

            // -------------------- delete 命令 --------------------
            ExecutionAction::Delete { id } => {
                if dry_run {
                    println!("[DRY-RUN] Would call ExecutionApi::delete()");
                    println!("  URL: {}/api.php/v1/executions/{}", config.url, id);
                    return;
                }
                match ExecutionApi::delete(&client, *id).await {
                    Ok(()) => {
                        println!("Execution {} deleted successfully", id);
                    }
                    Err(e) => {
                        eprintln!("Error: {}", e);
                    }
                }
            }
        }
    })
}
