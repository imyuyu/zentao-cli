//! ZenTao API 数据类型定义
//!
//! 这些结构体对应 ZenTao API 的请求和响应 JSON

use serde::{Deserialize, Serialize};

/// 从字符串或数字反序列化ID
fn deserialize_id<'de, D>(deserializer: D) -> Result<u64, D::Error>
where
    D: serde::Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum IdOrString {
        Id(u64),
        Str(String),
    }

    match IdOrString::deserialize(deserializer)? {
        IdOrString::Id(n) => Ok(n),
        IdOrString::Str(s) => {
            // 尝试解析 "case_1" 或类似格式，提取数字部分
            if let Some(num_str) = s.split('_').next_back() {
                num_str.parse::<u64>().map_err(serde::de::Error::custom)
            } else {
                s.parse::<u64>().map_err(serde::de::Error::custom)
            }
        }
    }
}

/// 从字符串、数字或空字符串反序列化可选ID
pub(crate) fn deserialize_optional_id<'de, D>(deserializer: D) -> Result<Option<u64>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum OptIdOrString {
        Null,
        Id(u64),
        Str(String),
    }

    match OptIdOrString::deserialize(deserializer)? {
        OptIdOrString::Null => Ok(None),
        OptIdOrString::Id(n) => Ok(Some(n)),
        OptIdOrString::Str(s) => {
            if s.is_empty() {
                Ok(None)
            } else if let Some(num_str) = s.split('_').next_back() {
                num_str
                    .parse::<u64>()
                    .map_err(serde::de::Error::custom)
                    .map(Some)
            } else {
                s.parse::<u64>().map_err(serde::de::Error::custom).map(Some)
            }
        }
    }
}

// ============================================================
// Story（需求）相关类型
// ============================================================

/// 需求数据结构
///
/// 对应 ZenTao 需求模块的字段
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Story {
    /// 需求 ID
    pub id: u64,
    /// 需求标题
    pub title: String,
    /// 需求描述/详细说明
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 需求状态：draft（草稿）/active（激活）/closed（关闭）
    pub status: String,
    /// 优先级：0-5
    pub pri: u8,
    /// 需求类别：feature/requirement/bug/improvement
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    /// 当前阶段：wait/plan/developed/测试中/released/closed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage: Option<String>,
    /// 所属产品 ID
    pub product: u64,
    /// 所属模块 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub module: Option<u64>,
    /// 指派给谁
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assigned_to: Option<String>,
    /// 创建者
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opened_by: Option<String>,
    /// 预估工时（小时）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimate: Option<f64>,
    /// 版本号（更新时用于乐观锁）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<u64>,
}

impl Story {
    pub fn web_url(&self, base_url: &str) -> String {
        format!("{}/story-view-{}.html", base_url, self.id)
    }
}

/// 需求列表查询参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoryListQuery {
    pub product: Option<u64>,
    pub project: Option<u64>,
    pub status: Option<String>,
    pub assigned_to: Option<String>,
}

// ============================================================
// Bug（缺陷）相关类型
// ============================================================

/// Bug 数据结构
///
/// 对应 ZenTao Bug 模块的字段
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bug {
    /// Bug ID
    pub id: u64,
    /// Bug 标题
    pub title: String,
    /// Bug 描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Bug 状态：active/resolved/closed
    pub status: String,
    /// 严重程度：1-4（1最严重）
    pub severity: u8,
    /// 优先级：0-5
    pub pri: u8,
    /// Bug 类型：codeerror/interface/design/others
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    /// 解决方案：fixed/bydesign/duplicate/fixed...
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolution: Option<String>,
    /// 重现步骤
    #[serde(skip_serializing_if = "Option::is_none")]
    pub steps: Option<String>,
    /// 所属产品 ID
    pub product: u64,
    /// 所属项目 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project: Option<u64>,
    /// 关联的需求 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub story: Option<u64>,
    /// 指派给谁
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assigned_to: Option<String>,
    /// 解决者
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolved_by: Option<String>,
    /// 解决日期
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolved_date: Option<String>,
}

impl Bug {
    pub fn web_url(&self, base_url: &str) -> String {
        format!("{}/bug-view-{}.html", base_url, self.id)
    }
}

/// Bug 列表查询参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BugListQuery {
    pub product: u64,
    pub branch: Option<u64>,
    pub status: Option<String>,
    pub assigned_to: Option<String>,
    pub browse_type: Option<String>,
}

// ============================================================
// User（用户）相关类型
// ============================================================

/// 用户数据结构
///
/// 对应 ZenTao 用户信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: u64,
    /// 登录账号
    pub account: String,
    /// 真实姓名
    pub realname: String,
    /// 邮箱
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// 部门 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dept: Option<u64>,
    /// 角色
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
}

impl User {
    pub fn web_url(&self, base_url: &str) -> String {
        format!("{}/user-view-{}.html", base_url, self.id)
    }
}

/// 用户列表查询参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserListQuery {
    pub dept: Option<u64>,
    pub role: Option<String>,
}

/// 用户简要信息（内嵌在 Feedback/Ticket 等实体中）
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

// ============================================================
// Department（部门）相关类型
// ============================================================

/// 部门数据结构
///
/// 对应 ZenTao 部门信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Department {
    /// 部门 ID
    pub id: u64,
    /// 部门名称
    pub name: String,
    /// 父部门 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<u64>,
    /// 部门排序
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<u64>,
    /// 部门路径
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
}

impl Department {
    pub fn web_url(&self, base_url: &str) -> String {
        format!("{}/dept-view-{}.html", base_url, self.id)
    }
}

// ============================================================
// Testcase（测试用例）相关类型
// ============================================================

/// 测试用例数据结构
///
/// 对应 ZenTao 测试用例模块的字段
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Testcase {
    /// 用例 ID - ZenTao有时返回字符串如"case_1"
    #[serde(deserialize_with = "deserialize_id")]
    pub id: u64,
    /// 所属产品
    pub product: u64,
    /// 所属分支
    pub branch: u64,
    /// 所属模块
    pub module: u64,
    /// 关联的需求
    pub story: u64,
    /// 需求版本
    pub story_version: u64,
    /// 用例标题
    pub title: String,
    /// 前置条件
    pub precondition: String,
    /// 优先级：0-5
    pub pri: u8,
    /// 用例类型：feature/performance/config/install/security/interface/unit/other
    #[serde(rename = "type")]
    pub type_: String,
    /// 适用阶段：unitest/feature/intergrate/system/smoke/bvt
    pub stage: String,
    /// 用例状态：wait/normal/blocked/investigate
    pub status: String,
    /// 测试步骤列表
    pub steps: Vec<TestcaseStep>,
    /// 来自于Bug
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_bug: Option<u64>,
    /// 来自于用例
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_case_id: Option<u64>,
    /// 创建人
    pub opened_by: String,
    /// 创建时间
    pub opened_date: String,
    /// 版本号
    pub version: u64,
    /// 是否删除
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,
}

/// 测试用例步骤
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestcaseStep {
    /// 步骤 ID
    pub id: Option<u64>,
    /// 步骤描述
    pub desc: String,
    /// 期望结果
    pub expect: String,
}

/// 测试用例列表查询参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestcaseListQuery {
    pub product: Option<u64>,
    pub project: Option<u64>,
    pub type_: Option<String>,
    pub status: Option<String>,
    pub severity: Option<u8>,
}

// ============================================================
// Execution（执行/里程碑）相关类型
// ============================================================

/// 执行/里程碑数据结构
///
/// 对应 ZenTao 执行模块的字段
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Execution {
    /// 执行 ID
    pub id: u64,
    /// 执行名称
    pub name: String,
    /// 所属项目 ID
    pub project: u64,
    /// 执行状态：wait/doing/closed/suspended
    pub status: String,
    /// 执行类型：iteration/milestone
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    /// 开始日期
    #[serde(skip_serializing_if = "Option::is_none")]
    pub begin: Option<String>,
    /// 结束日期
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<String>,
    /// 预计工期（天）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub days: Option<u64>,
    /// 执行描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desc: Option<String>,
    /// 创建者
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opened_by: Option<String>,
    /// 创建日期
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opened_date: Option<String>,
}

impl Execution {
    pub fn web_url(&self, base_url: &str) -> String {
        format!("{}/execution-view-{}.html", base_url, self.id)
    }
}

// ============================================================
// Build（版本）相关类型
// ============================================================

/// Build 数据结构
///
/// 对应 ZenTao 版本模块的字段
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Build {
    /// 版本 ID
    pub id: u64,
    /// 版本名称
    pub name: String,
    /// 所属产品 ID
    pub product: u64,
    /// 所属项目 ID
    pub project: u64,
    /// 所属分支/平台
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch: Option<u64>,
    /// SCM 路径
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scm_path: Option<String>,
    /// CI 名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ci: Option<String>,
    /// 包路径
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pkg: Option<String>,
    /// 文件大小（字节）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<String>,
    /// 生成时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generated_at: Option<String>,
    /// 是否已删除
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted: Option<String>,
    /// 编辑者
    #[serde(skip_serializing_if = "Option::is_none")]
    pub editor: Option<String>,
    /// 创建者
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    /// 创建日期
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<String>,
    /// 最后编辑者
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_edited_by: Option<String>,
    /// 最后编辑日期
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_edited_date: Option<String>,
    /// 关联的用例数
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumed_cards: Option<String>,
    /// 关联的需求数
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stories: Option<String>,
    /// 关联的 Bug 数
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bugs: Option<String>,
}

/// 版本列表响应格式（用于 /projects/{id}/builds 接口）
#[derive(Debug, Deserialize)]
pub struct BuildListResponse {
    pub total: u64,
    pub builds: Vec<Build>,
}

impl Build {
    pub fn web_url(&self, base_url: &str) -> String {
        format!("{}/build-view-{}.html", base_url, self.id)
    }
}

// ============================================================
// API 响应包装类型
// ============================================================

/// 通用 API 响应格式
///
/// ZenTao API 返回格式：{"status": "success", "data": {...}}
/// 或：{"status": "fail", "msg": "错误信息"}
#[derive(Debug, Deserialize)]
pub struct ApiResponse<T> {
    pub status: String,
    /// 响应数据（成功时存在）
    #[serde(default)]
    pub data: Option<T>,
    /// 错误信息（失败时存在）
    #[serde(default)]
    pub msg: Option<String>,
}

/// Token 响应格式
///
/// POST /api.php/v1/tokens 的响应
#[derive(Debug, Deserialize)]
pub struct TokenResponse {
    pub token: Option<String>,
    #[serde(rename = "status")]
    pub status: Option<String>,
}
