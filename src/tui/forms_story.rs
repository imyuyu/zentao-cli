//! 需求(Story)表单字段

use crate::api::story::{CreateStoryRequest, UpdateStoryRequest};
use crate::tui::forms::{FormField, FormFields};

/// Story 表单字段
///
/// product 由列表上下文带出，只读展示
#[derive(Debug, Clone)]
pub struct StoryFormFields {
    pub title: FormField,
    pub pri: FormField,
    pub category: FormField,
    pub spec: FormField,
    pub verify: FormField,
    pub source: FormField,
    pub module: FormField,
    pub keywords: FormField,
    pub product: FormField,
}

impl StoryFormFields {
    pub fn new(product_id: u64) -> Self {
        Self {
            title: FormField::new("标题 (Title)", "", "请输入需求标题", true),
            pri: FormField::new("优先级 (Pri)", "3", "0-5", false),
            category: FormField::new(
                "类别 (Category)",
                "feature",
                "feature/requirement/bug/...",
                false,
            ),
            spec: FormField::new("描述 (Spec)", "", "需求描述", false),
            verify: FormField::new("验收标准 (Verify)", "", "验收标准", false),
            source: FormField::new("来源 (Source)", "", "需求来源", false),
            module: FormField::new("模块 (Module)", "", "模块 ID", false),
            keywords: FormField::new("关键字 (Keywords)", "", "关键字", false),
            product: FormField::readonly("产品 (Product)", &product_id.to_string()),
        }
    }

    /// Story 结构体不含 spec/verify/source/sourceNote，只能留空
    pub fn from_story(story: &crate::api::Story) -> Self {
        Self {
            title: FormField::new("标题 (Title)", &story.title, "", true),
            pri: FormField::new("优先级 (Pri)", &story.pri.to_string(), "", false),
            category: FormField::new(
                "类别 (Category)",
                story.category.as_deref().unwrap_or(""),
                "",
                false,
            ),
            spec: FormField::new(
                "描述 (Spec)",
                story.description.as_deref().unwrap_or(""),
                "",
                false,
            ),
            verify: FormField::new("验收标准 (Verify)", "", "", false),
            source: FormField::new("来源 (Source)", "", "", false),
            module: FormField::new(
                "模块 (Module)",
                &story.module.map(|v| v.to_string()).unwrap_or_default(),
                "",
                false,
            ),
            keywords: FormField::new("关键字 (Keywords)", "", "", false),
            product: FormField::readonly("产品 (Product)", &story.product.to_string()),
        }
    }

    pub fn validate(&self) -> Option<String> {
        if self.title.value.trim().is_empty() {
            return Some("标题不能为空".to_string());
        }
        if self.product.value.trim().is_empty() {
            return Some("产品不能为空".to_string());
        }
        None
    }

    pub fn to_create_request(&self) -> CreateStoryRequest {
        CreateStoryRequest {
            title: self.title.value.clone(),
            product: self.product.value.trim().parse().unwrap_or(0),
            pri: self.pri.value.trim().parse().unwrap_or(0),
            category: opt_string(&self.category.value),
            spec: opt_string(&self.spec.value),
            verify: opt_string(&self.verify.value),
            estimate: None,
            source: opt_string(&self.source.value),
            sourceNote: None,
            module: self.module.value.trim().parse().ok(),
            keywords: opt_string(&self.keywords.value),
        }
    }

    pub fn to_update_request(&self) -> UpdateStoryRequest {
        UpdateStoryRequest {
            title: Some(self.title.value.clone()),
            module: if self.module.value.trim().is_empty() {
                None
            } else {
                self.module.value.trim().parse().ok()
            },
            source: opt_string(&self.source.value),
            sourceNote: None,
            pri: if self.pri.value.trim().is_empty() {
                None
            } else {
                self.pri.value.trim().parse().ok()
            },
            category: opt_string(&self.category.value),
            estimate: None,
            keywords: opt_string(&self.keywords.value),
            assigned_to: None,
            status: None,
        }
    }
}

/// 空串转 None
fn opt_string(v: &str) -> Option<String> {
    if v.is_empty() {
        None
    } else {
        Some(v.to_string())
    }
}

impl FormFields for StoryFormFields {
    fn get_fields(&self) -> Vec<&FormField> {
        vec![
            &self.title,
            &self.pri,
            &self.category,
            &self.spec,
            &self.verify,
            &self.source,
            &self.module,
            &self.keywords,
            &self.product,
        ]
    }
    fn get_mut_fields(&mut self) -> Vec<&mut FormField> {
        vec![
            &mut self.title,
            &mut self.pri,
            &mut self.category,
            &mut self.spec,
            &mut self.verify,
            &mut self.source,
            &mut self.module,
            &mut self.keywords,
            &mut self.product,
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn story_form_rejects_empty_title() {
        let mut f = StoryFormFields::new(1);
        assert!(f.validate().is_some(), "标题为空应拦截");
        f.title.value = "标题".into();
        assert!(f.validate().is_none(), "必填齐全应放行");
    }

    #[test]
    fn story_form_parses_pri() {
        let mut f = StoryFormFields::new(7);
        f.title.value = "标题".into();
        f.pri.value = "3".into();
        let req = f.to_create_request();
        assert_eq!(req.pri, 3);
        assert_eq!(req.product, 7);
    }
}
