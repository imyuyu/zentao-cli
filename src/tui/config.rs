#[derive(Clone, Debug)]
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

impl ConfigWizard {
    pub fn new() -> Self {
        Self {
            state: ConfigWizardState::new(),
        }
    }

    pub fn set_url(&mut self, url: String) {
        self.state = ConfigWizardState::Account {
            url,
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

    pub fn set_success(&mut self, url: String, token: String) {
        self.state = ConfigWizardState::Success { url, token };
    }

    pub fn set_error(&mut self, message: String) {
        self.state = ConfigWizardState::Error { message };
    }

    pub fn set_saved(&mut self, url: String, path: String) {
        self.state = ConfigWizardState::Saved { url, path };
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
