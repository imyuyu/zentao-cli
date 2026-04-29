//! ZenTao CLI 错误类型定义
//!
//! 定义统一的错误类型，支持错误码和友好的错误提示

use serde::Serialize;
use thiserror::Error;

// ============================================================
// 错误码常量
// ============================================================

/// API 相关错误
pub const ERR_API_ERROR: &str = "ZEN_API_ERROR";
/// 认证失败错误
pub const ERR_AUTH_FAILED: &str = "ZEN_AUTH_FAILED";
/// 资源不存在错误
pub const ERR_NOT_FOUND: &str = "ZEN_NOT_FOUND";
/// 配置无效错误
pub const ERR_CONFIG_INVALID: &str = "ZEN_CONFIG_INVALID";

// ============================================================
// 错误类型枚举
// ============================================================

/// ZenTao CLI 错误类型
///
/// 所有模块的错误都会转换为这个枚举类型
#[derive(Error, Debug)]
pub enum ZentaoError {
    /// API 调用错误（ZenTao 返回了错误状态码或解析失败）
    #[error("API error: {0}")]
    Api(String),

    /// 认证失败（Token 无效或过期）
    #[error("Authentication failed: {0}")]
    Auth(String),

    /// 资源不存在（ID 错误或已删除）
    #[error("Not found: {0}")]
    NotFound(String),

    /// 配置错误（URL 未设置、文件格式错误等）
    #[error("Invalid config: {0}")]
    Config(String),

    /// 网络错误（连接超时、DNS 失败等）
    #[error("Network error: {0}")]
    Network(String),
}

// ============================================================
// 错误响应结构
// ============================================================

/// API 错误响应格式
#[derive(Serialize)]
pub struct ErrorResponse {
    pub status: String,
    pub error: ErrorDetail,
}

/// 错误详情
#[derive(Serialize)]
pub struct ErrorDetail {
    /// 错误码
    pub code: String,
    /// 错误信息
    pub message: String,
    /// 解决提示
    pub hint: String,
}

// ============================================================
// 错误构造和转换方法
// ============================================================

impl ZentaoError {
    /// 创建 API 错误
    pub fn api(msg: impl Into<String>) -> Self {
        ZentaoError::Api(msg.into())
    }

    /// 创建认证错误
    pub fn auth(msg: impl Into<String>) -> Self {
        ZentaoError::Auth(msg.into())
    }

    /// 创建资源不存在错误
    pub fn not_found(msg: impl Into<String>) -> Self {
        ZentaoError::NotFound(msg.into())
    }

    /// 创建配置错误
    pub fn config(msg: impl Into<String>) -> Self {
        ZentaoError::Config(msg.into())
    }

    /// 创建网络错误
    pub fn network(msg: impl Into<String>) -> Self {
        ZentaoError::Network(msg.into())
    }

    /// 转换为 API 错误响应格式
    ///
    /// 用于向用户展示友好的错误信息
    pub fn to_response(&self) -> ErrorResponse {
        match self {
            ZentaoError::Api(msg) => ErrorResponse {
                status: "error".into(),
                error: ErrorDetail {
                    code: ERR_API_ERROR.into(),
                    message: msg.clone(),
                    hint: "Check ZENTAO_URL and network connectivity".into(),
                },
            },
            ZentaoError::Auth(msg) => ErrorResponse {
                status: "error".into(),
                error: ErrorDetail {
                    code: ERR_AUTH_FAILED.into(),
                    message: msg.clone(),
                    hint: "Run 'zentao auth login' to re-authenticate".into(),
                },
            },
            ZentaoError::NotFound(msg) => ErrorResponse {
                status: "error".into(),
                error: ErrorDetail {
                    code: ERR_NOT_FOUND.into(),
                    message: msg.clone(),
                    hint: "Verify the ID is correct".into(),
                },
            },
            ZentaoError::Config(msg) => ErrorResponse {
                status: "error".into(),
                error: ErrorDetail {
                    code: ERR_CONFIG_INVALID.into(),
                    message: msg.clone(),
                    hint: "Check configuration file or environment variables".into(),
                },
            },
            ZentaoError::Network(msg) => ErrorResponse {
                status: "error".into(),
                error: ErrorDetail {
                    code: "ZEN_NETWORK_ERROR".into(),
                    message: msg.clone(),
                    hint: "Check network connectivity".into(),
                },
            },
        }
    }
}

// ============================================================
// 错误类型转换
// ============================================================

/// 将 reqwest 网络库错误转换为 ZentaoError
impl From<reqwest::Error> for ZentaoError {
    fn from(err: reqwest::Error) -> Self {
        // 根据错误类型判断具体错误
        if err.is_connect() || err.is_timeout() {
            // 连接错误：网络不通或超时
            ZentaoError::Network(err.to_string())
        } else if let Some(status) = err.status() {
            // HTTP 状态码错误
            if status.as_u16() == 401 {
                // 401 未授权 = 认证失败
                ZentaoError::Auth(err.to_string())
            } else {
                // 其他状态码 = API 错误
                ZentaoError::Api(err.to_string())
            }
        } else {
            // 其他错误归类为 API 错误
            ZentaoError::Api(err.to_string())
        }
    }
}

/// 将 IO 错误转换为配置错误
impl From<std::io::Error> for ZentaoError {
    fn from(err: std::io::Error) -> Self {
        // 读取/写入配置文件失败归类为配置错误
        ZentaoError::Config(format!("IO error: {}", err))
    }
}

/// 将 TOML 解析错误转换为配置错误
impl From<toml::de::Error> for ZentaoError {
    fn from(err: toml::de::Error) -> Self {
        // 配置文件格式错误归类为配置错误
        ZentaoError::Config(format!("TOML parse error: {}", err))
    }
}

// ============================================================
// 单元测试
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_api_error_to_response() {
        let error = ZentaoError::api("connection refused");
        let response = error.to_response();
        assert_eq!(response.status, "error");
        assert_eq!(response.error.code, "ZEN_API_ERROR");
    }

    #[test]
    fn test_auth_error_to_response() {
        let error = ZentaoError::auth("token expired");
        let response = error.to_response();
        assert_eq!(response.error.code, "ZEN_AUTH_FAILED");
        assert!(response.error.hint.contains("re-authenticate"));
    }

    #[test]
    fn test_not_found_error_to_response() {
        let error = ZentaoError::not_found("product 123");
        let response = error.to_response();
        assert_eq!(response.error.code, "ZEN_NOT_FOUND");
    }

    #[test]
    fn test_config_error_to_response() {
        let error = ZentaoError::config("missing url");
        let response = error.to_response();
        assert_eq!(response.error.code, "ZEN_CONFIG_INVALID");
    }

    #[test]
    fn test_network_error_to_response() {
        let error = ZentaoError::network("timeout");
        let response = error.to_response();
        assert_eq!(response.error.code, "ZEN_NETWORK_ERROR");
    }
}
