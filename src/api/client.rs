//! ZenTao API HTTP 客户端
//!
//! 封装所有与 ZenTao API 的 HTTP 通信

use crate::core::ZentaoError;
use anyhow::Result;
use reqwest::Client;
use std::time::Duration;

/// API 客户端
///
/// 持有 HTTP 客户端和认证信息，负责发送所有 API 请求
pub struct ApiClient {
    client: Client,
    /// 禅道服务器基础地址
    base_url: String,
    /// 认证 Token
    token: Option<String>,
    /// API 版本 (v1 或 v2)，默认为 v1
    /// v1: Header "Token: xxx"
    /// v2: Header "token: xxx" (小写 t)
    api_version: String,
}

impl ApiClient {
    /// 创建 API 客户端
    ///
    /// # 参数
    /// - base_url: 禅道服务器地址，如 https://demo.zentao.site
    /// - token: 认证 Token（可选，不传则表示匿名请求）
    pub fn new(base_url: &str, token: Option<String>) -> Self {
        // 创建带超时配置的 HTTP 客户端
        // 所有请求最多等待 30 秒，超过则返回网络错误
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .expect("Failed to create HTTP client");

        Self {
            client,
            // 移除末尾斜杠，避免拼接路径时出现双斜杠
            base_url: base_url.trim_end_matches('/').to_string(),
            token,
            // 默认使用 v1 API
            api_version: "v1".to_string(),
        }
    }

    /// 设置 API 版本
    pub fn with_api_version(mut self, version: &str) -> Self {
        self.api_version = version.to_string();
        self
    }

    /// 链式调用：为客户端设置 Token
    ///
    /// # 示例
    /// ```rust,ignore
    /// let client = ApiClient::new(url, None, "v1").with_token(token);
    /// ```
    pub fn with_token(mut self, token: String) -> Self {
        self.token = Some(token);
        self
    }

    /// 拼接完整的 API URL
    ///
    /// /api.php/v1/stories  -> https://demo.zentao.site/api.php/v1/stories
    /// api.php/v1/stories   -> https://demo.zentao.site/api.php/v1/stories
    fn get_url(&self, path: &str) -> String {
        if path.starts_with('/') {
            format!("{}{}", self.base_url, path)
        } else {
            format!("{}/{}", self.base_url, path)
        }
    }

    /// 根据 API 版本获取认证 header 名称
    ///
    /// v1: "Token"
    /// v2: "token"
    fn auth_header_name(&self) -> &str {
        if self.api_version == "v2" {
            "token"
        } else {
            "Token"
        }
    }

    /// 发送 GET 请求
    ///
    /// # 参数
    /// - path: API 路径，如 /api.php/v1/stories
    ///
    /// # 返回
    /// 反序列化后的响应体
    pub async fn get<T: serde::de::DeserializeOwned>(&self, path: &str) -> Result<T> {
        let url = self.get_url(path);

        let mut req = self.client.get(&url);

        // 如果有 Token，添加认证请求头
        if let Some(ref token) = self.token {
            req = req.header(self.auth_header_name(), token.as_str());
        }

        let resp = req
            .send()
            .await
            .map_err(|e| ZentaoError::Network(e.to_string()))?;

        // 处理 404 错误，返回专门的 NotFound 错误
        if resp.status() == reqwest::StatusCode::NOT_FOUND {
            return Err(ZentaoError::NotFound(format!("Resource not found: {}", path)).into());
        }

        // 非 2xx 状态码视为 API 错误
        if !resp.status().is_success() {
            let status = resp.status();
            let msg = resp.text().await.unwrap_or_default();
            return Err(ZentaoError::Api(format!("API error {}: {}", status, msg)).into());
        }

        // 解析响应 JSON
        resp.json()
            .await
            .map_err(|e| ZentaoError::Api(e.to_string()).into())
    }

    /// 发送 POST 请求（创建资源）
    ///
    /// # 参数
    /// - path: API 路径
    /// - body: 请求体（会自动序列化为 JSON）
    pub async fn post<T: serde::de::DeserializeOwned, B: serde::Serialize>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<T> {
        let url = self.get_url(path);

        let mut req = self.client.post(&url).json(body);

        if let Some(ref token) = self.token {
            req = req.header(self.auth_header_name(), token.as_str());
        }

        let resp = req
            .send()
            .await
            .map_err(|e| ZentaoError::Network(e.to_string()))?;

        if !resp.status().is_success() {
            let status = resp.status();
            let msg = resp.text().await.unwrap_or_default();
            return Err(ZentaoError::Api(format!("API error {}: {}", status, msg)).into());
        }

        resp.json()
            .await
            .map_err(|e| ZentaoError::Api(e.to_string()).into())
    }

    /// 发送 PUT 请求（更新资源）
    ///
    /// # 参数
    /// - path: API 路径
    /// - body: 请求体
    pub async fn put<T: serde::de::DeserializeOwned, B: serde::Serialize>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<T> {
        let url = self.get_url(path);

        let mut req = self.client.put(&url).json(body);

        if let Some(ref token) = self.token {
            req = req.header(self.auth_header_name(), token.as_str());
        }

        let resp = req
            .send()
            .await
            .map_err(|e| ZentaoError::Network(e.to_string()))?;

        if !resp.status().is_success() {
            let status = resp.status();
            let msg = resp.text().await.unwrap_or_default();
            return Err(ZentaoError::Api(format!("API error {}: {}", status, msg)).into());
        }

        resp.json()
            .await
            .map_err(|e| ZentaoError::Api(e.to_string()).into())
    }

    /// 发送 DELETE 请求（删除资源）
    ///
    /// # 参数
    /// - path: API 路径
    pub async fn delete<T: serde::de::DeserializeOwned>(&self, path: &str) -> Result<T> {
        let url = self.get_url(path);

        let mut req = self.client.delete(&url);

        if let Some(ref token) = self.token {
            req = req.header(self.auth_header_name(), token.as_str());
        }

        let resp = req
            .send()
            .await
            .map_err(|e| ZentaoError::Network(e.to_string()))?;

        if !resp.status().is_success() {
            let status = resp.status();
            let msg = resp.text().await.unwrap_or_default();
            return Err(ZentaoError::Api(format!("API error {}: {}", status, msg)).into());
        }

        resp.json()
            .await
            .map_err(|e| ZentaoError::Api(e.to_string()).into())
    }
}
