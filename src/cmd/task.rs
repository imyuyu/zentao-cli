//! ZenTao Task(任务)命令模块
//!
//! CLI 命令入口，调用 TaskApi 处理用户请求
//!
//! # 禅道概念解释
//! - Task（任务）：具体的开发工作，是完成 Story 的具体步骤
//! - 任务有三类工时字段：estimate（预估）、consumed（已消耗）、left（剩余）

use clap::Subcommand;

use crate::api::{ApiClient, CreateTaskRequest, TaskApi, UpdateTaskRequest};
use crate::core::{Config, OutputFormat};

// ============================================================
// 子命令定义
// ============================================================

/// Task 子命令枚举
///
/// 定义 task 命令支持的子命令：
/// - list: 列出任务
/// - get: 获取任务详情
/// - create: 创建任务
/// - update: 更新任务
#[derive(Subcommand, Clone, Debug)]
pub enum TaskAction {
    /// 列出项目下的任务
    #[command(name = "+list")]
    List {
        /// 项目 ID（必填）
        /// 使用 #[arg(long)] 表示必须使用 --project 参数
        #[arg(long)]
        project: u64,
        /// 按指派人筛选（可选）
        /// 使用 Option<String> 表示参数可选
        #[arg(long)]
        assigned_to: Option<String>,
    },
    /// 获取任务详情
    #[command(name = "+get")]
    Get {
        /// 任务 ID
        id: u64,
    },
    /// 创建新任务
    #[command(name = "+create")]
    Create {
        /// 任务名称
        #[arg(long)]
        name: String,
        /// 所属项目 ID
        #[arg(long)]
        project: u64,
        /// 优先级 1-5
        #[arg(long)]
        pri: u8,
        /// 任务类型（可选）
        #[arg(long)]
        type_: Option<String>,
        /// 指派给谁（可选）
        #[arg(long)]
        assigned_to: Option<String>,
        /// 预估工时，小时（可选）
        #[arg(long)]
        estimate: Option<f64>,
    },
    /// 更新任务
    #[command(name = "+update")]
    Update {
        /// 任务 ID（位置参数，不需要 -- 前缀）
        id: u64,
        /// 新任务名称（可选）
        #[arg(long)]
        name: Option<String>,
        /// 新状态（可选）
        #[arg(long)]
        status: Option<String>,
        /// 新优先级（可选）
        #[arg(long)]
        pri: Option<u8>,
        /// 新的指派人（可选）
        #[arg(long)]
        assigned_to: Option<String>,
    },
}

// ============================================================
// 命令执行入口
// ============================================================

/// 执行 Task 相关命令
pub fn run(cmd: &TaskAction, config: &Config, _format: OutputFormat) {
    let rt = tokio::runtime::Runtime::new()
        .expect("Failed to create Tokio runtime - system may be out of memory");

    rt.block_on(async {
        let client = ApiClient::new(&config.url, config.token.clone())
            .with_api_version(config.api_version.as_deref().unwrap_or("v1"));

        match cmd {
            // -------------------- list 命令 --------------------
            TaskAction::List { project, assigned_to } => {
                // 调用任务列表 API
                // 传入 project ID 和可选的指派人筛选
                match TaskApi::list(&client, *project, assigned_to.clone()).await {
                    Ok(tasks) => {
                        println!("{}", serde_json::to_string_pretty(&tasks).unwrap_or_default());
                    }
                    Err(e) => {
                        eprintln!("Error: {}", e);
                    }
                }
            }

            // -------------------- get 命令 --------------------
            TaskAction::Get { id } => {
                match TaskApi::get(&client, *id).await {
                    Ok(task) => {
                        println!("{}", serde_json::to_string_pretty(&task).unwrap_or_default());
                    }
                    Err(e) => {
                        eprintln!("Error: {}", e);
                    }
                }
            }

            // -------------------- create 命令 --------------------
            TaskAction::Create {
                name,
                project,
                pri,
                type_,
                assigned_to,
                estimate,
            } => {
                // 构建创建请求结构体
                // 使用 clone() 是因为 req 需要 owned 值
                // 而 cmd 中的参数是引用
                let req = CreateTaskRequest {
                    name: name.clone(),
                    project: *project,
                    pri: *pri,
                    type_: type_.clone(),
                    assigned_to: assigned_to.clone(),
                    estimate: *estimate,
                };

                match TaskApi::create(&client, &req).await {
                    Ok(task) => {
                        println!("{}", serde_json::to_string_pretty(&task).unwrap_or_default());
                    }
                    Err(e) => {
                        eprintln!("Error: {}", e);
                    }
                }
            }

            // -------------------- update 命令 --------------------
            TaskAction::Update {
                id,
                name,
                status,
                pri,
                assigned_to,
            } => {
                // 构建更新请求结构体
                // 注意：UpdateTaskRequest 的所有字段都是 Option
                // 这样可以只更新部分字段
                let req = UpdateTaskRequest {
                    name: name.clone(),
                    status: status.clone(),
                    pri: *pri,
                    assigned_to: assigned_to.clone(),
                };

                match TaskApi::update(&client, *id, &req).await {
                    Ok(task) => {
                        println!("{}", serde_json::to_string_pretty(&task).unwrap_or_default());
                    }
                    Err(e) => {
                        eprintln!("Error: {}", e);
                    }
                }
            }
        }
    })
}
