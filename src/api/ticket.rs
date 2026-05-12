//! ZenTao Ticket(工单) API 模块
//!
//! 提供工单的查询操作

use anyhow::Result;
use serde::{Deserialize, Serialize};

use super::ApiClient;

// ============================================================
// 类型定义
// ============================================================

/// 工单数据结构
///
/// 对应 ZenTao 工单模块的字段
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ticket {
    /// 工单 ID
    pub id: u64,
    /// 工单标题
    pub title: String,
    /// 工单类型
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    /// 工单状态
    pub status: String,
    /// 优先级
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pri: Option<u8>,
    /// 严重程度
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity: Option<u8>,
    /// 工单描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desc: Option<String>,
    /// 所属产品 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product: Option<u64>,
    /// 所属项目 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project: Option<u64>,
    /// 关联的需求 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub story: Option<u64>,
    /// 关联的 Bug ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bug: Option<u64>,
    /// 指派给谁
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assigned_to: Option<String>,
    /// 创建者
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opened_by: Option<String>,
    /// 创建日期
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opened_date: Option<String>,
    /// 修改者
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edited_by: Option<String>,
    /// 修改日期
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edited_date: Option<String>,
    /// 关闭人
    #[serde(skip_serializing_if = "Option::is_none")]
    pub closed_by: Option<String>,
    /// 关闭日期
    #[serde(skip_serializing_if = "Option::is_none")]
    pub closed_date: Option<String>,
    /// 解决方案
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolution: Option<String>,
    /// 工单编号
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ticket_code: Option<String>,
}

impl Ticket {
    pub fn web_url(&self, base_url: &str) -> String {
        format!("{}/ticket-view-{}.html", base_url, self.id)
    }
}

/// 工单列表响应
#[derive(Debug, Deserialize)]
pub struct TicketListResponse {
    #[serde(default)]
    pub tickets: Option<Vec<Ticket>>,
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

pub struct TicketApi;

impl TicketApi {
    /// 获取工单列表
    ///
    /// GET /api.php/v1/tickets
    ///
    /// # 参数
    /// - page: 页码（默认 1）
    /// - limit: 每页数量（默认 100）
    pub async fn list(client: &ApiClient, page: u32, limit: u32) -> Result<Vec<Ticket>> {
        let path = format!(
            "/api.php/v1/tickets?page={}&limit={}",
            page, limit
        );

        let resp: TicketListResponse = client.get(&path).await?;
        Ok(resp.tickets.unwrap_or_default())
    }

    /// 获取单个工单详情
    ///
    /// GET /api.php/v1/tickets/{id}
    pub async fn get(client: &ApiClient, id: u64) -> Result<Ticket> {
        let path = format!("/api.php/v1/tickets/{}", id);
        let resp: Ticket = client.get(&path).await?;
        Ok(resp)
    }
}
