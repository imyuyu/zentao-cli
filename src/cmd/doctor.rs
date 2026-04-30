use crate::api::Auth;
use crate::core::{global_config_path, load_config, project_config_path};
use crate::safe_println;
use anyhow::Result;

pub async fn run_doctor() -> Result<()> {
    safe_println("ZenTao CLI Doctor");
    safe_println("=================");
    println!();

    let mut has_errors = false;

    // Check configuration
    safe_println("[1/4] Checking configuration...");
    match load_config() {
        Ok(config) => {
            if config.url.is_empty() {
                safe_println("  ✗ ZENTAO_URL is not set");
                has_errors = true;
            } else {
                println!("  ✓ ZENTAO_URL: {}", config.url);
            }

            if config.token.is_none() || config.token.as_ref().map(|t| t.is_empty()).unwrap_or(true)
            {
                safe_println("  ✗ ZENTAO_TOKEN is not set");
                has_errors = true;
            } else {
                safe_println("  ✓ Token is configured");
            }

            if config.product_id.is_none() {
                safe_println("  ! ZENTAO_PRODUCT_ID not set (optional)");
            } else {
                println!("  ✓ Product ID: {:?}", config.product_id);
            }
        }
        Err(e) => {
            println!("  ✗ Failed to load config: {}", e);
            has_errors = true;
        }
    }

    // Check config files
    println!();
    safe_println("[2/4] Checking config files...");
    let global_path = global_config_path();
    if global_path.exists() {
        println!("  ✓ Global config: {}", global_path.display());
    } else {
        println!(
            "  - Global config: {} (not found, using defaults)",
            global_path.display()
        );
    }

    let project_path = project_config_path();
    if project_path.exists() {
        println!("  ✓ Project config: {}", project_path.display());
    } else {
        println!("  - Project config: {} (not found)", project_path.display());
    }

    // Check network
    println!();
    safe_println("[3/4] Checking network connectivity...");
    let config = load_config()?;
    if !config.url.is_empty() {
        let auth = Auth::new(&config.url);
        if let Some(token) = &config.token {
            if !token.is_empty() {
                match auth.verify_token(token).await {
                    Ok(true) => safe_println("  ✓ API connection successful"),
                    Ok(false) => {
                        safe_println("  ✗ Token verification failed");
                        has_errors = true;
                    }
                    Err(e) => {
                        println!("  ✗ Connection failed: {}", e);
                        has_errors = true;
                    }
                }
            }
        }
    }

    // Summary
    println!();
    safe_println("[4/4] Summary");
    if has_errors {
        safe_println("  Some checks failed. Please fix the issues above.");
    } else {
        safe_println("  ✓ All checks passed!");
    }

    println!();
    safe_println("For help, see: https://github.com/yourusername/zentao-cli#configuration");

    Ok(())
}
