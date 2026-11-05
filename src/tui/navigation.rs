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
        // Bug 详情页：有截图就打开第一张，否则打开 Bug 页面
        AppState::BugDetail { bug, .. } => {
            first_image_url(bug.steps.as_deref(), base_url).or_else(|| Some(bug.web_url(base_url)))
        }
        _ => None,
    }
}

/// 取 steps 里的第一张图片地址；禅道的图片 src 已是完整 URL
pub fn first_image_url(steps: Option<&str>, base_url: &str) -> Option<String> {
    let html = steps?;
    let lower = html.to_lowercase();
    let mut from = 0;
    while let Some(pos) = lower[from..].find("<img") {
        let start = from + pos;
        let end = lower[start..].find('>').map(|e| start + e + 1)?;
        if let Some(src) = super::browser::extract_attr(&html[start..end], "src") {
            if !src.trim().is_empty() {
                let alt =
                    super::browser::extract_attr(&html[start..end], "alt").unwrap_or_default();
                return Some(super::browser::resolve_img_src(&src, &alt, base_url));
            }
        }
        from = end;
    }
    None
}

// Re-export AppState for use in browser.rs
pub use super::app::AppState;

#[cfg(test)]
mod image_tests {
    use super::first_image_url;

    const BASE: &str = "http://z:8001/zentao";

    #[test]
    fn finds_full_url_as_is() {
        let html = r#"<img src="http://h/z/file-read-1.png" alt="x">"#;
        assert_eq!(
            first_image_url(Some(html), BASE).as_deref(),
            Some("http://h/z/file-read-1.png")
        );
    }

    /// 列表接口返回占位符 src + 真实文件名 alt，必须还原成可打开的地址
    #[test]
    fn resolves_placeholder_src_from_alt() {
        let html = r#"<img src="{20220.png}" alt="file-read-20220.png" />"#;
        assert_eq!(
            first_image_url(Some(html), BASE).as_deref(),
            Some("http://z:8001/zentao/file-read-20220.png")
        );
    }

    /// 没有 alt 时按 {id.ext} 规则拼
    #[test]
    fn resolves_placeholder_without_alt() {
        let html = r#"<img src="{20094.gif}">"#;
        assert_eq!(
            first_image_url(Some(html), BASE).as_deref(),
            Some("http://z:8001/zentao/file-read-20094.gif")
        );
    }

    #[test]
    fn returns_none_when_no_image() {
        assert_eq!(first_image_url(Some("<p>纯文本</p>"), BASE), None);
        assert_eq!(first_image_url(None, BASE), None);
    }
}
