//! ZenTao API HTTP 客户端
//!
//! 封装所有与 ZenTao API 的 HTTP 通信

use crate::core::ZentaoError;
use anyhow::Result;
use reqwest::Client;
use std::time::Duration;

/// 当前 token 对应的用户信息（`GET /api.php/v1/user` 的 profile）
#[derive(Debug, Clone, serde::Deserialize)]
pub struct UserProfile {
    #[serde(default)]
    pub id: u64,
    #[serde(default)]
    pub account: String,
    #[serde(default)]
    pub realname: Option<String>,
}

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
    pub async fn get<T: serde::de::DeserializeOwned>(&self, path: &str) -> Result<T> {        let url = self.get_url(path);

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

        // 禅道某些 POST 成功时返回 2xx + 空 body（缺必需字段时也会这样），
        // 直接 resp.json() 会报 "EOF while parsing" 掩盖真实原因。
        let status = resp.status();
        let text = resp
            .text()
            .await
            .map_err(|e| ZentaoError::Api(e.to_string()))?;
        if text.trim().is_empty() {
            return Err(ZentaoError::Api(format!(
                "服务端返回空响应（POST {} → {}）。禅道在缺少必需参数时常返回 2xx 但不落库，\
                 请检查 product/execution/openedBuild 等字段。",
                path, status
            ))
            .into());
        }
        serde_json::from_str(text.trim())
            .map_err(|e| ZentaoError::Api(format!("响应解析失败: {} (body: {})", e, text)).into())
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

    /// 查询当前 token 对应的用户
    ///
    /// `GET /api.php/v1/user` 返回 `{"profile": {"id":3,"account":"...","realname":"..."}}`，
    /// 这里只取 CLI 需要的三个字段。
    pub async fn whoami(&self) -> Result<UserProfile> {
        #[derive(serde::Deserialize)]
        struct Wrapper {
            profile: UserProfile,
        }

        let wrapper: Wrapper = self.get("/api.php/v1/user").await?;
        Ok(wrapper.profile)
    }

    /// POST 且容忍空响应
    ///
    /// 禅道部分创建接口（如 release）成功时返回 200 + 空 body。返回响应里的 `id`
    /// （若有），没有则为 None —— 调用方需自行回查。
    pub async fn post_allowing_empty<B: serde::Serialize>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<Option<u64>> {
        let url = self.get_url(path);
        let mut req = self.client.post(&url).json(body);
        if let Some(ref token) = self.token {
            req = req.header(self.auth_header_name(), token.as_str());
        }

        let resp = req
            .send()
            .await
            .map_err(|e| ZentaoError::Network(e.to_string()))?;
        let status = resp.status();
        let text = resp
            .text()
            .await
            .map_err(|e| ZentaoError::Api(e.to_string()))?;

        if !status.is_success() {
            return Err(ZentaoError::Api(format!("API error {}: {}", status, text)).into());
        }
        if text.trim().is_empty() {
            return Ok(None);
        }
        let v: serde_json::Value = serde_json::from_str(text.trim()).unwrap_or_default();
        Ok(v.get("id").and_then(|i| i.as_u64()))
    }

    /// 探测服务端是否支持 v2 文件上传接口
    ///
    /// ZenTao 的文件上传只存在于 v2 API（≥ v22.0）。老版本（如 21.6）没有 v2 路由，
    /// 请求会被兜底路由吃掉——**返回 200 但 body 为空**，所以不能只看状态码，
    /// 必须确认拿到的是 JSON。用一个必然不存在的 id（0）探测：
    /// - v2 存在 → 返回 JSON 错误（如 `{"error":"..."}`）
    /// - v2 不存在 → 200 且 body 为空
    pub async fn supports_file_upload(&self) -> bool {
        let url = self.get_url("/api.php/v2/files/0");

        let mut req = self.client.get(&url);
        if let Some(ref token) = self.token {
            req = req.header("token", token.as_str());
        }

        let resp = match req.send().await {
            Ok(r) => r,
            // 网络问题不当作「不支持」，交给真正的上传去报错
            Err(_) => return true,
        };

        let body = resp.text().await.unwrap_or_default();
        let trimmed = body.trim();
        if trimmed.is_empty() {
            return false;
        }
        serde_json::from_str::<serde_json::Value>(trimmed).is_ok()
    }

    /// 上传文件（multipart/form-data，v2 API）
    ///
    /// 调用方需先用 [`Self::supports_file_upload`] 确认服务端支持。
    ///
    /// # 参数
    /// - `object_type`: 关联对象类型（bug / story / task / testcase）
    /// - `object_id`: 关联对象 ID
    /// - `filename`: 文件名
    /// - `content`: 文件内容
    pub async fn upload_file(
        &self,
        object_type: &str,
        object_id: u64,
        filename: &str,
        content: Vec<u8>,
    ) -> Result<u64> {
        let url = self.get_url("/api.php/v2/files");

        let part = reqwest::multipart::Part::bytes(content)
            .file_name(filename.to_string())
            .mime_str("application/octet-stream")
            .map_err(|e| ZentaoError::Config(e.to_string()))?;
        let form = reqwest::multipart::Form::new()
            .part("file", part)
            .text("objectType", object_type.to_string())
            .text("objectID", object_id.to_string());

        let mut req = self.client.post(&url).multipart(form);
        if let Some(ref token) = self.token {
            req = req.header("token", token.as_str());
        }

        let resp = req
            .send()
            .await
            .map_err(|e| ZentaoError::Network(e.to_string()))?;

        let status = resp.status();
        let body = resp.text().await.unwrap_or_default();
        if !status.is_success() {
            return Err(ZentaoError::Api(format!("upload failed {}: {}", status, body)).into());
        }

        let json: serde_json::Value = serde_json::from_str(body.trim()).map_err(|_| {
            ZentaoError::Api(format!(
                "上传接口返回非 JSON（该 ZenTao 版本可能不支持 API 上传附件）: {}",
                body
            ))
        })?;

        json.get("id")
            .and_then(|v| v.as_u64())
            .ok_or_else(|| {
                ZentaoError::Api(format!("上传响应缺少文件 id: {}", json)).into()
            })
    }
}
