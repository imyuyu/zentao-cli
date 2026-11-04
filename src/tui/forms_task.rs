//! Task 表单字段

use super::forms::{FormField, FormFields};
use crate::api::{CreateTaskRequest, Task, UpdateTaskRequest};

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
    pub fn new(execution_id: Option<u64>, project_id: Option<u64>) -> Self {
        Self {
            name: FormField::new("名称 (Name)", "", "请输入任务名称", true),
            assigned_to: FormField::new("指派给 (Assigned To)", "", "账号", true),
            type_: FormField::new(
                "类型 (Type)",
                "development",
                "development/design/test/study/discuss/ui/other",
                true,
            ),
            est_started: FormField::new("预计开始 (EstStarted)", "", "YYYY-MM-DD", true),
            deadline: FormField::new("预计结束 (Deadline)", "", "YYYY-MM-DD", true),
            desc: FormField::new("描述 (Desc)", "", "请输入任务描述", false),
            pri: FormField::new("优先级 (Pri)", "3", "1-5", false),
            module: FormField::new("模块 (Module)", "", "模块 ID", false),
            execution: FormField::readonly(
                "执行 (Execution)",
                &execution_id.map(|v| v.to_string()).unwrap_or_default(),
            ),
            project: FormField::readonly(
                "项目 (Project)",
                &project_id.map(|v| v.to_string()).unwrap_or_default(),
            ),
        }
    }

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

    pub fn to_create_request(&self) -> CreateTaskRequest {
        CreateTaskRequest {
            name: self.name.value.clone(),
            // 禅道创建要求数组，传字符串会 200 但不落库
            assigned_to: vec![self.assigned_to.value.clone()],
            type_: self.type_.value.clone(),
            est_started: self.est_started.value.clone(),
            deadline: self.deadline.value.clone(),
            project: self.project.value.parse().ok(),
            execution: self.execution.value.parse().ok(),
            desc: if self.desc.value.is_empty() {
                None
            } else {
                Some(self.desc.value.clone())
            },
            pri: self.pri.value.parse().ok(),
            module: self.module.value.parse().ok(),
            story: None,
            from_bug: None,
            estimate: None,
        }
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

    fn filled() -> TaskFormFields {
        let mut f = TaskFormFields::new(Some(1), Some(2));
        f.name.value = "任务".into();
        f.assigned_to.value = "admin".into();
        f.est_started.value = "2024-01-01".into();
        f.deadline.value = "2024-01-15".into();
        f
    }

    #[test]
    fn rejects_empty_name() {
        let mut f = filled();
        assert!(f.validate().is_none());
        f.name.value.clear();
        assert!(f.validate().is_some());
    }

    #[test]
    fn create_request_uses_single_element_array() {
        let req = filled().to_create_request();
        assert_eq!(req.assigned_to, vec!["admin".to_string()]);
        assert_eq!(req.assigned_to.len(), 1);
        assert_eq!(req.execution, Some(1));
        assert_eq!(req.project, Some(2));
    }
}
