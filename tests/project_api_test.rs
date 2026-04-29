//! Project API integration tests
//!
//! Tests for the Project API public interface.

mod common;

use zentao_cli::api::Project;

#[test]
fn test_project_serialization() {
    let project = Project {
        id: 1,
        name: "Alpha Project".to_string(),
        code: "ALPHA".to_string(),
        status: "doing".to_string(),
        desc: Some("Main project".to_string()),
    };

    let json = serde_json::to_string(&project).unwrap();
    assert!(json.contains("Alpha Project"));
    assert!(json.contains("ALPHA"));
    assert!(json.contains("doing"));
}

#[test]
fn test_project_deserialization() {
    let json = r#"{
        "id": 5,
        "name": "Beta Project",
        "code": "BETA",
        "status": "wait",
        "desc": "Future project"
    }"#;

    let project: Project = serde_json::from_str(json).unwrap();
    assert_eq!(project.id, 5);
    assert_eq!(project.name, "Beta Project");
    assert_eq!(project.code, "BETA");
    assert_eq!(project.status, "wait");
    assert_eq!(project.desc, Some("Future project".to_string()));
}

#[test]
fn test_project_minimal_json() {
    let json = r#"{
        "id": 6,
        "name": "Minimal",
        "code": "MIN",
        "status": "closed"
    }"#;

    let project: Project = serde_json::from_str(json).unwrap();
    assert_eq!(project.id, 6);
    assert_eq!(project.desc, None);
}

#[test]
fn test_project_list_response() {
    let projects = vec![
        Project {
            id: 1,
            name: "Project A".to_string(),
            code: "A".to_string(),
            status: "doing".to_string(),
            desc: None,
        },
        Project {
            id: 2,
            name: "Project B".to_string(),
            code: "B".to_string(),
            status: "wait".to_string(),
            desc: None,
        },
    ];

    let json = serde_json::to_string(&projects).unwrap();
    assert!(json.contains("Project A"));
    assert!(json.contains("Project B"));
}
