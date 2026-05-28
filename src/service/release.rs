use crate::api::release::{CreateReleaseRequest, UpdateReleaseRequest};
use crate::api::{Release, ReleaseApi};
use crate::core::logging::{log, LogLevel};
use crate::core::AppContext;
use anyhow::Result;

pub struct ReleaseService;

impl ReleaseService {
    pub async fn list(
        ctx: &AppContext,
        product: Option<u64>,
        project: Option<u64>,
    ) -> Result<Vec<Release>> {
        log(LogLevel::Info, "ReleaseService", "list");
        let client = ctx.client();
        if let Some(pid) = ctx.product_id(product) {
            ReleaseApi::list_by_product(&client, pid).await
        } else if let Some(pid) = ctx.project_id(project) {
            ReleaseApi::list_by_project(&client, pid).await
        } else {
            ReleaseApi::list(&client).await
        }
    }

    pub async fn get(ctx: &AppContext, id: u64) -> Result<Release> {
        log(LogLevel::Info, "ReleaseService", format!("get id={}", id));
        let client = ctx.client();
        ReleaseApi::get(&client, id).await
    }

    pub async fn create(ctx: &AppContext, req: CreateReleaseRequest) -> Result<Release> {
        log(LogLevel::Info, "ReleaseService", "create");
        let client = ctx.client();
        ReleaseApi::create(&client, &req).await
    }

    pub async fn update(ctx: &AppContext, id: u64, req: UpdateReleaseRequest) -> Result<Release> {
        log(
            LogLevel::Info,
            "ReleaseService",
            format!("update id={}", id),
        );
        let client = ctx.client();
        ReleaseApi::update(&client, id, &req).await
    }

    pub async fn delete(ctx: &AppContext, id: u64) -> Result<()> {
        log(
            LogLevel::Info,
            "ReleaseService",
            format!("delete id={}", id),
        );
        let client = ctx.client();
        ReleaseApi::delete(&client, id).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{Config, OutputFormat};
    use httpmock::prelude::*;

    const RELEASE_JSON: &str = r#"{"id":1,"name":"v1.0","product":1,"status":"normal"}"#;

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
        let ctx = AppContext::new(config, OutputFormat::Json, false);
        (server, ctx)
    }

    #[tokio::test]
    async fn list_routes_to_product_when_product_is_set() {
        let (server, ctx) = setup(Some(1), None);

        let mock = server.mock(|when, then| {
            when.method(GET)
                .path("/api.php/v1/products/1/releases");
            then.status(200)
                .json_body(serde_json::json!({"releases": [serde_json::from_str::<serde_json::Value>(RELEASE_JSON).unwrap()]}));
        });

        let result = ReleaseService::list(&ctx, Some(1), None).await.unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].name, "v1.0");
        mock.assert();
    }

    #[tokio::test]
    async fn list_routes_to_product_when_both_set() {
        let (server, ctx) = setup(Some(1), Some(2));

        let mock = server.mock(|when, then| {
            when.method(GET)
                .path("/api.php/v1/products/1/releases");
            then.status(200)
                .json_body(serde_json::json!({"releases": [serde_json::from_str::<serde_json::Value>(RELEASE_JSON).unwrap()]}));
        });

        let result = ReleaseService::list(&ctx, Some(1), Some(2)).await.unwrap();
        assert_eq!(result.len(), 1);
        mock.assert();
    }

    #[tokio::test]
    async fn list_routes_to_project_when_only_project_set() {
        let (server, ctx) = setup(None, Some(2));

        let mock = server.mock(|when, then| {
            when.method(GET)
                .path("/api.php/v1/projects/2/releases");
            then.status(200)
                .json_body(serde_json::json!({"releases": [serde_json::from_str::<serde_json::Value>(RELEASE_JSON).unwrap()]}));
        });

        let result = ReleaseService::list(&ctx, None, Some(2)).await.unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].name, "v1.0");
        mock.assert();
    }

    #[tokio::test]
    async fn list_routes_to_global_when_neither_set() {
        let (server, ctx) = setup(None, None);

        let mock = server.mock(|when, then| {
            when.method(GET)
                .path("/api.php/v1/releases");
            then.status(200)
                .json_body(serde_json::json!({"status":"success","data":[serde_json::from_str::<serde_json::Value>(RELEASE_JSON).unwrap()]}));
        });

        let result = ReleaseService::list(&ctx, None, None).await.unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].id, 1);
        mock.assert();
    }
}
