//! Story API integration tests
//!
//! Tests for the Story API public interface.

mod common;

use zentao_cli::api::{CreateStoryRequest, UpdateStoryRequest};

#[test]
fn test_create_story_request_builder() {
    let req = CreateStoryRequest {
        title: "New Feature Story".to_string(),
        product: 1,
        pri: 3,
        category: Some("feature".to_string()),
        spec: Some("Story specification".to_string()),
        verify: None,
        estimate: Some(8.0),
    };

    let json = serde_json::to_string(&req).unwrap();
    assert!(json.contains("New Feature Story"));
    assert!(json.contains("\"product\":1"));
    assert!(json.contains("\"pri\":3"));
    assert!(json.contains("feature"));
    assert!(json.contains("8"));
}

#[test]
fn test_create_story_request_minimal() {
    let req = CreateStoryRequest {
        title: "Minimal Story".to_string(),
        product: 2,
        pri: 1,
        category: None,
        spec: None,
        verify: None,
        estimate: None,
    };

    let json = serde_json::to_string(&req).unwrap();
    assert!(json.contains("Minimal Story"));
    assert!(!json.contains("category"));
    assert!(!json.contains("spec"));
}

#[test]
fn test_update_story_request_partial() {
    // Only update title
    let req = UpdateStoryRequest {
        title: Some("Updated Title".to_string()),
        status: None,
        pri: None,
        assigned_to: None,
    };

    let json = serde_json::to_string(&req).unwrap();
    assert!(json.contains("Updated Title"));
    assert!(!json.contains("status"));
    assert!(!json.contains("pri"));
}

#[test]
fn test_update_story_request_empty() {
    // All fields None - should serialize to empty object
    let req = UpdateStoryRequest {
        title: None,
        status: None,
        pri: None,
        assigned_to: None,
    };

    let json = serde_json::to_string(&req).unwrap();
    assert_eq!(json, "{}");
}
