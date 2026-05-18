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
    /// 所属产品 ID（必填）
    pub product: u64,
    /// 用例标题（必填）
    pub title: String,
    /// 用例类型：feature/performance/config/install/security/interface/unit/other（必填）
    #[serde(rename = "type")]
    pub type_: String,
    /// 所属分支
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch: Option<u64>,
    /// 所属模块
    #[serde(skip_serializing_if = "Option::is_none")]
    pub module: Option<u64>,
    /// 关联的需求 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub story: Option<u64>,
    /// 适用阶段：unitest/feature/intergrate/system/smoke/bvt
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage: Option<String>,
    /// 前置条件
    #[serde(skip_serializing_if = "Option::is_none")]
    pub precondition: Option<String>,
    /// 优先级：0-5
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pri: Option<u8>,
    /// 测试步骤（array of {desc, expect}）
    pub steps: Vec<TestcaseStep>,
    /// 关键词
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keywords: Option<String>,
}

/// 测试用例步骤
#[derive(Debug, Serialize)]
pub struct TestcaseStep {
    /// 步骤描述
    pub desc: String,
    /// 期望结果
    pub expect: String,
}

/// 更新测试用例的请求体
/// 所有字段可选，只更新传入的字段
#[derive(Debug, Serialize)]
pub struct UpdateTestcaseRequest {
    /// 所属分支
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch: Option<u64>,
    /// 所属模块
    #[serde(skip_serializing_if = "Option::is_none")]
    pub module: Option<u64>,
    /// 关联的需求 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub story: Option<u64>,
    /// 新标题
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// 新用例类型
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    /// 适用阶段
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage: Option<String>,
    /// 前置条件
    #[serde(skip_serializing_if = "Option::is_none")]
    pub precondition: Option<String>,
    /// 新优先级
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pri: Option<u8>,
    /// 测试步骤
    #[serde(skip_serializing_if = "Option::is_none")]
    pub steps: Option<Vec<TestcaseStep>>,
    /// 关键词
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keywords: Option<String>,
}

/// 执行测试用例的请求体
#[derive(Debug, Serialize)]
pub struct TestcaseResultRequest {
    /// 测试单 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub testtask: Option<u64>,
    /// 测试用例版本
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<u64>,
    /// 用例结果和描述（array）
    pub steps: Vec<TestcaseResultStep>,
}

/// 测试用例执行结果步骤
#[derive(Debug, Serialize)]
pub struct TestcaseResultStep {
    /// 结果：n/a/fail/blocked/pass
    pub result: String,
    /// 步骤描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desc: Option<String>,
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
            "product": 1,
            "branch": 0,
            "module": 0,
            "story": 0,
            "story_version": 1,
            "precondition": "Precondition text",
            "stage": "unitest",
            "opened_by": "admin",
            "opened_date": "2024-01-01",
            "version": 2,
            "steps": [
                {"id": 1, "desc": "Step 1: Do something", "expect": "Expected result"}
            ]
        }"#;
        let testcase: Testcase = serde_json::from_str(json).unwrap();
        assert_eq!(testcase.id, 100);
        assert_eq!(testcase.title, "Test Case Title");
        assert_eq!(testcase.type_, "feature");
        assert_eq!(testcase.pri, 2);
        assert_eq!(testcase.status, "normal");
        assert_eq!(testcase.product, 1);
        assert_eq!(testcase.opened_by, "admin");
        assert_eq!(testcase.version, 2);
        assert_eq!(testcase.steps.len(), 1);
        assert_eq!(testcase.steps[0].desc, "Step 1: Do something");
        assert_eq!(testcase.steps[0].expect, "Expected result");
    }

    /// 测试最小 Testcase 反序列化（只有必填字段）
    #[test]
    fn test_testcase_deserialization_minimal() {
        let json = r#"{
            "id": 101,
            "title": "Minimal Case",
            "type": "feature",
            "severity": 2,
            "pri": 1,
            "status": "wait",
            "product": 2,
            "branch": 0,
            "module": 0,
            "story": 0,
            "story_version": 1,
            "precondition": "",
            "stage": "unitest",
            "opened_by": "admin",
            "opened_date": "2024-01-01",
            "version": 1,
            "steps": []
        }"#;
        let testcase: Testcase = serde_json::from_str(json).unwrap();
        assert_eq!(testcase.id, 101);
        assert_eq!(testcase.title, "Minimal Case");
        assert_eq!(testcase.pri, 1);
        assert_eq!(testcase.status, "wait");
        assert_eq!(testcase.product, 2);
        assert_eq!(testcase.steps.len(), 0);
    }

    /// 测试不同状态的 Testcase 反序列化
    #[test]
    fn test_testcase_deserialization_different_statuses() {
        let blocked_json = r#"{
            "id": 102,
            "title": "Blocked Case",
            "type": "feature",
            "severity": 1,
            "pri": 0,
            "status": "blocked",
            "product": 1,
            "branch": 0,
            "module": 0,
            "story": 0,
            "story_version": 1,
            "precondition": "",
            "stage": "unitest",
            "opened_by": "admin",
            "opened_date": "2024-01-01",
            "version": 1,
            "steps": []
        }"#;
        let blocked: Testcase = serde_json::from_str(blocked_json).unwrap();
        assert_eq!(blocked.status, "blocked");

        let bypass_json = r#"{
            "id": 103,
            "title": "Bypass Case",
            "type": "feature",
            "severity": 4,
            "pri": 3,
            "status": "bypass",
            "product": 1,
            "branch": 0,
            "module": 0,
            "story": 0,
            "story_version": 1,
            "precondition": "",
            "stage": "unitest",
            "opened_by": "admin",
            "opened_date": "2024-01-01",
            "version": 1,
            "steps": []
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
