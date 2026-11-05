//! Build（版本）表单字段

use crate::api::build::UpdateBuildRequest;

use super::forms::{FormField, FormFields};

/// Build 表单字段
///
/// product / execution 只读（来自当前上下文），其余可编辑
#[derive(Debug, Clone)]
pub struct BuildFormFields {
    pub product: FormField,
    pub execution: FormField,
    pub name: FormField,
    pub builder: FormField,
    pub date: FormField,
    pub scm_path: FormField,
    pub file_path: FormField,
    pub desc: FormField,
}

/// 空串转 None
fn opt(value: &str) -> Option<String> {
    if value.is_empty() {
        None
    } else {
        Some(value.to_string())
    }
}

impl BuildFormFields {
    pub fn from_build(build: &crate::api::Build) -> Self {
        Self {
            product: FormField::readonly("产品 (Product)", &build.product.to_string()),
            // Build 无 execution 字段，只读占位
            execution: FormField::readonly("执行 (Execution)", ""),
            name: FormField::new("名称 (Name)", &build.name, "", true),
            // Build 无 builder 字段
            builder: FormField::new("构建者 (Builder)", "", "", true),
            // generated_at 是 "YYYY-MM-DD HH:MM:SS"，只取日期部分
            date: FormField::new(
                "打包日期 (Date)",
                build
                    .generated_at
                    .as_deref()
                    .and_then(|v| v.split(' ').next())
                    .unwrap_or(""),
                "",
                false,
            ),
            scm_path: FormField::new(
                "源码地址 (SCM Path)",
                build.scm_path.as_deref().unwrap_or(""),
                "",
                false,
            ),
            // 包路径在 Build 里叫 pkg
            file_path: FormField::new(
                "下载地址 (File Path)",
                build.pkg.as_deref().unwrap_or(""),
                "",
                false,
            ),
            desc: FormField::new("描述 (Desc)", "", "", false),
        }
    }

    pub fn validate(&self) -> Option<String> {
        if self.name.value.trim().is_empty() {
            return Some("名称不能为空".to_string());
        }
        if self.builder.value.trim().is_empty() {
            return Some("构建者不能为空".to_string());
        }
        None
    }

    pub fn to_update_request(&self) -> UpdateBuildRequest {
        UpdateBuildRequest {
            name: opt(&self.name.value),
            scm_path: opt(&self.scm_path.value),
            file_path: opt(&self.file_path.value),
            date: opt(&self.date.value),
            builder: opt(&self.builder.value),
            desc: opt(&self.desc.value),
        }
    }
}

impl FormFields for BuildFormFields {
    fn get_fields(&self) -> Vec<&FormField> {
        vec![
            &self.product,
            &self.execution,
            &self.name,
            &self.builder,
            &self.date,
            &self.scm_path,
            &self.file_path,
            &self.desc,
        ]
    }
    fn get_mut_fields(&mut self) -> Vec<&mut FormField> {
        vec![
            &mut self.product,
            &mut self.execution,
            &mut self.name,
            &mut self.builder,
            &mut self.date,
            &mut self.scm_path,
            &mut self.file_path,
            &mut self.desc,
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> crate::api::Build {
        serde_json::from_str(r#"{"id":1,"name":"v1.0","product":7,"project":3}"#)
            .expect("样例 JSON 应能解析")
    }

    /// 编辑态必须能从已有实体回填关键字段
    #[test]
    fn from_build_fills_key_field() {
        let f = BuildFormFields::from_build(&sample());
        assert_eq!(f.name.value, sample().name);
    }

    /// 名称/标题清空后必须拦住提交
    #[test]
    fn rejects_empty_name() {
        let mut f = BuildFormFields::from_build(&sample());
        f.name.value.clear();
        assert!(f.validate().is_some(), "空 name 应拦截");
    }
}
