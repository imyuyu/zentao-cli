#![allow(snake_case)]

pub mod api;
pub mod cmd;
pub mod core;
pub mod service;
pub mod tui;

pub use anyhow::Result;
pub use api::types::{Bug, Story};
pub use api::{ApiClient, ApiResponse, Auth};
pub use core::output::{safe_println, PaginationMeta};
pub use core::Config;

pub fn run() -> Result<()> {
    cmd::root::run()
}
