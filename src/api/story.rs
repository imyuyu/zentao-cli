//! ZenTao 需求(Story) API 模块
//!
//! 提供需求的增删改查操作

use anyhow::Result;
use serde::{Deserialize, Serialize};

use super::ApiClient;
use crate::api::types::Story;
use crate::core::ZentaoError;

// ============================================================
// 请求结构体 - 对应 ZenTao API 的请求体
// ============================================================

/// 创建需求的请求体
#[derive(Debug, Serialize)]
pub struct CreateStoryRequest {
    /// 需求标题（必填）
    pub title: String,
    /// 所属产品 ID（必填）
    pub product: u64,
    /// 优先级（必填）：0-5
    pub pri: u8,
    /// 需求类别：feature/requirement/bug/improvement
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    /// 需求描述/规格说明
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spec: Option<String>,
    /// 验收标准
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verify: Option<String>,
    /// 预估工时（小时）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimate: Option<f64>,
}

/// 更新需求的请求体
/// 所有字段可选，只更新传入的字段
#[derive(Debug, Serialize)]
pub struct UpdateStoryRequest {
    /// 新标题
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// 新状态：draft/active/closed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 新优先级
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pri: Option<u8>,
    /// 指派给谁
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assigned_to: Option<String>,
}

// ============================================================
// Story API - 需求相关 API 调用
// ============================================================

/// Story 列表分页响应
#[derive(Debug, Deserialize)]
pub struct StoryListResponse {
    #[serde(rename = "stories")]
    pub stories: Option<Vec<Story>>,
    #[serde(default)]
    pub page: Option<u32>,
    #[serde(default)]
    pub limit: Option<u32>,
    #[serde(default)]
    pub total: Option<u64>,
}

pub struct StoryApi;

impl StoryApi {
    /// 查询需求列表
    ///
    /// GET /api.php/v1/stories
    ///
    /// # 参数
    /// - client: API 客户端
    /// - product: 按产品 ID 筛选
    /// - status: 按状态筛选（draft/active/closed）
    /// - project: 按项目 ID 筛选
    ///
    /// # 返回
    /// 满足条件的需求列表
    pub async fn list(
        client: &ApiClient,
        product: Option<u64>,
        status: Option<String>,
        project: Option<u64>,
    ) -> Result<Vec<Story>> {
        Self::list_with_pagination(client, product, status, project, 1, 100).await
    }

    /// 带分页的需求列表查询
    pub async fn list_with_pagination(
        client: &ApiClient,
        product: Option<u64>,
        status: Option<String>,
        project: Option<u64>,
        page: u32,
        limit: u32,
    ) -> Result<Vec<Story>> {
        // ZenTao API 路径：/api.php/v1/products/{productID}/stories
        let mut path = if let Some(pid) = product {
            format!(
                "/api.php/v1/products/{}/stories?page={}&limit={}",
                pid, page, limit
            )
        } else {
            format!("/api.php/v1/stories?page={}&limit={}", page, limit)
        };

        // 添加状态筛选参数
        if let Some(s) = status {
            path.push_str(&format!("&browseType={}", s));
        }
        // 添加项目筛选参数
        if let Some(pid) = project {
            path.push_str(&format!("&projectID={}", pid));
        }

        let resp: StoryListResponse = client.get(&path).await?;
        // 如果 stories 为 None，返回空列表
        Ok(resp.stories.unwrap_or_default())
    }

    /// 获取单个需求详情
    ///
    /// GET /api.php/v1/stories/{id}
    pub async fn get(client: &ApiClient, id: u64) -> Result<Story> {
        let path = format!("/api.php/v1/stories/{}", id);
        let resp: Story = client.get(&path).await?;
        Ok(resp)
    }

    /// 创建新需求
    ///
    /// POST /api.php/v1/stories
    ///
    /// ZenTao 创建接口返回 {"id": 123}，需要再调用 get 获取完整信息
    pub async fn create(client: &ApiClient, req: &CreateStoryRequest) -> Result<Story> {
        #[derive(Deserialize)]
        struct CreateResponse {
            id: Option<u64>,
        }

        let path = "/api.php/v1/stories";
        let resp: CreateResponse = client.post(path, req).await?;

        // 创建成功后，返回的 id 用于获取完整的需求信息
        if let Some(id) = resp.id {
            Self::get(client, id).await
        } else {
            Err(ZentaoError::Api("Failed to create story".to_string()).into())
        }
    }

    /// 更新需求
    ///
    /// PUT /api.php/v1/stories/{id}
    ///
    /// ZenTao PUT 接口返回空 JSON {}，需要再调用 get 获取更新后的信息
    pub async fn update(client: &ApiClient, id: u64, req: &UpdateStoryRequest) -> Result<Story> {
        let path = format!("/api.php/v1/stories/{}", id);
        // 发送 PUT 请求，忽略响应体
        let _: serde_json::Value = client.put(&path, req).await?;
        // 获取更新后的完整需求信息
        Self::get(client, id).await
    }

    /// 变更需求
    ///
    /// POST /api.php/v1/stories/{id}/change
    ///
    /// 用于需求变更操作
    pub async fn change(client: &ApiClient, id: u64, req: &UpdateStoryRequest) -> Result<Story> {
        let path = format!("/api.php/v1/stories/{}/change", id);
        let _: serde_json::Value = client.post(&path, req).await?;
        Self::get(client, id).await
    }

    /// 删除需求
    ///
    /// DELETE /api.php/v1/stories/{id}
    pub async fn delete(client: &ApiClient, id: u64) -> Result<serde_json::Value> {
        let path = format!("/api.php/v1/stories/{}", id);
        let resp: serde_json::Value = client.delete(&path).await?;
        Ok(resp)
    }

    /// 关闭需求
    ///
    /// POST /api.php/v1/stories/{id}/close
    pub async fn close(client: &ApiClient, id: u64) -> Result<Story> {
        let path = format!("/api.php/v1/stories/{}/close", id);
        let _: serde_json::Value = client.post(&path, &serde_json::json!({})).await?;
        Self::get(client, id).await
    }
}

// ============================================================
// 单元测试
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::types::StoryListQuery;

    #[test]
    fn test_create_story_request_serialization() {
        let req = CreateStoryRequest {
            title: "Test Story".to_string(),
            product: 1,
            pri: 3,
            category: Some("feature".to_string()),
            spec: Some("Story spec".to_string()),
            verify: None,
            estimate: Some(5.0),
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("Test Story"));
        assert!(json.contains("\"product\":1"));
        assert!(json.contains("\"pri\":3"));
        assert!(json.contains("feature"));
        assert!(json.contains("Story spec"));
        assert!(json.contains("5"));
    }

    #[test]
    fn test_create_story_request_skips_optional_fields() {
        // 可选字段为 None 时，不应出现在 JSON 中
        let req = CreateStoryRequest {
            title: "Minimal Story".to_string(),
            product: 2,
            pri: 1,
            category: None,
            spec: None,
            verify: None,
            estimate: None,
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("Minimal Story"));
        assert!(!json.contains("category"));
        assert!(!json.contains("spec"));
    }

    #[test]
    fn test_update_story_request_serialization() {
        let req = UpdateStoryRequest {
            title: Some("Updated Title".to_string()),
            status: Some("closed".to_string()),
            pri: None,
            assigned_to: None,
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("Updated Title"));
        assert!(json.contains("closed"));
        // None 字段不应出现
        assert!(!json.contains("\"pri\""));
    }

    #[test]
    fn test_update_story_request_empty() {
        // 所有字段都为 None 时，应序列化为空对象
        let req = UpdateStoryRequest {
            title: None,
            status: None,
            pri: None,
            assigned_to: None,
        };
        let json = serde_json::to_string(&req).unwrap();
        assert_eq!(json, "{}");
    }

    #[test]
    fn test_story_api_deserialization() {
        let story_json = r#"{
            "id": 100,
            "title": "Story Title",
            "status": "active",
            "pri": 3,
            "product": 1
        }"#;
        let story: Story = serde_json::from_str(story_json).unwrap();
        assert_eq!(story.id, 100);
        assert_eq!(story.title, "Story Title");
        assert_eq!(story.status, "active");
        assert_eq!(story.pri, 3);
        assert_eq!(story.product, 1);
    }

    #[test]
    fn test_story_api_deserialization_with_optional_fields() {
        let story_json = r#"{
            "id": 101,
            "title": "Full Story",
            "description": "Story description",
            "status": "closed",
            "pri": 5,
            "category": "requirement",
            "stage": "released",
            "product": 2,
            "module": 10,
            "assigned_to": "admin",
            "estimate": 8.5
        }"#;
        let story: Story = serde_json::from_str(story_json).unwrap();
        assert_eq!(story.id, 101);
        assert_eq!(story.description, Some("Story description".to_string()));
        assert_eq!(story.category, Some("requirement".to_string()));
    }

    #[test]
    fn test_story_list_query_serialization() {
        let query = StoryListQuery {
            product: Some(1),
            project: Some(5),
            status: Some("active".to_string()),
            assigned_to: None,
        };
        let json = serde_json::to_string(&query).unwrap();
        assert!(json.contains("\"product\":1"));
        assert!(json.contains("\"project\":5"));
    }
}
