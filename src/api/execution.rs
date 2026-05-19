//! ZenTao 执行(Execution) API 模块
//!
//! 提供执行的增删改查操作

use anyhow::Result;
use serde::{Deserialize, Serialize};

use super::ApiClient;
use crate::api::types::Execution;
use crate::core::ZentaoError;

// ============================================================
// 请求结构体 - 对应 ZenTao API 的请求体
// ============================================================

/// 创建执行的请求体
#[derive(Debug, Serialize)]
pub struct CreateExecutionRequest {
    /// 执行名称（必填）
    pub name: String,
    /// 所属项目 ID（必填）
    pub project: u64,
    /// 执行代号（必填）
    pub code: String,
    /// 计划开始日期（必填）
    pub begin: String,
    /// 计划结束日期（必填）
    pub end: String,
    /// 执行类型：iteration/milestone
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub type_: Option<String>,
    /// 可用工作日
    #[serde(skip_serializing_if = "Option::is_none")]
    pub days: Option<u64>,
    /// 类型(short/长期/long/短期/ops/运维)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifetime: Option<String>,
    /// 产品负责人
    #[serde(skip_serializing_if = "Option::is_none")]
    pub PO: Option<String>,
    /// 迭代负责人
    #[serde(skip_serializing_if = "Option::is_none")]
    pub PM: Option<String>,
    /// 测试负责人
    #[serde(skip_serializing_if = "Option::is_none")]
    pub QD: Option<String>,
    /// 发布负责人
    #[serde(skip_serializing_if = "Option::is_none")]
    pub RD: Option<String>,
    /// 团队成员
    #[serde(skip_serializing_if = "Option::is_none")]
    pub teamMembers: Option<Vec<String>>,
    /// 执行描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desc: Option<String>,
    /// 访问控制（private/私有/open/继承项目权限）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acl: Option<String>,
    /// 白名单（acl=private时生效）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub whitelist: Option<Vec<String>>,
}

/// 更新执行的请求体
/// 所有字段可选，只更新传入的字段
///
/// 注意：官方文档列出 project, name, code, begin, end 为必填字段，
/// 但 ZenTao API 实际支持部分更新，只传入需要修改的字段即可
#[derive(Debug, Serialize)]
pub struct UpdateExecutionRequest {
    /// 新名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 新代号
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 新状态：wait/doing/closed/suspended
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 开始日期
    #[serde(skip_serializing_if = "Option::is_none")]
    pub begin: Option<String>,
    /// 结束日期
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<String>,
    /// 可用工作日
    #[serde(skip_serializing_if = "Option::is_none")]
    pub days: Option<u64>,
    /// 执行描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desc: Option<String>,
    /// 迭代负责人
    #[serde(skip_serializing_if = "Option::is_none")]
    pub PM: Option<String>,
    /// 测试负责人
    #[serde(skip_serializing_if = "Option::is_none")]
    pub QD: Option<String>,
    /// 发布负责人
    #[serde(skip_serializing_if = "Option::is_none")]
    pub RD: Option<String>,
}

// ============================================================
// Execution API - 执行相关 API 调用
// ============================================================

pub struct ExecutionApi;

impl ExecutionApi {
    /// 查询执行列表
    ///
    /// GET /api.php/v1/executions
    ///
    /// # 参数
    /// - client: API 客户端
    /// - project: 按项目 ID 筛选
    ///
    /// # 返回
    /// 满足条件的执行列表
    pub async fn list(client: &ApiClient, project: Option<u64>) -> Result<Vec<Execution>> {
        // 构建查询参数
        // ZenTao API: /projects/{projectId}/executions
        let path = if let Some(pid) = project {
            format!("/api.php/v1/projects/{}/executions", pid)
        } else {
            String::from("/api.php/v1/executions")
        };

        // 响应体结构：{"executions": [...]}
        #[derive(serde::Deserialize)]
        struct ExecutionListResponse {
            #[serde(rename = "executions")]
            executions: Option<Vec<Execution>>,
        }

        let resp: ExecutionListResponse = client.get(&path).await?;
        // 如果 executions 为 None，返回空列表
        Ok(resp.executions.unwrap_or_default())
    }

    /// 获取单个执行详情
    ///
    /// GET /api.php/v1/executions/{id}
    pub async fn get(client: &ApiClient, id: u64) -> Result<Execution> {
        let path = format!("/api.php/v1/executions/{}", id);
        let resp: Execution = client.get(&path).await?;
        Ok(resp)
    }

    /// 创建新执行
    ///
    /// POST /api.php/v1/projects/{projectId}/executions
    ///
    /// ZenTao 创建接口返回 {"id": 123}，需要再调用 get 获取完整信息
    pub async fn create(
        client: &ApiClient,
        project_id: u64,
        req: &CreateExecutionRequest,
    ) -> Result<Execution> {
        #[derive(Deserialize)]
        struct CreateResponse {
            id: Option<u64>,
        }

        let path = format!("/api.php/v1/projects/{}/executions", project_id);
        let resp: CreateResponse = client.post(&path, req).await?;

        // 创建成功后，返回的 id 用于获取完整的执行信息
        if let Some(id) = resp.id {
            Self::get(client, id).await
        } else {
            Err(ZentaoError::Api("Failed to create execution".to_string()).into())
        }
    }

    /// 更新执行
    ///
    /// PUT /api.php/v1/executions/{id}
    ///
    /// ZenTao PUT 接口返回空 JSON {}，需要再调用 get 获取更新后的信息
    pub async fn update(
        client: &ApiClient,
        id: u64,
        req: &UpdateExecutionRequest,
    ) -> Result<Execution> {
        let path = format!("/api.php/v1/executions/{}", id);
        // 发送 PUT 请求，忽略响应体
        let _: serde_json::Value = client.put(&path, req).await?;
        // 获取更新后的完整执行信息
        Self::get(client, id).await
    }

    /// 删除执行
    ///
    /// DELETE /api.php/v1/executions/{id}
    pub async fn delete(client: &ApiClient, id: u64) -> Result<()> {
        let path = format!("/api.php/v1/executions/{}", id);
        #[derive(Deserialize)]
        #[allow(dead_code)]
        struct DeleteResponse {
            result: Option<String>,
        }
        let _: DeleteResponse = client.delete(&path).await?;
        Ok(())
    }
}

// ============================================================
// 单元测试
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;

    // ==================== 反序列化测试 ====================

    /// 测试执行 JSON 反序列化
    #[test]
    fn test_execution_deserialization() {
        let execution_json = r#"{
            "id": 100,
            "name": "Sprint 1",
            "project": 1,
            "status": "doing",
            "type": "iteration",
            "begin": "2024-01-01",
            "end": "2024-01-14",
            "days": 14,
            "desc": "First sprint",
            "opened_by": "admin",
            "opened_date": "2024-01-01 10:00:00"
        }"#;
        let execution: Execution = serde_json::from_str(execution_json).unwrap();
        assert_eq!(execution.id, 100);
        assert_eq!(execution.name, "Sprint 1");
        assert_eq!(execution.project, 1);
        assert_eq!(execution.status, "doing");
        assert_eq!(execution.type_, Some("iteration".to_string()));
        assert_eq!(execution.days, Some(14));
        assert_eq!(execution.desc, Some("First sprint".to_string()));
        assert_eq!(execution.opened_by, Some("admin".to_string()));
    }

    /// 测试最小化执行 JSON 反序列化（只有必填字段）
    #[test]
    fn test_execution_deserialization_minimal() {
        let execution_json = r#"{
            "id": 101,
            "name": "Milestone 1",
            "project": 2,
            "status": "wait"
        }"#;
        let execution: Execution = serde_json::from_str(execution_json).unwrap();
        assert_eq!(execution.id, 101);
        assert_eq!(execution.name, "Milestone 1");
        assert_eq!(execution.project, 2);
        assert_eq!(execution.status, "wait");
        assert!(execution.type_.is_none());
        assert!(execution.begin.is_none());
        assert!(execution.end.is_none());
        assert!(execution.days.is_none());
        assert!(execution.desc.is_none());
        assert!(execution.opened_by.is_none());
        assert!(execution.opened_date.is_none());
    }

    // ==================== 序列化测试 ====================

    /// 测试执行结构序列化
    #[test]
    fn test_execution_serialization() {
        let execution = Execution {
            id: 100,
            name: "Sprint 1".to_string(),
            project: 1,
            status: "doing".to_string(),
            type_: Some("iteration".to_string()),
            begin: Some("2024-01-01".to_string()),
            end: Some("2024-01-14".to_string()),
            days: Some(14),
            desc: Some("First sprint".to_string()),
            opened_by: Some("admin".to_string()),
            opened_date: Some("2024-01-01 10:00:00".to_string()),
        };
        let json = serde_json::to_string(&execution).unwrap();
        assert!(json.contains("Sprint 1"));
        assert!(json.contains("iteration"));
        assert!(json.contains("2024-01-01"));
        assert!(json.contains("14"));
    }

    // ==================== 可选字段跳过测试 ====================

    /// 测试序列化时跳过 None 的可选字段
    #[test]
    fn test_execution_skips_optional_fields() {
        let execution = Execution {
            id: 102,
            name: "Minimal Execution".to_string(),
            project: 3,
            status: "wait".to_string(),
            type_: None,
            begin: None,
            end: None,
            days: None,
            desc: None,
            opened_by: None,
            opened_date: None,
        };
        let json = serde_json::to_string(&execution).unwrap();
        assert!(!json.contains("type"));
        assert!(!json.contains("begin"));
        assert!(!json.contains("end"));
        assert!(!json.contains("days"));
        assert!(!json.contains("desc"));
        assert!(!json.contains("opened_by"));
        assert!(!json.contains("opened_date"));
    }

    /// 测试里程碑类型执行的反序列化
    #[test]
    fn test_milestone_execution_deserialization() {
        let execution_json = r#"{
            "id": 200,
            "name": "V1.0 Release",
            "project": 1,
            "status": "closed",
            "type": "milestone",
            "begin": "2024-03-01",
            "end": "2024-03-15",
            "days": 15,
            "desc": "Version 1.0 release milestone",
            "opened_by": "manager",
            "opened_date": "2024-03-01 09:00:00"
        }"#;
        let execution: Execution = serde_json::from_str(execution_json).unwrap();
        assert_eq!(execution.type_, Some("milestone".to_string()));
        assert_eq!(execution.status, "closed");
        assert_eq!(execution.days, Some(15));
    }

    /// 测试暂停状态执行的反序列化
    #[test]
    fn test_suspended_execution_deserialization() {
        let execution_json = r#"{
            "id": 300,
            "name": "Paused Sprint",
            "project": 2,
            "status": "suspended"
        }"#;
        let execution: Execution = serde_json::from_str(execution_json).unwrap();
        assert_eq!(execution.status, "suspended");
        assert!(execution.type_.is_none());
    }
}
