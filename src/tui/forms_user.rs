//! 用户表单字段

use crate::api::user::{CreateUserRequest, UpdateUserRequest};
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
    pub fn new() -> Self {
        Self {
            account: FormField::new("账号 (Account)", "", "登录账号", true),
            password: FormField::new("密码 (Password)", "", "登录密码", true),
            realname: FormField::new("姓名 (Realname)", "", "真实姓名", true),
            role: FormField::new("角色 (Role)", "", "角色标识，如 dev", false),
            dept: FormField::new("部门 (Dept)", "", "部门 ID", false),
            email: FormField::new("邮箱 (Email)", "", "user@example.com", false),
            mobile: FormField::new("手机 (Mobile)", "", "手机号", false),
            phone: FormField::new("电话 (Phone)", "", "座机/分机", false),
        }
    }

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

    pub fn to_create_request(&self) -> CreateUserRequest {
        CreateUserRequest {
            account: self.account.value.clone(),
            password: self.password.value.clone(),
            realname: self.realname.value.clone(),
            role: opt(&self.role.value),
            dept: self.dept.value.parse().ok(),
            mobile: opt(&self.mobile.value),
            email: opt(&self.email.value),
            phone: opt(&self.phone.value),
        }
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

impl Default for UserFormFields {
    fn default() -> Self {
        Self::new()
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

    #[test]
    fn user_form_rejects_missing_required_fields() {
        let mut f = UserFormFields::new();
        assert!(f.validate().is_some(), "账号为空应拦截");

        f.account.value = "zhangsan".into();
        assert!(f.validate().is_some(), "密码为空应拦截");

        f.password.value = "123456".into();
        assert!(f.validate().is_some(), "姓名为空应拦截");

        f.realname.value = "张三".into();
        assert!(f.validate().is_none(), "必填齐全应放行");
    }

    #[test]
    fn user_form_empty_strings_become_none() {
        let mut f = UserFormFields::new();
        f.account.value = "zhangsan".into();
        f.password.value = "123456".into();
        f.realname.value = "张三".into();

        let req = f.to_create_request();
        assert_eq!(req.role, None);
        assert_eq!(req.dept, None);
        assert_eq!(req.email, None);
        assert_eq!(req.mobile, None);
        assert_eq!(req.phone, None);

        f.dept.value = "42".into();
        f.email.value = "a@b.com".into();
        let req = f.to_update_request();
        assert_eq!(req.dept, Some(42));
        assert_eq!(req.email, Some("a@b.com".to_string()));
        assert_eq!(req.mobile, None);
    }
}
