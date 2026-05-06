//! ZenTao 用户(User)命令模块

use crate::cmd::common::{log_command, print_dry_run, print_error, print_json};
use crate::core::{AppContext, OutputFormat};
use crate::service::user::UserService;
use clap::Subcommand;

#[derive(Subcommand, Clone, Debug)]
pub enum UserAction {
    #[command(name = "list")]
    List {
        #[arg(long)]
        dept: Option<u64>,
        #[arg(long)]
        role: Option<String>,
    },
    #[command(name = "get")]
    Get { id: u64 },
}

pub async fn run(cmd: &UserAction, ctx: &AppContext) {
    log_command("user", format!("{:?}", cmd));
    match cmd {
        UserAction::List { dept, role } => {
            if ctx.dry_run {
                print_dry_run(
                    "UserService::list()",
                    &format!("{}/api.php/v1/users", ctx.config.url),
                );
                println!("  Params:");
                if let Some(d) = dept {
                    println!("    dept: {}", d);
                }
                if let Some(r) = role {
                    println!("    role: {}", r);
                }
                return;
            }
            match UserService::list(ctx, *dept, role.clone()).await {
                Ok(users) => print_user_list(&users, ctx.format.clone()),
                Err(e) => print_error(&e),
            }
        }
        UserAction::Get { id } => {
            if ctx.dry_run {
                print_dry_run(
                    "UserService::get()",
                    &format!("{}/api.php/v1/users/{}", ctx.config.url, id),
                );
                return;
            }
            match UserService::get(ctx, *id).await {
                Ok(user) => print_json(&user),
                Err(e) => print_error(&e),
            }
        }
    }
}

fn print_user_list(users: &[crate::api::User], format: OutputFormat) {
    match format {
        OutputFormat::Table => {
            println!("Users:");
            for item in users {
                println!("  [{}] {} ({})", item.id, item.realname, item.account);
            }
        }
        _ => println!(
            "{}",
            serde_json::to_string_pretty(users).unwrap_or_default()
        ),
    }
}
