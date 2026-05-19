//! ZenTao Feedback(反馈)命令模块

use crate::api::feedback::{
    AssignFeedbackRequest, CloseFeedbackRequest, CreateFeedbackRequest, UpdateFeedbackRequest,
};
use crate::cmd::common::{
    log_command, print_dry_run, print_dry_run_with_body, print_error, print_json,
};
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
        FeedbackSubcommand::Create {
            product,
            title,
            type_,
            desc,
        } => {
            let product_id = match ctx.require_product_id(*product) {
                Ok(id) => id,
                Err(e) => {
                    print_error(&e);
                    return;
                }
            };
            let req = CreateFeedbackRequest {
                product: product_id,
                title: title.clone(),
                module: None,
                type_: type_.clone(),
                desc: desc.clone(),
                public: None,
                notify: None,
                notify_email: None,
                feedback_by: None,
            };
            if ctx.dry_run {
                print_dry_run_with_body(
                    "FeedbackService::create()",
                    &format!("{}/api.php/v1/feedbacks", ctx.config.url),
                    &req,
                );
                return;
            }
            match FeedbackService::create(ctx, req).await {
                Ok(feedback) => print_json(&feedback),
                Err(e) => print_error(&e),
            }
        }
        FeedbackSubcommand::Assign {
            id,
            assigned_to,
            comment,
        } => {
            let req = AssignFeedbackRequest {
                assigned_to: Some(assigned_to.clone()),
                comment: comment.clone(),
                mailto: None,
            };
            if ctx.dry_run {
                print_dry_run_with_body(
                    "FeedbackService::assign()",
                    &format!("{}/api.php/v1/feedbacks/{}/assign", ctx.config.url, id),
                    &req,
                );
                return;
            }
            match FeedbackService::assign(ctx, *id, req).await {
                Ok(feedback) => print_json(&feedback),
                Err(e) => print_error(&e),
            }
        }
        FeedbackSubcommand::Close {
            id,
            closed_reason,
            comment,
        } => {
            let req = CloseFeedbackRequest {
                closed_reason: closed_reason.clone(),
                comment: comment.clone(),
            };
            if ctx.dry_run {
                print_dry_run_with_body(
                    "FeedbackService::close()",
                    &format!("{}/api.php/v1/feedbacks/{}/close", ctx.config.url, id),
                    &req,
                );
                return;
            }
            match FeedbackService::close(ctx, *id, req).await {
                Ok(feedback) => print_json(&feedback),
                Err(e) => print_error(&e),
            }
        }
        FeedbackSubcommand::Update { id, title, desc } => {
            let req = UpdateFeedbackRequest {
                product: None,
                module: None,
                title: title.clone(),
                type_: None,
                desc: desc.clone(),
                public: None,
                notify: None,
                notify_email: None,
                feedback_by: None,
            };
            if ctx.dry_run {
                print_dry_run_with_body(
                    "FeedbackService::update()",
                    &format!("{}/api.php/v1/feedbacks/{}", ctx.config.url, id),
                    &req,
                );
                return;
            }
            match FeedbackService::update(ctx, *id, req).await {
                Ok(feedback) => print_json(&feedback),
                Err(e) => print_error(&e),
            }
        }
        FeedbackSubcommand::Delete { id } => {
            if ctx.dry_run {
                print_dry_run(
                    "FeedbackService::delete()",
                    &format!("{}/api.php/v1/feedbacks/{}", ctx.config.url, id),
                );
                return;
            }
            match FeedbackService::delete(ctx, *id).await {
                Ok(_) => {
                    println!("Feedback [{}] deleted successfully", id);
                }
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
                println!("  [{}] {} - {}", item.id, item.title, item.status);
            }
        }
        _ => println!(
            "{}",
            serde_json::to_string_pretty(feedbacks).unwrap_or_default()
        ),
    }
}
