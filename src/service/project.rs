use crate::api::{CreateProjectRequest, Project, ProjectApi, UpdateProjectRequest};
use crate::core::logging::{log, LogLevel};
use crate::core::AppContext;
use anyhow::Result;

pub struct ProjectService;

impl ProjectService {
    pub async fn list(ctx: &AppContext) -> Result<Vec<Project>> {
        log(LogLevel::Info, "ProjectService", "list");
        let client = ctx.client();
        ProjectApi::list(&client).await
    }

    pub async fn list_with_pagination(
        ctx: &AppContext,
        page: u32,
        limit: u32,
    ) -> Result<Vec<Project>> {
        log(
            LogLevel::Info,
            "ProjectService",
            format!("list_with_pagination page={} limit={}", page, limit),
        );
        let client = ctx.client();
        ProjectApi::list_with_pagination(&client, page, limit).await
    }

    pub async fn get(ctx: &AppContext, id: u64) -> Result<Project> {
        log(LogLevel::Info, "ProjectService", format!("get id={}", id));
        let client = ctx.client();
        ProjectApi::get(&client, id).await
    }

    pub async fn create(ctx: &AppContext, req: CreateProjectRequest) -> Result<Project> {
        log(LogLevel::Info, "ProjectService", "create");
        let client = ctx.client();
        ProjectApi::create(&client, &req).await
    }

    pub async fn update(ctx: &AppContext, id: u64, req: UpdateProjectRequest) -> Result<Project> {
        log(
            LogLevel::Info,
            "ProjectService",
            format!("update id={}", id),
        );
        let client = ctx.client();
        ProjectApi::update(&client, id, &req).await
    }

    pub async fn delete(ctx: &AppContext, id: u64) -> Result<()> {
        log(
            LogLevel::Info,
            "ProjectService",
            format!("delete id={}", id),
        );
        let client = ctx.client();
        ProjectApi::delete(&client, id).await
    }

    pub async fn get_name(ctx: &AppContext, id: u64) -> Result<String> {
        log(LogLevel::Info, "ProjectService", format!("get_name id={}", id));
        let project = Self::get(ctx, id).await?;
        Ok(project.name)
    }
}
