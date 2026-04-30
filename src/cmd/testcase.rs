//! ZenTao Testcase(测试用例)命令模块
//!
//! CLI 命令入口，调用 TestcaseApi 处理用户请求

use crate::api::testcase::{
    CreateTestcaseRequest, TestcaseApi, TestcaseResultRequest, UpdateTestcaseRequest,
};
use crate::api::ApiClient;
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

            // -------------------- create --------------------
            TestcaseSubcommand::Create {
                product,
                title,
                type_,
                severity,
                pri,
                steps,
                expectation,
                story,
                project,
            } => {
                if dry_run {
                    println!("[DRY-RUN] Would call TestcaseApi::create()");
                    println!("  URL: {}/api.php/v1/products/{}/testcases", config.url, product);
                    println!("  Body:");
                    println!("    title: {}", title);
                    println!("    product: {}", product);
                    if let Some(t) = type_ {
                        println!("    type: {}", t);
                    }
                    if let Some(s) = severity {
                        println!("    severity: {}", s);
                    }
                    if let Some(p) = pri {
                        println!("    pri: {}", p);
                    }
                    if let Some(s) = steps {
                        println!("    steps: {}", s);
                    }
                    if let Some(e) = expectation {
                        println!("    expectation: {}", e);
                    }
                    if let Some(s) = story {
                        println!("    story: {}", s);
                    }
                    if let Some(p) = project {
                        println!("    project: {}", p);
                    }
                    return;
                }
                let req = CreateTestcaseRequest {
                    title: title.clone(),
                    product: *product,
                    type_: type_.clone(),
                    severity: *severity,
                    pri: *pri,
                    steps: steps.clone(),
                    expectation: expectation.clone(),
                    story: *story,
                    project: *project,
                };
                match TestcaseApi::create(&client, *product, &req).await {
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

            // -------------------- update --------------------
            TestcaseSubcommand::Update {
                id,
                title,
                status,
                pri,
                severity,
                type_,
                steps,
                expectation,
            } => {
                if dry_run {
                    println!("[DRY-RUN] Would call TestcaseApi::update()");
                    println!("  URL: {}/api.php/v1/testcases/{}", config.url, id);
                    println!("  Body:");
                    if let Some(t) = title {
                        println!("    title: {}", t);
                    }
                    if let Some(s) = status {
                        println!("    status: {}", s);
                    }
                    if let Some(p) = pri {
                        println!("    pri: {}", p);
                    }
                    if let Some(s) = severity {
                        println!("    severity: {}", s);
                    }
                    if let Some(t) = type_ {
                        println!("    type: {}", t);
                    }
                    if let Some(s) = steps {
                        println!("    steps: {}", s);
                    }
                    if let Some(e) = expectation {
                        println!("    expectation: {}", e);
                    }
                    return;
                }
                let req = UpdateTestcaseRequest {
                    title: title.clone(),
                    status: status.clone(),
                    pri: *pri,
                    severity: *severity,
                    type_: type_.clone(),
                    steps: steps.clone(),
                    expectation: expectation.clone(),
                };
                match TestcaseApi::update(&client, *id, &req).await {
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

            // -------------------- delete --------------------
            TestcaseSubcommand::Delete { id } => {
                if dry_run {
                    println!("[DRY-RUN] Would call TestcaseApi::delete()");
                    println!("  URL: {}/api.php/v1/testcases/{}", config.url, id);
                    return;
                }
                match TestcaseApi::delete(&client, *id).await {
                    Ok(_) => {
                        println!("Testcase {} deleted successfully", id);
                    }
                    Err(e) => {
                        eprintln!("Error: {}", e);
                    }
                }
            }

            // -------------------- result --------------------
            TestcaseSubcommand::Result {
                id,
                result,
                consumed,
                remark,
                build,
            } => {
                if dry_run {
                    println!("[DRY-RUN] Would call TestcaseApi::create_result()");
                    println!("  URL: {}/api.php/v1/testcases/{}/results", config.url, id);
                    println!("  Body:");
                    println!("    result: {}", result);
                    if let Some(c) = consumed {
                        println!("    consumed: {}", c);
                    }
                    if let Some(r) = remark {
                        println!("    remark: {}", r);
                    }
                    if let Some(b) = build {
                        println!("    build: {}", b);
                    }
                    return;
                }
                let req = TestcaseResultRequest {
                    result: result.clone(),
                    consumed: *consumed,
                    remark: remark.clone(),
                    build: *build,
                };
                match TestcaseApi::create_result(&client, *id, &req).await {
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
