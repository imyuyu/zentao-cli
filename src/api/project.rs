//! ZenTao Project(项目) API 模块
//!
//! 提供项目实体的查询操作
//!
//! # 与 Product（产品）的区别
//! - Product（产品）：业务层面的产品线，关注市场需求和规划
//! - Project（项目）：具体的开发项目，是实现产品的具体工作

use anyhow::Result;
use serde::{Deserialize, Serialize};

use super::ApiClient;

// ============================================================
// 数据结构体
// ============================================================

/// 项目数据结构
///
/// 对应 ZenTao 系统的项目实体
///
/// # JSON 示例
/// ```json
/// {
///     "id": 5,
///     "name": "主产品 v1.0",
///     "code": "MAIN_V1",
///     "status": "doing",
///     "desc": "项目描述（可选）"
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    /// 项目 ID
    pub id: u64,
    /// 项目名称
    pub name: String,
    /// 项目代号
    pub code: String,
    /// 项目状态：wait（未开始）/doing（进行中）/closed（已关闭）
    pub status: String,
    /// 项目描述（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desc: Option<String>,
}

// ============================================================
// Project API
// ============================================================

/// 项目 API 操作类
///
/// 提供项目的列表查询和详情查询
pub struct ProjectApi;

/// 项目列表响应（ZenTao API 返回格式）
#[derive(Debug, Deserialize)]
pub struct ProjectListResponse {
    #[serde(default)]
    pub limit: Option<u64>,
    #[serde(default)]
    pub page: Option<u64>,
    #[serde(default)]
    pub total: Option<u64>,
    pub projects: Vec<Project>,
}

impl ProjectApi {
    /// 查询项目列表
    ///
    /// GET /api.php/v1/projects
    ///
    /// # 返回格式
    /// ZenTao API 返回格式：{"limit": 20, "page": 1, "projects": [...], "total": 2}
    pub async fn list(client: &ApiClient) -> Result<Vec<Project>> {
        Self::list_with_pagination(client, 1, 100).await
    }

    /// 带分页的项目列表查询
    pub async fn list_with_pagination(
        client: &ApiClient,
        page: u32,
        limit: u32,
    ) -> Result<Vec<Project>> {
        let path = format!("/api.php/v1/projects?page={}&limit={}", page, limit);
        let resp: ProjectListResponse = client.get(&path).await?;
        Ok(resp.projects)
    }

    /// 获取项目总数
    pub async fn count(client: &ApiClient) -> Result<u64> {
        let path = "/api.php/v1/projects?page=1&limit=1".to_string();
        let resp: ProjectListResponse = client.get(&path).await?;
        Ok(resp.total.unwrap_or(0))
    }

    /// 获取单个项目详情
    ///
    /// GET /api.php/v1/projects/{id}
    pub async fn get(client: &ApiClient, id: u64) -> Result<Project> {
        let path = format!("/api.php/v1/projects/{}", id);
        // 项目详情接口直接返回项目对象
        let resp: Project = client.get(&path).await?;
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

    /// 测试项目结构序列化
    #[test]
    fn test_project_serialization() {
        let project = Project {
            id: 1,
            name: "Test Project".to_string(),
            code: "TEST".to_string(),
            status: "doing".to_string(),
            desc: None,
        };
        let json = serde_json::to_string(&project).unwrap();
        assert!(json.contains("Test Project"));
        assert!(json.contains("TEST"));
        assert!(json.contains("doing"));
    }

    // ==================== 反序列化测试 ====================

    /// 测试项目 JSON 反序列化
    #[test]
    fn test_project_deserialization() {
        let project_json = r#"{
            "id": 20,
            "name": "My Project",
            "code": "MYPROJ",
            "status": "wait"
        }"#;
        let project: Project = serde_json::from_str(project_json).unwrap();
        assert_eq!(project.id, 20);
        assert_eq!(project.name, "My Project");
        assert_eq!(project.code, "MYPROJ");
        assert_eq!(project.status, "wait");
    }

    /// 测试带描述的项目反序列化
    #[test]
    fn test_project_deserialization_with_desc() {
        let project_json = r#"{
            "id": 21,
            "name": "Project With Desc",
            "code": "DESCPROJ",
            "status": "closed",
            "desc": "Project description here"
        }"#;
        let project: Project = serde_json::from_str(project_json).unwrap();
        assert_eq!(project.id, 21);
        assert_eq!(project.desc, Some("Project description here".to_string()));
    }

    // ==================== 可选字段跳过测试 ====================

    /// 测试序列化时跳过 None 的 desc 字段
    #[test]
    fn test_project_skips_none_desc() {
        let project = Project {
            id: 1,
            name: "No Desc".to_string(),
            code: "NODESC".to_string(),
            status: "normal".to_string(),
            desc: None,
        };
        let json = serde_json::to_string(&project).unwrap();
        assert!(!json.contains("desc"));
    }
}
