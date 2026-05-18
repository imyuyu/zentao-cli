use super::app::App;

/// Calculate the maximum valid index for the current list state
fn get_max_index(app: &App) -> usize {
    match &app.state {
        AppState::BugList { bugs, .. } => bugs.len().saturating_sub(1),
        AppState::StoryList { stories, .. } => stories.len().saturating_sub(1),
        AppState::ExecutionList { executions, .. } => executions.len().saturating_sub(1),
        AppState::BuildList { builds, .. } => builds.len().saturating_sub(1),
        AppState::ReleaseList { releases, .. } => releases.len().saturating_sub(1),
        AppState::UserList { users } => users.len().saturating_sub(1),
        AppState::DepartmentList { departments } => departments.len().saturating_sub(1),
        AppState::ProductList { products } => products.len().saturating_sub(1),
        AppState::ProjectList { projects } => projects.len().saturating_sub(1),
        AppState::TaskList { tasks, .. } => tasks.len().saturating_sub(1),
        AppState::TestcaseList { testcases, .. } => testcases.len().saturating_sub(1),
        AppState::TesttaskList { testtasks, .. } => testtasks.len().saturating_sub(1),
        AppState::FeedbackList { feedbacks } => feedbacks.len().saturating_sub(1),
        AppState::TicketList { tickets } => tickets.len().saturating_sub(1),
        AppState::ProgramList { programs } => programs.len().saturating_sub(1),
        AppState::ProductPlanList { plans, .. } => plans.len().saturating_sub(1),
        AppState::Settings { multi_config, .. } => {
            multi_config.list_account_names().len().saturating_sub(1)
        }
        AppState::ProductSelect { products, .. } => products.len().saturating_sub(1),
        AppState::AccountSelect { multi_config, .. } => {
            multi_config.list_account_names().len().saturating_sub(1)
        }
        AppState::MainMenu { .. } => App::get_main_menu_modules().len().saturating_sub(1),
        _ => 0,
    }
}

/// Sync the list selection state with app.selected_index
fn sync_list_state(app: &App) {
    if matches!(app.state, AppState::MainMenu { .. }) {
        let _ = app.main_menu_state.borrow().selected();
        app.main_menu_state
            .borrow_mut()
            .select(Some(app.selected_index));
    } else {
        let _ = app.list_state.borrow().selected();
        app.list_state.borrow_mut().select(Some(app.selected_index));
    }
}

/// Handle navigation upward (Up key or 'k')
pub fn handle_navigation_up(app: &mut App) {
    app.selected_index = app.selected_index.saturating_sub(1);
    sync_list_state(app);
}

/// Handle navigation downward (Down key or 'j')
pub fn handle_navigation_down(app: &mut App) {
    let max = get_max_index(app);
    if app.selected_index < max {
        app.selected_index += 1;
    }
    sync_list_state(app);
}

/// Check if the current state supports list navigation
pub fn is_list_state(app: &App) -> bool {
    matches!(
        app.state,
        AppState::BugList { .. }
            | AppState::StoryList { .. }
            | AppState::ExecutionList { .. }
            | AppState::BuildList { .. }
            | AppState::ReleaseList { .. }
            | AppState::UserList { .. }
            | AppState::DepartmentList { .. }
            | AppState::ProductList { .. }
            | AppState::ProjectList { .. }
            | AppState::TaskList { .. }
            | AppState::TestcaseList { .. }
            | AppState::TesttaskList { .. }
            | AppState::FeedbackList { .. }
            | AppState::TicketList { .. }
            | AppState::ProgramList { .. }
            | AppState::ProductPlanList { .. }
            | AppState::Settings { .. }
            | AppState::ProductSelect { .. }
            | AppState::AccountSelect { .. }
            | AppState::MainMenu { .. }
    )
}

/// Handle 'o' key to open URL in browser - returns the URL if applicable
pub fn get_open_url(app: &App) -> Option<String> {
    let base_url = &app.config.url;

    match &app.state {
        AppState::BugList { bugs, .. } => bugs.get(app.selected_index).map(|b| b.web_url(base_url)),
        AppState::StoryList { stories, .. } => {
            stories.get(app.selected_index).map(|s| s.web_url(base_url))
        }
        AppState::ReleaseList { releases, .. } => releases
            .get(app.selected_index)
            .map(|r| r.web_url(base_url)),
        AppState::UserList { users } => users.get(app.selected_index).map(|u| u.web_url(base_url)),
        AppState::DepartmentList { departments } => departments
            .get(app.selected_index)
            .map(|d| d.web_url(base_url)),
        AppState::ProductList { products } => products
            .get(app.selected_index)
            .map(|p| format!("{}/product-view-{}.html", base_url, p.id)),
        AppState::ProjectList { projects } => projects
            .get(app.selected_index)
            .map(|p| format!("{}/project-view-{}.html", base_url, p.id)),
        AppState::TaskList { tasks, .. } => tasks
            .get(app.selected_index)
            .map(|t| format!("{}/task-view-{}.html", base_url, t.id)),
        AppState::TestcaseList { testcases, .. } => testcases
            .get(app.selected_index)
            .map(|t| format!("{}/testcase-view-{}.html", base_url, t.id)),
        AppState::TesttaskList { testtasks, .. } => testtasks
            .get(app.selected_index)
            .map(|t| format!("{}/testtask-view-{}.html", base_url, t.id)),
        AppState::FeedbackList { feedbacks } => feedbacks
            .get(app.selected_index)
            .map(|f| format!("{}/feedback-view-{}.html", base_url, f.id)),
        AppState::TicketList { tickets } => tickets
            .get(app.selected_index)
            .map(|t| format!("{}/ticket-view-{}.html", base_url, t.id)),
        AppState::ProgramList { programs } => programs
            .get(app.selected_index)
            .map(|p| format!("{}/program-view-{}.html", base_url, p.id)),
        AppState::ProductPlanList { plans, .. } => plans
            .get(app.selected_index)
            .map(|p| format!("{}/productplan-view-{}.html", base_url, p.id)),
        _ => None,
    }
}

// Re-export AppState for use in browser.rs
pub use super::app::AppState;
