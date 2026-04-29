//! ZenTao Product(产品)命令模块
//!
//! CLI 命令入口，调用 ProductApi 处理用户请求
//!
//! # 禅道概念解释
//! - Product（产品）：业务层面的产品线，可以理解为"产品线"或"产品"
//! - 通常一个公司会有多个产品，每个产品有独立的需求和缺陷管理

use clap::Subcommand;

use crate::api::{ApiClient, ProductApi};
use crate::core::{Config, OutputFormat};

// ============================================================
// 子命令定义
// ============================================================

/// Product 子命令枚举
///
/// 定义 product 命令支持的子命令：
/// - list: 列出所有产品
/// - get: 获取单个产品详情
#[derive(Subcommand, Clone, Debug)]
pub enum ProductAction {
    /// 列出所有产品
    #[command(name = "+list", visible_alias = "+list")]
    List,
    /// 获取指定产品的详细信息
    #[command(name = "+get", visible_alias = "+get")]
    Get {
        /// 产品 ID
        id: u64,
    },
}

// ============================================================
// 命令执行入口
// ============================================================

/// 执行 Product 相关命令
///
/// 根据子命令类型调用对应的 API 并输出结果
///
/// # 参数说明
/// - `cmd`: 解析后的子命令
/// - `config`: 全局配置（包含 URL 和 Token）
/// - `_format`: 输出格式（预留参数，当前固定输出 JSON）
///
/// # 工作流程
/// ```text
/// 用户输入: zentao product list
///           ↓
/// clap 解析: ProductAction::List
///           ↓
/// run() 函数: match cmd
///           ↓
/// 调用 API: ProductApi::list()
///           ↓
/// 输出 JSON: println!()
/// ```
pub fn run(cmd: &ProductAction, config: &Config, _format: OutputFormat) {
    // 创建 Tokio 异步运行时
    // CLI 命令需要手动创建运行时来执行 async 代码
    // 类似于 Java 中的 ExecutorService 或 Go 中的 goroutine 调度器
    let rt = tokio::runtime::Runtime::new()
        .expect("Failed to create Tokio runtime - system may be out of memory");

    // block_on 是阻塞执行异步代码的方式
    // 它会等待整个异步操作完成后才返回
    // 类似于 Java 的 CompletableFuture.join() 或 Go 的 runtime.GOMAXPROCS
    rt.block_on(async {
        // 创建 API 客户端，传入 ZenTao 服务器地址和认证 Token
        // ApiClient 会在每次请求时自动添加 Authorization header
        let client = ApiClient::new(&config.url, config.token.clone())
            .with_api_version(config.api_version.as_deref().unwrap_or("v1"));

        // 根据子命令类型分发处理
        match cmd {
            // -------------------- list 命令 --------------------
            ProductAction::List => {
                // 调用产品列表接口
                match ProductApi::list(&client).await {
                    Ok(products) => {
                        // 成功时输出格式化的 JSON
                        // serde_json::to_string_pretty 相当于 Java 的 ObjectMapper.writerWithDefaultPrettyPrinter()
                        println!("{}", serde_json::to_string_pretty(&products).unwrap_or_default());
                    }
                    Err(e) => {
                        // 错误时输出到 stderr，格式：Error: <错误信息>
                        // 使用 eprintln! 而不是 println! 是为了区分错误输出和正常输出
                        // 类似于 System.err.println() vs System.out.println()
                        eprintln!("Error: {}", e);
                    }
                }
            }

            // -------------------- get 命令 --------------------
            ProductAction::Get { id } => {
                // id 前面的 * 是解引用操作
                // 因为 cmd 参数是引用，id 是 &u64，需要 * 获取 u64 值
                match ProductApi::get(&client, *id).await {
                    Ok(product) => {
                        println!("{}", serde_json::to_string_pretty(&product).unwrap_or_default());
                    }
                    Err(e) => {
                        eprintln!("Error: {}", e);
                    }
                }
            }
        }
    })
}
