use crate::api::{Build, BuildApi, CreateBuildRequest, UpdateBuildRequest};
use crate::core::logging::{log, LogLevel};
use crate::core::AppContext;
use anyhow::Result;

pub struct BuildService;

impl BuildService {
    pub async fn list(
        ctx: &AppContext,
        project: Option<u64>,
        product: Option<u64>,
        execution: Option<u64>,
    ) -> Result<Vec<Build>> {
        log(LogLevel::Info, "BuildService", "list");
        let client = ctx.client();
        if let Some(eid) = execution {
            BuildApi::list_by_execution(&client, eid).await
        } else {
            BuildApi::list(&client, ctx.project_id(project), ctx.product_id(product)).await
        }
    }

    pub async fn get(ctx: &AppContext, id: u64) -> Result<Build> {
        log(LogLevel::Info, "BuildService", format!("get id={}", id));
        let client = ctx.client();
        BuildApi::get(&client, id).await
    }

    pub async fn create(
        ctx: &AppContext,
        project: Option<u64>,
        req: CreateBuildRequest,
    ) -> Result<Build> {
        log(LogLevel::Info, "BuildService", "create");
        let client = ctx.client();
        BuildApi::create(&client, ctx.require_project_id(project)?, &req).await
    }

    pub async fn update(ctx: &AppContext, id: u64, req: UpdateBuildRequest) -> Result<Build> {
        log(LogLevel::Info, "BuildService", format!("update id={}", id));
        let client = ctx.client();
        BuildApi::update(&client, id, &req).await
    }

    pub async fn delete(ctx: &AppContext, id: u64) -> Result<()> {
        log(LogLevel::Info, "BuildService", format!("delete id={}", id));
        let client = ctx.client();
        BuildApi::delete(&client, id).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{Config, OutputFormat};
    use httpmock::prelude::*;

    const BUILD_JSON: &str =
        r#"{"id":1,"name":"build-1","product":1,"project":1,"status":"normal"}"#;

    fn setup(product_id: Option<u64>, project_id: Option<u64>) -> (MockServer, AppContext) {
        let server = MockServer::start();
        let config = Config {
            url: server.base_url(),
            token: None,
            product_id,
            project_id,
            api_version: Some("v1".into()),
            account: None,
        };
        (server, AppContext::new(config, OutputFormat::Json, false))
    }

    #[tokio::test]
    async fn list_routes_to_execution_when_execution_is_set() {
        let (server, ctx) = setup(None, None);

        let mock = server.mock(|when, then| {
            when.method(GET)
                .path("/api.php/v1/executions/5/builds");
            then.status(200)
                .json_body(serde_json::json!({"total":1,"builds":[serde_json::from_str::<serde_json::Value>(BUILD_JSON).unwrap()]}));
        });

        let result = BuildService::list(&ctx, None, None, Some(5)).await.unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].name, "build-1");
        mock.assert();
    }

    #[tokio::test]
    async fn list_routes_to_project_when_no_execution_but_project() {
        let (server, ctx) = setup(None, Some(3));

        let mock = server.mock(|when, then| {
            when.method(GET).path("/api.php/v1/projects/3/builds");
            then.status(200)
                .json_body(serde_json::json!({"total":0,"builds":[]}));
        });

        let result = BuildService::list(&ctx, Some(3), None, None).await.unwrap();
        assert!(result.is_empty());
        mock.assert();
    }

    #[tokio::test]
    async fn create_fails_without_project() {
        let (server, ctx) = setup(None, None);

        // put a mock up just in case, but it should not be hit
        let mock = server.mock(|when, then| {
            when.method(POST).path_contains("/builds");
            then.status(201).json_body(serde_json::json!({"id":10}));
        });

        let req = CreateBuildRequest {
            execution: 1,
            product: 1,
            name: "test".into(),
            builder: "builder".into(),
            branch: None,
            date: None,
            scm_path: None,
            file_path: None,
            desc: None,
        };
        let result = BuildService::create(&ctx, None, req).await;
        assert!(result.is_err());
        assert_eq!(mock.hits(), 0);
    }
}
