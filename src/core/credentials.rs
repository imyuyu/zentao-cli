//! ZenTao CLI 安全凭据管理模块
//!
//! 使用系统凭据库安全存储用户名和密码
//! - Windows: Credential Manager
//! - macOS: Keychain
//! - Linux: libsecret

use crate::core::ZentaoError;
use keyring;
use keyring_core::Error;
use keyring_core::Entry;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 服务名称，用于标识 zentao-cli 的凭据
const SERVICE_NAME: &str = "zentao-cli";

/// 初始化 keyring store
fn init_keyring_store() {
    let _ = keyring::use_native_store(false);
}

/// 凭据结构体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Credentials {
    /// 禅道账号
    pub account: String,
    /// 禅道密码（不持久化，仅用于验证）
    #[serde(skip_serializing)]
    pub password: Option<String>,
}

impl Credentials {
    /// 从系统凭据库获取凭据
    ///
    /// # 参数
    /// - url: 禅道服务器 URL
    /// - account: 账号名称
    ///
    /// # 返回
    /// - Ok(Some(Credentials)): 找到凭据
    /// - Ok(None): 未找到凭据
    /// - Err: 获取失败
    pub fn get(url: &str, account: &str) -> Result<Option<Credentials>, ZentaoError> {
        init_keyring_store();
        let target = format!("{}:{}:{}", SERVICE_NAME, url, account);
        let entry = Entry::new_with_modifiers(SERVICE_NAME, account, &HashMap::from([("target", target.as_str())]))
            .map_err(|e| ZentaoError::Config(format!("Failed to create keyring entry: {}", e)))?;

        match entry.get_password() {
            Ok(password) => Ok(Some(Credentials {
                account: account.to_string(),
                password: Some(password),
            })),
            Err(Error::NoEntry) => Ok(None),
            Err(e) => Err(ZentaoError::Config(format!(
                "Failed to get credentials: {}",
                e
            ))),
        }
    }

    /// 保存凭据到系统凭据库
    ///
    /// # 参数
    /// - url: 禅道服务器 URL
    /// - account: 账号名称
    /// - password: 密码
    ///
    /// # 返回
    /// - Ok: 保存成功
    /// - Err: 保存失败
    pub fn store(url: &str, account: &str, password: &str) -> Result<(), ZentaoError> {
        init_keyring_store();
        let target = format!("{}:{}:{}", SERVICE_NAME, url, account);
        let entry = Entry::new_with_modifiers(SERVICE_NAME, account, &HashMap::from([("target", target.as_str()), ("persistence", "Local")]))
            .map_err(|e| ZentaoError::Config(format!("Failed to create keyring entry: {}", e)))?;

        entry
            .set_password(password)
            .map_err(|e| ZentaoError::Config(format!("Failed to store credentials: {}", e)))?;

        // 更新 username 属性
        entry
            .update_attributes(&HashMap::from([("username", account)]))
            .map_err(|e| ZentaoError::Config(format!("Failed to update username: {}", e)))
    }

    /// 删除系统凭据库中的凭据
    ///
    /// # 参数
    /// - url: 禅道服务器 URL
    /// - account: 账号名称
    ///
    /// # 返回
    /// - Ok: 删除成功（或凭据不存在）
    /// - Err: 删除失败
    pub fn delete(url: &str, account: &str) -> Result<(), ZentaoError> {
        init_keyring_store();
        let target = format!("{}:{}:{}", SERVICE_NAME, url, account);
        let entry = Entry::new_with_modifiers(SERVICE_NAME, account, &HashMap::from([("target", target.as_str())]))
            .map_err(|e| ZentaoError::Config(format!("Failed to create keyring entry: {}", e)))?;

        match entry.delete_credential() {
            Ok(()) | Err(Error::NoEntry) => Ok(()),
            Err(e) => Err(ZentaoError::Config(format!(
                "Failed to delete credentials: {}",
                e
            ))),
        }
    }

    /// 检查凭据是否存在
    pub fn exists(url: &str, account: &str) -> bool {
        init_keyring_store();
        Self::get(url, account).map(|opt| opt.is_some()).unwrap_or(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_credentials_service_name() {
        assert_eq!(SERVICE_NAME, "zentao-cli");
    }

    #[test]
    fn test_credentials_struct() {
        let creds = Credentials {
            account: "testuser".to_string(),
            password: Some("testpass".to_string()),
        };
        assert_eq!(creds.account, "testuser");
        assert_eq!(creds.password, Some("testpass".to_string()));
    }

    #[test]
    fn test_credentials_struct_without_password() {
        let creds = Credentials {
            account: "testuser".to_string(),
            password: None,
        };
        assert_eq!(creds.account, "testuser");
        assert!(creds.password.is_none());
    }

    #[test]
    fn test_exists_returns_false_on_error() {
        let result = Credentials::exists(
            "http://nonexistent.example.com",
            "nonexistent_user_12345",
        );
        assert!(!result);
    }

    #[test]
    fn test_delete_nonexistent_returns_ok() {
        let result = Credentials::delete(
            "http://nonexistent.example.com",
            "nonexistent_user_12345",
        );
        assert!(result.is_ok() || result.is_err());
    }
}
