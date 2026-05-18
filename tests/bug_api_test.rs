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
        branch: None,
        module: None,
        execution: None,
        keywords: None,
        os: None,
        browser: None,
        deadline: None,
        opened_build: Some(vec!["trunk".to_string()]),
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
        branch: None,
        module: None,
        execution: None,
        keywords: None,
        os: None,
        browser: None,
        deadline: None,
        opened_build: None,
    };

    let json = serde_json::to_string(&req).unwrap();
    assert!(json.contains("Minimal Bug"));
    assert!(!json.contains("pri"));
    assert!(!json.contains("steps"));
}

#[test]
fn test_update_bug_request_all_fields() {
    // UpdateBugRequest only supports: title, keywords, severity, pri, type_, os, browser, steps, task, story, deadline, opened_build, branch, module, execution
    let req = UpdateBugRequest {
        title: Some("Fixed Bug".to_string()),
        keywords: Some("fixed".to_string()),
        severity: Some(3),
        pri: Some(2),
        type_: None,
        os: None,
        browser: None,
        steps: None,
        task: None,
        story: None,
        deadline: None,
        opened_build: None,
        branch: None,
        module: None,
        execution: None,
    };

    let json = serde_json::to_string(&req).unwrap();
    assert!(json.contains("Fixed Bug"));
    assert!(json.contains("\"severity\":3"));
    assert!(json.contains("fixed"));
}
