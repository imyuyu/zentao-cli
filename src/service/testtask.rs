use crate::api::{Testtask, TesttaskApi};
use crate::core::logging::{log, LogLevel};
use crate::core::AppContext;
use anyhow::Result;

pub struct TesttaskService;

impl TesttaskService {
    pub async fn list(ctx: &AppContext, page: u32, limit: u32) -> Result<Vec<Testtask>> {
        log(LogLevel::Info, "TesttaskService", "list");
        let client = ctx.client();
        TesttaskApi::list(&client, page, limit).await
    }

    pub async fn list_by_project(
        ctx: &AppContext,
        project: u64,
        page: u32,
        limit: u32,
    ) -> Result<Vec<Testtask>> {
        log(LogLevel::Info, "TesttaskService", "list_by_project");
        let client = ctx.client();
        TesttaskApi::list_by_project(&client, project, page, limit).await
    }

    pub async fn get(ctx: &AppContext, id: u64) -> Result<Testtask> {
        log(LogLevel::Info, "TesttaskService", format!("get id={}", id));
        let client = ctx.client();
        TesttaskApi::get(&client, id).await
    }
}
