use crate::api::{
    Bug, Department, Feedback, Product, ProductPlan, Program, Project, Release, Story, Task,
    Testcase, Testtask, Ticket, User,
};
use crate::core::config::{Config, MultiAccountConfig};

#[derive(Debug, Clone)]
pub enum AppState {
    Idle,
    Loading {
        message: String,
    },
    BugList {
        bugs: Vec<Bug>,
        product_name: Option<String>,
    },
    BugDetail {
        bug: Bug,
        product_name: Option<String>,
    },
    // Bug CRUD
    BugCreate {
        fields: std::collections::HashMap<String, String>,
        field_order: Vec<String>,
        focused_field: usize,
        error: Option<String>,
    },
    BugUpdate {
        id: u64,
        fields: std::collections::HashMap<String, String>,
        field_order: Vec<String>,
        focused_field: usize,
        error: Option<String>,
    },
    BugDelete {
        id: u64,
        name: String,
        confirm: bool,
    },
    StoryList {
        stories: Vec<Story>,
        product_name: Option<String>,
    },
    StoryDetail {
        story: Story,
        product_name: Option<String>,
    },
    // 执行列表
    ExecutionList {
        executions: Vec<crate::api::Execution>,
        project_name: Option<String>,
    },
    ExecutionDetail {
        execution: crate::api::Execution,
        project_name: Option<String>,
    },
    // 版本列表
    BuildList {
        builds: Vec<crate::api::Build>,
        product_name: Option<String>,
        project_name: Option<String>,
    },
    BuildDetail {
        build: crate::api::Build,
        product_name: Option<String>,
        project_name: Option<String>,
    },
    // 发布列表
    ReleaseList {
        releases: Vec<Release>,
        product_name: Option<String>,
    },
    ReleaseDetail {
        release: Release,
        product_name: Option<String>,
    },
    // 用户列表
    UserList {
        users: Vec<User>,
    },
    UserDetail {
        user: User,
    },
    // 部门列表
    DepartmentList {
        departments: Vec<Department>,
    },
    DepartmentDetail {
        department: Department,
    },
    // 产品列表
    ProductList {
        products: Vec<Product>,
    },
    ProductDetail {
        product: Product,
    },
    // 项目列表
    ProjectList {
        projects: Vec<Project>,
    },
    ProjectDetail {
        project: Project,
    },
    // 任务列表
    TaskList {
        tasks: Vec<Task>,
        project_name: Option<String>,
    },
    TaskDetail {
        task: Task,
        project_name: Option<String>,
    },
    // 测试用例列表
    TestcaseList {
        testcases: Vec<Testcase>,
        product_name: Option<String>,
    },
    TestcaseDetail {
        testcase: Testcase,
        product_name: Option<String>,
    },
    // 测试单列表
    TesttaskList {
        testtasks: Vec<Testtask>,
        project_name: Option<String>,
    },
    TesttaskDetail {
        testtask: Testtask,
        project_name: Option<String>,
    },
    // 反馈列表
    FeedbackList {
        feedbacks: Vec<Feedback>,
    },
    FeedbackDetail {
        feedback: Feedback,
    },
    // 工单列表
    TicketList {
        tickets: Vec<Ticket>,
    },
    TicketDetail {
        ticket: Ticket,
    },
    // 项目集列表
    ProgramList {
        programs: Vec<Program>,
    },
    ProgramDetail {
        program: Program,
    },
    // 产品计划列表
    ProductPlanList {
        plans: Vec<ProductPlan>,
        product_name: Option<String>,
    },
    ProductPlanDetail {
        plan: ProductPlan,
        product_name: Option<String>,
    },
    // Program 表单
    ProgramCreate {
        fields: crate::tui::forms::ProgramFormFields,
    },
    ProgramUpdate {
        id: u64,
        fields: crate::tui::forms::ProgramFormFields,
    },
    ProgramDelete {
        id: u64,
        name: String,
    },
    // ProductPlan 表单
    ProductPlanCreate {
        fields: crate::tui::forms::ProductPlanFormFields,
        product_id: u64,
    },
    ProductPlanUpdate {
        id: u64,
        fields: crate::tui::forms::ProductPlanFormFields,
    },
    ProductPlanDelete {
        id: u64,
        name: String,
    },
    // Release 表单
    ReleaseCreate {
        fields: crate::tui::forms::ReleaseFormFields,
        product_id: Option<u64>,
    },
    ReleaseUpdate {
        id: u64,
        fields: crate::tui::forms::ReleaseFormFields,
    },
    ReleaseDelete {
        id: u64,
        name: String,
    },
    // 表单提交中
    FormSubmitting {
        message: String,
    },
    Error {
        message: String,
    },
    Quit,
    // 设置面板
    Settings {
        multi_config: MultiAccountConfig,
        selected: usize,
        current_account: String,
    },
    // 产品选择下拉
    ProductSelect {
        products: Vec<Product>,
        selected: usize,
        loading: bool,
    },
    // 账户选择
    AccountSelect {
        multi_config: MultiAccountConfig,
        selected: usize,
    },
    // 主菜单
    MainMenu {
        selected: usize,
    },
    // 退出确认
    ConfirmQuit,
    // 模块已选中，等待加载（用于主菜单Enter后通知调用者）
    ModuleSelected {
        module_name: String,
    },
}

impl AppState {
    pub fn is_quitting(&self) -> bool {
        matches!(self, AppState::Quit)
    }
}

pub struct App {
    pub state: AppState,
    pub selected_index: usize,
    pub search_active: bool,
    pub search_query: String,
    pub help_visible: bool,
    pub config: crate::core::Config,
    pub multi_config: MultiAccountConfig,
    pub list_state: std::cell::RefCell<ratatui::widgets::ListState>,
    pub main_menu_state: std::cell::RefCell<ratatui::widgets::ListState>,
    // Saved list state for returning from detail pages
    pub saved_list: Option<SavedList>,
    pub saved_index: usize,
    pub saved_main_index: usize,
}

pub enum SavedList {
    BugList(Vec<Bug>, Option<String>),
    StoryList(Vec<Story>, Option<String>),
    ExecutionList(Vec<crate::api::Execution>, Option<String>),
    BuildList(Vec<crate::api::Build>, Option<String>, Option<String>),
    ReleaseList(Vec<Release>, Option<String>),
    UserList(Vec<User>),
    DepartmentList(Vec<Department>),
    ProductList(Vec<Product>),
    ProjectList(Vec<Project>),
    TaskList(Vec<Task>, Option<String>),
    TestcaseList(Vec<Testcase>, Option<String>),
    TesttaskList(Vec<Testtask>, Option<String>),
    FeedbackList(Vec<Feedback>),
    TicketList(Vec<Ticket>),
    ProgramList(Vec<Program>),
    ProductPlanList(Vec<ProductPlan>, Option<String>),
}

impl App {
    pub fn new(config: crate::core::Config, multi_config: MultiAccountConfig) -> Self {
        Self {
            state: AppState::Idle,
            selected_index: 0,
            search_active: false,
            search_query: String::new(),
            help_visible: false,
            config,
            multi_config,
            list_state: std::cell::RefCell::new(ratatui::widgets::ListState::default()),
            main_menu_state: std::cell::RefCell::new(ratatui::widgets::ListState::default()),
            saved_list: None,
            saved_index: 0,
            saved_main_index: 0,
        }
    }

    pub fn set_loading(&mut self, message: String) {
        self.state = AppState::Loading { message };
        self.selected_index = 0;
    }

    pub fn set_main_menu(&mut self) {
        let index = if self.saved_main_index > 0 {
            self.saved_main_index
        } else {
            0
        };
        self.state = AppState::MainMenu { selected: index };
        self.selected_index = index;
        self.main_menu_state.borrow_mut().select(Some(index));
    }

    pub fn set_module_selected(&mut self, module_name: String) {
        self.state = AppState::ModuleSelected { module_name };
    }

    pub fn get_selected_module(&self) -> Option<String> {
        if matches!(self.state, AppState::MainMenu { .. }) {
            let modules = App::get_main_menu_modules();
            modules.get(self.selected_index).map(|s| s.to_string())
        } else {
            None
        }
    }

    pub fn set_bug_list(&mut self, bugs: Vec<Bug>, product_name: Option<String>) {
        self.state = AppState::BugList { bugs, product_name };
        self.selected_index = 0;
        // Set default selection to first item
        self.list_state.borrow_mut().select(Some(0));
    }

    pub fn set_bug_detail(&mut self, bug: Bug, product_name: Option<String>) {
        self.state = AppState::BugDetail { bug, product_name };
        self.selected_index = 0;
    }

    pub fn set_bug_create(&mut self) {
        let mut fields = std::collections::HashMap::new();
        fields.insert("title".to_string(), "".to_string());
        fields.insert("severity".to_string(), "3".to_string());
        fields.insert("pri".to_string(), "3".to_string());
        fields.insert("type".to_string(), "codeerror".to_string());
        fields.insert("steps".to_string(), "".to_string());
        let field_order = vec![
            "title".to_string(),
            "severity".to_string(),
            "pri".to_string(),
            "type".to_string(),
            "steps".to_string(),
        ];
        self.state = AppState::BugCreate {
            fields,
            field_order,
            focused_field: 0,
            error: None,
        };
    }

    pub fn set_bug_update(&mut self, bug: &Bug) {
        let mut fields = std::collections::HashMap::new();
        fields.insert("title".to_string(), bug.title.clone());
        fields.insert("severity".to_string(), bug.severity.to_string());
        fields.insert("pri".to_string(), bug.pri.to_string());
        fields.insert(
            "type".to_string(),
            bug.type_.as_deref().unwrap_or("codeerror").to_string(),
        );
        fields.insert(
            "steps".to_string(),
            bug.steps.as_deref().unwrap_or("").to_string(),
        );
        let field_order = vec![
            "title".to_string(),
            "severity".to_string(),
            "pri".to_string(),
            "type".to_string(),
            "steps".to_string(),
        ];
        self.state = AppState::BugUpdate {
            id: bug.id,
            fields,
            field_order,
            focused_field: 0,
            error: None,
        };
    }

    pub fn set_bug_delete(&mut self, id: u64, name: String) {
        self.state = AppState::BugDelete {
            id,
            name,
            confirm: false,
        };
    }

    pub fn set_story_list(&mut self, stories: Vec<Story>, product_name: Option<String>) {
        self.state = AppState::StoryList {
            stories,
            product_name,
        };
        self.selected_index = 0;
        self.list_state.borrow_mut().select(Some(0));
    }

    pub fn set_story_detail(&mut self, story: Story, product_name: Option<String>) {
        self.state = AppState::StoryDetail {
            story,
            product_name,
        };
        self.selected_index = 0;
    }

    pub fn set_execution_list(
        &mut self,
        executions: Vec<crate::api::Execution>,
        project_name: Option<String>,
    ) {
        self.state = AppState::ExecutionList {
            executions,
            project_name,
        };
        self.selected_index = 0;
        self.list_state.borrow_mut().select(Some(0));
    }

    pub fn set_execution_detail(
        &mut self,
        execution: crate::api::Execution,
        project_name: Option<String>,
    ) {
        self.state = AppState::ExecutionDetail {
            execution,
            project_name,
        };
        self.selected_index = 0;
    }

    pub fn set_build_list(
        &mut self,
        builds: Vec<crate::api::Build>,
        product_name: Option<String>,
        project_name: Option<String>,
    ) {
        self.state = AppState::BuildList {
            builds,
            product_name,
            project_name,
        };
        self.selected_index = 0;
        self.list_state.borrow_mut().select(Some(0));
    }

    pub fn set_build_detail(
        &mut self,
        build: crate::api::Build,
        product_name: Option<String>,
        project_name: Option<String>,
    ) {
        self.state = AppState::BuildDetail {
            build,
            product_name,
            project_name,
        };
        self.selected_index = 0;
    }

    pub fn set_release_list(&mut self, releases: Vec<Release>, product_name: Option<String>) {
        self.state = AppState::ReleaseList {
            releases,
            product_name,
        };
        self.selected_index = 0;
        self.list_state.borrow_mut().select(Some(0));
    }

    pub fn set_release_detail(&mut self, release: Release, product_name: Option<String>) {
        self.state = AppState::ReleaseDetail {
            release,
            product_name,
        };
        self.selected_index = 0;
    }

    pub fn set_user_list(&mut self, users: Vec<User>) {
        self.state = AppState::UserList { users };
        self.selected_index = 0;
        self.list_state.borrow_mut().select(Some(0));
    }

    pub fn set_user_detail(&mut self, user: User) {
        self.state = AppState::UserDetail { user };
        self.selected_index = 0;
    }

    pub fn set_department_list(&mut self, departments: Vec<Department>) {
        self.state = AppState::DepartmentList { departments };
        self.selected_index = 0;
        self.list_state.borrow_mut().select(Some(0));
    }

    pub fn set_department_detail(&mut self, department: Department) {
        self.state = AppState::DepartmentDetail { department };
        self.selected_index = 0;
    }

    pub fn set_product_list(&mut self, products: Vec<Product>) {
        self.state = AppState::ProductList { products };
        self.selected_index = 0;
        self.list_state.borrow_mut().select(Some(0));
    }

    pub fn set_product_detail(&mut self, product: Product) {
        self.state = AppState::ProductDetail { product };
        self.selected_index = 0;
    }

    pub fn set_project_list(&mut self, projects: Vec<Project>) {
        self.state = AppState::ProjectList { projects };
        self.selected_index = 0;
        self.list_state.borrow_mut().select(Some(0));
    }

    pub fn set_project_detail(&mut self, project: Project) {
        self.state = AppState::ProjectDetail { project };
        self.selected_index = 0;
    }

    pub fn set_task_list(&mut self, tasks: Vec<Task>, project_name: Option<String>) {
        self.state = AppState::TaskList {
            tasks,
            project_name,
        };
        self.selected_index = 0;
        self.list_state.borrow_mut().select(Some(0));
    }

    pub fn set_task_detail(&mut self, task: Task, project_name: Option<String>) {
        self.state = AppState::TaskDetail { task, project_name };
        self.selected_index = 0;
    }

    pub fn set_testcase_list(&mut self, testcases: Vec<Testcase>, product_name: Option<String>) {
        self.state = AppState::TestcaseList {
            testcases,
            product_name,
        };
        self.selected_index = 0;
        self.list_state.borrow_mut().select(Some(0));
    }

    pub fn set_testcase_detail(&mut self, testcase: Testcase, product_name: Option<String>) {
        self.state = AppState::TestcaseDetail {
            testcase,
            product_name,
        };
        self.selected_index = 0;
    }

    pub fn set_testtask_list(&mut self, testtasks: Vec<Testtask>, project_name: Option<String>) {
        self.state = AppState::TesttaskList {
            testtasks,
            project_name,
        };
        self.selected_index = 0;
        self.list_state.borrow_mut().select(Some(0));
    }

    pub fn set_testtask_detail(&mut self, testtask: Testtask, project_name: Option<String>) {
        self.state = AppState::TesttaskDetail {
            testtask,
            project_name,
        };
        self.selected_index = 0;
    }

    pub fn set_feedback_list(&mut self, feedbacks: Vec<Feedback>) {
        self.state = AppState::FeedbackList { feedbacks };
        self.selected_index = 0;
        self.list_state.borrow_mut().select(Some(0));
    }

    pub fn set_feedback_detail(&mut self, feedback: Feedback) {
        self.state = AppState::FeedbackDetail { feedback };
        self.selected_index = 0;
    }

    pub fn set_ticket_list(&mut self, tickets: Vec<Ticket>) {
        self.state = AppState::TicketList { tickets };
        self.selected_index = 0;
        self.list_state.borrow_mut().select(Some(0));
    }

    pub fn set_ticket_detail(&mut self, ticket: Ticket) {
        self.state = AppState::TicketDetail { ticket };
        self.selected_index = 0;
    }

    pub fn set_program_list(&mut self, programs: Vec<Program>) {
        self.state = AppState::ProgramList { programs };
        self.selected_index = 0;
        self.list_state.borrow_mut().select(Some(0));
    }

    pub fn set_program_detail(&mut self, program: Program) {
        self.state = AppState::ProgramDetail { program };
        self.selected_index = 0;
    }

    pub fn set_productplan_list(&mut self, plans: Vec<ProductPlan>, product_name: Option<String>) {
        self.state = AppState::ProductPlanList {
            plans,
            product_name,
        };
        self.selected_index = 0;
        self.list_state.borrow_mut().select(Some(0));
    }

    pub fn set_productplan_detail(&mut self, plan: ProductPlan, product_name: Option<String>) {
        self.state = AppState::ProductPlanDetail { plan, product_name };
        self.selected_index = 0;
    }

    // Program 表单 setter
    pub fn set_program_create(&mut self) {
        self.state = AppState::ProgramCreate {
            fields: crate::tui::forms::ProgramFormFields::new(),
        };
        self.selected_index = 0;
    }

    pub fn set_program_update(&mut self, id: u64, program: &Program) {
        self.state = AppState::ProgramUpdate {
            id,
            fields: crate::tui::forms::ProgramFormFields::from_program(
                &program.name,
                &program.code,
                program.desc.as_deref().unwrap_or(""),
                program.begin.as_deref().unwrap_or(""),
                program.end.as_deref().unwrap_or(""),
            ),
        };
        self.selected_index = 0;
    }

    pub fn set_program_delete(&mut self, id: u64, name: &str) {
        self.state = AppState::ProgramDelete {
            id,
            name: name.to_string(),
        };
    }

    // ProductPlan 表单 setter
    pub fn set_productplan_create(&mut self, product_id: u64) {
        self.state = AppState::ProductPlanCreate {
            fields: crate::tui::forms::ProductPlanFormFields::new(),
            product_id,
        };
        self.selected_index = 0;
    }

    pub fn set_productplan_update(&mut self, id: u64, plan: &ProductPlan) {
        self.state = AppState::ProductPlanUpdate {
            id,
            fields: crate::tui::forms::ProductPlanFormFields::from_plan(
                plan.title.as_deref().unwrap_or(""),
                plan.desc.as_deref().unwrap_or(""),
                plan.begin.as_deref().unwrap_or(""),
                plan.end.as_deref().unwrap_or(""),
            ),
        };
        self.selected_index = 0;
    }

    pub fn set_productplan_delete(&mut self, id: u64, name: &str) {
        self.state = AppState::ProductPlanDelete {
            id,
            name: name.to_string(),
        };
    }

    // Release 表单 setter
    pub fn set_release_create(&mut self, product_id: Option<u64>) {
        self.state = AppState::ReleaseCreate {
            fields: crate::tui::forms::ReleaseFormFields::new(),
            product_id,
        };
        self.selected_index = 0;
    }

    pub fn set_release_update(&mut self, id: u64, release: &Release) {
        self.state = AppState::ReleaseUpdate {
            id,
            fields: crate::tui::forms::ReleaseFormFields::from_release(
                &release.name,
                release
                    .build
                    .map(|b| b.to_string())
                    .as_deref()
                    .unwrap_or(""),
                release.date.as_deref().unwrap_or(""),
                &release.status,
                release.desc.as_deref().unwrap_or(""),
            ),
        };
        self.selected_index = 0;
    }

    pub fn set_release_delete(&mut self, id: u64, name: &str) {
        self.state = AppState::ReleaseDelete {
            id,
            name: name.to_string(),
        };
    }

    pub fn set_form_submitting(&mut self, message: &str) {
        self.state = AppState::FormSubmitting {
            message: message.to_string(),
        };
    }

    pub fn get_main_menu_modules() -> Vec<&'static str> {
        vec![
            "Bug List",
            "Story List",
            "Execution List",
            "Build List",
            "Release List",
            "Product List",
            "Project List",
            "Task List",
            "Testcase List",
            "Testtask List",
            "Feedback List",
            "Ticket List",
            "Program List",
            "ProductPlan List",
            "User List",
            "Department List",
            "Settings",
        ]
    }

    pub fn set_error(&mut self, message: String) {
        self.state = AppState::Error { message };
    }

    pub fn go_back_to_list(&mut self) {
        match &self.state {
            AppState::BugDetail { .. } => {
                self.selected_index = 0;
            }
            AppState::StoryDetail { .. } => {
                self.selected_index = 0;
            }
            AppState::ExecutionDetail { .. } => {
                self.selected_index = 0;
            }
            AppState::BuildDetail { .. } => {
                self.selected_index = 0;
            }
            AppState::ReleaseDetail { .. } => {
                self.selected_index = 0;
            }
            AppState::UserDetail { .. } => {
                self.selected_index = 0;
            }
            AppState::DepartmentDetail { .. } => {
                self.selected_index = 0;
            }
            AppState::ProductDetail { .. } => {
                self.selected_index = 0;
            }
            AppState::ProjectDetail { .. } => {
                self.selected_index = 0;
            }
            AppState::TaskDetail { .. } => {
                self.selected_index = 0;
            }
            AppState::TestcaseDetail { .. } => {
                self.selected_index = 0;
            }
            AppState::TesttaskDetail { .. } => {
                self.selected_index = 0;
            }
            AppState::FeedbackDetail { .. } => {
                self.selected_index = 0;
            }
            AppState::TicketDetail { .. } => {
                self.selected_index = 0;
            }
            AppState::ProgramDetail { .. } => {
                self.selected_index = 0;
            }
            AppState::ProductPlanDetail { .. } => {
                self.selected_index = 0;
            }
            _ => {}
        }
    }

    pub fn quit(&mut self) {
        self.state = AppState::Quit;
    }

    pub fn save_list(&mut self) {
        let saved = match &self.state {
            AppState::BugList { bugs, product_name } => {
                Some(SavedList::BugList(bugs.clone(), product_name.clone()))
            }
            AppState::StoryList {
                stories,
                product_name,
            } => Some(SavedList::StoryList(stories.clone(), product_name.clone())),
            AppState::ExecutionList {
                executions,
                project_name,
            } => Some(SavedList::ExecutionList(
                executions.clone(),
                project_name.clone(),
            )),
            AppState::BuildList {
                builds,
                product_name,
                project_name,
            } => Some(SavedList::BuildList(
                builds.clone(),
                product_name.clone(),
                project_name.clone(),
            )),
            AppState::ReleaseList {
                releases,
                product_name,
            } => Some(SavedList::ReleaseList(
                releases.clone(),
                product_name.clone(),
            )),
            AppState::UserList { users } => Some(SavedList::UserList(users.clone())),
            AppState::DepartmentList { departments } => {
                Some(SavedList::DepartmentList(departments.clone()))
            }
            AppState::ProductList { products } => Some(SavedList::ProductList(products.clone())),
            AppState::ProjectList { projects } => Some(SavedList::ProjectList(projects.clone())),
            AppState::TaskList {
                tasks,
                project_name,
            } => Some(SavedList::TaskList(tasks.clone(), project_name.clone())),
            AppState::TestcaseList {
                testcases,
                product_name,
            } => Some(SavedList::TestcaseList(
                testcases.clone(),
                product_name.clone(),
            )),
            AppState::TesttaskList {
                testtasks,
                project_name,
            } => Some(SavedList::TesttaskList(
                testtasks.clone(),
                project_name.clone(),
            )),
            AppState::FeedbackList { feedbacks } => {
                Some(SavedList::FeedbackList(feedbacks.clone()))
            }
            AppState::TicketList { tickets } => Some(SavedList::TicketList(tickets.clone())),
            AppState::ProgramList { programs } => Some(SavedList::ProgramList(programs.clone())),
            AppState::ProductPlanList {
                plans,
                product_name,
            } => Some(SavedList::ProductPlanList(
                plans.clone(),
                product_name.clone(),
            )),
            _ => None,
        };
        self.saved_list = saved;
        self.saved_index = self.selected_index;
    }

    pub fn restore_list(&mut self) {
        if let Some(saved) = self.saved_list.take() {
            match saved {
                SavedList::BugList(bugs, product_name) => {
                    self.set_bug_list(bugs, product_name);
                }
                SavedList::StoryList(stories, product_name) => {
                    self.set_story_list(stories, product_name);
                }
                SavedList::ExecutionList(executions, project_name) => {
                    self.set_execution_list(executions, project_name);
                }
                SavedList::BuildList(builds, product_name, project_name) => {
                    self.set_build_list(builds, product_name, project_name);
                }
                SavedList::ReleaseList(releases, product_name) => {
                    self.set_release_list(releases, product_name);
                }
                SavedList::UserList(users) => {
                    self.set_user_list(users);
                }
                SavedList::DepartmentList(departments) => {
                    self.set_department_list(departments);
                }
                SavedList::ProductList(products) => {
                    self.set_product_list(products);
                }
                SavedList::ProjectList(projects) => {
                    self.set_project_list(projects);
                }
                SavedList::TaskList(tasks, project_name) => {
                    self.set_task_list(tasks, project_name);
                }
                SavedList::TestcaseList(testcases, product_name) => {
                    self.set_testcase_list(testcases, product_name);
                }
                SavedList::TesttaskList(testtasks, project_name) => {
                    self.set_testtask_list(testtasks, project_name);
                }
                SavedList::FeedbackList(feedbacks) => {
                    self.set_feedback_list(feedbacks);
                }
                SavedList::TicketList(tickets) => {
                    self.set_ticket_list(tickets);
                }
                SavedList::ProgramList(programs) => {
                    self.set_program_list(programs);
                }
                SavedList::ProductPlanList(plans, product_name) => {
                    self.set_productplan_list(plans, product_name);
                }
            }
            // Restore the saved index
            self.selected_index = self.saved_index;
            self.list_state.borrow_mut().select(Some(self.saved_index));
        } else {
            self.set_main_menu();
        }
    }

    pub fn get_selected_bug_id(&self) -> Option<u64> {
        if let AppState::BugList { bugs, .. } = &self.state {
            bugs.get(self.selected_index).map(|b| b.id)
        } else {
            None
        }
    }

    pub fn get_selected_story_id(&self) -> Option<u64> {
        if let AppState::StoryList { stories, .. } = &self.state {
            stories.get(self.selected_index).map(|s| s.id)
        } else {
            None
        }
    }

    pub fn get_selected_user_id(&self) -> Option<u64> {
        if let AppState::UserList { users } = &self.state {
            users.get(self.selected_index).map(|u| u.id)
        } else {
            None
        }
    }

    pub fn get_selected_department_id(&self) -> Option<u64> {
        if let AppState::DepartmentList { departments } = &self.state {
            departments.get(self.selected_index).map(|d| d.id)
        } else {
            None
        }
    }

    pub fn get_selected_product_id(&self) -> Option<u64> {
        if let AppState::ProductList { products } = &self.state {
            products.get(self.selected_index).map(|p| p.id)
        } else {
            None
        }
    }

    pub fn get_selected_project_id(&self) -> Option<u64> {
        if let AppState::ProjectList { projects } = &self.state {
            projects.get(self.selected_index).map(|p| p.id)
        } else {
            None
        }
    }

    pub fn get_selected_task_id(&self) -> Option<u64> {
        if let AppState::TaskList { tasks, .. } = &self.state {
            tasks.get(self.selected_index).map(|t| t.id)
        } else {
            None
        }
    }

    pub fn get_selected_testcase_id(&self) -> Option<u64> {
        if let AppState::TestcaseList { testcases, .. } = &self.state {
            testcases.get(self.selected_index).map(|t| t.id)
        } else {
            None
        }
    }

    pub fn get_selected_testtask_id(&self) -> Option<u64> {
        if let AppState::TesttaskList { testtasks, .. } = &self.state {
            testtasks.get(self.selected_index).map(|t| t.id)
        } else {
            None
        }
    }

    pub fn get_selected_feedback_id(&self) -> Option<u64> {
        if let AppState::FeedbackList { feedbacks } = &self.state {
            feedbacks.get(self.selected_index).map(|f| f.id)
        } else {
            None
        }
    }

    pub fn get_selected_ticket_id(&self) -> Option<u64> {
        if let AppState::TicketList { tickets } = &self.state {
            tickets.get(self.selected_index).map(|t| t.id)
        } else {
            None
        }
    }

    pub fn get_selected_program_id(&self) -> Option<u64> {
        if let AppState::ProgramList { programs } = &self.state {
            programs.get(self.selected_index).map(|p| p.id)
        } else {
            None
        }
    }

    pub fn get_selected_productplan_id(&self) -> Option<u64> {
        if let AppState::ProductPlanList { plans, .. } = &self.state {
            plans.get(self.selected_index).map(|p| p.id)
        } else {
            None
        }
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new(Config::default(), MultiAccountConfig::default())
    }
}
