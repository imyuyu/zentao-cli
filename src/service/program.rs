//! Program(项目集) Service 模块
//!
//! 提供项目集的业务逻辑操作

use crate::api::{Program, ProgramApi};
use crate::core::logging::{log, LogLevel};
use crate::core::AppContext;
use anyhow::Result;

pub struct ProgramService;

impl ProgramService {
    /// 获取项目集列表
    pub async fn list(ctx: &AppContext) -> Result<Vec<Program>> {
        log(LogLevel::Info, "ProgramService", "list");
        let client = ctx.client();
        ProgramApi::list(&client).await
    }

    /// 获取单个项目集详情
    pub async fn get(ctx: &AppContext, id: u64) -> Result<Program> {
        log(LogLevel::Info, "ProgramService", format!("get id={}", id));
        let client = ctx.client();
        ProgramApi::get(&client, id).await
    }

    /// 获取项目集名称
    pub async fn get_name(ctx: &AppContext, id: u64) -> Result<String> {
        let program = Self::get(ctx, id).await?;
        Ok(program.name)
    }
}
