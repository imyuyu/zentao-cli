use crate::api::testcase::{
    CreateTestcaseRequest, TestcaseApi, TestcaseResultRequest, UpdateTestcaseRequest,
};
use crate::api::Testcase;
use crate::core::logging::{log, LogLevel};
use crate::core::AppContext;
use anyhow::Result;

pub struct TestcaseService;

impl TestcaseService {
    pub async fn list(
        ctx: &AppContext,
        product: Option<u64>,
        project: Option<u64>,
        type_: Option<String>,
        status: Option<String>,
    ) -> Result<Vec<Testcase>> {
        log(LogLevel::Info, "TestcaseService", "list");
        let client = ctx.client();
        TestcaseApi::list(
            &client,
            ctx.product_id(product),
            ctx.project_id(project),
            type_,
            status,
        )
        .await
    }

    pub async fn get(ctx: &AppContext, id: u64) -> Result<Testcase> {
        log(LogLevel::Info, "TestcaseService", format!("get id={}", id));
        let client = ctx.client();
        TestcaseApi::get(&client, id).await
    }

    pub async fn create(
        ctx: &AppContext,
        product: Option<u64>,
        req: CreateTestcaseRequest,
    ) -> Result<Testcase> {
        log(LogLevel::Info, "TestcaseService", "create");
        let client = ctx.client();
        TestcaseApi::create(&client, ctx.require_product_id(product)?, &req).await
    }

    pub async fn update(ctx: &AppContext, id: u64, req: UpdateTestcaseRequest) -> Result<Testcase> {
        log(
            LogLevel::Info,
            "TestcaseService",
            format!("update id={}", id),
        );
        let client = ctx.client();
        TestcaseApi::update(&client, id, &req).await
    }

    pub async fn delete(ctx: &AppContext, id: u64) -> Result<()> {
        log(
            LogLevel::Info,
            "TestcaseService",
            format!("delete id={}", id),
        );
        let client = ctx.client();
        TestcaseApi::delete(&client, id).await
    }

    pub async fn create_result(
        ctx: &AppContext,
        id: u64,
        req: TestcaseResultRequest,
    ) -> Result<Testcase> {
        log(
            LogLevel::Info,
            "TestcaseService",
            format!("create_result id={}", id),
        );
        let client = ctx.client();
        TestcaseApi::create_result(&client, id, &req).await
    }
}
