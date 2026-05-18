use crate::api::{
    AssignFeedbackRequest, CloseFeedbackRequest, CreateFeedbackRequest, Feedback, FeedbackApi,
    UpdateFeedbackRequest,
};
use crate::core::logging::{log, LogLevel};
use crate::core::AppContext;
use anyhow::Result;

pub struct FeedbackService;

impl FeedbackService {
    pub async fn list(ctx: &AppContext, page: u32, limit: u32) -> Result<Vec<Feedback>> {
        log(LogLevel::Info, "FeedbackService", "list");
        let client = ctx.client();
        FeedbackApi::list(&client, page, limit).await
    }

    pub async fn get(ctx: &AppContext, id: u64) -> Result<Feedback> {
        log(LogLevel::Info, "FeedbackService", format!("get id={}", id));
        let client = ctx.client();
        FeedbackApi::get(&client, id).await
    }

    pub async fn create(ctx: &AppContext, req: CreateFeedbackRequest) -> Result<Feedback> {
        log(LogLevel::Info, "FeedbackService", "create");
        let client = ctx.client();
        FeedbackApi::create(&client, &req).await
    }

    pub async fn assign(ctx: &AppContext, id: u64, req: AssignFeedbackRequest) -> Result<Feedback> {
        log(
            LogLevel::Info,
            "FeedbackService",
            format!("assign id={}", id),
        );
        let client = ctx.client();
        FeedbackApi::assign(&client, id, &req).await
    }

    pub async fn close(ctx: &AppContext, id: u64, req: CloseFeedbackRequest) -> Result<Feedback> {
        log(
            LogLevel::Info,
            "FeedbackService",
            format!("close id={}", id),
        );
        let client = ctx.client();
        FeedbackApi::close(&client, id, &req).await
    }

    pub async fn update(ctx: &AppContext, id: u64, req: UpdateFeedbackRequest) -> Result<Feedback> {
        log(
            LogLevel::Info,
            "FeedbackService",
            format!("update id={}", id),
        );
        let client = ctx.client();
        FeedbackApi::update(&client, id, &req).await
    }

    pub async fn delete(ctx: &AppContext, id: u64) -> Result<()> {
        log(
            LogLevel::Info,
            "FeedbackService",
            format!("delete id={}", id),
        );
        let client = ctx.client();
        FeedbackApi::delete(&client, id).await
    }
}
