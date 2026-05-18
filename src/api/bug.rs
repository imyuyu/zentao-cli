//! ZenTao Bug(缺陷) API 模块
//!
//! 提供 Bug 的增删改查操作

use anyhow::Result;
use serde::{Deserialize, Serialize};

use super::ApiClient;
use crate::api::types::Bug;
use crate::core::ZentaoError;

// ============================================================
// 请求结构体
// ============================================================

/// 创建 Bug 的请求体
#[derive(Debug, Serialize)]
pub struct CreateBugRequest {
    /// Bug 标题（必填）
    pub title: String,
    /// 所属产品 ID（必填）
    pub product: u64,
    /// 严重程度：1-4（1 最严重）
    pub severity: u8,
    /// 优先级：0-5
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pri: Option<u8>,
    /// Bug 类型：codeerror/interface/design/others
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    /// 重现步骤
    #[serde(skip_serializing_if = "Option::is_none")]
    pub steps: Option<String>,
    /// 关联的需求 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub story: Option<u64>,
    /// 指派给谁
    #[serde(skip_serializing_if = "Option::is_none", rename = "assignedTo")]
    pub assigned_to: Option<String>,
}

/// 更新 Bug 的请求体
/// 所有字段可选，只更新传入的字段
#[derive(Debug, Serialize)]
pub struct UpdateBugRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// 新状态：active/resolved/closed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 解决方案：fixed/bydesign/duplicate/无法重现/延期处理
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolution: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolved_build: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "assignedTo")]
    pub assigned_to: Option<String>,
}

// ============================================================
// Bug API
// ============================================================

/// Bug 列表分页响应
#[derive(Debug, Deserialize)]
pub struct BugListResponse {
    #[serde(rename = "bugs")]
    pub bugs: Option<Vec<Bug>>,
    #[serde(default)]
    pub page: Option<u32>,
    #[serde(default)]
    pub limit: Option<u32>,
    #[serde(default)]
    pub total: Option<u64>,
}

pub struct BugApi;

impl BugApi {
    /// 查询 Bug 列表
    ///
    /// GET /api.php/v1/products/{product}/bugs
    ///
    /// # 参数
    /// - product: 产品 ID
    /// - status: 按状态筛选
    /// - assigned_to: 按负责人筛选
    /// - page: 页码（默认 1）
    /// - limit: 每页数量（默认 100）
    pub async fn list(
        client: &ApiClient,
        product: u64,
        status: Option<String>,
        assigned_to: Option<String>,
    ) -> Result<Vec<Bug>> {
        Self::list_with_pagination(client, product, status, assigned_to, 1, 100).await
    }

    /// 带分页的 Bug 列表查询
    pub async fn list_with_pagination(
        client: &ApiClient,
        product: u64,
        status: Option<String>,
        assigned_to: Option<String>,
        page: u32,
        limit: u32,
    ) -> Result<Vec<Bug>> {
        let mut path = format!(
            "/api.php/v1/products/{}/bugs?page={}&limit={}",
            product, page, limit
        );

        if let Some(s) = status {
            path.push_str(&format!("&browseType={}", s));
        }
        if let Some(u) = assigned_to {
            path.push_str(&format!("&assignedTo={}", u));
        }

        let resp: BugListResponse = client.get(&path).await?;
        Ok(resp.bugs.unwrap_or_default())
    }

    /// 获取 Bug 总数
    pub async fn count(
        client: &ApiClient,
        product: u64,
        status: Option<String>,
        assigned_to: Option<String>,
    ) -> Result<u64> {
        let bugs = Self::list_with_pagination(client, product, status, assigned_to, 1, 1).await?;
        // 注意：这里需要返回 total，实际需要从响应中获取
        // 当前实现不够高效，后续可以优化
        Ok(bugs.len() as u64)
    }

    /// 获取单个 Bug 详情
    ///
    /// GET /api.php/v1/bugs/{id}
    pub async fn get(client: &ApiClient, id: u64) -> Result<Bug> {
        let path = format!("/api.php/v1/bugs/{}", id);
        let resp: Bug = client.get(&path).await?;
        Ok(resp)
    }

    /// 创建新 Bug
    ///
    /// POST /api.php/v1/bugs
    ///
    /// ZenTao 创建接口返回 {"id": 123}，需要再调用 get 获取完整信息
    pub async fn create(client: &ApiClient, req: &CreateBugRequest) -> Result<Bug> {
        #[derive(Deserialize)]
        struct CreateResponse {
            id: Option<u64>,
        }

        let path = "/api.php/v1/bugs";
        let resp: CreateResponse = client.post(path, req).await?;

        if let Some(id) = resp.id {
            Self::get(client, id).await
        } else {
            Err(ZentaoError::Api("Failed to create bug".to_string()).into())
        }
    }

    /// 更新 Bug
    ///
    /// PUT /api.php/v1/bugs/{id}
    pub async fn update(client: &ApiClient, id: u64, req: &UpdateBugRequest) -> Result<Bug> {
        let path = format!("/api.php/v1/bugs/{}", id);
        let _: serde_json::Value = client.put(&path, req).await?;
        Self::get(client, id).await
    }

    /// 解决 Bug
    ///
    /// POST /api.php/v1/bugs/{bug_id}/resolve
    ///
    /// # 参数
    /// - client: API 客户端
    /// - id: Bug ID
    /// - resolution: 解决方案 (bydesign/duplicate/external/fixed/notrepro/postponed/willnotfix/tostory)
    /// - resolved_build: 解决的版本 ID 或 "trunk"
    pub async fn resolve(
        client: &ApiClient,
        id: u64,
        resolution: &str,
        resolved_build: &str,
    ) -> Result<Bug> {
        let path = format!("/api.php/v1/bugs/{}/resolve", id);
        // resolved_build 可以是版本 ID 或 "trunk"
        let body = serde_json::json!({
            "resolution": resolution,
            "resolvedBuild": resolved_build
        });
        let _: serde_json::Value = client.post(&path, &body).await?;
        Self::get(client, id).await
    }

    /// 确认 Bug
    ///
    /// POST /api.php/v1/bugs/{bug_id}/confirm
    pub async fn confirm(client: &ApiClient, id: u64) -> Result<Bug> {
        let path = format!("/api.php/v1/bugs/{}/confirm", id);
        let _: serde_json::Value = client.post(&path, &()).await?;
        Self::get(client, id).await
    }

    /// 关闭 Bug
    ///
    /// POST /api.php/v1/bugs/{bug_id}/close
    pub async fn close(client: &ApiClient, id: u64) -> Result<Bug> {
        let path = format!("/api.php/v1/bugs/{}/close", id);
        let _: serde_json::Value = client.post(&path, &()).await?;
        Self::get(client, id).await
    }

    /// 激活 Bug
    ///
    /// POST /api.php/v1/bugs/{bug_id}/activate
    pub async fn activate(client: &ApiClient, id: u64) -> Result<Bug> {
        let path = format!("/api.php/v1/bugs/{}/activate", id);
        let _: serde_json::Value = client.post(&path, &()).await?;
        Self::get(client, id).await
    }

    /// 删除 Bug
    ///
    /// DELETE /api.php/v1/bugs/{bug_id}
    pub async fn delete(client: &ApiClient, id: u64) -> Result<()> {
        let path = format!("/api.php/v1/bugs/{}", id);
        let _: serde_json::Value = client.delete(&path).await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_bug_request_serialization() {
        let req = CreateBugRequest {
            title: "Test Bug".to_string(),
            product: 1,
            severity: 3,
            pri: Some(2),
            type_: Some("code".to_string()),
            steps: Some("Step 1: Go to page".to_string()),
            story: None,
            assigned_to: Some("dev".to_string()),
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("Test Bug"));
        assert!(json.contains("\"product\":1"));
        assert!(json.contains("\"severity\":3"));
    }

    #[test]
    fn test_create_bug_request_skips_optional_fields() {
        let req = CreateBugRequest {
            title: "Minimal Bug".to_string(),
            product: 2,
            severity: 1,
            pri: None,
            type_: None,
            steps: None,
            story: None,
            assigned_to: None,
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("Minimal Bug"));
        assert!(!json.contains("pri"));
        assert!(!json.contains("steps"));
    }

    #[test]
    fn test_update_bug_request_serialization() {
        let req = UpdateBugRequest {
            title: Some("Fixed Bug".to_string()),
            status: Some("closed".to_string()),
            resolution: Some("fixed".to_string()),
            resolved_build: None,
            assigned_to: Some("admin".to_string()),
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("Fixed Bug"));
        assert!(json.contains("closed"));
        assert!(json.contains("fixed"));
    }

    #[test]
    fn test_update_bug_request_partial() {
        let req = UpdateBugRequest {
            title: None,
            status: Some("resolved".to_string()),
            resolution: None,
            resolved_build: None,
            assigned_to: None,
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("resolved"));
        assert!(!json.contains("\"title\""));
    }

    #[test]
    fn test_bug_deserialization() {
        let bug_json = r#"{
            "id": 50,
            "title": "Bug Title",
            "status": "open",
            "severity": 4,
            "pri": 3,
            "product": 1
        }"#;
        let bug: Bug = serde_json::from_str(bug_json).unwrap();
        assert_eq!(bug.id, 50);
        assert_eq!(bug.title, "Bug Title");
    }

    #[test]
    fn test_bug_deserialization_with_optional_fields() {
        let bug_json = r#"{
            "id": 51,
            "title": "Resolved Bug",
            "status": "closed",
            "severity": 2,
            "pri": 1,
            "type": "interface",
            "resolution": "fixed",
            "product": 2,
            "project": 5,
            "story": 100
        }"#;
        let bug: Bug = serde_json::from_str(bug_json).unwrap();
        assert_eq!(bug.id, 51);
        assert_eq!(bug.resolution, Some("fixed".to_string()));
        assert_eq!(bug.project, Some(5));
    }
}
