//! ZenTao Feedback(反馈) API 模块
//!
//! 提供反馈的查询操作

use anyhow::Result;
use serde::{Deserialize, Serialize};

use super::ApiClient;

// ============================================================
// 类型定义
// ============================================================

/// 反馈数据结构
///
/// 对应 ZenTao 反馈模块的字段
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Feedback {
    /// 反馈 ID
    pub id: u64,
    /// 反馈标题
    pub title: String,
    /// 反馈类型
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    /// 反馈状态：open/assigned/closed
    pub status: String,
    /// 优先级
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pri: Option<u8>,
    /// 反馈描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desc: Option<String>,
    /// 所属产品 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product: Option<u64>,
    /// 所属项目 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project: Option<u64>,
    /// 指派给谁
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assigned_to: Option<String>,
    /// 创建者
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opened_by: Option<String>,
    /// 创建日期
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opened_date: Option<String>,
    /// 处理人
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processed_by: Option<String>,
    /// 处理日期
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processed_date: Option<String>,
    /// 关闭人
    #[serde(skip_serializing_if = "Option::is_none")]
    pub closed_by: Option<String>,
    /// 关闭日期
    #[serde(skip_serializing_if = "Option::is_none")]
    pub closed_date: Option<String>,
    /// 反馈来源
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    /// 联系信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact: Option<String>,
}

impl Feedback {
    pub fn web_url(&self, base_url: &str) -> String {
        format!("{}/feedback-view-{}.html", base_url, self.id)
    }
}

/// 反馈列表响应
#[derive(Debug, Deserialize)]
pub struct FeedbackListResponse {
    #[serde(default)]
    pub feedbacks: Option<Vec<Feedback>>,
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

pub struct FeedbackApi;

impl FeedbackApi {
    /// 获取反馈列表
    ///
    /// GET /api.php/v1/feedbacks
    ///
    /// # 参数
    /// - page: 页码（默认 1）
    /// - limit: 每页数量（默认 100）
    pub async fn list(client: &ApiClient, page: u32, limit: u32) -> Result<Vec<Feedback>> {
        let path = format!(
            "/api.php/v1/feedbacks?page={}&limit={}",
            page, limit
        );

        let resp: FeedbackListResponse = client.get(&path).await?;
        Ok(resp.feedbacks.unwrap_or_default())
    }

    /// 获取单个反馈详情
    ///
    /// GET /api.php/v1/feedbacks/{id}
    pub async fn get(client: &ApiClient, id: u64) -> Result<Feedback> {
        let path = format!("/api.php/v1/feedbacks/{}", id);
        let resp: Feedback = client.get(&path).await?;
        Ok(resp)
    }
}
