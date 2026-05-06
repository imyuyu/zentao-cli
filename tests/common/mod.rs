use zentao_cli::{ApiClient, Config};

/// Creates a test configuration
#[allow(dead_code)]
pub fn test_config() -> Config {
    Config {
        url: "https://test.zentao.com".to_string(),
        token: Some("test_token_12345".to_string()),
        product_id: Some(1),
        project_id: Some(1),
        api_version: None,
    }
}

/// Creates an API client for testing
#[allow(dead_code)]
pub fn test_client() -> ApiClient {
    let config = test_config();
    ApiClient::new(&config.url, config.token)
}
