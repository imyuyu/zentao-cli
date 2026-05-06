use crate::api::execution::{CreateExecutionRequest, UpdateExecutionRequest};
use crate::api::{Execution, ExecutionApi};
use crate::core::logging::{log, LogLevel};
use crate::core::AppContext;
use anyhow::Result;

pub struct ExecutionService;

impl ExecutionService {
    pub async fn list(ctx: &AppContext, project: Option<u64>) -> Result<Vec<Execution>> {
        log(LogLevel::Info, "ExecutionService", "list");
        let client = ctx.client();
        ExecutionApi::list(&client, ctx.project_id(project)).await
    }

    pub async fn get(ctx: &AppContext, id: u64) -> Result<Execution> {
        log(LogLevel::Info, "ExecutionService", format!("get id={}", id));
        let client = ctx.client();
        ExecutionApi::get(&client, id).await
    }

    pub async fn create(
        ctx: &AppContext,
        project: Option<u64>,
        req: CreateExecutionRequest,
    ) -> Result<Execution> {
        log(LogLevel::Info, "ExecutionService", "create");
        let client = ctx.client();
        ExecutionApi::create(&client, ctx.require_project_id(project)?, &req).await
    }

    pub async fn update(
        ctx: &AppContext,
        id: u64,
        req: UpdateExecutionRequest,
    ) -> Result<Execution> {
        log(
            LogLevel::Info,
            "ExecutionService",
            format!("update id={}", id),
        );
        let client = ctx.client();
        ExecutionApi::update(&client, id, &req).await
    }

    pub async fn delete(ctx: &AppContext, id: u64) -> Result<()> {
        log(
            LogLevel::Info,
            "ExecutionService",
            format!("delete id={}", id),
        );
        let client = ctx.client();
        ExecutionApi::delete(&client, id).await
    }
}
