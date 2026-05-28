#![allow(snake_case)]
//! ZenTao Product(产品) API 模块
//!
//! 提供产品的查询操作（禅道产品）
//!
//! # 与其他模块的区别
//! - Product（产品）：ZenTao 中的产品概念，对应一个业务产品线
//! - Project（项目）：具体的开发项目，一个产品下可以有多个项目
//! - Story（需求）：产品需求，一个产品下可以有多个需求
//! - Bug（缺陷）：产品缺陷
//! - Task（任务）：具体的开发任务

use anyhow::Result;
use serde::{Deserialize, Serialize};

use super::ApiClient;
use crate::core::ZentaoError;

// ============================================================
// 数据结构体
// ============================================================

/// 产品数据结构
///
/// 对应 ZenTao 系统的产品实体
///
/// # JSON 示例
/// ```json
/// {
///     "id": 1,
///     "name": "主产品",
///     "code": "MAIN_PRODUCT",
///     "status": "normal",
///     "desc": "产品描述（可选）"
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Product {
    /// 产品 ID（ZenTao 中的唯一标识）
    pub id: u64,
    /// 产品名称
    pub name: String,
    /// 产品代号（英文标识）
    pub code: String,
    /// 产品状态：normal（正常）/closed（关闭）
    pub status: String,
    /// 产品描述（可选字段）
    /// 使用 `skip_serializing_if` 优化：None 时不序列化到 JSON
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desc: Option<String>,
}

// ============================================================
// 请求结构体
// ============================================================

/// 创建产品的请求体
///
/// POST /api.php/v1/products
/// ZenTao API 必填字段: name, code, program
#[derive(Debug, Serialize)]
pub struct CreateProductRequest {
    /// 产品名称（必填）
    pub name: String,
    /// 产品代号（必填）
    pub code: String,
    /// 项目集 ID（必填）
    pub program: u64,
    /// 产品线
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line: Option<u64>,
    /// 产品负责人
    #[serde(skip_serializing_if = "Option::is_none")]
    pub PO: Option<String>,
    /// 测试负责人
    #[serde(skip_serializing_if = "Option::is_none")]
    pub QD: Option<String>,
    /// 发布负责人
    #[serde(skip_serializing_if = "Option::is_none")]
    pub RD: Option<String>,
    /// 产品类型: normal（普通产品）/branch（多分支产品）/platform（多平台产品）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    /// 产品描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desc: Option<String>,
    /// 访问控制: open（公开）/private（私有）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acl: Option<String>,
    /// 白名单
    #[serde(skip_serializing_if = "Option::is_none")]
    pub whitelist: Option<Vec<String>>,
}

/// 更新产品的请求体
///
/// PUT /api.php/v1/products/{id}
#[derive(Debug, Serialize)]
pub struct UpdateProductRequest {
    /// 新名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 新代号
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 产品类型: normal/branch/platform
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    /// 产品线
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line: Option<u64>,
    /// 项目集
    #[serde(skip_serializing_if = "Option::is_none")]
    pub program: Option<u64>,
    /// 新状态：normal/closed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 新描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desc: Option<String>,
    /// 产品负责人
    #[serde(skip_serializing_if = "Option::is_none")]
    pub PO: Option<String>,
    /// 测试负责人
    #[serde(skip_serializing_if = "Option::is_none")]
    pub QD: Option<String>,
    /// 发布负责人
    #[serde(skip_serializing_if = "Option::is_none")]
    pub RD: Option<String>,
}

// ============================================================
// Product API
// ============================================================

/// 产品 API 操作类
///
/// 提供产品的列表查询、详情查询、创建、更新和删除
///
/// # 使用示例
/// ```rust,ignore
/// let products = ProductApi::list(&client).await?;
/// let product = ProductApi::get(&client, 1).await?;
/// ```
pub struct ProductApi;

/// 产品列表响应（ZenTao API 返回格式）
#[derive(Debug, Deserialize)]
pub struct ProductListResponse {
    #[serde(default)]
    pub limit: Option<u64>,
    #[serde(default)]
    pub page: Option<u64>,
    #[serde(default)]
    pub total: Option<u64>,
    pub products: Vec<Product>,
}

impl ProductApi {
    /// 查询产品列表
    ///
    /// GET /api.php/v1/products
    ///
    /// # 返回值
    /// 返回所有有权限访问的产品列表
    pub async fn list(client: &ApiClient) -> Result<Vec<Product>> {
        Self::list_with_pagination(client, 1, 100).await
    }

    /// 带分页的产品列表查询
    pub async fn list_with_pagination(
        client: &ApiClient,
        page: u32,
        limit: u32,
    ) -> Result<Vec<Product>> {
        let path = format!("/api.php/v1/products?page={}&limit={}", page, limit);
        let resp: ProductListResponse = client.get(&path).await?;
        Ok(resp.products)
    }

    /// 获取产品总数
    pub async fn count(client: &ApiClient) -> Result<u64> {
        let path = "/api.php/v1/products?page=1&limit=1".to_string();
        let resp: ProductListResponse = client.get(&path).await?;
        Ok(resp.total.unwrap_or(0))
    }

    /// 获取单个产品详情
    ///
    /// GET /api.php/v1/products/{id}
    ///
    /// # 参数
    /// - `client`: API 客户端实例
    /// - `id`: 产品 ID
    ///
    /// # 返回值
    /// 返回指定产品的完整信息
    pub async fn get(client: &ApiClient, id: u64) -> Result<Product> {
        let path = format!("/api.php/v1/products/{}", id);
        // 产品详情接口返回的是直接的产品对象，不需要 ApiResponse 包装
        let resp: Product = client.get(&path).await?;
        Ok(resp)
    }

    /// 创建新产品
    ///
    /// POST /api.php/v1/products
    ///
    /// ZenTao 创建接口返回 {"id": 123}，需要再调用 get 获取完整信息
    pub async fn create(client: &ApiClient, req: &CreateProductRequest) -> Result<Product> {
        #[derive(Deserialize)]
        struct CreateResponse {
            id: Option<u64>,
        }

        let path = "/api.php/v1/products";
        let resp: CreateResponse = client.post(path, req).await?;

        if let Some(id) = resp.id {
            Self::get(client, id).await
        } else {
            Err(ZentaoError::Api("Failed to create product".to_string()).into())
        }
    }

    /// 更新产品
    ///
    /// PUT /api.php/v1/products/{id}
    ///
    /// ZenTao PUT 接口返回空 JSON {}，需要再调用 get 获取更新后的信息
    pub async fn update(
        client: &ApiClient,
        id: u64,
        req: &UpdateProductRequest,
    ) -> Result<Product> {
        let path = format!("/api.php/v1/products/{}", id);
        let _: serde_json::Value = client.put(&path, req).await?;
        Self::get(client, id).await
    }

    /// 删除产品
    ///
    /// DELETE /api.php/v1/products/{id}
    pub async fn delete(client: &ApiClient, id: u64) -> Result<()> {
        let path = format!("/api.php/v1/products/{}", id);
        let _: serde_json::Value = client.delete(&path).await?;
        Ok(())
    }
}

// ============================================================
// 单元测试
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;

    // ==================== 序列化测试 ====================

    /// 测试产品结构序列化
    ///
    /// 验证 Product 结构能正确序列化为 JSON 字符串
    #[test]
    fn test_product_serialization() {
        let product = Product {
            id: 1,
            name: "Test Product".to_string(),
            code: "TEST".to_string(),
            status: "normal".to_string(),
            desc: None,
        };
        let json = serde_json::to_string(&product).unwrap();
        // 验证基本字段存在
        assert!(json.contains("Test Product"));
        assert!(json.contains("TEST"));
        assert!(json.contains("normal"));
        // 验证 None 的 desc 字段被跳过（不包含在 JSON 中）
        assert!(!json.contains("desc"));
    }

    // ==================== 反序列化测试 ====================

    /// 测试产品 JSON 反序列化
    ///
    /// 验证 JSON 字符串能正确解析为 Product 结构
    #[test]
    fn test_product_deserialization() {
        let product_json = r#"{
            "id": 10,
            "name": "My Product",
            "code": "MYPROD",
            "status": "active"
        }"#;
        let product: Product = serde_json::from_str(product_json).unwrap();
        assert_eq!(product.id, 10);
        assert_eq!(product.name, "My Product");
        assert_eq!(product.code, "MYPROD");
        assert_eq!(product.status, "active");
    }

    /// 测试带描述字段的产品反序列化
    ///
    /// 验证可选字段 desc 能正确解析
    #[test]
    fn test_product_deserialization_with_desc() {
        let product_json = r#"{
            "id": 11,
            "name": "Product With Desc",
            "code": "DESCPROD",
            "status": "normal",
            "desc": "Product description here"
        }"#;
        let product: Product = serde_json::from_str(product_json).unwrap();
        assert_eq!(product.id, 11);
        assert_eq!(product.desc, Some("Product description here".to_string()));
    }

    // ==================== 可选字段跳过测试 ====================

    /// 测试序列化时跳过 None 的 desc 字段
    ///
    /// 验证 `skip_serializing_if = "Option::is_none"` 生效
    #[test]
    fn test_product_skips_none_desc() {
        let product = Product {
            id: 1,
            name: "No Desc".to_string(),
            code: "NODESC".to_string(),
            status: "normal".to_string(),
            desc: None,
        };
        let json = serde_json::to_string(&product).unwrap();
        // desc 为 None 时，不应该出现在 JSON 中
        assert!(!json.contains("desc"));
    }
}
