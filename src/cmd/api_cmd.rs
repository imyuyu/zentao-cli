use crate::api::ApiClient;
use crate::tui::api_selector::run_selector;
use anyhow::Result;
use clap::Subcommand;

// ============================================================
// Schema 结构定义 (AI-friendly API 自省)
// ============================================================

/// Schema 参数定义
#[derive(Clone, Debug, serde::Serialize)]
pub struct SchemaParam {
    pub name: &'static str,
    pub required: bool,
    pub type_: &'static str,
    pub description: &'static str,
}

/// Schema 方法定义
#[derive(Clone, Debug, serde::Serialize)]
pub struct SchemaMethod {
    pub name: &'static str,
    pub http_method: &'static str,
    pub path: &'static str,
    pub description: &'static str,
    pub params: Vec<SchemaParam>,
}

/// Schema 服务定义
#[derive(Clone, Debug, serde::Serialize)]
pub struct SchemaService {
    pub name: &'static str,
    pub description: &'static str,
    pub methods: Vec<SchemaMethod>,
}

impl SchemaService {
    /// 返回所有 Schema 服务
    pub fn all() -> Vec<Self> {
        vec![
            // Story 服务
            SchemaService {
                name: "story",
                description: "需求/故事管理",
                methods: vec![
                    SchemaMethod {
                        name: "list",
                        http_method: "GET",
                        path: "/api.php/v1/stories",
                        description: "获取故事列表",
                        params: vec![
                            SchemaParam {
                                name: "product",
                                required: false,
                                type_: "u64",
                                description: "产品 ID",
                            },
                            SchemaParam {
                                name: "project",
                                required: false,
                                type_: "u64",
                                description: "项目 ID",
                            },
                            SchemaParam {
                                name: "status",
                                required: false,
                                type_: "string",
                                description: "状态筛选",
                            },
                            SchemaParam {
                                name: "limit",
                                required: false,
                                type_: "u32",
                                description: "返回数量限制",
                            },
                            SchemaParam {
                                name: "page",
                                required: false,
                                type_: "u32",
                                description: "页码",
                            },
                        ],
                    },
                    SchemaMethod {
                        name: "get",
                        http_method: "GET",
                        path: "/api.php/v1/stories/{id}",
                        description: "获取单个故事详情",
                        params: vec![SchemaParam {
                            name: "id",
                            required: true,
                            type_: "u64",
                            description: "故事 ID",
                        }],
                    },
                    SchemaMethod {
                        name: "create",
                        http_method: "POST",
                        path: "/api.php/v1/stories",
                        description: "创建故事",
                        params: vec![
                            SchemaParam {
                                name: "title",
                                required: true,
                                type_: "string",
                                description: "故事标题",
                            },
                            SchemaParam {
                                name: "product",
                                required: true,
                                type_: "u64",
                                description: "所属产品 ID",
                            },
                            SchemaParam {
                                name: "project",
                                required: false,
                                type_: "u64",
                                description: "所属项目 ID",
                            },
                            SchemaParam {
                                name: "pri",
                                required: false,
                                type_: "u8",
                                description: "优先级 (1-4)",
                            },
                            SchemaParam {
                                name: "estimate",
                                required: false,
                                type_: "f64",
                                description: "预计工时",
                            },
                        ],
                    },
                    SchemaMethod {
                        name: "update",
                        http_method: "PUT",
                        path: "/api.php/v1/stories/{id}",
                        description: "更新故事",
                        params: vec![
                            SchemaParam {
                                name: "id",
                                required: true,
                                type_: "u64",
                                description: "故事 ID",
                            },
                            SchemaParam {
                                name: "title",
                                required: false,
                                type_: "string",
                                description: "故事标题",
                            },
                            SchemaParam {
                                name: "status",
                                required: false,
                                type_: "string",
                                description: "状态",
                            },
                            SchemaParam {
                                name: "pri",
                                required: false,
                                type_: "u8",
                                description: "优先级",
                            },
                        ],
                    },
                ],
            },
            // Bug 服务
            SchemaService {
                name: "bug",
                description: "缺陷管理",
                methods: vec![
                    SchemaMethod {
                        name: "list",
                        http_method: "GET",
                        path: "/api.php/v1/bugs",
                        description: "获取缺陷列表",
                        params: vec![
                            SchemaParam {
                                name: "product",
                                required: false,
                                type_: "u64",
                                description: "产品 ID",
                            },
                            SchemaParam {
                                name: "project",
                                required: false,
                                type_: "u64",
                                description: "项目 ID",
                            },
                            SchemaParam {
                                name: "status",
                                required: false,
                                type_: "string",
                                description: "状态筛选",
                            },
                            SchemaParam {
                                name: "severity",
                                required: false,
                                type_: "u8",
                                description: "严重程度",
                            },
                        ],
                    },
                    SchemaMethod {
                        name: "get",
                        http_method: "GET",
                        path: "/api.php/v1/bugs/{id}",
                        description: "获取单个缺陷详情",
                        params: vec![SchemaParam {
                            name: "id",
                            required: true,
                            type_: "u64",
                            description: "缺陷 ID",
                        }],
                    },
                    SchemaMethod {
                        name: "create",
                        http_method: "POST",
                        path: "/api.php/v1/bugs",
                        description: "创建缺陷",
                        params: vec![
                            SchemaParam {
                                name: "title",
                                required: true,
                                type_: "string",
                                description: "缺陷标题",
                            },
                            SchemaParam {
                                name: "product",
                                required: true,
                                type_: "u64",
                                description: "产品 ID",
                            },
                            SchemaParam {
                                name: "severity",
                                required: false,
                                type_: "u8",
                                description: "严重程度 (1-4)",
                            },
                            SchemaParam {
                                name: "pri",
                                required: false,
                                type_: "u8",
                                description: "优先级",
                            },
                        ],
                    },
                    SchemaMethod {
                        name: "update",
                        http_method: "PUT",
                        path: "/api.php/v1/bugs/{id}",
                        description: "更新缺陷",
                        params: vec![
                            SchemaParam {
                                name: "id",
                                required: true,
                                type_: "u64",
                                description: "缺陷 ID",
                            },
                            SchemaParam {
                                name: "status",
                                required: false,
                                type_: "string",
                                description: "状态",
                            },
                            SchemaParam {
                                name: "resolved",
                                required: false,
                                type_: "string",
                                description: "解决方案",
                            },
                        ],
                    },
                ],
            },
            // Task 服务
            SchemaService {
                name: "task",
                description: "任务管理",
                methods: vec![
                    SchemaMethod {
                        name: "list",
                        http_method: "GET",
                        path: "/api.php/v1/tasks",
                        description: "获取任务列表",
                        params: vec![
                            SchemaParam {
                                name: "project",
                                required: false,
                                type_: "u64",
                                description: "项目 ID",
                            },
                            SchemaParam {
                                name: "status",
                                required: false,
                                type_: "string",
                                description: "状态筛选",
                            },
                            SchemaParam {
                                name: "limit",
                                required: false,
                                type_: "u32",
                                description: "返回数量",
                            },
                        ],
                    },
                    SchemaMethod {
                        name: "get",
                        http_method: "GET",
                        path: "/api.php/v1/tasks/{id}",
                        description: "获取单个任务详情",
                        params: vec![SchemaParam {
                            name: "id",
                            required: true,
                            type_: "u64",
                            description: "任务 ID",
                        }],
                    },
                    SchemaMethod {
                        name: "create",
                        http_method: "POST",
                        path: "/api.php/v1/tasks",
                        description: "创建任务",
                        params: vec![
                            SchemaParam {
                                name: "name",
                                required: true,
                                type_: "string",
                                description: "任务名称",
                            },
                            SchemaParam {
                                name: "project",
                                required: true,
                                type_: "u64",
                                description: "所属项目 ID",
                            },
                            SchemaParam {
                                name: "pri",
                                required: false,
                                type_: "u8",
                                description: "优先级",
                            },
                            SchemaParam {
                                name: "estimate",
                                required: false,
                                type_: "f64",
                                description: "预计工时",
                            },
                        ],
                    },
                    SchemaMethod {
                        name: "update",
                        http_method: "PUT",
                        path: "/api.php/v1/tasks/{id}",
                        description: "更新任务",
                        params: vec![
                            SchemaParam {
                                name: "id",
                                required: true,
                                type_: "u64",
                                description: "任务 ID",
                            },
                            SchemaParam {
                                name: "status",
                                required: false,
                                type_: "string",
                                description: "状态",
                            },
                        ],
                    },
                ],
            },
            // Product 服务
            SchemaService {
                name: "product",
                description: "产品管理",
                methods: vec![
                    SchemaMethod {
                        name: "list",
                        http_method: "GET",
                        path: "/api.php/v1/products",
                        description: "获取产品列表",
                        params: vec![],
                    },
                    SchemaMethod {
                        name: "get",
                        http_method: "GET",
                        path: "/api.php/v1/products/{id}",
                        description: "获取单个产品详情",
                        params: vec![SchemaParam {
                            name: "id",
                            required: true,
                            type_: "u64",
                            description: "产品 ID",
                        }],
                    },
                ],
            },
            // Project 服务
            SchemaService {
                name: "project",
                description: "项目管理",
                methods: vec![
                    SchemaMethod {
                        name: "list",
                        http_method: "GET",
                        path: "/api.php/v1/projects",
                        description: "获取项目列表",
                        params: vec![],
                    },
                    SchemaMethod {
                        name: "get",
                        http_method: "GET",
                        path: "/api.php/v1/projects/{id}",
                        description: "获取单个项目详情",
                        params: vec![SchemaParam {
                            name: "id",
                            required: true,
                            type_: "u64",
                            description: "项目 ID",
                        }],
                    },
                ],
            },
            // User 服务
            SchemaService {
                name: "user",
                description: "用户管理",
                methods: vec![
                    SchemaMethod {
                        name: "list",
                        http_method: "GET",
                        path: "/api.php/v1/users",
                        description: "获取用户列表",
                        params: vec![],
                    },
                    SchemaMethod {
                        name: "get",
                        http_method: "GET",
                        path: "/api.php/v1/users/{id}",
                        description: "获取单个用户详情",
                        params: vec![SchemaParam {
                            name: "id",
                            required: true,
                            type_: "u64",
                            description: "用户 ID",
                        }],
                    },
                ],
            },
            // Testcase 服务
            SchemaService {
                name: "testcase",
                description: "测试用例管理",
                methods: vec![
                    SchemaMethod {
                        name: "list",
                        http_method: "GET",
                        path: "/api.php/v1/testcases",
                        description: "获取测试用例列表",
                        params: vec![
                            SchemaParam {
                                name: "product",
                                required: false,
                                type_: "u64",
                                description: "产品 ID",
                            },
                            SchemaParam {
                                name: "project",
                                required: false,
                                type_: "u64",
                                description: "项目 ID",
                            },
                        ],
                    },
                    SchemaMethod {
                        name: "get",
                        http_method: "GET",
                        path: "/api.php/v1/testcases/{id}",
                        description: "获取单个测试用例详情",
                        params: vec![SchemaParam {
                            name: "id",
                            required: true,
                            type_: "u64",
                            description: "用例 ID",
                        }],
                    },
                ],
            },
            // Release 服务
            SchemaService {
                name: "release",
                description: "发布管理",
                methods: vec![
                    SchemaMethod {
                        name: "list",
                        http_method: "GET",
                        path: "/api.php/v1/releases",
                        description: "获取发布列表",
                        params: vec![],
                    },
                    SchemaMethod {
                        name: "get",
                        http_method: "GET",
                        path: "/api.php/v1/releases/{id}",
                        description: "获取单个发布详情",
                        params: vec![SchemaParam {
                            name: "id",
                            required: true,
                            type_: "u64",
                            description: "发布 ID",
                        }],
                    },
                ],
            },
            // Build 服务
            SchemaService {
                name: "build",
                description: "版本/Build 管理",
                methods: vec![
                    SchemaMethod {
                        name: "list",
                        http_method: "GET",
                        path: "/api.php/v1/builds",
                        description: "获取版本列表",
                        params: vec![
                            SchemaParam {
                                name: "project",
                                required: false,
                                type_: "u64",
                                description: "项目 ID",
                            },
                            SchemaParam {
                                name: "product",
                                required: false,
                                type_: "u64",
                                description: "产品 ID",
                            },
                        ],
                    },
                    SchemaMethod {
                        name: "get",
                        http_method: "GET",
                        path: "/api.php/v1/builds/{id}",
                        description: "获取单个版本详情",
                        params: vec![SchemaParam {
                            name: "id",
                            required: true,
                            type_: "u64",
                            description: "版本 ID",
                        }],
                    },
                ],
            },
            // Execution 服务
            SchemaService {
                name: "execution",
                description: "执行/迭代管理",
                methods: vec![
                    SchemaMethod {
                        name: "list",
                        http_method: "GET",
                        path: "/api.php/v1/executions",
                        description: "获取执行列表",
                        params: vec![SchemaParam {
                            name: "project",
                            required: false,
                            type_: "u64",
                            description: "项目 ID",
                        }],
                    },
                    SchemaMethod {
                        name: "get",
                        http_method: "GET",
                        path: "/api.php/v1/executions/{id}",
                        description: "获取单个执行详情",
                        params: vec![SchemaParam {
                            name: "id",
                            required: true,
                            type_: "u64",
                            description: "执行 ID",
                        }],
                    },
                ],
            },
            // Doc 服务
            SchemaService {
                name: "doc",
                description: "文档管理",
                methods: vec![
                    SchemaMethod {
                        name: "list",
                        http_method: "GET",
                        path: "/api.php/v1/docs",
                        description: "获取文档列表",
                        params: vec![],
                    },
                    SchemaMethod {
                        name: "get",
                        http_method: "GET",
                        path: "/api.php/v1/docs/{id}",
                        description: "获取单个文档详情",
                        params: vec![SchemaParam {
                            name: "id",
                            required: true,
                            type_: "u64",
                            description: "文档 ID",
                        }],
                    },
                ],
            },
        ]
    }

    /// 按服务名筛选
    pub fn filter(services: &[Self], name: &str) -> Vec<Self> {
        services
            .iter()
            .filter(|s| s.name.to_lowercase().contains(&name.to_lowercase()))
            .cloned()
            .collect()
    }
}

// ============================================================
// API 端点定义
// ============================================================

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
            ApiEndpoint {
                name: "获取 Token",
                method: "POST",
                path: "/api.php/v1/tokens",
                description: "登录获取访问令牌",
            },
            // 用户
            ApiEndpoint {
                name: "当前用户",
                method: "GET",
                path: "/api.php/v1/user",
                description: "获取当前登录用户信息",
            },
            // 产品
            ApiEndpoint {
                name: "产品列表",
                method: "GET",
                path: "/api.php/v1/products",
                description: "获取产品列表",
            },
            ApiEndpoint {
                name: "产品详情",
                method: "GET",
                path: "/api.php/v1/products/{id}",
                description: "获取单个产品详情",
            },
            ApiEndpoint {
                name: "产品Bug列表",
                method: "GET",
                path: "/api.php/v1/products/{id}/bugs",
                description: "获取指定产品的Bug列表",
            },
            ApiEndpoint {
                name: "产品故事列表",
                method: "GET",
                path: "/api.php/v1/products/{id}/stories",
                description: "获取指定产品的故事列表",
            },
            // 项目
            ApiEndpoint {
                name: "项目列表",
                method: "GET",
                path: "/api.php/v1/projects",
                description: "获取项目列表",
            },
            ApiEndpoint {
                name: "项目详情",
                method: "GET",
                path: "/api.php/v1/projects/{id}",
                description: "获取单个项目详情",
            },
            ApiEndpoint {
                name: "项目任务列表",
                method: "GET",
                path: "/api.php/v1/projects/{id}/tasks",
                description: "获取指定项目的任务列表",
            },
            ApiEndpoint {
                name: "项目故事列表",
                method: "GET",
                path: "/api.php/v1/projects/{id}/stories",
                description: "获取指定项目的故事列表",
            },
            // 任务
            ApiEndpoint {
                name: "任务详情",
                method: "GET",
                path: "/api.php/v1/tasks/{id}",
                description: "获取单个任务详情",
            },
            // 测试用例
            ApiEndpoint {
                name: "用例列表",
                method: "GET",
                path: "/api.php/v1/testcases",
                description: "获取测试用例列表",
            },
            ApiEndpoint {
                name: "用例详情",
                method: "GET",
                path: "/api.php/v1/testcases/{id}",
                description: "获取单个测试用例详情",
            },
            // 文档
            ApiEndpoint {
                name: "文档列表",
                method: "GET",
                path: "/api.php/v1/docs",
                description: "获取文档列表",
            },
            ApiEndpoint {
                name: "文档详情",
                method: "GET",
                path: "/api.php/v1/docs/{id}",
                description: "获取单个文档详情",
            },
        ]
    }
}

#[derive(Subcommand, Clone, Debug)]
pub enum ApiSubcommand {
    /// 测试 API 连接是否正常
    #[command(name = "test")]
    Test,
    /// 列出所有可用的 API 端点
    #[command(name = "endpoints")]
    Endpoints,
    /// 交互式选择并调用 API 端点
    #[command(name = "list")]
    List {
        /// 直接指定端点名称（跳过交互选择）
        #[arg(long)]
        name: Option<String>,
    },
    /// AI 友好的 API Schema 自省
    #[command(name = "schema")]
    Schema {
        /// 按服务名筛选（如 story, bug, task）
        #[arg(long)]
        service: Option<String>,
        /// 输出格式: table (默认) 或 json
        #[arg(long, default_value = "table")]
        output: String,
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
                    println!(
                        "Response: {}",
                        serde_json::to_string_pretty(&data).unwrap_or_default()
                    );
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
                if let Some(endpoint) = endpoints
                    .iter()
                    .find(|e| e.name.contains(name_str) || e.path.contains(name_str))
                {
                    if let Err(e) = call_api_endpoint(config, endpoint).await {
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

                match call_api_endpoint(config, &endpoint).await {
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
                        if std::io::stdin().read_line(&mut input).is_ok() {
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
        ApiSubcommand::Schema { service, output } => {
            let services = SchemaService::all();

            // 按服务名筛选
            let filtered: Vec<SchemaService> = if let Some(s) = service {
                SchemaService::filter(&services, s)
            } else {
                services
            };

            if filtered.is_empty() {
                println!(
                    "未找到匹配的服务: {}",
                    service.as_ref().unwrap_or(&String::new())
                );
                return Ok(());
            }

            match output.as_str() {
                "json" => {
                    // JSON 格式输出
                    println!(
                        "{}",
                        serde_json::to_string_pretty(&filtered).unwrap_or_default()
                    );
                }
                _ => {
                    // Table 格式输出（默认）
                    for svc in &filtered {
                        println!("{} - {}", svc.name, svc.description);
                        println!("{}", "=".repeat(60));
                        for method in &svc.methods {
                            println!(
                                "  {}({:?})",
                                method.name,
                                method.params.iter().map(|p| p.name).collect::<Vec<_>>()
                            );
                            println!("    {} {}", method.http_method, method.path);
                            println!("    {}", method.description);
                            if !method.params.is_empty() {
                                println!("    参数:");
                                for param in &method.params {
                                    let req = if param.required {
                                        "[必填]"
                                    } else {
                                        "[可选]"
                                    };
                                    println!(
                                        "      - {}: {} {} - {}",
                                        param.name, param.type_, req, param.description
                                    );
                                }
                            }
                            println!();
                        }
                        println!();
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
        "GET" => match client.get::<serde_json::Value>(endpoint.path).await {
            Ok(data) => {
                println!("✓ 成功!");
                println!(
                    "{}",
                    serde_json::to_string_pretty(&data).unwrap_or_default()
                );
            }
            Err(e) => {
                return Err(e);
            }
        },
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
