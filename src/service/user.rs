use crate::api::{User, UserApi};
use crate::core::logging::{log, LogLevel};
use crate::core::AppContext;
use anyhow::Result;

pub struct UserService;

impl UserService {
    pub async fn list(
        ctx: &AppContext,
        dept: Option<u64>,
        role: Option<String>,
    ) -> Result<Vec<User>> {
        log(LogLevel::Info, "UserService", "list");
        let client = ctx.client();
        UserApi::list(&client, dept, role).await
    }

    pub async fn get(ctx: &AppContext, id: u64) -> Result<User> {
        log(LogLevel::Info, "UserService", format!("get id={}", id));
        let client = ctx.client();
        UserApi::get(&client, id).await
    }
}
