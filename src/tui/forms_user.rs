//! 用户表单字段

use crate::api::user::UpdateUserRequest;
use crate::tui::forms::{FormField, FormFields};

/// 用户表单字段
///
/// 必填项对齐 CreateUserRequest：account / password / realname
#[derive(Debug, Clone)]
pub struct UserFormFields {
    pub account: FormField,
    pub password: FormField,
    pub realname: FormField,
    pub role: FormField,
    pub dept: FormField,
    pub email: FormField,
    pub mobile: FormField,
    pub phone: FormField,
}

impl UserFormFields {
    /// 从已有用户填充（编辑场景，账号不可改）
    pub fn from_user(u: &crate::api::User) -> Self {
        Self {
            account: FormField::readonly("账号 (Account)", &u.account),
            password: FormField::new("密码 (Password)", "", "留空则不修改", false),
            realname: FormField::new("姓名 (Realname)", &u.realname, "", true),
            role: FormField::new("角色 (Role)", u.role.as_deref().unwrap_or(""), "", false),
            dept: FormField::new(
                "部门 (Dept)",
                &u.dept.map(|v| v.to_string()).unwrap_or_default(),
                "部门 ID",
                false,
            ),
            email: FormField::new("邮箱 (Email)", u.email.as_deref().unwrap_or(""), "", false),
            mobile: FormField::new("手机 (Mobile)", "", "", false),
            phone: FormField::new("电话 (Phone)", "", "", false),
        }
    }

    pub fn validate(&self) -> Option<String> {
        if self.account.value.trim().is_empty() {
            return Some("账号不能为空".to_string());
        }
        if self.password.value.trim().is_empty() {
            return Some("密码不能为空".to_string());
        }
        if self.realname.value.trim().is_empty() {
            return Some("姓名不能为空".to_string());
        }
        None
    }

    pub fn to_update_request(&self) -> UpdateUserRequest {
        UpdateUserRequest {
            dept: self.dept.value.parse().ok(),
            role: opt(&self.role.value),
            mobile: opt(&self.mobile.value),
            realname: opt(&self.realname.value),
            email: opt(&self.email.value),
            phone: opt(&self.phone.value),
        }
    }
}

/// 空串转 None
fn opt(s: &str) -> Option<String> {
    if s.is_empty() {
        None
    } else {
        Some(s.to_string())
    }
}

impl FormFields for UserFormFields {
    fn get_fields(&self) -> Vec<&FormField> {
        vec![
            &self.account,
            &self.password,
            &self.realname,
            &self.role,
            &self.dept,
            &self.email,
            &self.mobile,
            &self.phone,
        ]
    }
    fn get_mut_fields(&mut self) -> Vec<&mut FormField> {
        vec![
            &mut self.account,
            &mut self.password,
            &mut self.realname,
            &mut self.role,
            &mut self.dept,
            &mut self.email,
            &mut self.mobile,
            &mut self.phone,
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> crate::api::User {
        serde_json::from_str(r#"{"id":1,"account":"admin","realname":"管理员","role":"dev"}"#)
            .expect("样例 JSON 应能解析")
    }

    /// 编辑态必须能从已有实体回填关键字段
    #[test]
    fn from_user_fills_key_field() {
        let f = UserFormFields::from_user(&sample());
        assert_eq!(f.realname.value, sample().realname);
    }

    /// 名称/标题清空后必须拦住提交
    #[test]
    fn rejects_empty_name() {
        let mut f = UserFormFields::from_user(&sample());
        f.realname.value.clear();
        assert!(f.validate().is_some(), "空 name 应拦截");
    }
}
