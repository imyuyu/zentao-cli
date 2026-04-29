//! ZenTao Testcase(测试用例)命令模块
//!
//! CLI 命令入口，调用 TestcaseApi 处理用户请求

use crate::api::{ApiClient, TestcaseApi};
use crate::cmd::root::TestcaseSubcommand;
use crate::core::{Config, OutputFormat};

/// 执行测试用例相关命令
///
/// 根据子命令类型调用对应的 API 并输出结果
pub fn run(cmd: &TestcaseSubcommand, config: &Config, _format: OutputFormat, dry_run: bool) {
    let rt = tokio::runtime::Runtime::new().expect("Failed to create Tokio runtime");

    rt.block_on(async {
        let client = ApiClient::new(&config.url, config.token.clone())
            .with_api_version(config.api_version.as_deref().unwrap_or("v1"));

        match cmd {
            // -------------------- list --------------------
            TestcaseSubcommand::List {
                product,
                project,
                type_,
                status,
            } => {
                if dry_run {
                    println!("[DRY-RUN] Would call TestcaseApi::list()");
                    println!("  URL: {}/api.php/v1/testcases", config.url);
                    println!("  Params:");
                    if let Some(p) = product {
                        println!("    product: {}", p);
                    }
                    if let Some(p) = project {
                        println!("    project: {}", p);
                    }
                    if let Some(t) = type_ {
                        println!("    type: {}", t);
                    }
                    if let Some(s) = status {
                        println!("    status: {}", s);
                    }
                    return;
                }
                match TestcaseApi::list(&client, *product, *project, type_.clone(), status.clone())
                    .await
                {
                    Ok(testcases) => {
                        println!(
                            "{}",
                            serde_json::to_string_pretty(&testcases).unwrap_or_default()
                        );
                    }
                    Err(e) => {
                        eprintln!("Error: {}", e);
                    }
                }
            }

            // -------------------- get --------------------
            TestcaseSubcommand::Get { id } => {
                if dry_run {
                    println!("[DRY-RUN] Would call TestcaseApi::get()");
                    println!("  URL: {}/api.php/v1/testcases/{}", config.url, id);
                    return;
                }
                match TestcaseApi::get(&client, *id).await {
                    Ok(testcase) => {
                        println!(
                            "{}",
                            serde_json::to_string_pretty(&testcase).unwrap_or_default()
                        );
                    }
                    Err(e) => {
                        eprintln!("Error: {}", e);
                    }
                }
            }
        }
    });
}
