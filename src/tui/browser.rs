use anyhow::Result;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Terminal,
};
use std::io::Stdout;

use super::app::{App, AppState};

pub struct Browser {
    terminal: Terminal<CrosstermBackend<Stdout>>,
}

impl Browser {
    pub fn new() -> Result<Self> {
        let stdout = std::io::stdout();
        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend)?;
        Ok(Self { terminal })
    }

    pub fn run(&mut self, app: &mut App) -> Result<()> {
        loop {
            if app.state.is_quitting() {
                break;
            }

            let selected = app.selected_index;
            self.terminal.draw(|f| {
                let area = f.size();
                match &app.state {
                    AppState::Idle => {
                        Self::render_idle(f, area);
                    }
                    AppState::Loading { message } => {
                        Self::render_loading(f, area, message);
                    }
                    AppState::BugList { bugs } => {
                        Self::render_bug_list(f, area, bugs, selected);
                    }
                    AppState::BugDetail { bug } => {
                        Self::render_bug_detail(f, area, bug);
                    }
                    AppState::StoryList { stories } => {
                        Self::render_story_list(f, area, stories, selected);
                    }
                    AppState::StoryDetail { story } => {
                        Self::render_story_detail(f, area, story);
                    }
                    AppState::Error { message } => {
                        Self::render_error(f, area, message);
                    }
                    AppState::Quit => {}
                }
            })?;

            // Handle input
            if crossterm::event::poll(std::time::Duration::from_millis(100))? {
                if let Ok(crossterm::event::Event::Key(key)) = crossterm::event::read() {
                    self.handle_key_event(key, app);
                }
            }
        }
        Ok(())
    }

    fn handle_key_event(&mut self, key: KeyEvent, app: &mut App) {
        match key.code {
            KeyCode::Up | KeyCode::Char('k') => {
                app.selected_index = app.selected_index.saturating_sub(1);
            }
            KeyCode::Down | KeyCode::Char('j') => {
                let max = match &app.state {
                    AppState::BugList { bugs } => bugs.len().saturating_sub(1),
                    AppState::StoryList { stories } => stories.len().saturating_sub(1),
                    _ => 0,
                };
                if app.selected_index < max {
                    app.selected_index += 1;
                }
            }
            KeyCode::Enter => {
                // Handle selection - caller should process this
            }
            KeyCode::Esc | KeyCode::Char('q') => match &app.state {
                AppState::BugDetail { .. } | AppState::StoryDetail { .. } => {
                    app.go_back_to_list();
                }
                _ => {
                    app.quit();
                }
            },
            KeyCode::Char('r') => {
                // Refresh - caller should handle this
            }
            _ => {}
        }
    }

    fn render_idle(f: &mut ratatui::Frame, area: Rect) {
        let text = Paragraph::new(Text::from(vec![
            Line::from(Span::raw("ZenTao CLI")),
            Line::from(Span::raw("")),
            Line::from(Span::raw(
                "Use commands: zentao bug browse, zentao story browse",
            )),
        ]))
        .block(Block::default().borders(Borders::ALL).title("ZenTao"));

        f.render_widget(text, area);
    }

    fn render_loading(f: &mut ratatui::Frame, area: Rect, message: &str) {
        let text = Paragraph::new(Text::from(vec![
            Line::from(Span::raw(message)),
            Line::from(Span::raw("")),
            Line::from(Span::styled(
                "Loading...",
                Style::default().fg(Color::Yellow),
            )),
        ]))
        .block(Block::default().borders(Borders::ALL).title("Please wait"));

        f.render_widget(text, area);
    }

    fn render_bug_list(
        f: &mut ratatui::Frame,
        area: Rect,
        bugs: &[crate::api::Bug],
        selected: usize,
    ) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(0),
                Constraint::Length(3),
            ])
            .split(area);

        let header = Paragraph::new(Text::from(vec![Line::from(vec![
            Span::raw("Bug List ("),
            Span::raw(format!("{}", bugs.len())),
            Span::raw(" items) - "),
            Span::styled(
                "↑↓ select | Enter view | q quit",
                Style::default().fg(Color::DarkGray),
            ),
        ])]))
        .block(Block::default().borders(Borders::ALL).title("ZenTao"));

        f.render_widget(header, chunks[0]);

        let items: Vec<ListItem> = bugs
            .iter()
            .map(|bug| {
                ListItem::new(Line::from(vec![
                    Span::raw(format!("{:6}", bug.id)),
                    Span::raw(" "),
                    Span::raw(&bug.title),
                    Span::raw(" - "),
                    Span::raw(&bug.status),
                ]))
            })
            .collect();

        let list = List::new(items)
            .block(Block::default().borders(Borders::ALL))
            .highlight_style(Style::default().bg(Color::Blue).fg(Color::White));

        f.render_widget(list, chunks[1]);

        let footer = Paragraph::new(Text::from(vec![Line::from(vec![
            Span::raw("Selected: "),
            Span::raw(format!("{}", selected + 1)),
            Span::raw(" / "),
            Span::raw(format!("{}", bugs.len())),
        ])]));

        f.render_widget(footer, chunks[2]);
    }

    fn render_bug_detail(f: &mut ratatui::Frame, area: Rect, bug: &crate::api::Bug) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Length(10),
                Constraint::Min(0),
                Constraint::Length(3),
            ])
            .split(area);

        let title = Paragraph::new(Text::from(vec![Line::from(vec![
            Span::raw(format!("Bug #{} - ", bug.id)),
            Span::styled(&bug.title, Style::default().add_modifier(Modifier::BOLD)),
        ])]))
        .block(Block::default().borders(Borders::ALL).title("Bug Detail"));

        f.render_widget(title, chunks[0]);

        let details = Paragraph::new(Text::from(vec![
            Line::from(vec![Span::raw("Status: "), Span::raw(&bug.status)]),
            Line::from(vec![
                Span::raw("Severity: "),
                Span::raw(format!("{}", bug.severity)),
            ]),
            Line::from(vec![
                Span::raw("Priority: "),
                Span::raw(format!("{}", bug.pri)),
            ]),
            Line::from(vec![
                Span::raw("Resolution: "),
                Span::raw(bug.resolution.as_deref().unwrap_or("N/A")),
            ]),
            Line::from(vec![
                Span::raw("Assigned: "),
                Span::raw(bug.assigned_to.as_deref().unwrap_or("Unassigned")),
            ]),
        ]))
        .block(Block::default().borders(Borders::ALL).title("Details"));

        f.render_widget(details, chunks[1]);

        let steps = Paragraph::new(Text::from(vec![Line::from(Span::raw(
            bug.steps
                .as_deref()
                .unwrap_or("No reproduction steps provided."),
        ))]))
        .block(Block::default().borders(Borders::ALL).title("Steps"));

        f.render_widget(steps, chunks[2]);

        let footer = Paragraph::new(Text::from(vec![Line::from(Span::styled(
            "q - back to list",
            Style::default().fg(Color::DarkGray),
        ))]));

        f.render_widget(footer, chunks[3]);
    }

    fn render_story_list(
        f: &mut ratatui::Frame,
        area: Rect,
        stories: &[crate::api::Story],
        selected: usize,
    ) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(0),
                Constraint::Length(3),
            ])
            .split(area);

        let header = Paragraph::new(Text::from(vec![Line::from(vec![
            Span::raw("Story List ("),
            Span::raw(format!("{}", stories.len())),
            Span::raw(" items) - "),
            Span::styled(
                "↑↓ select | Enter view | q quit",
                Style::default().fg(Color::DarkGray),
            ),
        ])]))
        .block(Block::default().borders(Borders::ALL).title("ZenTao"));

        f.render_widget(header, chunks[0]);

        let items: Vec<ListItem> = stories
            .iter()
            .map(|story| {
                ListItem::new(Line::from(vec![
                    Span::raw(format!("{:6}", story.id)),
                    Span::raw(" "),
                    Span::raw(&story.title),
                    Span::raw(" - "),
                    Span::raw(&story.status),
                ]))
            })
            .collect();

        let list = List::new(items)
            .block(Block::default().borders(Borders::ALL))
            .highlight_style(Style::default().bg(Color::Blue).fg(Color::White));

        f.render_widget(list, chunks[1]);

        let footer = Paragraph::new(Text::from(vec![Line::from(vec![
            Span::raw("Selected: "),
            Span::raw(format!("{}", selected + 1)),
            Span::raw(" / "),
            Span::raw(format!("{}", stories.len())),
        ])]));

        f.render_widget(footer, chunks[2]);
    }

    fn render_story_detail(f: &mut ratatui::Frame, area: Rect, story: &crate::api::Story) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Length(10),
                Constraint::Min(0),
                Constraint::Length(3),
            ])
            .split(area);

        let title = Paragraph::new(Text::from(vec![Line::from(vec![
            Span::raw(format!("Story #{} - ", story.id)),
            Span::styled(&story.title, Style::default().add_modifier(Modifier::BOLD)),
        ])]))
        .block(Block::default().borders(Borders::ALL).title("Story Detail"));

        f.render_widget(title, chunks[0]);

        let details = Paragraph::new(Text::from(vec![
            Line::from(vec![Span::raw("Status: "), Span::raw(&story.status)]),
            Line::from(vec![
                Span::raw("Priority: "),
                Span::raw(format!("{}", story.pri)),
            ]),
            Line::from(vec![
                Span::raw("Stage: "),
                Span::raw(story.stage.as_deref().unwrap_or("N/A")),
            ]),
            Line::from(vec![
                Span::raw("Assigned: "),
                Span::raw(story.assigned_to.as_deref().unwrap_or("Unassigned")),
            ]),
            Line::from(vec![
                Span::raw("Estimate: "),
                Span::raw(
                    story
                        .estimate
                        .map(|e| format!("{}h", e))
                        .unwrap_or_else(|| "N/A".to_string()),
                ),
            ]),
        ]))
        .block(Block::default().borders(Borders::ALL).title("Details"));

        f.render_widget(details, chunks[1]);

        let desc = Paragraph::new(Text::from(vec![Line::from(Span::raw(
            story
                .description
                .as_deref()
                .unwrap_or("No description provided."),
        ))]))
        .block(Block::default().borders(Borders::ALL).title("Description"));

        f.render_widget(desc, chunks[2]);

        let footer = Paragraph::new(Text::from(vec![Line::from(Span::styled(
            "q - back to list",
            Style::default().fg(Color::DarkGray),
        ))]));

        f.render_widget(footer, chunks[3]);
    }

    fn render_error(f: &mut ratatui::Frame, area: Rect, message: &str) {
        let text = Paragraph::new(Text::from(vec![
            Line::from(Span::styled(
                "Error:",
                Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
            )),
            Line::from(Span::raw(message)),
            Line::from(Span::raw("")),
            Line::from(Span::styled(
                "Press q to quit",
                Style::default().fg(Color::DarkGray),
            )),
        ]))
        .block(Block::default().borders(Borders::ALL).title("Error"));

        f.render_widget(text, area);
    }
}
