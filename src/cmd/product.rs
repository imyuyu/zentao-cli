#![allow(snake_case)]
//! ZenTao Product(产品)命令模块

use clap::Subcommand;

use crate::api::product::{CreateProductRequest, UpdateProductRequest};
use crate::cmd::common::{
    log_command, print_deleted, print_dry_run, print_dry_run_with_body, print_error, print_json,
};
use crate::core::{AppContext, OutputFormat};
use crate::service::product::ProductService;

#[derive(Subcommand, Clone, Debug)]
pub enum ProductAction {
    #[command(name = "list")]
    List,
    #[command(name = "get")]
    Get { id: u64 },
    #[command(name = "create")]
    Create {
        #[arg(long)]
        name: String,
        #[arg(long)]
        code: Option<String>,
        #[arg(long)]
        desc: Option<String>,
        /// 项目集ID（必填）
        #[arg(long)]
        program: u64,
        #[arg(long)]
        line: Option<u64>,
        #[arg(long)]
        PO: Option<String>,
        #[arg(long)]
        QD: Option<String>,
        #[arg(long)]
        RD: Option<String>,
        #[arg(long)]
        type_: Option<String>,
        #[arg(long)]
        acl: Option<String>,
        #[arg(long)]
        whitelist: Option<Vec<String>>,
    },
    #[command(name = "update")]
    Update {
        id: u64,
        #[arg(long)]
        name: Option<String>,
        #[arg(long)]
        status: Option<String>,
        #[arg(long)]
        desc: Option<String>,
    },
    #[command(name = "delete")]
    Delete { id: u64 },
}

pub async fn run(cmd: &ProductAction, ctx: &AppContext) {
    log_command("product", format!("{:?}", cmd));
    match cmd {
        ProductAction::List => {
            if ctx.dry_run {
                print_dry_run(
                    "ProductService::list()",
                    &format!("{}/api.php/v1/products", ctx.config.url),
                );
                return;
            }
            match ProductService::list(ctx).await {
                Ok(products) => print_list(&products, ctx.format.clone()),
                Err(e) => print_error(&e),
            }
        }
        ProductAction::Get { id } => {
            if ctx.dry_run {
                print_dry_run(
                    "ProductService::get()",
                    &format!("{}/api.php/v1/products/{}", ctx.config.url, id),
                );
                return;
            }
            match ProductService::get(ctx, *id).await {
                Ok(product) => print_json(&product),
                Err(e) => print_error(&e),
            }
        }
        ProductAction::Create {
            name,
            code,
            desc,
            program,
            line,
            PO,
            QD,
            RD,
            type_,
            acl,
            whitelist,
        } => {
            let req = CreateProductRequest {
                name: name.clone(),
                code: code.as_ref().unwrap_or(&String::new()).clone(),
                program: *program,
                desc: desc.clone(),
                line: *line,
                PO: PO.clone(),
                QD: QD.clone(),
                RD: RD.clone(),
                type_: type_.clone(),
                acl: acl.clone(),
                whitelist: whitelist.clone(),
            };
            if ctx.dry_run {
                print_dry_run_with_body(
                    "ProductService::create()",
                    &format!("{}/api.php/v1/products", ctx.config.url),
                    &req,
                );
                return;
            }
            match ProductService::create(ctx, req).await {
                Ok(product) => print_json(&product),
                Err(e) => print_error(&e),
            }
        }
        ProductAction::Update {
            id,
            name,
            status,
            desc,
        } => {
            let req = UpdateProductRequest {
                name: name.clone(),
                code: None,
                type_: None,
                line: None,
                program: None,
                status: status.clone(),
                desc: desc.clone(),
                PO: None,
                QD: None,
                RD: None,
            };
            if ctx.dry_run {
                print_dry_run_with_body(
                    "ProductService::update()",
                    &format!("{}/api.php/v1/products/{}", ctx.config.url, id),
                    &req,
                );
                return;
            }
            match ProductService::update(ctx, *id, req).await {
                Ok(product) => print_json(&product),
                Err(e) => print_error(&e),
            }
        }
        ProductAction::Delete { id } => {
            if ctx.dry_run {
                print_dry_run(
                    "ProductService::delete()",
                    &format!("{}/api.php/v1/products/{}", ctx.config.url, id),
                );
                return;
            }
            match ProductService::delete(ctx, *id).await {
                Ok(_) => print_deleted("Product", *id),
                Err(e) => print_error(&e),
            }
        }
    }
}

fn print_list(products: &[crate::api::Product], format: OutputFormat) {
    match format {
        OutputFormat::Table => {
            println!("Products:");
            for item in products {
                println!("  [{}] {} - {}", item.id, item.name, item.status);
            }
        }
        _ => println!(
            "{}",
            serde_json::to_string_pretty(products).unwrap_or_default()
        ),
    }
}
