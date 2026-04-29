//! API Client integration tests
//!
//! These tests verify the API client module behavior.

mod common;

use zentao_cli::{ApiClient, Auth};

#[test]
fn test_api_client_creation() {
    let client = common::test_client();
    // Client created successfully with valid config
    let _ = client;
}

#[test]
fn test_api_client_with_token() {
    let client = ApiClient::new("https://example.com", Some("my_token".to_string()));
    let _ = client;
}

#[test]
fn test_auth_new() {
    let config = common::test_config();
    let _auth = Auth::new(&config.url);
}

#[test]
fn test_config_token_extraction() {
    let config = common::test_config();
    assert!(config.token.is_some());
    assert_eq!(config.token.unwrap(), "test_token_12345");
}
