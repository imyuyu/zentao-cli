//! ZenTao Program(项目集) API 模块
//!
//! 提供项目集的查询操作
//!
//! # 概述
//! - Program（项目集）：ZenTao 中的项目集概念，用于管理多个相关项目

use anyhow::Result;
use serde::{Deserialize, Serialize};

use super::ApiClient;

// ============================================================
// 数据结构体
// ============================================================

/// 项目集数据结构
///
/// 对应 ZenTao 系统的项目集实体
///
/// # JSON 示例
/// ```json
/// {
///     "id": 1,
///     "name": "主项目集",
///     "code": "MAIN_PROGRAM",
///     "status": "doing",
///     "type": "program"
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Program {
    /// 项目集 ID
    pub id: u64,
    /// 项目集名称
    pub name: String,
    /// 项目集代号
    pub code: String,
    /// 项目集状态：doing（进行中）/wait（等待）/closed（已关闭）
    pub status: String,
    /// 项目集类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    /// 描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desc: Option<String>,
    /// 父项目集 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<u64>,
    /// 负责人
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manager: Option<String>,
    /// 开始日期
    #[serde(skip_serializing_if = "Option::is_none")]
    pub begin: Option<String>,
    /// 结束日期
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<String>,
    /// 真实结束日期
    #[serde(skip_serializing_if = "Option::is_none")]
    pub real_end: Option<String>,
    /// 团队名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team: Option<String>,
}

// ============================================================
// 请求结构体
// ============================================================

/// 创建项目集的请求体
#[derive(Debug, Serialize)]
pub struct CreateProgramRequest {
    /// 项目集名称（必填）
    pub name: String,
    /// 项目集代号
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 项目集类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    /// 描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desc: Option<String>,
    /// 父项目集 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<u64>,
}

/// 更新项目集的请求体
#[derive(Debug, Serialize)]
pub struct UpdateProgramRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desc: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manager: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub begin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<String>,
}

// ============================================================
// Program API
// ============================================================

/// 项目集 API 操作类
pub struct ProgramApi;

/// 项目集列表响应
#[derive(Debug, Deserialize)]
pub struct ProgramListResponse {
    #[serde(default)]
    pub programs: Vec<Program>,
    #[serde(default)]
    pub limit: Option<u64>,
    #[serde(default)]
    pub page: Option<u64>,
    #[serde(default)]
    pub total: Option<u64>,
}

impl ProgramApi {
    /// 查询项目集列表
    ///
    /// GET /api.php/v1/programs
    pub async fn list(client: &ApiClient) -> Result<Vec<Program>> {
        Self::list_with_pagination(client, 1, 100).await
    }

    /// 带分页的项目集列表查询
    pub async fn list_with_pagination(
        client: &ApiClient,
        page: u32,
        limit: u32,
    ) -> Result<Vec<Program>> {
        let path = format!("/api.php/v1/programs?page={}&limit={}", page, limit);
        let resp: ProgramListResponse = client.get(&path).await?;
        Ok(resp.programs)
    }

    /// 获取项目集总数
    pub async fn count(client: &ApiClient) -> Result<u64> {
        let path = "/api.php/v1/programs?page=1&limit=1".to_string();
        let resp: ProgramListResponse = client.get(&path).await?;
        Ok(resp.total.unwrap_or(0))
    }

    /// 获取单个项目集详情
    ///
    /// GET /api.php/v1/programs/{id}
    pub async fn get(client: &ApiClient, id: u64) -> Result<Program> {
        let path = format!("/api.php/v1/programs/{}", id);
        let resp: Program = client.get(&path).await?;
        Ok(resp)
    }

    /// 创建新项目集
    ///
    /// POST /api.php/v1/programs
    pub async fn create(client: &ApiClient, req: &CreateProgramRequest) -> Result<Program> {
        #[derive(Deserialize)]
        struct CreateResponse {
            id: Option<u64>,
        }

        let path = "/api.php/v1/programs";
        let resp: CreateResponse = client.post(path, req).await?;

        if let Some(id) = resp.id {
            Self::get(client, id).await
        } else {
            anyhow::bail!("Failed to create program")
        }
    }

    /// 更新项目集
    ///
    /// PUT /api.php/v1/programs/{id}
    pub async fn update(
        client: &ApiClient,
        id: u64,
        req: &UpdateProgramRequest,
    ) -> Result<Program> {
        let path = format!("/api.php/v1/programs/{}", id);
        let _: serde_json::Value = client.put(&path, req).await?;
        Self::get(client, id).await
    }

    /// 删除项目集
    ///
    /// DELETE /api.php/v1/programs/{id}
    pub async fn delete(client: &ApiClient, id: u64) -> Result<()> {
        let path = format!("/api.php/v1/programs/{}", id);
        let _: serde_json::Value = client.delete(&path).await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_program_serialization() {
        let program = Program {
            id: 1,
            name: "Test Program".to_string(),
            code: "TEST".to_string(),
            status: "doing".to_string(),
            type_: Some("program".to_string()),
            desc: None,
            parent: None,
            manager: None,
            begin: None,
            end: None,
            real_end: None,
            team: None,
        };
        let json = serde_json::to_string(&program).unwrap();
        assert!(json.contains("Test Program"));
        assert!(json.contains("TEST"));
        assert!(json.contains("doing"));
    }

    #[test]
    fn test_program_deserialization() {
        let json = r#"{
            "id": 10,
            "name": "My Program",
            "code": "MYPROG",
            "status": "doing",
            "type": "program"
        }"#;
        let program: Program = serde_json::from_str(json).unwrap();
        assert_eq!(program.id, 10);
        assert_eq!(program.name, "My Program");
        assert_eq!(program.status, "doing");
    }
}
