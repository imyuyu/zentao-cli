//! ZenTao API 模块
//!
//! 封装所有与 ZenTao 服务器的 HTTP 通信
//!
//! # 模块结构
//! - `auth`: 认证（登录、Token 验证）
//! - `client`: HTTP 客户端（GET/POST/PUT 请求）
//! - `types`: 数据类型定义（Story、Bug、Product 等）
//! - `story`: 需求 API
//! - `bug`: Bug API
//! - `product`: 产品 API
//! - `project`: 项目 API
//! - `task`: 任务 API
//! - `testcase`: 测试用例 API
//! - `testtask`: 测试单 API
//! - `feedback`: 反馈 API
//! - `ticket`: 工单 API

pub mod auth;
pub mod auth_client;
pub mod bug;
pub mod build;
pub mod client;
pub mod department;
pub mod doc;
pub mod execution;
pub mod feedback;
pub mod product;
pub mod productplan;
pub mod program;
pub mod project;
pub mod release;
pub mod story;
pub mod task;
pub mod testcase;
pub mod testtask;
pub mod ticket;
pub mod types;
pub mod user;

// 导出公共 API
pub use auth::Auth;
pub use auth_client::AuthClient;
pub use bug::{BugApi, CreateBugRequest, UpdateBugRequest};
pub use build::{BuildApi, CreateBuildRequest, UpdateBuildRequest};
pub use client::ApiClient;
pub use department::DepartmentApi;
pub use doc::{Doc, DocApi};
pub use execution::ExecutionApi;
pub use feedback::{Feedback, FeedbackApi};
pub use product::{CreateProductRequest, Product, ProductApi, UpdateProductRequest};
pub use productplan::{ProductPlan, ProductPlanApi};
pub use program::{Program, ProgramApi};
pub use project::{CreateProjectRequest, Project, ProjectApi, UpdateProjectRequest};
pub use release::{Release, ReleaseApi};
pub use story::{CreateStoryRequest, StoryApi, UpdateStoryRequest};
pub use task::{CreateTaskRequest, Task, TaskApi, TaskEstimate, UpdateTaskRequest};
pub use testcase::TestcaseApi;
pub use testtask::{Testtask, TesttaskApi};
pub use ticket::{Ticket, TicketApi};
pub use types::*;
pub use types::{Bug, Department, Story, Testcase, TestcaseListQuery, User};
pub use user::UserApi;
