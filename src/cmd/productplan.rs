//! ZenTao ProductPlan(产品计划)命令模块

use crate::cmd::common::{log_command, print_dry_run, print_error, print_json};
use crate::cmd::root::ProductPlanSubcommand;
use crate::core::{AppContext, OutputFormat};
use crate::service::productplan::ProductPlanService;

pub async fn run(cmd: &ProductPlanSubcommand, ctx: &AppContext) {
    log_command("productplan", format!("{:?}", cmd));
    match cmd {
        ProductPlanSubcommand::List { product } => {
            let product_id = ctx.product_id(*product);
            if ctx.dry_run {
                if let Some(pid) = product_id {
                    print_dry_run(
                        "ProductPlanService::list()",
                        &format!("{}/api.php/v1/products/{}/plans", ctx.config.url, pid),
                    );
                } else {
                    print_dry_run("ProductPlanService::list()", "product ID is required");
                }
                return;
            }
            match ProductPlanService::list(ctx, product_id).await {
                Ok(plans) => print_productplan_list(&plans, ctx.format.clone()),
                Err(e) => print_error(&e),
            }
        }
        ProductPlanSubcommand::Get { id } => {
            if ctx.dry_run {
                print_dry_run(
                    "ProductPlanService::get()",
                    &format!("{}/api.php/v1/productplans/{}", ctx.config.url, id),
                );
                return;
            }
            match ProductPlanService::get(ctx, *id).await {
                Ok(plan) => print_json(&plan),
                Err(e) => print_error(&e),
            }
        }
    }
}

fn print_productplan_list(plans: &[crate::api::ProductPlan], format: OutputFormat) {
    match format {
        OutputFormat::Table => {
            println!("Product Plans:");
            for item in plans {
                let name = item.name.as_deref().unwrap_or("-");
                let status = item.status.as_deref().unwrap_or("-");
                println!("  [{}] {} - {}", item.id, name, status);
            }
        }
        _ => println!(
            "{}",
            serde_json::to_string_pretty(plans).unwrap_or_default()
        ),
    }
}
