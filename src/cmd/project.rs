//! ZenTao Project(项目)命令模块
//!
//! CLI 命令入口，调用 ProjectApi 处理用户请求
//!
//! # 禅道概念解释
//! - Product（产品）：业务层面的产品线
//! - Project（项目）：具体的开发项目，是实现产品的具体工作
//! - 一个 Product 下可以有多个 Project

use clap::Subcommand;

use crate::api::{ApiClient, CreateProjectRequest, ProjectApi, UpdateProjectRequest};
use crate::core::{Config, OutputFormat};

// ============================================================
// 子命令定义
// ============================================================

/// Project 子命令枚举
#[derive(Subcommand, Clone, Debug)]
pub enum ProjectAction {
    /// 列出所有项目
    #[command(name = "+list")]
    List,
    /// 获取指定项目的详细信息
    #[command(name = "+get")]
    Get {
        /// 项目 ID
        id: u64,
    },
    /// 创建项目
    #[command(name = "+create")]
    Create {
        /// 项目名称
        #[arg(long)]
        name: String,
        /// 项目代号
        #[arg(long)]
        code: String,
        /// 项目描述
        #[arg(long)]
        desc: Option<String>,
    },
    /// 更新项目
    #[command(name = "+update")]
    Update {
        /// 项目 ID
        id: u64,
        /// 新名称
        #[arg(long)]
        name: Option<String>,
        /// 新状态：wait/doing/closed
        #[arg(long)]
        status: Option<String>,
        /// 新描述
        #[arg(long)]
        desc: Option<String>,
    },
    /// 删除项目
    #[command(name = "+delete")]
    Delete {
        /// 项目 ID
        id: u64,
    },
}

// ============================================================
// 命令执行入口
// ============================================================

/// 执行 Project 相关命令
pub fn run(cmd: &ProjectAction, config: &Config, _format: OutputFormat, dry_run: bool) {
    // 创建 Tokio 运行时
    // 与 product.rs 的模式完全相同，参考那里的详细注释
    let rt = tokio::runtime::Runtime::new()
        .expect("Failed to create Tokio runtime - system may be out of memory");

    rt.block_on(async {
        let client = ApiClient::new(&config.url, config.token.clone())
            .with_api_version(config.api_version.as_deref().unwrap_or("v1"));

        match cmd {
            // -------------------- list 命令 --------------------
            ProjectAction::List => {
                if dry_run {
                    println!("[DRY-RUN] Would call ProjectApi::list()");
                    println!("  URL: {}/api.php/v1/projects", config.url);
                    return;
                }
                match ProjectApi::list(&client).await {
                    Ok(projects) => {
                        println!(
                            "{}",
                            serde_json::to_string_pretty(&projects).unwrap_or_default()
                        );
                    }
                    Err(e) => {
                        eprintln!("Error: {}", e);
                    }
                }
            }

            // -------------------- get 命令 --------------------
            ProjectAction::Get { id } => {
                if dry_run {
                    println!("[DRY-RUN] Would call ProjectApi::get()");
                    println!("  URL: {}/api.php/v1/projects/{}", config.url, id);
                    return;
                }
                match ProjectApi::get(&client, *id).await {
                    Ok(project) => {
                        println!(
                            "{}",
                            serde_json::to_string_pretty(&project).unwrap_or_default()
                        );
                    }
                    Err(e) => {
                        eprintln!("Error: {}", e);
                    }
                }
            }

            // -------------------- create 命令 --------------------
            ProjectAction::Create { name, code, desc } => {
                let req = CreateProjectRequest {
                    name: name.clone(),
                    code: code.clone(),
                    desc: desc.clone(),
                };

                if dry_run {
                    println!("[DRY-RUN] Would call ProjectApi::create()");
                    println!("  URL: {}/api.php/v1/projects", config.url);
                    println!(
                        "  Body: {}",
                        serde_json::to_string_pretty(&req).unwrap_or_default()
                    );
                    return;
                }

                match ProjectApi::create(&client, &req).await {
                    Ok(project) => {
                        println!(
                            "{}",
                            serde_json::to_string_pretty(&project).unwrap_or_default()
                        );
                    }
                    Err(e) => {
                        eprintln!("Error: {}", e);
                    }
                }
            }

            // -------------------- update 命令 --------------------
            ProjectAction::Update {
                id,
                name,
                status,
                desc,
            } => {
                let req = UpdateProjectRequest {
                    name: name.clone(),
                    status: status.clone(),
                    desc: desc.clone(),
                };

                if dry_run {
                    println!("[DRY-RUN] Would call ProjectApi::update()");
                    println!("  URL: {}/api.php/v1/projects/{}", config.url, id);
                    println!(
                        "  Body: {}",
                        serde_json::to_string_pretty(&req).unwrap_or_default()
                    );
                    return;
                }

                match ProjectApi::update(&client, *id, &req).await {
                    Ok(project) => {
                        println!(
                            "{}",
                            serde_json::to_string_pretty(&project).unwrap_or_default()
                        );
                    }
                    Err(e) => {
                        eprintln!("Error: {}", e);
                    }
                }
            }

            // -------------------- delete 命令 --------------------
            ProjectAction::Delete { id } => {
                if dry_run {
                    println!("[DRY-RUN] Would call ProjectApi::delete()");
                    println!("  URL: {}/api.php/v1/projects/{}", config.url, id);
                    return;
                }

                match ProjectApi::delete(&client, *id).await {
                    Ok(_) => {
                        println!("Project {} deleted successfully", id);
                    }
                    Err(e) => {
                        eprintln!("Error: {}", e);
                    }
                }
            }
        }
    })
}
