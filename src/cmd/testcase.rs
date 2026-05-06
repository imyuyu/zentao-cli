//! ZenTao Testcase(测试用例)命令模块

use crate::api::testcase::{CreateTestcaseRequest, TestcaseResultRequest, UpdateTestcaseRequest};
use crate::cmd::common::{
    log_command, print_deleted, print_dry_run, print_dry_run_with_body, print_error, print_json,
};
use crate::cmd::root::TestcaseSubcommand;
use crate::core::{AppContext, OutputFormat};
use crate::service::testcase::TestcaseService;

pub async fn run(cmd: &TestcaseSubcommand, ctx: &AppContext) {
    log_command("testcase", format!("{:?}", cmd));
    match cmd {
        TestcaseSubcommand::List {
            product,
            project,
            type_,
            status,
        } => {
            let product_id = ctx.product_id(*product);
            let project_id = ctx.project_id(*project);
            if ctx.dry_run {
                print_dry_run(
                    "TestcaseService::list()",
                    &format!("{}/api.php/v1/testcases", ctx.config.url),
                );
                println!("  Params:");
                if let Some(p) = product_id {
                    println!("    product: {}", p);
                }
                if let Some(p) = project_id {
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
            match TestcaseService::list(ctx, *product, *project, type_.clone(), status.clone())
                .await
            {
                Ok(testcases) => print_testcase_list(&testcases, ctx.format.clone()),
                Err(e) => print_error(&e),
            }
        }
        TestcaseSubcommand::Get { id } => {
            if ctx.dry_run {
                print_dry_run(
                    "TestcaseService::get()",
                    &format!("{}/api.php/v1/testcases/{}", ctx.config.url, id),
                );
                return;
            }
            match TestcaseService::get(ctx, *id).await {
                Ok(testcase) => print_json(&testcase),
                Err(e) => print_error(&e),
            }
        }
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
            let product_id = match ctx.require_product_id(*product) {
                Ok(id) => id,
                Err(e) => {
                    print_error(&e);
                    return;
                }
            };
            let req = CreateTestcaseRequest {
                title: title.clone(),
                product: product_id,
                type_: type_.clone(),
                severity: *severity,
                pri: *pri,
                steps: steps.clone(),
                expectation: expectation.clone(),
                story: *story,
                project: ctx.project_id(*project),
            };
            if ctx.dry_run {
                print_dry_run_with_body(
                    "TestcaseService::create()",
                    &format!(
                        "{}/api.php/v1/products/{}/testcases",
                        ctx.config.url, product_id
                    ),
                    &req,
                );
                return;
            }
            match TestcaseService::create(ctx, *product, req).await {
                Ok(testcase) => print_json(&testcase),
                Err(e) => print_error(&e),
            }
        }
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
            let req = UpdateTestcaseRequest {
                title: title.clone(),
                status: status.clone(),
                pri: *pri,
                severity: *severity,
                type_: type_.clone(),
                steps: steps.clone(),
                expectation: expectation.clone(),
            };
            if ctx.dry_run {
                print_dry_run_with_body(
                    "TestcaseService::update()",
                    &format!("{}/api.php/v1/testcases/{}", ctx.config.url, id),
                    &req,
                );
                return;
            }
            match TestcaseService::update(ctx, *id, req).await {
                Ok(testcase) => print_json(&testcase),
                Err(e) => print_error(&e),
            }
        }
        TestcaseSubcommand::Delete { id } => {
            if ctx.dry_run {
                print_dry_run(
                    "TestcaseService::delete()",
                    &format!("{}/api.php/v1/testcases/{}", ctx.config.url, id),
                );
                return;
            }
            match TestcaseService::delete(ctx, *id).await {
                Ok(_) => print_deleted("Testcase", *id),
                Err(e) => print_error(&e),
            }
        }
        TestcaseSubcommand::Result {
            id,
            result,
            consumed,
            remark,
            build,
        } => {
            let req = TestcaseResultRequest {
                result: result.clone(),
                consumed: *consumed,
                remark: remark.clone(),
                build: *build,
            };
            if ctx.dry_run {
                print_dry_run_with_body(
                    "TestcaseService::create_result()",
                    &format!("{}/api.php/v1/testcases/{}/results", ctx.config.url, id),
                    &req,
                );
                return;
            }
            match TestcaseService::create_result(ctx, *id, req).await {
                Ok(testcase) => print_json(&testcase),
                Err(e) => print_error(&e),
            }
        }
    }
}

fn print_testcase_list(items: &[crate::api::Testcase], format: OutputFormat) {
    match format {
        OutputFormat::Table => {
            println!("Testcases:");
            for item in items {
                println!("  [{}] {} - {}", item.id, item.title, item.status);
            }
        }
        _ => println!(
            "{}",
            serde_json::to_string_pretty(items).unwrap_or_default()
        ),
    }
}
