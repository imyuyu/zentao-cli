//! TUI 表单模块
//!
//! 提供表单字段数据结构和通用表单组件

use crate::api::productplan::{CreateProductPlanRequest, UpdateProductPlanRequest};
use crate::api::program::{CreateProgramRequest, UpdateProgramRequest};
use crate::api::release::{CreateReleaseRequest, UpdateReleaseRequest};

/// 表单字段
#[derive(Debug, Clone)]
pub struct FormField {
    pub label: String,
    pub value: String,
    pub placeholder: String,
    pub editable: bool,
    pub is_required: bool,
}

/// 表单字段容器 —— 让 TUI 的输入处理不必按 AppState 逐个 match
pub trait FormFields {
    fn get_fields(&self) -> Vec<&FormField>;
    fn get_mut_fields(&mut self) -> Vec<&mut FormField>;
}

impl FormField {
    pub fn new(label: &str, value: &str, placeholder: &str, is_required: bool) -> Self {
        Self {
            label: label.to_string(),
            value: value.to_string(),
            placeholder: placeholder.to_string(),
            editable: true,
            is_required,
        }
    }

    pub fn readonly(label: &str, value: &str) -> Self {
        Self {
            label: label.to_string(),
            value: value.to_string(),
            placeholder: String::new(),
            editable: false,
            is_required: false,
        }
    }
}

/// Program 表单字段
#[derive(Debug, Clone)]
pub struct ProgramFormFields {
    pub name: FormField,
    pub code: FormField,
    pub desc: FormField,
    pub begin: FormField,
    pub end: FormField,
}

impl ProgramFormFields {
    pub fn new() -> Self {
        Self {
            name: FormField::new("名称 (Name)", "", "请输入项目集名称", true),
            code: FormField::new("代号 (Code)", "", "请输入项目集代号", true),
            desc: FormField::new("描述 (Desc)", "", "请输入描述", false),
            begin: FormField::new("开始日期 (Begin)", "", "YYYY-MM-DD", false),
            end: FormField::new("结束日期 (End)", "", "YYYY-MM-DD", false),
        }
    }

    pub fn from_program(name: &str, code: &str, desc: &str, begin: &str, end: &str) -> Self {
        Self {
            name: FormField::new("名称 (Name)", name, "请输入项目集名称", true),
            code: FormField::new("代号 (Code)", code, "请输入项目集代号", true),
            desc: FormField::new("描述 (Desc)", desc, "请输入描述", false),
            begin: FormField::new("开始日期 (Begin)", begin, "YYYY-MM-DD", false),
            end: FormField::new("结束日期 (End)", end, "YYYY-MM-DD", false),
        }
    }

    pub fn to_create_request(&self) -> CreateProgramRequest {
        CreateProgramRequest {
            name: self.name.value.clone(),
            code: Some(self.code.value.clone()),
            type_: None,
            desc: Some(self.desc.value.clone()),
            parent: None,
            PM: None,
            budget: None,
            budgetUnit: None,
            begin: if self.begin.value.is_empty() {
                None
            } else {
                Some(self.begin.value.clone())
            },
            end: if self.end.value.is_empty() {
                None
            } else {
                Some(self.end.value.clone())
            },
            acl: None,
            whitelist: None,
        }
    }

    pub fn to_update_request(&self) -> UpdateProgramRequest {
        UpdateProgramRequest {
            name: Some(self.name.value.clone()),
            desc: Some(self.desc.value.clone()),
            PM: None,
            budget: None,
            budgetUnit: None,
            begin: if self.begin.value.is_empty() {
                None
            } else {
                Some(self.begin.value.clone())
            },
            end: if self.end.value.is_empty() {
                None
            } else {
                Some(self.end.value.clone())
            },
            acl: None,
            whitelist: None,
            parent: None,
        }
    }

    pub fn get_fields(&self) -> Vec<&FormField> {
        vec![&self.name, &self.code, &self.desc, &self.begin, &self.end]
    }

    pub fn get_mut_fields(&mut self) -> Vec<&mut FormField> {
        vec![
            &mut self.name,
            &mut self.code,
            &mut self.desc,
            &mut self.begin,
            &mut self.end,
        ]
    }

    pub fn validate(&self) -> Option<String> {
        if self.name.value.trim().is_empty() {
            return Some("名称不能为空".to_string());
        }
        if self.code.value.trim().is_empty() {
            return Some("代号不能为空".to_string());
        }
        None
    }
}
impl FormFields for ProgramFormFields {
    fn get_fields(&self) -> Vec<&FormField> {
        self.get_fields()
    }
    fn get_mut_fields(&mut self) -> Vec<&mut FormField> {
        self.get_mut_fields()
    }
}

/// ProductPlan 表单字段
#[derive(Debug, Clone)]
pub struct ProductPlanFormFields {
    pub title: FormField,
    pub desc: FormField,
    pub start: FormField,
    pub end: FormField,
}

impl ProductPlanFormFields {
    pub fn new() -> Self {
        Self {
            title: FormField::new("标题 (Title)", "", "请输入计划标题", true),
            desc: FormField::new("描述 (Desc)", "", "请输入描述", false),
            start: FormField::new("开始日期 (Start)", "", "YYYY-MM-DD", false),
            end: FormField::new("结束日期 (End)", "", "YYYY-MM-DD", false),
        }
    }

    pub fn from_plan(title: &str, desc: &str, start: &str, end: &str) -> Self {
        Self {
            title: FormField::new("标题 (Title)", title, "请输入计划标题", true),
            desc: FormField::new("描述 (Desc)", desc, "请输入描述", false),
            start: FormField::new("开始日期 (Start)", start, "YYYY-MM-DD", false),
            end: FormField::new("结束日期 (End)", end, "YYYY-MM-DD", false),
        }
    }

    pub fn to_create_request(&self, product_id: u64) -> CreateProductPlanRequest {
        CreateProductPlanRequest {
            title: self.title.value.clone(),
            product: Some(product_id),
            desc: Some(self.desc.value.clone()),
            begin: if self.start.value.is_empty() {
                None
            } else {
                Some(self.start.value.clone())
            },
            end: if self.end.value.is_empty() {
                None
            } else {
                Some(self.end.value.clone())
            },
            branch: None,
            code: None,
            type_: None,
            parent: None,
        }
    }

    pub fn to_update_request(&self) -> UpdateProductPlanRequest {
        UpdateProductPlanRequest {
            name: Some(self.title.value.clone()),
            desc: Some(self.desc.value.clone()),
            begin: if self.start.value.is_empty() {
                None
            } else {
                Some(self.start.value.clone())
            },
            end: if self.end.value.is_empty() {
                None
            } else {
                Some(self.end.value.clone())
            },
            status: None,
            owner: None,
        }
    }

    pub fn get_fields(&self) -> Vec<&FormField> {
        vec![&self.title, &self.desc, &self.start, &self.end]
    }

    pub fn get_mut_fields(&mut self) -> Vec<&mut FormField> {
        vec![
            &mut self.title,
            &mut self.desc,
            &mut self.start,
            &mut self.end,
        ]
    }

    pub fn validate(&self) -> Option<String> {
        if self.title.value.trim().is_empty() {
            return Some("标题不能为空".to_string());
        }
        None
    }
}
impl FormFields for ProductPlanFormFields {
    fn get_fields(&self) -> Vec<&FormField> {
        self.get_fields()
    }
    fn get_mut_fields(&mut self) -> Vec<&mut FormField> {
        self.get_mut_fields()
    }
}

/// Release 表单字段
#[derive(Debug, Clone)]
pub struct ReleaseFormFields {
    pub name: FormField,
    pub build: FormField,
    pub date: FormField,
    pub status: FormField,
    pub remarks: FormField,
}

impl ReleaseFormFields {
    pub fn new() -> Self {
        Self {
            name: FormField::new("名称 (Name)", "", "请输入发布名称", true),
            build: FormField::new("Build", "", "请输入Build ID", false),
            date: FormField::new("发布日期 (Date)", "", "YYYY-MM-DD", false),
            status: FormField::new("状态 (Status)", "normal", "normal / closed", false),
            remarks: FormField::new("备注 (Remarks)", "", "请输入备注", false),
        }
    }

    pub fn from_release(name: &str, build: &str, date: &str, status: &str, remarks: &str) -> Self {
        Self {
            name: FormField::new("名称 (Name)", name, "请输入发布名称", true),
            build: FormField::new("Build", build, "请输入Build ID", false),
            date: FormField::new("发布日期 (Date)", date, "YYYY-MM-DD", false),
            status: FormField::new("状态 (Status)", status, "normal / closed", false),
            remarks: FormField::new("备注 (Remarks)", remarks, "请输入备注", false),
        }
    }

    pub fn to_create_request(&self, product_id: Option<u64>) -> CreateReleaseRequest {
        CreateReleaseRequest {
            name: self.name.value.clone(),
            product: product_id,
            build: if self.build.value.is_empty() {
                None
            } else {
                self.build.value.parse().ok()
            },
            released_date: if self.date.value.is_empty() {
                None
            } else {
                Some(self.date.value.clone())
            },
            date: if self.date.value.is_empty() {
                None
            } else {
                Some(self.date.value.clone())
            },
            desc: if self.remarks.value.is_empty() {
                None
            } else {
                Some(self.remarks.value.clone())
            },
            project: None,
        }
    }

    pub fn to_update_request(&self) -> UpdateReleaseRequest {
        UpdateReleaseRequest {
            name: Some(self.name.value.clone()),
            build: if self.build.value.is_empty() {
                None
            } else {
                self.build.value.parse().ok()
            },
            date: if self.date.value.is_empty() {
                None
            } else {
                Some(self.date.value.clone())
            },
            desc: if self.remarks.value.is_empty() {
                None
            } else {
                Some(self.remarks.value.clone())
            },
            status: Some(self.status.value.clone()),
        }
    }

    pub fn get_fields(&self) -> Vec<&FormField> {
        vec![
            &self.name,
            &self.build,
            &self.date,
            &self.status,
            &self.remarks,
        ]
    }

    pub fn get_mut_fields(&mut self) -> Vec<&mut FormField> {
        vec![
            &mut self.name,
            &mut self.build,
            &mut self.date,
            &mut self.status,
            &mut self.remarks,
        ]
    }

    pub fn validate(&self) -> Option<String> {
        if self.name.value.trim().is_empty() {
            return Some("名称不能为空".to_string());
        }
        None
    }
}
impl FormFields for ReleaseFormFields {
    fn get_fields(&self) -> Vec<&FormField> {
        self.get_fields()
    }
    fn get_mut_fields(&mut self) -> Vec<&mut FormField> {
        self.get_mut_fields()
    }
}

/// Bug 表单字段
///
/// 禅道提 Bug 的必填项（缺 project/execution/module 会返回 201 但不落库）
#[derive(Debug, Clone)]
pub struct BugFormFields {
    pub title: FormField,
    pub severity: FormField,
    pub pri: FormField,
    pub type_: FormField,
    pub project: FormField,
    pub execution: FormField,
    pub module: FormField,
    pub steps: FormField,
}

impl BugFormFields {
    pub fn new(project_id: Option<u64>) -> Self {
        Self {
            title: FormField::new("标题 (Title)", "", "请输入 Bug 标题", true),
            severity: FormField::new("严重程度 (Severity)", "3", "1-5", true),
            pri: FormField::new("优先级 (Pri)", "3", "1-4", false),
            type_: FormField::new(
                "类型 (Type)",
                "codeerror",
                "codeerror/config/install/...",
                true,
            ),
            project: FormField::new(
                "项目 (Project)",
                &project_id.map(|v| v.to_string()).unwrap_or_default(),
                "项目 ID（必填）",
                true,
            ),
            execution: FormField::new("执行 (Execution)", "", "执行 ID（必填）", true),
            module: FormField::new("模块 (Module)", "", "模块 ID（必填）", true),
            steps: FormField::new("重现步骤 (Steps)", "", "复现步骤", true),
        }
    }

    pub fn from_bug(bug: &crate::api::Bug) -> Self {
        Self {
            title: FormField::new("标题 (Title)", &bug.title, "", true),
            severity: FormField::new("严重程度 (Severity)", &bug.severity.to_string(), "", true),
            pri: FormField::new("优先级 (Pri)", &bug.pri.to_string(), "", false),
            type_: FormField::new(
                "类型 (Type)",
                bug.type_.as_deref().unwrap_or("codeerror"),
                "",
                true,
            ),
            project: FormField::readonly("项目 (Project)", ""),
            execution: FormField::new("执行 (Execution)", "", "", false),
            module: FormField::new("模块 (Module)", "", "", false),
            steps: FormField::new(
                "重现步骤 (Steps)",
                bug.steps.as_deref().unwrap_or(""),
                "",
                true,
            ),
        }
    }

    pub fn validate(&self) -> Option<String> {
        if self.title.value.trim().is_empty() {
            return Some("标题不能为空".to_string());
        }
        if self.project.value.trim().is_empty() {
            return Some("项目不能为空（禅道缺 project 会静默不落库）".to_string());
        }
        if self.execution.value.trim().is_empty() {
            return Some("执行不能为空（禅道缺 execution 会静默不落库）".to_string());
        }
        if self.module.value.trim().is_empty() {
            return Some("模块不能为空（禅道缺 module 会静默不落库）".to_string());
        }
        None
    }
}

impl FormFields for BugFormFields {
    fn get_fields(&self) -> Vec<&FormField> {
        vec![
            &self.title,
            &self.severity,
            &self.pri,
            &self.type_,
            &self.project,
            &self.execution,
            &self.module,
            &self.steps,
        ]
    }
    fn get_mut_fields(&mut self) -> Vec<&mut FormField> {
        vec![
            &mut self.title,
            &mut self.severity,
            &mut self.pri,
            &mut self.type_,
            &mut self.project,
            &mut self.execution,
            &mut self.module,
            &mut self.steps,
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 禅道缺 project/execution/module 会返回 201 但不落库，
    /// 表单必须在这三项为空时拦住提交。
    #[test]
    fn bug_form_rejects_missing_required_fields() {
        let mut f = BugFormFields::new(Some(1));
        assert!(f.validate().is_some(), "标题为空应拦截");

        f.title.value = "标题".into();
        assert!(f.validate().is_some(), "执行/模块为空应拦截");

        f.execution.value = "2".into();
        f.module.value = "3".into();
        assert!(f.validate().is_none(), "必填齐全应放行");

        f.project.value.clear();
        assert!(f.validate().is_some(), "项目为空应拦截");
    }

    #[test]
    fn bug_form_carries_config_project_as_default() {
        assert_eq!(BugFormFields::new(Some(42)).project.value, "42");
        assert_eq!(BugFormFields::new(None).project.value, "");
    }
}
