//! ZenTao 用户(User)命令模块
//!
//! CLI 命令入口，调用 UserApi 处理用户请求

use clap::Subcommand;
use crate::api::{ApiClient, UserApi};
use crate::core::{Config, OutputFormat};

#[derive(Subcommand, Clone, Debug)]
pub enum UserAction {
    /// 列出用户
    #[command(name = "+list")]
    List {
        #[arg(long)]
        dept: Option<u64>,
        #[arg(long)]
        role: Option<String>,
    },
    /// 获取用户详情
    #[command(name = "+get")]
    Get {
        id: u64,
    },
}

/// 执行 User 相关命令
///
/// 根据子命令类型调用对应的 API 并输出结果
pub fn run(cmd: &UserAction, config: &Config, _format: OutputFormat) {
    let rt = tokio::runtime::Runtime::new()
        .expect("Failed to create Tokio runtime");

    rt.block_on(async {
        let client = ApiClient::new(&config.url, config.token.clone())
            .with_api_version(config.api_version.as_deref().unwrap_or("v1"));

        match cmd {
            UserAction::List { dept, role } => {
                match UserApi::list(&client, *dept, role.clone()).await {
                    Ok(users) => {
                        println!("{}", serde_json::to_string_pretty(&users).unwrap_or_default());
                    }
                    Err(e) => {
                        eprintln!("Error: {}", e);
                    }
                }
            }
            UserAction::Get { id } => {
                match UserApi::get(&client, *id).await {
                    Ok(user) => {
                        println!("{}", serde_json::to_string_pretty(&user).unwrap_or_default());
                    }
                    Err(e) => {
                        eprintln!("Error: {}", e);
                    }
                }
            }
        }
    });
}
