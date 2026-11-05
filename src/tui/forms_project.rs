//! Project（项目）表单字段

use crate::api::project::UpdateProjectRequest;

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
    pub fn from_project(p: &crate::api::Project) -> Self {
        Self {
            name: FormField::new("名称 (Name)", &p.name, "", true),
            code: FormField::new("代号 (Code)", &p.code, "", true),
            status: FormField::new("状态 (Status)", &p.status, "", false),
            desc: FormField::new("描述 (Desc)", p.desc.as_deref().unwrap_or(""), "", false),
            // Project 无 PM/预算/日期/关联产品字段，编辑时保持为空
            PM: FormField::new("项目经理 (PM)", "", "", false),
            budget: FormField::new("预算 (Budget)", "", "", false),
            budgetUnit: FormField::new("币种 (BudgetUnit)", "CNY", "", false),
            days: FormField::new("工作日 (Days)", "", "", false),
            begin: FormField::new("开始日期 (Begin)", "", "", false),
            end: FormField::new("结束日期 (End)", "", "", false),
            products: FormField::new("关联产品 (Products)", "", "", false),
        }
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

    fn sample() -> crate::api::Project {
        serde_json::from_str(r#"{"id":1,"name":"智小途","code":"tuzai","status":"doing"}"#)
            .expect("样例 JSON 应能解析")
    }

    /// 编辑态必须能从已有实体回填关键字段
    #[test]
    fn from_project_fills_key_field() {
        let f = ProjectFormFields::from_project(&sample());
        assert_eq!(f.name.value, sample().name);
    }

    /// 名称/标题清空后必须拦住提交
    #[test]
    fn rejects_empty_name() {
        let mut f = ProjectFormFields::from_project(&sample());
        f.name.value.clear();
        assert!(f.validate().is_some(), "空 name 应拦截");
    }
}
