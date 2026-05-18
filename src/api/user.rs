//! ZenTao 用户(User) API 模块
//!
//! 提供用户的查询操作

use anyhow::Result;
use serde::Deserialize;

use super::ApiClient;
use crate::api::types::User;

// ============================================================
// User API - 用户相关 API 调用
// ============================================================

pub struct UserApi;

impl UserApi {
    /// 查询用户列表
    ///
    /// GET /api.php/v1/users
    ///
    /// # 参数
    /// - client: API 客户端
    /// - dept: 按部门 ID 筛选
    /// - role: 按角色筛选
    ///
    /// # 返回
    /// 满足条件的用户列表
    pub async fn list(
        client: &ApiClient,
        dept: Option<u64>,
        role: Option<String>,
    ) -> Result<Vec<User>> {
        let mut all_users = Vec::new();
        let mut page = 1;
        let limit = 100;

        loop {
            let mut path = format!("/api.php/v1/users?page={}&limit={}", page, limit);

            if let Some(d) = dept {
                path.push_str(&format!("&dept={}", d));
            }
            if let Some(ref r) = role {
                path.push_str(&format!("&role={}", r));
            }

            #[derive(Deserialize)]
            struct UserListResponse {
                #[serde(rename = "users")]
                users: Option<Vec<User>>,
            }

            let resp: UserListResponse = client.get(&path).await?;
            let users = resp.users.unwrap_or_default();
            let count = users.len();

            if users.is_empty() {
                break;
            }
            all_users.extend(users);

            if count < limit as usize {
                break;
            }
            page += 1;
        }

        Ok(all_users)
    }

    /// 获取单个用户详情
    ///
    /// GET /api.php/v1/users/{id}
    pub async fn get(client: &ApiClient, id: u64) -> Result<User> {
        let path = format!("/api.php/v1/users/{}", id);
        let resp: User = client.get(&path).await?;
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
    fn test_user_deserialization() {
        let user_json = r#"{
            "id": 1,
            "account": "admin",
            "realname": "Administrator",
            "email": "admin@example.com",
            "dept": 1,
            "role": "dev"
        }"#;
        let user: User = serde_json::from_str(user_json).unwrap();
        assert_eq!(user.id, 1);
        assert_eq!(user.account, "admin");
        assert_eq!(user.realname, "Administrator");
        assert_eq!(user.email, Some("admin@example.com".to_string()));
        assert_eq!(user.dept, Some(1));
        assert_eq!(user.role, Some("dev".to_string()));
    }

    #[test]
    fn test_user_deserialization_minimal() {
        let user_json = r#"{
            "id": 2,
            "account": "user1",
            "realname": "User One"
        }"#;
        let user: User = serde_json::from_str(user_json).unwrap();
        assert_eq!(user.id, 2);
        assert_eq!(user.account, "user1");
        assert_eq!(user.realname, "User One");
        assert_eq!(user.email, None);
        assert_eq!(user.dept, None);
        assert_eq!(user.role, None);
    }
}
