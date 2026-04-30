//! ZenTao Task(任务)命令模块
//!
//! CLI 命令入口，调用 TaskApi 处理用户请求
//!
//! # 禅道概念解释
//! - Task（任务）：具体的开发工作，是完成 Story 的具体步骤
//! - 任务有三类工时字段：estimate（预估）、consumed（已消耗）、left（剩余）

use clap::Subcommand;

use crate::api::{ApiClient, CreateTaskRequest, TaskApi, UpdateTaskRequest, TaskEstimate};
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
    /// 删除任务
    #[command(name = "+delete")]
    Delete {
        /// 任务 ID
        id: u64,
    },
    /// 开始任务
    #[command(name = "+start")]
    Start {
        /// 任务 ID
        id: u64,
    },
    /// 暂停任务
    #[command(name = "+pause")]
    Pause {
        /// 任务 ID
        id: u64,
    },
    /// 继续任务
    #[command(name = "+restart")]
    Restart {
        /// 任务 ID
        id: u64,
    },
    /// 完成任务
    #[command(name = "+finish")]
    Finish {
        /// 任务 ID
        id: u64,
    },
    /// 关闭任务
    #[command(name = "+close")]
    Close {
        /// 任务 ID
        id: u64,
    },
    /// 添加任务日志（工时）
    #[command(name = "+estimate")]
    Estimate {
        /// 任务 ID
        id: u64,
        /// 消耗工时（小时）
        #[arg(long)]
        consumed: f64,
        /// 剩余工时（小时）
        #[arg(long)]
        left: f64,
        /// 备注（可选）
        #[arg(long)]
        notes: Option<String>,
    },
    /// 获取任务日志
    #[command(name = "+get-estimate")]
    GetEstimate {
        /// 任务 ID
        id: u64,
    },
}

// ============================================================
// 命令执行入口
// ============================================================

/// 执行 Task 相关命令
pub fn run(cmd: &TaskAction, config: &Config, _format: OutputFormat, dry_run: bool) {
    let rt = tokio::runtime::Runtime::new()
        .expect("Failed to create Tokio runtime - system may be out of memory");

    rt.block_on(async {
        let client = ApiClient::new(&config.url, config.token.clone())
            .with_api_version(config.api_version.as_deref().unwrap_or("v1"));

        match cmd {
            // -------------------- list 命令 --------------------
            TaskAction::List {
                project,
                assigned_to,
            } => {
                if dry_run {
                    println!("[DRY-RUN] Would call TaskApi::list()");
                    println!("  Step 1: GET /api.php/v1/projects/{}/executions", project);
                    println!("  Step 2: For each execution, GET /api.php/v1/executions/{{id}}/tasks");
                    if let Some(a) = assigned_to {
                        println!("  Filter: assignedTo={}", a);
                    }
                    return;
                }
                // 调用任务列表 API
                // 传入 project ID 和可选的指派人筛选
                match TaskApi::list(&client, *project, assigned_to.clone()).await {
                    Ok(tasks) => {
                        println!(
                            "{}",
                            serde_json::to_string_pretty(&tasks).unwrap_or_default()
                        );
                    }
                    Err(e) => {
                        eprintln!("Error: {}", e);
                    }
                }
            }

            // -------------------- get 命令 --------------------
            TaskAction::Get { id } => {
                if dry_run {
                    println!("[DRY-RUN] Would call TaskApi::get()");
                    println!("  URL: {}/api.php/v1/tasks/{}", config.url, id);
                    return;
                }
                match TaskApi::get(&client, *id).await {
                    Ok(task) => {
                        println!(
                            "{}",
                            serde_json::to_string_pretty(&task).unwrap_or_default()
                        );
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

                if dry_run {
                    println!("[DRY-RUN] Would call TaskApi::create()");
                    println!("  URL: {}/api.php/v1/tasks", config.url);
                    println!(
                        "  Body: {}",
                        serde_json::to_string_pretty(&req).unwrap_or_default()
                    );
                    return;
                }

                match TaskApi::create(&client, &req).await {
                    Ok(task) => {
                        println!(
                            "{}",
                            serde_json::to_string_pretty(&task).unwrap_or_default()
                        );
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

                if dry_run {
                    println!("[DRY-RUN] Would call TaskApi::update()");
                    println!("  URL: {}/api.php/v1/tasks/{}", config.url, id);
                    println!(
                        "  Body: {}",
                        serde_json::to_string_pretty(&req).unwrap_or_default()
                    );
                    return;
                }

                match TaskApi::update(&client, *id, &req).await {
                    Ok(task) => {
                        println!(
                            "{}",
                            serde_json::to_string_pretty(&task).unwrap_or_default()
                        );
                    }
                    Err(e) => {
                        eprintln!("Error: {}", e);
                    }
                }
            }

            // -------------------- delete 命令 --------------------
            TaskAction::Delete { id } => {
                if dry_run {
                    println!("[DRY-RUN] Would call TaskApi::delete()");
                    println!("  URL: {}/api.php/v1/tasks/{}", config.url, id);
                    return;
                }
                match TaskApi::delete(&client, *id).await {
                    Ok(_) => {
                        println!("Task {} deleted successfully", id);
                    }
                    Err(e) => {
                        eprintln!("Error: {}", e);
                    }
                }
            }

            // -------------------- start 命令 --------------------
            TaskAction::Start { id } => {
                if dry_run {
                    println!("[DRY-RUN] Would call TaskApi::start()");
                    println!("  URL: {}/api.php/v1/tasks/{}/start", config.url, id);
                    return;
                }
                match TaskApi::start(&client, *id).await {
                    Ok(task) => {
                        println!(
                            "{}",
                            serde_json::to_string_pretty(&task).unwrap_or_default()
                        );
                    }
                    Err(e) => {
                        eprintln!("Error: {}", e);
                    }
                }
            }

            // -------------------- pause 命令 --------------------
            TaskAction::Pause { id } => {
                if dry_run {
                    println!("[DRY-RUN] Would call TaskApi::pause()");
                    println!("  URL: {}/api.php/v1/tasks/{}/pause", config.url, id);
                    return;
                }
                match TaskApi::pause(&client, *id).await {
                    Ok(task) => {
                        println!(
                            "{}",
                            serde_json::to_string_pretty(&task).unwrap_or_default()
                        );
                    }
                    Err(e) => {
                        eprintln!("Error: {}", e);
                    }
                }
            }

            // -------------------- restart 命令 --------------------
            TaskAction::Restart { id } => {
                if dry_run {
                    println!("[DRY-RUN] Would call TaskApi::restart()");
                    println!("  URL: {}/api.php/v1/tasks/{}/restart", config.url, id);
                    return;
                }
                match TaskApi::restart(&client, *id).await {
                    Ok(task) => {
                        println!(
                            "{}",
                            serde_json::to_string_pretty(&task).unwrap_or_default()
                        );
                    }
                    Err(e) => {
                        eprintln!("Error: {}", e);
                    }
                }
            }

            // -------------------- finish 命令 --------------------
            TaskAction::Finish { id } => {
                if dry_run {
                    println!("[DRY-RUN] Would call TaskApi::finish()");
                    println!("  URL: {}/api.php/v1/tasks/{}/finish", config.url, id);
                    return;
                }
                match TaskApi::finish(&client, *id).await {
                    Ok(task) => {
                        println!(
                            "{}",
                            serde_json::to_string_pretty(&task).unwrap_or_default()
                        );
                    }
                    Err(e) => {
                        eprintln!("Error: {}", e);
                    }
                }
            }

            // -------------------- close 命令 --------------------
            TaskAction::Close { id } => {
                if dry_run {
                    println!("[DRY-RUN] Would call TaskApi::close()");
                    println!("  URL: {}/api.php/v1/tasks/{}/close", config.url, id);
                    return;
                }
                match TaskApi::close(&client, *id).await {
                    Ok(task) => {
                        println!(
                            "{}",
                            serde_json::to_string_pretty(&task).unwrap_or_default()
                        );
                    }
                    Err(e) => {
                        eprintln!("Error: {}", e);
                    }
                }
            }

            // -------------------- estimate 命令 --------------------
            TaskAction::Estimate { id, consumed, left, notes } => {
                if dry_run {
                    println!("[DRY-RUN] Would call TaskApi::add_estimate()");
                    println!("  URL: {}/api.php/v1/tasks/{}/estimate", config.url, id);
                    println!("  consumed: {}, left: {}", consumed, left);
                    if let Some(n) = notes {
                        println!("  notes: {}", n);
                    }
                    return;
                }
                match TaskApi::add_estimate(&client, *id, *consumed, *left, notes.clone()).await {
                    Ok(estimate) => {
                        println!(
                            "{}",
                            serde_json::to_string_pretty(&estimate).unwrap_or_default()
                        );
                    }
                    Err(e) => {
                        eprintln!("Error: {}", e);
                    }
                }
            }

            // -------------------- get-estimate 命令 --------------------
            TaskAction::GetEstimate { id } => {
                if dry_run {
                    println!("[DRY-RUN] Would call TaskApi::get_estimates()");
                    println!("  URL: {}/api.php/v1/tasks/{}/estimate", config.url, id);
                    return;
                }
                match TaskApi::get_estimates(&client, *id).await {
                    Ok(estimates) => {
                        println!(
                            "{}",
                            serde_json::to_string_pretty(&estimates).unwrap_or_default()
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
