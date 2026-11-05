#![allow(snake_case)]
use crate::api::{Bug, BugApi, CreateBugRequest, UpdateBugRequest};
use crate::core::logging::{log, LogLevel};
use crate::core::AppContext;
use crate::core::ZentaoError;
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
        let product_id = ctx.require_product_id(product)?;

        // 禅道 21.6 的 /products/{id}/bugs 不认 assignedTo 参数（传不存在的账号
        // 也返回全量），只能客户端过滤。过滤就必须翻页拿全量，否则目标 bug
        // 不在第一页时结果为空。
        let Some(account) = assigned_to else {
            return BugApi::list(&client, product_id, status, None).await;
        };
        let account = account.trim().to_lowercase();

        let mut matched = Vec::new();
        let mut page = 1u32;
        loop {
            let batch =
                BugApi::list_with_pagination(&client, product_id, status.clone(), None, page, 100)
                    .await?;
            if batch.is_empty() {
                break;
            }
            matched.extend(
                batch
                    .iter()
                    .filter(|b| {
                        b.assigned_to
                            .as_ref()
                            .is_some_and(|u| u.account_or_realname().to_lowercase() == account)
                    })
                    .cloned(),
            );
            // 最后一页（不满 limit）即结束
            if batch.len() < 100 {
                break;
            }
            page += 1;
            if page > 200 {
                break; // 兜底，避免异常数据导致无限翻页
            }
        }
        Ok(matched)
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
        project: Option<u64>,
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
        // 提 Bug 表单上的必填项：所属产品/项目/执行/模块/影响版本/标题/类型/严重程度/优先级/重现步骤。
        // 禅道对缺失的**非文档标注必填**字段（project/execution/module）不报错，
        // 而是返回 201 + 空 body 且不落库——静默丢弃，必须自己挡。
        let execution = execution.ok_or_else(|| {
            ZentaoError::Config(
                "execution (所属执行) 是必需的，用 --execution 指定（如 6）".to_string(),
            )
        })?;
        let project = ctx.require_project_id(project)?;
        let module = module.ok_or_else(|| {
            ZentaoError::Config(
                "module (所属模块) 是必需的，用 --module 指定（如 293）".to_string(),
            )
        })?;
        let req = CreateBugRequest {
            title,
            product: ctx.require_product_id(product)?,
            project: Some(project),
            severity,
            pri,
            type_,
            steps,
            story,
            branch,
            module: Some(module),
            execution: Some(execution),
            keywords,
            os,
            browser,
            deadline,
            // openedBuild 是禅道的必填项，不传会报「影响版本不能为空」
            opened_build: Some(opened_build.unwrap_or_else(|| vec!["trunk".to_string()])),
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

    pub async fn close(ctx: &AppContext, id: u64, comment: Option<&str>) -> Result<Bug> {
        log(LogLevel::Info, "BugService", format!("close id={}", id));
        let client = ctx.client();
        BugApi::close(&client, id, comment).await
    }

    pub async fn activate(
        ctx: &AppContext,
        id: u64,
        assigned_to: Option<&str>,
        opened_build: Option<Vec<String>>,
        comment: Option<&str>,
    ) -> Result<Bug> {
        log(LogLevel::Info, "BugService", format!("activate id={}", id));
        let client = ctx.client();
        BugApi::activate(&client, id, assigned_to, opened_build, comment).await
    }

    pub async fn delete(ctx: &AppContext, id: u64) -> Result<()> {
        log(LogLevel::Info, "BugService", format!("delete id={}", id));
        let client = ctx.client();
        BugApi::delete(&client, id).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{Config, OutputFormat};
    use httpmock::prelude::*;

    const BUG_JSON: &str =
        r#"{"id":99,"title":"Crash on login","product":1,"status":"active","severity":2,"pri":1}"#;

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
    async fn list_fails_without_product() {
        let (_server, ctx) = setup(None);
        let result = BugService::list(&ctx, None, None, None).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn resolve_posts_then_gets_bug() {
        let (server, ctx) = setup(None);

        let post_mock = server.mock(|when, then| {
            when.method(POST)
                .path("/api.php/v1/bugs/1/resolve")
                .body_contains(r#""resolution":"fixed""#);
            then.status(200).json_body(serde_json::json!({}));
        });
        let get_mock = server.mock(|when, then| {
            when.method(GET).path("/api.php/v1/bugs/1");
            then.status(200)
                .json_body(serde_json::from_str::<serde_json::Value>(BUG_JSON).unwrap());
        });

        let result = BugService::resolve(&ctx, 1, "fixed", "build-2", None, None, None, None)
            .await
            .unwrap();
        assert_eq!(result.id, 99);
        assert_eq!(result.title, "Crash on login");
        post_mock.assert();
        get_mock.assert();
    }

    #[tokio::test]
    async fn confirm_posts_to_confirm_then_gets_bug() {
        let (server, ctx) = setup(None);

        let post_mock = server.mock(|when, then| {
            when.method(POST).path("/api.php/v1/bugs/2/confirm");
            then.status(200).json_body(serde_json::json!({}));
        });
        let get_mock = server.mock(|when, then| {
            when.method(GET).path("/api.php/v1/bugs/2");
            then.status(200)
                .json_body(serde_json::from_str::<serde_json::Value>(BUG_JSON).unwrap());
        });

        let result = BugService::confirm(&ctx, 2, Some("dev1"), None, None, None, None)
            .await
            .unwrap();
        assert_eq!(result.title, "Crash on login");
        post_mock.assert();
        get_mock.assert();
    }

    #[tokio::test]
    async fn activate_posts_to_active_then_gets_bug() {
        let (server, ctx) = setup(None);

        let post_mock = server.mock(|when, then| {
            when.method(POST).path("/api.php/v1/bugs/3/active");
            then.status(200).json_body(serde_json::json!({}));
        });
        let get_mock = server.mock(|when, then| {
            when.method(GET).path("/api.php/v1/bugs/3");
            then.status(200)
                .json_body(serde_json::from_str::<serde_json::Value>(BUG_JSON).unwrap());
        });

        let result = BugService::activate(&ctx, 3, None, None, None)
            .await
            .unwrap();
        assert_eq!(result.id, 99);
        post_mock.assert();
        get_mock.assert();
    }

    /// 禅道 21.6 的接口不认 assignedTo，必须在客户端过滤；
    /// 且目标 bug 可能不在第一页，所以要翻页拿全量。
    #[tokio::test]
    async fn assigned_to_filters_client_side_across_pages() {
        let (server, ctx) = setup(Some(1));

        // 第 1 页：100 条别人的 bug（满页，触发翻页）
        let mut page1 = Vec::new();
        for i in 1..=100 {
            page1.push(serde_json::json!({
                "id": i, "title": "other", "product": 1, "status": "active",
                "severity": 2, "pri": 1,
                "assignedTo": {"id": 1, "account": "someone_else"}
            }));
        }
        // 第 2 页：1 条目标 bug（不满页，结束）
        let page2 = vec![serde_json::json!({
            "id": 999, "title": "mine", "product": 1, "status": "active",
            "severity": 2, "pri": 1,
            "assignedTo": {"id": 2, "account": "me"}
        })];

        let m1 = server.mock(|when, then| {
            when.method(GET)
                .path("/api.php/v1/products/1/bugs")
                .query_param("page", "1");
            then.status(200)
                .json_body(serde_json::json!({"total": 101, "bugs": page1}));
        });
        let m2 = server.mock(|when, then| {
            when.method(GET)
                .path("/api.php/v1/products/1/bugs")
                .query_param("page", "2");
            then.status(200)
                .json_body(serde_json::json!({"total": 101, "bugs": page2}));
        });

        let bugs = BugService::list(&ctx, Some(1), None, Some("me".into()))
            .await
            .unwrap();

        assert_eq!(bugs.len(), 1, "应只返回自己的 bug");
        assert_eq!(bugs[0].id, 999);
        m1.assert();
        m2.assert();
    }

    #[tokio::test]
    async fn list_without_assigned_to_keeps_first_page_only() {
        let (server, ctx) = setup(Some(1));
        let mock = server.mock(|when, then| {
            when.method(GET).path("/api.php/v1/products/1/bugs");
            then.status(200).json_body(serde_json::json!({
                "total": 1,
                "bugs": [serde_json::from_str::<serde_json::Value>(BUG_JSON).unwrap()]
            }));
        });

        let bugs = BugService::list(&ctx, Some(1), None, None).await.unwrap();
        assert_eq!(bugs.len(), 1);
        mock.assert();
    }
}
