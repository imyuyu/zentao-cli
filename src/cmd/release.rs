//! ZenTao Release(发布)命令模块
//!
//! CLI 命令入口，调用 ReleaseApi 处理用户请求

use crate::api::{ApiClient, ReleaseApi};
use crate::core::{Config, OutputFormat};
use crate::cmd::root::ReleaseSubcommand;

/// 执行 Release 相关命令
///
/// 根据子命令类型调用对应的 API 并输出结果
pub fn run(cmd: &ReleaseSubcommand, config: &Config, _format: OutputFormat) {
    let rt = tokio::runtime::Runtime::new()
        .expect("Failed to create Tokio runtime");

    rt.block_on(async {
        let client = ApiClient::new(&config.url, config.token.clone())
            .with_api_version(config.api_version.as_deref().unwrap_or("v1"));

        match cmd {
            // -------------------- list --------------------
            ReleaseSubcommand::List => {
                match ReleaseApi::list(&client).await {
                    Ok(releases) => {
                        println!("{}", serde_json::to_string_pretty(&releases).unwrap_or_default());
                    }
                    Err(e) => {
                        eprintln!("Error: {}", e);
                    }
                }
            }

            // -------------------- get --------------------
            ReleaseSubcommand::Get { id } => {
                match ReleaseApi::get(&client, *id).await {
                    Ok(release) => {
                        println!("{}", serde_json::to_string_pretty(&release).unwrap_or_default());
                    }
                    Err(e) => {
                        eprintln!("Error: {}", e);
                    }
                }
            }
        }
    });
}
