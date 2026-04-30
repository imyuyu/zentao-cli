//! ZenTao 测试用例(Testcase) API 模块
//!
//! 提供测试用例的增删改查操作

use anyhow::Result;
use serde::{Deserialize, Serialize};

use super::ApiClient;
use crate::api::types::Testcase;
use crate::core::ZentaoError;

// ============================================================
// 请求结构体
// ============================================================

/// 创建测试用例的请求体
#[derive(Debug, Serialize)]
pub struct CreateTestcaseRequest {
    /// 用例标题（必填）
    pub title: String,
    /// 所属产品 ID（必填）
    pub product: u64,
    /// 用例类型：feature/performance/interface/security/concurrency/destructive/install/others
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    /// 严重程度：1-4（1 最严重）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity: Option<u8>,
    /// 优先级：0-5
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pri: Option<u8>,
    /// 测试步骤
    #[serde(skip_serializing_if = "Option::is_none")]
    pub steps: Option<String>,
    /// 期望结果
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expectation: Option<String>,
    /// 关联的需求 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub story: Option<u64>,
    /// 所属项目 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project: Option<u64>,
}

/// 更新测试用例的请求体
/// 所有字段可选，只更新传入的字段
#[derive(Debug, Serialize)]
pub struct UpdateTestcaseRequest {
    /// 新标题
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// 新状态：wait/normal/blocked/bypass
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 新优先级
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pri: Option<u8>,
    /// 新严重程度
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity: Option<u8>,
    /// 新用例类型
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    /// 新测试步骤
    #[serde(skip_serializing_if = "Option::is_none")]
    pub steps: Option<String>,
    /// 新期望结果
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expectation: Option<String>,
}

/// 执行测试用例的请求体
#[derive(Debug, Serialize)]
pub struct TestcaseResultRequest {
    /// 执行结果：pass/fail/blocked
    pub result: String,
    /// 执行耗时（分钟）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumed: Option<f64>,
    /// 执行备注
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remark: Option<String>,
    /// 关联的版本 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build: Option<u64>,
}

// ============================================================
// Testcase API - 测试用例相关 API 调用
// ============================================================

pub struct TestcaseApi;

impl TestcaseApi {
    /// 查询测试用例列表
    ///
    /// GET /api.php/v1/testcases
    ///
    /// # 参数
    /// - client: API 客户端
    /// - product: 按产品 ID 筛选
    /// - project: 按项目 ID 筛选
    /// - type_: 按用例类型筛选
    /// - status: 按状态筛选
    ///
    /// # 返回
    /// 满足条件的测试用例列表
    pub async fn list(
        client: &ApiClient,
        product: Option<u64>,
        _project: Option<u64>,
        type_: Option<String>,
        status: Option<String>,
    ) -> Result<Vec<Testcase>> {
        // ZenTao API: /products/{productId}/testcases
        let path = if let Some(pid) = product {
            let mut p = format!("/api.php/v1/products/{}/testcases", pid);
            if let Some(ref t) = type_ {
                p.push_str(&format!("&type={}", t));
            }
            if let Some(ref s) = status {
                p.push_str(&format!("&status={}", s));
            }
            p
        } else {
            let mut p = String::from("/api.php/v1/testcases");
            if let Some(ref t) = type_ {
                p.push_str(&format!("?type={}", t));
            }
            if let Some(ref s) = status {
                if p.contains('?') {
                    p.push_str(&format!("&status={}", s));
                } else {
                    p.push_str(&format!("?status={}", s));
                }
            }
            p
        };

        #[derive(Deserialize)]
        struct TestcaseListResponse {
            #[serde(rename = "testcases")]
            testcases: Option<Vec<Testcase>>,
        }

        let resp: TestcaseListResponse = client.get(&path).await?;
        Ok(resp.testcases.unwrap_or_default())
    }

    /// 获取单个测试用例详情
    ///
    /// GET /api.php/v1/testcases/{id}
    pub async fn get(client: &ApiClient, id: u64) -> Result<Testcase> {
        let path = format!("/api.php/v1/testcases/{}", id);
        let resp: Testcase = client.get(&path).await?;
        Ok(resp)
    }

    /// 创建新测试用例
    ///
    /// POST /api.php/v1/products/{productId}/testcases
    ///
    /// ZenTao 创建接口返回 {"id": 123}，需要再调用 get 获取完整信息
    pub async fn create(
        client: &ApiClient,
        product_id: u64,
        req: &CreateTestcaseRequest,
    ) -> Result<Testcase> {
        #[derive(Deserialize)]
        struct CreateResponse {
            id: Option<u64>,
        }

        let path = format!("/api.php/v1/products/{}/testcases", product_id);
        let resp: CreateResponse = client.post(&path, req).await?;

        if let Some(id) = resp.id {
            Self::get(client, id).await
        } else {
            Err(ZentaoError::Api("Failed to create testcase".to_string()).into())
        }
    }

    /// 更新测试用例
    ///
    /// PUT /api.php/v1/testcases/{id}
    pub async fn update(
        client: &ApiClient,
        id: u64,
        req: &UpdateTestcaseRequest,
    ) -> Result<Testcase> {
        let path = format!("/api.php/v1/testcases/{}", id);
        let _: serde_json::Value = client.put(&path, req).await?;
        Self::get(client, id).await
    }

    /// 删除测试用例
    ///
    /// DELETE /api.php/v1/testcases/{id}
    pub async fn delete(client: &ApiClient, id: u64) -> Result<()> {
        let path = format!("/api.php/v1/testcases/{}", id);
        let _: serde_json::Value = client.delete(&path).await?;
        Ok(())
    }

    /// 执行测试用例
    ///
    /// POST /api.php/v1/testcases/{id}/results
    pub async fn create_result(
        client: &ApiClient,
        id: u64,
        req: &TestcaseResultRequest,
    ) -> Result<Testcase> {
        let path = format!("/api.php/v1/testcases/{}/results", id);
        let _: serde_json::Value = client.post(&path, req).await?;
        Self::get(client, id).await
    }
}

// ============================================================
// 单元测试
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;

    // ==================== Testcase 反序列化测试 ====================

    /// 测试完整 Testcase 反序列化
    #[test]
    fn test_testcase_deserialization() {
        let json = r#"{
            "id": 100,
            "title": "Test Case Title",
            "type": "feature",
            "severity": 3,
            "pri": 2,
            "status": "normal",
            "steps": "Step 1: Do something\nStep 2: Verify result",
            "expectation": "Expected result",
            "product": 1,
            "project": 5,
            "opened_by": "admin",
            "version": 2
        }"#;
        let testcase: Testcase = serde_json::from_str(json).unwrap();
        assert_eq!(testcase.id, 100);
        assert_eq!(testcase.title, "Test Case Title");
        assert_eq!(testcase.type_, Some("feature".to_string()));
        assert_eq!(testcase.severity, 3);
        assert_eq!(testcase.pri, 2);
        assert_eq!(testcase.status, "normal");
        assert_eq!(
            testcase.steps,
            Some("Step 1: Do something\nStep 2: Verify result".to_string())
        );
        assert_eq!(testcase.expectation, Some("Expected result".to_string()));
        assert_eq!(testcase.product, 1);
        assert_eq!(testcase.project, Some(5));
        assert_eq!(testcase.opened_by, Some("admin".to_string()));
        assert_eq!(testcase.version, Some(2));
    }

    /// 测试最小 Testcase 反序列化（只有必填字段）
    #[test]
    fn test_testcase_deserialization_minimal() {
        let json = r#"{
            "id": 101,
            "title": "Minimal Case",
            "severity": 2,
            "pri": 1,
            "status": "wait",
            "product": 2
        }"#;
        let testcase: Testcase = serde_json::from_str(json).unwrap();
        assert_eq!(testcase.id, 101);
        assert_eq!(testcase.title, "Minimal Case");
        assert_eq!(testcase.severity, 2);
        assert_eq!(testcase.pri, 1);
        assert_eq!(testcase.status, "wait");
        assert_eq!(testcase.product, 2);
        // 可选字段
        assert_eq!(testcase.type_, None);
        assert_eq!(testcase.steps, None);
        assert_eq!(testcase.expectation, None);
        assert_eq!(testcase.project, None);
        assert_eq!(testcase.opened_by, None);
        assert_eq!(testcase.version, None);
    }

    /// 测试不同状态的 Testcase 反序列化
    #[test]
    fn test_testcase_deserialization_different_statuses() {
        let blocked_json = r#"{
            "id": 102,
            "title": "Blocked Case",
            "severity": 1,
            "pri": 0,
            "status": "blocked",
            "product": 1
        }"#;
        let blocked: Testcase = serde_json::from_str(blocked_json).unwrap();
        assert_eq!(blocked.status, "blocked");

        let bypass_json = r#"{
            "id": 103,
            "title": "Bypass Case",
            "severity": 4,
            "pri": 3,
            "status": "bypass",
            "product": 1
        }"#;
        let bypass: Testcase = serde_json::from_str(bypass_json).unwrap();
        assert_eq!(bypass.status, "bypass");
    }

    // ==================== TestcaseListQuery 测试 ====================

    /// 测试完整的查询参数序列化
    #[test]
    fn test_testcase_list_query_serialization() {
        use crate::api::types::TestcaseListQuery;

        let query = TestcaseListQuery {
            product: Some(1),
            project: Some(5),
            type_: Some("feature".to_string()),
            status: Some("normal".to_string()),
            severity: Some(3),
        };
        let json = serde_json::to_string(&query).unwrap();
        assert!(json.contains("\"product\":1"));
        assert!(json.contains("\"project\":5"));
        assert!(json.contains("feature"));
        assert!(json.contains("normal"));
        assert!(json.contains("\"severity\":3"));
    }

    /// 测试查询参数包含必填字段
    #[test]
    fn test_testcase_list_query_contains_required_field() {
        use crate::api::types::TestcaseListQuery;

        let query = TestcaseListQuery {
            product: Some(1),
            project: None,
            type_: None,
            status: None,
            severity: None,
        };
        let json = serde_json::to_string(&query).unwrap();
        // Only product field is required and has a value
        assert!(json.contains("\"product\":1"));
    }

    /// 测试不同类型筛选的查询参数
    #[test]
    fn test_testcase_list_query_different_types() {
        use crate::api::types::TestcaseListQuery;

        let types = vec![
            "feature",
            "performance",
            "interface",
            "security",
            "concurrency",
            "destructive",
            "install",
            "others",
        ];

        for type_str in types {
            let query = TestcaseListQuery {
                product: Some(1),
                project: None,
                type_: Some(type_str.to_string()),
                status: None,
                severity: None,
            };
            let json = serde_json::to_string(&query).unwrap();
            assert!(
                json.contains(type_str),
                "Type {} should be in JSON",
                type_str
            );
        }
    }
}
