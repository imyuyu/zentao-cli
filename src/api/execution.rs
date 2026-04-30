//! ZenTao 执行(Execution) API 模块
//!
//! 提供执行的增删改查操作

use anyhow::Result;

use super::ApiClient;
use crate::api::types::Execution;

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
