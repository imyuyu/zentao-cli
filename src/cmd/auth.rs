//! ZenTao 认证命令模块
//!
//! 提供登录、登出、状态查看功能
//! 类似 Java 的 @Service 层，封装认证业务逻辑

use crate::api::Auth;
use crate::cmd::common::log_command;
use crate::core::{Credentials, load_config, update_config};
use crate::safe_println;
use anyhow::Result; // anyhow: 错误处理库，类似 Go 的 error 但更灵活
use clap::Subcommand; // clap: 命令行参数解析库，类似 Java 的 JCommander 或 Python 的 argparse // 导入 API 认证客户端
use std::io::Write; // 导入 Write trait 以使用 flush()

// ============================================================
// 命令定义 - 类似 Java 的枚举类或 TypeScript 的 union type
// ============================================================

#[derive(Subcommand, Clone, Debug)]
pub enum AuthSubcommand {
    /// 登录命令（不带参数时启动交互式 TUI 登录）
    #[command(name = "login")]
    Login {
        /// 禅道账号（可选，未提供时启动 TUI）
        #[arg(long, env = "ZENTAO_ACCOUNT")]
        account: Option<String>,

        /// 禅道密码（可选，未提供时启动 TUI）
        #[arg(long, env = "ZENTAO_PASSWORD")]
        password: Option<String>,

        /// 保存到全局配置（不加参数默认保存到项目配置）
        #[arg(long, short = 'g')]
        global: bool,
    },

    /// 登出命令 - 清除保存的 token
    #[command(name = "logout")]
    Logout,

    /// 查看认证状态 - 验证 token 是否有效
    #[command(name = "status")]
    Status,

    /// 查看当前登录的用户名
    #[command(name = "whoami")]
    Whoami,

    /// 刷新 token - 从凭据库读取密码并重新登录
    #[command(name = "refresh")]
    Refresh,
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
    log_command("auth", format!("{:?}", auth_cmd));
    match auth_cmd {
        // -------------------- 登录 --------------------
        AuthSubcommand::Login {
            account,
            password,
            global,
        } => {
            // 获取配置
            let config = load_config()?;
            let url = cli_url
                .map(String::from)
                .unwrap_or_else(|| config.url.clone());

            // 如果 account 或 password 未提供，使用终端输入
            let account = if let Some(a) = account {
                a.clone()
            } else {
                print!("请输入账号: ");
                std::io::stdout().flush().ok();
                let mut input = String::new();
                std::io::stdin().read_line(&mut input).ok();
                input.trim().to_string()
            };

            let password = if let Some(p) = password {
                p.clone()
            } else {
                print!("请输入密码: ");
                std::io::stdout().flush().ok();
                rpassword::read_password().unwrap_or_default()
            };

            // URL 为空，启动完整配置向导
            if url.is_empty() {
                safe_println("ZenTao URL not configured. Starting interactive setup...");
                crate::cmd::config_cmd::run(&crate::cmd::config_cmd::ConfigSubcommand::Init {
                    global: false,
                })
                .await?;
                let config = load_config()?;
                if config.url.is_empty() {
                    anyhow::bail!("Setup cancelled or failed. Please configure URL manually.");
                }
                let url = config.url;
                println!("Logging in to {}", url);
                let auth = Auth::new(&url);
                match auth.login(&account, &password).await {
                    Ok(token) => {
                        safe_println("✓ Login successful");
                        // 保存 account 到 config
                        update_config("account", &account, *global)?;
                        // 保存 password 到系统凭据库
                        if let Err(e) = Credentials::store(&url, &account, &password) {
                            println!("⚠ Failed to store credentials: {}", e);
                        }
                        update_config("token", &token, *global)?;
                        safe_println("✓ Token saved to config");
                        println!("  Token: {}...", &token[..token.len().min(8)]);
                        match auth.verify_token(&token).await {
                            Ok(true) => safe_println("✓ Token verified"),
                            Ok(false) => safe_println("⚠ Token verification failed"),
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
                match auth.login(&account, &password).await {
                    Ok(token) => {
                        safe_println("✓ Login successful");
                        // 保存 account 到 config
                        update_config("account", &account, *global)?;
                        // 保存 password 到系统凭据库
                        if let Err(e) = Credentials::store(&url, &account, &password) {
                            println!("⚠ Failed to store credentials: {}", e);
                        }
                        update_config("token", &token, *global)?;
                        safe_println("✓ Token saved to config");
                        println!("  Token: {}...", &token[..token.len().min(8)]);
                        match auth.verify_token(&token).await {
                            Ok(true) => safe_println("✓ Token verified"),
                            Ok(false) => safe_println("⚠ Token verification failed"),
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
                update_config("token", "", false)?;
                safe_println("✓ Token cleared");
            }
            // 删除凭据
            if let Some(account) = &config.account {
                if let Err(e) = Credentials::delete(&config.url, account) {
                    println!("⚠ Failed to delete credentials: {}", e);
                } else {
                    safe_println("✓ Credentials deleted");
                }
                update_config("account", "", false)?;
                safe_println("✓ Account cleared");
            }
            safe_println("Logged out successfully");
            Ok(())
        }

        // -------------------- 状态 --------------------
        AuthSubcommand::Status => {
            let config = load_config()?;

            // 检查是否有 token
            if let Some(token) = &config.token {
                if !token.is_empty() {
                    safe_println("✓ Authenticated");
                    println!("  URL: {}", config.url);
                    if let Some(account) = &config.account {
                        let masked = if account.len() <= 2 {
                            "**".to_string()
                        } else {
                            format!("{}**{}", &account[..1], &account[account.len()-1..])
                        };
                        println!("  Account: {}", masked);
                    }
                    println!("  Token: {}...", &token[..token.len().min(8)]);

                    // 验证 token
                    let auth = Auth::new(&config.url);
                    match auth.verify_token(token).await {
                        Ok(true) => safe_println("  Status: Valid"),
                        Ok(false) => safe_println("  Status: Invalid"),
                        Err(e) => println!("  Status: Could not verify ({})", e),
                    }
                } else {
                    safe_println("✗ Not authenticated (empty token)");
                }
            } else {
                safe_println("✗ Not authenticated (no token)");
                safe_println("  Set ZENTAO_TOKEN or run 'zentao auth login'");
            }
            Ok(())
        }

        // -------------------- whoami --------------------
        AuthSubcommand::Whoami => {
            let config = load_config()?;
            if let Some(account) = &config.account {
                println!("{}", account);
            } else {
                safe_println("Not logged in");
                safe_println("  Run 'zentao auth login' to login");
            }
            Ok(())
        }

        // -------------------- refresh --------------------
        AuthSubcommand::Refresh => {
            let config = load_config()?;

            // 检查是否有 url 和 account
            if config.url.is_empty() {
                anyhow::bail!("URL not configured. Run 'zentao auth login' first.");
            }
            let account = config.account.as_ref()
                .ok_or_else(|| anyhow::anyhow!("Account not configured. Run 'zentao auth login' first."))?;

            // 从 keyring 获取凭据
            let creds = Credentials::get(&config.url, account)
                .map_err(|e| anyhow::anyhow!("Failed to get credentials: {}", e))?
                .ok_or_else(|| anyhow::anyhow!("No credentials found. Run 'zentao auth login' first."))?;

            let password = creds.password
                .ok_or_else(|| anyhow::anyhow!("Password not found in credentials"))?;

            // 重新登录获取新 token
            println!("Refreshing token for {}", config.url);
            let auth = Auth::new(&config.url);
            match auth.login(account, &password).await {
                Ok(token) => {
                    safe_println("✓ Login successful");
                    update_config("token", &token, false)?;
                    safe_println("✓ Token saved to config");
                    println!("  Token: {}...", &token[..token.len().min(8)]);
                    match auth.verify_token(&token).await {
                        Ok(true) => safe_println("✓ Token verified"),
                        Ok(false) => safe_println("⚠ Token verification failed"),
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
}
