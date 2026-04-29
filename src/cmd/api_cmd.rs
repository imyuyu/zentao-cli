use clap::Subcommand;
use anyhow::Result;
use crate::api::ApiClient;
use crate::tui::api_selector::run_selector;

/// API 端点定义
#[derive(Clone, Debug)]
pub struct ApiEndpoint {
    pub name: &'static str,
    pub method: &'static str,
    pub path: &'static str,
    pub description: &'static str,
}

impl ApiEndpoint {
    pub fn all() -> Vec<Self> {
        vec![
            // 认证
            ApiEndpoint { name: "获取 Token", method: "POST", path: "/api.php/v1/tokens", description: "登录获取访问令牌" },
            // 用户
            ApiEndpoint { name: "当前用户", method: "GET", path: "/api.php/v1/user", description: "获取当前登录用户信息" },
            // 产品
            ApiEndpoint { name: "产品列表", method: "GET", path: "/api.php/v1/products", description: "获取产品列表" },
            ApiEndpoint { name: "产品详情", method: "GET", path: "/api.php/v1/products/{id}", description: "获取单个产品详情" },
            ApiEndpoint { name: "产品Bug列表", method: "GET", path: "/api.php/v1/products/{id}/bugs", description: "获取指定产品的Bug列表" },
            ApiEndpoint { name: "产品故事列表", method: "GET", path: "/api.php/v1/products/{id}/stories", description: "获取指定产品的故事列表" },
            // 项目
            ApiEndpoint { name: "项目列表", method: "GET", path: "/api.php/v1/projects", description: "获取项目列表" },
            ApiEndpoint { name: "项目详情", method: "GET", path: "/api.php/v1/projects/{id}", description: "获取单个项目详情" },
            ApiEndpoint { name: "项目任务列表", method: "GET", path: "/api.php/v1/projects/{id}/tasks", description: "获取指定项目的任务列表" },
            ApiEndpoint { name: "项目故事列表", method: "GET", path: "/api.php/v1/projects/{id}/stories", description: "获取指定项目的故事列表" },
            // 任务
            ApiEndpoint { name: "任务详情", method: "GET", path: "/api.php/v1/tasks/{id}", description: "获取单个任务详情" },
            // 测试用例
            ApiEndpoint { name: "用例列表", method: "GET", path: "/api.php/v1/testcases", description: "获取测试用例列表" },
            ApiEndpoint { name: "用例详情", method: "GET", path: "/api.php/v1/testcases/{id}", description: "获取单个测试用例详情" },
            // 文档
            ApiEndpoint { name: "文档列表", method: "GET", path: "/api.php/v1/docs", description: "获取文档列表" },
            ApiEndpoint { name: "文档详情", method: "GET", path: "/api.php/v1/docs/{id}", description: "获取单个文档详情" },
        ]
    }
}

#[derive(Subcommand, Clone, Debug)]
pub enum ApiSubcommand {
    /// 测试 API 连接是否正常
    #[command(name = "+test", visible_alias = "+test")]
    Test,
    /// 列出所有可用的 API 端点
    #[command(name = "+endpoints", visible_alias = "+endpoints")]
    Endpoints,
    /// 交互式选择并调用 API 端点
    #[command(name = "+list", visible_alias = "+list")]
    List {
        /// 直接指定端点名称（跳过交互选择）
        #[arg(long)]
        name: Option<String>,
    },
}

pub async fn run(api_cmd: &ApiSubcommand, config: &crate::core::Config) -> Result<()> {
    match api_cmd {
        ApiSubcommand::Test => {
            println!("Testing ZenTao API connection...");
            println!("URL: {}", config.url);

            if config.url.is_empty() {
                println!("✗ Error: ZENTAO_URL is not set");
                return Ok(());
            }

            let client = ApiClient::new(&config.url, config.token.clone())
                .with_api_version(config.api_version.as_deref().unwrap_or("v1"));

            // Try to fetch current user
            match client.get::<serde_json::Value>("/api.php/v1/user").await {
                Ok(data) => {
                    println!("✓ Connected successfully!");
                    println!("Response: {}", serde_json::to_string_pretty(&data).unwrap_or_default());
                }
                Err(e) => {
                    println!("✗ Connection failed: {}", e);
                }
            }
            Ok(())
        }
        ApiSubcommand::Endpoints => {
            println!("ZenTao API v1 Endpoints");
            println!("======================");
            println!();
            println!("Authentication:");
            println!("  POST   /tokens              - Get access token");
            println!();
            println!("Stories:");
            println!("  GET    /stories              - List stories");
            println!("  GET    /stories/{{id}}        - Get story details");
            println!("  POST   /stories              - Create story");
            println!("  PUT    /stories/{{id}}        - Update story");
            println!();
            println!("Bugs:");
            println!("  GET    /bugs                - List bugs");
            println!("  GET    /bugs/{{id}}          - Get bug details");
            println!("  POST   /bugs                - Create bug");
            println!("  PUT    /bugs/{{id}}          - Update bug");
            println!();
            println!("Products:");
            println!("  GET    /products             - List products");
            println!();
            println!("Users:");
            println!("  GET    /users/me             - Current user info");
            println!("  GET    /users                - List users");
            Ok(())
        }
        ApiSubcommand::List { name } => {
            let endpoints = ApiEndpoint::all();

            if let Some(name) = name {
                // 直接调用指定端点
                let name_str = name.as_str();
                if let Some(endpoint) = endpoints.iter().find(|e| e.name.contains(name_str) || e.path.contains(name_str)) {
                    if let Err(e) = call_api_endpoint(&config, endpoint).await {
                        eprintln!("✗ 调用失败: {}", e);
                    }
                } else {
                    println!("未找到匹配的端点: {}", name);
                    println!("使用 'zentao-cli api list' 查看所有端点");
                }
                return Ok(());
            }

            // 使用 TUI 选择器
            loop {
                let endpoint = match run_selector() {
                    Some(ep) => ep,
                    None => {
                        println!("已取消");
                        break;
                    }
                };

                match call_api_endpoint(&config, &endpoint).await {
                    Ok(_) => {
                        // 调用成功，退出循环
                        break;
                    }
                    Err(e) => {
                        eprintln!("\n✗ 调用失败: {}", e);
                        eprintln!();
                        eprintln!("1. 重新选择 API");
                        eprintln!("2. 退出");
                        eprintln!();
                        eprint!("请选择 (1-2): ");

                        let mut input = String::new();
                        if let Ok(_) = std::io::stdin().read_line(&mut input) {
                            let input = input.trim();
                            if input != "1" {
                                // 选择 2 或其他，退出循环
                                break;
                            }
                            // 选择 1，继续循环重新选择
                        } else {
                            break;
                        }
                    }
                }
            }

            Ok(())
        }
    }
}

async fn call_api_endpoint(config: &crate::core::Config, endpoint: &ApiEndpoint) -> Result<()> {
    if config.url.is_empty() {
        println!("✗ Error: ZENTAO_URL is not set");
        return Ok(());
    }

    println!("\n调用: {} {}", endpoint.method, endpoint.path);
    println!("描述: {}", endpoint.description);

    let client = ApiClient::new(&config.url, config.token.clone())
        .with_api_version(config.api_version.as_deref().unwrap_or("v1"));

    match endpoint.method {
        "GET" => {
            match client.get::<serde_json::Value>(endpoint.path).await {
                Ok(data) => {
                    println!("✓ 成功!");
                    println!("{}", serde_json::to_string_pretty(&data).unwrap_or_default());
                }
                Err(e) => {
                    return Err(e.into());
                }
            }
        }
        "POST" => {
            // POST 请求需要 body，这里只做示例
            println!("⚠ POST 请求需要额外参数，请使用 --data 指定请求体");
        }
        "PUT" => {
            println!("⚠ PUT 请求需要额外参数，请使用 --data 指定请求体");
        }
        _ => {}
    }

    Ok(())
}
