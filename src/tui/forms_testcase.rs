//! 测试用例表单字段
//!
//! 简化版：steps 只编辑单步（desc + expect），多步请走 API 直传

use crate::api::testcase::{CreateTestcaseRequest, TestcaseStep, UpdateTestcaseRequest};
use crate::tui::forms::{FormField, FormFields};

/// 测试用例表单字段
#[derive(Debug, Clone)]
pub struct TestcaseFormFields {
    pub product: FormField,
    pub module: FormField,
    pub title: FormField,
    pub type_: FormField,
    pub stage: FormField,
    pub precondition: FormField,
    pub pri: FormField,
    pub steps_desc: FormField,
    pub steps_expect: FormField,
    pub keywords: FormField,
}

impl TestcaseFormFields {
    pub fn new(product_id: u64) -> Self {
        Self {
            product: FormField::readonly("产品 (Product)", &product_id.to_string()),
            module: FormField::readonly("模块 (Module)", ""),
            title: FormField::new("标题 (Title)", "", "请输入用例标题", true),
            type_: FormField::new(
                "类型 (Type)",
                "feature",
                "feature/performance/config/install/security/interface/unit/other",
                true,
            ),
            stage: FormField::new(
                "阶段 (Stage)",
                "",
                "unitest/feature/intergrate/system/smoke/bvt",
                false,
            ),
            precondition: FormField::new("前置条件 (Precondition)", "", "前置条件", false),
            pri: FormField::new("优先级 (Pri)", "3", "0-5", false),
            steps_desc: FormField::new("步骤 (Steps)", "", "操作步骤", true),
            steps_expect: FormField::new("期望结果 (Expect)", "", "期望结果", false),
            keywords: FormField::new("关键词 (Keywords)", "", "多个用逗号分隔", false),
        }
    }

    pub fn from_testcase(tc: &crate::api::Testcase) -> Self {
        let step = tc.steps.first();
        Self {
            product: FormField::readonly("产品 (Product)", &tc.product.to_string()),
            module: FormField::readonly("模块 (Module)", &tc.module.to_string()),
            title: FormField::new("标题 (Title)", &tc.title, "", true),
            type_: FormField::new("类型 (Type)", &tc.type_, "", true),
            stage: FormField::new("阶段 (Stage)", &tc.stage, "", false),
            precondition: FormField::new("前置条件 (Precondition)", &tc.precondition, "", false),
            pri: FormField::new("优先级 (Pri)", &tc.pri.to_string(), "", false),
            steps_desc: FormField::new(
                "步骤 (Steps)",
                step.map(|s| s.desc.as_str()).unwrap_or(""),
                "",
                true,
            ),
            steps_expect: FormField::new(
                "期望结果 (Expect)",
                step.map(|s| s.expect.as_str()).unwrap_or(""),
                "",
                false,
            ),
            keywords: FormField::new("关键词 (Keywords)", "", "", false),
        }
    }

    pub fn validate(&self) -> Option<String> {
        if self.title.value.trim().is_empty() {
            return Some("标题不能为空".to_string());
        }
        None
    }

    pub fn to_create_request(&self) -> CreateTestcaseRequest {
        CreateTestcaseRequest {
            product: self.product.value.parse().unwrap_or_default(),
            title: self.title.value.clone(),
            type_: self.type_.value.clone(),
            branch: None,
            module: self.module.value.parse().ok(),
            story: None,
            stage: opt(&self.stage.value),
            precondition: opt(&self.precondition.value),
            pri: self.pri.value.parse().ok(),
            steps: vec![TestcaseStep {
                desc: self.steps_desc.value.clone(),
                expect: self.steps_expect.value.clone(),
            }],
            keywords: opt(&self.keywords.value),
        }
    }

    pub fn to_update_request(&self) -> UpdateTestcaseRequest {
        UpdateTestcaseRequest {
            branch: None,
            module: self.module.value.parse().ok(),
            story: None,
            title: opt(&self.title.value),
            type_: opt(&self.type_.value),
            stage: opt(&self.stage.value),
            precondition: opt(&self.precondition.value),
            pri: self.pri.value.parse().ok(),
            steps: Some(vec![TestcaseStep {
                desc: self.steps_desc.value.clone(),
                expect: self.steps_expect.value.clone(),
            }]),
            keywords: opt(&self.keywords.value),
        }
    }
}

/// 空串转 None
fn opt(v: &str) -> Option<String> {
    if v.is_empty() {
        None
    } else {
        Some(v.to_string())
    }
}

impl FormFields for TestcaseFormFields {
    fn get_fields(&self) -> Vec<&FormField> {
        vec![
            &self.product,
            &self.module,
            &self.title,
            &self.type_,
            &self.stage,
            &self.precondition,
            &self.pri,
            &self.steps_desc,
            &self.steps_expect,
            &self.keywords,
        ]
    }
    fn get_mut_fields(&mut self) -> Vec<&mut FormField> {
        vec![
            &mut self.product,
            &mut self.module,
            &mut self.title,
            &mut self.type_,
            &mut self.stage,
            &mut self.precondition,
            &mut self.pri,
            &mut self.steps_desc,
            &mut self.steps_expect,
            &mut self.keywords,
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn testcase_form_rejects_empty_title() {
        let f = TestcaseFormFields::new(1);
        assert!(f.validate().is_some(), "标题为空应拦截");
    }

    #[test]
    fn testcase_form_create_has_single_step() {
        let mut f = TestcaseFormFields::new(7);
        f.title.value = "登录用例".into();
        f.steps_desc.value = "输入账号密码".into();
        let req = f.to_create_request();
        assert_eq!(req.product, 7);
        assert_eq!(req.steps.len(), 1);
        assert_eq!(req.steps[0].desc, "输入账号密码");
        assert_eq!(req.type_, "feature");
    }
}
