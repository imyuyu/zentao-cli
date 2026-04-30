use crate::cmd::api_cmd::ApiSubcommand;
use crate::cmd::auth::AuthSubcommand;
use crate::cmd::config_cmd::ConfigSubcommand;
use crate::cmd::{
    api_cmd, auth, browse, bug, build, config_cmd, doc, doctor, execution, product, project,
    release, story, task, testcase, user,
};
use crate::core::{load_config, Config, OutputFormat};
use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "zentao-cli")]
#[command(version = "0.0.3")]
#[command(about = "ZenTao CLI - Command line tool for ZenTao project management")]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    #[arg(long, global = true, env = "ZENTAO_URL")]
    url: Option<String>,

    #[arg(long, global = true, env = "ZENTAO_TOKEN")]
    token: Option<String>,

    #[arg(long, global = true, value_enum, default_value = "table")]
    format: OutputFormat,

    /// 预执行模式：只显示将要执行的操作，不实际调用 API
    #[arg(long, global = true)]
    dry_run: bool,
}

#[derive(Clone, Debug, Subcommand)]
enum Commands {
    /// 故事（需求）管理 - 创建、查询、更新故事
    Story {
        #[command(subcommand)]
        action: StorySubcommand,
    },
    /// Bug 管理 - 创建、查询、更新 Bug
    Bug {
        #[command(subcommand)]
        action: BugSubcommand,
    },
    /// 认证管理 - 登录、登出、状态查询
    Auth {
        #[command(subcommand)]
        action: AuthSubcommand,
    },
    /// 配置管理 - 初始化、查看、设置配置
    Config {
        #[command(subcommand)]
        action: ConfigSubcommand,
    },
    /// API 调试 - 交互式调用 ZenTao API 接口
    Api {
        #[command(subcommand)]
        action: ApiSubcommand,
    },
    /// 产品管理 - 查看和管理产品
    Product {
        #[command(subcommand)]
        action: product::ProductAction,
    },
    /// 项目管理 - 查看和管理项目
    Project {
        #[command(subcommand)]
        action: project::ProjectAction,
    },
    /// 任务管理 - 查看和管理任务
    Task {
        #[command(subcommand)]
        action: task::TaskAction,
    },
    /// 用户管理 - 查看用户信息
    User {
        #[command(subcommand)]
        action: user::UserAction,
    },
    /// 测试用例管理 - 查看测试用例
    Testcase {
        #[command(subcommand)]
        action: TestcaseSubcommand,
    },
    /// 发布管理 - 查看发布信息
    Release {
        #[command(subcommand)]
        action: ReleaseSubcommand,
    },
    /// 构建管理 - 查看构建信息
    Build {
        #[command(subcommand)]
        action: build::BuildAction,
    },
    /// 执行管理 - 查看执行进度
    Execution {
        #[command(subcommand)]
        action: execution::ExecutionAction,
    },
    /// 文档管理 - 查看和管理文档
    Doc {
        #[command(subcommand)]
        action: doc::DocAction,
    },
    /// 诊断工具 - 检查配置和网络连接
    Doctor,
    /// Bug 浏览器 - TUI 模式浏览 Bug 列表
    BugBrowse {
        #[arg(long)]
        product: Option<u64>,
    },
    /// 故事浏览器 - TUI 模式浏览故事列表
    StoryBrowse {
        #[arg(long)]
        product: Option<u64>,
    },
}

#[derive(Subcommand, Clone, Debug)]
pub enum StorySubcommand {
    /// 列出需求列表
    #[command(name = "+list")]
    List {
        #[arg(long)]
        product: Option<u64>,
        #[arg(long)]
        project: Option<u64>,
        #[arg(long)]
        status: Option<String>,
    },
    /// 获取需求详情
    #[command(name = "+get")]
    Get { id: u64 },
    /// 创建需求
    #[command(name = "+create")]
    Create {
        #[arg(long)]
        title: String,
        #[arg(long)]
        product: u64,
        #[arg(long)]
        pri: u8,
        #[arg(long)]
        category: Option<String>,
        #[arg(long)]
        spec: Option<String>,
        #[arg(long)]
        estimate: Option<f64>,
    },
    /// 更新需求
    #[command(name = "+update")]
    Update {
        id: u64,
        #[arg(long)]
        title: Option<String>,
        #[arg(long)]
        status: Option<String>,
        #[arg(long)]
        pri: Option<u8>,
        #[arg(long)]
        assigned_to: Option<String>,
    },
}

#[derive(Subcommand, Clone, Debug)]
pub enum BugSubcommand {
    /// 列出 Bug 列表
    #[command(name = "+list")]
    List {
        #[arg(long)]
        product: u64,
        #[arg(long)]
        status: Option<String>,
        #[arg(long)]
        assigned_to: Option<String>,
    },
    /// 获取 Bug 详情
    #[command(name = "+get")]
    Get { id: u64 },
    /// 创建 Bug
    #[command(name = "+create")]
    Create {
        #[arg(long)]
        title: String,
        #[arg(long)]
        product: u64,
        #[arg(long)]
        severity: u8,
        #[arg(long)]
        pri: Option<u8>,
        #[arg(long)]
        type_: Option<String>,
        #[arg(long)]
        steps: Option<String>,
        #[arg(long)]
        story: Option<u64>,
    },
    /// 更新 Bug
    #[command(name = "+update")]
    Update {
        id: u64,
        #[arg(long)]
        title: Option<String>,
        #[arg(long)]
        status: Option<String>,
        #[arg(long)]
        resolution: Option<String>,
        #[arg(long)]
        resolved_build: Option<u64>,
        #[arg(long)]
        assigned_to: Option<String>,
    },
}

#[derive(Subcommand, Clone, Debug)]
pub enum TestcaseSubcommand {
    /// 列出测试用例
    #[command(name = "+list")]
    List {
        #[arg(long)]
        product: Option<u64>,
        #[arg(long)]
        project: Option<u64>,
        #[arg(long)]
        type_: Option<String>,
        #[arg(long)]
        status: Option<String>,
    },
    /// 获取用例详情
    #[command(name = "+get")]
    Get { id: u64 },
}

#[derive(Subcommand, Clone, Debug)]
pub enum ReleaseSubcommand {
    /// 列出发布
    #[command(name = "+list")]
    List {
        /// 按产品 ID 筛选
        #[arg(long)]
        product: Option<u64>,
        /// 按项目 ID 筛选
        #[arg(long)]
        project: Option<u64>,
    },
    /// 获取发布详情
    #[command(name = "+get")]
    Get { id: u64 },
}

pub fn run() -> Result<()> {
    let cli = Cli::parse();

    // Extract CLI args first (before they get moved)
    let cli_url = cli.url.as_deref();
    let cli_token = cli.token.as_deref();

    // Build config from args and env
    // 先加载配置文件/环境变量，然后合并 CLI 参数（CLI 参数优先级最高）
    let file_config = load_config().unwrap_or_else(|_| Config {
        url: String::new(),
        token: None,
        product_id: None,
        project_id: None,
        api_version: None,
    });

    // CLI 参数始终覆盖文件配置
    let config = Config {
        url: cli_url
            .map(String::from)
            .unwrap_or_else(|| file_config.url.clone()),
        token: cli_token.map(String::from).or(file_config.token),
        product_id: file_config.product_id,
        project_id: file_config.project_id,
        api_version: file_config.api_version.clone(),
    };

    match cli.command {
        Commands::Story { action } => {
            story::run(&action, &config, cli.format, cli.dry_run);
        }
        Commands::Bug { action } => {
            bug::run(&action, &config, cli.format, cli.dry_run);
        }
        Commands::Auth { action } => {
            let rt = tokio::runtime::Runtime::new()
                .expect("Failed to create Tokio runtime - system may be out of memory");
            rt.block_on(auth::run(&action, cli_url, cli_token))
                .expect("Auth command failed");
        }
        Commands::Config { action } => {
            let rt = tokio::runtime::Runtime::new().expect("Failed to create Tokio runtime");
            rt.block_on(config_cmd::run(&action))
                .expect("Config command failed");
        }
        Commands::Api { action } => {
            let rt = tokio::runtime::Runtime::new()
                .expect("Failed to create Tokio runtime - system may be out of memory");
            rt.block_on(api_cmd::run(&action, &config))
                .expect("API command failed");
        }
        Commands::Product { action } => {
            product::run(&action, &config, cli.format, cli.dry_run);
        }
        Commands::Project { action } => {
            project::run(&action, &config, cli.format, cli.dry_run);
        }
        Commands::Task { action } => {
            task::run(&action, &config, cli.format, cli.dry_run);
        }
        Commands::User { action } => {
            user::run(&action, &config, cli.format, cli.dry_run);
        }
        Commands::Testcase { action } => {
            testcase::run(&action, &config, cli.format, cli.dry_run);
        }
        Commands::Release { action } => {
            release::run(&action, &config, cli.format, cli.dry_run);
        }
        Commands::Build { action } => {
            build::run(&action, &config, cli.format, cli.dry_run);
        }
        Commands::Execution { action } => {
            execution::run(&action, &config, cli.format, cli.dry_run);
        }
        Commands::Doc { action } => {
            doc::run(&action, &config, cli.format, cli.dry_run);
        }
        Commands::Doctor => {
            let rt = tokio::runtime::Runtime::new()
                .expect("Failed to create Tokio runtime - system may be out of memory");
            rt.block_on(doctor::run_doctor())
                .expect("Doctor command failed");
        }
        Commands::BugBrowse { product } => {
            let mut cfg = config.clone();
            if let Some(pid) = product {
                cfg.product_id = Some(pid);
            }
            browse::bug_browse(&cfg).expect("Bug browse failed");
        }
        Commands::StoryBrowse { product } => {
            let mut cfg = config.clone();
            if let Some(pid) = product {
                cfg.product_id = Some(pid);
            }
            browse::story_browse(&cfg).expect("Story browse failed");
        }
    }

    Ok(())
}
