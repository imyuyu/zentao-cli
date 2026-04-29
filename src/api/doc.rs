//! ZenTao Doc(文档) API 模块
//!
//! 提供文档的查询操作（禅道文档库）
//!
//! # 文档类型说明
//! - lib: 文档库 ID
//! - type: 文档类型（如 doc, article 等）

use anyhow::Result;
use serde::{Deserialize, Serialize};

use super::{ApiClient, ApiResponse};

// ============================================================
// 数据结构体
// ============================================================

/// 文档数据结构
///
/// 对应 ZenTao 系统的文档实体
///
/// # JSON 示例
/// ```json
/// {
///     "id": 1,
///     "title": "技术文档",
///     "product": 1,
///     "project": 1,
///     "lib": 1,
///     "type": "doc",
///     "size": "1024",
///     "addedBy": "admin",
///     "addedDate": "2024-01-01 00:00:00",
///     "editedDate": "2024-01-02 00:00:00",
///     "deleted": "0"
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Doc {
    /// 文档 ID
    pub id: u64,
    /// 文档标题
    pub title: String,
    /// 所属产品 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product: Option<u64>,
    /// 所属项目 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project: Option<u64>,
    /// 所属文档库 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lib: Option<u64>,
    /// 文档类型
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    /// 文档大小（字节）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    /// 创建者账号
    #[serde(rename = "addedBy", skip_serializing_if = "Option::is_none")]
    pub added_by: Option<String>,
    /// 创建日期
    #[serde(rename = "addedDate", skip_serializing_if = "Option::is_none")]
    pub added_date: Option<String>,
    /// 最后编辑日期
    #[serde(rename = "editedDate", skip_serializing_if = "Option::is_none")]
    pub edited_date: Option<String>,
    /// 是否已删除：0-未删除，1-已删除
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted: Option<String>,
}

// ============================================================
// Doc API
// ============================================================

/// 文档 API 操作类
///
/// 提供文档的列表查询和详情查询
///
/// # 使用示例
/// ```rust,ignore
/// let docs = DocApi::list(&client).await?;
/// let doc = DocApi::get(&client, 1).await?;
/// ```
pub struct DocApi;

impl DocApi {
    /// 查询文档列表
    ///
    /// GET /api.php/v1/docs
    ///
    /// # 返回值
    /// 返回所有有权限访问的文档列表
    pub async fn list(client: &ApiClient) -> Result<Vec<Doc>> {
        let resp: ApiResponse<Vec<Doc>> = client.get("/api.php/v1/docs").await?;
        Ok(resp.data.unwrap_or_default())
    }

    /// 获取单个文档详情
    ///
    /// GET /api.php/v1/docs/{id}
    ///
    /// # 参数
    /// - `client`: API 客户端实例
    /// - `id`: 文档 ID
    ///
    /// # 返回值
    /// 返回指定文档的完整信息
    pub async fn get(client: &ApiClient, id: u64) -> Result<Doc> {
        let path = format!("/api.php/v1/docs/{}", id);
        let doc: Doc = client.get(&path).await?;
        Ok(doc)
    }
}

// ============================================================
// 单元测试
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;

    // ==================== 序列化测试 ====================

    /// 测试文档结构序列化
    ///
    /// 验证 Doc 结构能正确序列化为 JSON 字符串
    #[test]
    fn test_doc_serialization() {
        let doc = Doc {
            id: 1,
            title: "Test Doc".to_string(),
            product: Some(1),
            project: Some(1),
            lib: Some(1),
            type_: Some("doc".to_string()),
            size: Some("1024".to_string()),
            added_by: Some("admin".to_string()),
            added_date: Some("2024-01-01 00:00:00".to_string()),
            edited_date: Some("2024-01-02 00:00:00".to_string()),
            deleted: Some("0".to_string()),
        };
        let json = serde_json::to_string(&doc).unwrap();
        assert!(json.contains("Test Doc"));
        assert!(json.contains("doc"));
        assert!(json.contains("1024"));
    }

    // ==================== 反序列化测试 ====================

    /// 测试文档 JSON 反序列化
    ///
    /// 验证 JSON 字符串能正确解析为 Doc 结构
    #[test]
    fn test_doc_deserialization() {
        let doc_json = r#"{
            "id": 10,
            "title": "My Doc",
            "product": 1,
            "lib": 2,
            "type": "article",
            "addedBy": "admin",
            "deleted": "0"
        }"#;
        let doc: Doc = serde_json::from_str(doc_json).unwrap();
        assert_eq!(doc.id, 10);
        assert_eq!(doc.title, "My Doc");
        assert_eq!(doc.product, Some(1));
        assert_eq!(doc.lib, Some(2));
        assert_eq!(doc.type_, Some("article".to_string()));
        assert_eq!(doc.added_by, Some("admin".to_string()));
    }

    /// 测试最小文档 JSON 反序列化
    ///
    /// 验证只有必填字段时能正确解析
    #[test]
    fn test_doc_minimal_deserialization() {
        let doc_json = r#"{
            "id": 1,
            "title": "Minimal Doc"
        }"#;
        let doc: Doc = serde_json::from_str(doc_json).unwrap();
        assert_eq!(doc.id, 1);
        assert_eq!(doc.title, "Minimal Doc");
        assert_eq!(doc.product, None);
        assert_eq!(doc.lib, None);
    }

    // ==================== 可选字段跳过测试 ====================

    /// 测试序列化时跳过 None 的可选字段
    ///
    /// 验证 `skip_serializing_if = "Option::is_none"` 生效
    #[test]
    fn test_doc_skips_none_fields() {
        let doc = Doc {
            id: 1,
            title: "No Optional".to_string(),
            product: None,
            project: None,
            lib: None,
            type_: None,
            size: None,
            added_by: None,
            added_date: None,
            edited_date: None,
            deleted: None,
        };
        let json = serde_json::to_string(&doc).unwrap();
        // 可选字段都不应该出现在 JSON 中
        assert!(!json.contains("product"));
        assert!(!json.contains("project"));
        assert!(!json.contains("lib"));
        assert!(!json.contains("type"));
    }
}
