//! Build API integration tests
//!
//! Tests for the Build API public interface.

mod common;

use zentao_cli::api::Build;

#[test]
fn test_build_serialization() {
    let build = Build {
        id: 1,
        name: "v1.0.0".to_string(),
        product: 1,
        project: 1,
        branch: Some(1),
        scm_path: Some("git@gitlab.example.com:repo.git".to_string()),
        ci: Some("Jenkins #123".to_string()),
        pkg: Some("/path/to/package.tar.gz".to_string()),
        file_size: Some("1048576".to_string()),
        generated_at: Some("2024-01-15 10:00:00".to_string()),
        deleted: Some("0".to_string()),
        editor: Some("admin".to_string()),
        created_by: Some("admin".to_string()),
        created_date: Some("2024-01-15 10:00:00".to_string()),
        last_edited_by: Some("admin".to_string()),
        last_edited_date: Some("2024-01-15 10:00:00".to_string()),
        consumed_cards: Some("10".to_string()),
        stories: Some("5".to_string()),
        bugs: Some("2".to_string()),
    };

    let json = serde_json::to_string(&build).unwrap();
    assert!(json.contains("v1.0.0"));
    assert!(json.contains("\"product\":1"));
    assert!(json.contains("\"project\":1"));
    assert!(json.contains("git@gitlab.example.com:repo.git"));
    assert!(json.contains("Jenkins #123"));
}

#[test]
fn test_build_without_optional_fields() {
    let build = Build {
        id: 2,
        name: "v2.0.0".to_string(),
        product: 1,
        project: 1,
        branch: None,
        scm_path: None,
        ci: None,
        pkg: None,
        file_size: None,
        generated_at: None,
        deleted: None,
        editor: None,
        created_by: None,
        created_date: None,
        last_edited_by: None,
        last_edited_date: None,
        consumed_cards: None,
        stories: None,
        bugs: None,
    };

    let json = serde_json::to_string(&build).unwrap();
    assert!(json.contains("v2.0.0"));
    // optional fields should be skipped when None
    assert!(!json.contains("branch"));
    assert!(!json.contains("scm_path"));
    assert!(!json.contains("ci"));
    assert!(!json.contains("pkg"));
    assert!(!json.contains("file_size"));
    assert!(!json.contains("generated_at"));
}

#[test]
fn test_build_deserialization() {
    let json = r#"{
        "id": 10,
        "name": "Build-2024-01-15",
        "product": 2,
        "project": 3,
        "branch": 1,
        "scm_path": "git@gitlab.example.com:repo.git",
        "ci": "GitLab CI #456",
        "pkg": "/artifacts/app.tar.gz",
        "file_size": "2097152",
        "generated_at": "2024-01-15 14:30:00",
        "deleted": "0",
        "editor": "developer",
        "created_by": "developer",
        "created_date": "2024-01-15 14:30:00",
        "last_edited_by": "developer",
        "last_edited_date": "2024-01-15 14:30:00",
        "consumed_cards": "15",
        "stories": "8",
        "bugs": "3"
    }"#;

    let build: Build = serde_json::from_str(json).unwrap();
    assert_eq!(build.id, 10);
    assert_eq!(build.name, "Build-2024-01-15");
    assert_eq!(build.product, 2);
    assert_eq!(build.project, 3);
    assert_eq!(build.branch, Some(1));
    assert_eq!(build.stories, Some("8".to_string()));
    assert_eq!(build.bugs, Some("3".to_string()));
}

#[test]
fn test_build_with_minimal_fields() {
    let json = r#"{
        "id": 5,
        "name": "Minimal Build",
        "product": 1,
        "project": 1
    }"#;

    let build: Build = serde_json::from_str(json).unwrap();
    assert_eq!(build.id, 5);
    assert_eq!(build.name, "Minimal Build");
    assert_eq!(build.product, 1);
    assert_eq!(build.project, 1);
    assert!(build.branch.is_none());
    assert!(build.scm_path.is_none());
    assert!(build.ci.is_none());
    assert!(build.pkg.is_none());
}
