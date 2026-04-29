//! ZenTao Shortcuts(快捷命令)模块
//!
//! + 前缀的快捷命令，提供快速访问常用功能
//!
//! 支持多种输出格式：json, pretty, table, ndjson, csv
//! 支持分页：--page-all, --page-limit, --page-delay
//! 支持 dry-run 模式

use clap::Parser;

use crate::api::{ApiClient, BugApi, ProductApi, ProjectApi, StoryApi, TaskApi};
use crate::core::{Config, OutputFormat};
use crate::core::output::format_output;

/// 默认每页数量
const DEFAULT_PAGE_SIZE: u32 = 100;
/// 最大每页数量
const MAX_PAGE_SIZE: u32 = 500;

/// 分页参数
#[derive(Parser, Clone, Debug, Default)]
pub struct PaginationArgs {
    /// 获取所有数据（不分页）
    #[arg(long)]
    pub page_all: bool,
    /// 每页数量（默认 100，最大 500）
    #[arg(long, default_value_t = DEFAULT_PAGE_SIZE)]
    pub page_limit: u32,
    /// 分页请求间隔（毫秒，默认 100）
    #[arg(long, default_value_t = 100)]
    pub page_delay: u32,
}

impl PaginationArgs {
    /// 获取有效的分页大小
    pub fn effective_limit(&self) -> u32 {
        if self.page_all {
            MAX_PAGE_SIZE
        } else {
            self.page_limit.min(MAX_PAGE_SIZE)
        }
    }
}

/// Shortcut 子命令枚举
///
/// 定义 + 前缀快捷命令支持的子命令
#[derive(Parser, Clone, Debug)]
pub enum ShortcutCommand {
    /// 获取产品列表
    #[command(name = "+products", visible_alias = "+products")]
    Products {
        /// 分页参数
        #[command(flatten)]
        pagination: PaginationArgs,
    },
    /// 获取项目列表
    #[command(name = "+projects", visible_alias = "+projects")]
    Projects {
        /// 分页参数
        #[command(flatten)]
        pagination: PaginationArgs,
    },
    /// 获取 Bug 列表
    #[command(name = "+bugs", visible_alias = "+bugs")]
    Bugs {
        /// 产品 ID
        #[arg(long)]
        product: Option<u64>,
        /// 分页参数
        #[command(flatten)]
        pagination: PaginationArgs,
    },
    /// 获取故事列表
    #[command(name = "+stories", visible_alias = "+stories")]
    Stories {
        /// 产品 ID
        #[arg(long)]
        product: Option<u64>,
        /// 分页参数
        #[command(flatten)]
        pagination: PaginationArgs,
    },
    /// 获取任务列表
    #[command(name = "+tasks", visible_alias = "+tasks")]
    Tasks {
        /// 项目 ID
        #[arg(long)]
        project: Option<u64>,
        /// 分页参数
        #[command(flatten)]
        pagination: PaginationArgs,
    },
}

/// 输出列定义
mod columns {
    /// 产品列表列
    pub const PRODUCT: &[&str] = &["id", "name", "code", "status"];
    /// 项目列表列
    pub const PROJECT: &[&str] = &["id", "name", "code", "status"];
    /// Bug 列表列
    pub const BUG: &[&str] = &["id", "title", "status", "severity", "pri", "product"];
    /// Story 列表列
    pub const STORY: &[&str] = &["id", "title", "status", "pri", "product"];
    /// Task 列表列
    pub const TASK: &[&str] = &["id", "name", "status", "pri", "project"];
}

/// Dry-run 模式：只显示要执行的操作，不实际调用 API
pub fn dry_run_report(cmd: &str, params: &[(&str, String)]) {
    println!("[DRY-RUN] Would execute: {}", cmd);
    for (key, value) in params {
        println!("  --{} {}", key, value);
    }
}

/// 执行 Shortcut 相关命令
///
/// 根据子命令类型调用对应的 API 并输出结果
pub fn run(cmd: &ShortcutCommand, config: &Config, format: OutputFormat) {
    let rt = tokio::runtime::Runtime::new()
        .expect("Failed to create Tokio runtime");

    rt.block_on(async {
        let client = ApiClient::new(&config.url, config.token.clone())
            .with_api_version(config.api_version.as_deref().unwrap_or("v1"));

        match cmd {
            // -------------------- +products --------------------
            ShortcutCommand::Products { pagination } => {
                if pagination.page_all {
                    // 分页获取所有产品
                    let all_products = fetch_all_products(&client, &pagination).await;
                    print_items(&all_products, format, columns::PRODUCT);
                } else {
                    match ProductApi::list_with_pagination(&client, 1, pagination.effective_limit()).await {
                        Ok(products) => {
                            print_items(&products, format, columns::PRODUCT);
                        }
                        Err(e) => {
                            eprintln!("Error: {}", e);
                        }
                    }
                }
            }

            // -------------------- +projects --------------------
            ShortcutCommand::Projects { pagination } => {
                if pagination.page_all {
                    let all_projects = fetch_all_projects(&client, &pagination).await;
                    print_items(&all_projects, format, columns::PROJECT);
                } else {
                    match ProjectApi::list_with_pagination(&client, 1, pagination.effective_limit()).await {
                        Ok(projects) => {
                            print_items(&projects, format, columns::PROJECT);
                        }
                        Err(e) => {
                            eprintln!("Error: {}", e);
                        }
                    }
                }
            }

            // -------------------- +bugs --------------------
            ShortcutCommand::Bugs { product, pagination } => {
                let product_id = product.or(config.product_id);
                match product_id {
                    Some(pid) => {
                        if pagination.page_all {
                            let all_bugs = fetch_all_bugs(&client, pid, &pagination).await;
                            print_items(&all_bugs, format, columns::BUG);
                        } else {
                            match BugApi::list_with_pagination(&client, pid, None, None, 1, pagination.effective_limit()).await {
                                Ok(bugs) => {
                                    print_items(&bugs, format, columns::BUG);
                                }
                                Err(e) => {
                                    eprintln!("Error: {}", e);
                                }
                            }
                        }
                    }
                    None => {
                        eprintln!("Error: product ID is required. Use --product or set ZENTAO_PRODUCT_ID");
                    }
                }
            }

            // -------------------- +stories --------------------
            ShortcutCommand::Stories { product, pagination } => {
                let product_id = product.or(config.product_id);
                match product_id {
                    Some(pid) => {
                        if pagination.page_all {
                            let all_stories = fetch_all_stories(&client, pid, &pagination).await;
                            print_items(&all_stories, format, columns::STORY);
                        } else {
                            match StoryApi::list_with_pagination(&client, Some(pid), None, None, 1, pagination.effective_limit()).await {
                                Ok(stories) => {
                                    print_items(&stories, format, columns::STORY);
                                }
                                Err(e) => {
                                    eprintln!("Error: {}", e);
                                }
                            }
                        }
                    }
                    None => {
                        eprintln!("Error: product ID is required. Use --product or set ZENTAO_PRODUCT_ID");
                    }
                }
            }

            // -------------------- +tasks --------------------
            ShortcutCommand::Tasks { project, pagination } => {
                let project_id = project.or(config.project_id);
                match project_id {
                    Some(pid) => {
                        if pagination.page_all {
                            let all_tasks = fetch_all_tasks(&client, pid, &pagination).await;
                            print_items(&all_tasks, format, columns::TASK);
                        } else {
                            match TaskApi::list(&client, pid, None).await {
                                Ok(tasks) => {
                                    print_items(&tasks, format, columns::TASK);
                                }
                                Err(e) => {
                                    eprintln!("Error: {}", e);
                                }
                            }
                        }
                    }
                    None => {
                        eprintln!("Error: project ID is required. Use --project or set ZENTAO_PROJECT_ID");
                    }
                }
            }
        }
    });
}

/// 打印items，使用指定格式
fn print_items<T: serde::Serialize>(items: &[T], format: OutputFormat, columns: &[&str]) {
    let output = format_output(items, format, columns);
    println!("{}", output);
}

// ============================================================
// 分页获取所有数据
// ============================================================

use crate::api::types::{Bug, Story};
use crate::api::{Product, Project, Task};

async fn fetch_all_products(client: &ApiClient, pagination: &PaginationArgs) -> Vec<Product> {
    let mut all_products = Vec::new();
    let limit = pagination.effective_limit();
    let delay = pagination.page_delay;
    let mut page = 1u32;

    loop {
        match ProductApi::list_with_pagination(client, page, limit).await {
            Ok(products) => {
                if products.is_empty() {
                    break;
                }
                all_products.extend(products);

                if !pagination.page_all {
                    break;
                }

                page += 1;

                if delay > 0 {
                    tokio::time::sleep(tokio::time::Duration::from_millis(delay as u64)).await;
                }
            }
            Err(e) => {
                eprintln!("Error fetching products (page {}): {}", page, e);
                break;
            }
        }

        if page > 100 {
            eprintln!("Warning: reached maximum page limit (100)");
            break;
        }
    }

    all_products
}

async fn fetch_all_projects(client: &ApiClient, pagination: &PaginationArgs) -> Vec<Project> {
    let mut all_projects = Vec::new();
    let limit = pagination.effective_limit();
    let delay = pagination.page_delay;
    let mut page = 1u32;

    loop {
        match ProjectApi::list_with_pagination(client, page, limit).await {
            Ok(projects) => {
                if projects.is_empty() {
                    break;
                }
                all_projects.extend(projects);

                if !pagination.page_all {
                    break;
                }

                page += 1;

                if delay > 0 {
                    tokio::time::sleep(tokio::time::Duration::from_millis(delay as u64)).await;
                }
            }
            Err(e) => {
                eprintln!("Error fetching projects (page {}): {}", page, e);
                break;
            }
        }

        if page > 100 {
            eprintln!("Warning: reached maximum page limit (100)");
            break;
        }
    }

    all_projects
}

async fn fetch_all_bugs(client: &ApiClient, product_id: u64, pagination: &PaginationArgs) -> Vec<Bug> {
    let mut all_bugs = Vec::new();
    let delay = pagination.page_delay;
    let limit = pagination.effective_limit();
    let mut page = 1u32;

    loop {
        match BugApi::list_with_pagination(client, product_id, None, None, page, limit).await {
            Ok(bugs) => {
                if bugs.is_empty() {
                    break;
                }
                all_bugs.extend(bugs);

                // 如果不是获取全部，只获取一页
                if !pagination.page_all {
                    break;
                }

                page += 1;

                // 请求间隔，避免过快
                if delay > 0 {
                    tokio::time::sleep(tokio::time::Duration::from_millis(delay as u64)).await;
                }
            }
            Err(e) => {
                eprintln!("Error fetching bugs (page {}): {}", page, e);
                break;
            }
        }

        // 安全限制，防止无限循环
        if page > 100 {
            eprintln!("Warning: reached maximum page limit (100)");
            break;
        }
    }

    all_bugs
}

async fn fetch_all_stories(client: &ApiClient, product_id: u64, pagination: &PaginationArgs) -> Vec<Story> {
    let mut all_stories = Vec::new();
    let delay = pagination.page_delay;
    let limit = pagination.effective_limit();
    let mut page = 1u32;

    loop {
        match StoryApi::list_with_pagination(client, Some(product_id), None, None, page, limit).await {
            Ok(stories) => {
                if stories.is_empty() {
                    break;
                }
                all_stories.extend(stories);

                if !pagination.page_all {
                    break;
                }

                page += 1;

                if delay > 0 {
                    tokio::time::sleep(tokio::time::Duration::from_millis(delay as u64)).await;
                }
            }
            Err(e) => {
                eprintln!("Error fetching stories (page {}): {}", page, e);
                break;
            }
        }

        if page > 100 {
            eprintln!("Warning: reached maximum page limit (100)");
            break;
        }
    }

    all_stories
}

async fn fetch_all_tasks(client: &ApiClient, project_id: u64, _pagination: &PaginationArgs) -> Vec<Task> {
    let mut all_tasks = Vec::new();

    match TaskApi::list(client, project_id, None).await {
        Ok(tasks) => all_tasks.extend(tasks),
        Err(e) => eprintln!("Error fetching tasks: {}", e),
    }

    all_tasks
}
