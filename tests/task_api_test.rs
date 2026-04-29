//! Task API integration tests
//!
//! Tests for the Task API public interface.

mod common;

use zentao_cli::api::{CreateTaskRequest, Task, UpdateTaskRequest};

#[test]
fn test_task_serialization() {
    let task = Task {
        id: 1,
        name: "Implement login".to_string(),
        project: 5,
        status: "in progress".to_string(),
        pri: 3,
        assigned_to: Some("developer".to_string()),
        estimate: Some(8.0),
        consumed: Some(3.0),
        left: Some(5.0),
    };

    let json = serde_json::to_string(&task).unwrap();
    assert!(json.contains("Implement login"));
    assert!(json.contains("\"project\":5"));
    assert!(json.contains("\"pri\":3"));
    assert!(json.contains("developer"));
}

#[test]
fn test_task_minimal() {
    let task = Task {
        id: 2,
        name: "Quick fix".to_string(),
        project: 3,
        status: "open".to_string(),
        pri: 1,
        assigned_to: None,
        estimate: None,
        consumed: None,
        left: None,
    };

    let json = serde_json::to_string(&task).unwrap();
    assert!(json.contains("Quick fix"));
    // Optional fields should be skipped
    assert!(!json.contains("assigned_to"));
    assert!(!json.contains("estimate"));
}

#[test]
fn test_create_task_request() {
    let req = CreateTaskRequest {
        name: "New Task".to_string(),
        project: 1,
        pri: 2,
        type_: Some("development".to_string()),
        assigned_to: Some("dev1".to_string()),
        estimate: Some(13.5),
    };

    let json = serde_json::to_string(&req).unwrap();
    assert!(json.contains("New Task"));
    assert!(json.contains("\"project\":1"));
    assert!(json.contains("\"pri\":2"));
    assert!(json.contains("development"));
    assert!(json.contains("13.5"));
}

#[test]
fn test_update_task_request_partial() {
    // Only update status
    let req = UpdateTaskRequest {
        name: None,
        status: Some("done".to_string()),
        pri: None,
        assigned_to: None,
    };

    let json = serde_json::to_string(&req).unwrap();
    assert!(json.contains("done"));
    assert!(!json.contains("name"));
    assert!(!json.contains("pri"));
}

#[test]
fn test_task_deserialization() {
    let json = r#"{
        "id": 100,
        "name": "Story Task",
        "project": 4,
        "status": "closed",
        "pri": 5,
        "assigned_to": "admin",
        "estimate": 5.0,
        "consumed": 5.0,
        "left": 0.0
    }"#;

    let task: Task = serde_json::from_str(json).unwrap();
    assert_eq!(task.id, 100);
    assert_eq!(task.name, "Story Task");
    assert_eq!(task.status, "closed");
    assert_eq!(task.estimate, Some(5.0));
    assert_eq!(task.consumed, Some(5.0));
    assert_eq!(task.left, Some(0.0));
}
