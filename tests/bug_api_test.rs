//! Bug API integration tests
//!
//! Tests for the Bug API public interface.

mod common;

use zentao_cli::api::{CreateBugRequest, UpdateBugRequest};

#[test]
fn test_create_bug_request_builder() {
    let req = CreateBugRequest {
        title: "Critical Bug Found".to_string(),
        product: 1,
        severity: 4,
        pri: Some(1),
        type_: Some("code".to_string()),
        steps: Some("1. Go to page\n2. Click button".to_string()),
        story: Some(100),
        assigned_to: Some("dev".to_string()),
    };

    let json = serde_json::to_string(&req).unwrap();
    assert!(json.contains("Critical Bug Found"));
    assert!(json.contains("\"product\":1"));
    assert!(json.contains("\"severity\":4"));
    assert!(json.contains("\"pri\":1"));
}

#[test]
fn test_create_bug_request_minimal() {
    let req = CreateBugRequest {
        title: "Minimal Bug".to_string(),
        product: 2,
        severity: 3,
        pri: None,
        type_: None,
        steps: None,
        story: None,
        assigned_to: None,
    };

    let json = serde_json::to_string(&req).unwrap();
    assert!(json.contains("Minimal Bug"));
    assert!(!json.contains("pri"));
    assert!(!json.contains("steps"));
}

#[test]
fn test_update_bug_request_resolution() {
    let req = UpdateBugRequest {
        title: None,
        status: Some("resolved".to_string()),
        resolution: Some("fixed".to_string()),
        resolved_build: None,
        assigned_to: None,
    };

    let json = serde_json::to_string(&req).unwrap();
    assert!(json.contains("resolved"));
    assert!(json.contains("fixed"));
    assert!(!json.contains("title"));
}

#[test]
fn test_update_bug_request_all_fields() {
    let req = UpdateBugRequest {
        title: Some("Fixed Bug".to_string()),
        status: Some("closed".to_string()),
        resolution: Some("fixed".to_string()),
        resolved_build: None,
        assigned_to: Some("admin".to_string()),
    };

    let json = serde_json::to_string(&req).unwrap();
    assert!(json.contains("Fixed Bug"));
    assert!(json.contains("closed"));
    assert!(json.contains("fixed"));
    assert!(json.contains("admin"));
}
