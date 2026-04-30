//! ZenTao API 认证模块
//!
//! 负责与 ZenTao 服务器进行身份认证交互

use crate::core::ZentaoError;
use anyhow::Result;
use reqwest::Client;
use serde::Deserialize;

/// 禅道认证客户端
///
/// 封装登录和 Token 验证的 HTTP 请求逻辑
pub struct Auth {
    client: Client,
    /// 禅道服务器基础地址，如 https://demo.zentao.site
    base_url: String,
}

impl Auth {
    /// 创建认证客户端
    pub fn new(base_url: &str) -> Self {
        let base_url = base_url.trim();
        // 如果没有协议头，自动添加 http://
        let base_url = if !base_url.starts_with("http://") && !base_url.starts_with("https://") {
            format!("http://{}", base_url)
        } else {
            base_url.to_string()
        };
        Self {
            client: Client::new(),
            // 移除 URL 末尾的 /，避免拼接路径时出现双斜杠
            base_url: base_url.trim_end_matches('/').to_string(),
        }
    }

    /// 登录禅道获取 Token
    ///
    /// 调用 POST /api.php/v1/tokens 接口
    /// 请求体: {"account": "xxx", "password": "xxx"}
    /// 响应体: {"token": "xxx", "status": "success"}
    ///
    /// # 参数
    /// - account: 禅道账号
    /// - password: 禅道密码
    ///
    /// # 返回
    /// - Ok(token): 登录成功，返回认证 Token
    /// - Err: 登录失败（网络错误、账号密码错误等）
    pub async fn login(&self, account: &str, password: &str) -> Result<String> {
        // ZenTao API v1 获取 Token 的接口
        let url = format!("{}/api.php/v1/tokens", self.base_url);

        // 发送 POST 请求，Content-Type 为 application/json
        let resp = self
            .client
            .post(&url)
            .json(&serde_json::json!({
                "account": account,
                "password": password
            }))
            .send() // 发送请求
            .await // async: 等待响应
            // 网络层错误（如连接超时、DNS 解析失败）
            .map_err(|e| ZentaoError::Network(e.to_string()))?;

        // 解析响应 JSON
        #[derive(Deserialize)]
        struct TokenResp {
            /// 认证 Token，成功登录时存在
            token: Option<String>,
            #[allow(dead_code)]
            status: Option<String>,
        }

        let token_resp: TokenResp = resp
            .json()
            .await
            // API 返回的 JSON 解析失败（如响应不是 JSON 格式）
            .map_err(|e| ZentaoError::Api(e.to_string()))?;

        // 从响应中提取 token，如果为空则返回认证失败错误
        token_resp
            .token
            .ok_or_else(|| ZentaoError::Auth("Failed to get token".into()).into())
    }

    /// 验证 Token 是否有效
    ///
    /// 调用 GET /api.php/v1/user 接口
    /// 携带请求头: Token: {token}
    ///
    /// # 参数
    /// - token: 待验证的认证 Token
    ///
    /// # 返回
    /// - Ok(true): Token 有效
    /// - Ok(false): Token 无效（服务器返回非 2xx 状态码）
    /// - Err: 网络错误
    pub async fn verify_token(&self, token: &str) -> Result<bool> {
        // ZenTao API v1 获取当前用户信息接口
        let url = format!("{}/api.php/v1/user", self.base_url);

        let resp = self
            .client
            .get(&url)
            // ZenTao API v1 使用 Token header
            .header("Token", token)
            .send()
            .await
            .map_err(|e| ZentaoError::Network(e.to_string()))?;

        // 检查 HTTP 状态码是否成功
        Ok(resp.status().is_success())
    }
}
