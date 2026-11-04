#![allow(non_snake_case)]
//! Execution（执行）表单字段
//!
//! 禅道创建执行时 name/project/code/begin/end 为必填

use crate::api::execution::{CreateExecutionRequest, UpdateExecutionRequest};

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
    pub fn new(project_id: Option<u64>) -> Self {
        Self {
            name: FormField::new("名称 (Name)", "", "请输入执行名称", true),
            code: FormField::new("代号 (Code)", "", "请输入执行代号", true),
            status: FormField::new(
                "状态 (Status)",
                "wait",
                "wait / doing / closed / suspended",
                false,
            ),
            begin: FormField::new("开始日期 (Begin)", "", "YYYY-MM-DD", true),
            end: FormField::new("结束日期 (End)", "", "YYYY-MM-DD", true),
            days: FormField::new("工作日 (Days)", "", "可用工作日", false),
            desc: FormField::new("描述 (Desc)", "", "请输入描述", false),
            PM: FormField::new("负责人 (PM)", "", "迭代负责人", false),
            project: FormField::new(
                "项目 (Project)",
                &project_id.map(|v| v.to_string()).unwrap_or_default(),
                "项目 ID（必填）",
                true,
            ),
        }
    }

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

    pub fn to_create_request(&self) -> CreateExecutionRequest {
        CreateExecutionRequest {
            name: self.name.value.clone(),
            project: self.project.value.trim().parse().unwrap_or_default(),
            code: self.code.value.clone(),
            begin: self.begin.value.clone(),
            end: self.end.value.clone(),
            type_: None,
            days: parse_opt(&self.days.value),
            lifetime: None,
            PO: None,
            PM: non_empty(&self.PM.value),
            QD: None,
            RD: None,
            teamMembers: None,
            desc: non_empty(&self.desc.value),
            acl: None,
            whitelist: None,
        }
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

    #[test]
    fn execution_form_rejects_empty_name() {
        let mut f = ExecutionFormFields::new(Some(1));
        f.code.value = "c1".into();
        f.begin.value = "2024-01-01".into();
        f.end.value = "2024-01-14".into();
        assert!(f.validate().is_some(), "名称为空应拦截");

        f.name.value = "Sprint 1".into();
        assert!(f.validate().is_none(), "必填齐全应放行");
    }

    #[test]
    fn execution_form_empty_fields_become_none() {
        let mut f = ExecutionFormFields::new(Some(1));
        f.name.value = "Sprint 1".into();
        f.days.value = "5".into();
        let req = f.to_update_request();
        assert_eq!(req.name, Some("Sprint 1".into()));
        assert_eq!(req.days, Some(5));
        assert!(req.desc.is_none());
        assert!(req.PM.is_none());
    }
}
