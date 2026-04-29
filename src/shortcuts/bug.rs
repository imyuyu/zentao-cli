use crate::api::{ApiClient, BugApi};
use crate::core::Config;
use anyhow::Result;

pub struct BugShortcut;

impl BugShortcut {
    pub fn list(
        config: &Config,
        status: Option<String>,
        assigned_to: Option<String>,
    ) -> Result<()> {
        let rt = tokio::runtime::Runtime::new()?;
        let api_version = config.api_version.as_deref().unwrap_or("v1");
        rt.block_on(async {
            let client =
                ApiClient::new(&config.url, config.token.clone()).with_api_version(api_version);
            let product_id = config.product_id.unwrap_or(1);
            match BugApi::list(&client, product_id, status, assigned_to).await {
                Ok(bugs) => {
                    println!("Bugs:");
                    for bug in bugs {
                        println!(
                            "  [{}] {} - {} (Severity: {})",
                            bug.id, bug.title, bug.status, bug.severity
                        );
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
            let client =
                ApiClient::new(&config.url, config.token.clone()).with_api_version(api_version);
            match BugApi::get(&client, id).await {
                Ok(bug) => {
                    println!("Bug #{}", bug.id);
                    println!("  Title: {}", bug.title);
                    println!("  Status: {}", bug.status);
                    println!("  Severity: {}", bug.severity);
                    println!("  Priority: {}", bug.pri);
                    if let Some(resolution) = &bug.resolution {
                        println!("  Resolution: {}", resolution);
                    }
                    if let Some(steps) = &bug.steps {
                        println!("  Steps:\n{}", steps);
                    }
                }
                Err(e) => eprintln!("Error: {}", e),
            }
        });
        Ok(())
    }

    pub fn my_bugs(config: &Config) -> Result<()> {
        Self::list(config, Some("active".to_string()), Some("me".to_string()))
    }
}
