use crate::api::ApiClient;
use crate::core::logging::{log, LogLevel};
use crate::core::{Config, OutputFormat, ZentaoError};
use anyhow::Result;
use std::sync::OnceLock;

#[derive(Clone, Debug)]
pub struct AppContext {
    pub config: Config,
    pub format: OutputFormat,
    pub dry_run: bool,
    /// v2 文件上传能力探测结果，首次使用时探测并缓存
    ///
    /// 用 `OnceLock` 而非构造时探测：探测需要一次 HTTP 往返，而上传是低频操作，
    /// 不该让 `bug list` 这类命令为它多付一次 RTT。
    file_upload_supported: OnceLock<bool>,
}

impl AppContext {
    pub fn new(config: Config, format: OutputFormat, dry_run: bool) -> Self {
        Self {
            config,
            format,
            dry_run,
            file_upload_supported: OnceLock::new(),
        }
    }

    /// 是否支持 v2 文件上传接口（首次调用时探测，之后复用结果）
    ///
    /// ZenTao 上传附件需要 v22.0+ 的 v2 API；老版本（如 21.6）没有该路由。
    pub async fn supports_file_upload(&self) -> bool {
        if let Some(cached) = self.file_upload_supported.get() {
            return *cached;
        }
        let supported = self.client().supports_file_upload().await;
        // 并发调用时可能都探测了一次，写不进去也无妨——结果一致
        let _ = self.file_upload_supported.set(supported);
        supported
    }

    /// 获取 API 客户端
    pub fn client(&self) -> ApiClient {
        log(
            LogLevel::Debug,
            "AppContext",
            format!(
                "create client url={} api_version={}",
                self.config.url,
                self.config.api_version.as_deref().unwrap_or("v1")
            ),
        );
        ApiClient::new(&self.config.url, self.config.token.clone())
            .with_api_version(self.config.api_version.as_deref().unwrap_or("v1"))
    }

    /// 刷新认证 Token
    ///
    /// 当 API 返回 401 时调用此方法，从 keyring 获取凭据重新登录
    pub async fn refresh_token(&mut self) -> Result<()> {
        let account = self.config.account.as_ref().ok_or_else(|| {
            ZentaoError::Auth("No account configured. Please login first.".into())
        })?;

        let creds = crate::core::Credentials::get(&self.config.url, account)
            .map_err(|e| {
                crate::core::logging::log(
                    crate::core::logging::LogLevel::Debug,
                    "Auth",
                    format!("refresh_token 取凭据失败: {}", e),
                );
                ZentaoError::Config(format!("Failed to get credentials: {}", e))
            })?
            .ok_or_else(|| {
                ZentaoError::Auth("No credentials stored. Please login first.".into())
            })?;

        let password = creds
            .password
            .ok_or_else(|| ZentaoError::Auth("Password not found in credentials".into()))?;

        let auth = crate::api::Auth::new(&self.config.url);
        let new_token = auth.login(&creds.account, &password).await?;

        self.config.token = Some(new_token.clone());
        crate::core::save_config(&self.config)?;

        log(LogLevel::Info, "AppContext", "Token refreshed successfully");

        Ok(())
    }

    pub fn product_id(&self, cli_value: Option<u64>) -> Option<u64> {
        self.config.product_id(cli_value)
    }

    pub fn project_id(&self, cli_value: Option<u64>) -> Option<u64> {
        self.config.project_id(cli_value)
    }

    pub fn require_product_id(&self, cli_value: Option<u64>) -> Result<u64> {
        self.product_id(cli_value).ok_or_else(|| {
            ZentaoError::Config(
                "product ID is required. Provide via --product or set ZENTAO_PRODUCT_ID"
                    .to_string(),
            )
            .into()
        })
    }

    pub fn require_project_id(&self, cli_value: Option<u64>) -> Result<u64> {
        self.project_id(cli_value).ok_or_else(|| {
            ZentaoError::Config(
                "project ID is required. Provide via --project or set ZENTAO_PROJECT_ID"
                    .to_string(),
            )
            .into()
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_context_new() {
        let config = Config {
            url: "https://test.com".to_string(),
            token: Some("test_token".to_string()),
            product_id: Some(100),
            project_id: Some(200),
            api_version: Some("v2".to_string()),
            account: Some("testuser".to_string()),
        };
        let ctx = AppContext::new(config.clone(), OutputFormat::Json, false);
        assert_eq!(ctx.config.url, "https://test.com");
        assert_eq!(ctx.format, OutputFormat::Json);
        assert!(!ctx.dry_run);
    }

    #[test]
    fn test_app_context_product_id_cli_override() {
        let config = Config {
            url: "https://test.com".to_string(),
            token: None,
            product_id: Some(100),
            project_id: None,
            api_version: None,
            account: None,
        };
        let ctx = AppContext::new(config, OutputFormat::Json, false);
        // CLI value takes precedence
        assert_eq!(ctx.product_id(Some(50)), Some(50));
        // Falls back to config value
        assert_eq!(ctx.product_id(None), Some(100));
    }

    #[test]
    fn test_app_context_project_id_cli_override() {
        let config = Config {
            url: "https://test.com".to_string(),
            token: None,
            product_id: None,
            project_id: Some(200),
            api_version: None,
            account: None,
        };
        let ctx = AppContext::new(config, OutputFormat::Json, false);
        assert_eq!(ctx.project_id(Some(50)), Some(50));
        assert_eq!(ctx.project_id(None), Some(200));
    }

    #[test]
    fn test_require_product_id_success() {
        let config = Config {
            url: "https://test.com".to_string(),
            token: None,
            product_id: Some(100),
            project_id: None,
            api_version: None,
            account: None,
        };
        let ctx = AppContext::new(config, OutputFormat::Json, false);
        let result = ctx.require_product_id(None);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 100);
    }

    #[test]
    fn test_require_product_id_failure() {
        let config = Config {
            url: "https://test.com".to_string(),
            token: None,
            product_id: None,
            project_id: None,
            api_version: None,
            account: None,
        };
        let ctx = AppContext::new(config, OutputFormat::Json, false);
        let result = ctx.require_product_id(None);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("product ID is required"));
    }

    #[test]
    fn test_require_project_id_failure() {
        let config = Config {
            url: "https://test.com".to_string(),
            token: None,
            product_id: None,
            project_id: None,
            api_version: None,
            account: None,
        };
        let ctx = AppContext::new(config, OutputFormat::Json, false);
        let result = ctx.require_project_id(None);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("project ID is required"));
    }

    #[test]
    fn test_app_context_clone() {
        let config = Config {
            url: "https://test.com".to_string(),
            token: Some("token".to_string()),
            product_id: Some(1),
            project_id: None,
            api_version: None,
            account: None,
        };
        let ctx1 = AppContext::new(config, OutputFormat::Pretty, true);
        let ctx2 = ctx1.clone();
        assert_eq!(ctx1.config.url, ctx2.config.url);
        assert_eq!(ctx1.format, ctx2.format);
        assert_eq!(ctx1.dry_run, ctx2.dry_run);
    }
}
