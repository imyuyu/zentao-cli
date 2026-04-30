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

pub mod auth;
pub mod bug;
pub mod build;
pub mod client;
pub mod doc;
pub mod execution;
pub mod product;
pub mod project;
pub mod release;
pub mod story;
pub mod task;
pub mod testcase;
pub mod types;
pub mod user;

// 导出公共 API
pub use auth::Auth;
pub use bug::{BugApi, CreateBugRequest, UpdateBugRequest};
pub use build::{BuildApi, CreateBuildRequest, UpdateBuildRequest};
pub use client::ApiClient;
pub use doc::{Doc, DocApi};
pub use execution::ExecutionApi;
pub use product::{CreateProductRequest, Product, ProductApi, UpdateProductRequest};
pub use project::{CreateProjectRequest, Project, ProjectApi, UpdateProjectRequest};
pub use release::{Release, ReleaseApi};
pub use story::{CreateStoryRequest, StoryApi, UpdateStoryRequest};
pub use task::{CreateTaskRequest, Task, TaskApi, TaskEstimate, UpdateTaskRequest};
pub use testcase::TestcaseApi;
pub use types::*;
pub use types::{Bug, Story, Testcase, TestcaseListQuery, User};
pub use user::UserApi;
