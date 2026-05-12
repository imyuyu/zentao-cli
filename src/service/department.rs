//! ZenTao 部门(Department) Service 模块
//!
//! 提供部门的业务逻辑操作

use crate::api::department::DepartmentApi;
use crate::api::types::Department;
use crate::core::logging::{log, LogLevel};
use crate::core::AppContext;
use anyhow::Result;

pub struct DepartmentService;

impl DepartmentService {
    pub async fn list(ctx: &AppContext) -> Result<Vec<Department>> {
        log(LogLevel::Info, "DepartmentService", "list");
        let client = ctx.client();
        DepartmentApi::list(&client).await
    }

    pub async fn get(ctx: &AppContext, id: u64) -> Result<Department> {
        log(LogLevel::Info, "DepartmentService", format!("get id={}", id));
        let client = ctx.client();
        DepartmentApi::get(&client, id).await
    }
}
