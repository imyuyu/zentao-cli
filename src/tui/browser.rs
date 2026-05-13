use anyhow::Result;
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use crossterm::execute;
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
use crate::api::{
    Bug, Build, Department, Execution, Feedback, Product, ProductPlan, Program, Project, Release,
    Story, Task, Testcase, Testtask, Ticket, User,
};
use crate::core::{AppContext, OutputFormat};
use crate::service::{
    bug::BugService, build::BuildService, department::DepartmentService,
    execution::ExecutionService, feedback::FeedbackService, product::ProductService,
    productplan::ProductPlanService, program::ProgramService, project::ProjectService,
    release::ReleaseService, story::StoryService, task::TaskService, testcase::TestcaseService,
    testtask::TesttaskService, ticket::TicketService, user::UserService,
};

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

enum EnterAction {
    BugDetail {
        bug: Bug,
        product_name: Option<String>,
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
}

pub struct Browser {
    terminal: Terminal<CrosstermBackend<Stdout>>,
    pending_products: Option<Vec<Product>>,
    pending_reload: Option<Box<dyn FnOnce(&mut App)>>,
    spinner_frame: usize,
    loading_cancelled: bool,
}

impl Browser {
    pub fn new() -> Result<Self> {
        let stdout = std::io::stdout();
        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend)?;
        Ok(Self {
            terminal,
            pending_products: None,
            pending_reload: None,
            spinner_frame: 0,
            loading_cancelled: false,
        })
    }

    pub fn run(&mut self, app: &mut App) -> Result<()> {
        let rt = tokio::runtime::Runtime::new()?;
        let (tx, rx) = mpsc::channel::<(AppState, Option<String>, Option<String>)>();

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
            if !self.loading_cancelled && !matches!(app.state, AppState::MainMenu { .. }) {
                if let Ok((new_state, product_name, project_name)) = rx.try_recv() {
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

            let selected = app.selected_index;
            let current_module: Option<String> =
                if matches!(app.state, AppState::ModuleSelected { .. }) {
                    if let AppState::ModuleSelected { module_name } = &app.state {
                        let name = module_name.clone();
                        app.state = AppState::Loading {
                            message: format!("Loading {}...", name),
                        };
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
                        // States without custom rendering - use Idle as fallback
                        _ => {
                            Self::render_idle(f, area, app);
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
                                Err(_) => (
                                    AppState::Error {
                                        message: "Failed to load bugs".to_string(),
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
                                Err(_) => (
                                    AppState::Error {
                                        message: "Failed to load stories".to_string(),
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
                                Err(_) => (
                                    AppState::Error {
                                        message: "Failed to load executions".to_string(),
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
                                Err(_) => (
                                    AppState::Error {
                                        message: "Failed to load builds".to_string(),
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
                                Err(_) => (
                                    AppState::Error {
                                        message: "Failed to load releases".to_string(),
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
                                                message: "Failed to load users".to_string(),
                                            },
                                            None,
                                            None,
                                        ),
                                    }
                                } else {
                                    (
                                        AppState::Error {
                                            message: "Failed to load users".to_string(),
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
                                                message: "Failed to load departments".to_string(),
                                            },
                                            None,
                                            None,
                                        ),
                                    }
                                } else {
                                    (
                                        AppState::Error {
                                            message: "Failed to load departments".to_string(),
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
                                                message: "Failed to load products".to_string(),
                                            },
                                            None,
                                            None,
                                        ),
                                    }
                                } else {
                                    (
                                        AppState::Error {
                                            message: "Failed to load products".to_string(),
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
                                                message: "Failed to load projects".to_string(),
                                            },
                                            None,
                                            None,
                                        ),
                                    }
                                } else {
                                    (
                                        AppState::Error {
                                            message: "Failed to load projects".to_string(),
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
                                Err(_) => (
                                    AppState::Error {
                                        message: "Failed to load tasks".to_string(),
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
                                Err(_) => (
                                    AppState::Error {
                                        message: "Failed to load testcases".to_string(),
                                    },
                                    None,
                                    None,
                                ),
                            }
                        }
                        "Testtask List" => match TesttaskService::list(&ctx, 1, 100).await {
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
                                eprintln!("Error loading testtasks (trying token refresh): {}", e);
                                if ctx.refresh_token().await.is_ok() {
                                    match TesttaskService::list(&ctx, 1, 100).await {
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
                                        Err(_) => (
                                            AppState::Error {
                                                message: "Failed to load testtasks".to_string(),
                                            },
                                            None,
                                            None,
                                        ),
                                    }
                                } else {
                                    (
                                        AppState::Error {
                                            message: "Failed to load testtasks".to_string(),
                                        },
                                        None,
                                        None,
                                    )
                                }
                            }
                        },
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
                                                message: "Failed to load feedbacks".to_string(),
                                            },
                                            None,
                                            None,
                                        ),
                                    }
                                } else {
                                    (
                                        AppState::Error {
                                            message: "Failed to load feedbacks".to_string(),
                                        },
                                        None,
                                        None,
                                    )
                                }
                            }
                        },
                        "Ticket List" => match TicketService::list(&ctx, 1, 100).await {
                            Ok(tickets) => (AppState::TicketList { tickets }, None, None),
                            Err(e) => {
                                eprintln!("Error loading tickets (trying token refresh): {}", e);
                                if ctx.refresh_token().await.is_ok() {
                                    match TicketService::list(&ctx, 1, 100).await {
                                        Ok(tickets) => {
                                            (AppState::TicketList { tickets }, None, None)
                                        }
                                        Err(_) => (
                                            AppState::Error {
                                                message: "Failed to load tickets".to_string(),
                                            },
                                            None,
                                            None,
                                        ),
                                    }
                                } else {
                                    (
                                        AppState::Error {
                                            message: "Failed to load tickets".to_string(),
                                        },
                                        None,
                                        None,
                                    )
                                }
                            }
                        },
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
                                                message: "Failed to load programs".to_string(),
                                            },
                                            None,
                                            None,
                                        ),
                                    }
                                } else {
                                    (
                                        AppState::Error {
                                            message: "Failed to load programs".to_string(),
                                        },
                                        None,
                                        None,
                                    )
                                }
                            }
                        },
                        "ProductPlan List" => {
                            let plans = match ProductPlanService::list(&ctx, config.product_id)
                                .await
                            {
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
                                Err(_) => (
                                    AppState::Error {
                                        message: "Failed to load product plans".to_string(),
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
            if crossterm::event::poll(std::time::Duration::from_millis(100))? {
                if let Ok(crossterm::event::Event::Key(key)) = crossterm::event::read() {
                    self.handle_key_event(key, app);
                }
            }
        }
        Ok(())
    }

    fn handle_key_event(&mut self, key: KeyEvent, app: &mut App) {
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
                app.selected_index = app.selected_index.saturating_sub(1);
                // Sync with list_state for MainMenu
                if matches!(app.state, AppState::MainMenu { .. }) {
                    let _ = app.main_menu_state.borrow().selected();
                    app.main_menu_state
                        .borrow_mut()
                        .select(Some(app.selected_index));
                } else {
                    // Sync with list_state for other list states
                    let _ = app.list_state.borrow().selected();
                    app.list_state.borrow_mut().select(Some(app.selected_index));
                }
            }
            KeyCode::Down | KeyCode::Char('j') => {
                let max = match &app.state {
                    AppState::BugList { bugs, .. } => bugs.len().saturating_sub(1),
                    AppState::StoryList { stories, .. } => stories.len().saturating_sub(1),
                    AppState::ExecutionList { executions, .. } => {
                        executions.len().saturating_sub(1)
                    }
                    AppState::BuildList { builds, .. } => builds.len().saturating_sub(1),
                    AppState::ReleaseList { releases, .. } => releases.len().saturating_sub(1),
                    AppState::UserList { users, .. } => users.len().saturating_sub(1),
                    AppState::DepartmentList { departments, .. } => {
                        departments.len().saturating_sub(1)
                    }
                    AppState::ProductList { products, .. } => products.len().saturating_sub(1),
                    AppState::ProjectList { projects, .. } => projects.len().saturating_sub(1),
                    AppState::TaskList { tasks, .. } => tasks.len().saturating_sub(1),
                    AppState::TestcaseList { testcases, .. } => testcases.len().saturating_sub(1),
                    AppState::TesttaskList { testtasks, .. } => testtasks.len().saturating_sub(1),
                    AppState::FeedbackList { feedbacks, .. } => feedbacks.len().saturating_sub(1),
                    AppState::TicketList { tickets, .. } => tickets.len().saturating_sub(1),
                    AppState::ProgramList { programs, .. } => programs.len().saturating_sub(1),
                    AppState::ProductPlanList { plans, .. } => plans.len().saturating_sub(1),
                    AppState::Settings { multi_config, .. } => {
                        multi_config.list_account_names().len().saturating_sub(1)
                    }
                    AppState::ProductSelect { products, .. } => products.len().saturating_sub(1),
                    AppState::AccountSelect { multi_config, .. } => {
                        multi_config.list_account_names().len().saturating_sub(1)
                    }
                    AppState::MainMenu { .. } => {
                        App::get_main_menu_modules().len().saturating_sub(1)
                    }
                    _ => 0,
                };
                if app.selected_index < max {
                    app.selected_index += 1;
                }
                // Sync with list_state
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
                    _ => None,
                };

                // Execute the action after borrow is released
                if let Some(act) = action {
                    app.save_list();
                    match act {
                        EnterAction::BugDetail { bug, product_name } => {
                            app.set_bug_detail(bug, product_name)
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
                    app.set_main_menu();
                }
                AppState::Error { .. } => {
                    // Return to MainMenu from error and cancel any pending loading
                    self.loading_cancelled = true;
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
                _ => {}
            },
            KeyCode::Char('r') | KeyCode::F(5) => {
                if let Some(reload) = self.pending_reload.take() {
                    reload(app);
                }
            }
            KeyCode::Char('o') => {
                let base_url = &app.config.url;
                let url = match &app.state {
                    AppState::BugList { bugs, .. } => {
                        bugs.get(app.selected_index).map(|b| b.web_url(base_url))
                    }
                    AppState::StoryList { stories, .. } => {
                        stories.get(app.selected_index).map(|s| s.web_url(base_url))
                    }
                    AppState::ReleaseList { releases, .. } => releases
                        .get(app.selected_index)
                        .map(|r| r.web_url(base_url)),
                    AppState::UserList { users } => {
                        users.get(app.selected_index).map(|u| u.web_url(base_url))
                    }
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
                };

                if let Some(url) = url {
                    let _ = open::that(&url);
                }
            }
            KeyCode::Char('y') | KeyCode::Char('Y') => {
                if matches!(app.state, AppState::ConfirmQuit) {
                    app.quit();
                }
            }
            _ => {}
        }
    }

    fn render_help_overlay(f: &mut ratatui::Frame, area: Rect) {
        let chunks = Layout::default()
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

    fn render_bug_list(
        f: &mut ratatui::Frame,
        area: Rect,
        bugs: &[crate::api::Bug],
        selected: usize,
        app: &App,
    ) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Header with search
                Constraint::Min(0),    // List
                Constraint::Length(3), // Footer with shortcuts
            ])
            .split(area);

        // Search hint in header
        let search_hint = if app.search_active {
            format!("[SEARCH: {}] (Esc to cancel)", app.search_query)
        } else {
            "[Ctrl+F] Search".to_string()
        };
        let header = Paragraph::new(Text::from(vec![Line::from(vec![
            Span::styled("Bug List", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" ("),
            Span::raw(format!("{}", bugs.len())),
            Span::raw(" items)  |  "),
            Span::styled(&search_hint, Style::default().fg(Color::Cyan)),
        ])]))
        .block(Block::default().borders(Borders::ALL).title("ZenTao"));

        f.render_widget(header, chunks[0]);

        // Filter bugs if search is active
        let display_bugs: Vec<_> = if app.search_query.is_empty() {
            bugs.iter().collect()
        } else {
            let query = app.search_query.to_lowercase();
            bugs.iter()
                .filter(|b| {
                    b.title.to_lowercase().contains(&query)
                        || b.status.to_lowercase().contains(&query)
                        || format!("{}", b.id).contains(&query)
                })
                .collect()
        };

        let items: Vec<ListItem> = display_bugs
            .iter()
            .map(|bug| {
                ListItem::new(Line::from(vec![
                    Span::raw(format!("{:6}", bug.id)),
                    Span::raw(" "),
                    Span::styled(&bug.title, Style::default()),
                    Span::raw(" | "),
                    Span::styled(
                        format!("[{}]", bug.status),
                        match bug.status.as_str() {
                            "active" => Style::default().fg(Color::Green),
                            "closed" => Style::default().fg(Color::Red),
                            _ => Style::default().fg(Color::Yellow),
                        },
                    ),
                    Span::raw(" | "),
                    Span::styled(format!("Pri:{}", bug.pri), Style::default().fg(Color::Blue)),
                    Span::raw(" | "),
                    Span::styled(
                        format!("Sev:{}", bug.severity),
                        match bug.severity {
                            1 => Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
                            2 => Style::default().fg(Color::Yellow),
                            _ => Style::default().fg(Color::DarkGray),
                        },
                    ),
                ]))
            })
            .collect();

        let list = List::new(items)
            .block(Block::default().borders(Borders::ALL))
            .highlight_style(Style::default().bg(Color::Blue).fg(Color::White));

        f.render_stateful_widget(list, chunks[1], &mut app.list_state.borrow_mut());

        // Footer with navigation hints
        let footer = Paragraph::new(Text::from(vec![Line::from(vec![
            Span::styled("↑↓", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw("/"),
            Span::styled("jk", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" nav  "),
            Span::styled("Enter", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" view  "),
            Span::styled("?", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" help  "),
            Span::styled("o", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" open  "),
            Span::styled("q", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw("/"),
            Span::styled("Esc", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" quit  |  "),
            Span::raw(format!("Selected: {} / {}", selected + 1, bugs.len())),
        ])]));
        f.render_widget(footer, chunks[2]);
    }

    fn render_bug_detail(f: &mut ratatui::Frame, area: Rect, bug: &crate::api::Bug) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Length(10),
                Constraint::Min(0),
                Constraint::Length(3),
            ])
            .split(area);

        let title = Paragraph::new(Text::from(vec![Line::from(vec![
            Span::raw(format!("Bug #{} - ", bug.id)),
            Span::styled(&bug.title, Style::default().add_modifier(Modifier::BOLD)),
        ])]))
        .block(Block::default().borders(Borders::ALL).title("Bug Detail"));

        f.render_widget(title, chunks[0]);

        let details = Paragraph::new(Text::from(vec![
            Line::from(vec![Span::raw("Status: "), Span::raw(&bug.status)]),
            Line::from(vec![
                Span::raw("Severity: "),
                Span::raw(format!("{}", bug.severity)),
            ]),
            Line::from(vec![
                Span::raw("Priority: "),
                Span::raw(format!("{}", bug.pri)),
            ]),
            Line::from(vec![
                Span::raw("Resolution: "),
                Span::raw(bug.resolution.as_deref().unwrap_or("N/A")),
            ]),
            Line::from(vec![
                Span::raw("Assigned: "),
                Span::raw(bug.assigned_to.as_deref().unwrap_or("Unassigned")),
            ]),
        ]))
        .block(Block::default().borders(Borders::ALL).title("Details"));

        f.render_widget(details, chunks[1]);

        // Strip HTML tags from steps
        let steps_text = bug
            .steps
            .as_deref()
            .unwrap_or("No reproduction steps provided.");
        let steps_text = strip_html_tags(steps_text);
        // Split by newline to create multiple lines
        let steps_lines: Vec<Line> = steps_text
            .split('\n')
            .map(|s| Line::from(Span::raw(s)))
            .collect();
        let steps = Paragraph::new(Text::from(steps_lines))
            .block(Block::default().borders(Borders::ALL).title("Steps"));

        f.render_widget(steps, chunks[2]);

        let footer = Paragraph::new(Text::from(vec![Line::from(vec![
            Span::styled("q", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" back  "),
            Span::styled("o", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" open"),
        ])]));

        f.render_widget(footer, chunks[3]);
    }

    fn render_story_list(
        f: &mut ratatui::Frame,
        area: Rect,
        stories: &[crate::api::Story],
        selected: usize,
        app: &App,
    ) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Header with search
                Constraint::Min(0),    // List
                Constraint::Length(3), // Footer with shortcuts
            ])
            .split(area);

        // Search hint in header
        let search_hint = if app.search_active {
            format!("[SEARCH: {}] (Esc to cancel)", app.search_query)
        } else {
            "[Ctrl+F] Search".to_string()
        };
        let header = Paragraph::new(Text::from(vec![Line::from(vec![
            Span::styled("Story List", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" ("),
            Span::raw(format!("{}", stories.len())),
            Span::raw(" items)  |  "),
            Span::styled(&search_hint, Style::default().fg(Color::Cyan)),
        ])]))
        .block(Block::default().borders(Borders::ALL).title("ZenTao"));

        f.render_widget(header, chunks[0]);

        // Filter stories if search is active
        let display_stories: Vec<_> = if app.search_query.is_empty() {
            stories.iter().collect()
        } else {
            let query = app.search_query.to_lowercase();
            stories
                .iter()
                .filter(|s| {
                    s.title.to_lowercase().contains(&query)
                        || s.status.to_lowercase().contains(&query)
                        || format!("{}", s.id).contains(&query)
                })
                .collect()
        };

        let items: Vec<ListItem> = display_stories
            .iter()
            .map(|story| {
                ListItem::new(Line::from(vec![
                    Span::raw(format!("{:6}", story.id)),
                    Span::raw(" "),
                    Span::styled(&story.title, Style::default()),
                    Span::raw(" | "),
                    Span::styled(
                        format!("[{}]", story.status),
                        match story.status.as_str() {
                            "active" => Style::default().fg(Color::Green),
                            "closed" => Style::default().fg(Color::Red),
                            _ => Style::default().fg(Color::Yellow),
                        },
                    ),
                    Span::raw(" | "),
                    Span::styled(
                        format!("Pri:{}", story.pri),
                        Style::default().fg(Color::Blue),
                    ),
                    Span::raw(" | "),
                    Span::raw(format!(
                        "{}",
                        story
                            .estimate
                            .map(|e| format!("{}h", e))
                            .unwrap_or_default()
                    )),
                ]))
            })
            .collect();

        let list = List::new(items)
            .block(Block::default().borders(Borders::ALL))
            .highlight_style(Style::default().bg(Color::Blue).fg(Color::White));

        f.render_stateful_widget(list, chunks[1], &mut app.list_state.borrow_mut());

        // Footer with navigation hints
        let footer = Paragraph::new(Text::from(vec![Line::from(vec![
            Span::styled("↑↓", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw("/"),
            Span::styled("jk", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" nav  "),
            Span::styled("Enter", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" view  "),
            Span::styled("?", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" help  "),
            Span::styled("o", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" open  "),
            Span::styled("q", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw("/"),
            Span::styled("Esc", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" quit  |  "),
            Span::raw(format!("Selected: {} / {}", selected + 1, stories.len())),
        ])]));
        f.render_widget(footer, chunks[2]);
    }

    fn render_story_detail(f: &mut ratatui::Frame, area: Rect, story: &crate::api::Story) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Length(10),
                Constraint::Min(0),
                Constraint::Length(3),
            ])
            .split(area);

        let title = Paragraph::new(Text::from(vec![Line::from(vec![
            Span::raw(format!("Story #{} - ", story.id)),
            Span::styled(&story.title, Style::default().add_modifier(Modifier::BOLD)),
        ])]))
        .block(Block::default().borders(Borders::ALL).title("Story Detail"));

        f.render_widget(title, chunks[0]);

        let details = Paragraph::new(Text::from(vec![
            Line::from(vec![Span::raw("Status: "), Span::raw(&story.status)]),
            Line::from(vec![
                Span::raw("Priority: "),
                Span::raw(format!("{}", story.pri)),
            ]),
            Line::from(vec![
                Span::raw("Stage: "),
                Span::raw(story.stage.as_deref().unwrap_or("N/A")),
            ]),
            Line::from(vec![
                Span::raw("Assigned: "),
                Span::raw(story.assigned_to.as_deref().unwrap_or("Unassigned")),
            ]),
            Line::from(vec![
                Span::raw("Estimate: "),
                Span::raw(
                    story
                        .estimate
                        .map(|e| format!("{}h", e))
                        .unwrap_or_else(|| "N/A".to_string()),
                ),
            ]),
        ]))
        .block(Block::default().borders(Borders::ALL).title("Details"));

        f.render_widget(details, chunks[1]);

        let desc = Paragraph::new(Text::from(vec![Line::from(Span::raw(
            story
                .description
                .as_deref()
                .unwrap_or("No description provided."),
        ))]))
        .block(Block::default().borders(Borders::ALL).title("Description"));

        f.render_widget(desc, chunks[2]);

        let footer = Paragraph::new(Text::from(vec![Line::from(vec![
            Span::styled("q", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" back  "),
            Span::styled("o", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" open"),
        ])]));

        f.render_widget(footer, chunks[3]);
    }

    // ============================================================
    // Execution Render Functions
    // ============================================================

    fn render_execution_list(
        f: &mut ratatui::Frame,
        area: Rect,
        executions: &[crate::api::Execution],
        selected: usize,
        app: &App,
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
                "Execution List",
                Style::default().add_modifier(Modifier::BOLD),
            ),
            Span::raw(" ("),
            Span::raw(format!("{}", executions.len())),
            Span::raw(" items)"),
        ])]))
        .block(Block::default().borders(Borders::ALL).title("ZenTao"));

        f.render_widget(header, chunks[0]);

        let items: Vec<ListItem> = executions
            .iter()
            .map(|exec| {
                ListItem::new(Line::from(vec![
                    Span::raw(format!("{:6}", exec.id)),
                    Span::raw(" "),
                    Span::styled(&exec.name, Style::default()),
                    Span::raw(" | "),
                    Span::styled(
                        format!("[{}]", exec.status),
                        match exec.status.as_str() {
                            "doing" => Style::default().fg(Color::Green),
                            "closed" => Style::default().fg(Color::Red),
                            "suspended" => Style::default().fg(Color::Yellow),
                            _ => Style::default().fg(Color::Blue),
                        },
                    ),
                    Span::raw(" | "),
                    Span::raw(exec.begin.clone().unwrap_or_else(|| "-".to_string())),
                    Span::raw(" ~ "),
                    Span::raw(exec.end.clone().unwrap_or_else(|| "-".to_string())),
                ]))
            })
            .collect();

        let list = List::new(items)
            .block(Block::default().borders(Borders::ALL))
            .highlight_style(Style::default().bg(Color::Blue).fg(Color::White));

        f.render_stateful_widget(list, chunks[1], &mut app.list_state.borrow_mut());

        let footer = Paragraph::new(Text::from(vec![Line::from(vec![
            Span::styled("↑↓", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw("/"),
            Span::styled("jk", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" nav  "),
            Span::styled("Enter", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" view  "),
            Span::styled("q", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw("/"),
            Span::styled("Esc", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" back  |  "),
            Span::raw(format!("Selected: {} / {}", selected + 1, executions.len())),
        ])]));
        f.render_widget(footer, chunks[2]);
    }

    fn render_execution_detail(
        f: &mut ratatui::Frame,
        area: Rect,
        execution: &crate::api::Execution,
    ) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Length(10),
                Constraint::Min(0),
                Constraint::Length(3),
            ])
            .split(area);

        let title = Paragraph::new(Text::from(vec![Line::from(vec![
            Span::raw(format!("Execution #{} - ", execution.id)),
            Span::styled(
                &execution.name,
                Style::default().add_modifier(Modifier::BOLD),
            ),
        ])]))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Execution Detail"),
        );

        f.render_widget(title, chunks[0]);

        let details = Paragraph::new(Text::from(vec![
            Line::from(vec![Span::raw("Status: "), Span::raw(&execution.status)]),
            Line::from(vec![
                Span::raw("Type: "),
                Span::raw(execution.type_.as_deref().unwrap_or("N/A")),
            ]),
            Line::from(vec![
                Span::raw("Project: "),
                Span::raw(format!("{}", execution.project)),
            ]),
            Line::from(vec![
                Span::raw("Begin: "),
                Span::raw(execution.begin.as_deref().unwrap_or("N/A")),
            ]),
            Line::from(vec![
                Span::raw("End: "),
                Span::raw(execution.end.as_deref().unwrap_or("N/A")),
            ]),
            Line::from(vec![
                Span::raw("Days: "),
                Span::raw(format!(
                    "{}",
                    execution
                        .days
                        .map(|d| d.to_string())
                        .unwrap_or_else(|| "N/A".to_string())
                )),
            ]),
        ]))
        .block(Block::default().borders(Borders::ALL).title("Details"));

        f.render_widget(details, chunks[1]);

        let desc = Paragraph::new(Text::from(vec![Line::from(Span::raw(
            execution
                .desc
                .as_deref()
                .unwrap_or("No description provided."),
        ))]))
        .block(Block::default().borders(Borders::ALL).title("Description"));

        f.render_widget(desc, chunks[2]);

        let footer = Paragraph::new(Text::from(vec![Line::from(vec![
            Span::styled("q", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" back  "),
            Span::styled("o", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" open"),
        ])]));

        f.render_widget(footer, chunks[3]);
    }

    // ============================================================
    // Build Render Functions
    // ============================================================

    fn render_build_list(
        f: &mut ratatui::Frame,
        area: Rect,
        builds: &[crate::api::Build],
        selected: usize,
        app: &App,
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
            Span::styled("Build List", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" ("),
            Span::raw(format!("{}", builds.len())),
            Span::raw(" items)"),
        ])]))
        .block(Block::default().borders(Borders::ALL).title("ZenTao"));

        f.render_widget(header, chunks[0]);

        let items: Vec<ListItem> = builds
            .iter()
            .map(|build| {
                ListItem::new(Line::from(vec![
                    Span::raw(format!("{:6}", build.id)),
                    Span::raw(" "),
                    Span::styled(&build.name, Style::default()),
                    Span::raw(" | "),
                    Span::raw(format!(
                        "Stories: {}",
                        build.stories.as_deref().unwrap_or("0")
                    )),
                    Span::raw(" | "),
                    Span::raw(format!("Bugs: {}", build.bugs.as_deref().unwrap_or("0"))),
                ]))
            })
            .collect();

        let list = List::new(items)
            .block(Block::default().borders(Borders::ALL))
            .highlight_style(Style::default().bg(Color::Blue).fg(Color::White));

        f.render_stateful_widget(list, chunks[1], &mut app.list_state.borrow_mut());

        let footer = Paragraph::new(Text::from(vec![Line::from(vec![
            Span::styled("↑↓", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw("/"),
            Span::styled("jk", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" nav  "),
            Span::styled("Enter", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" view  "),
            Span::styled("q", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw("/"),
            Span::styled("Esc", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" back  |  "),
            Span::raw(format!("Selected: {} / {}", selected + 1, builds.len())),
        ])]));
        f.render_widget(footer, chunks[2]);
    }

    fn render_build_detail(f: &mut ratatui::Frame, area: Rect, build: &crate::api::Build) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Length(12),
                Constraint::Min(0),
                Constraint::Length(3),
            ])
            .split(area);

        let title = Paragraph::new(Text::from(vec![Line::from(vec![
            Span::raw(format!("Build #{} - ", build.id)),
            Span::styled(&build.name, Style::default().add_modifier(Modifier::BOLD)),
        ])]))
        .block(Block::default().borders(Borders::ALL).title("Build Detail"));

        f.render_widget(title, chunks[0]);

        let details = Paragraph::new(Text::from(vec![
            Line::from(vec![
                Span::raw("Product: "),
                Span::raw(format!("{}", build.product)),
            ]),
            Line::from(vec![
                Span::raw("Project: "),
                Span::raw(format!("{}", build.project)),
            ]),
            Line::from(vec![
                Span::raw("Branch: "),
                Span::raw(format!(
                    "{}",
                    build
                        .branch
                        .map(|b| b.to_string())
                        .unwrap_or_else(|| "N/A".to_string())
                )),
            ]),
            Line::from(vec![
                Span::raw("SCM Path: "),
                Span::raw(build.scm_path.as_deref().unwrap_or("N/A")),
            ]),
            Line::from(vec![
                Span::raw("CI: "),
                Span::raw(build.ci.as_deref().unwrap_or("N/A")),
            ]),
            Line::from(vec![
                Span::raw("Package: "),
                Span::raw(build.pkg.as_deref().unwrap_or("N/A")),
            ]),
            Line::from(vec![
                Span::raw("File Size: "),
                Span::raw(build.file_size.as_deref().unwrap_or("N/A")),
            ]),
            Line::from(vec![
                Span::raw("Generated: "),
                Span::raw(build.generated_at.as_deref().unwrap_or("N/A")),
            ]),
            Line::from(vec![
                Span::raw("Stories: "),
                Span::raw(build.stories.as_deref().unwrap_or("0")),
            ]),
            Line::from(vec![
                Span::raw("Bugs: "),
                Span::raw(build.bugs.as_deref().unwrap_or("0")),
            ]),
        ]))
        .block(Block::default().borders(Borders::ALL).title("Details"));

        f.render_widget(details, chunks[1]);

        let footer = Paragraph::new(Text::from(vec![Line::from(vec![
            Span::styled("q", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" back  "),
            Span::styled("o", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" open"),
        ])]));

        f.render_widget(footer, chunks[3]);
    }

    // ============================================================
    // Release Render Functions
    // ============================================================

    fn render_release_list(
        f: &mut ratatui::Frame,
        area: Rect,
        releases: &[Release],
        selected: usize,
        app: &App,
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
                "Release List",
                Style::default().add_modifier(Modifier::BOLD),
            ),
            Span::raw(" ("),
            Span::raw(format!("{}", releases.len())),
            Span::raw(" items)"),
        ])]))
        .block(Block::default().borders(Borders::ALL).title("ZenTao"));

        f.render_widget(header, chunks[0]);

        let items: Vec<ListItem> = releases
            .iter()
            .map(|release| {
                ListItem::new(Line::from(vec![
                    Span::raw(format!("{:6}", release.id)),
                    Span::raw(" "),
                    Span::styled(&release.name, Style::default()),
                    Span::raw(" | "),
                    Span::styled(
                        format!("[{}]", release.status),
                        match release.status.as_str() {
                            "normal" => Style::default().fg(Color::Green),
                            "closed" => Style::default().fg(Color::Red),
                            _ => Style::default().fg(Color::Yellow),
                        },
                    ),
                    Span::raw(" | "),
                    Span::raw(release.marker.as_deref().unwrap_or("-")),
                ]))
            })
            .collect();

        let list = List::new(items)
            .block(Block::default().borders(Borders::ALL))
            .highlight_style(Style::default().bg(Color::Blue).fg(Color::White));

        f.render_stateful_widget(list, chunks[1], &mut app.list_state.borrow_mut());

        let footer = Paragraph::new(Text::from(vec![Line::from(vec![
            Span::styled("↑↓", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw("/"),
            Span::styled("jk", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" nav  "),
            Span::styled("Enter", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" view  "),
            Span::styled("q", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw("/"),
            Span::styled("Esc", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" back  |  "),
            Span::raw(format!("Selected: {} / {}", selected + 1, releases.len())),
        ])]));
        f.render_widget(footer, chunks[2]);
    }

    fn render_release_detail(f: &mut ratatui::Frame, area: Rect, release: &Release) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Length(10),
                Constraint::Min(0),
                Constraint::Length(3),
            ])
            .split(area);

        let title = Paragraph::new(Text::from(vec![Line::from(vec![
            Span::raw(format!("Release #{} - ", release.id)),
            Span::styled(&release.name, Style::default().add_modifier(Modifier::BOLD)),
        ])]))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Release Detail"),
        );

        f.render_widget(title, chunks[0]);

        let details = Paragraph::new(Text::from(vec![
            Line::from(vec![Span::raw("Status: "), Span::raw(&release.status)]),
            Line::from(vec![
                Span::raw("Product: "),
                Span::raw(
                    release
                        .product
                        .map(|p| p.to_string())
                        .unwrap_or_else(|| "N/A".to_string()),
                ),
            ]),
            Line::from(vec![
                Span::raw("Build: "),
                Span::raw(format!(
                    "{}",
                    release
                        .build
                        .map(|b| b.to_string())
                        .unwrap_or_else(|| "N/A".to_string())
                )),
            ]),
            Line::from(vec![
                Span::raw("Marker: "),
                Span::raw(release.marker.as_deref().unwrap_or("N/A")),
            ]),
            Line::from(vec![
                Span::raw("Date: "),
                Span::raw(release.date.as_deref().unwrap_or("N/A")),
            ]),
        ]))
        .block(Block::default().borders(Borders::ALL).title("Details"));

        f.render_widget(details, chunks[1]);

        let footer = Paragraph::new(Text::from(vec![Line::from(vec![
            Span::styled("q", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" back  "),
            Span::styled("o", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" open"),
        ])]));

        f.render_widget(footer, chunks[3]);
    }

    // ============================================================
    // User Render Functions
    // ============================================================

    fn render_user_list(
        f: &mut ratatui::Frame,
        area: Rect,
        users: &[User],
        selected: usize,
        app: &App,
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
            Span::styled("User List", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" ("),
            Span::raw(format!("{}", users.len())),
            Span::raw(" items)"),
        ])]))
        .block(Block::default().borders(Borders::ALL).title("ZenTao"));

        f.render_widget(header, chunks[0]);

        let items: Vec<ListItem> = users
            .iter()
            .map(|user| {
                ListItem::new(Line::from(vec![
                    Span::raw(format!("{:6}", user.id)),
                    Span::raw(" "),
                    Span::styled(&user.account, Style::default()),
                    Span::raw(" ("),
                    Span::raw(&user.realname),
                    Span::raw(")"),
                    Span::raw(" | "),
                    Span::raw(user.role.as_deref().unwrap_or("-")),
                ]))
            })
            .collect();

        let list = List::new(items)
            .block(Block::default().borders(Borders::ALL))
            .highlight_style(Style::default().bg(Color::Blue).fg(Color::White));

        f.render_stateful_widget(list, chunks[1], &mut app.list_state.borrow_mut());

        let footer = Paragraph::new(Text::from(vec![Line::from(vec![
            Span::styled("↑↓", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw("/"),
            Span::styled("jk", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" nav  "),
            Span::styled("Enter", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" view  "),
            Span::styled("q", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw("/"),
            Span::styled("Esc", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" back  |  "),
            Span::raw(format!("Selected: {} / {}", selected + 1, users.len())),
        ])]));
        f.render_widget(footer, chunks[2]);
    }

    fn render_user_detail(f: &mut ratatui::Frame, area: Rect, user: &User) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Length(8),
                Constraint::Min(0),
                Constraint::Length(3),
            ])
            .split(area);

        let title = Paragraph::new(Text::from(vec![Line::from(vec![
            Span::raw(format!("User #{} - ", user.id)),
            Span::styled(
                &user.realname,
                Style::default().add_modifier(Modifier::BOLD),
            ),
            Span::raw(" ("),
            Span::raw(&user.account),
            Span::raw(")"),
        ])]))
        .block(Block::default().borders(Borders::ALL).title("User Detail"));

        f.render_widget(title, chunks[0]);

        let details = Paragraph::new(Text::from(vec![
            Line::from(vec![Span::raw("Account: "), Span::raw(&user.account)]),
            Line::from(vec![Span::raw("Realname: "), Span::raw(&user.realname)]),
            Line::from(vec![
                Span::raw("Email: "),
                Span::raw(user.email.as_deref().unwrap_or("N/A")),
            ]),
            Line::from(vec![
                Span::raw("Dept: "),
                Span::raw(format!(
                    "{}",
                    user.dept
                        .map(|d| d.to_string())
                        .unwrap_or_else(|| "N/A".to_string())
                )),
            ]),
            Line::from(vec![
                Span::raw("Role: "),
                Span::raw(user.role.as_deref().unwrap_or("N/A")),
            ]),
        ]))
        .block(Block::default().borders(Borders::ALL).title("Details"));

        f.render_widget(details, chunks[1]);

        let footer = Paragraph::new(Text::from(vec![Line::from(vec![
            Span::styled("q", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" back  "),
            Span::styled("o", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" open"),
        ])]));

        f.render_widget(footer, chunks[3]);
    }

    // ============================================================
    // Department Render Functions
    // ============================================================

    fn render_department_list(
        f: &mut ratatui::Frame,
        area: Rect,
        departments: &[Department],
        selected: usize,
        app: &App,
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
                "Department List",
                Style::default().add_modifier(Modifier::BOLD),
            ),
            Span::raw(" ("),
            Span::raw(format!("{}", departments.len())),
            Span::raw(" items)"),
        ])]))
        .block(Block::default().borders(Borders::ALL).title("ZenTao"));

        f.render_widget(header, chunks[0]);

        let items: Vec<ListItem> = departments
            .iter()
            .map(|dept| {
                ListItem::new(Line::from(vec![
                    Span::raw(format!("{:6}", dept.id)),
                    Span::raw(" "),
                    Span::styled(&dept.name, Style::default()),
                    Span::raw(" | "),
                    Span::raw(format!(
                        "Parent: {}",
                        dept.parent
                            .map(|p| p.to_string())
                            .unwrap_or_else(|| "root".to_string())
                    )),
                ]))
            })
            .collect();

        let list = List::new(items)
            .block(Block::default().borders(Borders::ALL))
            .highlight_style(Style::default().bg(Color::Blue).fg(Color::White));

        f.render_stateful_widget(list, chunks[1], &mut app.list_state.borrow_mut());

        let footer = Paragraph::new(Text::from(vec![Line::from(vec![
            Span::styled("↑↓", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw("/"),
            Span::styled("jk", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" nav  "),
            Span::styled("Enter", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" view  "),
            Span::styled("q", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw("/"),
            Span::styled("Esc", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" back  |  "),
            Span::raw(format!(
                "Selected: {} / {}",
                selected + 1,
                departments.len()
            )),
        ])]));
        f.render_widget(footer, chunks[2]);
    }

    fn render_department_detail(f: &mut ratatui::Frame, area: Rect, department: &Department) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Length(8),
                Constraint::Min(0),
                Constraint::Length(3),
            ])
            .split(area);

        let title = Paragraph::new(Text::from(vec![Line::from(vec![
            Span::raw(format!("Department #{} - ", department.id)),
            Span::styled(
                &department.name,
                Style::default().add_modifier(Modifier::BOLD),
            ),
        ])]))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Department Detail"),
        );

        f.render_widget(title, chunks[0]);

        let details = Paragraph::new(Text::from(vec![
            Line::from(vec![
                Span::raw("ID: "),
                Span::raw(format!("{}", department.id)),
            ]),
            Line::from(vec![Span::raw("Name: "), Span::raw(&department.name)]),
            Line::from(vec![
                Span::raw("Parent: "),
                Span::raw(format!(
                    "{}",
                    department
                        .parent
                        .map(|p| p.to_string())
                        .unwrap_or_else(|| "root".to_string())
                )),
            ]),
            Line::from(vec![
                Span::raw("Order: "),
                Span::raw(format!(
                    "{}",
                    department
                        .order
                        .map(|o| o.to_string())
                        .unwrap_or_else(|| "N/A".to_string())
                )),
            ]),
            Line::from(vec![
                Span::raw("Path: "),
                Span::raw(department.path.as_deref().unwrap_or("N/A")),
            ]),
        ]))
        .block(Block::default().borders(Borders::ALL).title("Details"));

        f.render_widget(details, chunks[1]);

        let footer = Paragraph::new(Text::from(vec![Line::from(vec![
            Span::styled("q", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" back  "),
            Span::styled("o", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" open"),
        ])]));

        f.render_widget(footer, chunks[3]);
    }

    // ============================================================
    // Product Render Functions
    // ============================================================

    fn render_product_list(
        f: &mut ratatui::Frame,
        area: Rect,
        products: &[Product],
        selected: usize,
        app: &App,
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
                "Product List",
                Style::default().add_modifier(Modifier::BOLD),
            ),
            Span::raw(" ("),
            Span::raw(format!("{}", products.len())),
            Span::raw(" items)"),
        ])]))
        .block(Block::default().borders(Borders::ALL).title("ZenTao"));

        f.render_widget(header, chunks[0]);

        let items: Vec<ListItem> = products
            .iter()
            .map(|product| {
                ListItem::new(Line::from(vec![
                    Span::raw(format!("{:6}", product.id)),
                    Span::raw(" "),
                    Span::styled(&product.name, Style::default()),
                    Span::raw(" ("),
                    Span::raw(&product.code),
                    Span::raw(") | "),
                    Span::styled(
                        format!("[{}]", product.status),
                        match product.status.as_str() {
                            "normal" => Style::default().fg(Color::Green),
                            "closed" => Style::default().fg(Color::Red),
                            _ => Style::default().fg(Color::Yellow),
                        },
                    ),
                ]))
            })
            .collect();

        let list = List::new(items)
            .block(Block::default().borders(Borders::ALL))
            .highlight_style(Style::default().bg(Color::Blue).fg(Color::White));

        f.render_stateful_widget(list, chunks[1], &mut app.list_state.borrow_mut());

        let footer = Paragraph::new(Text::from(vec![Line::from(vec![
            Span::styled("↑↓", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw("/"),
            Span::styled("jk", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" nav  "),
            Span::styled("Enter", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" view  "),
            Span::styled("q", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw("/"),
            Span::styled("Esc", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" back  |  "),
            Span::raw(format!("Selected: {} / {}", selected + 1, products.len())),
        ])]));
        f.render_widget(footer, chunks[2]);
    }

    fn render_product_detail(f: &mut ratatui::Frame, area: Rect, product: &Product) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Length(8),
                Constraint::Min(0),
                Constraint::Length(3),
            ])
            .split(area);

        let title = Paragraph::new(Text::from(vec![Line::from(vec![
            Span::raw(format!("Product #{} - ", product.id)),
            Span::styled(&product.name, Style::default().add_modifier(Modifier::BOLD)),
        ])]))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Product Detail"),
        );

        f.render_widget(title, chunks[0]);

        let details = Paragraph::new(Text::from(vec![
            Line::from(vec![
                Span::raw("ID: "),
                Span::raw(format!("{}", product.id)),
            ]),
            Line::from(vec![Span::raw("Name: "), Span::raw(&product.name)]),
            Line::from(vec![Span::raw("Code: "), Span::raw(&product.code)]),
            Line::from(vec![Span::raw("Status: "), Span::raw(&product.status)]),
        ]))
        .block(Block::default().borders(Borders::ALL).title("Details"));

        f.render_widget(details, chunks[1]);

        let desc = Paragraph::new(Text::from(vec![Line::from(Span::raw(
            product
                .desc
                .as_deref()
                .unwrap_or("No description provided."),
        ))]))
        .block(Block::default().borders(Borders::ALL).title("Description"));

        f.render_widget(desc, chunks[2]);

        let footer = Paragraph::new(Text::from(vec![Line::from(vec![
            Span::styled("q", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" back  "),
            Span::styled("o", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" open"),
        ])]));

        f.render_widget(footer, chunks[3]);
    }

    // ============================================================
    // Project Render Functions
    // ============================================================

    fn render_project_list(
        f: &mut ratatui::Frame,
        area: Rect,
        projects: &[Project],
        selected: usize,
        app: &App,
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
                "Project List",
                Style::default().add_modifier(Modifier::BOLD),
            ),
            Span::raw(" ("),
            Span::raw(format!("{}", projects.len())),
            Span::raw(" items)"),
        ])]))
        .block(Block::default().borders(Borders::ALL).title("ZenTao"));

        f.render_widget(header, chunks[0]);

        let items: Vec<ListItem> = projects
            .iter()
            .map(|project| {
                ListItem::new(Line::from(vec![
                    Span::raw(format!("{:6}", project.id)),
                    Span::raw(" "),
                    Span::styled(&project.name, Style::default()),
                    Span::raw(" ("),
                    Span::raw(&project.code),
                    Span::raw(") | "),
                    Span::styled(
                        format!("[{}]", project.status),
                        match project.status.as_str() {
                            "doing" => Style::default().fg(Color::Green),
                            "closed" => Style::default().fg(Color::Red),
                            "wait" => Style::default().fg(Color::Yellow),
                            _ => Style::default().fg(Color::Blue),
                        },
                    ),
                ]))
            })
            .collect();

        let list = List::new(items)
            .block(Block::default().borders(Borders::ALL))
            .highlight_style(Style::default().bg(Color::Blue).fg(Color::White));

        f.render_stateful_widget(list, chunks[1], &mut app.list_state.borrow_mut());

        let footer = Paragraph::new(Text::from(vec![Line::from(vec![
            Span::styled("↑↓", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw("/"),
            Span::styled("jk", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" nav  "),
            Span::styled("Enter", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" view  "),
            Span::styled("q", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw("/"),
            Span::styled("Esc", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" back  |  "),
            Span::raw(format!("Selected: {} / {}", selected + 1, projects.len())),
        ])]));
        f.render_widget(footer, chunks[2]);
    }

    fn render_project_detail(f: &mut ratatui::Frame, area: Rect, project: &Project) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Length(8),
                Constraint::Min(0),
                Constraint::Length(3),
            ])
            .split(area);

        let title = Paragraph::new(Text::from(vec![Line::from(vec![
            Span::raw(format!("Project #{} - ", project.id)),
            Span::styled(&project.name, Style::default().add_modifier(Modifier::BOLD)),
        ])]))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Project Detail"),
        );

        f.render_widget(title, chunks[0]);

        let details = Paragraph::new(Text::from(vec![
            Line::from(vec![
                Span::raw("ID: "),
                Span::raw(format!("{}", project.id)),
            ]),
            Line::from(vec![Span::raw("Name: "), Span::raw(&project.name)]),
            Line::from(vec![Span::raw("Code: "), Span::raw(&project.code)]),
            Line::from(vec![Span::raw("Status: "), Span::raw(&project.status)]),
        ]))
        .block(Block::default().borders(Borders::ALL).title("Details"));

        f.render_widget(details, chunks[1]);

        let desc = Paragraph::new(Text::from(vec![Line::from(Span::raw(
            project
                .desc
                .as_deref()
                .unwrap_or("No description provided."),
        ))]))
        .block(Block::default().borders(Borders::ALL).title("Description"));

        f.render_widget(desc, chunks[2]);

        let footer = Paragraph::new(Text::from(vec![Line::from(vec![
            Span::styled("q", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" back  "),
            Span::styled("o", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" open"),
        ])]));

        f.render_widget(footer, chunks[3]);
    }

    // ============================================================
    // Task Render Functions
    // ============================================================

    fn render_task_list(
        f: &mut ratatui::Frame,
        area: Rect,
        tasks: &[Task],
        selected: usize,
        app: &App,
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
            Span::styled("Task List", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" ("),
            Span::raw(format!("{}", tasks.len())),
            Span::raw(" items)"),
        ])]))
        .block(Block::default().borders(Borders::ALL).title("ZenTao"));

        f.render_widget(header, chunks[0]);

        let items: Vec<ListItem> = tasks
            .iter()
            .map(|task| {
                ListItem::new(Line::from(vec![
                    Span::raw(format!("{:6}", task.id)),
                    Span::raw(" "),
                    Span::styled(&task.name, Style::default()),
                    Span::raw(" | "),
                    Span::styled(
                        format!("[{}]", task.status),
                        match task.status.as_str() {
                            "done" => Style::default().fg(Color::Green),
                            "closed" => Style::default().fg(Color::Red),
                            "in progress" => Style::default().fg(Color::Yellow),
                            _ => Style::default().fg(Color::Blue),
                        },
                    ),
                    Span::raw(" | Pri:"),
                    Span::raw(format!("{}", task.pri)),
                ]))
            })
            .collect();

        let list = List::new(items)
            .block(Block::default().borders(Borders::ALL))
            .highlight_style(Style::default().bg(Color::Blue).fg(Color::White));

        f.render_stateful_widget(list, chunks[1], &mut app.list_state.borrow_mut());

        let footer = Paragraph::new(Text::from(vec![Line::from(vec![
            Span::styled("↑↓", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw("/"),
            Span::styled("jk", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" nav  "),
            Span::styled("Enter", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" view  "),
            Span::styled("q", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw("/"),
            Span::styled("Esc", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" back  |  "),
            Span::raw(format!("Selected: {} / {}", selected + 1, tasks.len())),
        ])]));
        f.render_widget(footer, chunks[2]);
    }

    fn render_task_detail(f: &mut ratatui::Frame, area: Rect, task: &Task) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Length(10),
                Constraint::Min(0),
                Constraint::Length(3),
            ])
            .split(area);

        let title = Paragraph::new(Text::from(vec![Line::from(vec![
            Span::raw(format!("Task #{} - ", task.id)),
            Span::styled(&task.name, Style::default().add_modifier(Modifier::BOLD)),
        ])]))
        .block(Block::default().borders(Borders::ALL).title("Task Detail"));

        f.render_widget(title, chunks[0]);

        let details = Paragraph::new(Text::from(vec![
            Line::from(vec![Span::raw("Status: "), Span::raw(&task.status)]),
            Line::from(vec![
                Span::raw("Priority: "),
                Span::raw(format!("{}", task.pri)),
            ]),
            Line::from(vec![
                Span::raw("Project: "),
                Span::raw(format!("{}", task.project)),
            ]),
            Line::from(vec![
                Span::raw("Assigned: "),
                Span::raw(task.assigned_to.as_deref().unwrap_or("Unassigned")),
            ]),
            Line::from(vec![
                Span::raw("Estimate: "),
                Span::raw(format!(
                    "{}",
                    task.estimate
                        .map(|e| format!("{}h", e))
                        .unwrap_or_else(|| "N/A".to_string())
                )),
            ]),
            Line::from(vec![
                Span::raw("Consumed: "),
                Span::raw(format!(
                    "{}",
                    task.consumed
                        .map(|c| format!("{}h", c))
                        .unwrap_or_else(|| "N/A".to_string())
                )),
            ]),
            Line::from(vec![
                Span::raw("Left: "),
                Span::raw(format!(
                    "{}",
                    task.left
                        .map(|l| format!("{}h", l))
                        .unwrap_or_else(|| "N/A".to_string())
                )),
            ]),
        ]))
        .block(Block::default().borders(Borders::ALL).title("Details"));

        f.render_widget(details, chunks[1]);

        let footer = Paragraph::new(Text::from(vec![Line::from(vec![
            Span::styled("q", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" back  "),
            Span::styled("o", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" open"),
        ])]));

        f.render_widget(footer, chunks[3]);
    }

    // ============================================================
    // Testcase Render Functions
    // ============================================================

    fn render_testcase_list(
        f: &mut ratatui::Frame,
        area: Rect,
        testcases: &[Testcase],
        selected: usize,
        app: &App,
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
                "Testcase List",
                Style::default().add_modifier(Modifier::BOLD),
            ),
            Span::raw(" ("),
            Span::raw(format!("{}", testcases.len())),
            Span::raw(" items)"),
        ])]))
        .block(Block::default().borders(Borders::ALL).title("ZenTao"));

        f.render_widget(header, chunks[0]);

        let items: Vec<ListItem> = testcases
            .iter()
            .map(|tc| {
                ListItem::new(Line::from(vec![
                    Span::raw(format!("{:6}", tc.id)),
                    Span::raw(" "),
                    Span::styled(&tc.title, Style::default()),
                    Span::raw(" | "),
                    Span::styled(
                        format!("[{}]", tc.status),
                        match tc.status.as_str() {
                            "normal" => Style::default().fg(Color::Green),
                            "blocked" => Style::default().fg(Color::Red),
                            "bypass" => Style::default().fg(Color::Yellow),
                            _ => Style::default().fg(Color::Blue),
                        },
                    ),
                    Span::raw(" | "),
                    Span::styled(
                        format!(
                            "Sev:{}",
                            tc.severity
                                .map(|s| s.to_string())
                                .unwrap_or_else(|| "N/A".to_string())
                        ),
                        match tc.severity {
                            Some(1) => Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
                            Some(2) => Style::default().fg(Color::Yellow),
                            _ => Style::default().fg(Color::DarkGray),
                        },
                    ),
                ]))
            })
            .collect();

        let list = List::new(items)
            .block(Block::default().borders(Borders::ALL))
            .highlight_style(Style::default().bg(Color::Blue).fg(Color::White));

        f.render_stateful_widget(list, chunks[1], &mut app.list_state.borrow_mut());

        let footer = Paragraph::new(Text::from(vec![Line::from(vec![
            Span::styled("↑↓", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw("/"),
            Span::styled("jk", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" nav  "),
            Span::styled("Enter", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" view  "),
            Span::styled("q", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw("/"),
            Span::styled("Esc", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" back  |  "),
            Span::raw(format!("Selected: {} / {}", selected + 1, testcases.len())),
        ])]));
        f.render_widget(footer, chunks[2]);
    }

    fn render_testcase_detail(f: &mut ratatui::Frame, area: Rect, testcase: &Testcase) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Length(10),
                Constraint::Min(0),
                Constraint::Length(3),
            ])
            .split(area);

        let title = Paragraph::new(Text::from(vec![Line::from(vec![
            Span::raw(format!("Testcase #{} - ", testcase.id)),
            Span::styled(
                &testcase.title,
                Style::default().add_modifier(Modifier::BOLD),
            ),
        ])]))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Testcase Detail"),
        );

        f.render_widget(title, chunks[0]);

        let details = Paragraph::new(Text::from(vec![
            Line::from(vec![Span::raw("Status: "), Span::raw(&testcase.status)]),
            Line::from(vec![
                Span::raw("Type: "),
                Span::raw(testcase.type_.as_deref().unwrap_or("N/A")),
            ]),
            Line::from(vec![
                Span::raw("Severity: "),
                Span::raw(
                    testcase
                        .severity
                        .map(|s| s.to_string())
                        .unwrap_or_else(|| "N/A".to_string()),
                ),
            ]),
            Line::from(vec![
                Span::raw("Priority: "),
                Span::raw(
                    testcase
                        .pri
                        .map(|p| p.to_string())
                        .unwrap_or_else(|| "N/A".to_string()),
                ),
            ]),
            Line::from(vec![
                Span::raw("Product: "),
                Span::raw(format!("{}", testcase.product)),
            ]),
            Line::from(vec![
                Span::raw("Version: "),
                Span::raw(format!(
                    "{}",
                    testcase
                        .version
                        .map(|v| v.to_string())
                        .unwrap_or_else(|| "N/A".to_string())
                )),
            ]),
        ]))
        .block(Block::default().borders(Borders::ALL).title("Details"));

        f.render_widget(details, chunks[1]);

        // Strip HTML tags from steps
        let steps_text = testcase.steps.as_deref().unwrap_or("No steps provided.");
        let steps_text = strip_html_tags(steps_text);
        let steps_lines: Vec<Line> = steps_text
            .split('\n')
            .map(|s| Line::from(Span::raw(s)))
            .collect();
        let steps = Paragraph::new(Text::from(steps_lines))
            .block(Block::default().borders(Borders::ALL).title("Steps"));

        f.render_widget(steps, chunks[2]);

        let footer = Paragraph::new(Text::from(vec![Line::from(vec![
            Span::styled("q", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" back  "),
            Span::styled("o", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" open"),
        ])]));

        f.render_widget(footer, chunks[3]);
    }

    // ============================================================
    // Testtask Render Functions
    // ============================================================

    fn render_testtask_list(
        f: &mut ratatui::Frame,
        area: Rect,
        testtasks: &[Testtask],
        selected: usize,
        app: &App,
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
                "Testtask List",
                Style::default().add_modifier(Modifier::BOLD),
            ),
            Span::raw(" ("),
            Span::raw(format!("{}", testtasks.len())),
            Span::raw(" items)"),
        ])]))
        .block(Block::default().borders(Borders::ALL).title("ZenTao"));

        f.render_widget(header, chunks[0]);

        let items: Vec<ListItem> = testtasks
            .iter()
            .map(|tt| {
                ListItem::new(Line::from(vec![
                    Span::raw(format!("{:6}", tt.id)),
                    Span::raw(" "),
                    Span::styled(&tt.name, Style::default()),
                    Span::raw(" | "),
                    Span::styled(
                        format!("[{}]", tt.status),
                        match tt.status.as_str() {
                            "done" => Style::default().fg(Color::Green),
                            "closed" => Style::default().fg(Color::Red),
                            "doing" => Style::default().fg(Color::Yellow),
                            _ => Style::default().fg(Color::Blue),
                        },
                    ),
                    Span::raw(" | "),
                    Span::raw(format!(
                        "Cases: {}",
                        tt.case_count.as_deref().unwrap_or("0")
                    )),
                ]))
            })
            .collect();

        let list = List::new(items)
            .block(Block::default().borders(Borders::ALL))
            .highlight_style(Style::default().bg(Color::Blue).fg(Color::White));

        f.render_stateful_widget(list, chunks[1], &mut app.list_state.borrow_mut());

        let footer = Paragraph::new(Text::from(vec![Line::from(vec![
            Span::styled("↑↓", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw("/"),
            Span::styled("jk", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" nav  "),
            Span::styled("Enter", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" view  "),
            Span::styled("q", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw("/"),
            Span::styled("Esc", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" back  |  "),
            Span::raw(format!("Selected: {} / {}", selected + 1, testtasks.len())),
        ])]));
        f.render_widget(footer, chunks[2]);
    }

    fn render_testtask_detail(f: &mut ratatui::Frame, area: Rect, testtask: &Testtask) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Length(12),
                Constraint::Min(0),
                Constraint::Length(3),
            ])
            .split(area);

        let title = Paragraph::new(Text::from(vec![Line::from(vec![
            Span::raw(format!("Testtask #{} - ", testtask.id)),
            Span::styled(
                &testtask.name,
                Style::default().add_modifier(Modifier::BOLD),
            ),
        ])]))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Testtask Detail"),
        );

        f.render_widget(title, chunks[0]);

        let details = Paragraph::new(Text::from(vec![
            Line::from(vec![Span::raw("Status: "), Span::raw(&testtask.status)]),
            Line::from(vec![
                Span::raw("Type: "),
                Span::raw(testtask.type_.as_deref().unwrap_or("N/A")),
            ]),
            Line::from(vec![
                Span::raw("Project: "),
                Span::raw(format!("{}", testtask.project)),
            ]),
            Line::from(vec![
                Span::raw("Product: "),
                Span::raw(format!(
                    "{}",
                    testtask
                        .product
                        .map(|p| p.to_string())
                        .unwrap_or_else(|| "N/A".to_string())
                )),
            ]),
            Line::from(vec![
                Span::raw("Assigned: "),
                Span::raw(testtask.assigned_to.as_deref().unwrap_or("Unassigned")),
            ]),
            Line::from(vec![
                Span::raw("Begin: "),
                Span::raw(testtask.begin.as_deref().unwrap_or("N/A")),
            ]),
            Line::from(vec![
                Span::raw("End: "),
                Span::raw(testtask.end.as_deref().unwrap_or("N/A")),
            ]),
            Line::from(vec![
                Span::raw("Cases: "),
                Span::raw(testtask.case_count.as_deref().unwrap_or("0")),
            ]),
            Line::from(vec![
                Span::raw("Passed: "),
                Span::raw(testtask.passed_count.as_deref().unwrap_or("0")),
            ]),
            Line::from(vec![
                Span::raw("Failed: "),
                Span::raw(testtask.failed_count.as_deref().unwrap_or("0")),
            ]),
        ]))
        .block(Block::default().borders(Borders::ALL).title("Details"));

        f.render_widget(details, chunks[1]);

        let footer = Paragraph::new(Text::from(vec![Line::from(vec![
            Span::styled("q", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" back  "),
            Span::styled("o", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" open"),
        ])]));

        f.render_widget(footer, chunks[3]);
    }

    // ============================================================
    // Feedback Render Functions
    // ============================================================

    fn render_feedback_list(
        f: &mut ratatui::Frame,
        area: Rect,
        feedbacks: &[Feedback],
        selected: usize,
        app: &App,
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
                "Feedback List",
                Style::default().add_modifier(Modifier::BOLD),
            ),
            Span::raw(" ("),
            Span::raw(format!("{}", feedbacks.len())),
            Span::raw(" items)"),
        ])]))
        .block(Block::default().borders(Borders::ALL).title("ZenTao"));

        f.render_widget(header, chunks[0]);

        let items: Vec<ListItem> = feedbacks
            .iter()
            .map(|fb| {
                ListItem::new(Line::from(vec![
                    Span::raw(format!("{:6}", fb.id)),
                    Span::raw(" "),
                    Span::styled(&fb.title, Style::default()),
                    Span::raw(" | "),
                    Span::styled(
                        format!("[{}]", fb.status),
                        match fb.status.as_str() {
                            "open" => Style::default().fg(Color::Green),
                            "assigned" => Style::default().fg(Color::Yellow),
                            "closed" => Style::default().fg(Color::Red),
                            _ => Style::default().fg(Color::Blue),
                        },
                    ),
                    Span::raw(" | "),
                    Span::raw(fb.type_.as_deref().unwrap_or("-")),
                ]))
            })
            .collect();

        let list = List::new(items)
            .block(Block::default().borders(Borders::ALL))
            .highlight_style(Style::default().bg(Color::Blue).fg(Color::White));

        f.render_stateful_widget(list, chunks[1], &mut app.list_state.borrow_mut());

        let footer = Paragraph::new(Text::from(vec![Line::from(vec![
            Span::styled("↑↓", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw("/"),
            Span::styled("jk", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" nav  "),
            Span::styled("Enter", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" view  "),
            Span::styled("q", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw("/"),
            Span::styled("Esc", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" back  |  "),
            Span::raw(format!("Selected: {} / {}", selected + 1, feedbacks.len())),
        ])]));
        f.render_widget(footer, chunks[2]);
    }

    fn render_feedback_detail(f: &mut ratatui::Frame, area: Rect, feedback: &Feedback) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Length(12),
                Constraint::Min(0),
                Constraint::Length(3),
            ])
            .split(area);

        let title = Paragraph::new(Text::from(vec![Line::from(vec![
            Span::raw(format!("Feedback #{} - ", feedback.id)),
            Span::styled(
                &feedback.title,
                Style::default().add_modifier(Modifier::BOLD),
            ),
        ])]))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Feedback Detail"),
        );

        f.render_widget(title, chunks[0]);

        let details = Paragraph::new(Text::from(vec![
            Line::from(vec![Span::raw("Status: "), Span::raw(&feedback.status)]),
            Line::from(vec![
                Span::raw("Type: "),
                Span::raw(feedback.type_.as_deref().unwrap_or("N/A")),
            ]),
            Line::from(vec![
                Span::raw("Priority: "),
                Span::raw(format!(
                    "{}",
                    feedback
                        .pri
                        .map(|p| p.to_string())
                        .unwrap_or_else(|| "N/A".to_string())
                )),
            ]),
            Line::from(vec![
                Span::raw("Product: "),
                Span::raw(format!(
                    "{}",
                    feedback
                        .product
                        .map(|p| p.to_string())
                        .unwrap_or_else(|| "N/A".to_string())
                )),
            ]),
            Line::from(vec![
                Span::raw("Assigned: "),
                Span::raw(feedback.assigned_to.as_deref().unwrap_or("Unassigned")),
            ]),
            Line::from(vec![
                Span::raw("Opened By: "),
                Span::raw(feedback.opened_by.as_deref().unwrap_or("N/A")),
            ]),
            Line::from(vec![
                Span::raw("Source: "),
                Span::raw(feedback.source.as_deref().unwrap_or("N/A")),
            ]),
            Line::from(vec![
                Span::raw("Contact: "),
                Span::raw(feedback.contact.as_deref().unwrap_or("N/A")),
            ]),
        ]))
        .block(Block::default().borders(Borders::ALL).title("Details"));

        f.render_widget(details, chunks[1]);

        let footer = Paragraph::new(Text::from(vec![Line::from(vec![
            Span::styled("q", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" back  "),
            Span::styled("o", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" open"),
        ])]));

        f.render_widget(footer, chunks[3]);
    }

    // ============================================================
    // Ticket Render Functions
    // ============================================================

    fn render_ticket_list(
        f: &mut ratatui::Frame,
        area: Rect,
        tickets: &[Ticket],
        selected: usize,
        app: &App,
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
            Span::styled("Ticket List", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" ("),
            Span::raw(format!("{}", tickets.len())),
            Span::raw(" items)"),
        ])]))
        .block(Block::default().borders(Borders::ALL).title("ZenTao"));

        f.render_widget(header, chunks[0]);

        let items: Vec<ListItem> = tickets
            .iter()
            .map(|ticket| {
                ListItem::new(Line::from(vec![
                    Span::raw(format!("{:6}", ticket.id)),
                    Span::raw(" "),
                    Span::styled(&ticket.title, Style::default()),
                    Span::raw(" | "),
                    Span::styled(
                        format!("[{}]", ticket.status),
                        match ticket.status.as_str() {
                            "open" => Style::default().fg(Color::Green),
                            "closed" => Style::default().fg(Color::Red),
                            _ => Style::default().fg(Color::Yellow),
                        },
                    ),
                    Span::raw(" | "),
                    Span::raw(ticket.type_.as_deref().unwrap_or("-")),
                ]))
            })
            .collect();

        let list = List::new(items)
            .block(Block::default().borders(Borders::ALL))
            .highlight_style(Style::default().bg(Color::Blue).fg(Color::White));

        f.render_stateful_widget(list, chunks[1], &mut app.list_state.borrow_mut());

        let footer = Paragraph::new(Text::from(vec![Line::from(vec![
            Span::styled("↑↓", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw("/"),
            Span::styled("jk", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" nav  "),
            Span::styled("Enter", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" view  "),
            Span::styled("q", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw("/"),
            Span::styled("Esc", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" back  |  "),
            Span::raw(format!("Selected: {} / {}", selected + 1, tickets.len())),
        ])]));
        f.render_widget(footer, chunks[2]);
    }

    fn render_ticket_detail(f: &mut ratatui::Frame, area: Rect, ticket: &Ticket) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Length(14),
                Constraint::Min(0),
                Constraint::Length(3),
            ])
            .split(area);

        let title = Paragraph::new(Text::from(vec![Line::from(vec![
            Span::raw(format!("Ticket #{} - ", ticket.id)),
            Span::styled(&ticket.title, Style::default().add_modifier(Modifier::BOLD)),
        ])]))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Ticket Detail"),
        );

        f.render_widget(title, chunks[0]);

        let details = Paragraph::new(Text::from(vec![
            Line::from(vec![Span::raw("Status: "), Span::raw(&ticket.status)]),
            Line::from(vec![
                Span::raw("Type: "),
                Span::raw(ticket.type_.as_deref().unwrap_or("N/A")),
            ]),
            Line::from(vec![
                Span::raw("Priority: "),
                Span::raw(format!(
                    "{}",
                    ticket
                        .pri
                        .map(|p| p.to_string())
                        .unwrap_or_else(|| "N/A".to_string())
                )),
            ]),
            Line::from(vec![
                Span::raw("Severity: "),
                Span::raw(format!(
                    "{}",
                    ticket
                        .severity
                        .map(|s| s.to_string())
                        .unwrap_or_else(|| "N/A".to_string())
                )),
            ]),
            Line::from(vec![
                Span::raw("Product: "),
                Span::raw(format!(
                    "{}",
                    ticket
                        .product
                        .map(|p| p.to_string())
                        .unwrap_or_else(|| "N/A".to_string())
                )),
            ]),
            Line::from(vec![
                Span::raw("Assigned: "),
                Span::raw(ticket.assigned_to.as_deref().unwrap_or("Unassigned")),
            ]),
            Line::from(vec![
                Span::raw("Opened By: "),
                Span::raw(ticket.opened_by.as_deref().unwrap_or("N/A")),
            ]),
            Line::from(vec![
                Span::raw("Resolution: "),
                Span::raw(ticket.resolution.as_deref().unwrap_or("N/A")),
            ]),
            Line::from(vec![
                Span::raw("Ticket Code: "),
                Span::raw(ticket.ticket_code.as_deref().unwrap_or("N/A")),
            ]),
        ]))
        .block(Block::default().borders(Borders::ALL).title("Details"));

        f.render_widget(details, chunks[1]);

        let footer = Paragraph::new(Text::from(vec![Line::from(vec![
            Span::styled("q", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" back  "),
            Span::styled("o", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" open"),
        ])]));

        f.render_widget(footer, chunks[3]);
    }

    // ============================================================
    // Program Render Functions
    // ============================================================

    fn render_program_list(
        f: &mut ratatui::Frame,
        area: Rect,
        programs: &[Program],
        selected: usize,
        app: &App,
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
                "Program List",
                Style::default().add_modifier(Modifier::BOLD),
            ),
            Span::raw(" ("),
            Span::raw(format!("{}", programs.len())),
            Span::raw(" items)"),
        ])]))
        .block(Block::default().borders(Borders::ALL).title("ZenTao"));

        f.render_widget(header, chunks[0]);

        let items: Vec<ListItem> = programs
            .iter()
            .map(|program| {
                ListItem::new(Line::from(vec![
                    Span::raw(format!("{:6}", program.id)),
                    Span::raw(" "),
                    Span::styled(&program.name, Style::default()),
                    Span::raw(" ("),
                    Span::raw(&program.code),
                    Span::raw(") | "),
                    Span::styled(
                        format!("[{}]", program.status),
                        match program.status.as_str() {
                            "doing" => Style::default().fg(Color::Green),
                            "closed" => Style::default().fg(Color::Red),
                            "wait" => Style::default().fg(Color::Yellow),
                            _ => Style::default().fg(Color::Blue),
                        },
                    ),
                ]))
            })
            .collect();

        let list = List::new(items)
            .block(Block::default().borders(Borders::ALL))
            .highlight_style(Style::default().bg(Color::Blue).fg(Color::White));

        f.render_stateful_widget(list, chunks[1], &mut app.list_state.borrow_mut());

        let footer = Paragraph::new(Text::from(vec![Line::from(vec![
            Span::styled("↑↓", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw("/"),
            Span::styled("jk", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" nav  "),
            Span::styled("Enter", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" view  "),
            Span::styled("q", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw("/"),
            Span::styled("Esc", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" back  |  "),
            Span::raw(format!("Selected: {} / {}", selected + 1, programs.len())),
        ])]));
        f.render_widget(footer, chunks[2]);
    }

    fn render_program_detail(f: &mut ratatui::Frame, area: Rect, program: &Program) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Length(12),
                Constraint::Min(0),
                Constraint::Length(3),
            ])
            .split(area);

        let title = Paragraph::new(Text::from(vec![Line::from(vec![
            Span::raw(format!("Program #{} - ", program.id)),
            Span::styled(&program.name, Style::default().add_modifier(Modifier::BOLD)),
        ])]))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Program Detail"),
        );

        f.render_widget(title, chunks[0]);

        let details = Paragraph::new(Text::from(vec![
            Line::from(vec![
                Span::raw("ID: "),
                Span::raw(format!("{}", program.id)),
            ]),
            Line::from(vec![Span::raw("Name: "), Span::raw(&program.name)]),
            Line::from(vec![Span::raw("Code: "), Span::raw(&program.code)]),
            Line::from(vec![Span::raw("Status: "), Span::raw(&program.status)]),
            Line::from(vec![
                Span::raw("Type: "),
                Span::raw(program.type_.as_deref().unwrap_or("N/A")),
            ]),
            Line::from(vec![
                Span::raw("Parent: "),
                Span::raw(format!(
                    "{}",
                    program
                        .parent
                        .map(|p| p.to_string())
                        .unwrap_or_else(|| "N/A".to_string())
                )),
            ]),
            Line::from(vec![
                Span::raw("Manager: "),
                Span::raw(program.manager.as_deref().unwrap_or("N/A")),
            ]),
            Line::from(vec![
                Span::raw("Begin: "),
                Span::raw(program.begin.as_deref().unwrap_or("N/A")),
            ]),
            Line::from(vec![
                Span::raw("End: "),
                Span::raw(program.end.as_deref().unwrap_or("N/A")),
            ]),
        ]))
        .block(Block::default().borders(Borders::ALL).title("Details"));

        f.render_widget(details, chunks[1]);

        let footer = Paragraph::new(Text::from(vec![Line::from(vec![
            Span::styled("q", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" back  "),
            Span::styled("o", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" open"),
        ])]));

        f.render_widget(footer, chunks[3]);
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
                "Press q to quit",
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
        selected: usize,
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

        let settings_para =
            Paragraph::new(Text::from(settings_text)).block(Block::default().borders(Borders::ALL));

        let list = if items.is_empty() {
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
        selected: usize,
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
        selected: usize,
        app: &App,
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

    fn render_main_menu(f: &mut ratatui::Frame, area: Rect, selected: usize, app: &App) {
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
}
