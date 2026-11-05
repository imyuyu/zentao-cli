use crate::api::{Product, Project};

#[derive(Clone, Debug)]
#[allow(clippy::enum_variant_names)]
pub enum ConfigWizardState {
    Url,
    Account {
        url: String,
        account: String,
    },
    Password {
        url: String,
        account: String,
        password: String,
    },
    Connecting {
        url: String,
        account: String,
        password: String,
    },
    Success {
        url: String,
        token: String,
        account: String,
    },
    SelectProduct {
        url: String,
        token: String,
        account: String,
        products: Vec<Product>,
        selected: usize,
        loading: bool,
        error: Option<String>,
    },
    SelectProject {
        url: String,
        token: String,
        account: String,
        product_id: Option<u64>,
        projects: Vec<Project>,
        selected: usize,
        loading: bool,
        error: Option<String>,
    },
    Saved {
        url: String,
        path: String,
    }, // 保存成功后的确认状态
    Error {
        message: String,
    },
}

impl ConfigWizardState {
    pub fn new() -> Self {
        Self::Url
    }
}

pub struct ConfigWizard {
    pub state: ConfigWizardState,
}

/// 规范化禅道地址：只补协议、去尾斜杠
///
/// 部署路径（/zentao、/pms…）因环境而异，绝不猜测；缺失时由请求失败反馈给用户。
pub fn normalize_zentao_url(input: &str) -> String {
    let mut url = input.trim().to_string();
    if url.is_empty() {
        return url;
    }
    if !url.starts_with("http://") && !url.starts_with("https://") {
        url = format!("http://{}", url);
    }
    while url.ends_with('/') {
        url.pop();
    }
    url
}

impl ConfigWizard {
    pub fn new() -> Self {
        Self {
            state: ConfigWizardState::new(),
        }
    }

    pub fn set_url(&mut self, url: String) {
        self.state = ConfigWizardState::Account {
            url: normalize_zentao_url(&url),
            account: String::new(),
        };
    }

    pub fn set_account(&mut self, account: String) {
        if let ConfigWizardState::Account { url, .. } = &self.state {
            self.state = ConfigWizardState::Password {
                url: url.clone(),
                account,
                password: String::new(),
            };
        }
    }

    pub fn set_password(&mut self, password: String) {
        if let ConfigWizardState::Password { url, account, .. } = &self.state {
            self.state = ConfigWizardState::Connecting {
                url: url.clone(),
                account: account.clone(),
                password: password.clone(),
            };
        }
    }

    pub fn set_success(&mut self, url: String, token: String, account: String) {
        self.state = ConfigWizardState::Success {
            url,
            token,
            account,
        };
    }

    pub fn set_error(&mut self, message: String) {
        self.state = ConfigWizardState::Error { message };
    }

    pub fn set_saved(&mut self, url: String, path: String) {
        self.state = ConfigWizardState::Saved { url, path };
    }

    pub fn set_select_product(&mut self, url: String, token: String, account: String) {
        self.state = ConfigWizardState::SelectProduct {
            url,
            token,
            account,
            products: Vec::new(),
            selected: 0,
            loading: true,
            error: None,
        };
    }

    pub fn set_products(&mut self, products: Vec<Product>) {
        if let ConfigWizardState::SelectProduct { products: p, .. } = &mut self.state {
            *p = products;
            if let ConfigWizardState::SelectProduct { loading, .. } = &mut self.state {
                *loading = false;
            }
        }
    }

    pub fn set_product_selected(&mut self) {
        if let ConfigWizardState::SelectProduct {
            url,
            token,
            account,
            products,
            selected,
            ..
        } = &mut self.state
        {
            let selected_product_id = if !products.is_empty() {
                let idx = (*selected).min(products.len() - 1);
                Some(products[idx].id)
            } else {
                None
            };

            self.state = ConfigWizardState::SelectProject {
                url: url.clone(),
                token: token.clone(),
                account: account.clone(),
                product_id: selected_product_id,
                projects: Vec::new(),
                selected: 0,
                loading: true,
                error: None,
            };
        }
    }

    pub fn set_projects(&mut self, projects: Vec<Project>) {
        if let ConfigWizardState::SelectProject { projects: p, .. } = &mut self.state {
            *p = projects;
            if let ConfigWizardState::SelectProject { loading, .. } = &mut self.state {
                *loading = false;
            }
        }
    }

    pub fn set_project_selected(&mut self) {
        if let ConfigWizardState::SelectProject {
            product_id,
            projects,
            selected,
            ..
        } = &mut self.state
        {
            let selected_project_id = if !projects.is_empty() {
                let idx = (*selected).min(projects.len() - 1);
                Some(projects[idx].id)
            } else {
                None
            };

            // Update product_id with selected project
            *product_id = selected_project_id;
            // Clear projects list and mark as not loading (we're done selecting)
            *projects = Vec::new();
            // State stays in SelectProject - handle_enter will process the save
        }
    }

    pub fn set_load_error(&mut self, message: String) {
        if let ConfigWizardState::SelectProduct { error, loading, .. } = &mut self.state {
            *error = Some(message);
            *loading = false;
        } else if let ConfigWizardState::SelectProject { error, loading, .. } = &mut self.state {
            *error = Some(message);
            *loading = false;
        }
    }

    pub fn move_up(&mut self) {
        if let ConfigWizardState::SelectProduct { selected, .. } = &mut self.state {
            *selected = selected.saturating_sub(1);
        } else if let ConfigWizardState::SelectProject { selected, .. } = &mut self.state {
            *selected = selected.saturating_sub(1);
        }
    }

    pub fn move_down(&mut self) {
        if let ConfigWizardState::SelectProduct {
            selected, products, ..
        } = &mut self.state
        {
            *selected = (*selected + 1).min(products.len().saturating_sub(1));
        } else if let ConfigWizardState::SelectProject {
            selected, projects, ..
        } = &mut self.state
        {
            *selected = (*selected + 1).min(projects.len().saturating_sub(1));
        }
    }
}

impl Default for ConfigWizardState {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for ConfigWizard {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod url_tests {
    use super::normalize_zentao_url;

    #[test]
    fn adds_http_scheme() {
        assert_eq!(
            normalize_zentao_url("172.168.80.30:8001/zentao"),
            "http://172.168.80.30:8001/zentao"
        );
    }

    #[test]
    fn keeps_https() {
        assert_eq!(
            normalize_zentao_url("https://demo.zentao.site/zentao"),
            "https://demo.zentao.site/zentao"
        );
    }

    #[test]
    fn strips_trailing_slash() {
        assert_eq!(
            normalize_zentao_url("http://h:8001/zentao/"),
            "http://h:8001/zentao"
        );
    }

    /// 部署路径因环境而异（/zentao、/pms、根路径），绝不猜测
    #[test]
    fn never_invents_deploy_path() {
        assert_eq!(normalize_zentao_url("h:8001"), "http://h:8001");
        assert_eq!(normalize_zentao_url("h:8001/pms"), "http://h:8001/pms");
    }

    #[test]
    fn empty_stays_empty() {
        assert_eq!(normalize_zentao_url("   "), "");
    }
}
