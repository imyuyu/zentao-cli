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
}
