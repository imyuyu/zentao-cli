use crate::api::{Doc, DocApi};
use crate::core::logging::{log, LogLevel};
use crate::core::AppContext;
use anyhow::Result;

pub struct DocService;

impl DocService {
    pub async fn list(ctx: &AppContext) -> Result<Vec<Doc>> {
        log(LogLevel::Info, "DocService", "list");
        let client = ctx.client();
        DocApi::list(&client).await
    }

    pub async fn get(ctx: &AppContext, id: u64) -> Result<Doc> {
        log(LogLevel::Info, "DocService", format!("get id={}", id));
        let client = ctx.client();
        DocApi::get(&client, id).await
    }
}
