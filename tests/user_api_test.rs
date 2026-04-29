//! User API integration tests
//!
//! Tests for the User API public interface.

mod common;

use zentao_cli::api::types::User;

#[test]
fn test_user_serialization() {
    let user = User {
        id: 1,
        account: "admin".to_string(),
        realname: "Administrator".to_string(),
        email: Some("admin@example.com".to_string()),
        dept: Some(1),
        role: Some("dev".to_string()),
    };

    let json = serde_json::to_string(&user).unwrap();
    assert!(json.contains("admin"));
    assert!(json.contains("Administrator"));
    assert!(json.contains("admin@example.com"));
    assert!(json.contains("\"dept\":1"));
    assert!(json.contains("\"role\":\"dev\""));
}

#[test]
fn test_user_minimal() {
    let user = User {
        id: 2,
        account: "user1".to_string(),
        realname: "User One".to_string(),
        email: None,
        dept: None,
        role: None,
    };

    let json = serde_json::to_string(&user).unwrap();
    assert!(json.contains("user1"));
    assert!(json.contains("User One"));
    // Optional fields should be skipped when None
    assert!(!json.contains("email"));
    assert!(!json.contains("dept"));
    assert!(!json.contains("role"));
}

#[test]
fn test_user_deserialization() {
    let json = r#"{
        "id": 10,
        "account": "testuser",
        "realname": "Test User",
        "email": "test@example.com",
        "dept": 5,
        "role": "qa"
    }"#;

    let user: User = serde_json::from_str(json).unwrap();
    assert_eq!(user.id, 10);
    assert_eq!(user.account, "testuser");
    assert_eq!(user.realname, "Test User");
    assert_eq!(user.email, Some("test@example.com".to_string()));
    assert_eq!(user.dept, Some(5));
    assert_eq!(user.role, Some("qa".to_string()));
}

#[test]
fn test_user_with_minimal_fields() {
    let json = r#"{
        "id": 11,
        "account": "minimal",
        "realname": "Minimal User"
    }"#;

    let user: User = serde_json::from_str(json).unwrap();
    assert_eq!(user.id, 11);
    assert_eq!(user.account, "minimal");
    assert_eq!(user.realname, "Minimal User");
    assert_eq!(user.email, None);
    assert_eq!(user.dept, None);
    assert_eq!(user.role, None);
}

#[test]
fn test_user_list_response_empty() {
    let json = r#"{
        "users": []
    }"#;

    #[derive(serde::Deserialize)]
    struct UserListResponse {
        users: Option<Vec<User>>,
    }

    let response: UserListResponse = serde_json::from_str(json).unwrap();
    assert!(response.users.is_some());
    assert!(response.users.unwrap().is_empty());
}

#[test]
fn test_user_list_response_with_users() {
    let json = r#"{
        "users": [
            {
                "id": 1,
                "account": "admin",
                "realname": "Administrator"
            },
            {
                "id": 2,
                "account": "user1",
                "realname": "User One"
            }
        ]
    }"#;

    #[derive(serde::Deserialize)]
    struct UserListResponse {
        users: Option<Vec<User>>,
    }

    let response: UserListResponse = serde_json::from_str(json).unwrap();
    let users = response.users.unwrap();
    assert_eq!(users.len(), 2);
    assert_eq!(users[0].account, "admin");
    assert_eq!(users[1].account, "user1");
}

#[test]
fn test_user_list_response_null_users() {
    let json = r#"{
        "users": null
    }"#;

    #[derive(serde::Deserialize)]
    struct UserListResponse {
        users: Option<Vec<User>>,
    }

    let response: UserListResponse = serde_json::from_str(json).unwrap();
    assert!(response.users.is_none());
}
