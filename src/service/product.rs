use crate::api::{CreateProductRequest, Product, ProductApi, UpdateProductRequest};
use crate::core::logging::{log, LogLevel};
use crate::core::AppContext;
use anyhow::Result;

pub struct ProductService;

impl ProductService {
    pub async fn list(ctx: &AppContext) -> Result<Vec<Product>> {
        log(LogLevel::Info, "ProductService", "list");
        let client = ctx.client();
        ProductApi::list(&client).await
    }

    pub async fn list_with_pagination(
        ctx: &AppContext,
        page: u32,
        limit: u32,
    ) -> Result<Vec<Product>> {
        log(
            LogLevel::Info,
            "ProductService",
            format!("list_with_pagination page={} limit={}", page, limit),
        );
        let client = ctx.client();
        ProductApi::list_with_pagination(&client, page, limit).await
    }

    pub async fn get(ctx: &AppContext, id: u64) -> Result<Product> {
        log(LogLevel::Info, "ProductService", format!("get id={}", id));
        let client = ctx.client();
        ProductApi::get(&client, id).await
    }

    pub async fn create(ctx: &AppContext, req: CreateProductRequest) -> Result<Product> {
        log(LogLevel::Info, "ProductService", "create");
        let client = ctx.client();
        ProductApi::create(&client, &req).await
    }

    pub async fn update(ctx: &AppContext, id: u64, req: UpdateProductRequest) -> Result<Product> {
        log(
            LogLevel::Info,
            "ProductService",
            format!("update id={}", id),
        );
        let client = ctx.client();
        ProductApi::update(&client, id, &req).await
    }

    pub async fn delete(ctx: &AppContext, id: u64) -> Result<()> {
        log(
            LogLevel::Info,
            "ProductService",
            format!("delete id={}", id),
        );
        let client = ctx.client();
        ProductApi::delete(&client, id).await
    }

    pub async fn get_name(ctx: &AppContext, id: u64) -> Result<String> {
        log(
            LogLevel::Info,
            "ProductService",
            format!("get_name id={}", id),
        );
        let product = Self::get(ctx, id).await?;
        Ok(product.name)
    }
}
