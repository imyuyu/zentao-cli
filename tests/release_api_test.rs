//! Release API integration tests
//!
//! Tests for the Release API public interface.

mod common;

use zentao_cli::api::Release;

#[test]
fn test_release_serialization() {
    let release = Release {
        id: 1,
        name: "v1.0.0".to_string(),
        product: 1,
        build: Some(10),
        status: "normal".to_string(),
        marker: Some("stable".to_string()),
        date: Some("2024-01-15".to_string()),
    };

    let json = serde_json::to_string(&release).unwrap();
    assert!(json.contains("v1.0.0"));
    assert!(json.contains("normal"));
    assert!(json.contains("stable"));
    assert!(json.contains("2024-01-15"));
}

#[test]
fn test_release_without_optional_fields() {
    let release = Release {
        id: 2,
        name: "v2.0.0".to_string(),
        product: 1,
        build: None,
        status: "normal".to_string(),
        marker: None,
        date: None,
    };

    let json = serde_json::to_string(&release).unwrap();
    assert!(json.contains("v2.0.0"));
    // optional fields should be skipped when None
    assert!(!json.contains("build"));
    assert!(!json.contains("marker"));
    assert!(!json.contains("date"));
}

#[test]
fn test_release_deserialization() {
    let json = r#"{
        "id": 10,
        "name": "v3.0.0",
        "product": 1,
        "build": 20,
        "status": "normal",
        "marker": "beta",
        "date": "2024-06-01"
    }"#;

    let release: Release = serde_json::from_str(json).unwrap();
    assert_eq!(release.id, 10);
    assert_eq!(release.name, "v3.0.0");
    assert_eq!(release.product, 1);
    assert_eq!(release.build, Some(20));
    assert_eq!(release.status, "normal");
    assert_eq!(release.marker, Some("beta".to_string()));
    assert_eq!(release.date, Some("2024-06-01".to_string()));
}

#[test]
fn test_release_with_minimal_fields() {
    let json = r#"{
        "id": 11,
        "name": "v4.0.0",
        "product": 2,
        "status": "closed"
    }"#;

    let release: Release = serde_json::from_str(json).unwrap();
    assert_eq!(release.id, 11);
    assert_eq!(release.name, "v4.0.0");
    assert_eq!(release.product, 2);
    assert_eq!(release.build, None);
    assert_eq!(release.marker, None);
    assert_eq!(release.date, None);
}
