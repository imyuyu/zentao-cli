use crate::core::{load_multi_account_config, AppContext, Config, OutputFormat};
use crate::service::bug::BugService;
use crate::service::build::BuildService;
use crate::service::department::DepartmentService;
use crate::service::execution::ExecutionService;
use crate::service::product::ProductService;
use crate::service::project::ProjectService;
use crate::service::release::ReleaseService;
use crate::service::story::StoryService;
use crate::service::user::UserService;
use crate::tui::{App, Browser};
use anyhow::Result;

/// Run the TUI home screen (module selection)
///
/// 进 TUI 前必须先有 token：没有 token 时每个模块都会加载失败并反复触发
/// refresh_token。缺配置或未登录时直接打开配置向导，而不是把错误抛到命令行。
pub fn run_tui(config: &Config) -> Result<()> {
    let mut config = config.clone();
    let has_token = config.token.as_ref().is_some_and(|t| !t.is_empty());

    if !has_token {
        let rt = tokio::runtime::Runtime::new()?;
        // 已有 URL 时只需登录；没有 URL 时走完整配置向导（URL/账号/密码/产品/项目）
        if config.url.trim().is_empty() {
            rt.block_on(crate::cmd::config_cmd::run(
                &crate::cmd::config_cmd::ConfigSubcommand::Init { global: false },
            ))?;
        } else {
            rt.block_on(crate::cmd::auth::run(
                &crate::cmd::auth::AuthSubcommand::Login {
                    account: None,
                    password: None,
                    global: false,
                },
                Some(&config.url),
                None,
            ))?;
        }

        // 向导/登录都会写配置文件，重载拿新凭据；文件里缺失时保留原有值
        if let Ok(saved) = crate::core::load_config() {
            if saved.token.as_ref().is_some_and(|t| !t.is_empty()) {
                config.token = saved.token;
                config.account = saved.account.or(config.account);
                config.product_id = saved.product_id.or(config.product_id);
                config.project_id = saved.project_id.or(config.project_id);
                if config.url.trim().is_empty() {
                    config.url = saved.url;
                }
            }
        }
        if config.token.as_ref().is_none_or(|t| t.is_empty()) {
            crate::core::output::safe_println("未登录，已退出");
            return Ok(());
        }
    }

    let mut app = App::new(
        config.clone(),
        load_multi_account_config().unwrap_or_default(),
    );
    app.set_main_menu();

    let mut browser = Browser::new()?;
    browser.run(&mut app)?;
    Ok(())
}

pub fn bug_browse(config: &Config) -> Result<()> {
    let rt = tokio::runtime::Runtime::new()?;
    let mut ctx = AppContext::new(config.clone(), OutputFormat::Table, false);

    rt.block_on(async {
        let mut app = App::new(
            config.clone(),
            load_multi_account_config().unwrap_or_default(),
        );
        app.set_loading("Fetching bugs...".to_string());

        // 尝试获取 bugs，失败时尝试刷新 token 重试
        let bugs = match BugService::list(&ctx, config.product_id, Some("active".to_string()), None)
            .await
        {
            Ok(bugs) => bugs,
            Err(e) => {
                // 检查是否是 401 错误，尝试刷新 token
                eprintln!("Error fetching bugs (trying token refresh): {}", e);
                if ctx.refresh_token().await.is_ok() {
                    match BugService::list(
                        &ctx,
                        config.product_id,
                        Some("active".to_string()),
                        None,
                    )
                    .await
                    {
                        Ok(bugs) => bugs,
                        Err(e2) => {
                            eprintln!("Error fetching bugs after token refresh: {}", e2);
                            return Ok(());
                        }
                    }
                } else {
                    eprintln!("Token refresh failed, skipping retry");
                    return Ok(());
                }
            }
        };

        let mut app = App::new(
            config.clone(),
            load_multi_account_config().unwrap_or_default(),
        );
        let product_name = if let Some(id) = config.product_id {
            ProductService::get_name(&ctx, id).await.ok()
        } else {
            None
        };
        app.set_bug_list(bugs, product_name);

        let mut browser = Browser::new()?;
        browser.run(&mut app)?;
        Ok(())
    })
}

pub fn story_browse(config: &Config) -> Result<()> {
    let rt = tokio::runtime::Runtime::new()?;
    let mut ctx = AppContext::new(config.clone(), OutputFormat::Table, false);

    rt.block_on(async {
        let mut app = App::new(
            config.clone(),
            load_multi_account_config().unwrap_or_default(),
        );
        app.set_loading("Fetching stories...".to_string());

        // 尝试获取 stories，失败时尝试刷新 token 重试
        let stories = match StoryService::list(&ctx, config.product_id, config.project_id, None)
            .await
        {
            Ok(stories) => stories,
            Err(e) => {
                eprintln!("Error fetching stories (trying token refresh): {}", e);
                if ctx.refresh_token().await.is_ok() {
                    match StoryService::list(&ctx, config.product_id, config.project_id, None).await
                    {
                        Ok(stories) => stories,
                        Err(e2) => {
                            eprintln!("Error fetching stories after token refresh: {}", e2);
                            return Ok(());
                        }
                    }
                } else {
                    eprintln!("Token refresh failed, skipping retry");
                    return Ok(());
                }
            }
        };

        let mut app = App::new(
            config.clone(),
            load_multi_account_config().unwrap_or_default(),
        );
        let product_name = if let Some(id) = config.product_id {
            ProductService::get_name(&ctx, id).await.ok()
        } else {
            None
        };
        app.set_story_list(stories, product_name);

        let mut browser = Browser::new()?;
        browser.run(&mut app)?;
        Ok(())
    })
}

pub fn execution_browse(config: &Config) -> Result<()> {
    let rt = tokio::runtime::Runtime::new()?;
    let mut ctx = AppContext::new(config.clone(), OutputFormat::Table, false);

    rt.block_on(async {
        let mut app = App::new(
            config.clone(),
            load_multi_account_config().unwrap_or_default(),
        );
        app.set_loading("Fetching executions...".to_string());

        // 尝试获取 executions，失败时尝试刷新 token 重试
        let executions = match ExecutionService::list(&ctx, config.project_id).await {
            Ok(executions) => executions,
            Err(e) => {
                eprintln!("Error fetching executions (trying token refresh): {}", e);
                if ctx.refresh_token().await.is_ok() {
                    match ExecutionService::list(&ctx, config.project_id).await {
                        Ok(executions) => executions,
                        Err(e2) => {
                            eprintln!("Error fetching executions after token refresh: {}", e2);
                            return Ok(());
                        }
                    }
                } else {
                    eprintln!("Token refresh failed, skipping retry");
                    return Ok(());
                }
            }
        };

        let mut app = App::new(
            config.clone(),
            load_multi_account_config().unwrap_or_default(),
        );
        let project_name = if let Some(id) = config.project_id {
            ProjectService::get_name(&ctx, id).await.ok()
        } else {
            None
        };
        app.set_execution_list(executions, project_name);

        let mut browser = Browser::new()?;
        browser.run(&mut app)?;
        Ok(())
    })
}

pub fn build_browse(config: &Config) -> Result<()> {
    let rt = tokio::runtime::Runtime::new()?;
    let mut ctx = AppContext::new(config.clone(), OutputFormat::Table, false);

    rt.block_on(async {
        let mut app = App::new(
            config.clone(),
            load_multi_account_config().unwrap_or_default(),
        );
        app.set_loading("Fetching builds...".to_string());

        // 尝试获取 builds，失败时尝试刷新 token 重试
        let builds = match BuildService::list(&ctx, config.project_id, config.product_id, None)
            .await
        {
            Ok(builds) => builds,
            Err(e) => {
                eprintln!("Error fetching builds (trying token refresh): {}", e);
                if ctx.refresh_token().await.is_ok() {
                    match BuildService::list(&ctx, config.project_id, config.product_id, None).await
                    {
                        Ok(builds) => builds,
                        Err(e2) => {
                            eprintln!("Error fetching builds after token refresh: {}", e2);
                            return Ok(());
                        }
                    }
                } else {
                    eprintln!("Token refresh failed, skipping retry");
                    return Ok(());
                }
            }
        };

        let mut app = App::new(
            config.clone(),
            load_multi_account_config().unwrap_or_default(),
        );
        let product_name = if let Some(id) = config.product_id {
            ProductService::get_name(&ctx, id).await.ok()
        } else {
            None
        };
        let project_name = if let Some(id) = config.project_id {
            ProjectService::get_name(&ctx, id).await.ok()
        } else {
            None
        };
        app.set_build_list(builds, product_name, project_name);

        let mut browser = Browser::new()?;
        browser.run(&mut app)?;
        Ok(())
    })
}

pub fn release_browse(config: &Config) -> Result<()> {
    let rt = tokio::runtime::Runtime::new()?;
    let mut ctx = AppContext::new(config.clone(), OutputFormat::Table, false);

    rt.block_on(async {
        let mut app = App::new(
            config.clone(),
            load_multi_account_config().unwrap_or_default(),
        );
        app.set_loading("Fetching releases...".to_string());

        // 尝试获取 releases，失败时尝试刷新 token 重试
        let releases = match ReleaseService::list(&ctx, config.product_id, config.project_id).await
        {
            Ok(releases) => releases,
            Err(e) => {
                eprintln!("Error fetching releases (trying token refresh): {}", e);
                if ctx.refresh_token().await.is_ok() {
                    match ReleaseService::list(&ctx, config.product_id, config.project_id).await {
                        Ok(releases) => releases,
                        Err(e2) => {
                            eprintln!("Error fetching releases after token refresh: {}", e2);
                            return Ok(());
                        }
                    }
                } else {
                    eprintln!("Token refresh failed, skipping retry");
                    return Ok(());
                }
            }
        };

        let mut app = App::new(
            config.clone(),
            load_multi_account_config().unwrap_or_default(),
        );
        let product_name = if let Some(id) = config.product_id {
            ProductService::get_name(&ctx, id).await.ok()
        } else {
            None
        };
        app.set_release_list(releases, product_name);

        let mut browser = Browser::new()?;
        browser.run(&mut app)?;
        Ok(())
    })
}

pub fn user_browse(config: &Config) -> Result<()> {
    let rt = tokio::runtime::Runtime::new()?;
    let mut ctx = AppContext::new(config.clone(), OutputFormat::Table, false);

    rt.block_on(async {
        let mut app = App::new(
            config.clone(),
            load_multi_account_config().unwrap_or_default(),
        );
        app.set_loading("Fetching users...".to_string());

        // 尝试获取 users，失败时尝试刷新 token 重试
        let users = match UserService::list(&ctx, None, None).await {
            Ok(users) => users,
            Err(e) => {
                eprintln!("Error fetching users (trying token refresh): {}", e);
                if ctx.refresh_token().await.is_ok() {
                    match UserService::list(&ctx, None, None).await {
                        Ok(users) => users,
                        Err(e2) => {
                            eprintln!("Error fetching users after token refresh: {}", e2);
                            return Ok(());
                        }
                    }
                } else {
                    eprintln!("Token refresh failed, skipping retry");
                    return Ok(());
                }
            }
        };

        let mut app = App::new(
            config.clone(),
            load_multi_account_config().unwrap_or_default(),
        );
        app.set_user_list(users);

        let mut browser = Browser::new()?;
        browser.run(&mut app)?;
        Ok(())
    })
}

pub fn department_browse(config: &Config) -> Result<()> {
    let rt = tokio::runtime::Runtime::new()?;
    let mut ctx = AppContext::new(config.clone(), OutputFormat::Table, false);

    rt.block_on(async {
        let mut app = App::new(
            config.clone(),
            load_multi_account_config().unwrap_or_default(),
        );
        app.set_loading("Fetching departments...".to_string());

        // 尝试获取 departments，失败时尝试刷新 token 重试
        let departments = match DepartmentService::list(&ctx).await {
            Ok(departments) => departments,
            Err(e) => {
                eprintln!("Error fetching departments (trying token refresh): {}", e);
                if ctx.refresh_token().await.is_ok() {
                    match DepartmentService::list(&ctx).await {
                        Ok(departments) => departments,
                        Err(e2) => {
                            eprintln!("Error fetching departments after token refresh: {}", e2);
                            return Ok(());
                        }
                    }
                } else {
                    eprintln!("Token refresh failed, skipping retry");
                    return Ok(());
                }
            }
        };

        let mut app = App::new(
            config.clone(),
            load_multi_account_config().unwrap_or_default(),
        );
        app.set_department_list(departments);

        let mut browser = Browser::new()?;
        browser.run(&mut app)?;
        Ok(())
    })
}
