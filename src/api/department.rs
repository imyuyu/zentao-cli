//! ZenTao 部门(Department) API 模块
//!
//! 提供部门的查询操作

use anyhow::Result;
use serde::Deserialize;

use super::ApiClient;
use crate::api::types::Department;

// ============================================================
// Department API - 部门相关 API 调用
// ============================================================

pub struct DepartmentApi;

impl DepartmentApi {
    /// 查询部门列表
    ///
    /// GET /api.php/v1/departments
    ///
    /// # 参数
    /// - client: API 客户端
    ///
    /// # 返回
    /// 部门列表
    pub async fn list(client: &ApiClient) -> Result<Vec<Department>> {
        let path = String::from("/api.php/v1/departments");

        #[derive(Deserialize)]
        #[serde(untagged)]
        enum DepartmentResponse {
            List(Vec<Department>),
            Wrapped { departments: Vec<Department> },
            Map(serde_json::Map<String, serde_json::Value>),
        }

        let resp: DepartmentResponse = client.get(&path).await?;
        match resp {
            DepartmentResponse::List(depts) => Ok(depts),
            DepartmentResponse::Wrapped { departments } => Ok(departments),
            DepartmentResponse::Map(_) => Ok(Vec::new()),
        }
    }

    /// 获取单个部门详情
    ///
    /// GET /api.php/v1/departments/{id}
    pub async fn get(client: &ApiClient, id: u64) -> Result<Department> {
        let path = format!("/api.php/v1/departments/{}", id);
        let resp: Department = client.get(&path).await?;
        Ok(resp)
    }
}

// ============================================================
// 单元测试
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_department_deserialization() {
        let dept_json = r#"{
            "id": 1,
            "name": "Engineering",
            "parent": 0,
            "order": 1,
            "path": "/1/"
        }"#;
        let dept: Department = serde_json::from_str(dept_json).unwrap();
        assert_eq!(dept.id, 1);
        assert_eq!(dept.name, "Engineering");
        assert_eq!(dept.parent, Some(0));
        assert_eq!(dept.order, Some(1));
        assert_eq!(dept.path, Some("/1/".to_string()));
    }

    #[test]
    fn test_department_deserialization_minimal() {
        let dept_json = r#"{
            "id": 2,
            "name": "Sales"
        }"#;
        let dept: Department = serde_json::from_str(dept_json).unwrap();
        assert_eq!(dept.id, 2);
        assert_eq!(dept.name, "Sales");
        assert_eq!(dept.parent, None);
        assert_eq!(dept.order, None);
        assert_eq!(dept.path, None);
    }
}
