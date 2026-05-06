use crate::api::ApiClient;
use crate::core::logging::{log, LogLevel};
use crate::core::{Config, OutputFormat, ZentaoError};
use anyhow::Result;

#[derive(Clone, Debug)]
pub struct AppContext {
    pub config: Config,
    pub format: OutputFormat,
    pub dry_run: bool,
}

impl AppContext {
    pub fn new(config: Config, format: OutputFormat, dry_run: bool) -> Self {
        Self {
            config,
            format,
            dry_run,
        }
    }

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
