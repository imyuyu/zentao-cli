//! ZenTao Task(任务) API 模块
//!
//! 提供任务的增删改查操作
//!
//! # 与 Story（需求）的区别
//! - Story（需求）：用户视角的功能需求，关注"做什么"
//! - Task（任务）：开发视角的具体工作，关注"怎么做"

use anyhow::Result;
use serde::{Deserialize, Serialize};

use super::ApiClient;

// ============================================================
// 请求结构体
// ============================================================

/// 任务数据结构
///
/// 对应 ZenTao 系统的任务实体
///
/// # 与 Story 的主要区别
/// - Task 有 estimate（预估工时）、consumed（已消耗）、left（剩余）工时字段
/// - Task 有 assigned_to（指派给谁）字段
/// - Task 有 project（所属项目）字段，没有 product 字段
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    /// 任务 ID
    pub id: u64,
    /// 任务名称
    pub name: String,
    /// 所属项目 ID
    pub project: u64,
    /// 任务状态：todo（待开始）/in progress（进行中）/done（已完成）
    pub status: String,
    /// 优先级：1-5（1 最高）
    pub pri: u8,
    /// 指派给谁（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assigned_to: Option<String>,
    /// 预估工时（小时，可选）
    /// 相当于其他语言的 `Double` 或 `floating`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimate: Option<f64>,
    /// 已消耗工时（小时，可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumed: Option<f64>,
    /// 剩余工时（小时，可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub left: Option<f64>,
}

/// 创建任务的请求体
///
/// 所有字段对应 ZenTao 任务创建接口
#[derive(Debug, Serialize)]
pub struct CreateTaskRequest {
    /// 任务名称（必填）
    pub name: String,
    /// 所属项目 ID（必填）
    pub project: u64,
    /// 优先级：1-5（必填）
    pub pri: u8,
    /// 任务类型：development（开发）/design（设计）/test（测试）/study（研究）/discuss（讨论）/ui（界面）/other（其他）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    /// 指派给谁（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assigned_to: Option<String>,
    /// 预估工时，小时（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimate: Option<f64>,
}

/// 更新任务的请求体
///
/// 所有字段可选，只更新传入的字段
#[derive(Debug, Serialize)]
pub struct UpdateTaskRequest {
    /// 新任务名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 新状态：todo/in progress/done/closed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 新优先级
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pri: Option<u8>,
    /// 新的指派人
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assigned_to: Option<String>,
}

/// 任务工时日志结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskEstimate {
    /// 日志 ID
    pub id: u64,
    /// 任务 ID
    pub task: u64,
    /// 消耗工时
    pub consumed: f64,
    /// 剩余工时
    pub left: f64,
    /// 记录日期
    pub date: String,
    /// 备注
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

// ============================================================
// Task API
// ============================================================

/// 任务 API 操作类
pub struct TaskApi;

impl TaskApi {
    /// 查询任务列表
    ///
    /// GET /api.php/v1/tasks?projectID={project}
    ///
    /// # 参数
    /// - `client`: API 客户端
    /// - `project`: 项目 ID（必填）
    /// - `assigned_to`: 按指派人筛选（可选）
    ///
    /// # 返回值
    /// 返回指定项目下的任务列表
    pub async fn list(
        client: &ApiClient,
        project: u64,
        assigned_to: Option<String>,
    ) -> Result<Vec<Task>> {
        // 先获取执行列表，再获取每个执行的任务
        // ZenTao API: /projects/{projectId}/executions 获取执行列表
        // 然后: /executions/{executionId}/tasks 获取每个执行的任务
        #[derive(Deserialize)]
        struct ExecutionListResponse {
            #[serde(rename = "executions")]
            executions: Option<Vec<crate::api::types::Execution>>,
        }

        #[derive(Deserialize)]
        struct TaskListResponse {
            #[serde(rename = "tasks")]
            tasks: Option<Vec<Task>>,
        }

        // 获取项目的执行列表
        let exec_path = format!("/api.php/v1/projects/{}/executions", project);
        let exec_resp: ExecutionListResponse = client.get(&exec_path).await?;
        let executions = exec_resp.executions.unwrap_or_default();

        // 获取所有执行的任务
        let mut all_tasks = Vec::new();
        for exec in executions {
            let task_path = if let Some(ref u) = assigned_to {
                format!("/api.php/v1/executions/{}/tasks?assignedTo={}", exec.id, u)
            } else {
                format!("/api.php/v1/executions/{}/tasks", exec.id)
            };

            if let Ok(task_resp) = client.get::<TaskListResponse>(&task_path).await {
                if let Some(tasks) = task_resp.tasks {
                    all_tasks.extend(tasks);
                }
            }
        }

        Ok(all_tasks)
    }

    /// 获取单个任务详情
    ///
    /// GET /api.php/v1/tasks/{id}
    pub async fn get(client: &ApiClient, id: u64) -> Result<Task> {
        let path = format!("/api.php/v1/tasks/{}", id);
        let resp: Task = client.get(&path).await?;
        Ok(resp)
    }

    /// 创建新任务
    ///
    /// POST /api.php/v1/tasks
    ///
    /// ZenTao 创建接口返回 {"id": 123}，需要再调用 get 获取完整信息
    pub async fn create(client: &ApiClient, req: &CreateTaskRequest) -> Result<Task> {
        #[derive(Deserialize)]
        struct CreateResponse {
            id: Option<u64>,
        }

        let resp: CreateResponse = client.post("/api.php/v1/tasks", req).await?;

        if let Some(id) = resp.id {
            // 创建成功后，调用 get 获取完整的任务信息
            Self::get(client, id).await
        } else {
            // 创建失败，返回错误
            // 使用 anyhow::bail! 而不是返回 Err，因为这是业务逻辑错误
            anyhow::bail!("Failed to create task")
        }
    }

    /// 更新任务
    ///
    /// PUT /api.php/v1/tasks/{id}
    pub async fn update(client: &ApiClient, id: u64, req: &UpdateTaskRequest) -> Result<Task> {
        let path = format!("/api.php/v1/tasks/{}", id);
        // 更新接口返回的是空 JSON 或任务对象，用 Value 忽略具体格式
        let _: serde_json::Value = client.put(&path, req).await?;
        // 更新后调用 get 获取最新数据
        Self::get(client, id).await
    }

    /// 删除任务
    ///
    /// DELETE /api.php/v1/tasks/{id}
    pub async fn delete(client: &ApiClient, id: u64) -> Result<()> {
        let path = format!("/api.php/v1/tasks/{}", id);
        let _: serde_json::Value = client.delete(&path).await?;
        Ok(())
    }

    /// 开始任务
    ///
    /// POST /api.php/v1/tasks/{id}/start
    pub async fn start(client: &ApiClient, id: u64) -> Result<Task> {
        let path = format!("/api.php/v1/tasks/{}/start", id);
        let _: serde_json::Value = client.post(&path, &()).await?;
        Self::get(client, id).await
    }

    /// 暂停任务
    ///
    /// POST /api.php/v1/tasks/{id}/pause
    pub async fn pause(client: &ApiClient, id: u64) -> Result<Task> {
        let path = format!("/api.php/v1/tasks/{}/pause", id);
        let _: serde_json::Value = client.post(&path, &()).await?;
        Self::get(client, id).await
    }

    /// 继续任务
    ///
    /// POST /api.php/v1/tasks/{id}/restart
    pub async fn restart(client: &ApiClient, id: u64) -> Result<Task> {
        let path = format!("/api.php/v1/tasks/{}/restart", id);
        let _: serde_json::Value = client.post(&path, &()).await?;
        Self::get(client, id).await
    }

    /// 完成任务
    ///
    /// POST /api.php/v1/tasks/{id}/finish
    pub async fn finish(client: &ApiClient, id: u64) -> Result<Task> {
        let path = format!("/api.php/v1/tasks/{}/finish", id);
        let _: serde_json::Value = client.post(&path, &()).await?;
        Self::get(client, id).await
    }

    /// 关闭任务
    ///
    /// POST /api.php/v1/tasks/{id}/close
    pub async fn close(client: &ApiClient, id: u64) -> Result<Task> {
        let path = format!("/api.php/v1/tasks/{}/close", id);
        let _: serde_json::Value = client.post(&path, &()).await?;
        Self::get(client, id).await
    }

    /// 添加任务日志（工时）
    ///
    /// POST /api.php/v1/tasks/{id}/estimate
    pub async fn add_estimate(
        client: &ApiClient,
        id: u64,
        consumed: f64,
        left: f64,
        notes: Option<String>,
    ) -> Result<TaskEstimate> {
        let path = format!("/api.php/v1/tasks/{}/estimate", id);
        #[derive(Serialize)]
        struct EstimateRequest {
            consumed: f64,
            left: f64,
            #[serde(skip_serializing_if = "Option::is_none")]
            notes: Option<String>,
        }
        let req = EstimateRequest {
            consumed,
            left,
            notes,
        };
        let estimate: TaskEstimate = client.post(&path, &req).await?;
        Ok(estimate)
    }

    /// 获取任务日志列表
    ///
    /// GET /api.php/v1/tasks/{id}/estimate
    pub async fn get_estimates(client: &ApiClient, id: u64) -> Result<Vec<TaskEstimate>> {
        let path = format!("/api.php/v1/tasks/{}/estimate", id);
        #[derive(Deserialize)]
        struct EstimateListResponse {
            estimates: Option<Vec<TaskEstimate>>,
        }
        let resp: EstimateListResponse = client.get(&path).await?;
        Ok(resp.estimates.unwrap_or_default())
    }
}

// ============================================================
// 单元测试
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;

    // ==================== CreateTaskRequest 测试 ====================

    /// 测试完整创建请求序列化
    #[test]
    fn test_create_task_request_serialization() {
        let req = CreateTaskRequest {
            name: "Test Task".to_string(),
            project: 1,
            pri: 3,
            type_: Some("development".to_string()),
            assigned_to: Some("dev".to_string()),
            estimate: Some(8.0),
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("Test Task"));
        assert!(json.contains("\"project\":1"));
        assert!(json.contains("\"pri\":3"));
        assert!(json.contains("development"));
        assert!(json.contains("dev"));
    }

    /// 测试可选字段被正确跳过
    #[test]
    fn test_create_task_request_skips_optional_fields() {
        let req = CreateTaskRequest {
            name: "Minimal Task".to_string(),
            project: 2,
            pri: 1,
            type_: None,
            assigned_to: None,
            estimate: None,
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("Minimal Task"));
        // 可选字段不应该出现在 JSON 中
        assert!(!json.contains("type_"));
        assert!(!json.contains("assigned_to"));
        assert!(!json.contains("estimate"));
    }

    // ==================== UpdateTaskRequest 测试 ====================

    /// 测试完整更新请求序列化
    #[test]
    fn test_update_task_request_serialization() {
        let req = UpdateTaskRequest {
            name: Some("Updated Task".to_string()),
            status: Some("done".to_string()),
            pri: Some(5),
            assigned_to: Some("admin".to_string()),
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("Updated Task"));
        assert!(json.contains("done"));
        assert!(json.contains("\"pri\":5"));
    }

    /// 测试部分更新（只有部分字段有值）
    #[test]
    fn test_update_task_request_partial() {
        let req = UpdateTaskRequest {
            name: None,
            status: Some("in progress".to_string()),
            pri: None,
            assigned_to: None,
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("in progress"));
        // None 的字段不应该出现在 JSON 中
        assert!(!json.contains("\"name\""));
        assert!(!json.contains("\"pri\""));
    }

    // ==================== Task 反序列化测试 ====================

    /// 测试基本任务反序列化
    #[test]
    fn test_task_deserialization() {
        let task_json = r#"{
            "id": 100,
            "name": "Task Title",
            "project": 5,
            "status": "in progress",
            "pri": 3
        }"#;
        let task: Task = serde_json::from_str(task_json).unwrap();
        assert_eq!(task.id, 100);
        assert_eq!(task.name, "Task Title");
        assert_eq!(task.project, 5);
        assert_eq!(task.status, "in progress");
        assert_eq!(task.pri, 3);
    }

    /// 测试带所有可选字段的任务反序列化
    #[test]
    fn test_task_deserialization_with_optional_fields() {
        let task_json = r#"{
            "id": 101,
            "name": "Full Task",
            "project": 6,
            "status": "done",
            "pri": 4,
            "assigned_to": "developer",
            "estimate": 13.5,
            "consumed": 10.0,
            "left": 0.0
        }"#;
        let task: Task = serde_json::from_str(task_json).unwrap();
        assert_eq!(task.id, 101);
        assert_eq!(task.assigned_to, Some("developer".to_string()));
        assert_eq!(task.estimate, Some(13.5));
        assert_eq!(task.consumed, Some(10.0));
        assert_eq!(task.left, Some(0.0));
    }

    /// 测试序列化时可选字段被正确跳过
    #[test]
    fn test_task_skips_optional_fields_when_none() {
        let task = Task {
            id: 1,
            name: "Basic Task".to_string(),
            project: 1,
            status: "open".to_string(),
            pri: 2,
            assigned_to: None,
            estimate: None,
            consumed: None,
            left: None,
        };
        let json = serde_json::to_string(&task).unwrap();
        assert!(!json.contains("assigned_to"));
        assert!(!json.contains("estimate"));
        assert!(!json.contains("consumed"));
    }
}
