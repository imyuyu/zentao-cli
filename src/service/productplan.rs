//! ProductPlan(产品计划) Service 模块
//!
//! 提供产品计划的业务逻辑操作

use crate::api::{ProductPlan, ProductPlanApi};
use crate::core::logging::{log, LogLevel};
use crate::core::AppContext;
use anyhow::Result;

pub struct ProductPlanService;

impl ProductPlanService {
    /// 获取产品计划列表
    pub async fn list(ctx: &AppContext, product: Option<u64>) -> Result<Vec<ProductPlan>> {
        log(LogLevel::Info, "ProductPlanService", "list");
        let client = ctx.client();
        let product_id = ctx.require_product_id(product)?;
        ProductPlanApi::list(&client, product_id).await
    }

    /// 获取单个产品计划详情
    pub async fn get(ctx: &AppContext, id: u64) -> Result<ProductPlan> {
        log(LogLevel::Info, "ProductPlanService", format!("get id={}", id));
        let client = ctx.client();
        ProductPlanApi::get(&client, id).await
    }

    /// 获取产品计划名称
    pub async fn get_name(ctx: &AppContext, id: u64) -> Result<String> {
        let plan = Self::get(ctx, id).await?;
        Ok(plan.name.unwrap_or_default())
    }
}
