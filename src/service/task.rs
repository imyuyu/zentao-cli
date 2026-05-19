use crate::api::{
    BatchEstimateRequest, CloseTaskRequest, CreateTaskRequest, FinishTaskRequest, PauseTaskRequest,
    RestartTaskRequest, StartTaskRequest, Task, TaskApi, TaskEstimate, UpdateTaskRequest,
};
use crate::core::logging::{log, LogLevel};
use crate::core::AppContext;
use anyhow::Result;

pub struct TaskService;

impl TaskService {
    pub async fn list(
        ctx: &AppContext,
        project: Option<u64>,
        assigned_to: Option<String>,
    ) -> Result<Vec<Task>> {
        log(LogLevel::Info, "TaskService", "list");
        let client = ctx.client();
        TaskApi::list(&client, ctx.require_project_id(project)?, assigned_to).await
    }

    pub async fn get(ctx: &AppContext, id: u64) -> Result<Task> {
        log(LogLevel::Info, "TaskService", format!("get id={}", id));
        let client = ctx.client();
        TaskApi::get(&client, id).await
    }

    pub async fn create(ctx: &AppContext, req: CreateTaskRequest) -> Result<Task> {
        log(LogLevel::Info, "TaskService", "create");
        let client = ctx.client();
        TaskApi::create(&client, &req).await
    }

    pub async fn update(ctx: &AppContext, id: u64, req: UpdateTaskRequest) -> Result<Task> {
        log(LogLevel::Info, "TaskService", format!("update id={}", id));
        let client = ctx.client();
        TaskApi::update(&client, id, &req).await
    }

    pub async fn delete(ctx: &AppContext, id: u64) -> Result<()> {
        log(LogLevel::Info, "TaskService", format!("delete id={}", id));
        let client = ctx.client();
        TaskApi::delete(&client, id).await
    }

    pub async fn start(ctx: &AppContext, id: u64, req: StartTaskRequest) -> Result<Task> {
        let client = ctx.client();
        TaskApi::start(&client, id, &req).await
    }

    pub async fn pause(ctx: &AppContext, id: u64, req: PauseTaskRequest) -> Result<Task> {
        let client = ctx.client();
        TaskApi::pause(&client, id, &req).await
    }

    pub async fn restart(ctx: &AppContext, id: u64, req: RestartTaskRequest) -> Result<Task> {
        let client = ctx.client();
        TaskApi::restart(&client, id, &req).await
    }

    pub async fn finish(ctx: &AppContext, id: u64, req: FinishTaskRequest) -> Result<Task> {
        let client = ctx.client();
        TaskApi::finish(&client, id, &req).await
    }

    pub async fn close(ctx: &AppContext, id: u64, req: CloseTaskRequest) -> Result<Task> {
        let client = ctx.client();
        TaskApi::close(&client, id, &req).await
    }

    pub async fn add_estimate(
        ctx: &AppContext,
        id: u64,
        dates: Vec<String>,
        work: Vec<f64>,
        consumed: Vec<f64>,
        left: Vec<f64>,
    ) -> Result<TaskEstimate> {
        let client = ctx.client();
        let req = BatchEstimateRequest {
            dates,
            work,
            consumed,
            left,
        };
        TaskApi::add_estimate(&client, id, &req).await
    }

    pub async fn get_estimates(ctx: &AppContext, id: u64) -> Result<Vec<TaskEstimate>> {
        let client = ctx.client();
        TaskApi::get_estimates(&client, id).await
    }
}
