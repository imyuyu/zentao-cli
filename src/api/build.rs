//! ZenTao 版本(Build) API 模块
//!
//! 提供版本的查询操作（禅道版本/构建）

use anyhow::Result;
use serde::{Deserialize, Serialize};

use super::{ApiClient, ApiResponse};
use crate::api::types::{Build, BuildListResponse};
use crate::core::ZentaoError;

// ============================================================
// 请求结构体
// ============================================================

/// 创建版本(Build)的请求体
#[derive(Debug, Serialize)]
pub struct CreateBuildRequest {
    /// 所属执行（必填）
    pub execution: u64,
    /// 所属产品（必填）
    pub product: u64,
    /// 版本名称（必填）
    pub name: String,
    /// 构建者（必填）
    pub builder: String,
    /// 所属分支
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch: Option<u64>,
    /// 打包日期
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    /// 源代码地址
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scm_path: Option<String>,
    /// 下载地址
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "filePath")]
    pub file_path: Option<String>,
    /// 版本描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desc: Option<String>,
}

/// 更新版本(Build)的请求体
/// 所有字段可选，只更新传入的字段
#[derive(Debug, Serialize)]
pub struct UpdateBuildRequest {
    /// 版本名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 源代码地址
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scm_path: Option<String>,
    /// 下载地址
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "filePath")]
    pub file_path: Option<String>,
    /// 打包日期
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    /// 构建者
    #[serde(skip_serializing_if = "Option::is_none")]
    pub builder: Option<String>,
    /// 版本描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desc: Option<String>,
}

// ============================================================
// Build API
// ============================================================

/// 版本 API 操作类
///
/// 提供版本的列表查询和详情查询
///
/// # 使用示例
/// ```rust,ignore
/// let builds = BuildApi::list(&client).await?;
/// let build = BuildApi::get(&client, 1).await?;
/// ```
pub struct BuildApi;

impl BuildApi {
    /// 查询版本列表
    ///
    /// GET /api.php/v1/builds
    ///
    /// # 参数
    /// - client: API 客户端
    /// - project: 按项目 ID 筛选（可选）
    /// - product: 按产品 ID 筛选（可选）
    ///
    /// # 返回值
    /// 返回所有有权限访问的版本列表
    pub async fn list(
        client: &ApiClient,
        project: Option<u64>,
        product: Option<u64>,
    ) -> Result<Vec<Build>> {
        // 如果指定了 project，调用 /api.php/v1/projects/{id}/builds
        if let Some(pid) = project {
            let path = format!("/api.php/v1/projects/{}/builds", pid);
            let resp: BuildListResponse = client.get(&path).await?;
            return Ok(resp.builds);
        }

        // ZenTao API 要求必须指定 project 或 execution
        // 只提供 product 而没有 project 是无效的
        if product.is_some() && project.is_none() {
            anyhow::bail!("--product requires --project to be specified. Use --project to filter builds by product within a project, or use --execution to list builds for a specific execution.");
        }

        // 否则调用 /api.php/v1/builds
        let path = String::from("/api.php/v1/builds");

        let resp: ApiResponse<Vec<Build>> = client.get(&path).await?;
        Ok(resp.data.unwrap_or_default())
    }

    /// 查询执行版本列表
    ///
    /// GET /api.php/v1/executions/{id}/builds
    ///
    /// # 参数
    /// - client: API 客户端
    /// - execution: 执行 ID
    ///
    /// # 返回值
    /// 返回指定执行下的版本列表
    pub async fn list_by_execution(client: &ApiClient, execution: u64) -> Result<Vec<Build>> {
        let path = format!("/api.php/v1/executions/{}/builds", execution);
        let resp: BuildListResponse = client.get(&path).await?;
        Ok(resp.builds)
    }

    /// 获取单个版本详情
    ///
    /// GET /api.php/v1/builds/{id}
    ///
    /// # 参数
    /// - client: API 客户端实例
    /// - id: 版本 ID
    ///
    /// # 返回值
    /// 返回指定版本的完整信息
    pub async fn get(client: &ApiClient, id: u64) -> Result<Build> {
        let path = format!("/api.php/v1/builds/{}", id);
        let resp: Build = client.get(&path).await?;
        Ok(resp)
    }

    /// 创建新版本
    ///
    /// POST /api.php/v1/projects/{projectId}/builds
    ///
    /// ZenTao 创建接口返回 {"id": 123}，需要再调用 get 获取完整信息
    pub async fn create(
        client: &ApiClient,
        project_id: u64,
        req: &CreateBuildRequest,
    ) -> Result<Build> {
        #[derive(Deserialize)]
        struct CreateResponse {
            id: Option<u64>,
        }

        let path = format!("/api.php/v1/projects/{}/builds", project_id);
        let resp: CreateResponse = client.post(&path, req).await?;

        if let Some(id) = resp.id {
            Self::get(client, id).await
        } else {
            Err(ZentaoError::Api("Failed to create build".to_string()).into())
        }
    }

    /// 更新版本
    ///
    /// PUT /api.php/v1/builds/{id}
    pub async fn update(client: &ApiClient, id: u64, req: &UpdateBuildRequest) -> Result<Build> {
        let path = format!("/api.php/v1/builds/{}", id);
        let _: serde_json::Value = client.put(&path, req).await?;
        Self::get(client, id).await
    }

    /// 删除版本
    ///
    /// DELETE /api.php/v1/builds/{id}
    pub async fn delete(client: &ApiClient, id: u64) -> Result<()> {
        let path = format!("/api.php/v1/builds/{}", id);
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

    /// 测试 Build 结构序列化
    #[test]
    fn test_build_serialization() {
        let build = Build {
            id: 1,
            name: "v1.0.0".to_string(),
            product: 1,
            project: 1,
            branch: Some(1),
            scm_path: Some("git@gitlab.example.com:repo.git".to_string()),
            ci: Some("Jenkins #123".to_string()),
            pkg: Some("/path/to/package.tar.gz".to_string()),
            file_size: Some("1048576".to_string()),
            generated_at: Some("2024-01-15 10:00:00".to_string()),
            deleted: Some("0".to_string()),
            editor: Some("admin".to_string()),
            created_by: Some("admin".to_string()),
            created_date: Some("2024-01-15 10:00:00".to_string()),
            last_edited_by: Some("admin".to_string()),
            last_edited_date: Some("2024-01-15 10:00:00".to_string()),
            consumed_cards: Some("10".to_string()),
            stories: Some("5".to_string()),
            bugs: Some("2".to_string()),
        };
        let json = serde_json::to_string(&build).unwrap();
        assert!(json.contains("v1.0.0"));
        assert!(json.contains("\"product\":1"));
        assert!(json.contains("\"project\":1"));
    }

    // ==================== 反序列化测试 ====================

    /// 测试 Build JSON 反序列化
    #[test]
    fn test_build_deserialization() {
        let build_json = r#"{
            "id": 10,
            "name": "Build-2024-01-15",
            "product": 2,
            "project": 3,
            "branch": 1,
            "scm_path": "git@gitlab.example.com:repo.git",
            "ci": "GitLab CI #456",
            "pkg": "/artifacts/app.tar.gz",
            "file_size": "2097152",
            "generated_at": "2024-01-15 14:30:00",
            "deleted": "0",
            "editor": "developer",
            "created_by": "developer",
            "created_date": "2024-01-15 14:30:00",
            "last_edited_by": "developer",
            "last_edited_date": "2024-01-15 14:30:00",
            "consumed_cards": "15",
            "stories": "8",
            "bugs": "3"
        }"#;
        let build: Build = serde_json::from_str(build_json).unwrap();
        assert_eq!(build.id, 10);
        assert_eq!(build.name, "Build-2024-01-15");
        assert_eq!(build.product, 2);
        assert_eq!(build.project, 3);
        assert_eq!(build.branch, Some(1));
        assert_eq!(build.stories, Some("8".to_string()));
        assert_eq!(build.bugs, Some("3".to_string()));
    }

    /// 测试最小 Build JSON 反序列化
    ///
    /// 验证可选字段为 None 时能正确解析
    #[test]
    fn test_build_minimal_deserialization() {
        let build_json = r#"{
            "id": 5,
            "name": "Minimal Build",
            "product": 1,
            "project": 1
        }"#;
        let build: Build = serde_json::from_str(build_json).unwrap();
        assert_eq!(build.id, 5);
        assert_eq!(build.name, "Minimal Build");
        assert_eq!(build.product, 1);
        assert_eq!(build.project, 1);
        assert!(build.branch.is_none());
        assert!(build.scm_path.is_none());
        assert!(build.ci.is_none());
        assert!(build.pkg.is_none());
    }

    // ==================== 可选字段跳过测试 ====================

    /// 测试序列化时跳过 None 的可选字段
    #[test]
    fn test_build_skips_none_fields() {
        let build = Build {
            id: 1,
            name: "Test Build".to_string(),
            product: 1,
            project: 1,
            branch: None,
            scm_path: None,
            ci: None,
            pkg: None,
            file_size: None,
            generated_at: None,
            deleted: None,
            editor: None,
            created_by: None,
            created_date: None,
            last_edited_by: None,
            last_edited_date: None,
            consumed_cards: None,
            stories: None,
            bugs: None,
        };
        let json = serde_json::to_string(&build).unwrap();
        assert!(!json.contains("branch"));
        assert!(!json.contains("scm_path"));
        assert!(!json.contains("ci"));
        assert!(!json.contains("pkg"));
    }

    // ==================== 创建/更新/删除请求测试 ====================

    #[test]
    fn test_create_build_request_serialization() {
        let req = CreateBuildRequest {
            execution: 1,
            product: 1,
            name: "v1.0.0".to_string(),
            builder: "admin".to_string(),
            branch: Some(1),
            date: Some("2024-01-15".to_string()),
            scm_path: Some("git@gitlab.example.com:repo.git".to_string()),
            file_path: Some("/path/to/package.tar.gz".to_string()),
            desc: Some("Build description".to_string()),
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("v1.0.0"));
        assert!(json.contains("\"execution\":1"));
        assert!(json.contains("\"product\":1"));
        assert!(json.contains("\"builder\":\"admin\""));
    }

    #[test]
    fn test_create_build_request_skips_optional_fields() {
        let req = CreateBuildRequest {
            execution: 1,
            product: 1,
            name: "Minimal Build".to_string(),
            builder: "admin".to_string(),
            branch: None,
            date: None,
            scm_path: None,
            file_path: None,
            desc: None,
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("Minimal Build"));
        assert!(!json.contains("branch"));
        assert!(!json.contains("scm_path"));
    }

    #[test]
    fn test_update_build_request_serialization() {
        let req = UpdateBuildRequest {
            name: Some("Updated Build".to_string()),
            scm_path: Some("git@gitlab.example.com:updated.git".to_string()),
            file_path: None,
            date: None,
            builder: None,
            desc: None,
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("Updated Build"));
        assert!(json.contains("updated.git"));
    }

    #[test]
    fn test_update_build_request_partial() {
        let req = UpdateBuildRequest {
            name: None,
            scm_path: None,
            file_path: Some("/new/path.tar.gz".to_string()),
            date: Some("2024-02-01".to_string()),
            builder: Some("developer".to_string()),
            desc: None,
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("/new/path.tar.gz"));
        assert!(!json.contains("\"name\""));
    }
}
