use crate::api::{Bug, BugApi, CreateBugRequest, UpdateBugRequest};
use crate::core::logging::{log, LogLevel};
use crate::core::AppContext;
use anyhow::Result;

pub struct BugService;

impl BugService {
    pub async fn list(
        ctx: &AppContext,
        product: Option<u64>,
        status: Option<String>,
        assigned_to: Option<String>,
    ) -> Result<Vec<Bug>> {
        log(LogLevel::Info, "BugService", "list");
        let client = ctx.client();
        BugApi::list(
            &client,
            ctx.require_product_id(product)?,
            status,
            assigned_to,
        )
        .await
    }

    pub async fn get(ctx: &AppContext, id: u64) -> Result<Bug> {
        log(LogLevel::Info, "BugService", format!("get id={}", id));
        let client = ctx.client();
        BugApi::get(&client, id).await
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn create(
        ctx: &AppContext,
        title: String,
        product: Option<u64>,
        severity: u8,
        pri: Option<u8>,
        type_: Option<String>,
        steps: Option<String>,
        story: Option<u64>,
        branch: Option<u64>,
        module: Option<u64>,
        execution: Option<u64>,
        keywords: Option<String>,
        os: Option<String>,
        browser: Option<String>,
        deadline: Option<String>,
        opened_build: Option<Vec<String>>,
    ) -> Result<Bug> {
        log(LogLevel::Info, "BugService", "create");
        let client = ctx.client();
        let req = CreateBugRequest {
            title,
            product: ctx.require_product_id(product)?,
            severity,
            pri,
            type_,
            steps,
            story,
            branch,
            module,
            execution,
            keywords,
            os,
            browser,
            deadline,
            opened_build,
        };

        BugApi::create(&client, &req).await
    }

    pub async fn update(ctx: &AppContext, id: u64, req: UpdateBugRequest) -> Result<Bug> {
        log(LogLevel::Info, "BugService", format!("update id={}", id));
        let client = ctx.client();
        BugApi::update(&client, id, &req).await
    }

    pub async fn resolve(
        ctx: &AppContext,
        id: u64,
        resolution: &str,
        resolved_build: &str,
        assigned_to: Option<&str>,
        duplicate_bug: Option<u64>,
        resolved_date: Option<&str>,
        comment: Option<&str>,
    ) -> Result<Bug> {
        log(LogLevel::Info, "BugService", format!("resolve id={}", id));
        let client = ctx.client();
        BugApi::resolve(
            &client,
            id,
            resolution,
            resolved_build,
            assigned_to,
            duplicate_bug,
            resolved_date,
            comment,
        )
        .await
    }

    pub async fn confirm(
        ctx: &AppContext,
        id: u64,
        assigned_to: Option<&str>,
        type_: Option<&str>,
        pri: Option<u8>,
        mailto: Option<Vec<String>>,
        comment: Option<&str>,
    ) -> Result<Bug> {
        log(LogLevel::Info, "BugService", format!("confirm id={}", id));
        let client = ctx.client();
        BugApi::confirm(&client, id, assigned_to, type_, pri, mailto, comment).await
    }

    pub async fn close(ctx: &AppContext, id: u64) -> Result<Bug> {
        log(LogLevel::Info, "BugService", format!("close id={}", id));
        let client = ctx.client();
        BugApi::close(&client, id).await
    }

    pub async fn activate(ctx: &AppContext, id: u64) -> Result<Bug> {
        log(LogLevel::Info, "BugService", format!("activate id={}", id));
        let client = ctx.client();
        BugApi::activate(&client, id).await
    }

    pub async fn delete(ctx: &AppContext, id: u64) -> Result<()> {
        log(LogLevel::Info, "BugService", format!("delete id={}", id));
        let client = ctx.client();
        BugApi::delete(&client, id).await
    }
}
