//! ZenTao 用户(User) API 模块
//!
//! 提供用户的查询和操作

use anyhow::Result;
use serde::{Deserialize, Serialize};

use super::ApiClient;
use crate::api::types::User;

// ============================================================
/// 用户创建请求
#[derive(Debug, Clone, Serialize)]
pub struct CreateUserRequest {
    pub account: String,
    pub password: String,
    pub realname: String,
    pub role: Option<String>,
    pub dept: Option<u64>,
    pub mobile: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
}

/// 用户更新请求
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateUserRequest {
    pub dept: Option<u64>,
    pub role: Option<String>,
    pub mobile: Option<String>,
    pub realname: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
}

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

    /// 获取当前登录用户信息
    ///
    /// GET /api.php/v1/user
    pub async fn me(client: &ApiClient) -> Result<User> {
        let path = "/api.php/v1/user".to_string();
        let resp: User = client.get(&path).await?;
        Ok(resp)
    }

    /// 创建用户
    ///
    /// POST /api.php/v1/users
    pub async fn create(client: &ApiClient, req: &CreateUserRequest) -> Result<User> {
        let path = "/api.php/v1/users".to_string();
        let resp: User = client.post(&path, req).await?;
        Ok(resp)
    }

    /// 更新用户信息
    ///
    /// PUT /api.php/v1/users/{user_id}
    pub async fn update(client: &ApiClient, user_id: u64, req: &UpdateUserRequest) -> Result<User> {
        let path = format!("/api.php/v1/users/{}", user_id);
        let resp: User = client.put(&path, req).await?;
        Ok(resp)
    }

    /// 删除用户
    ///
    /// DELETE /api.php/v1/users/{user_id}
    pub async fn delete(client: &ApiClient, user_id: u64) -> Result<()> {
        let path = format!("/api.php/v1/users/{}", user_id);
        client.delete::<()>(&path).await?;
        Ok(())
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
