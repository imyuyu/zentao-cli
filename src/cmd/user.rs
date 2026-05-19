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
    #[command(name = "me")]
    Me,
    #[command(name = "create")]
    Create {
        #[arg(long)]
        account: String,
        #[arg(long)]
        password: String,
        #[arg(long)]
        realname: String,
        #[arg(long)]
        role: Option<String>,
        #[arg(long)]
        dept: Option<u64>,
        #[arg(long)]
        mobile: Option<String>,
        #[arg(long)]
        email: Option<String>,
        #[arg(long)]
        phone: Option<String>,
    },
    #[command(name = "update")]
    Update {
        id: u64,
        #[arg(long)]
        dept: Option<u64>,
        #[arg(long)]
        role: Option<String>,
        #[arg(long)]
        mobile: Option<String>,
        #[arg(long)]
        realname: Option<String>,
        #[arg(long)]
        email: Option<String>,
        #[arg(long)]
        phone: Option<String>,
    },
    #[command(name = "delete")]
    Delete { id: u64 },
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
        UserAction::Me => {
            if ctx.dry_run {
                print_dry_run(
                    "UserService::me()",
                    &format!("{}/api.php/v1/user", ctx.config.url),
                );
                return;
            }
            match UserService::me(ctx).await {
                Ok(user) => print_json(&user),
                Err(e) => print_error(&e),
            }
        }
        UserAction::Create { account, password, realname, role, dept, mobile, email, phone } => {
            if ctx.dry_run {
                print_dry_run(
                    "UserService::create()",
                    &format!("{}/api.php/v1/users", ctx.config.url),
                );
                println!("  Params:");
                println!("    account: {}", account);
                println!("    realname: {}", realname);
                if let Some(r) = role { println!("    role: {}", r); }
                if let Some(d) = dept { println!("    dept: {}", d); }
                if let Some(m) = mobile { println!("    mobile: {}", m); }
                if let Some(e) = email { println!("    email: {}", e); }
                if let Some(p) = phone { println!("    phone: {}", p); }
                return;
            }
            let req = crate::api::CreateUserRequest {
                account: account.clone(),
                password: password.clone(),
                realname: realname.clone(),
                role: role.clone(),
                dept: *dept,
                mobile: mobile.clone(),
                email: email.clone(),
                phone: phone.clone(),
            };
            match UserService::create(ctx, &req).await {
                Ok(user) => print_json(&user),
                Err(e) => print_error(&e),
            }
        }
        UserAction::Update { id, dept, role, mobile, realname, email, phone } => {
            if ctx.dry_run {
                print_dry_run(
                    "UserService::update()",
                    &format!("{}/api.php/v1/users/{}", ctx.config.url, id),
                );
                println!("  Params:");
                if let Some(d) = dept { println!("    dept: {}", d); }
                if let Some(r) = role { println!("    role: {}", r); }
                if let Some(m) = mobile { println!("    mobile: {}", m); }
                if let Some(rn) = realname { println!("    realname: {}", rn); }
                if let Some(e) = email { println!("    email: {}", e); }
                if let Some(p) = phone { println!("    phone: {}", p); }
                return;
            }
            let req = crate::api::UpdateUserRequest {
                dept: *dept,
                role: role.clone(),
                mobile: mobile.clone(),
                realname: realname.clone(),
                email: email.clone(),
                phone: phone.clone(),
            };
            match UserService::update(ctx, *id, &req).await {
                Ok(user) => print_json(&user),
                Err(e) => print_error(&e),
            }
        }
        UserAction::Delete { id } => {
            if ctx.dry_run {
                print_dry_run(
                    "UserService::delete()",
                    &format!("{}/api.php/v1/users/{}", ctx.config.url, id),
                );
                return;
            }
            match UserService::delete(ctx, *id).await {
                Ok(()) => println!("User {} deleted successfully", id),
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
