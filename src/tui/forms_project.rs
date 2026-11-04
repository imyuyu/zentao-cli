//! Project（项目）表单字段

use crate::api::project::{CreateProjectRequest, UpdateProjectRequest};

use super::forms::{FormField, FormFields};

/// Project 表单字段
///
/// code/begin/end/products 是禅道创建接口的必填项，编辑时后端忽略
#[allow(non_snake_case)]
#[derive(Debug, Clone)]
pub struct ProjectFormFields {
    pub name: FormField,
    pub code: FormField,
    pub status: FormField,
    pub desc: FormField,
    pub PM: FormField,
    pub budget: FormField,
    pub budgetUnit: FormField,
    pub days: FormField,
    pub begin: FormField,
    pub end: FormField,
    pub products: FormField,
}

/// 空串转 None
fn opt(value: &str) -> Option<String> {
    if value.is_empty() {
        None
    } else {
        Some(value.to_string())
    }
}

impl ProjectFormFields {
    pub fn new() -> Self {
        Self {
            name: FormField::new("名称 (Name)", "", "请输入项目名称", true),
            code: FormField::new("代号 (Code)", "", "请输入项目代号", true),
            status: FormField::new(
                "状态 (Status)",
                "wait",
                "wait / doing / closed / suspend",
                false,
            ),
            desc: FormField::new("描述 (Desc)", "", "请输入描述", false),
            PM: FormField::new("项目经理 (PM)", "", "请输入 PM 账号", false),
            budget: FormField::new("预算 (Budget)", "", "数字", false),
            budgetUnit: FormField::new("币种 (BudgetUnit)", "CNY", "CNY / USD", false),
            days: FormField::new("工作日 (Days)", "", "数字", false),
            begin: FormField::new("开始日期 (Begin)", "", "YYYY-MM-DD", true),
            end: FormField::new("结束日期 (End)", "", "YYYY-MM-DD", true),
            products: FormField::new("关联产品 (Products)", "", "产品 ID，逗号分隔，如 1,2", true),
        }
    }

    pub fn from_project(p: &crate::api::Project) -> Self {
        let mut f = Self::new();
        f.name.value = p.name.clone();
        f.code.value = p.code.clone();
        f.status.value = p.status.clone();
        f.desc.value = p.desc.clone().unwrap_or_default();
        // Project 无 PM/预算/日期/关联产品字段，保持为空
        f
    }

    pub fn validate(&self) -> Option<String> {
        if self.name.value.trim().is_empty() {
            return Some("名称不能为空".to_string());
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
        if self.products.value.trim().is_empty() {
            return Some("关联产品不能为空（至少一个产品 ID）".to_string());
        }
        None
    }

    pub fn to_create_request(&self) -> CreateProjectRequest {
        CreateProjectRequest {
            name: self.name.value.clone(),
            code: self.code.value.clone(),
            begin: self.begin.value.clone(),
            end: self.end.value.clone(),
            products: self
                .products
                .value
                .split(',')
                .filter_map(|s| s.trim().parse().ok())
                .collect(),
            model: None,
            parent: None,
            desc: opt(&self.desc.value),
        }
    }

    pub fn to_update_request(&self) -> UpdateProjectRequest {
        UpdateProjectRequest {
            name: opt(&self.name.value),
            status: opt(&self.status.value),
            desc: opt(&self.desc.value),
            parent: None,
            PM: opt(&self.PM.value),
            budget: self.budget.value.trim().parse().ok(),
            budgetUnit: opt(&self.budgetUnit.value),
            days: self.days.value.trim().parse().ok(),
            acl: None,
            whitelist: None,
            auth: None,
        }
    }
}

impl FormFields for ProjectFormFields {
    fn get_fields(&self) -> Vec<&FormField> {
        vec![
            &self.name,
            &self.code,
            &self.status,
            &self.desc,
            &self.PM,
            &self.budget,
            &self.budgetUnit,
            &self.days,
            &self.begin,
            &self.end,
            &self.products,
        ]
    }
    fn get_mut_fields(&mut self) -> Vec<&mut FormField> {
        vec![
            &mut self.name,
            &mut self.code,
            &mut self.status,
            &mut self.desc,
            &mut self.PM,
            &mut self.budget,
            &mut self.budgetUnit,
            &mut self.days,
            &mut self.begin,
            &mut self.end,
            &mut self.products,
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn project_form_rejects_empty_name() {
        let f = ProjectFormFields::new();
        assert!(f.validate().is_some(), "名称为空应拦截");
    }

    #[test]
    fn project_form_empty_fields_become_none() {
        let f = ProjectFormFields::new();
        let req = f.to_update_request();
        assert!(req.name.is_none());
        assert!(req.desc.is_none());
        assert!(req.PM.is_none());
        assert!(req.budget.is_none());
        assert!(req.days.is_none());
        // status / budgetUnit 默认值非空
        assert_eq!(req.status.as_deref(), Some("wait"));
        assert_eq!(req.budgetUnit.as_deref(), Some("CNY"));
    }

    #[test]
    fn project_create_request_parses_products() {
        let mut f = ProjectFormFields::new();
        f.products.value = " 1, 2 ,x".into();
        assert_eq!(f.to_create_request().products, vec![1, 2]);
    }
}
