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
