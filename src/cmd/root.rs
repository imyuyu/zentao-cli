use crate::cmd::api_cmd::ApiSubcommand;
use crate::cmd::auth::AuthSubcommand;
use crate::cmd::common::log_command;
use crate::cmd::config_cmd::ConfigSubcommand;
use crate::cmd::{
    api_cmd, auth, browse, bug, build, common, config_cmd, department, doctor, execution,
    feedback, product, productplan, program, project, release, story, task, testcase,
    testtask_cmd, ticket, user,
};
use crate::core::logging;
use crate::core::{load_config, AppContext, Config, OutputFormat};
use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "zentao-cli")]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(about = "ZenTao CLI - Command line tool for ZenTao project management")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    #[arg(long, global = true, env = "ZENTAO_URL")]
    url: Option<String>,

    #[arg(long, global = true, env = "ZENTAO_TOKEN")]
    token: Option<String>,

    #[arg(long, global = true, value_enum, default_value = "table")]
    format: OutputFormat,

    /// 预执行模式：只显示将要执行的操作，不实际调用 API
    #[arg(long, global = true)]
    dry_run: bool,

    /// 输出内部调试日志到 stderr
    #[arg(long, global = true)]
    debug: bool,

    /// 设置日志级别并写入系统日志文件
    #[arg(long, global = true, value_enum)]
    log_level: Option<logging::LogLevel>,
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
    /// 部门管理 - 查看部门列表
    Department {
        #[command(subcommand)]
        action: DepartmentSubcommand,
    },
    /// 项目集管理 - 查看项目集列表
    Program {
        #[command(subcommand)]
        action: ProgramSubcommand,
    },
    /// 产品计划管理 - 查看产品计划
    ProductPlan {
        #[command(subcommand)]
        action: ProductPlanSubcommand,
    },
    /// 测试单管理 - 查看测试单
    Testtask {
        #[command(subcommand)]
        action: TesttaskSubcommand,
    },
    /// 反馈管理 - 查看反馈
    Feedback {
        #[command(subcommand)]
        action: FeedbackSubcommand,
    },
    /// 工单管理 - 查看工单
    Ticket {
        #[command(subcommand)]
        action: TicketSubcommand,
    },
    /// 诊断工具 - 检查配置和网络连接
    Doctor,
    /// 浏览器 - TUI 模式浏览所有模块（默认）
    Browse,
    /// Bug 浏览器 - TUI 模式浏览 Bug 列表
    BugBrowse {
        #[arg(long)]
        product: Option<u64>,
        #[arg(long)]
        account: Option<String>,
    },
    /// 故事浏览器 - TUI 模式浏览故事列表
    StoryBrowse {
        #[arg(long)]
        product: Option<u64>,
        #[arg(long)]
        account: Option<String>,
    },
    /// 查看当前登录的用户名
    Whoami,
}

#[derive(Subcommand, Clone, Debug)]
pub enum StorySubcommand {
    /// 列出需求列表（API Commands）
    #[command(name = "list")]
    List {
        #[arg(long)]
        product: Option<u64>,
        #[arg(long)]
        project: Option<u64>,
        #[arg(long)]
        status: Option<String>,
    },
    /// 获取需求详情（API Commands）
    #[command(name = "get")]
    Get { id: u64 },
    /// 创建需求（API Commands）
    #[command(name = "create")]
    Create {
        #[arg(long)]
        title: String,
        #[arg(long)]
        product: Option<u64>,
        #[arg(long)]
        pri: u8,
        #[arg(long)]
        category: Option<String>,
        #[arg(long)]
        spec: Option<String>,
        #[arg(long)]
        estimate: Option<f64>,
    },
    /// 更新需求（API Commands）
    #[command(name = "update")]
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
    /// 变更需求（API Commands）
    #[command(name = "change")]
    Change {
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
    /// 删除需求（API Commands）
    #[command(name = "delete")]
    Delete { id: u64 },
    /// 关闭需求（API Commands）
    #[command(name = "close")]
    Close { id: u64 },
}

#[derive(Subcommand, Clone, Debug)]
pub enum BugSubcommand {
    /// 列出 Bug 列表（API Commands）
    #[command(name = "list")]
    List {
        #[arg(long)]
        product: Option<u64>,
        #[arg(long)]
        status: Option<String>,
        #[arg(long)]
        assigned_to: Option<String>,
    },
    /// 获取 Bug 详情（API Commands）
    #[command(name = "get")]
    Get { id: u64 },
    /// 创建 Bug（API Commands）
    #[command(name = "create")]
    Create {
        #[arg(long)]
        title: String,
        #[arg(long)]
        product: Option<u64>,
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
    /// 更新 Bug（API Commands）
    #[command(name = "update")]
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
    /// 解决 Bug（API Commands）
    #[command(name = "resolve")]
    Resolve {
        id: u64,
        #[arg(long)]
        resolution: String,
        #[arg(long)]
        resolved_build: String,
    },
    /// 确认 Bug（API Commands）
    #[command(name = "confirm")]
    Confirm { id: u64 },
    /// 关闭 Bug（API Commands）
    #[command(name = "close")]
    Close { id: u64 },
    /// 激活 Bug（API Commands）
    #[command(name = "activate")]
    Activate { id: u64 },
    /// 删除 Bug（API Commands）
    #[command(name = "delete")]
    Delete { id: u64 },
}

#[derive(Subcommand, Clone, Debug)]
pub enum TestcaseSubcommand {
    /// 列出测试用例
    #[command(name = "list")]
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
    #[command(name = "get")]
    Get { id: u64 },
    /// 创建测试用例
    #[command(name = "create")]
    Create {
        /// 产品 ID（必填）
        #[arg(long)]
        product: Option<u64>,
        /// 用例标题（必填）
        #[arg(long)]
        title: String,
        /// 用例类型：feature/performance/interface/security/concurrency/destructive/install/others
        #[arg(long)]
        type_: Option<String>,
        /// 严重程度：1-4（1 最严重）
        #[arg(long)]
        severity: Option<u8>,
        /// 优先级：0-5
        #[arg(long)]
        pri: Option<u8>,
        /// 测试步骤
        #[arg(long)]
        steps: Option<String>,
        /// 期望结果
        #[arg(long)]
        expectation: Option<String>,
        /// 关联的需求 ID
        #[arg(long)]
        story: Option<u64>,
        /// 所属项目 ID
        #[arg(long)]
        project: Option<u64>,
    },
    /// 更新测试用例
    #[command(name = "update")]
    Update {
        /// 用例 ID（必填）
        id: u64,
        /// 新标题
        #[arg(long)]
        title: Option<String>,
        /// 新状态：wait/normal/blocked/bypass
        #[arg(long)]
        status: Option<String>,
        /// 新优先级
        #[arg(long)]
        pri: Option<u8>,
        /// 新严重程度
        #[arg(long)]
        severity: Option<u8>,
        /// 新用例类型
        #[arg(long)]
        type_: Option<String>,
        /// 新测试步骤
        #[arg(long)]
        steps: Option<String>,
        /// 新期望结果
        #[arg(long)]
        expectation: Option<String>,
    },
    /// 删除测试用例
    #[command(name = "delete")]
    Delete {
        /// 用例 ID（必填）
        id: u64,
    },
    /// 执行测试用例
    #[command(name = "result")]
    Result {
        /// 用例 ID（必填）
        id: u64,
        /// 执行结果（必填）：pass/fail/blocked
        #[arg(long)]
        result: String,
        /// 执行耗时（分钟）
        #[arg(long)]
        consumed: Option<f64>,
        /// 执行备注
        #[arg(long)]
        remark: Option<String>,
        /// 关联的版本 ID
        #[arg(long)]
        build: Option<u64>,
    },
}

#[derive(Subcommand, Clone, Debug)]
pub enum ReleaseSubcommand {
    /// 列出发布
    #[command(name = "list")]
    List {
        /// 按产品 ID 筛选
        #[arg(long)]
        product: Option<u64>,
        /// 按项目 ID 筛选
        #[arg(long)]
        project: Option<u64>,
    },
    /// 获取发布详情
    #[command(name = "get")]
    Get { id: u64 },
}

#[derive(Subcommand, Clone, Debug)]
pub enum DepartmentSubcommand {
    /// 列出部门
    #[command(name = "list")]
    List,
    /// 获取部门详情
    #[command(name = "get")]
    Get { id: u64 },
}

#[derive(Subcommand, Clone, Debug)]
pub enum ProgramSubcommand {
    /// 列出项目集
    #[command(name = "list")]
    List,
    /// 获取项目集详情
    #[command(name = "get")]
    Get { id: u64 },
}

#[derive(Subcommand, Clone, Debug)]
pub enum ProductPlanSubcommand {
    /// 列出产品计划
    #[command(name = "list")]
    List {
        /// 按产品 ID 筛选
        #[arg(long)]
        product: Option<u64>,
    },
    /// 获取产品计划详情
    #[command(name = "get")]
    Get { id: u64 },
}

#[derive(Subcommand, Clone, Debug)]
pub enum TesttaskSubcommand {
    /// 列出测试单
    #[command(name = "list")]
    List {
        /// 按项目 ID 筛选
        #[arg(long)]
        project: Option<u64>,
    },
    /// 获取测试单详情
    #[command(name = "get")]
    Get { id: u64 },
}

#[derive(Subcommand, Clone, Debug)]
pub enum FeedbackSubcommand {
    /// 列出反馈
    #[command(name = "list")]
    List,
    /// 获取反馈详情
    #[command(name = "get")]
    Get { id: u64 },
}

#[derive(Subcommand, Clone, Debug)]
pub enum TicketSubcommand {
    /// 列出工单
    #[command(name = "list")]
    List,
    /// 获取工单详情
    #[command(name = "get")]
    Get { id: u64 },
}

pub fn run() -> Result<()> {
    let cli = Cli::parse();
    logging::init(cli.debug, cli.log_level);
    if (cli.debug || cli.log_level.is_some()) && logging::current_log_path().is_some() {
        eprintln!(
            "zentao-cli logging to {}",
            logging::current_log_path().unwrap().display()
        );
    }

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
        account: None,
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
        account: file_config.account.clone(),
    };
    let ctx = AppContext::new(config.clone(), cli.format.clone(), cli.dry_run);
    let rt = tokio::runtime::Runtime::new()
        .expect("Failed to create Tokio runtime - system may be out of memory");

    // If no command provided, run TUI browse mode
    let command = cli.command.unwrap_or(Commands::Browse);

    match command {
        Commands::Story { action } => {
            log_command("root", format!("dispatch story {:?}", action));
            rt.block_on(story::run(&action, &ctx));
        }
        Commands::Bug { action } => {
            log_command("root", format!("dispatch bug {:?}", action));
            rt.block_on(bug::run(&action, &ctx));
        }
        Commands::Auth { action } => {
            log_command("root", format!("dispatch auth {:?}", action));
            rt.block_on(auth::run(&action, cli_url, cli_token))?;
        }
        Commands::Config { action } => {
            log_command("root", format!("dispatch config {:?}", action));
            rt.block_on(config_cmd::run(&action))?;
        }
        Commands::Api { action } => {
            log_command("root", format!("dispatch api {:?}", action));
            rt.block_on(api_cmd::run(&action, &config))?;
        }
        Commands::Product { action } => {
            log_command("root", format!("dispatch product {:?}", action));
            rt.block_on(product::run(&action, &ctx));
        }
        Commands::Project { action } => {
            log_command("root", format!("dispatch project {:?}", action));
            rt.block_on(project::run(&action, &ctx));
        }
        Commands::Task { action } => {
            log_command("root", format!("dispatch task {:?}", action));
            rt.block_on(task::run(&action, &ctx));
        }
        Commands::User { action } => {
            log_command("root", format!("dispatch user {:?}", action));
            rt.block_on(user::run(&action, &ctx));
        }
        Commands::Testcase { action } => {
            log_command("root", format!("dispatch testcase {:?}", action));
            rt.block_on(testcase::run(&action, &ctx));
        }
        Commands::Release { action } => {
            log_command("root", format!("dispatch release {:?}", action));
            rt.block_on(release::run(&action, &ctx));
        }
        Commands::Build { action } => {
            log_command("root", format!("dispatch build {:?}", action));
            rt.block_on(build::run(&action, &ctx));
        }
        Commands::Execution { action } => {
            log_command("root", format!("dispatch execution {:?}", action));
            rt.block_on(execution::run(&action, &ctx));
        }
        Commands::Department { action } => {
            log_command("root", format!("dispatch department {:?}", action));
            rt.block_on(department::run(&action, &ctx));
        }
        Commands::Program { action } => {
            log_command("root", format!("dispatch program {:?}", action));
            rt.block_on(program::run(&action, &ctx));
        }
        Commands::ProductPlan { action } => {
            log_command("root", format!("dispatch productplan {:?}", action));
            rt.block_on(productplan::run(&action, &ctx));
        }
        Commands::Testtask { action } => {
            log_command("root", format!("dispatch testtask {:?}", action));
            rt.block_on(testtask_cmd::run(&action, &ctx));
        }
        Commands::Feedback { action } => {
            log_command("root", format!("dispatch feedback {:?}", action));
            rt.block_on(feedback::run(&action, &ctx));
        }
        Commands::Ticket { action } => {
            log_command("root", format!("dispatch ticket {:?}", action));
            rt.block_on(ticket::run(&action, &ctx));
        }
        Commands::Doctor => {
            log_command("root", "dispatch doctor");
            rt.block_on(doctor::run_doctor())?;
        }
        Commands::Browse => {
            log_command("root", "dispatch browse (home)");
            browse::run_tui(&config).expect("Browse failed");
        }
        Commands::BugBrowse { product, account } => {
            let mut cfg = config.clone();
            if let Some(pid) = product {
                cfg.product_id = Some(pid);
            }
            // If account is specified, use that account's config
            if let Some(acc) = account {
                if let Ok(multi) = crate::core::load_multi_account_config() {
                    if let Some(acc_config) = multi.accounts.get(&acc) {
                        cfg = acc_config.clone();
                        if let Some(pid) = product {
                            cfg.product_id = Some(pid);
                        }
                    }
                }
            }
            browse::bug_browse(&cfg).expect("Bug browse failed");
        }
        Commands::StoryBrowse { product, account } => {
            let mut cfg = config.clone();
            if let Some(pid) = product {
                cfg.product_id = Some(pid);
            }
            // If account is specified, use that account's config
            if let Some(acc) = account {
                if let Ok(multi) = crate::core::load_multi_account_config() {
                    if let Some(acc_config) = multi.accounts.get(&acc) {
                        cfg = acc_config.clone();
                        if let Some(pid) = product {
                            cfg.product_id = Some(pid);
                        }
                    }
                }
            }
            browse::story_browse(&cfg).expect("Story browse failed");
        }
        Commands::Whoami => {
            if let Some(account) = &config.account {
                println!("{}", account);
            } else {
                println!("Not logged in");
                println!("  Run 'zentao auth login' to login");
            }
        }
    }

    Ok(())
}
