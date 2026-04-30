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
    /// 创建版本
    #[command(name = "+create")]
    Create {
        /// 版本名称
        #[arg(long)]
        name: String,
        /// 所属项目 ID
        #[arg(long)]
        project: u64,
        /// 所属产品 ID
        #[arg(long)]
        product: u64,
        /// 分支/平台 ID
        #[arg(long)]
        branch: Option<u64>,
        /// SCM 路径
        #[arg(long)]
        scm_path: Option<String>,
        /// CI 名称
        #[arg(long)]
        ci: Option<String>,
        /// 包路径
        #[arg(long)]
        pkg: Option<String>,
    },
    /// 更新版本
    #[command(name = "+update")]
    Update {
        /// 版本 ID
        id: u64,
        /// 版本名称
        #[arg(long)]
        name: Option<String>,
        /// SCM 路径
        #[arg(long)]
        scm_path: Option<String>,
        /// CI 名称
        #[arg(long)]
        ci: Option<String>,
        /// 包路径
        #[arg(long)]
        pkg: Option<String>,
    },
    /// 删除版本
    #[command(name = "+delete")]
    Delete {
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
            BuildAction::List {
                project,
                product,
                execution,
            } => {
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
                        println!(
                            "{}",
                            serde_json::to_string_pretty(&builds).unwrap_or_default()
                        );
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

            BuildAction::Create {
                name,
                project,
                product,
                branch,
                scm_path,
                ci,
                pkg,
            } => {
                if dry_run {
                    println!("[DRY-RUN] Would call BuildApi::create()");
                    println!(
                        "  URL: {}/api.php/v1/projects/{}/builds",
                        config.url, project
                    );
                    println!("  Body: {{");
                    println!("    name: {}", name);
                    println!("    project: {}", project);
                    println!("    product: {}", product);
                    if let Some(b) = branch {
                        println!("    branch: {}", b);
                    }
                    if let Some(ref s) = scm_path {
                        println!("    scm_path: {}", s);
                    }
                    if let Some(ref c) = ci {
                        println!("    ci: {}", c);
                    }
                    if let Some(ref p) = pkg {
                        println!("    pkg: {}", p);
                    }
                    println!("  }}");
                    return;
                }

                let req = crate::api::build::CreateBuildRequest {
                    name: name.clone(),
                    project: *project,
                    product: *product,
                    branch: *branch,
                    scm_path: scm_path.clone(),
                    ci: ci.clone(),
                    pkg: pkg.clone(),
                };

                match BuildApi::create(&client, *project, &req).await {
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

            BuildAction::Update {
                id,
                name,
                scm_path,
                ci,
                pkg,
            } => {
                if dry_run {
                    println!("[DRY-RUN] Would call BuildApi::update()");
                    println!("  URL: {}/api.php/v1/builds/{}", config.url, id);
                    println!("  Body: {{");
                    if let Some(ref n) = name {
                        println!("    name: {}", n);
                    }
                    if let Some(ref s) = scm_path {
                        println!("    scm_path: {}", s);
                    }
                    if let Some(ref c) = ci {
                        println!("    ci: {}", c);
                    }
                    if let Some(ref p) = pkg {
                        println!("    pkg: {}", p);
                    }
                    println!("  }}");
                    return;
                }

                let req = crate::api::build::UpdateBuildRequest {
                    name: name.clone(),
                    scm_path: scm_path.clone(),
                    ci: ci.clone(),
                    pkg: pkg.clone(),
                    file_size: None,
                    generated_at: None,
                };

                match BuildApi::update(&client, *id, &req).await {
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

            BuildAction::Delete { id } => {
                if dry_run {
                    println!("[DRY-RUN] Would call BuildApi::delete()");
                    println!("  URL: {}/api.php/v1/builds/{}", config.url, id);
                    return;
                }

                match BuildApi::delete(&client, *id).await {
                    Ok(_) => {
                        println!("Build {} deleted successfully", id);
                    }
                    Err(e) => {
                        eprintln!("Error: {}", e);
                    }
                }
            }
        }
    })
}
