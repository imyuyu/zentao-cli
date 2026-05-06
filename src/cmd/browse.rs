use crate::core::{AppContext, Config, OutputFormat};
use crate::service::bug::BugService;
use crate::service::story::StoryService;
use crate::tui::{App, Browser};
use anyhow::Result;

pub fn bug_browse(config: &Config) -> Result<()> {
    let rt = tokio::runtime::Runtime::new()?;
    let ctx = AppContext::new(config.clone(), OutputFormat::Table, false);

    rt.block_on(async {
        let mut app = App::new();
        app.set_loading("Fetching bugs...".to_string());

        match BugService::list(&ctx, config.product_id, Some("active".to_string()), None).await {
            Ok(bugs) => {
                let mut app = App::new();
                app.set_bug_list(bugs);

                let mut browser = Browser::new()?;
                browser.run(&mut app)?;
            }
            Err(e) => {
                eprintln!("Error fetching bugs: {}", e);
            }
        }
        Ok(())
    })
}

pub fn story_browse(config: &Config) -> Result<()> {
    let rt = tokio::runtime::Runtime::new()?;
    let ctx = AppContext::new(config.clone(), OutputFormat::Table, false);

    rt.block_on(async {
        let mut app = App::new();
        app.set_loading("Fetching stories...".to_string());

        match StoryService::list(&ctx, config.product_id, config.project_id, None).await {
            Ok(stories) => {
                let mut app = App::new();
                app.set_story_list(stories);

                let mut browser = Browser::new()?;
                browser.run(&mut app)?;
            }
            Err(e) => {
                eprintln!("Error fetching stories: {}", e);
            }
        }
        Ok(())
    })
}
