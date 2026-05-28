use crate::api::{CreateStoryRequest, Story, StoryApi, UpdateStoryRequest};
use crate::core::logging::{log, LogLevel};
use crate::core::AppContext;
use anyhow::Result;

pub struct StoryService;

impl StoryService {
    pub async fn list(
        ctx: &AppContext,
        product: Option<u64>,
        project: Option<u64>,
        status: Option<String>,
    ) -> Result<Vec<Story>> {
        log(LogLevel::Info, "StoryService", "list");
        let client = ctx.client();
        StoryApi::list(
            &client,
            ctx.product_id(product),
            status,
            ctx.project_id(project),
        )
        .await
    }

    pub async fn get(ctx: &AppContext, id: u64) -> Result<Story> {
        log(LogLevel::Info, "StoryService", format!("get id={}", id));
        let client = ctx.client();
        StoryApi::get(&client, id).await
    }

    pub async fn create(
        ctx: &AppContext,
        title: String,
        product: Option<u64>,
        pri: u8,
        category: Option<String>,
        spec: Option<String>,
        estimate: Option<f64>,
    ) -> Result<Story> {
        log(LogLevel::Info, "StoryService", "create");
        let client = ctx.client();
        let req = CreateStoryRequest {
            title,
            product: ctx.require_product_id(product)?,
            pri,
            category,
            spec,
            verify: None,
            estimate,
            source: None,
            sourceNote: None,
            module: None,
            keywords: None,
        };

        StoryApi::create(&client, &req).await
    }

    pub async fn update(ctx: &AppContext, id: u64, req: UpdateStoryRequest) -> Result<Story> {
        log(LogLevel::Info, "StoryService", format!("update id={}", id));
        let client = ctx.client();
        StoryApi::update(&client, id, &req).await
    }

    pub async fn change(ctx: &AppContext, id: u64, req: UpdateStoryRequest) -> Result<Story> {
        log(LogLevel::Info, "StoryService", format!("change id={}", id));
        let client = ctx.client();
        StoryApi::change(&client, id, &req).await
    }

    pub async fn delete(ctx: &AppContext, id: u64) -> Result<serde_json::Value> {
        log(LogLevel::Info, "StoryService", format!("delete id={}", id));
        let client = ctx.client();
        StoryApi::delete(&client, id).await
    }

    pub async fn close(ctx: &AppContext, id: u64) -> Result<Story> {
        log(LogLevel::Info, "StoryService", format!("close id={}", id));
        let client = ctx.client();
        StoryApi::close(&client, id).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{Config, OutputFormat};
    use httpmock::prelude::*;

    const STORY_JSON: &str =
        r#"{"id":7,"title":"Feature X","status":"active","pri":3,"product":1}"#;

    fn setup(product_id: Option<u64>) -> (MockServer, AppContext) {
        let server = MockServer::start();
        let config = Config {
            url: server.base_url(),
            token: None,
            product_id,
            project_id: None,
            api_version: Some("v1".into()),
            account: None,
        };
        (server, AppContext::new(config, OutputFormat::Json, false))
    }

    #[tokio::test]
    async fn create_fails_without_product() {
        let (_server, ctx) = setup(None);
        let result = StoryService::create(&ctx, "Test".into(), None, 3, None, None, None).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn update_puts_then_gets_story() {
        let (server, ctx) = setup(None);

        let put_mock = server.mock(|when, then| {
            when.method(PUT).path("/api.php/v1/stories/1");
            then.status(200).json_body(serde_json::json!({}));
        });
        let get_mock = server.mock(|when, then| {
            when.method(GET).path("/api.php/v1/stories/1");
            then.status(200)
                .json_body(serde_json::from_str::<serde_json::Value>(STORY_JSON).unwrap());
        });

        let req = UpdateStoryRequest {
            title: Some("Updated".into()),
            module: None,
            source: None,
            sourceNote: None,
            pri: None,
            category: None,
            estimate: None,
            keywords: None,
            assigned_to: None,
            status: None,
        };
        let result = StoryService::update(&ctx, 1, req).await.unwrap();
        assert_eq!(result.title, "Feature X");
        put_mock.assert();
        get_mock.assert();
    }

    #[tokio::test]
    async fn change_posts_to_change_then_gets_story() {
        let (server, ctx) = setup(None);

        let post_mock = server.mock(|when, then| {
            when.method(POST).path("/api.php/v1/stories/2/change");
            then.status(200).json_body(serde_json::json!({}));
        });
        let get_mock = server.mock(|when, then| {
            when.method(GET).path("/api.php/v1/stories/2");
            then.status(200)
                .json_body(serde_json::from_str::<serde_json::Value>(STORY_JSON).unwrap());
        });

        let req = UpdateStoryRequest {
            title: Some("Changed".into()),
            module: None,
            source: None,
            sourceNote: None,
            pri: None,
            category: None,
            estimate: None,
            keywords: None,
            assigned_to: None,
            status: None,
        };
        let result = StoryService::change(&ctx, 2, req).await.unwrap();
        assert_eq!(result.title, "Feature X");
        post_mock.assert();
        get_mock.assert();
    }
}
