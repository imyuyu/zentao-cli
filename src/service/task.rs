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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{Config, OutputFormat};
    use httpmock::prelude::*;

    const TASK_JSON: &str =
        r#"{"id":10,"name":"Test Task","project":1,"status":"in progress","pri":3}"#;

    fn setup(project_id: Option<u64>) -> (MockServer, AppContext) {
        let server = MockServer::start();
        let config = Config {
            url: server.base_url(),
            token: None,
            product_id: None,
            project_id,
            api_version: Some("v1".into()),
            account: None,
        };
        (server, AppContext::new(config, OutputFormat::Json, false))
    }

    #[tokio::test]
    async fn list_fails_when_project_not_set() {
        let (_server, ctx) = setup(None);
        let result = TaskService::list(&ctx, None, None).await;
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("project ID is required"));
    }

    #[tokio::test]
    async fn start_posts_to_start_then_gets_task() {
        let (server, ctx) = setup(None);

        let start_mock = server.mock(|when, then| {
            when.method(POST).path("/api.php/v1/tasks/5/start");
            then.status(200).json_body(serde_json::json!({}));
        });
        let get_mock = server.mock(|when, then| {
            when.method(GET).path("/api.php/v1/tasks/5");
            then.status(200)
                .json_body(serde_json::from_str::<serde_json::Value>(TASK_JSON).unwrap());
        });

        let req = StartTaskRequest {
            left: 4.0,
            consumed: Some(2.0),
            assigned_to: None,
            real_started: None,
            comment: None,
        };
        let result = TaskService::start(&ctx, 5, req).await.unwrap();
        assert_eq!(result.id, 10);
        assert_eq!(result.name, "Test Task");
        start_mock.assert();
        get_mock.assert();
    }

    #[tokio::test]
    async fn add_estimate_assembles_request_and_posts() {
        let (server, ctx) = setup(None);

        let mock = server.mock(|when, then| {
            when.method(POST)
                .path("/api.php/v1/tasks/10/estimate")
                .body_contains(r#""dates":["2024-01-01"]"#);
            then.status(200).json_body(
                serde_json::json!({"effort":{"id":42,"task":10,"consumed":1.0,"left":1.5,"date":"2024-01-01"}}),
            );
        });

        let result = TaskService::add_estimate(
            &ctx,
            10,
            vec!["2024-01-01".into()],
            vec![2.5],
            vec![1.0],
            vec![1.5],
        )
        .await
        .unwrap();

        assert_eq!(result.id, 42);
        assert_eq!(result.consumed, 1.0);
        assert_eq!(result.left, 1.5);
        mock.assert();
    }
}
