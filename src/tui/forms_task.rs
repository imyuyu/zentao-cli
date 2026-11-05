//! Task 表单字段

use super::forms::{FormField, FormFields};
use crate::api::{Task, UpdateTaskRequest};

/// Task 表单字段
///
/// execution/project 来自列表上下文，只读展示
#[derive(Debug, Clone)]
pub struct TaskFormFields {
    pub name: FormField,
    pub assigned_to: FormField,
    pub type_: FormField,
    pub est_started: FormField,
    pub deadline: FormField,
    pub desc: FormField,
    pub pri: FormField,
    pub module: FormField,
    pub execution: FormField,
    pub project: FormField,
}

impl TaskFormFields {
    /// 编辑用。Task 没有 type/日期/desc/module，留空或取默认值
    pub fn from_task(task: &Task) -> Self {
        Self {
            name: FormField::new("名称 (Name)", &task.name, "", true),
            assigned_to: FormField::new(
                "指派给 (Assigned To)",
                task.assigned_to.as_deref().unwrap_or(""),
                "账号",
                true,
            ),
            type_: FormField::new("类型 (Type)", "development", "", true),
            est_started: FormField::new("预计开始 (EstStarted)", "", "YYYY-MM-DD", true),
            deadline: FormField::new("预计结束 (Deadline)", "", "YYYY-MM-DD", true),
            desc: FormField::new("描述 (Desc)", "", "请输入任务描述", false),
            pri: FormField::new("优先级 (Pri)", &task.pri.to_string(), "1-5", false),
            module: FormField::new("模块 (Module)", "", "模块 ID", false),
            execution: FormField::readonly("执行 (Execution)", ""),
            project: FormField::readonly("项目 (Project)", &task.project.to_string()),
        }
    }

    pub fn validate(&self) -> Option<String> {
        if self.name.value.trim().is_empty() {
            return Some("名称不能为空".to_string());
        }
        if self.assigned_to.value.trim().is_empty() {
            return Some("指派给不能为空".to_string());
        }
        if self.est_started.value.trim().is_empty() {
            return Some("预计开始日期不能为空".to_string());
        }
        if self.deadline.value.trim().is_empty() {
            return Some("预计结束日期不能为空".to_string());
        }
        None
    }

    pub fn to_update_request(&self) -> UpdateTaskRequest {
        UpdateTaskRequest {
            name: Some(self.name.value.clone()),
            status: None,
            pri: self.pri.value.parse().ok(),
            // 更新接口这里是字符串，不是数组
            assigned_to: Some(self.assigned_to.value.clone()),
            module: self.module.value.parse().ok(),
            story: None,
            from_bug: None,
            type_: Some(self.type_.value.clone()),
            est_started: if self.est_started.value.is_empty() {
                None
            } else {
                Some(self.est_started.value.clone())
            },
            deadline: if self.deadline.value.is_empty() {
                None
            } else {
                Some(self.deadline.value.clone())
            },
            estimate: None,
        }
    }
}

impl FormFields for TaskFormFields {
    fn get_fields(&self) -> Vec<&FormField> {
        vec![
            &self.name,
            &self.assigned_to,
            &self.type_,
            &self.est_started,
            &self.deadline,
            &self.desc,
            &self.pri,
            &self.module,
            &self.execution,
            &self.project,
        ]
    }
    fn get_mut_fields(&mut self) -> Vec<&mut FormField> {
        vec![
            &mut self.name,
            &mut self.assigned_to,
            &mut self.type_,
            &mut self.est_started,
            &mut self.deadline,
            &mut self.desc,
            &mut self.pri,
            &mut self.module,
            &mut self.execution,
            &mut self.project,
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> crate::api::Task {
        serde_json::from_str(r#"{"id":1,"name":"修登录","project":3,"status":"todo","pri":2}"#)
            .expect("样例 JSON 应能解析")
    }

    /// 编辑态必须能从已有实体回填关键字段
    #[test]
    fn from_task_fills_key_field() {
        let f = TaskFormFields::from_task(&sample());
        assert_eq!(f.name.value, sample().name);
    }

    /// 名称/标题清空后必须拦住提交
    #[test]
    fn rejects_empty_name() {
        let mut f = TaskFormFields::from_task(&sample());
        f.name.value.clear();
        assert!(f.validate().is_some(), "空 name 应拦截");
    }
}
