//! ZenTao Feedback(反馈)命令模块

use crate::cmd::common::{log_command, print_dry_run, print_error, print_json};
use crate::cmd::root::FeedbackSubcommand;
use crate::core::{AppContext, OutputFormat};
use crate::service::feedback::FeedbackService;

pub async fn run(cmd: &FeedbackSubcommand, ctx: &AppContext) {
    log_command("feedback", format!("{:?}", cmd));
    match cmd {
        FeedbackSubcommand::List => {
            if ctx.dry_run {
                print_dry_run(
                    "FeedbackService::list()",
                    &format!("{}/api.php/v1/feedbacks", ctx.config.url),
                );
                return;
            }
            match FeedbackService::list(ctx, 1, 100).await {
                Ok(feedbacks) => print_feedback_list(&feedbacks, ctx.format.clone()),
                Err(e) => print_error(&e),
            }
        }
        FeedbackSubcommand::Get { id } => {
            if ctx.dry_run {
                print_dry_run(
                    "FeedbackService::get()",
                    &format!("{}/api.php/v1/feedbacks/{}", ctx.config.url, id),
                );
                return;
            }
            match FeedbackService::get(ctx, *id).await {
                Ok(feedback) => print_json(&feedback),
                Err(e) => print_error(&e),
            }
        }
    }
}

fn print_feedback_list(feedbacks: &[crate::api::Feedback], format: OutputFormat) {
    match format {
        OutputFormat::Table => {
            println!("Feedbacks:");
            for item in feedbacks {
                let pri = item.pri.map(|p| p.to_string()).unwrap_or_else(|| "N/A".to_string());
                println!("  [{}] {} (pri: {}) - {}", item.id, item.title, pri, item.status);
            }
        }
        _ => println!(
            "{}",
            serde_json::to_string_pretty(feedbacks).unwrap_or_default()
        ),
    }
}
