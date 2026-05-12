use crate::api::{Feedback, FeedbackApi};
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
}
