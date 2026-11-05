#![allow(non_snake_case)]
//! Execution（执行）表单字段
//!
//! 禅道创建执行时 name/project/code/begin/end 为必填

use crate::api::execution::UpdateExecutionRequest;

use super::forms::{FormField, FormFields};

/// 空串转 None
fn non_empty(s: &str) -> Option<String> {
    if s.trim().is_empty() {
        None
    } else {
        Some(s.to_string())
    }
}

/// 空串转 None 后再解析数字
fn parse_opt(s: &str) -> Option<u64> {
    non_empty(s).and_then(|v| v.parse().ok())
}

/// 执行表单字段
#[derive(Debug, Clone)]
pub struct ExecutionFormFields {
    pub name: FormField,
    pub code: FormField,
    pub status: FormField,
    pub begin: FormField,
    pub end: FormField,
    pub days: FormField,
    pub desc: FormField,
    pub PM: FormField,
    pub project: FormField,
}

impl ExecutionFormFields {
    pub fn from_execution(e: &crate::api::Execution) -> Self {
        Self {
            name: FormField::new("名称 (Name)", &e.name, "", true),
            code: FormField::new("代号 (Code)", "", "", true),
            status: FormField::new(
                "状态 (Status)",
                &e.status,
                "wait / doing / closed / suspended",
                false,
            ),
            begin: FormField::new(
                "开始日期 (Begin)",
                e.begin.as_deref().unwrap_or(""),
                "YYYY-MM-DD",
                true,
            ),
            end: FormField::new(
                "结束日期 (End)",
                e.end.as_deref().unwrap_or(""),
                "YYYY-MM-DD",
                true,
            ),
            days: FormField::new(
                "工作日 (Days)",
                &e.days.map(|v| v.to_string()).unwrap_or_default(),
                "",
                false,
            ),
            desc: FormField::new("描述 (Desc)", e.desc.as_deref().unwrap_or(""), "", false),
            PM: FormField::new("负责人 (PM)", "", "", false),
            project: FormField::readonly("项目 (Project)", &e.project.to_string()),
        }
    }

    /// Create 请求的必填项校验
    pub fn validate(&self) -> Option<String> {
        if self.name.value.trim().is_empty() {
            return Some("名称不能为空".to_string());
        }
        if self.project.value.trim().is_empty() {
            return Some("项目不能为空".to_string());
        }
        if self.code.value.trim().is_empty() {
            return Some("代号不能为空".to_string());
        }
        if self.begin.value.trim().is_empty() {
            return Some("开始日期不能为空".to_string());
        }
        if self.end.value.trim().is_empty() {
            return Some("结束日期不能为空".to_string());
        }
        None
    }

    pub fn to_update_request(&self) -> UpdateExecutionRequest {
        UpdateExecutionRequest {
            name: non_empty(&self.name.value),
            code: non_empty(&self.code.value),
            status: non_empty(&self.status.value),
            begin: non_empty(&self.begin.value),
            end: non_empty(&self.end.value),
            days: parse_opt(&self.days.value),
            desc: non_empty(&self.desc.value),
            PM: non_empty(&self.PM.value),
            QD: None,
            RD: None,
        }
    }
}

impl FormFields for ExecutionFormFields {
    fn get_fields(&self) -> Vec<&FormField> {
        vec![
            &self.name,
            &self.code,
            &self.status,
            &self.begin,
            &self.end,
            &self.days,
            &self.desc,
            &self.PM,
            &self.project,
        ]
    }
    fn get_mut_fields(&mut self) -> Vec<&mut FormField> {
        vec![
            &mut self.name,
            &mut self.code,
            &mut self.status,
            &mut self.begin,
            &mut self.end,
            &mut self.days,
            &mut self.desc,
            &mut self.PM,
            &mut self.project,
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> crate::api::Execution {
        serde_json::from_str(r#"{"id":1,"name":"迭代一","code":"s1","status":"doing","project":3}"#)
            .expect("样例 JSON 应能解析")
    }

    /// 编辑态必须能从已有实体回填关键字段
    #[test]
    fn from_execution_fills_key_field() {
        let f = ExecutionFormFields::from_execution(&sample());
        assert_eq!(f.name.value, sample().name);
    }

    /// 名称/标题清空后必须拦住提交
    #[test]
    fn rejects_empty_name() {
        let mut f = ExecutionFormFields::from_execution(&sample());
        f.name.value.clear();
        assert!(f.validate().is_some(), "空 name 应拦截");
    }
}
