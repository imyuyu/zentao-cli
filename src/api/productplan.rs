//! ZenTao ProductPlan(产品计划) API 模块
//!
//! 提供产品计划的查询操作
//!
//! # 概述
//! - ProductPlan（产品计划）：ZenTao 中的产品计划，用于规划产品的版本和里程碑

use anyhow::Result;
use serde::{Deserialize, Serialize};

use super::ApiClient;

// ============================================================
// 数据结构体
// ============================================================

/// 产品计划数据结构
///
/// 对应 ZenTao 系统的产品计划实体
///
/// # JSON 示例
/// ```json
/// {
///     "id": 1,
///     "product": 1,
///     "name": "V1.0 计划",
///     "code": "V1",
///     "status": "done",
///     "type": "ship",
///     "begin": "2024-01-01",
///     "end": "2024-06-30"
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductPlan {
    /// 计划 ID
    pub id: u64,
    /// 所属产品 ID
    pub product: u64,
    /// 计划名称
    #[serde(default)]
    pub name: Option<String>,
    /// 计划代号
    #[serde(default)]
    pub code: Option<String>,
    /// 计划状态：wait（未开始）/doing（进行中）/done（已完成）
    #[serde(default)]
    pub status: Option<String>,
    /// 计划类型：ship（发布计划）/roadmap（路线图）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    /// 描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desc: Option<String>,
    /// 开始日期
    #[serde(skip_serializing_if = "Option::is_none")]
    pub begin: Option<String>,
    /// 结束日期
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<String>,
    /// 真实结束日期
    #[serde(skip_serializing_if = "Option::is_none")]
    pub real_end: Option<String>,
    /// 负责人
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    /// 关联的需求数量
    #[serde(skip_serializing_if = "Option::is_none")]
    pub story_count: Option<u64>,
    /// 关联的 Bug 数量
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bug_count: Option<u64>,
}

// ============================================================
// 请求结构体
// ============================================================

/// 创建产品计划的请求体
#[derive(Debug, Serialize)]
pub struct CreateProductPlanRequest {
    /// 所属产品 ID（必填）
    pub product: u64,
    /// 计划名称（必填）
    pub name: String,
    /// 计划代号
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 计划类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    /// 描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desc: Option<String>,
    /// 开始日期
    #[serde(skip_serializing_if = "Option::is_none")]
    pub begin: Option<String>,
    /// 结束日期
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<String>,
}

/// 更新产品计划的请求体
#[derive(Debug, Serialize)]
pub struct UpdateProductPlanRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desc: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub begin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
}

// ============================================================
// ProductPlan API
// ============================================================

/// 产品计划 API 操作类
pub struct ProductPlanApi;

/// 产品计划列表响应
#[derive(Debug, Deserialize)]
pub struct ProductPlanListResponse {
    #[serde(default)]
    pub plans: Vec<ProductPlan>,
    #[serde(default)]
    pub limit: Option<u64>,
    #[serde(default)]
    pub page: Option<u64>,
    #[serde(default)]
    pub total: Option<u64>,
}

impl ProductPlanApi {
    /// 查询产品计划列表
    ///
    /// GET /api.php/v1/products/{productId}/plans
    ///
    /// # 参数
    /// - product: 产品 ID
    /// - status: 按状态筛选（可选）
    pub async fn list(client: &ApiClient, product: u64) -> Result<Vec<ProductPlan>> {
        Self::list_with_pagination(client, product, None, 1, 100).await
    }

    /// 带分页的产品计划列表查询
    pub async fn list_with_pagination(
        client: &ApiClient,
        product: u64,
        status: Option<String>,
        page: u32,
        limit: u32,
    ) -> Result<Vec<ProductPlan>> {
        let mut path = format!(
            "/api.php/v1/products/{}/plans?page={}&limit={}",
            product, page, limit
        );
        if let Some(s) = status {
            path.push_str(&format!("&status={}", s));
        }
        let resp: ProductPlanListResponse = client.get(&path).await?;
        Ok(resp.plans)
    }

    /// 获取产品计划总数
    pub async fn count(client: &ApiClient, product: u64) -> Result<u64> {
        let path = format!("/api.php/v1/products/{}/plans?page=1&limit=1", product);
        let resp: ProductPlanListResponse = client.get(&path).await?;
        Ok(resp.total.unwrap_or(0))
    }

    /// 获取单个产品计划详情
    ///
    /// GET /api.php/v1/productplans/{id}
    pub async fn get(client: &ApiClient, id: u64) -> Result<ProductPlan> {
        let path = format!("/api.php/v1/productplans/{}", id);
        let resp: ProductPlan = client.get(&path).await?;
        Ok(resp)
    }

    /// 创建产品计划
    ///
    /// POST /api.php/v1/productplans
    pub async fn create(
        client: &ApiClient,
        req: &CreateProductPlanRequest,
    ) -> Result<ProductPlan> {
        #[derive(Deserialize)]
        struct CreateResponse {
            id: Option<u64>,
        }

        let path = "/api.php/v1/productplans";
        let resp: CreateResponse = client.post(path, req).await?;

        if let Some(id) = resp.id {
            Self::get(client, id).await
        } else {
            anyhow::bail!("Failed to create product plan")
        }
    }

    /// 更新产品计划
    ///
    /// PUT /api.php/v1/productplans/{id}
    pub async fn update(
        client: &ApiClient,
        id: u64,
        req: &UpdateProductPlanRequest,
    ) -> Result<ProductPlan> {
        let path = format!("/api.php/v1/productplans/{}", id);
        let _: serde_json::Value = client.put(&path, req).await?;
        Self::get(client, id).await
    }

    /// 删除产品计划
    ///
    /// DELETE /api.php/v1/productplans/{id}
    pub async fn delete(client: &ApiClient, id: u64) -> Result<()> {
        let path = format!("/api.php/v1/productplans/{}", id);
        let _: serde_json::Value = client.delete(&path).await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_product_plan_serialization() {
        let plan = ProductPlan {
            id: 1,
            product: 1,
            name: "V1.0 Plan".to_string(),
            code: "V1".to_string(),
            status: "doing".to_string(),
            type_: Some("ship".to_string()),
            desc: None,
            begin: Some("2024-01-01".to_string()),
            end: Some("2024-06-30".to_string()),
            real_end: None,
            owner: None,
            story_count: None,
            bug_count: None,
        };
        let json = serde_json::to_string(&plan).unwrap();
        assert!(json.contains("V1.0 Plan"));
        assert!(json.contains("V1"));
        assert!(json.contains("doing"));
    }

    #[test]
    fn test_product_plan_deserialization() {
        let json = r#"{
            "id": 10,
            "product": 1,
            "name": "My Plan",
            "code": "PLAN",
            "status": "wait"
        }"#;
        let plan: ProductPlan = serde_json::from_str(json).unwrap();
        assert_eq!(plan.id, 10);
        assert_eq!(plan.name, "My Plan");
        assert_eq!(plan.status, "wait");
    }
}
