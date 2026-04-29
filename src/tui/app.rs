use crate::api::{Bug, Story};

#[derive(Debug, Clone)]
pub enum AppState {
    Idle,
    Loading { message: String },
    BugList { bugs: Vec<Bug> },
    BugDetail { bug: Bug },
    StoryList { stories: Vec<Story> },
    StoryDetail { story: Story },
    Error { message: String },
    Quit,
}

impl AppState {
    pub fn is_quitting(&self) -> bool {
        matches!(self, AppState::Quit)
    }
}

pub struct App {
    pub state: AppState,
    pub selected_index: usize,
}

impl App {
    pub fn new() -> Self {
        Self {
            state: AppState::Idle,
            selected_index: 0,
        }
    }

    pub fn set_loading(&mut self, message: String) {
        self.state = AppState::Loading { message };
        self.selected_index = 0;
    }

    pub fn set_bug_list(&mut self, bugs: Vec<Bug>) {
        self.state = AppState::BugList { bugs };
        self.selected_index = 0;
    }

    pub fn set_bug_detail(&mut self, bug: Bug) {
        self.state = AppState::BugDetail { bug };
        self.selected_index = 0;
    }

    pub fn set_story_list(&mut self, stories: Vec<Story>) {
        self.state = AppState::StoryList { stories };
        self.selected_index = 0;
    }

    pub fn set_story_detail(&mut self, story: Story) {
        self.state = AppState::StoryDetail { story };
        self.selected_index = 0;
    }

    pub fn set_error(&mut self, message: String) {
        self.state = AppState::Error { message };
    }

    pub fn go_back_to_list(&mut self) {
        match &self.state {
            AppState::BugDetail { .. } => {
                // Go back to bug list - caller should reload
                self.selected_index = 0;
            }
            AppState::StoryDetail { .. } => {
                // Go back to story list - caller should reload
                self.selected_index = 0;
            }
            _ => {}
        }
    }

    pub fn quit(&mut self) {
        self.state = AppState::Quit;
    }

    pub fn get_selected_bug_id(&self) -> Option<u64> {
        if let AppState::BugList { bugs } = &self.state {
            bugs.get(self.selected_index).map(|b| b.id)
        } else {
            None
        }
    }

    pub fn get_selected_story_id(&self) -> Option<u64> {
        if let AppState::StoryList { stories } = &self.state {
            stories.get(self.selected_index).map(|s| s.id)
        } else {
            None
        }
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}
