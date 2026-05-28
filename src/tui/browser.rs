use anyhow::Result;
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Wrap},
    Terminal,
};
use std::io::Stdout;
use std::sync::mpsc;

use super::app::{App, AppState};
use super::navigation::{get_open_url, handle_navigation_down, handle_navigation_up};
use crate::api::{
    Bug, Department, Execution, Feedback, Product, ProductPlan, Program, Project, Release, Story,
    Task, Testcase, Testtask, Ticket, User,
};
use crate::core::{AppContext, OutputFormat};
use crate::service::{
    bug::BugService, build::BuildService, department::DepartmentService,
    execution::ExecutionService, feedback::FeedbackService, product::ProductService,
    productplan::ProductPlanService, program::ProgramService, project::ProjectService,
    release::ReleaseService, story::StoryService, task::TaskService, testcase::TestcaseService,
    testtask::TesttaskService, ticket::TicketService, user::UserService,
};
use crate::tui::forms::{ProgramFormFields, ProductPlanFormFields, ReleaseFormFields};

use super::pages::*;

// Strip HTML tags from a string
pub fn strip_html_tags(html: &str) -> String {
    let mut result = String::with_capacity(html.len());
    let mut in_tag = false;
    let mut in_entity = false;
    let mut entity_buf = String::new();
    let mut tag_buf = String::new();

    let chars: Vec<char> = html.chars().collect();
    let len = chars.len();
    let mut i = 0;

    while i < len {
        let c = chars[i];

        if in_tag {
            tag_buf.push(c);
            if c == '>' {
                in_tag = false;
                // Check if it's a <br> tag and convert to newline
                let tag_lower = tag_buf.to_lowercase();
                if tag_lower.trim() == "br"
                    || tag_lower.trim() == "br/>"
                    || tag_lower.trim() == "br /"
                {
                    result.push('\n');
                }
                tag_buf.clear();
            }
        } else if in_entity {
            entity_buf.push(c);
            if c == ';' {
                in_entity = false;
                // Decode common HTML entities
                match entity_buf.as_str() {
                    "nbsp" => result.push(' '),
                    "lt" | "LT" => result.push('<'),
                    "gt" | "GT" => result.push('>'),
                    "amp" | "AMP" => result.push('&'),
                    "quot" | "QUOT" => result.push('"'),
                    "apos" | "APOS" => result.push('\''),
                    "&" => result.push('&'),
                    _ => {
                        // Unknown entity, keep as-is
                        result.push_str(&entity_buf);
                    }
                }
                entity_buf.clear();
            }
        } else if c == '<' {
            in_tag = true;
        } else if c == '&' {
            in_entity = true;
            entity_buf.clear();
            entity_buf.push(c);
        } else {
            result.push(c);
        }

        i += 1;
    }

    // Handle any remaining entity without semicolon
    if in_entity && !entity_buf.is_empty() {
        if entity_buf == "&nbsp" {
            result.push(' ');
        } else if entity_buf != "&" {
            result.push_str(&entity_buf);
        }
    }

    result
}

#[allow(clippy::enum_variant_names)]
enum EnterAction {
    BugDetail {
        bug: Bug,
        product_name: Option<String>,
    },
    BugCreate,
    BugUpdate {
        id: u64,
    },
    BugDelete {
        id: u64,
        #[allow(dead_code)]
        name: String,
    },
    StoryDetail {
        story: Story,
        product_name: Option<String>,
    },
    ExecutionDetail {
        execution: Execution,
        project_name: Option<String>,
    },
    ReleaseDetail {
        release: Release,
        product_name: Option<String>,
    },
    UserDetail {
        user: User,
    },
    DepartmentDetail {
        dept: Department,
    },
    ProductDetail {
        product: Product,
    },
    ProjectDetail {
        project: Project,
    },
    TaskDetail {
        task: Task,
    },
    TestcaseDetail {
        tc: Testcase,
    },
    TesttaskDetail {
        tt: Testtask,
    },
    FeedbackDetail {
        fb: Feedback,
    },
    TicketDetail {
        ticket: Ticket,
    },
    ProgramDetail {
        program: Program,
    },
    ProductPlanDetail {
        plan: ProductPlan,
    },
    DeleteProgram(u64),
    DeleteProductPlan(u64),
    DeleteRelease(u64),
    CreateProgram,
    UpdateProgram,
    CreateProductPlan,
    UpdateProductPlan,
    CreateRelease,
    UpdateRelease,
}

enum FormSubmitResult {
    BugCreateSuccess(Bug),
    BugCreateError(String),
    BugUpdateSuccess(Bug),
    BugUpdateError(String),
    BugDeleteSuccess(u64),
    BugDeleteError(String),
}

#[allow(clippy::type_complexity)]
pub struct Browser {
    terminal: Terminal<CrosstermBackend<Stdout>>,
    pending_products: Option<Vec<Product>>,
    pending_reload: Option<Box<dyn FnOnce(&mut App)>>,
    pending_form_submit: Option<FormSubmitAction>,
    spinner_frame: usize,
    loading_cancelled: bool,
    loading_module: Option<String>,
    async_handle: Option<AsyncHandle>,
}

// Reusable runtime and form_tx for use in handle_key_event
struct AsyncHandle {
    rt: std::sync::Arc<tokio::runtime::Runtime>,
    form_tx: std::sync::Arc<mpsc::Sender<FormSubmitResult>>,
}

pub enum FormSubmitAction {
    DeleteProgram(u64),
    DeleteProductPlan(u64),
    DeleteRelease(u64),
    CreateProgram(ProgramFormFields),
    UpdateProgram(u64, ProgramFormFields),
    CreateProductPlan(u64, ProductPlanFormFields),
    UpdateProductPlan(u64, ProductPlanFormFields),
    CreateRelease(Option<u64>, ReleaseFormFields),
    UpdateRelease(u64, ReleaseFormFields),
}

impl Browser {
    pub fn new() -> Result<Self> {
        let stdout = std::io::stdout();
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        // Clear the terminal first to ensure clean state (required for Clear widget to work)
        terminal.clear()?;

        // Enter alternate screen to avoid terminal buffer conflicts
        crossterm::execute!(
            terminal.backend_mut(),
            crossterm::terminal::EnterAlternateScreen
        )?;

        terminal.hide_cursor()?;
        Ok(Self {
            terminal,
            pending_products: None,
            pending_reload: None,
            pending_form_submit: None,
            spinner_frame: 0,
            loading_cancelled: false,
            loading_module: None,
            async_handle: None,
        })
    }

    pub fn run(&mut self, app: &mut App) -> Result<()> {
        let rt = tokio::runtime::Runtime::new()?;
        let rt_arc = std::sync::Arc::new(rt);
        let (tx, rx) = mpsc::channel::<(AppState, Option<String>, Option<String>)>();
        let (form_tx, form_rx) = mpsc::channel::<FormSubmitResult>();
        let async_handle = AsyncHandle {
            rt: rt_arc.clone(),
            form_tx: std::sync::Arc::new(form_tx.clone()),
        };
        self.async_handle = Some(async_handle);
        let rt = rt_arc;

        // Drain ALL pending stdin events before entering main loop
        // This fixes PowerShell's ConPTY stdin buffering issue
        for _ in 0..200 {
            match crossterm::event::poll(std::time::Duration::from_millis(0)) {
                Ok(true) => {
                    let _ = crossterm::event::read();
                }
                Ok(false) | Err(_) => break,
            }
        }

        // Wait for terminal to fully settle (fixes PowerShell ConPTY event timing)
        std::thread::sleep(std::time::Duration::from_millis(500));

        // Drain any events that arrived during the wait
        for _ in 0..50 {
            match crossterm::event::poll(std::time::Duration::from_millis(0)) {
                Ok(true) => {
                    let _ = crossterm::event::read();
                }
                Ok(false) | Err(_) => break,
            }
        }

        loop {
            if app.state.is_quitting() {
                break;
            }

            // Check for async loading results (skip if loading was cancelled or user already back to MainMenu)
            let is_main_menu = matches!(app.state, AppState::MainMenu { .. });
            if !self.loading_cancelled && !is_main_menu {
                // Handle pending form submissions
                if let Some(action) = self.pending_form_submit.take() {
                    let config = app.config.clone();
                    let tx = tx.clone();
                    rt.spawn(async move {
                        let ctx = AppContext::new(config.clone(), OutputFormat::Table, false);
                        match action {
                            FormSubmitAction::DeleteProgram(id) => {
                                match ProgramService::delete(&ctx, id).await {
                                    Ok(_) => {
                                        let _ = tx.send((AppState::MainMenu { selected: 0 }, None, None));
                                    }
                                    Err(e) => {
                                        let _ = tx.send((AppState::Error { message: e.to_string() }, None, None));
                                    }
                                }
                            }
                            FormSubmitAction::DeleteProductPlan(id) => {
                                match ProductPlanService::delete(&ctx, id).await {
                                    Ok(_) => {
                                        let _ = tx.send((AppState::MainMenu { selected: 0 }, None, None));
                                    }
                                    Err(e) => {
                                        let _ = tx.send((AppState::Error { message: e.to_string() }, None, None));
                                    }
                                }
                            }
                            FormSubmitAction::DeleteRelease(id) => {
                                match ReleaseService::delete(&ctx, id).await {
                                    Ok(_) => {
                                        let _ = tx.send((AppState::MainMenu { selected: 0 }, None, None));
                                    }
                                    Err(e) => {
                                        let _ = tx.send((AppState::Error { message: e.to_string() }, None, None));
                                    }
                                }
                            }
                            FormSubmitAction::CreateProgram(fields) => {
                                match ProgramService::create(&ctx, fields.to_create_request()).await {
                                    Ok(_) => {
                                        let _ = tx.send((AppState::MainMenu { selected: 0 }, None, None));
                                    }
                                    Err(e) => {
                                        let _ = tx.send((AppState::Error { message: e.to_string() }, None, None));
                                    }
                                }
                            }
                            FormSubmitAction::UpdateProgram(id, fields) => {
                                match ProgramService::update(&ctx, id, fields.to_update_request()).await {
                                    Ok(_) => {
                                        let _ = tx.send((AppState::MainMenu { selected: 0 }, None, None));
                                    }
                                    Err(e) => {
                                        let _ = tx.send((AppState::Error { message: e.to_string() }, None, None));
                                    }
                                }
                            }
                            FormSubmitAction::CreateProductPlan(product_id, fields) => {
                                match ProductPlanService::create(&ctx, product_id, fields.to_create_request(product_id)).await {
                                    Ok(_) => {
                                        let _ = tx.send((AppState::MainMenu { selected: 0 }, None, None));
                                    }
                                    Err(e) => {
                                        let _ = tx.send((AppState::Error { message: e.to_string() }, None, None));
                                    }
                                }
                            }
                            FormSubmitAction::UpdateProductPlan(id, fields) => {
                                match ProductPlanService::update(&ctx, id, fields.to_update_request()).await {
                                    Ok(_) => {
                                        let _ = tx.send((AppState::MainMenu { selected: 0 }, None, None));
                                    }
                                    Err(e) => {
                                        let _ = tx.send((AppState::Error { message: e.to_string() }, None, None));
                                    }
                                }
                            }
                            FormSubmitAction::CreateRelease(product_id, fields) => {
                                match ReleaseService::create(&ctx, fields.to_create_request(product_id)).await {
                                    Ok(_) => {
                                        let _ = tx.send((AppState::MainMenu { selected: 0 }, None, None));
                                    }
                                    Err(e) => {
                                        let _ = tx.send((AppState::Error { message: e.to_string() }, None, None));
                                    }
                                }
                            }
                            FormSubmitAction::UpdateRelease(id, fields) => {
                                match ReleaseService::update(&ctx, id, fields.to_update_request()).await {
                                    Ok(_) => {
                                        let _ = tx.send((AppState::MainMenu { selected: 0 }, None, None));
                                    }
                                    Err(e) => {
                                        let _ = tx.send((AppState::Error { message: e.to_string() }, None, None));
                                    }
                                }
                            }
                        }
                    });
                }

                if let Ok((new_state, product_name, project_name)) = rx.try_recv() {
                    // Only accept the result if we're still loading the same module
                    let state_module_name = match &new_state {
                        AppState::BugList { .. } => Some("Bug List"),
                        AppState::StoryList { .. } => Some("Story List"),
                        AppState::ProjectList { .. } => Some("Project List"),
                        AppState::TaskList { .. } => Some("Task List"),
                        AppState::ExecutionList { .. } => Some("Execution List"),
                        AppState::BuildList { .. } => Some("Build List"),
                        AppState::ReleaseList { .. } => Some("Release List"),
                        AppState::UserList { .. } => Some("User List"),
                        AppState::DepartmentList { .. } => Some("Department List"),
                        AppState::ProductList { .. } => Some("Product List"),
                        AppState::ProductPlanList { .. } => Some("ProductPlan List"),
                        AppState::TestcaseList { .. } => Some("Testcase List"),
                        AppState::TesttaskList { .. } => Some("Testtask List"),
                        AppState::FeedbackList { .. } => Some("Feedback List"),
                        AppState::TicketList { .. } => Some("Ticket List"),
                        AppState::ProgramList { .. } => Some("Program List"),
                        AppState::Error { .. } => None, // Error states are always valid
                        _ => None,
                    };

                    // Skip stale results from previous module loads
                    if let Some(name) = state_module_name {
                        if self.loading_module.as_ref() != Some(&name.to_string()) {
                            // This is a stale result, skip it
                            self.loading_module = None;
                            continue;
                        }
                    }
                    self.loading_module = None;

                    // Update state with the loaded data
                    app.state = new_state;
                    app.selected_index = 0;
                    app.list_state.borrow_mut().select(Some(0));
                    if let Some(name) = product_name {
                        if let AppState::BugList {
                            ref mut product_name,
                            ..
                        } = app.state
                        {
                            *product_name = Some(name);
                        } else if let AppState::StoryList {
                            ref mut product_name,
                            ..
                        } = app.state
                        {
                            *product_name = Some(name);
                        } else if let AppState::ReleaseList {
                            ref mut product_name,
                            ..
                        } = app.state
                        {
                            *product_name = Some(name);
                        } else if let AppState::TestcaseList {
                            ref mut product_name,
                            ..
                        } = app.state
                        {
                            *product_name = Some(name);
                        }
                    }
                    if let Some(name) = project_name {
                        if let AppState::ExecutionList {
                            ref mut project_name,
                            ..
                        } = app.state
                        {
                            *project_name = Some(name);
                        } else if let AppState::BuildList {
                            ref mut project_name,
                            ..
                        } = app.state
                        {
                            *project_name = Some(name);
                        } else if let AppState::TaskList {
                            ref mut project_name,
                            ..
                        } = app.state
                        {
                            *project_name = Some(name);
                        } else if let AppState::TesttaskList {
                            ref mut project_name,
                            ..
                        } = app.state
                        {
                            *project_name = Some(name);
                        } else if let AppState::ProductPlanList {
                            ref mut product_name,
                            ..
                        } = app.state
                        {
                            *product_name = Some(name);
                        }
                    }
                }
                self.loading_cancelled = false;
            }

            // Check for form submission results
            if let Ok(result) = form_rx.try_recv() {
                match result {
                    FormSubmitResult::BugCreateSuccess(bug) => {
                        app.set_bug_detail(bug, None);
                    }
                    FormSubmitResult::BugCreateError(msg) => {
                        if let AppState::BugCreate { ref mut error, .. } = &mut app.state {
                            *error = Some(msg);
                        }
                    }
                    FormSubmitResult::BugUpdateSuccess(bug) => {
                        app.set_bug_detail(bug, None);
                    }
                    FormSubmitResult::BugUpdateError(msg) => {
                        if let AppState::BugUpdate { ref mut error, .. } = &mut app.state {
                            *error = Some(msg);
                        }
                    }
                    FormSubmitResult::BugDeleteSuccess(_id) => {
                        // Reload the bug list
                        app.restore_list();
                        app.set_module_selected("Bug List".to_string());
                    }
                    FormSubmitResult::BugDeleteError(msg) => {
                        app.set_error(format!("Delete failed: {}", msg));
                    }
                }
            }

            let selected = app.selected_index;
            let current_module: Option<String> =
                if matches!(app.state, AppState::ModuleSelected { .. }) {
                    if let AppState::ModuleSelected { module_name } = &app.state {
                        let name = module_name.clone();
                        app.state = AppState::Loading {
                            message: format!("Loading {}...", name),
                        };
                        self.loading_cancelled = false;
                        self.loading_module = Some(name.clone());
                        Some(name)
                    } else {
                        None
                    }
                } else {
                    None
                };

            // Update spinner frame for loading animation
            if matches!(app.state, AppState::Loading { .. }) {
                self.spinner_frame = (self.spinner_frame + 1) % 4;
            }

            self.terminal.draw(|f| {
                let area = f.size();
                // Clear the screen before rendering to avoid residual elements
                f.render_widget(Clear, area);

                if app.help_visible {
                    Self::render_help_overlay(f, area);
                } else {
                    match &app.state {
                        AppState::Idle => {
                            Self::render_idle(f, area, app);
                        }
                        AppState::Loading { message } => {
                            Self::render_loading(f, area, message, self.spinner_frame);
                        }
                        AppState::BugList { bugs, .. } => {
                            render_bug_list(f, area, bugs, selected, app);
                        }
                        AppState::BugDetail { bug, .. } => {
                            render_bug_detail(f, area, bug);
                        }
                        AppState::BugCreate {
                            fields,
                            field_order,
                            focused_field,
                            error,
                        } => {
                            Self::render_bug_form(
                                f,
                                area,
                                fields,
                                field_order,
                                *focused_field,
                                error.as_deref(),
                                true,
                            );
                        }
                        AppState::BugUpdate {
                            fields,
                            field_order,
                            focused_field,
                            error,
                            ..
                        } => {
                            Self::render_bug_form(
                                f,
                                area,
                                fields,
                                field_order,
                                *focused_field,
                                error.as_deref(),
                                false,
                            );
                        }
                        AppState::BugDelete { id, name, confirm } => {
                            Self::render_delete_dialog(f, area, "Bug", *id, name.clone(), *confirm);
                        }
                        AppState::StoryList { stories, .. } => {
                            render_story_list(f, area, stories, selected, app);
                        }
                        AppState::StoryDetail { story, .. } => {
                            render_story_detail(f, area, story);
                        }
                        AppState::ExecutionList { executions, .. } => {
                            render_execution_list(f, area, executions, selected, app);
                        }
                        AppState::ExecutionDetail { execution, .. } => {
                            render_execution_detail(f, area, execution);
                        }
                        AppState::BuildList { builds, .. } => {
                            render_build_list(f, area, builds, selected, app);
                        }
                        AppState::BuildDetail { build, .. } => {
                            render_build_detail(f, area, build);
                        }
                        AppState::ReleaseList { releases, .. } => {
                            render_release_list(f, area, releases, selected, app);
                        }
                        AppState::ReleaseDetail { release, .. } => {
                            render_release_detail(f, area, release);
                        }
                        AppState::UserList { users, .. } => {
                            render_user_list(f, area, users, selected, app);
                        }
                        AppState::UserDetail { user, .. } => {
                            render_user_detail(f, area, user);
                        }
                        AppState::DepartmentList { departments, .. } => {
                            render_department_list(f, area, departments, selected, app);
                        }
                        AppState::DepartmentDetail { department, .. } => {
                            render_department_detail(f, area, department);
                        }
                        AppState::ProductList { products, .. } => {
                            render_product_list(f, area, products, selected, app);
                        }
                        AppState::ProductDetail { product, .. } => {
                            render_product_detail(f, area, product);
                        }
                        AppState::ProjectList { projects, .. } => {
                            render_project_list(f, area, projects, selected, app);
                        }
                        AppState::ProjectDetail { project, .. } => {
                            render_project_detail(f, area, project);
                        }
                        AppState::TaskList { tasks, .. } => {
                            render_task_list(f, area, tasks, selected, app);
                        }
                        AppState::TaskDetail { task, .. } => {
                            render_task_detail(f, area, task);
                        }
                        AppState::TestcaseList { testcases, .. } => {
                            render_testcase_list(f, area, testcases, selected, app);
                        }
                        AppState::TestcaseDetail { testcase, .. } => {
                            render_testcase_detail(f, area, testcase);
                        }
                        AppState::TesttaskList { testtasks, .. } => {
                            render_testtask_list(f, area, testtasks, selected, app);
                        }
                        AppState::TesttaskDetail { testtask, .. } => {
                            render_testtask_detail(f, area, testtask);
                        }
                        AppState::FeedbackList { feedbacks, .. } => {
                            render_feedback_list(f, area, feedbacks, selected, app);
                        }
                        AppState::FeedbackDetail { feedback, .. } => {
                            render_feedback_detail(f, area, feedback);
                        }
                        AppState::TicketList { tickets, .. } => {
                            render_ticket_list(f, area, tickets, selected, app);
                        }
                        AppState::TicketDetail { ticket, .. } => {
                            render_ticket_detail(f, area, ticket);
                        }
                        AppState::ProgramList { programs, .. } => {
                            render_program_list(f, area, programs, selected, app);
                        }
                        AppState::ProgramDetail { program, .. } => {
                            render_program_detail(f, area, program);
                        }
                        AppState::ProductPlanList { plans, .. } => {
                            render_productplan_list(f, area, plans, selected, app);
                        }
                        AppState::ProductPlanDetail { plan, .. } => {
                            render_productplan_detail(f, area, plan);
                        }
                        AppState::ProgramCreate { .. } => {
                            render_program_create(f, area, app);
                        }
                        AppState::ProgramUpdate { .. } => {
                            render_program_update(f, area, app);
                        }
                        AppState::ProgramDelete { .. } => {
                            render_delete_dialog(f, area, app);
                        }
                        AppState::ProductPlanCreate { .. } => {
                            render_productplan_create(f, area, app);
                        }
                        AppState::ProductPlanUpdate { .. } => {
                            render_productplan_update(f, area, app);
                        }
                        AppState::ProductPlanDelete { .. } => {
                            render_delete_dialog(f, area, app);
                        }
                        AppState::ReleaseCreate { .. } => {
                            render_release_create(f, area, app);
                        }
                        AppState::ReleaseUpdate { .. } => {
                            render_release_update(f, area, app);
                        }
                        AppState::ReleaseDelete { .. } => {
                            render_delete_dialog(f, area, app);
                        }
                        AppState::FormSubmitting { message } => {
                            Self::render_loading(f, area, message, self.spinner_frame);
                        }
                        AppState::Error { message } => {
                            Self::render_error(f, area, message);
                        }
                        AppState::Quit => {}
                        AppState::Settings {
                            multi_config,
                            selected,
                            current_account,
                        } => {
                            Self::render_settings(
                                f,
                                area,
                                multi_config,
                                *selected,
                                current_account,
                                app,
                            );
                        }
                        AppState::ProductSelect {
                            products,
                            selected,
                            loading,
                        } => {
                            Self::render_product_select(f, area, products, *selected, *loading);
                        }
                        AppState::AccountSelect {
                            multi_config,
                            selected,
                        } => {
                            Self::render_account_select(f, area, multi_config, *selected, app);
                        }
                        AppState::MainMenu { .. } => {
                            Self::render_main_menu(f, area, app.selected_index, app);
                        }
                        AppState::ConfirmQuit => {
                            Self::render_confirm_quit(f, area);
                        }
                        AppState::ModuleSelected { module_name } => {
                            Self::render_loading(
                                f,
                                area,
                                &format!("Loading {}...", module_name),
                                self.spinner_frame,
                            );
                        }
                    }
                }
            })?;

            // Handle async loading for ModuleSelected
            if let Some(module_name) = current_module {
                let config = app.config.clone();
                let tx = tx.clone();
                rt.spawn(async move {
                    let mut ctx = AppContext::new(config.clone(), OutputFormat::Table, false);

                    let result = match module_name.as_str() {
                        "Bug List" => {
                            let bugs = match BugService::list(
                                &ctx,
                                config.product_id,
                                Some("active".to_string()),
                                None,
                            )
                            .await
                            {
                                Ok(b) => Ok(b),
                                Err(e) => {
                                    eprintln!("Error loading bugs (trying token refresh): {}", e);
                                    if ctx.refresh_token().await.is_ok() {
                                        BugService::list(
                                            &ctx,
                                            config.product_id,
                                            Some("active".to_string()),
                                            None,
                                        )
                                        .await
                                    } else {
                                        Err(e)
                                    }
                                }
                            };
                            match bugs {
                                Ok(bugs) => {
                                    let product_name = if let Some(id) = config.product_id {
                                        ProductService::get_name(&ctx, id).await.ok()
                                    } else {
                                        None
                                    };
                                    let pn = product_name.clone();
                                    (AppState::BugList { bugs, product_name }, pn, None)
                                }
                                Err(e) => (
                                    AppState::Error {
                                        message: format!("Failed to load bugs: {}", e),
                                    },
                                    None,
                                    None,
                                ),
                            }
                        }
                        "Story List" => {
                            let stories = match StoryService::list(
                                &ctx,
                                config.product_id,
                                config.project_id,
                                None,
                            )
                            .await
                            {
                                Ok(s) => Ok(s),
                                Err(e) => {
                                    eprintln!(
                                        "Error loading stories (trying token refresh): {}",
                                        e
                                    );
                                    if ctx.refresh_token().await.is_ok() {
                                        StoryService::list(
                                            &ctx,
                                            config.product_id,
                                            config.project_id,
                                            None,
                                        )
                                        .await
                                    } else {
                                        Err(e)
                                    }
                                }
                            };
                            match stories {
                                Ok(stories) => {
                                    let product_name = if let Some(id) = config.product_id {
                                        ProductService::get_name(&ctx, id).await.ok()
                                    } else {
                                        None
                                    };
                                    let pn = product_name.clone();
                                    (
                                        AppState::StoryList {
                                            stories,
                                            product_name,
                                        },
                                        pn,
                                        None,
                                    )
                                }
                                Err(e) => (
                                    AppState::Error {
                                        message: format!("Failed to load stories: {}", e),
                                    },
                                    None,
                                    None,
                                ),
                            }
                        }
                        "Execution List" => {
                            let executions =
                                match ExecutionService::list(&ctx, config.project_id).await {
                                    Ok(e) => Ok(e),
                                    Err(e) => {
                                        eprintln!(
                                            "Error loading executions (trying token refresh): {}",
                                            e
                                        );
                                        if ctx.refresh_token().await.is_ok() {
                                            ExecutionService::list(&ctx, config.project_id).await
                                        } else {
                                            Err(e)
                                        }
                                    }
                                };
                            match executions {
                                Ok(executions) => {
                                    let project_name = if let Some(id) = config.project_id {
                                        ProjectService::get_name(&ctx, id).await.ok()
                                    } else {
                                        None
                                    };
                                    let pn = project_name.clone();
                                    (
                                        AppState::ExecutionList {
                                            executions,
                                            project_name,
                                        },
                                        None,
                                        pn,
                                    )
                                }
                                Err(e) => (
                                    AppState::Error {
                                        message: format!("Failed to load executions: {}", e),
                                    },
                                    None,
                                    None,
                                ),
                            }
                        }
                        "Build List" => {
                            let builds = match BuildService::list(
                                &ctx,
                                config.project_id,
                                config.product_id,
                                None,
                            )
                            .await
                            {
                                Ok(b) => Ok(b),
                                Err(e) => {
                                    eprintln!("Error loading builds (trying token refresh): {}", e);
                                    if ctx.refresh_token().await.is_ok() {
                                        BuildService::list(
                                            &ctx,
                                            config.project_id,
                                            config.product_id,
                                            None,
                                        )
                                        .await
                                    } else {
                                        Err(e)
                                    }
                                }
                            };
                            match builds {
                                Ok(builds) => {
                                    let product_name = if let Some(id) = config.product_id {
                                        ProductService::get_name(&ctx, id).await.ok()
                                    } else {
                                        None
                                    };
                                    let project_name = if let Some(id) = config.project_id {
                                        ProjectService::get_name(&ctx, id).await.ok()
                                    } else {
                                        None
                                    };
                                    let pn = product_name.clone();
                                    let pjn = project_name.clone();
                                    (
                                        AppState::BuildList {
                                            builds,
                                            product_name,
                                            project_name,
                                        },
                                        pn,
                                        pjn,
                                    )
                                }
                                Err(e) => (
                                    AppState::Error {
                                        message: format!("Failed to load builds: {}", e),
                                    },
                                    None,
                                    None,
                                ),
                            }
                        }
                        "Release List" => {
                            let releases = match ReleaseService::list(
                                &ctx,
                                config.product_id,
                                config.project_id,
                            )
                            .await
                            {
                                Ok(r) => Ok(r),
                                Err(e) => {
                                    eprintln!(
                                        "Error loading releases (trying token refresh): {}",
                                        e
                                    );
                                    if ctx.refresh_token().await.is_ok() {
                                        ReleaseService::list(
                                            &ctx,
                                            config.product_id,
                                            config.project_id,
                                        )
                                        .await
                                    } else {
                                        Err(e)
                                    }
                                }
                            };
                            match releases {
                                Ok(releases) => {
                                    let product_name = if let Some(id) = config.product_id {
                                        ProductService::get_name(&ctx, id).await.ok()
                                    } else {
                                        None
                                    };
                                    let pn = product_name.clone();
                                    (
                                        AppState::ReleaseList {
                                            releases,
                                            product_name,
                                        },
                                        pn,
                                        None,
                                    )
                                }
                                Err(e) => (
                                    AppState::Error {
                                        message: format!("Failed to load releases: {}", e),
                                    },
                                    None,
                                    None,
                                ),
                            }
                        }
                        "User List" => match UserService::list(&ctx, None, None).await {
                            Ok(users) => (AppState::UserList { users }, None, None),
                            Err(e) => {
                                eprintln!("Error loading users (trying token refresh): {}", e);
                                if ctx.refresh_token().await.is_ok() {
                                    match UserService::list(&ctx, None, None).await {
                                        Ok(users) => (AppState::UserList { users }, None, None),
                                        Err(_) => (
                                            AppState::Error {
                                                message: format!("Failed to load users: {}", e),
                                            },
                                            None,
                                            None,
                                        ),
                                    }
                                } else {
                                    (
                                        AppState::Error {
                                            message: format!("Failed to load users: {}", e),
                                        },
                                        None,
                                        None,
                                    )
                                }
                            }
                        },
                        "Department List" => match DepartmentService::list(&ctx).await {
                            Ok(departments) => {
                                (AppState::DepartmentList { departments }, None, None)
                            }
                            Err(e) => {
                                eprintln!(
                                    "Error loading departments (trying token refresh): {}",
                                    e
                                );
                                if ctx.refresh_token().await.is_ok() {
                                    match DepartmentService::list(&ctx).await {
                                        Ok(departments) => {
                                            (AppState::DepartmentList { departments }, None, None)
                                        }
                                        Err(_) => (
                                            AppState::Error {
                                                message: format!(
                                                    "Failed to load departments: {}",
                                                    e
                                                ),
                                            },
                                            None,
                                            None,
                                        ),
                                    }
                                } else {
                                    (
                                        AppState::Error {
                                            message: format!("Failed to load departments: {}", e),
                                        },
                                        None,
                                        None,
                                    )
                                }
                            }
                        },
                        "Product List" => match ProductService::list(&ctx).await {
                            Ok(products) => (AppState::ProductList { products }, None, None),
                            Err(e) => {
                                eprintln!("Error loading products (trying token refresh): {}", e);
                                if ctx.refresh_token().await.is_ok() {
                                    match ProductService::list(&ctx).await {
                                        Ok(products) => {
                                            (AppState::ProductList { products }, None, None)
                                        }
                                        Err(_) => (
                                            AppState::Error {
                                                message: format!("Failed to load products: {}", e),
                                            },
                                            None,
                                            None,
                                        ),
                                    }
                                } else {
                                    (
                                        AppState::Error {
                                            message: format!("Failed to load products: {}", e),
                                        },
                                        None,
                                        None,
                                    )
                                }
                            }
                        },
                        "Project List" => match ProjectService::list(&ctx).await {
                            Ok(projects) => (AppState::ProjectList { projects }, None, None),
                            Err(e) => {
                                eprintln!("Error loading projects (trying token refresh): {}", e);
                                if ctx.refresh_token().await.is_ok() {
                                    match ProjectService::list(&ctx).await {
                                        Ok(projects) => {
                                            (AppState::ProjectList { projects }, None, None)
                                        }
                                        Err(_) => (
                                            AppState::Error {
                                                message: format!("Failed to load projects: {}", e),
                                            },
                                            None,
                                            None,
                                        ),
                                    }
                                } else {
                                    (
                                        AppState::Error {
                                            message: format!("Failed to load projects: {}", e),
                                        },
                                        None,
                                        None,
                                    )
                                }
                            }
                        },
                        "Task List" => {
                            let tasks = match TaskService::list(&ctx, config.project_id, None).await
                            {
                                Ok(t) => Ok(t),
                                Err(e) => {
                                    eprintln!("Error loading tasks (trying token refresh): {}", e);
                                    if ctx.refresh_token().await.is_ok() {
                                        TaskService::list(&ctx, config.project_id, None).await
                                    } else {
                                        Err(e)
                                    }
                                }
                            };
                            match tasks {
                                Ok(tasks) => {
                                    let project_name = if let Some(id) = config.project_id {
                                        ProjectService::get_name(&ctx, id).await.ok()
                                    } else {
                                        None
                                    };
                                    let pn = project_name.clone();
                                    (
                                        AppState::TaskList {
                                            tasks,
                                            project_name,
                                        },
                                        None,
                                        pn,
                                    )
                                }
                                Err(e) => (
                                    AppState::Error {
                                        message: format!("Failed to load tasks: {}", e),
                                    },
                                    None,
                                    None,
                                ),
                            }
                        }
                        "Testcase List" => {
                            let testcases = match TestcaseService::list(
                                &ctx,
                                config.product_id,
                                None,
                                None,
                                None,
                            )
                            .await
                            {
                                Ok(tc) => Ok(tc),
                                Err(e) => {
                                    eprintln!(
                                        "Error loading testcases (trying token refresh): {}",
                                        e
                                    );
                                    if ctx.refresh_token().await.is_ok() {
                                        TestcaseService::list(
                                            &ctx,
                                            config.product_id,
                                            None,
                                            None,
                                            None,
                                        )
                                        .await
                                    } else {
                                        Err(e)
                                    }
                                }
                            };
                            match testcases {
                                Ok(testcases) => {
                                    let product_name = if let Some(id) = config.product_id {
                                        ProductService::get_name(&ctx, id).await.ok()
                                    } else {
                                        None
                                    };
                                    let pn = product_name.clone();
                                    (
                                        AppState::TestcaseList {
                                            testcases,
                                            product_name,
                                        },
                                        pn,
                                        None,
                                    )
                                }
                                Err(e) => (
                                    AppState::Error {
                                        message: format!("Failed to load testcases: {}", e),
                                    },
                                    None,
                                    None,
                                ),
                            }
                        }
                        "Testtask List" => {
                            match TesttaskService::list(&ctx, 1, 100, None, None, None).await {
                                Ok(testtasks) => {
                                    let project_name = if let Some(id) = config.project_id {
                                        ProjectService::get_name(&ctx, id).await.ok()
                                    } else {
                                        None
                                    };
                                    let pn = project_name.clone();
                                    (
                                        AppState::TesttaskList {
                                            testtasks,
                                            project_name,
                                        },
                                        None,
                                        pn,
                                    )
                                }
                                Err(e) => {
                                    eprintln!(
                                        "Error loading testtasks (trying token refresh): {}",
                                        e
                                    );
                                    if ctx.refresh_token().await.is_ok() {
                                        match TesttaskService::list(&ctx, 1, 100, None, None, None)
                                            .await
                                        {
                                            Ok(testtasks) => {
                                                let project_name = if let Some(id) =
                                                    config.project_id
                                                {
                                                    ProjectService::get_name(&ctx, id).await.ok()
                                                } else {
                                                    None
                                                };
                                                let pn = project_name.clone();
                                                (
                                                    AppState::TesttaskList {
                                                        testtasks,
                                                        project_name,
                                                    },
                                                    None,
                                                    pn,
                                                )
                                            }
                                            Err(_) => (
                                                AppState::Error {
                                                    message: format!(
                                                        "Failed to load testtasks: {}",
                                                        e
                                                    ),
                                                },
                                                None,
                                                None,
                                            ),
                                        }
                                    } else {
                                        (
                                            AppState::Error {
                                                message: format!("Failed to load testtasks: {}", e),
                                            },
                                            None,
                                            None,
                                        )
                                    }
                                }
                            }
                        }
                        "Feedback List" => match FeedbackService::list(&ctx, 1, 100).await {
                            Ok(feedbacks) => (AppState::FeedbackList { feedbacks }, None, None),
                            Err(e) => {
                                eprintln!("Error loading feedbacks (trying token refresh): {}", e);
                                if ctx.refresh_token().await.is_ok() {
                                    match FeedbackService::list(&ctx, 1, 100).await {
                                        Ok(feedbacks) => {
                                            (AppState::FeedbackList { feedbacks }, None, None)
                                        }
                                        Err(_) => (
                                            AppState::Error {
                                                message: format!("Failed to load feedbacks: {}", e),
                                            },
                                            None,
                                            None,
                                        ),
                                    }
                                } else {
                                    (
                                        AppState::Error {
                                            message: format!("Failed to load feedbacks: {}", e),
                                        },
                                        None,
                                        None,
                                    )
                                }
                            }
                        },
                        "Ticket List" => {
                            match TicketService::list(&ctx, None, None, 1, 100).await {
                                Ok(tickets) => (AppState::TicketList { tickets }, None, None),
                                Err(e) => {
                                    eprintln!(
                                        "Error loading tickets (trying token refresh): {}",
                                        e
                                    );
                                    if ctx.refresh_token().await.is_ok() {
                                        match TicketService::list(&ctx, None, None, 1, 100).await {
                                            Ok(tickets) => {
                                                (AppState::TicketList { tickets }, None, None)
                                            }
                                            Err(_) => (
                                                AppState::Error {
                                                    message: format!(
                                                        "Failed to load tickets: {}",
                                                        e
                                                    ),
                                                },
                                                None,
                                                None,
                                            ),
                                        }
                                    } else {
                                        (
                                            AppState::Error {
                                                message: format!("Failed to load tickets: {}", e),
                                            },
                                            None,
                                            None,
                                        )
                                    }
                                }
                            }
                        }
                        "Program List" => match ProgramService::list(&ctx).await {
                            Ok(programs) => (AppState::ProgramList { programs }, None, None),
                            Err(e) => {
                                eprintln!("Error loading programs (trying token refresh): {}", e);
                                if ctx.refresh_token().await.is_ok() {
                                    match ProgramService::list(&ctx).await {
                                        Ok(programs) => {
                                            (AppState::ProgramList { programs }, None, None)
                                        }
                                        Err(_) => (
                                            AppState::Error {
                                                message: format!("Failed to load programs: {}", e),
                                            },
                                            None,
                                            None,
                                        ),
                                    }
                                } else {
                                    (
                                        AppState::Error {
                                            message: format!("Failed to load programs: {}", e),
                                        },
                                        None,
                                        None,
                                    )
                                }
                            }
                        },
                        "ProductPlan List" => {
                            let plans =
                                match ProductPlanService::list(&ctx, config.product_id).await {
                                    Ok(p) => Ok(p),
                                    Err(e) => {
                                        eprintln!(
                                        "Error loading product plans (trying token refresh): {}",
                                        e
                                    );
                                        if ctx.refresh_token().await.is_ok() {
                                            ProductPlanService::list(&ctx, config.product_id).await
                                        } else {
                                            Err(e)
                                        }
                                    }
                                };
                            match plans {
                                Ok(plans) => {
                                    let product_name = if let Some(id) = config.product_id {
                                        ProductService::get_name(&ctx, id).await.ok()
                                    } else {
                                        None
                                    };
                                    let pn = product_name.clone();
                                    (
                                        AppState::ProductPlanList {
                                            plans,
                                            product_name,
                                        },
                                        pn,
                                        None,
                                    )
                                }
                                Err(e) => (
                                    AppState::Error {
                                        message: format!("Failed to load product plans: {}", e),
                                    },
                                    None,
                                    None,
                                ),
                            }
                        }
                        _ => (
                            AppState::Error {
                                message: format!("Unknown module: {}", module_name),
                            },
                            None,
                            None,
                        ),
                    };
                    let _ = tx.send(result);
                });
            }

            // Handle input
            let async_rt = self.async_handle.as_ref().map(|h| h.rt.clone());
            let async_form_tx = self.async_handle.as_ref().map(|h| h.form_tx.clone());
            if crossterm::event::poll(std::time::Duration::from_millis(100))? {
                if let Ok(crossterm::event::Event::Key(key)) = crossterm::event::read() {
                    self.handle_key_event(key, app, &async_rt, &async_form_tx);
                }
            }
        }
        Ok(())
    }

    fn handle_key_event(
        &mut self,
        key: KeyEvent,
        app: &mut App,
        async_rt: &Option<std::sync::Arc<tokio::runtime::Runtime>>,
        async_form_tx: &Option<std::sync::Arc<mpsc::Sender<FormSubmitResult>>>,
    ) {
        // Only handle Press events - ignore Release and Repeat
        if !matches!(key.kind, KeyEventKind::Press) {
            return;
        }

        // Handle help overlay first
        if app.help_visible {
            if key.code == KeyCode::Char('?') || key.code == KeyCode::Esc {
                app.help_visible = false;
            }
            return;
        }

        // Handle search mode
        if app.search_active {
            match key.code {
                KeyCode::Esc => {
                    app.search_active = false;
                    app.search_query.clear();
                }
                KeyCode::Enter => {
                    app.search_active = false;
                }
                KeyCode::Backspace => {
                    app.search_query.pop();
                }
                KeyCode::Char(c) => {
                    app.search_query.push(c);
                }
                _ => {}
            }
            return;
        }

        // Handle Ctrl+F to activate search
        if key.code == KeyCode::Char('f') && key.modifiers.contains(KeyModifiers::CONTROL) {
            app.search_active = true;
            return;
        }

        // Handle ? for help
        if key.code == KeyCode::Char('?') {
            app.help_visible = true;
            return;
        }

        // Handle 'c' for create in list states
        if key.code == KeyCode::Char('c') {
            match &app.state {
                AppState::ProgramList { .. } => {
                    app.set_program_create();
                    return;
                }
                AppState::ProductPlanList { plans, .. } if !plans.is_empty() => {
                    // Create ProductPlan requires a product, get first product ID from plans
                    if let Some(plan) = plans.first() {
                        app.set_productplan_create(plan.product);
                        return;
                    }
                }
                AppState::ReleaseList { releases, .. } if !releases.is_empty() => {
                    // Create Release requires a product, get from first release
                    if let Some(release) = releases.first() {
                        app.set_release_create(release.product);
                        return;
                    }
                }
                _ => {}
            }
        }

        // Handle 'e' for edit in detail states
        if key.code == KeyCode::Char('e') {
            match &app.state {
                AppState::ProgramDetail { program } => {
                    let program_clone = program.clone();
                    app.set_program_update(program_clone.id, &program_clone);
                    return;
                }
                AppState::ProductPlanDetail { plan, .. } => {
                    let plan_clone = (*plan).clone();
                    app.set_productplan_update(plan_clone.id, &plan_clone);
                    return;
                }
                AppState::ReleaseDetail { release, .. } => {
                    let release_clone = release.clone();
                    app.set_release_update(release_clone.id, &release_clone);
                    return;
                }
                _ => {}
            }
        }

        // Handle 'd' for delete in detail states
        if key.code == KeyCode::Char('d') {
            match &app.state {
                AppState::ProgramDetail { program } => {
                    let id = program.id;
                    let name = program.name.clone();
                    app.set_program_delete(id, &name);
                    return;
                }
                AppState::ProductPlanDetail { plan, .. } => {
                    let id = plan.id;
                    let name = plan.title.as_deref().unwrap_or("").to_string();
                    app.set_productplan_delete(id, &name);
                    return;
                }
                AppState::ReleaseDetail { release, .. } => {
                    let id = release.id;
                    let name = release.name.clone();
                    app.set_release_delete(id, &name);
                    return;
                }
                _ => {}
            }
        }

        // Handle 'p' for product select
        if key.code == KeyCode::Char('p') {
            let products = self.pending_products.take();
            if let Some(products) = products {
                app.state = AppState::ProductSelect {
                    products,
                    selected: 0,
                    loading: false,
                };
            }
            return;
        }

        // Handle 's' for settings
        if key.code == KeyCode::Char('s') {
            let current_account = app.config.account.clone().unwrap_or_default();
            app.state = AppState::Settings {
                multi_config: app.multi_config.clone(),
                selected: 0,
                current_account,
            };
            return;
        }

        match key.code {
            KeyCode::Up | KeyCode::Char('k') => {
                handle_navigation_up(app);
            }
            KeyCode::Down | KeyCode::Char('j') => {
                handle_navigation_down(app);
            }
            KeyCode::Enter => {
                // Enter key handling for navigation
                // Extract data first to avoid borrow conflicts
                let action = match &app.state {
                    AppState::BugList { bugs, product_name }
                        if !bugs.is_empty() && app.selected_index < bugs.len() =>
                    {
                        let bug = bugs[app.selected_index].clone();
                        let product_name = product_name.clone();
                        Some(EnterAction::BugDetail { bug, product_name })
                    }
                    AppState::StoryList {
                        stories,
                        product_name,
                    } if !stories.is_empty() && app.selected_index < stories.len() => {
                        let story = stories[app.selected_index].clone();
                        let product_name = product_name.clone();
                        Some(EnterAction::StoryDetail {
                            story,
                            product_name,
                        })
                    }
                    AppState::ExecutionList {
                        executions,
                        project_name,
                    } if !executions.is_empty() && app.selected_index < executions.len() => {
                        let execution = executions[app.selected_index].clone();
                        let project_name = project_name.clone();
                        Some(EnterAction::ExecutionDetail {
                            execution,
                            project_name,
                        })
                    }
                    AppState::ReleaseList {
                        releases,
                        product_name,
                    } if !releases.is_empty() && app.selected_index < releases.len() => {
                        let release = releases[app.selected_index].clone();
                        let product_name = product_name.clone();
                        Some(EnterAction::ReleaseDetail {
                            release,
                            product_name,
                        })
                    }
                    AppState::UserList { users }
                        if !users.is_empty() && app.selected_index < users.len() =>
                    {
                        let user = users[app.selected_index].clone();
                        Some(EnterAction::UserDetail { user })
                    }
                    AppState::DepartmentList { departments }
                        if !departments.is_empty() && app.selected_index < departments.len() =>
                    {
                        let dept = departments[app.selected_index].clone();
                        Some(EnterAction::DepartmentDetail { dept })
                    }
                    AppState::ProductList { products }
                        if !products.is_empty() && app.selected_index < products.len() =>
                    {
                        let product = products[app.selected_index].clone();
                        Some(EnterAction::ProductDetail { product })
                    }
                    AppState::ProjectList { projects }
                        if !projects.is_empty() && app.selected_index < projects.len() =>
                    {
                        let project = projects[app.selected_index].clone();
                        Some(EnterAction::ProjectDetail { project })
                    }
                    AppState::TaskList { tasks, .. }
                        if !tasks.is_empty() && app.selected_index < tasks.len() =>
                    {
                        let task = tasks[app.selected_index].clone();
                        Some(EnterAction::TaskDetail { task })
                    }
                    AppState::TestcaseList { testcases, .. }
                        if !testcases.is_empty() && app.selected_index < testcases.len() =>
                    {
                        let tc = testcases[app.selected_index].clone();
                        Some(EnterAction::TestcaseDetail { tc })
                    }
                    AppState::TesttaskList { testtasks, .. }
                        if !testtasks.is_empty() && app.selected_index < testtasks.len() =>
                    {
                        let tt = testtasks[app.selected_index].clone();
                        Some(EnterAction::TesttaskDetail { tt })
                    }
                    AppState::FeedbackList { feedbacks }
                        if !feedbacks.is_empty() && app.selected_index < feedbacks.len() =>
                    {
                        let fb = feedbacks[app.selected_index].clone();
                        Some(EnterAction::FeedbackDetail { fb })
                    }
                    AppState::TicketList { tickets }
                        if !tickets.is_empty() && app.selected_index < tickets.len() =>
                    {
                        let ticket = tickets[app.selected_index].clone();
                        Some(EnterAction::TicketDetail { ticket })
                    }
                    AppState::ProgramList { programs }
                        if !programs.is_empty() && app.selected_index < programs.len() =>
                    {
                        let program = programs[app.selected_index].clone();
                        Some(EnterAction::ProgramDetail { program })
                    }
                    AppState::ProductPlanList { plans, .. }
                        if !plans.is_empty() && app.selected_index < plans.len() =>
                    {
                        let plan = plans[app.selected_index].clone();
                        Some(EnterAction::ProductPlanDetail { plan })
                    }
                    AppState::MainMenu { .. } => {
                        if let Some(module_name) = app.get_selected_module() {
                            app.saved_main_index = app.selected_index;
                            // Settings doesn't load data, it shows settings panel directly
                            if module_name == "Settings" {
                                let current_account =
                                    app.config.account.clone().unwrap_or_default();
                                app.state = AppState::Settings {
                                    multi_config: app.multi_config.clone(),
                                    selected: 0,
                                    current_account,
                                };
                            } else {
                                app.set_module_selected(module_name);
                            }
                        }
                        None
                    }
                    AppState::ConfirmQuit => {
                        app.quit();
                        None
                    }
                    // Bug CRUD
                    AppState::BugCreate { .. } => Some(EnterAction::BugCreate),
                    AppState::BugUpdate { id, .. } => Some(EnterAction::BugUpdate { id: *id }),
                    AppState::BugDelete {
                        id,
                        name,
                        confirm: false,
                        ..
                    } => Some(EnterAction::BugDelete {
                        id: *id,
                        name: name.clone(),
                    }),
                    AppState::BugDelete { confirm: true, .. } => {
                        None
                    }
                    // Program/ProductPlan/Release CRUD
                    AppState::ProgramCreate { .. } => Some(EnterAction::CreateProgram),
                    AppState::ProgramUpdate { .. } => Some(EnterAction::UpdateProgram),
                    AppState::ProgramDelete { id, .. } => Some(EnterAction::DeleteProgram(*id)),
                    AppState::ProductPlanCreate { .. } => Some(EnterAction::CreateProductPlan),
                    AppState::ProductPlanUpdate { .. } => Some(EnterAction::UpdateProductPlan),
                    AppState::ProductPlanDelete { id, .. } => Some(EnterAction::DeleteProductPlan(*id)),
                    AppState::ReleaseCreate { .. } => Some(EnterAction::CreateRelease),
                    AppState::ReleaseUpdate { .. } => Some(EnterAction::UpdateRelease),
                    AppState::ReleaseDelete { id, .. } => Some(EnterAction::DeleteRelease(*id)),
                    _ => None,
                };

                // Execute the action after borrow is released
                if let Some(act) = action {
                    app.save_list();
                    match act {
                        EnterAction::BugDetail { bug, product_name } => {
                            app.set_bug_detail(bug, product_name)
                        }
                        EnterAction::BugCreate => {
                            if let (Some(rt), Some(form_tx)) = (async_rt, async_form_tx) {
                                if let AppState::BugCreate { fields, .. } = &app.state {
                                    let fields = fields.clone();
                                    let config = app.config.clone();
                                    let form_tx = (**form_tx).clone();
                                    let rt = rt.clone();
                                    rt.block_on(async move {
                                        let ctx = AppContext::new(
                                            config.clone(),
                                            OutputFormat::Table,
                                            false,
                                        );
                                        let result = BugService::create(
                                            &ctx,
                                            fields.get("title").cloned().unwrap_or_default(),
                                            config.product_id,
                                            fields
                                                .get("severity")
                                                .and_then(|s| s.parse().ok())
                                                .unwrap_or(3),
                                            fields.get("pri").and_then(|s| s.parse().ok()),
                                            fields.get("type").cloned(),
                                            fields.get("steps").cloned(),
                                            fields.get("story").and_then(|s| s.parse().ok()),
                                            fields.get("branch").and_then(|s| s.parse().ok()),
                                            fields.get("module").and_then(|s| s.parse().ok()),
                                            fields.get("execution").and_then(|s| s.parse().ok()),
                                            fields.get("keywords").cloned(),
                                            fields.get("os").cloned(),
                                            fields.get("browser").cloned(),
                                            fields.get("deadline").cloned(),
                                            None,
                                        )
                                        .await;
                                        match result {
                                            Ok(bug) => {
                                                let _ = form_tx
                                                    .send(FormSubmitResult::BugCreateSuccess(bug));
                                            }
                                            Err(e) => {
                                                let _ = form_tx.send(
                                                    FormSubmitResult::BugCreateError(e.to_string()),
                                                );
                                            }
                                        }
                                    });
                                }
                            }
                        }
                        EnterAction::BugUpdate { id } => {
                            if let (Some(rt), Some(form_tx)) = (async_rt, async_form_tx) {
                                if let AppState::BugUpdate { fields, .. } = &app.state {
                                    let fields = fields.clone();
                                    let config = app.config.clone();
                                    let form_tx = (**form_tx).clone();
                                    let rt = rt.clone();
                                    rt.block_on(async move {
                                        let ctx = AppContext::new(
                                            config.clone(),
                                            OutputFormat::Table,
                                            false,
                                        );
                                        let req = crate::api::UpdateBugRequest {
                                            title: fields.get("title").cloned(),
                                            keywords: fields.get("keywords").cloned(),
                                            severity: fields
                                                .get("severity")
                                                .and_then(|s| s.parse().ok()),
                                            pri: fields.get("pri").and_then(|s| s.parse().ok()),
                                            type_: fields.get("type").cloned(),
                                            os: fields.get("os").cloned(),
                                            browser: fields.get("browser").cloned(),
                                            steps: fields.get("steps").cloned(),
                                            task: fields.get("task").and_then(|s| s.parse().ok()),
                                            story: fields.get("story").and_then(|s| s.parse().ok()),
                                            deadline: fields.get("deadline").cloned(),
                                            opened_build: None,
                                            branch: fields
                                                .get("branch")
                                                .and_then(|s| s.parse().ok()),
                                            module: fields
                                                .get("module")
                                                .and_then(|s| s.parse().ok()),
                                            execution: fields
                                                .get("execution")
                                                .and_then(|s| s.parse().ok()),
                                        };
                                        let result = BugService::update(&ctx, id, req).await;
                                        match result {
                                            Ok(bug) => {
                                                let _ = form_tx
                                                    .send(FormSubmitResult::BugUpdateSuccess(bug));
                                            }
                                            Err(e) => {
                                                let _ = form_tx.send(
                                                    FormSubmitResult::BugUpdateError(e.to_string()),
                                                );
                                            }
                                        }
                                    });
                                }
                            }
                        }
                        EnterAction::BugDelete { id, name: _ } => {
                            if let (Some(rt), Some(form_tx)) = (async_rt, async_form_tx) {
                                let config = app.config.clone();
                                let form_tx = (**form_tx).clone();
                                let rt = rt.clone();
                                rt.block_on(async move {
                                    let ctx = AppContext::new(config, OutputFormat::Table, false);
                                    let result = BugService::delete(&ctx, id).await;
                                    match result {
                                        Ok(()) => {
                                            let _ = form_tx
                                                .send(FormSubmitResult::BugDeleteSuccess(id));
                                        }
                                        Err(e) => {
                                            let _ = form_tx.send(FormSubmitResult::BugDeleteError(
                                                e.to_string(),
                                            ));
                                        }
                                    }
                                });
                            }
                        }
                        EnterAction::StoryDetail {
                            story,
                            product_name,
                        } => app.set_story_detail(story, product_name),
                        EnterAction::ExecutionDetail {
                            execution,
                            project_name,
                        } => app.set_execution_detail(execution, project_name),
                        EnterAction::ReleaseDetail {
                            release,
                            product_name,
                        } => app.set_release_detail(release, product_name),
                        EnterAction::UserDetail { user } => app.set_user_detail(user),
                        EnterAction::DepartmentDetail { dept } => app.set_department_detail(dept),
                        EnterAction::ProductDetail { product } => app.set_product_detail(product),
                        EnterAction::ProjectDetail { project } => app.set_project_detail(project),
                        EnterAction::TaskDetail { task } => app.set_task_detail(task, None),
                        EnterAction::TestcaseDetail { tc } => app.set_testcase_detail(tc, None),
                        EnterAction::TesttaskDetail { tt } => app.set_testtask_detail(tt, None),
                        EnterAction::FeedbackDetail { fb } => app.set_feedback_detail(fb),
                        EnterAction::TicketDetail { ticket } => app.set_ticket_detail(ticket),
                        EnterAction::ProgramDetail { program } => app.set_program_detail(program),
                        EnterAction::ProductPlanDetail { plan } => {
                            app.set_productplan_detail(plan, None)
                        }
                        EnterAction::DeleteProgram(id) => {
                            app.set_form_submitting("正在删除项目集...");
                            self.pending_form_submit = Some(FormSubmitAction::DeleteProgram(id));
                        }
                        EnterAction::DeleteProductPlan(id) => {
                            app.set_form_submitting("正在删除产品计划...");
                            self.pending_form_submit = Some(FormSubmitAction::DeleteProductPlan(id));
                        }
                        EnterAction::DeleteRelease(id) => {
                            app.set_form_submitting("正在删除发布...");
                            self.pending_form_submit = Some(FormSubmitAction::DeleteRelease(id));
                        }
                        EnterAction::CreateProgram => {
                            if let AppState::ProgramCreate { fields } = &app.state {
                                if let Some(msg) = fields.validate() {
                                    app.state = AppState::Error { message: msg };
                                    return;
                                }
                                let fields_clone = fields.clone();
                                app.set_form_submitting("正在创建项目集...");
                                self.pending_form_submit = Some(FormSubmitAction::CreateProgram(fields_clone));
                            }
                        }
                        EnterAction::UpdateProgram => {
                            if let AppState::ProgramUpdate { id, fields } = &app.state {
                                if let Some(msg) = fields.validate() {
                                    app.state = AppState::Error { message: msg };
                                    return;
                                }
                                let id = *id;
                                let fields_clone = fields.clone();
                                app.set_form_submitting("正在更新项目集...");
                                self.pending_form_submit = Some(FormSubmitAction::UpdateProgram(id, fields_clone));
                            }
                        }
                        EnterAction::CreateProductPlan => {
                            if let AppState::ProductPlanCreate { fields, product_id } = &app.state {
                                if let Some(msg) = fields.validate() {
                                    app.state = AppState::Error { message: msg };
                                    return;
                                }
                                let product_id = *product_id;
                                let fields_clone = fields.clone();
                                app.set_form_submitting("正在创建产品计划...");
                                self.pending_form_submit = Some(FormSubmitAction::CreateProductPlan(product_id, fields_clone));
                            }
                        }
                        EnterAction::UpdateProductPlan => {
                            if let AppState::ProductPlanUpdate { id, fields } = &app.state {
                                if let Some(msg) = fields.validate() {
                                    app.state = AppState::Error { message: msg };
                                    return;
                                }
                                let id = *id;
                                let fields_clone = fields.clone();
                                app.set_form_submitting("正在更新产品计划...");
                                self.pending_form_submit = Some(FormSubmitAction::UpdateProductPlan(id, fields_clone));
                            }
                        }
                        EnterAction::CreateRelease => {
                            if let AppState::ReleaseCreate { fields, product_id } = &app.state {
                                if let Some(msg) = fields.validate() {
                                    app.state = AppState::Error { message: msg };
                                    return;
                                }
                                let product_id = *product_id;
                                let fields_clone = fields.clone();
                                app.set_form_submitting("正在创建发布...");
                                self.pending_form_submit = Some(FormSubmitAction::CreateRelease(product_id, fields_clone));
                            }
                        }
                        EnterAction::UpdateRelease => {
                            if let AppState::ReleaseUpdate { id, fields } = &app.state {
                                if let Some(msg) = fields.validate() {
                                    app.state = AppState::Error { message: msg };
                                    return;
                                }
                                let id = *id;
                                let fields_clone = fields.clone();
                                app.set_form_submitting("正在更新发布...");
                                self.pending_form_submit = Some(FormSubmitAction::UpdateRelease(id, fields_clone));
                            }
                        }
                    }
                }
            }
            KeyCode::Tab | KeyCode::BackTab => {
                // Tab navigation in forms
                let max_field = match &app.state {
                    AppState::ProgramCreate { .. } | AppState::ProgramUpdate { .. } => 4,
                    AppState::ProductPlanCreate { .. } | AppState::ProductPlanUpdate { .. } => 3,
                    AppState::ReleaseCreate { .. } | AppState::ReleaseUpdate { .. } => 4,
                    _ => return,
                };

                // Navigate form fields
                if key.code == KeyCode::BackTab || key.modifiers.contains(KeyModifiers::SHIFT) {
                    if app.selected_index > 0 {
                        app.selected_index -= 1;
                    }
                } else if app.selected_index < max_field {
                    app.selected_index += 1;
                }
            }
            KeyCode::Char(c) => {
                // Handle character input in forms
                let is_form_state = match &app.state {
                    AppState::ProgramCreate { .. } | AppState::ProgramUpdate { .. } |
                    AppState::ProductPlanCreate { .. } | AppState::ProductPlanUpdate { .. } |
                    AppState::ReleaseCreate { .. } | AppState::ReleaseUpdate { .. } => true,
                    _ => false,
                };

                if is_form_state {
                    match &mut app.state {
                        AppState::ProgramCreate { fields } | AppState::ProgramUpdate { fields, .. } => {
                            let idx = app.selected_index;
                            let mut form_fields = fields.get_mut_fields();
                            if idx < form_fields.len() {
                                let field = &mut *form_fields[idx];
                                if field.editable {
                                    field.value.push(c);
                                }
                            }
                        }
                        AppState::ProductPlanCreate { fields, .. } | AppState::ProductPlanUpdate { fields, .. } => {
                            let idx = app.selected_index;
                            let mut form_fields = fields.get_mut_fields();
                            if idx < form_fields.len() {
                                let field = &mut *form_fields[idx];
                                if field.editable {
                                    field.value.push(c);
                                }
                            }
                        }
                        AppState::ReleaseCreate { fields, .. } | AppState::ReleaseUpdate { fields, .. } => {
                            let idx = app.selected_index;
                            let mut form_fields = fields.get_mut_fields();
                            if idx < form_fields.len() {
                                let field = &mut *form_fields[idx];
                                if field.editable {
                                    field.value.push(c);
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
            KeyCode::Backspace => {
                // Handle backspace in forms
                let is_form_state = match &app.state {
                    AppState::ProgramCreate { .. } | AppState::ProgramUpdate { .. } |
                    AppState::ProductPlanCreate { .. } | AppState::ProductPlanUpdate { .. } |
                    AppState::ReleaseCreate { .. } | AppState::ReleaseUpdate { .. } => true,
                    _ => false,
                };

                if is_form_state {
                    match &mut app.state {
                        AppState::ProgramCreate { fields } | AppState::ProgramUpdate { fields, .. } => {
                            let idx = app.selected_index;
                            let mut form_fields = fields.get_mut_fields();
                            if idx < form_fields.len() {
                                let field = &mut *form_fields[idx];
                                if field.editable {
                                    field.value.pop();
                                }
                            }
                        }
                        AppState::ProductPlanCreate { fields, .. } | AppState::ProductPlanUpdate { fields, .. } => {
                            let idx = app.selected_index;
                            let mut form_fields = fields.get_mut_fields();
                            if idx < form_fields.len() {
                                let field = &mut *form_fields[idx];
                                if field.editable {
                                    field.value.pop();
                                }
                            }
                        }
                        AppState::ReleaseCreate { fields, .. } | AppState::ReleaseUpdate { fields, .. } => {
                            let idx = app.selected_index;
                            let mut form_fields = fields.get_mut_fields();
                            if idx < form_fields.len() {
                                let field = &mut *form_fields[idx];
                                if field.editable {
                                    field.value.pop();
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
            KeyCode::Esc | KeyCode::Char('q') => match &app.state {
                AppState::BugDetail { .. }
                | AppState::StoryDetail { .. }
                | AppState::ExecutionDetail { .. }
                | AppState::BuildDetail { .. }
                | AppState::ReleaseDetail { .. }
                | AppState::UserDetail { .. }
                | AppState::DepartmentDetail { .. }
                | AppState::ProductDetail { .. }
                | AppState::ProjectDetail { .. }
                | AppState::TaskDetail { .. }
                | AppState::TestcaseDetail { .. }
                | AppState::TesttaskDetail { .. }
                | AppState::FeedbackDetail { .. }
                | AppState::TicketDetail { .. }
                | AppState::ProgramDetail { .. }
                | AppState::ProductPlanDetail { .. } => {
                    app.restore_list();
                }
                // List states - return to MainMenu
                | AppState::StoryDetail { .. }
                | AppState::ExecutionDetail { .. }
                | AppState::BuildDetail { .. }
                | AppState::ReleaseDetail { .. }
                | AppState::UserDetail { .. }
                | AppState::DepartmentDetail { .. }
                | AppState::ProductDetail { .. }
                | AppState::ProjectDetail { .. }
                | AppState::TaskDetail { .. }
                | AppState::TestcaseDetail { .. }
                | AppState::TesttaskDetail { .. }
                | AppState::FeedbackDetail { .. }
                | AppState::TicketDetail { .. }
                | AppState::ProgramDetail { .. }
                | AppState::ProductPlanDetail { .. } => {
                    app.restore_list();
                }
                // List states - return to MainMenu
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
                | AppState::ProductPlanList { .. } => {
                    app.set_main_menu();
                }
                AppState::Loading { .. } => {
                    // Cancel loading and return to MainMenu
                    self.loading_cancelled = true;
                    self.loading_module = None;
                    app.set_main_menu();
                }
                AppState::Error { .. } => {
                    // Return to MainMenu from error and cancel any pending loading
                    self.loading_cancelled = true;
                    self.loading_module = None;
                    app.set_main_menu();
                }
                AppState::MainMenu { .. } => {
                    app.state = AppState::ConfirmQuit;
                }
                AppState::ConfirmQuit => {
                    // n or Esc cancels quit, goes back to MainMenu
                    app.set_main_menu();
                }
                AppState::Settings { .. } => {
                    app.set_main_menu();
                }
                AppState::ProductSelect { .. } | AppState::AccountSelect { .. } => {
                    app.state = AppState::Idle;
                }
                // CRUD form states - cancel and go back
                AppState::ProgramCreate { .. } | AppState::ProgramUpdate { .. } => {
                    app.restore_list();
                }
                AppState::ProductPlanCreate { .. } | AppState::ProductPlanUpdate { .. } => {
                    app.restore_list();
                }
                AppState::ReleaseCreate { .. } | AppState::ReleaseUpdate { .. } => {
                    app.restore_list();
                }
                AppState::ProgramDelete { .. } => {
                    app.restore_list();
                }
                AppState::ProductPlanDelete { .. } => {
                    app.restore_list();
                }
                AppState::ReleaseDelete { .. } => {
                    app.restore_list();
                }
                AppState::FormSubmitting { .. } => {
                    app.restore_list();
                }
                // Bug CRUD states - cancel and go back
                AppState::BugCreate { .. } | AppState::BugUpdate { .. } => {
                    app.restore_list();
                }
                AppState::BugDelete { .. } => {
                    app.restore_list();
                }
                _ => {}
            },
            KeyCode::Char('r') | KeyCode::F(5) => {
                // Handle retry in Error state
                if matches!(app.state, AppState::Error { .. }) {
                    if let Some(ref module_name) = self.loading_module {
                        app.set_module_selected(module_name.clone());
                    }
                } else if let Some(reload) = self.pending_reload.take() {
                    reload(app);
                }
            }
            KeyCode::Char('o') => {
                if let Some(url) = get_open_url(app) {
                    let _ = open::that(&url);
                }
            }
            // Bug CRUD shortcuts
            KeyCode::Char('c') => {
                if matches!(app.state, AppState::BugList { .. }) {
                    app.set_bug_create();
                }
            }
            KeyCode::Char('e') => {
                if let AppState::BugDetail { bug, .. } = &app.state {
                    app.set_bug_update(&bug.clone());
                }
            }
            KeyCode::Char('d') => {
                if let AppState::BugDetail { bug, .. } = &app.state {
                    app.set_bug_delete(bug.id, bug.title.clone());
                }
            }
            KeyCode::Tab => {
                // Navigate to next field in forms
                if let AppState::BugCreate {
                    ref mut focused_field,
                    ..
                } = &mut app.state
                {
                    let max = 4; // field_order.len() - 1
                    if *focused_field < max {
                        *focused_field += 1;
                    }
                } else if let AppState::BugUpdate {
                    ref mut focused_field,
                    ..
                } = &mut app.state
                {
                    let max = 4;
                    if *focused_field < max {
                        *focused_field += 1;
                    }
                }
            }
            KeyCode::BackTab => {
                // Navigate to previous field in forms
                if let AppState::BugCreate {
                    ref mut focused_field,
                    ..
                } = &mut app.state
                {
                    if *focused_field > 0 {
                        *focused_field -= 1;
                    }
                } else if let AppState::BugUpdate {
                    ref mut focused_field,
                    ..
                } = &mut app.state
                {
                    if *focused_field > 0 {
                        *focused_field -= 1;
                    }
                }
            }
            KeyCode::Char(c) => {
                match c {
                    // Handle quit confirmation
                    'y' | 'Y' if matches!(app.state, AppState::ConfirmQuit) => {
                        app.quit();
                    }
                    // Type in focused field for BugCreate/BugUpdate
                    _ => {
                        if let AppState::BugCreate {
                            ref mut fields,
                            ref field_order,
                            focused_field,
                            ..
                        } = &mut app.state
                        {
                            if let Some(key) = field_order.get(*focused_field) {
                                fields.get_mut(key).map(|v| v.push(c));
                            }
                        } else if let AppState::BugUpdate {
                            ref mut fields,
                            ref field_order,
                            focused_field,
                            ..
                        } = &mut app.state
                        {
                            if let Some(key) = field_order.get(*focused_field) {
                                fields.get_mut(key).map(|v| v.push(c));
                            }
                        }
                    }
                }
            }
            KeyCode::Backspace => {
                // Delete character in focused field
                if let AppState::BugCreate {
                    ref mut fields,
                    ref field_order,
                    focused_field,
                    ..
                } = &mut app.state
                {
                    if let Some(key) = field_order.get(*focused_field) {
                        fields.get_mut(key).map(|v| {
                            v.pop();
                        });
                    }
                } else if let AppState::BugUpdate {
                    ref mut fields,
                    ref field_order,
                    focused_field,
                    ..
                } = &mut app.state
                {
                    if let Some(key) = field_order.get(*focused_field) {
                        fields.get_mut(key).map(|v| {
                            v.pop();
                        });
                    }
                }
            }
            _ => {}
        }
    }

    fn render_help_overlay(f: &mut ratatui::Frame, area: Rect) {
        let _chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(0),
                Constraint::Length(25),
                Constraint::Min(0),
            ])
            .split(area);

        let help_text = vec![
            Line::from(Span::styled(
                "Keyboard Shortcuts",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            )),
            Line::from(Span::raw("")),
            Line::from(vec![
                Span::raw("  "),
                Span::styled("↑/k", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw("    Move selection up"),
            ]),
            Line::from(vec![
                Span::raw("  "),
                Span::styled("↓/j", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw("    Move selection down"),
            ]),
            Line::from(vec![
                Span::raw("  "),
                Span::styled("Enter", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw("    View selected item details"),
            ]),
            Line::from(vec![
                Span::raw("  "),
                Span::styled("Esc/q", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw("    Back / Quit"),
            ]),
            Line::from(vec![
                Span::raw("  "),
                Span::styled("r", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw("      Refresh list"),
            ]),
            Line::from(vec![
                Span::raw("  "),
                Span::styled("Ctrl+F", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw("  Activate search"),
            ]),
            Line::from(vec![
                Span::raw("  "),
                Span::styled("?", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw("      Toggle this help overlay"),
            ]),
            Line::from(Span::raw("")),
            Line::from(Span::styled(
                "Press any key to close",
                Style::default().fg(Color::DarkGray),
            )),
        ];

        let paragraph = Paragraph::new(Text::from(help_text))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Help")
                    .title_style(Style::default().fg(Color::Cyan)),
            )
            .wrap(Wrap { trim: true })
            .alignment(ratatui::layout::Alignment::Left);

        f.render_widget(paragraph, area);
    }

    fn render_idle(f: &mut ratatui::Frame, area: Rect, app: &App) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(0),
                Constraint::Length(3),
            ])
            .split(area);

        // Header with search bar hint
        let search_hint = if app.search_active {
            format!("[SEARCH: {}] (Esc to cancel)", app.search_query)
        } else {
            "[Ctrl+F] Search".to_string()
        };
        let header = Paragraph::new(Text::from(vec![Line::from(vec![
            Span::styled("ZenTao CLI", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw("  |  "),
            Span::styled(&search_hint, Style::default().fg(Color::Cyan)),
        ])]))
        .block(Block::default().borders(Borders::ALL).title("ZenTao"));

        f.render_widget(header, chunks[0]);

        let text = Paragraph::new(Text::from(vec![
            Line::from(Span::raw("")),
            Line::from(Span::raw(
                "Use commands: zentao bug browse, zentao story browse",
            )),
            Line::from(Span::raw("")),
            Line::from(Span::styled(
                "Press ? for help",
                Style::default().fg(Color::DarkGray),
            )),
        ]))
        .block(Block::default().borders(Borders::ALL));

        f.render_widget(text, chunks[1]);

        // Footer with shortcuts hint
        let footer = Paragraph::new(Text::from(vec![Line::from(vec![
            Span::styled("?", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" help  "),
            Span::styled("Ctrl+F", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" search  "),
            Span::styled("q", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" quit"),
        ])]));
        f.render_widget(footer, chunks[2]);
    }

    fn render_loading(f: &mut ratatui::Frame, area: Rect, message: &str, frame: usize) {
        let spinner_chars = ['|', '/', '-', '\\'];
        let spinner = spinner_chars[frame];
        let text = Paragraph::new(Text::from(vec![
            Line::from(Span::raw(message)),
            Line::from(Span::raw("")),
            Line::from(vec![Span::styled(
                format!("[{}] Loading...", spinner),
                Style::default().fg(Color::Yellow),
            )]),
        ]))
        .block(Block::default().borders(Borders::ALL).title("Please wait"));

        f.render_widget(text, area);
    }

    fn render_error(f: &mut ratatui::Frame, area: Rect, message: &str) {
        let text = Paragraph::new(Text::from(vec![
            Line::from(Span::styled(
                "Error:",
                Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
            )),
            Line::from(Span::raw(message)),
            Line::from(Span::raw("")),
            Line::from(Span::styled(
                "Press r or F5 to retry, q to quit",
                Style::default().fg(Color::DarkGray),
            )),
        ]))
        .block(Block::default().borders(Borders::ALL).title("Error"));

        f.render_widget(text, area);
    }

    fn render_settings(
        f: &mut ratatui::Frame,
        area: Rect,
        multi_config: &crate::core::config::MultiAccountConfig,
        _selected: usize,
        current_account: &str,
        app: &App,
    ) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Header
                Constraint::Min(0),    // Content
                Constraint::Length(3), // Footer
            ])
            .split(area);

        // Header
        let header = Paragraph::new(Text::from(vec![Line::from(vec![
            Span::styled("Settings", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" - Press q/ESC to close"),
        ])]))
        .block(Block::default().borders(Borders::ALL).title("ZenTao"));

        f.render_widget(header, chunks[0]);

        // Settings content
        let accounts: Vec<&String> = multi_config.list_account_names();
        let items: Vec<ListItem> = accounts
            .iter()
            .map(|name| {
                let is_current = *name == current_account;
                let suffix = if is_current { " (current)" } else { "" };
                ListItem::new(Line::from(vec![Span::styled(
                    format!("{}{}", name, suffix),
                    if is_current {
                        Style::default()
                            .add_modifier(Modifier::BOLD)
                            .fg(Color::Green)
                    } else {
                        Style::default()
                    },
                )]))
            })
            .collect();

        let settings_text = vec![
            Line::from(Span::raw("")),
            Line::from(vec![Span::styled(
                "Accounts",
                Style::default().add_modifier(Modifier::BOLD),
            )]),
            Line::from(Span::raw("")),
        ];

        let _settings_para =
            Paragraph::new(Text::from(settings_text)).block(Block::default().borders(Borders::ALL));

        let _list = if items.is_empty() {
            Paragraph::new(Text::from(vec![Line::from(Span::styled(
                "No accounts configured. Run 'zentao auth login' first.",
                Style::default().fg(Color::DarkGray),
            ))]))
        } else {
            Paragraph::new(Text::from(vec![]))
        };

        let content = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(chunks[1]);

        let account_items: Vec<ListItem> = accounts
            .iter()
            .map(|name| {
                let is_current = *name == current_account;
                ListItem::new(Line::from(vec![
                    if is_current {
                        Span::styled("> ", Style::default().fg(Color::Green))
                    } else {
                        Span::raw("  ")
                    },
                    Span::raw(*name),
                    if is_current {
                        Span::styled(" (current)", Style::default().fg(Color::DarkGray))
                    } else {
                        Span::raw("")
                    },
                ]))
            })
            .collect();

        let list = List::new(account_items)
            .block(Block::default().borders(Borders::ALL).title("Accounts"))
            .highlight_style(Style::default().bg(Color::Blue).fg(Color::White));

        f.render_widget(list, content[0]);

        let right = Paragraph::new(Text::from(vec![
            Line::from(Span::raw("")),
            Line::from(vec![Span::styled(
                "Current Config:",
                Style::default().add_modifier(Modifier::BOLD),
            )]),
            Line::from(Span::raw("")),
            Line::from(Span::raw(format!("URL: {}", app.config.url))),
            Line::from(Span::raw(format!("Account: {}", current_account))),
            Line::from(Span::raw(format!(
                "Product ID: {:?}",
                app.config.product_id
            ))),
        ]))
        .block(Block::default().borders(Borders::ALL));

        f.render_widget(right, content[1]);

        // Footer
        let footer = Paragraph::new(Text::from(vec![Line::from(vec![
            Span::styled("↑↓", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" select account  "),
            Span::styled("Enter", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" switch  "),
            Span::styled("q", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" close"),
        ])]));
        f.render_widget(footer, chunks[2]);
    }

    fn render_product_select(
        f: &mut ratatui::Frame,
        area: Rect,
        products: &[Product],
        _selected: usize,
        loading: bool,
    ) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(0),
                Constraint::Length(3),
            ])
            .split(area);

        let header = Paragraph::new(Text::from(vec![Line::from(vec![
            Span::styled(
                "Select Product",
                Style::default().add_modifier(Modifier::BOLD),
            ),
            Span::raw(" - Press q/ESC to close"),
        ])]))
        .block(Block::default().borders(Borders::ALL).title("ZenTao"));

        f.render_widget(header, chunks[0]);

        if loading {
            let loading_text = Paragraph::new(Text::from(vec![Line::from(Span::styled(
                "Loading products...",
                Style::default().fg(Color::Yellow),
            ))]))
            .block(Block::default().borders(Borders::ALL));
            f.render_widget(loading_text, chunks[1]);
        } else if products.is_empty() {
            let empty_text = Paragraph::new(Text::from(vec![Line::from(Span::styled(
                "No products available",
                Style::default().fg(Color::DarkGray),
            ))]))
            .block(Block::default().borders(Borders::ALL));
            f.render_widget(empty_text, chunks[1]);
        } else {
            let items: Vec<ListItem> = products
                .iter()
                .map(|p| {
                    ListItem::new(Line::from(vec![
                        Span::raw(format!("{:6}", p.id)),
                        Span::raw(" "),
                        Span::styled(&p.name, Style::default()),
                        Span::raw(" ("),
                        Span::raw(&p.code),
                        Span::raw(")"),
                    ]))
                })
                .collect();

            let list = List::new(items)
                .block(Block::default().borders(Borders::ALL))
                .highlight_style(Style::default().bg(Color::Blue).fg(Color::White));

            f.render_widget(list, chunks[1]);
        }

        let footer = Paragraph::new(Text::from(vec![Line::from(vec![
            Span::styled("↑↓", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" select  "),
            Span::styled("Enter", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" confirm  "),
            Span::styled("q", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" close"),
        ])]));
        f.render_widget(footer, chunks[2]);
    }

    fn render_account_select(
        f: &mut ratatui::Frame,
        area: Rect,
        multi_config: &crate::core::config::MultiAccountConfig,
        _selected: usize,
        _app: &App,
    ) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(0),
                Constraint::Length(3),
            ])
            .split(area);

        let accounts: Vec<&String> = multi_config.list_account_names();

        let header = Paragraph::new(Text::from(vec![Line::from(vec![
            Span::styled(
                "Select Account",
                Style::default().add_modifier(Modifier::BOLD),
            ),
            Span::raw(" - Press q/ESC to close"),
        ])]))
        .block(Block::default().borders(Borders::ALL).title("ZenTao"));

        f.render_widget(header, chunks[0]);

        if accounts.is_empty() {
            let empty_text = Paragraph::new(Text::from(vec![Line::from(Span::styled(
                "No accounts configured. Run 'zentao auth login' first.",
                Style::default().fg(Color::DarkGray),
            ))]))
            .block(Block::default().borders(Borders::ALL));
            f.render_widget(empty_text, chunks[1]);
        } else {
            let items: Vec<ListItem> = accounts
                .iter()
                .map(|name| {
                    let is_default = multi_config.default_account.as_deref() == Some(*name);
                    ListItem::new(Line::from(vec![
                        if is_default {
                            Span::styled("* ", Style::default().fg(Color::Green))
                        } else {
                            Span::raw("  ")
                        },
                        Span::raw(*name),
                    ]))
                })
                .collect();

            let list = List::new(items)
                .block(Block::default().borders(Borders::ALL))
                .highlight_style(Style::default().bg(Color::Blue).fg(Color::White));

            f.render_widget(list, chunks[1]);
        }

        let footer = Paragraph::new(Text::from(vec![Line::from(vec![
            Span::styled("↑↓", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" select  "),
            Span::styled("Enter", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" switch  "),
            Span::styled("q", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" close"),
        ])]));
        f.render_widget(footer, chunks[2]);
    }

    fn render_main_menu(f: &mut ratatui::Frame, area: Rect, selected: usize, _app: &App) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Header
                Constraint::Min(0),    // Menu items
                Constraint::Length(3), // Footer
            ])
            .split(area);

        // Header
        let header = Paragraph::new(Text::from(vec![Line::from(vec![
            Span::styled("ZenTao CLI", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" - Main Menu"),
        ])]))
        .block(Block::default().borders(Borders::ALL).title("ZenTao"));

        f.render_widget(header, chunks[0]);

        // Menu items - single column centered
        let modules = App::get_main_menu_modules();
        let items: Vec<ListItem> = modules
            .iter()
            .enumerate()
            .map(|(i, module)| {
                let is_selected = i == selected;
                let prefix = if is_selected { "> " } else { "  " };
                let style = if is_selected {
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(Color::White)
                };
                ListItem::new(Line::from(vec![
                    Span::styled(
                        prefix,
                        if is_selected {
                            Color::Cyan
                        } else {
                            Color::Gray
                        },
                    ),
                    Span::styled(*module, style),
                ]))
            })
            .collect();

        let menu_list = List::new(items)
            .block(Block::default().borders(Borders::ALL))
            .style(Style::default());

        f.render_widget(menu_list, chunks[1]);

        // Footer with shortcuts
        let footer = Paragraph::new(Text::from(vec![Line::from(vec![
            Span::styled("↑↓/jk", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" nav  "),
            Span::styled("Enter", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" select  "),
            Span::styled("?", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" help  "),
            Span::styled("q", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw("/"),
            Span::styled("Esc", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" quit"),
        ])]));
        f.render_widget(footer, chunks[2]);
    }

    fn render_confirm_quit(f: &mut ratatui::Frame, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(5),
                Constraint::Length(3),
                Constraint::Min(0),
            ])
            .split(area);

        let dialog = Paragraph::new(Text::from(vec![
            Line::from(Span::raw("")),
            Line::from(Span::styled(
                "  Confirm Quit  ",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            )),
            Line::from(Span::raw("")),
            Line::from(Span::raw("  Do you want to quit ZenTao CLI?")),
            Line::from(Span::raw("")),
        ]))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Quit")
                .title_style(Style::default().fg(Color::Cyan)),
        )
        .alignment(ratatui::layout::Alignment::Center);

        f.render_widget(dialog, chunks[0]);

        let choices = Paragraph::new(Text::from(vec![Line::from(vec![
            Span::styled("Y", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw("/"),
            Span::styled("Enter", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" quit  "),
            Span::styled("n", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw("/"),
            Span::styled("Esc", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" cancel"),
        ])]))
        .alignment(ratatui::layout::Alignment::Center);

        f.render_widget(choices, chunks[1]);
    }

    fn render_bug_form(
        f: &mut ratatui::Frame,
        area: Rect,
        fields: &std::collections::HashMap<String, String>,
        field_order: &[String],
        focused_field: usize,
        error: Option<&str>,
        is_create: bool,
    ) {
        use ratatui::{
            layout::{Constraint, Direction, Layout},
            style::{Color, Modifier, Style},
            text::{Line, Span, Text},
            widgets::{Block, Borders, Paragraph, Wrap},
        };

        let title = if is_create { "Create Bug" } else { "Edit Bug" };

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(0),
                Constraint::Length(1),
                Constraint::Length(3),
            ])
            .split(area);

        let header = Paragraph::new(Text::from(vec![Line::from(vec![
            Span::styled(title, Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" - Tab:next field, Enter:submit, Esc:cancel"),
        ])]))
        .block(Block::default().borders(Borders::ALL).title("Bug Form"));

        f.render_widget(header, chunks[0]);

        // Render form fields
        let field_lines: Vec<Line> = field_order
            .iter()
            .enumerate()
            .map(|(i, key)| {
                let is_focused = i == focused_field;
                let prefix = if is_focused { "> " } else { "  " };
                let style = if is_focused {
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default()
                };
                let display_value = fields.get(key).map(|s| s.as_str()).unwrap_or("");
                Line::from(vec![
                    Span::styled(prefix, Style::default().fg(Color::Yellow)),
                    Span::styled(format!("{}: ", key), style),
                    Span::raw(display_value),
                ])
            })
            .collect();

        let form = Paragraph::new(Text::from(field_lines))
            .block(Block::default().borders(Borders::ALL))
            .wrap(Wrap { trim: false });

        f.render_widget(form, chunks[1]);

        // Error message
        if let Some(err) = error {
            let error_line = Paragraph::new(Text::from(vec![Line::from(vec![
                Span::styled("Error: ", Style::default().fg(Color::Red)),
                Span::raw(err),
            ])]));
            f.render_widget(error_line, chunks[2]);
        }

        // Footer
        let footer = Paragraph::new(Text::from(vec![Line::from(vec![
            Span::styled("Tab", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(": next  "),
            Span::styled("Enter", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(": submit  "),
            Span::styled("Esc", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(": cancel"),
        ])]));
        f.render_widget(footer, chunks[3]);
    }

    fn render_delete_dialog(
        f: &mut ratatui::Frame,
        area: Rect,
        entity_type: &str,
        id: u64,
        name: String,
        _confirm: bool,
    ) {
        use ratatui::{
            layout::{Constraint, Direction, Layout},
            style::{Color, Modifier, Style},
            text::{Line, Span, Text},
            widgets::{Block, Borders, Paragraph},
        };

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(5),
                Constraint::Length(3),
                Constraint::Min(0),
            ])
            .split(area);

        let dialog = Paragraph::new(Text::from(vec![
            Line::from(Span::raw("")),
            Line::from(Span::styled(
                format!("  Delete {}  ", entity_type),
                Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
            )),
            Line::from(Span::raw("")),
            Line::from(Span::raw(format!("  ID: {}", id))),
            Line::from(Span::raw(format!("  Name: {}", name))),
            Line::from(Span::raw("")),
            Line::from(Span::styled(
                "  Are you sure?  ",
                Style::default().fg(Color::Yellow),
            )),
            Line::from(Span::raw("")),
        ]))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Confirm Delete")
                .title_style(Style::default().fg(Color::Cyan)),
        )
        .alignment(ratatui::layout::Alignment::Center);

        f.render_widget(dialog, chunks[0]);

        let choices = Paragraph::new(Text::from(vec![Line::from(vec![
            Span::styled("Enter", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw("/"),
            Span::styled("Y", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(": delete  "),
            Span::styled("Esc", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw("/"),
            Span::styled("n", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(": cancel"),
        ])]))
        .alignment(ratatui::layout::Alignment::Center);

        f.render_widget(choices, chunks[1]);
    }
}
