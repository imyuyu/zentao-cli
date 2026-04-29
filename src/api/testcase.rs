//! ZenTao 测试用例(Testcase) API 模块
//!
//! 提供测试用例的查询操作

use anyhow::Result;
use serde::Deserialize;

use super::ApiClient;
use crate::api::types::Testcase;

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
        project: Option<u64>,
        type_: Option<String>,
        status: Option<String>,
    ) -> Result<Vec<Testcase>> {
        let mut path = String::from("/api.php/v1/testcases?");

        if let Some(pid) = product {
            path.push_str(&format!("productID={}", pid));
        }
        if let Some(pid) = project {
            if path.contains('=') {
                path.push('&');
            }
            path.push_str(&format!("projectID={}", pid));
        }
        if let Some(t) = type_ {
            if path.contains('=') {
                path.push('&');
            }
            path.push_str(&format!("type={}", t));
        }
        if let Some(s) = status {
            if path.contains('=') {
                path.push('&');
            }
            path.push_str(&format!("status={}", s));
        }

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
