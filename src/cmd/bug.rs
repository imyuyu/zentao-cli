//! ZenTao Bug(缺陷)命令模块
//!
//! CLI 命令入口，调用 BugApi 处理用户请求

use crate::api::{ApiClient, BugApi, CreateBugRequest, UpdateBugRequest};
use crate::cmd::root::BugSubcommand;
use crate::core::{Config, OutputFormat};

/// 执行 Bug 相关命令
///
/// 根据子命令类型调用对应的 API 并输出结果
pub fn run(cmd: &BugSubcommand, config: &Config, _format: OutputFormat) {
    let rt = tokio::runtime::Runtime::new().expect("Failed to create Tokio runtime");

    rt.block_on(async {
        let client = ApiClient::new(&config.url, config.token.clone())
            .with_api_version(config.api_version.as_deref().unwrap_or("v1"));

        match cmd {
            // -------------------- list --------------------
            BugSubcommand::List {
                product,
                status,
                assigned_to,
            } => match BugApi::list(&client, *product, status.clone(), assigned_to.clone()).await {
                Ok(bugs) => {
                    println!(
                        "{}",
                        serde_json::to_string_pretty(&bugs).unwrap_or_default()
                    );
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                }
            },

            // -------------------- get --------------------
            BugSubcommand::Get { id } => match BugApi::get(&client, *id).await {
                Ok(bug) => {
                    println!("{}", serde_json::to_string_pretty(&bug).unwrap_or_default());
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                }
            },

            // -------------------- create --------------------
            BugSubcommand::Create {
                title,
                product,
                severity,
                pri,
                type_,
                steps,
                story,
            } => {
                let req = CreateBugRequest {
                    title: title.clone(),
                    product: *product,
                    severity: *severity,
                    pri: *pri,
                    type_: type_.clone(),
                    steps: steps.clone(),
                    story: *story,
                    assigned_to: None,
                };

                match BugApi::create(&client, &req).await {
                    Ok(bug) => {
                        println!("{}", serde_json::to_string_pretty(&bug).unwrap_or_default());
                    }
                    Err(e) => {
                        eprintln!("Error: {}", e);
                    }
                }
            }

            // -------------------- update --------------------
            BugSubcommand::Update {
                id,
                title,
                status,
                resolution,
                assigned_to,
            } => {
                let req = UpdateBugRequest {
                    title: title.clone(),
                    status: status.clone(),
                    resolution: resolution.clone(),
                    assigned_to: assigned_to.clone(),
                };

                match BugApi::update(&client, *id, &req).await {
                    Ok(bug) => {
                        println!("{}", serde_json::to_string_pretty(&bug).unwrap_or_default());
                    }
                    Err(e) => {
                        eprintln!("Error: {}", e);
                    }
                }
            }
        }
    });
}
