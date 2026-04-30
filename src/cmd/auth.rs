//! ZenTao 认证命令模块
//!
//! 提供登录、登出、状态查看功能
//! 类似 Java 的 @Service 层，封装认证业务逻辑

use crate::api::Auth;
use crate::core::{load_config, update_config}; // 导入配置模块
use anyhow::Result; // anyhow: 错误处理库，类似 Go 的 error 但更灵活
use clap::Subcommand; // clap: 命令行参数解析库，类似 Java 的 JCommander 或 Python 的 argparse // 导入 API 认证客户端

// ============================================================
// 命令定义 - 类似 Java 的枚举类或 TypeScript 的 union type
// ============================================================

#[derive(Subcommand, Clone, Debug)]
pub enum AuthSubcommand {
    /// 登录命令
    /// - account: 禅道账号 (对应环境变量 ZENTAO_ACCOUNT)
    /// - password: 禅道密码 (对应环境变量 ZENTAO_PASSWORD)
    #[command(name = "login")]
    Login {
        #[arg(long, env = "ZENTAO_ACCOUNT")]
        account: String,

        #[arg(long, env = "ZENTAO_PASSWORD")]
        password: String,
    },

    /// 登出命令 - 清除保存的 token
    #[command(name = "logout")]
    Logout,

    /// 查看认证状态 - 验证 token 是否有效
    #[command(name = "status")]
    Status,
}

// ============================================================
// 异步命令执行入口 - 类似 Spring Boot 的 @Service 方法
// ============================================================

/// 执行认证命令
///
/// # 参数
/// * `auth_cmd` - 命令类型 (Login/Logout/Status)
/// * `cli_url` - CLI 传入的 URL 参数 (优先级最高)
/// * `cli_token` - CLI 传入的 token 参数 (优先级最高)
///
/// # 返回
/// * `Result<()>` - 类似 Go 的 error，anyhow::Result 是更友好的封装
pub async fn run(
    auth_cmd: &AuthSubcommand,
    cli_url: Option<&str>,
    _cli_token: Option<&str>,
) -> Result<()> {
    match auth_cmd {
        // -------------------- 登录 --------------------
        AuthSubcommand::Login { account, password } => {
            // 1. 优先使用 CLI 参数，否则从配置文件读取
            let config = load_config()?;
            let url = cli_url
                .map(String::from)
                .unwrap_or_else(|| config.url.clone());

            // 2. 如果 URL 为空，启动交互式配置向导
            if url.is_empty() {
                println!("ZenTao URL not configured. Starting interactive setup...");
                // 调用交互式配置向导
                crate::cmd::config_cmd::run(&crate::cmd::config_cmd::ConfigSubcommand::Init {
                    global: false,
                })
                .await?;
                // 向导结束后重新加载配置
                let config = load_config()?;
                if config.url.is_empty() {
                    anyhow::bail!("Setup cancelled or failed. Please configure URL manually.");
                }
                // 使用新的 URL 登录
                let url = config.url;
                println!("Logging in to {}", url);
                let auth = Auth::new(&url);
                match auth.login(account, password).await {
                    Ok(token) => {
                        println!("✓ Login successful");
                        update_config("token", &token)?;
                        println!("✓ Token saved to config");
                        println!("  Token: {}...", &token[..token.len().min(8)]);
                        match auth.verify_token(&token).await {
                            Ok(true) => println!("✓ Token verified"),
                            Ok(false) => println!("⚠ Token verification failed"),
                            Err(e) => println!("⚠ Token verification error: {}", e),
                        }
                        Ok(())
                    }
                    Err(e) => {
                        println!("✗ Login failed: {}", e);
                        Err(e)
                    }
                }
            } else {
                // URL 已配置，直接登录
                println!("Logging in to {}", url);
                let auth = Auth::new(&url);
                match auth.login(account, password).await {
                    Ok(token) => {
                        println!("✓ Login successful");
                        update_config("token", &token)?;
                        println!("✓ Token saved to config");
                        println!("  Token: {}...", &token[..token.len().min(8)]);
                        match auth.verify_token(&token).await {
                            Ok(true) => println!("✓ Token verified"),
                            Ok(false) => println!("⚠ Token verification failed"),
                            Err(e) => println!("⚠ Token verification error: {}", e),
                        }
                        Ok(())
                    }
                    Err(e) => {
                        println!("✗ Login failed: {}", e);
                        Err(e)
                    }
                }
            }
        }

        // -------------------- 登出 --------------------
        AuthSubcommand::Logout => {
            let config = load_config()?;

            // 清空 token
            if config.token.is_some() {
                // 使用空字符串清空 token，update_config 会处理 None 的情况
                update_config("token", "")?;
                println!("✓ Logged out (token cleared)");
            } else {
                println!("Not logged in");
            }
            Ok(())
        }

        // -------------------- 状态 --------------------
        AuthSubcommand::Status => {
            let config = load_config()?;

            // 检查是否有 token
            if let Some(token) = &config.token {
                if !token.is_empty() {
                    println!("✓ Authenticated");
                    println!("  URL: {}", config.url);
                    println!("  Token: {}...", &token[..token.len().min(8)]);

                    // 验证 token
                    let auth = Auth::new(&config.url);
                    match auth.verify_token(token).await {
                        Ok(true) => println!("  Status: Valid"),
                        Ok(false) => println!("  Status: Invalid"),
                        Err(e) => println!("  Status: Could not verify ({})", e),
                    }
                } else {
                    println!("✗ Not authenticated (empty token)");
                }
            } else {
                println!("✗ Not authenticated (no token)");
                println!("  Set ZENTAO_TOKEN or run 'zentao auth login'");
            }
            Ok(())
        }
    }
}
