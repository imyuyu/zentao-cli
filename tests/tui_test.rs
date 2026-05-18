//! TUI 自动化测试
//!
//! Tests for the TUI module including AppState transitions and helper functions.

mod common;

use zentao_cli::api::{Bug, Department, Story};
use zentao_cli::tui::{App, AppState};

// ============================================================
// AppState 状态转换测试
// ============================================================

fn create_test_app() -> App {
    let config = common::test_config();
    let multi_config = zentao_cli::core::config::MultiAccountConfig::default();
    App::new(config, multi_config)
}

#[test]
fn test_initial_state_is_idle() {
    let app = create_test_app();
    match app.state {
        AppState::Idle => {}
        _ => panic!("Expected initial state to be Idle"),
    }
}

#[test]
fn test_set_main_menu() {
    let mut app = create_test_app();
    app.set_main_menu();
    match &app.state {
        AppState::MainMenu { selected } => {
            assert_eq!(*selected, 0);
        }
        _ => panic!("Expected MainMenu state"),
    }
    assert_eq!(app.selected_index, 0);
}

#[test]
fn test_set_bug_list() {
    let mut app = create_test_app();
    let bugs = vec![
        Bug {
            id: 1,
            title: "Test Bug 1".to_string(),
            description: Some("Description 1".to_string()),
            status: "active".to_string(),
            severity: 3,
            pri: 3,
            type_: None,
            resolution: None,
            steps: None,
            product: 1,
            project: None,
            story: None,
            assigned_to: None,
            resolved_by: None,
            resolved_date: None,
        },
        Bug {
            id: 2,
            title: "Test Bug 2".to_string(),
            description: Some("Description 2".to_string()),
            status: "active".to_string(),
            severity: 4,
            pri: 2,
            type_: None,
            resolution: None,
            steps: None,
            product: 1,
            project: None,
            story: None,
            assigned_to: None,
            resolved_by: None,
            resolved_date: None,
        },
    ];

    app.set_bug_list(bugs.clone(), Some("Test Product".to_string()));

    match &app.state {
        AppState::BugList {
            bugs: returned_bugs,
            product_name,
        } => {
            assert_eq!(returned_bugs.len(), 2);
            assert_eq!(returned_bugs[0].id, 1);
            assert_eq!(returned_bugs[1].id, 2);
            assert_eq!(product_name.as_deref(), Some("Test Product"));
        }
        _ => panic!("Expected BugList state"),
    }
    assert_eq!(app.selected_index, 0);
}

#[test]
fn test_set_bug_detail() {
    let mut app = create_test_app();
    let bug = Bug {
        id: 42,
        title: "Specific Bug".to_string(),
        description: Some("Bug description".to_string()),
        status: "active".to_string(),
        severity: 2,
        pri: 1,
        type_: None,
        resolution: None,
        steps: None,
        product: 1,
        project: None,
        story: None,
        assigned_to: None,
        resolved_by: None,
        resolved_date: None,
    };

    app.set_bug_detail(bug.clone(), Some("Test Product".to_string()));

    match &app.state {
        AppState::BugDetail {
            bug: returned_bug,
            product_name,
        } => {
            assert_eq!(returned_bug.id, 42);
            assert_eq!(returned_bug.title, "Specific Bug");
            assert_eq!(product_name.as_deref(), Some("Test Product"));
        }
        _ => panic!("Expected BugDetail state"),
    }
    assert_eq!(app.selected_index, 0);
}

#[test]
fn test_set_story_list() {
    let mut app = create_test_app();
    let stories = vec![Story {
        id: 1,
        title: "Test Story 1".to_string(),
        description: Some("Description 1".to_string()),
        status: "active".to_string(),
        pri: 3,
        category: None,
        stage: None,
        product: 1,
        module: None,
        assigned_to: None,
        opened_by: None,
        estimate: None,
        version: None,
    }];

    app.set_story_list(stories.clone(), Some("Test Product".to_string()));

    match &app.state {
        AppState::StoryList {
            stories: returned_stories,
            product_name,
        } => {
            assert_eq!(returned_stories.len(), 1);
            assert_eq!(returned_stories[0].id, 1);
            assert_eq!(product_name.as_deref(), Some("Test Product"));
        }
        _ => panic!("Expected StoryList state"),
    }
}

#[test]
fn test_set_story_detail() {
    let mut app = create_test_app();
    let story = Story {
        id: 100,
        title: "Specific Story".to_string(),
        description: Some("Story description".to_string()),
        status: "active".to_string(),
        pri: 2,
        category: None,
        stage: None,
        product: 1,
        module: None,
        assigned_to: None,
        opened_by: None,
        estimate: None,
        version: None,
    };

    app.set_story_detail(story.clone(), Some("Test Product".to_string()));

    match &app.state {
        AppState::StoryDetail {
            story: returned_story,
            product_name,
        } => {
            assert_eq!(returned_story.id, 100);
            assert_eq!(returned_story.title, "Specific Story");
            assert_eq!(product_name.as_deref(), Some("Test Product"));
        }
        _ => panic!("Expected StoryDetail state"),
    }
}

#[test]
fn test_go_back_to_list_from_bug_detail() {
    let mut app = create_test_app();

    // First set a bug detail state
    let bug = Bug {
        id: 1,
        title: "Test".to_string(),
        description: None,
        status: "active".to_string(),
        severity: 3,
        pri: 3,
        type_: None,
        resolution: None,
        steps: None,
        product: 1,
        project: None,
        story: None,
        assigned_to: None,
        resolved_by: None,
        resolved_date: None,
    };
    app.state = AppState::BugDetail {
        bug,
        product_name: None,
    };
    app.selected_index = 5;

    // go_back_to_list should reset selected_index to 0
    app.go_back_to_list();
    assert_eq!(app.selected_index, 0);
}

#[test]
fn test_go_back_to_list_from_story_detail() {
    let mut app = create_test_app();

    let story = Story {
        id: 1,
        title: "Test".to_string(),
        description: None,
        status: "active".to_string(),
        pri: 3,
        category: None,
        stage: None,
        product: 1,
        module: None,
        assigned_to: None,
        opened_by: None,
        estimate: None,
        version: None,
    };
    app.state = AppState::StoryDetail {
        story,
        product_name: None,
    };
    app.selected_index = 10;

    app.go_back_to_list();
    assert_eq!(app.selected_index, 0);
}

#[test]
fn test_go_back_to_list_no_op_for_idle() {
    let mut app = create_test_app();
    app.selected_index = 5;

    // go_back_to_list should not change anything for Idle state
    app.go_back_to_list();
    assert_eq!(app.selected_index, 5);
}

#[test]
fn test_set_loading() {
    let mut app = create_test_app();
    app.set_loading("Loading bugs...".to_string());

    match &app.state {
        AppState::Loading { message } => {
            assert_eq!(message, "Loading bugs...");
        }
        _ => panic!("Expected Loading state"),
    }
    assert_eq!(app.selected_index, 0);
}

#[test]
fn test_set_error() {
    let mut app = create_test_app();
    app.set_error("Something went wrong".to_string());

    match &app.state {
        AppState::Error { message } => {
            assert_eq!(message, "Something went wrong");
        }
        _ => panic!("Expected Error state"),
    }
}

#[test]
fn test_quit() {
    let mut app = create_test_app();
    app.quit();

    match app.state {
        AppState::Quit => {}
        _ => panic!("Expected Quit state"),
    }
}

// ============================================================
// 辅助函数测试
// ============================================================

#[test]
fn test_get_selected_bug_id_in_bug_list() {
    let mut app = create_test_app();
    let bugs = vec![
        Bug {
            id: 10,
            title: "Bug 1".to_string(),
            description: None,
            status: "active".to_string(),
            severity: 3,
            pri: 3,
            type_: None,
            resolution: None,
            steps: None,
            product: 1,
            project: None,
            story: None,
            assigned_to: None,
            resolved_by: None,
            resolved_date: None,
        },
        Bug {
            id: 20,
            title: "Bug 2".to_string(),
            description: None,
            status: "active".to_string(),
            severity: 3,
            pri: 3,
            type_: None,
            resolution: None,
            steps: None,
            product: 1,
            project: None,
            story: None,
            assigned_to: None,
            resolved_by: None,
            resolved_date: None,
        },
    ];

    app.state = AppState::BugList {
        bugs,
        product_name: None,
    };
    app.selected_index = 1;

    assert_eq!(app.get_selected_bug_id(), Some(20));
}

#[test]
fn test_get_selected_bug_id_not_in_bug_list() {
    let app = create_test_app();
    // Not in BugList state
    assert_eq!(app.get_selected_bug_id(), None);
}

#[test]
fn test_get_selected_story_id_in_story_list() {
    let mut app = create_test_app();
    let stories = vec![
        Story {
            id: 100,
            title: "Story 1".to_string(),
            description: None,
            status: "active".to_string(),
            pri: 3,
            category: None,
            stage: None,
            product: 1,
            module: None,
            assigned_to: None,
            opened_by: None,
            estimate: None,
            version: None,
        },
        Story {
            id: 200,
            title: "Story 2".to_string(),
            description: None,
            status: "active".to_string(),
            pri: 3,
            category: None,
            stage: None,
            product: 1,
            module: None,
            assigned_to: None,
            opened_by: None,
            estimate: None,
            version: None,
        },
    ];

    app.state = AppState::StoryList {
        stories,
        product_name: None,
    };
    app.selected_index = 0;

    assert_eq!(app.get_selected_story_id(), Some(100));
}

#[test]
fn test_get_selected_story_id_not_in_story_list() {
    let app = create_test_app();
    // Not in StoryList state
    assert_eq!(app.get_selected_story_id(), None);
}

#[test]
fn test_get_selected_department_id_in_department_list() {
    let mut app = create_test_app();
    let departments = vec![
        Department {
            id: 1,
            name: "Engineering".to_string(),
            parent: None,
            order: Some(1),
            path: None,
        },
        Department {
            id: 2,
            name: "Sales".to_string(),
            parent: None,
            order: Some(2),
            path: None,
        },
    ];

    app.state = AppState::DepartmentList { departments };
    app.selected_index = 1;

    assert_eq!(app.get_selected_department_id(), Some(2));
}

#[test]
fn test_get_selected_department_id_not_in_department_list() {
    let app = create_test_app();
    // Not in DepartmentList state
    assert_eq!(app.get_selected_department_id(), None);
}

#[test]
fn test_get_main_menu_modules() {
    let modules = App::get_main_menu_modules();
    assert_eq!(modules.len(), 17);
    assert!(modules.contains(&"Bug List"));
    assert!(modules.contains(&"Story List"));
    assert!(modules.contains(&"Settings"));
}

#[test]
fn test_selected_index_bounds() {
    let mut app = create_test_app();
    let bugs = vec![Bug {
        id: 1,
        title: "Bug 1".to_string(),
        description: None,
        status: "active".to_string(),
        severity: 3,
        pri: 3,
        type_: None,
        resolution: None,
        steps: None,
        product: 1,
        project: None,
        story: None,
        assigned_to: None,
        resolved_by: None,
        resolved_date: None,
    }];

    app.state = AppState::BugList {
        bugs,
        product_name: None,
    };

    // With only 1 bug, selected_index 0 is valid
    assert_eq!(app.get_selected_bug_id(), Some(1));

    // selected_index out of bounds returns None
    app.selected_index = 10;
    assert_eq!(app.get_selected_bug_id(), None);
}

#[test]
fn test_is_quitting() {
    let mut app = create_test_app();

    // Idle is not quitting
    assert!(!app.state.is_quitting());

    // Quit state is quitting
    app.quit();
    assert!(app.state.is_quitting());
}

#[test]
fn test_set_department_list() {
    let mut app = create_test_app();
    let departments = vec![Department {
        id: 1,
        name: "HR".to_string(),
        parent: None,
        order: Some(1),
        path: None,
    }];

    app.set_department_list(departments.clone());

    match &app.state {
        AppState::DepartmentList { departments: depts } => {
            assert_eq!(depts.len(), 1);
            assert_eq!(depts[0].id, 1);
            assert_eq!(depts[0].name, "HR");
        }
        _ => panic!("Expected DepartmentList state"),
    }
}

#[test]
fn test_set_department_detail() {
    let mut app = create_test_app();
    let department = Department {
        id: 5,
        name: "Engineering".to_string(),
        parent: Some(1),
        order: Some(2),
        path: Some("1/5".to_string()),
    };

    app.set_department_detail(department.clone());

    match &app.state {
        AppState::DepartmentDetail { department: dept } => {
            assert_eq!(dept.id, 5);
            assert_eq!(dept.name, "Engineering");
            assert_eq!(dept.parent, Some(1));
        }
        _ => panic!("Expected DepartmentDetail state"),
    }
}
