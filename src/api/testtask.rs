//! ZenTao Testtask(测试单) API 模块
//!
//! 提供测试单的查询操作

use anyhow::Result;
use serde::{Deserialize, Serialize};

use super::ApiClient;

// ============================================================
// 类型定义
// ============================================================

/// 测试单数据结构
///
/// 对应 ZenTao 测试单模块的字段
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Testtask {
    /// 测试单 ID
    pub id: u64,
    /// 测试单名称
    pub name: String,
    /// 所属项目 ID
    pub project: u64,
    /// 所属执行/迭代 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution: Option<u64>,
    /// 测试单状态：wait/doing/done/closed
    pub status: String,
    /// 测试单类型
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    /// 所属产品 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product: Option<u64>,
    /// 所属产品名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub productName: Option<String>,
    /// 关联执行名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub executionName: Option<String>,
    /// 关联版本/构建名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buildName: Option<String>,
    /// 所属分支
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch: Option<u64>,
    /// 关联的版本/构建 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build: Option<u64>,
    /// 开始日期
    #[serde(skip_serializing_if = "Option::is_none")]
    pub begin: Option<String>,
    /// 结束日期
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<String>,
    /// 负责人
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assigned_to: Option<String>,
    /// 负责人名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    /// 创建者
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opened_by: Option<String>,
    /// 创建日期
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opened_date: Option<String>,
    /// 实际开始日期
    #[serde(skip_serializing_if = "Option::is_none")]
    pub real_begin: Option<String>,
    /// 实际结束日期
    #[serde(skip_serializing_if = "Option::is_none")]
    pub real_end: Option<String>,
    /// 测试单描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desc: Option<String>,
    /// 关联的用例数
    #[serde(skip_serializing_if = "Option::is_none")]
    pub case_count: Option<String>,
    /// 已执行用例数
    #[serde(skip_serializing_if = "Option::is_none")]
    pub passed_count: Option<String>,
    /// 失败用例数
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_count: Option<String>,
    /// 阻塞用例数
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blocked_count: Option<String>,
}

impl Testtask {
    pub fn web_url(&self, base_url: &str) -> String {
        format!("{}/testtask-view-{}.html", base_url, self.id)
    }
}

/// 测试单列表响应
#[derive(Debug, Deserialize)]
pub struct TesttaskListResponse {
    #[serde(default)]
    pub testtasks: Option<Vec<Testtask>>,
    #[serde(default)]
    pub total: Option<u64>,
    #[serde(default)]
    pub page: Option<u32>,
    #[serde(default)]
    pub limit: Option<u32>,
}

// ============================================================
// API
// ============================================================

pub struct TesttaskApi;

impl TesttaskApi {
    /// 获取测试单列表
    ///
    /// GET /api.php/v1/testtasks
    ///
    /// # 参数
    /// - client: API 客户端
    /// - page: 页码（默认 1）
    /// - limit: 每页数量（默认 100）
    /// - order: 排序字段（如 created_date_DESC）
    /// - product: 按产品 ID 筛选
    /// - branch: 按分支筛选
    pub async fn list(
        client: &ApiClient,
        page: u32,
        limit: u32,
        order: Option<String>,
        product: Option<u64>,
        branch: Option<u64>,
    ) -> Result<Vec<Testtask>> {
        let mut path = format!("/api.php/v1/testtasks?page={}&limit={}", page, limit);
        if let Some(ref o) = order {
            path.push_str(&format!("&order={}", o));
        }
        if let Some(p) = product {
            path.push_str(&format!("&product={}", p));
        }
        if let Some(b) = branch {
            path.push_str(&format!("&branch={}", b));
        }

        let resp: TesttaskListResponse = client.get(&path).await?;
        Ok(resp.testtasks.unwrap_or_default())
    }

    /// 获取项目下的测试单列表
    ///
    /// GET /api.php/v1/projects/{projectId}/testtasks
    pub async fn list_by_project(
        client: &ApiClient,
        project: u64,
        page: u32,
        limit: u32,
    ) -> Result<Vec<Testtask>> {
        let path = format!(
            "/api.php/v1/projects/{}/testtasks?page={}&limit={}",
            project, page, limit
        );

        let resp: TesttaskListResponse = client.get(&path).await?;
        Ok(resp.testtasks.unwrap_or_default())
    }

    /// 获取单个测试单详情
    ///
    /// GET /api.php/v1/testtasks/{id}
    pub async fn get(client: &ApiClient, id: u64) -> Result<Testtask> {
        let path = format!("/api.php/v1/testtasks/{}", id);
        let resp: Testtask = client.get(&path).await?;
        Ok(resp)
    }
}
