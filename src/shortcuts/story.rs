use anyhow::Result;
use crate::core::Config;
use crate::api::{ApiClient, StoryApi};

pub struct StoryShortcut;

impl StoryShortcut {
    pub fn list(config: &Config, status: Option<String>) -> Result<()> {
        let rt = tokio::runtime::Runtime::new()?;
        let api_version = config.api_version.as_deref().unwrap_or("v1");
        rt.block_on(async {
            let client = ApiClient::new(&config.url, config.token.clone()).with_api_version(api_version);
            match StoryApi::list(&client, config.product_id, status, config.project_id).await {
                Ok(stories) => {
                    println!("Stories:");
                    for story in stories {
                        println!("  [{}] {} - {}", story.id, story.title, story.status);
                    }
                }
                Err(e) => eprintln!("Error: {}", e),
            }
        });
        Ok(())
    }

    pub fn get(config: &Config, id: u64) -> Result<()> {
        let rt = tokio::runtime::Runtime::new()?;
        let api_version = config.api_version.as_deref().unwrap_or("v1");
        rt.block_on(async {
            let client = ApiClient::new(&config.url, config.token.clone()).with_api_version(api_version);
            match StoryApi::get(&client, id).await {
                Ok(story) => {
                    println!("Story #{}", story.id);
                    println!("  Title: {}", story.title);
                    println!("  Status: {}", story.status);
                    println!("  Priority: {}", story.pri);
                    if let Some(desc) = &story.description {
                        println!("  Description: {}", desc);
                    }
                }
                Err(e) => eprintln!("Error: {}", e),
            }
        });
        Ok(())
    }
}
