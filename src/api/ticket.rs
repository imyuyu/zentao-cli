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
    /// 所属产品
    pub product: u64,
    /// 所属分类
    pub module: u64,
    /// 工单标题
    pub title: String,
    /// 工单类型：code/data/stuck/security/affair
    #[serde(rename = "type")]
    pub type_: String,
    /// 描述
    pub desc: String,
    /// 影响版本
    pub opened_build: String,
    /// 相关反馈
    pub feedback: u64,
    /// 指派给
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assigned_to: Option<String>,
    /// 指派日期
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assigned_date: Option<String>,
    /// 实际开始日期
    #[serde(skip_serializing_if = "Option::is_none")]
    pub real_started: Option<String>,
    /// 由谁开始
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_by: Option<String>,
    /// 开始日期
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_date: Option<String>,
    /// 截止日期
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deadline: Option<String>,
    /// 优先级
    pub pri: u8,
    /// 预计工时
    pub estimate: f64,
    /// 剩余工时
    pub left: f64,
    /// 状态
    pub status: String,
    /// 创建人
    pub opened_by: UserInfo,
    /// 创建时间
    pub opened_date: String,
    /// 激活次数
    pub activated_count: u64,
    /// 由谁关闭
    #[serde(skip_serializing_if = "Option::is_none")]
    pub closed_by: Option<String>,
    /// 关闭时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub closed_date: Option<String>,
    /// 关闭原因
    #[serde(skip_serializing_if = "Option::is_none")]
    pub closed_reason: Option<String>,
    /// 由谁完成
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finished_by: Option<String>,
    /// 完成日期
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finished_date: Option<String>,
    /// 由谁解决
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolved_by: Option<String>,
    /// 解决日期
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolved_date: Option<String>,
    /// 解决方案
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolution: Option<String>,
    /// 最后处理人
    pub edited_by: UserInfo,
    /// 最后修改时间
    pub edited_date: String,
    /// 是否删除
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,
    /// 关键词
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keywords: Option<String>,
    /// 重复工单
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repeat_ticket: Option<u64>,
    /// 抄送列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mailto: Option<Vec<String>>,
    /// 消耗工时
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumed: Option<f64>,
    /// 产品名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_name: Option<String>,
    /// 分类名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub module_name: Option<String>,
}

/// 用户信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInfo {
    pub id: Option<u64>,
    pub account: Option<String>,
    pub avatar: Option<String>,
    pub realname: Option<String>,
}

impl Ticket {
    pub fn web_url(&self, base_url: &str) -> String {
        format!("{}/ticket-view-{}.html", base_url, self.id)
    }
}

/// 工单列表响应
#[derive(Debug, Deserialize)]
pub struct TicketListResponse {
    pub page: u32,
    pub total: u64,
    pub limit: u32,
    pub tickets: Vec<Ticket>,
}

// ============================================================
// 请求结构体
// ============================================================

/// 创建工单请求
#[derive(Debug, Serialize)]
pub struct CreateTicketRequest {
    /// 所属产品（必填）
    pub product: u64,
    /// 标题（必填）
    pub title: String,
    /// 所属分类
    #[serde(skip_serializing_if = "Option::is_none")]
    pub module: Option<u64>,
    /// 工单类型：code/data/stuck/security/affair
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// 更新工单请求
#[derive(Debug, Serialize)]
pub struct UpdateTicketRequest {
    /// 所属产品
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product: Option<u64>,
    /// 所属分类
    #[serde(skip_serializing_if = "Option::is_none")]
    pub module: Option<u64>,
    /// 标题
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// 工单类型
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    /// 描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desc: Option<String>,
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
    /// - browse_type: 工单状态（all/wait/doing/done/finishedbyme/openedbyme/assignedtome）
    /// - order_by: 排序规则
    /// - page: 页码（默认 1）
    /// - limit: 每页数量（默认 20）
    pub async fn list(
        client: &ApiClient,
        browse_type: Option<String>,
        order_by: Option<String>,
        page: u32,
        limit: u32,
    ) -> Result<Vec<Ticket>> {
        let mut path = format!("/api.php/v1/tickets?page={}&limit={}", page, limit);
        if let Some(bt) = browse_type {
            path.push_str(&format!("&browseType={}", bt));
        }
        if let Some(ob) = order_by {
            path.push_str(&format!("&orderBy={}", ob));
        }

        let resp: TicketListResponse = client.get(&path).await?;
        Ok(resp.tickets)
    }

    /// 获取单个工单详情
    ///
    /// GET /api.php/v1/tickets/{id}
    pub async fn get(client: &ApiClient, id: u64) -> Result<Ticket> {
        let path = format!("/api.php/v1/tickets/{}", id);
        let resp: Ticket = client.get(&path).await?;
        Ok(resp)
    }

    /// 创建工单
    ///
    /// POST /api.php/v1/tickets
    pub async fn create(client: &ApiClient, req: &CreateTicketRequest) -> Result<Ticket> {
        let path = "/api.php/v1/tickets";
        let resp: Ticket = client.post(path, req).await?;
        Ok(resp)
    }

    /// 更新工单
    ///
    /// PUT /api.php/v1/tickets/{id}
    pub async fn update(client: &ApiClient, id: u64, req: &UpdateTicketRequest) -> Result<Ticket> {
        let path = format!("/api.php/v1/tickets/{}", id);
        let resp: Ticket = client.put(&path, req).await?;
        Ok(resp)
    }

    /// 删除工单
    ///
    /// DELETE /api.php/v1/tickets/{id}
    pub async fn delete(client: &ApiClient, id: u64) -> Result<()> {
        let path = format!("/api.php/v1/tickets/{}", id);
        let _: serde_json::Value = client.delete(&path).await?;
        Ok(())
    }
}
