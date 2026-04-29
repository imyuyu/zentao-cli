pub mod cmd;
pub mod core;
pub mod api;
pub mod shortcuts;
pub mod tui;

pub use anyhow::Result;
pub use core::Config;
pub use api::{ApiClient, ApiResponse, Auth};
pub use api::types::{Story, Bug};
pub use core::output::PaginationMeta;

pub fn run() -> Result<()> {
    cmd::root::run()
}
