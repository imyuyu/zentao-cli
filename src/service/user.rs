use crate::api::{CreateUserRequest, UpdateUserRequest, User, UserApi};
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

    pub async fn me(ctx: &AppContext) -> Result<User> {
        log(LogLevel::Info, "UserService", "me");
        let client = ctx.client();
        UserApi::me(&client).await
    }

    pub async fn create(ctx: &AppContext, req: &CreateUserRequest) -> Result<User> {
        log(LogLevel::Info, "UserService", "create");
        let client = ctx.client();
        UserApi::create(&client, req).await
    }

    pub async fn update(ctx: &AppContext, user_id: u64, req: &UpdateUserRequest) -> Result<User> {
        log(
            LogLevel::Info,
            "UserService",
            format!("update id={}", user_id),
        );
        let client = ctx.client();
        UserApi::update(&client, user_id, req).await
    }

    pub async fn delete(ctx: &AppContext, user_id: u64) -> Result<()> {
        log(
            LogLevel::Info,
            "UserService",
            format!("delete id={}", user_id),
        );
        let client = ctx.client();
        UserApi::delete(&client, user_id).await
    }
}
