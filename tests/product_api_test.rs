//! Product API integration tests
//!
//! Tests for the Product API public interface.

mod common;

use zentao_cli::api::Product;

#[test]
fn test_product_serialization() {
    let product = Product {
        id: 1,
        name: "Test Product".to_string(),
        code: "TEST".to_string(),
        status: "normal".to_string(),
        desc: Some("A test product".to_string()),
    };

    let json = serde_json::to_string(&product).unwrap();
    assert!(json.contains("Test Product"));
    assert!(json.contains("TEST"));
    assert!(json.contains("normal"));
    assert!(json.contains("A test product"));
}

#[test]
fn test_product_without_desc() {
    let product = Product {
        id: 2,
        name: "No Desc Product".to_string(),
        code: "NODESC".to_string(),
        status: "active".to_string(),
        desc: None,
    };

    let json = serde_json::to_string(&product).unwrap();
    assert!(json.contains("No Desc Product"));
    // desc should be skipped when None
    assert!(!json.contains("desc"));
}

#[test]
fn test_product_deserialization() {
    let json = r#"{
        "id": 10,
        "name": "My Product",
        "code": "MYPROD",
        "status": "active"
    }"#;

    let product: Product = serde_json::from_str(json).unwrap();
    assert_eq!(product.id, 10);
    assert_eq!(product.name, "My Product");
    assert_eq!(product.code, "MYPROD");
    assert_eq!(product.status, "active");
}

#[test]
fn test_product_with_description() {
    let json = r#"{
        "id": 11,
        "name": "Product With Desc",
        "code": "DESCPROD",
        "status": "normal",
        "desc": "Product description here"
    }"#;

    let product: Product = serde_json::from_str(json).unwrap();
    assert_eq!(product.id, 11);
    assert_eq!(product.desc, Some("Product description here".to_string()));
}
