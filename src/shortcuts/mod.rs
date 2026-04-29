pub mod story;
pub mod bug;

use clap::Parser;

/// ZenTao Shortcuts - Quick actions for common operations
#[derive(Parser, Debug)]
pub enum Shortcut {
    /// List stories in current project
    StoryList,
    /// Get a story by ID
    StoryGet {
        id: u64,
    },
    /// List bugs in current project
    BugList,
    /// Get a bug by ID
    BugGet {
        id: u64,
    },
    /// List all open bugs assigned to current user
    MyBugs,
}

impl Shortcut {
    pub fn run(&self) -> anyhow::Result<()> {
        match self {
            Self::StoryList => {
                println!("+story-list");
                // This would invoke: zentao story list
            }
            Self::StoryGet { id } => {
                println!("+story-get {}", id);
            }
            Self::BugList => {
                println!("+bug-list");
            }
            Self::BugGet { id } => {
                println!("+bug-get {}", id);
            }
            Self::MyBugs => {
                println!("+my-bugs");
            }
        }
        Ok(())
    }
}

/// All available shortcuts
pub const SHORTCUTS: &[(&str, &str)] = &[
    ("+story-list", "List stories in current project"),
    ("+story-get", "Get a story by ID"),
    ("+bug-list", "List bugs in current project"),
    ("+bug-get", "Get a bug by ID"),
    ("+my-bugs", "List bugs assigned to me"),
    ("+my-stories", "List stories assigned to me"),
    ("+create-bug", "Create a new bug interactively"),
    ("+create-story", "Create a new story interactively"),
];

pub fn list_shortcuts() {
    println!("Available shortcuts:");
    for (name, desc) in SHORTCUTS {
        println!("  {:20} - {}", name, desc);
    }
}
