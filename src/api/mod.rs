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

pub mod types;
pub mod auth;
pub mod client;
pub mod story;
pub mod bug;
pub mod product;
pub mod project;
pub mod task;
pub mod user;
pub mod release;
pub mod testcase;
pub mod doc;
pub mod execution;
pub mod build;

// 导出公共 API
pub use types::*;
pub use auth::Auth;
pub use client::ApiClient;
pub use types::{Story, Bug, User, Testcase, TestcaseListQuery};
pub use story::{StoryApi, CreateStoryRequest, UpdateStoryRequest};
pub use bug::{BugApi, CreateBugRequest, UpdateBugRequest};
pub use product::{Product, ProductApi};
pub use project::{Project, ProjectApi};
pub use task::{Task, TaskApi, CreateTaskRequest, UpdateTaskRequest};
pub use user::UserApi;
pub use release::{Release, ReleaseApi};
pub use testcase::TestcaseApi;
pub use doc::{Doc, DocApi};
pub use execution::ExecutionApi;
pub use build::BuildApi;
