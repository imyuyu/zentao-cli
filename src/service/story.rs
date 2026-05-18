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
