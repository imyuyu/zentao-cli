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
    /// 所属产品
    pub product: u64,
    /// 所属分类
    pub module: u64,
    /// 反馈标题
    pub title: String,
    /// 反馈类型：story/task/bug/todo/advice/issue/risk/opportunity
    #[serde(rename = "type")]
    pub type_: String,
    /// 处理结果：unclosed/all/public/tostory/totask/tobug/totodo/review/assigntome
    pub solution: String,
    /// 反馈描述
    pub desc: String,
    /// 状态：wait/commenting/replied/closed
    pub status: String,
    /// 子状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_status: Option<String>,
    /// 公开
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public: Option<u8>,
    /// 通知
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notify: Option<u8>,
    /// 通知邮箱
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notify_email: Option<String>,
    /// 点赞人
    #[serde(skip_serializing_if = "Option::is_none")]
    pub likes: Option<String>,
    /// 转化结果
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<u64>,
    /// FAQ
    #[serde(skip_serializing_if = "Option::is_none")]
    pub faq: Option<u64>,
    /// 创建人
    pub opened_by: UserInfo,
    /// 创建时间
    pub opened_date: String,
    /// 评审人
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reviewed_by: Option<String>,
    /// 评审时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reviewed_date: Option<String>,
    /// 处理人
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processed_by: Option<String>,
    /// 处理时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processed_date: Option<String>,
    /// 关闭人
    #[serde(skip_serializing_if = "Option::is_none")]
    pub closed_by: Option<String>,
    /// 关闭时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub closed_date: Option<String>,
    /// 关闭原因
    #[serde(skip_serializing_if = "Option::is_none")]
    pub closed_reason: Option<String>,
    /// 最后处理人
    pub edited_by: UserInfo,
    /// 最后修改时间
    pub edited_date: String,
    /// 指派给
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assigned_to: Option<String>,
    /// 指派时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assigned_date: Option<String>,
    /// 反馈者
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feedback_by: Option<String>,
    /// 抄送给
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mailto: Option<Vec<String>>,
    /// 已删除
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,
    /// 点赞总数
    #[serde(skip_serializing_if = "Option::is_none")]
    pub likes_count: Option<u64>,
    /// 附件列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<Vec<FileInfo>>,
    /// 产品名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_name: Option<String>,
}

/// 用户信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInfo {
    pub id: Option<u64>,
    pub account: Option<String>,
    pub avatar: Option<String>,
    pub realname: Option<String>,
}

impl UserInfo {
    pub fn account_string(&self) -> String {
        self.account.clone().unwrap_or_default()
    }
}

/// 文件信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileInfo {
    pub id: Option<u64>,
    pub name: Option<String>,
    pub path: Option<String>,
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
// 请求结构体
// ============================================================

/// 创建反馈请求
#[derive(Debug, Serialize)]
pub struct CreateFeedbackRequest {
    /// 所属产品（必填）
    pub product: u64,
    /// 标题（必填）
    pub title: String,
    /// 所属分类
    #[serde(skip_serializing_if = "Option::is_none")]
    pub module: Option<u64>,
    /// 类型
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    /// 描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desc: Option<String>,
    /// 公开
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public: Option<u8>,
    /// 通知
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notify: Option<u8>,
    /// 通知邮箱
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notify_email: Option<String>,
    /// 反馈者
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feedback_by: Option<String>,
}

/// 指派反馈请求
#[derive(Debug, Serialize)]
pub struct AssignFeedbackRequest {
    /// 指派给
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assigned_to: Option<String>,
    /// 备注
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    /// 抄送给
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mailto: Option<String>,
}

/// 关闭反馈请求
#[derive(Debug, Serialize)]
pub struct CloseFeedbackRequest {
    /// 关闭原因：commented/repeat/refuse
    #[serde(skip_serializing_if = "Option::is_none")]
    pub closed_reason: Option<String>,
    /// 备注
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
}

/// 更新反馈请求
#[derive(Debug, Serialize)]
pub struct UpdateFeedbackRequest {
    /// 所属产品
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product: Option<u64>,
    /// 所属分类
    #[serde(skip_serializing_if = "Option::is_none")]
    pub module: Option<u64>,
    /// 标题
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// 类型
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    /// 描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desc: Option<String>,
    /// 公开
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public: Option<u8>,
    /// 通知
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notify: Option<u8>,
    /// 通知邮箱
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notify_email: Option<String>,
    /// 反馈者
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feedback_by: Option<String>,
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
    /// - limit: 每页数量（默认 20）
    pub async fn list(client: &ApiClient, page: u32, limit: u32) -> Result<Vec<Feedback>> {
        let path = format!("/api.php/v1/feedbacks?page={}&limit={}", page, limit);

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

    /// 创建反馈
    ///
    /// POST /api.php/v1/feedbacks
    pub async fn create(client: &ApiClient, req: &CreateFeedbackRequest) -> Result<Feedback> {
        let path = "/api.php/v1/feedbacks";
        let resp: Feedback = client.post(path, req).await?;
        Ok(resp)
    }

    /// 指派反馈
    ///
    /// POST /api.php/v1/feedbacks/{id}/assign
    pub async fn assign(
        client: &ApiClient,
        id: u64,
        req: &AssignFeedbackRequest,
    ) -> Result<Feedback> {
        let path = format!("/api.php/v1/feedbacks/{}/assign", id);
        let resp: Feedback = client.post(&path, req).await?;
        Ok(resp)
    }

    /// 关闭反馈
    ///
    /// POST /api.php/v1/feedbacks/{id}/close
    pub async fn close(
        client: &ApiClient,
        id: u64,
        req: &CloseFeedbackRequest,
    ) -> Result<Feedback> {
        let path = format!("/api.php/v1/feedbacks/{}/close", id);
        let resp: Feedback = client.post(&path, req).await?;
        Ok(resp)
    }

    /// 更新反馈
    ///
    /// PUT /api.php/v1/feedbacks/{id}
    pub async fn update(
        client: &ApiClient,
        id: u64,
        req: &UpdateFeedbackRequest,
    ) -> Result<Feedback> {
        let path = format!("/api.php/v1/feedbacks/{}", id);
        let resp: Feedback = client.put(&path, req).await?;
        Ok(resp)
    }

    /// 删除反馈
    ///
    /// DELETE /api.php/v1/feedbacks/{id}
    pub async fn delete(client: &ApiClient, id: u64) -> Result<()> {
        let path = format!("/api.php/v1/feedbacks/{}", id);
        let _: serde_json::Value = client.delete(&path).await?;
        Ok(())
    }
}
