//! ZenTao Release(发布) API 模块
//!
//! 提供发布的查询操作（禅道产品发布）

use anyhow::Result;
use serde::{Deserialize, Serialize};

use super::{ApiClient, ApiResponse};

// ============================================================
// 数据结构体
// ============================================================

/// 发布数据结构
///
/// 对应 ZenTao 系统的发布实体
///
/// # JSON 示例
/// ```json
/// {
///     "id": 1,
///     "name": "v1.0.0",
///     "product": 1,
///     "build": 10,
///     "status": "normal",
///     "marker": "stable",
///     "date": "2024-01-15"
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Release {
    /// 发布 ID（ZenTao 中的唯一标识）
    pub id: u64,
    /// 发布名称
    pub name: String,
    /// 所属产品 ID
    #[serde(deserialize_with = "crate::api::types::deserialize_optional_id")]
    pub product: Option<u64>,
    /// 关联的 Build（版本）ID
    #[serde(
        default,
        deserialize_with = "crate::api::types::deserialize_optional_id",
        skip_serializing_if = "Option::is_none"
    )]
    pub build: Option<u64>,
    /// 发布状态：normal（正常）/closed（关闭）
    pub status: String,
    /// 发布标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    /// 发布日期
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
}

impl Release {
    pub fn web_url(&self, base_url: &str) -> String {
        format!("{}/release-view-{}.html", base_url, self.id)
    }
}

// ============================================================
// Release API
// ============================================================

/// 发布 API 操作类
///
/// 提供发布的列表查询和详情查询
///
/// # 使用示例
/// ```rust,ignore
/// let releases = ReleaseApi::list(&client).await?;
/// let release = ReleaseApi::get(&client, 1).await?;
/// ```
pub struct ReleaseApi;

impl ReleaseApi {
    /// 查询发布列表（所有）
    ///
    /// GET /api.php/v1/releases
    ///
    /// # 返回值
    /// 返回所有有权限访问的发布列表
    pub async fn list(client: &ApiClient) -> Result<Vec<Release>> {
        let resp: ApiResponse<Vec<Release>> = client.get("/api.php/v1/releases").await?;
        Ok(resp.data.unwrap_or_default())
    }

    /// 查询产品发布列表
    ///
    /// GET /api.php/v1/products/{productId}/releases
    pub async fn list_by_product(client: &ApiClient, product: u64) -> Result<Vec<Release>> {
        let path = format!("/api.php/v1/products/{}/releases", product);
        #[derive(Deserialize)]
        struct ReleaseListResponse {
            #[serde(rename = "releases")]
            releases: Option<Vec<Release>>,
        }
        let resp: ReleaseListResponse = client.get(&path).await?;
        Ok(resp.releases.unwrap_or_default())
    }

    /// 查询项目发布列表
    ///
    /// GET /api.php/v1/projects/{projectId}/releases
    pub async fn list_by_project(client: &ApiClient, project: u64) -> Result<Vec<Release>> {
        let path = format!("/api.php/v1/projects/{}/releases", project);
        #[derive(Deserialize)]
        struct ReleaseListResponse {
            #[serde(rename = "releases")]
            releases: Option<Vec<Release>>,
        }
        let resp: ReleaseListResponse = client.get(&path).await?;
        Ok(resp.releases.unwrap_or_default())
    }

    /// 获取单个发布详情
    ///
    /// GET /api.php/v1/releases/{id}
    ///
    /// # 参数
    /// - `client`: API 客户端实例
    /// - `id`: 发布 ID
    ///
    /// # 返回值
    /// 返回指定发布的完整信息
    pub async fn get(client: &ApiClient, id: u64) -> Result<Release> {
        let path = format!("/api.php/v1/releases/{}", id);
        // 发布详情接口返回的是直接的对象，不需要 ApiResponse 包装
        let resp: Release = client.get(&path).await?;
        Ok(resp)
    }
}

// ============================================================
// 单元测试
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;

    // ==================== 序列化测试 ====================

    /// 测试发布结构序列化
    ///
    /// 验证 Release 结构能正确序列化为 JSON 字符串
    #[test]
    fn test_release_serialization() {
        let release = Release {
            id: 1,
            name: "v1.0.0".to_string(),
            product: Some(1),
            build: Some(10),
            status: "normal".to_string(),
            marker: Some("stable".to_string()),
            date: Some("2024-01-15".to_string()),
        };
        let json = serde_json::to_string(&release).unwrap();
        // 验证基本字段存在
        assert!(json.contains("v1.0.0"));
        assert!(json.contains("normal"));
        assert!(json.contains("stable"));
    }

    // ==================== 反序列化测试 ====================

    /// 测试发布 JSON 反序列化
    ///
    /// 验证 JSON 字符串能正确解析为 Release 结构
    #[test]
    fn test_release_deserialization() {
        let release_json = r#"{
            "id": 10,
            "name": "v2.0.0",
            "product": 1,
            "build": 20,
            "status": "normal",
            "marker": "beta",
            "date": "2024-06-01"
        }"#;
        let release: Release = serde_json::from_str(release_json).unwrap();
        assert_eq!(release.id, 10);
        assert_eq!(release.name, "v2.0.0");
        assert_eq!(release.product, Some(1));
        assert_eq!(release.build, Some(20));
        assert_eq!(release.status, "normal");
        assert_eq!(release.marker, Some("beta".to_string()));
        assert_eq!(release.date, Some("2024-06-01".to_string()));
    }

    /// 测试带可选字段为空的发布反序列化
    ///
    /// 验证可选字段 build/marker/date 为空时能正确解析
    #[test]
    fn test_release_deserialization_optional_fields() {
        let release_json = r#"{
            "id": 11,
            "name": "v3.0.0",
            "product": 2,
            "status": "closed"
        }"#;
        let release: Release = serde_json::from_str(release_json).unwrap();
        assert_eq!(release.id, 11);
        assert_eq!(release.name, "v3.0.0");
        assert_eq!(release.build, None);
        assert_eq!(release.marker, None);
        assert_eq!(release.date, None);
    }

    // ==================== 可选字段跳过测试 ====================

    /// 测试序列化时跳过 None 的可选字段
    ///
    /// 验证 `skip_serializing_if = "Option::is_none"` 生效
    #[test]
    fn test_release_skips_none_optional_fields() {
        let release = Release {
            id: 1,
            name: "No Optional".to_string(),
            product: Some(1),
            build: None,
            status: "normal".to_string(),
            marker: None,
            date: None,
        };
        let json = serde_json::to_string(&release).unwrap();
        // None 的可选字段不应该出现在 JSON 中
        assert!(!json.contains("build"));
        assert!(!json.contains("marker"));
        assert!(!json.contains("date"));
    }
}
