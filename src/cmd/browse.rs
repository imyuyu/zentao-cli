use anyhow::Result;
use crate::core::Config;
use crate::api::{ApiClient, BugApi, StoryApi};
use crate::tui::{Browser, App};

pub fn bug_browse(config: &Config) -> Result<()> {
    let rt = tokio::runtime::Runtime::new()?;

    rt.block_on(async {
        let client = ApiClient::new(&config.url, config.token.clone())
            .with_api_version(config.api_version.as_deref().unwrap_or("v1"));
        let product_id = config.product_id.unwrap_or(1);

        let mut app = App::new();
        app.set_loading("Fetching bugs...".to_string());

        // Spawn async task
        let bugs = BugApi::list(&client, product_id, Some("active".to_string()), None).await;

        match bugs {
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

    rt.block_on(async {
        let client = ApiClient::new(&config.url, config.token.clone())
            .with_api_version(config.api_version.as_deref().unwrap_or("v1"));

        let mut app = App::new();
        app.set_loading("Fetching stories...".to_string());

        let stories = StoryApi::list(&client, config.product_id, None, config.project_id).await;

        match stories {
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
