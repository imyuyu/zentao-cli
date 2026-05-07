use anyhow::Result;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
    Terminal,
};
use std::io::Stdout;

use super::app::{App, AppState};
use crate::api::Product;

pub struct Browser {
    terminal: Terminal<CrosstermBackend<Stdout>>,
    pending_products: Option<Vec<Product>>,
    pending_reload: Option<Box<dyn FnOnce(&mut App)>>,
}

impl Browser {
    pub fn new() -> Result<Self> {
        let stdout = std::io::stdout();
        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend)?;
        Ok(Self {
            terminal,
            pending_products: None,
            pending_reload: None,
        })
    }

    pub fn run(&mut self, app: &mut App) -> Result<()> {
        loop {
            if app.state.is_quitting() {
                break;
            }

            let selected = app.selected_index;
            self.terminal.draw(|f| {
                let area = f.size();
                if app.help_visible {
                    Self::render_help_overlay(f, area);
                } else {
                    match &app.state {
                        AppState::Idle => {
                            Self::render_idle(f, area, app);
                        }
                        AppState::Loading { message } => {
                            Self::render_loading(f, area, message);
                        }
                        AppState::BugList { bugs } => {
                            Self::render_bug_list(f, area, bugs, selected, app);
                        }
                        AppState::BugDetail { bug } => {
                            Self::render_bug_detail(f, area, bug);
                        }
                        AppState::StoryList { stories } => {
                            Self::render_story_list(f, area, stories, selected, app);
                        }
                        AppState::StoryDetail { story } => {
                            Self::render_story_detail(f, area, story);
                        }
                        AppState::Error { message } => {
                            Self::render_error(f, area, message);
                        }
                        AppState::Quit => {}
                        AppState::Settings {
                            multi_config,
                            selected,
                            current_account,
                        } => {
                            Self::render_settings(
                                f,
                                area,
                                multi_config,
                                *selected,
                                current_account,
                                app,
                            );
                        }
                        AppState::ProductSelect {
                            products,
                            selected,
                            loading,
                        } => {
                            Self::render_product_select(f, area, products, *selected, *loading);
                        }
                        AppState::AccountSelect {
                            multi_config,
                            selected,
                        } => {
                            Self::render_account_select(f, area, multi_config, *selected, app);
                        }
                    }
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
        // Handle help overlay first
        if app.help_visible {
            if key.code == KeyCode::Char('?') || key.code == KeyCode::Esc {
                app.help_visible = false;
            }
            return;
        }

        // Handle search mode
        if app.search_active {
            match key.code {
                KeyCode::Esc => {
                    app.search_active = false;
                    app.search_query.clear();
                }
                KeyCode::Enter => {
                    app.search_active = false;
                }
                KeyCode::Backspace => {
                    app.search_query.pop();
                }
                KeyCode::Char(c) => {
                    app.search_query.push(c);
                }
                _ => {}
            }
            return;
        }

        // Handle Ctrl+F to activate search
        if key.code == KeyCode::Char('f') && key.modifiers.contains(KeyModifiers::CONTROL) {
            app.search_active = true;
            return;
        }

        // Handle ? for help
        if key.code == KeyCode::Char('?') {
            app.help_visible = true;
            return;
        }

        // Handle 'p' for product select
        if key.code == KeyCode::Char('p') {
            let products = self.pending_products.take();
            if let Some(products) = products {
                app.state = AppState::ProductSelect {
                    products,
                    selected: 0,
                    loading: false,
                };
            }
            return;
        }

        // Handle 's' for settings
        if key.code == KeyCode::Char('s') {
            let current_account = app.config.account.clone().unwrap_or_default();
            app.state = AppState::Settings {
                multi_config: app.multi_config.clone(),
                selected: 0,
                current_account,
            };
            return;
        }

        match key.code {
            KeyCode::Up | KeyCode::Char('k') => {
                app.selected_index = app.selected_index.saturating_sub(1);
            }
            KeyCode::Down | KeyCode::Char('j') => {
                let max = match &app.state {
                    AppState::BugList { bugs } => bugs.len().saturating_sub(1),
                    AppState::StoryList { stories } => stories.len().saturating_sub(1),
                    AppState::Settings { multi_config, .. } => {
                        multi_config.list_account_names().len().saturating_sub(1)
                    }
                    AppState::ProductSelect { products, .. } => products.len().saturating_sub(1),
                    AppState::AccountSelect { multi_config, .. } => {
                        multi_config.list_account_names().len().saturating_sub(1)
                    }
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
                AppState::Settings { .. }
                | AppState::ProductSelect { .. }
                | AppState::AccountSelect { .. } => {
                    // Close settings/product/account panel
                    app.state = AppState::Idle;
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

    fn render_help_overlay(f: &mut ratatui::Frame, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(0),
                Constraint::Length(25),
                Constraint::Min(0),
            ])
            .split(area);

        let help_text = vec![
            Line::from(Span::styled(
                "Keyboard Shortcuts",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            )),
            Line::from(Span::raw("")),
            Line::from(vec![
                Span::raw("  "),
                Span::styled("↑/k", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw("    Move selection up"),
            ]),
            Line::from(vec![
                Span::raw("  "),
                Span::styled("↓/j", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw("    Move selection down"),
            ]),
            Line::from(vec![
                Span::raw("  "),
                Span::styled("Enter", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw("    View selected item details"),
            ]),
            Line::from(vec![
                Span::raw("  "),
                Span::styled("Esc/q", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw("    Back / Quit"),
            ]),
            Line::from(vec![
                Span::raw("  "),
                Span::styled("r", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw("      Refresh list"),
            ]),
            Line::from(vec![
                Span::raw("  "),
                Span::styled("Ctrl+F", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw("  Activate search"),
            ]),
            Line::from(vec![
                Span::raw("  "),
                Span::styled("?", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw("      Toggle this help overlay"),
            ]),
            Line::from(Span::raw("")),
            Line::from(Span::styled(
                "Press any key to close",
                Style::default().fg(Color::DarkGray),
            )),
        ];

        let paragraph = Paragraph::new(Text::from(help_text))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Help")
                    .title_style(Style::default().fg(Color::Cyan)),
            )
            .wrap(Wrap { trim: true })
            .alignment(ratatui::layout::Alignment::Left);

        f.render_widget(paragraph, area);
    }

    fn render_idle(f: &mut ratatui::Frame, area: Rect, app: &App) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(0),
                Constraint::Length(3),
            ])
            .split(area);

        // Header with search bar hint
        let search_hint = if app.search_active {
            format!("[SEARCH: {}] (Esc to cancel)", app.search_query)
        } else {
            "[Ctrl+F] Search".to_string()
        };
        let header = Paragraph::new(Text::from(vec![Line::from(vec![
            Span::styled("ZenTao CLI", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw("  |  "),
            Span::styled(&search_hint, Style::default().fg(Color::Cyan)),
        ])]))
        .block(Block::default().borders(Borders::ALL).title("ZenTao"));

        f.render_widget(header, chunks[0]);

        let text = Paragraph::new(Text::from(vec![
            Line::from(Span::raw("")),
            Line::from(Span::raw(
                "Use commands: zentao bug browse, zentao story browse",
            )),
            Line::from(Span::raw("")),
            Line::from(Span::styled(
                "Press ? for help",
                Style::default().fg(Color::DarkGray),
            )),
        ]))
        .block(Block::default().borders(Borders::ALL));

        f.render_widget(text, chunks[1]);

        // Footer with shortcuts hint
        let footer = Paragraph::new(Text::from(vec![Line::from(vec![
            Span::styled("?", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" help  "),
            Span::styled("Ctrl+F", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" search  "),
            Span::styled("q", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" quit"),
        ])]));
        f.render_widget(footer, chunks[2]);
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
        app: &App,
    ) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Header with search
                Constraint::Min(0),    // List
                Constraint::Length(3), // Footer with shortcuts
            ])
            .split(area);

        // Search hint in header
        let search_hint = if app.search_active {
            format!("[SEARCH: {}] (Esc to cancel)", app.search_query)
        } else {
            "[Ctrl+F] Search".to_string()
        };
        let header = Paragraph::new(Text::from(vec![Line::from(vec![
            Span::styled("Bug List", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" ("),
            Span::raw(format!("{}", bugs.len())),
            Span::raw(" items)  |  "),
            Span::styled(&search_hint, Style::default().fg(Color::Cyan)),
        ])]))
        .block(Block::default().borders(Borders::ALL).title("ZenTao"));

        f.render_widget(header, chunks[0]);

        // Filter bugs if search is active
        let display_bugs: Vec<_> = if app.search_query.is_empty() {
            bugs.iter().collect()
        } else {
            let query = app.search_query.to_lowercase();
            bugs.iter()
                .filter(|b| {
                    b.title.to_lowercase().contains(&query)
                        || b.status.to_lowercase().contains(&query)
                        || format!("{}", b.id).contains(&query)
                })
                .collect()
        };

        let items: Vec<ListItem> = display_bugs
            .iter()
            .map(|bug| {
                ListItem::new(Line::from(vec![
                    Span::raw(format!("{:6}", bug.id)),
                    Span::raw(" "),
                    Span::styled(&bug.title, Style::default()),
                    Span::raw(" | "),
                    Span::styled(
                        format!("[{}]", bug.status),
                        match bug.status.as_str() {
                            "active" => Style::default().fg(Color::Green),
                            "closed" => Style::default().fg(Color::Red),
                            _ => Style::default().fg(Color::Yellow),
                        },
                    ),
                    Span::raw(" | "),
                    Span::styled(format!("Pri:{}", bug.pri), Style::default().fg(Color::Blue)),
                    Span::raw(" | "),
                    Span::styled(
                        format!("Sev:{}", bug.severity),
                        match bug.severity {
                            1 => Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
                            2 => Style::default().fg(Color::Yellow),
                            _ => Style::default().fg(Color::DarkGray),
                        },
                    ),
                ]))
            })
            .collect();

        let list = List::new(items)
            .block(Block::default().borders(Borders::ALL))
            .highlight_style(Style::default().bg(Color::Blue).fg(Color::White));

        f.render_widget(list, chunks[1]);

        // Footer with navigation hints
        let footer = Paragraph::new(Text::from(vec![Line::from(vec![
            Span::styled("↑↓", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw("/"),
            Span::styled("jk", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" nav  "),
            Span::styled("Enter", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" view  "),
            Span::styled("?", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" help  "),
            Span::styled("q", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw("/"),
            Span::styled("Esc", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" quit  |  "),
            Span::raw(format!("Selected: {} / {}", selected + 1, bugs.len())),
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
        app: &App,
    ) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Header with search
                Constraint::Min(0),    // List
                Constraint::Length(3), // Footer with shortcuts
            ])
            .split(area);

        // Search hint in header
        let search_hint = if app.search_active {
            format!("[SEARCH: {}] (Esc to cancel)", app.search_query)
        } else {
            "[Ctrl+F] Search".to_string()
        };
        let header = Paragraph::new(Text::from(vec![Line::from(vec![
            Span::styled("Story List", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" ("),
            Span::raw(format!("{}", stories.len())),
            Span::raw(" items)  |  "),
            Span::styled(&search_hint, Style::default().fg(Color::Cyan)),
        ])]))
        .block(Block::default().borders(Borders::ALL).title("ZenTao"));

        f.render_widget(header, chunks[0]);

        // Filter stories if search is active
        let display_stories: Vec<_> = if app.search_query.is_empty() {
            stories.iter().collect()
        } else {
            let query = app.search_query.to_lowercase();
            stories
                .iter()
                .filter(|s| {
                    s.title.to_lowercase().contains(&query)
                        || s.status.to_lowercase().contains(&query)
                        || format!("{}", s.id).contains(&query)
                })
                .collect()
        };

        let items: Vec<ListItem> = display_stories
            .iter()
            .map(|story| {
                ListItem::new(Line::from(vec![
                    Span::raw(format!("{:6}", story.id)),
                    Span::raw(" "),
                    Span::styled(&story.title, Style::default()),
                    Span::raw(" | "),
                    Span::styled(
                        format!("[{}]", story.status),
                        match story.status.as_str() {
                            "active" => Style::default().fg(Color::Green),
                            "closed" => Style::default().fg(Color::Red),
                            _ => Style::default().fg(Color::Yellow),
                        },
                    ),
                    Span::raw(" | "),
                    Span::styled(
                        format!("Pri:{}", story.pri),
                        Style::default().fg(Color::Blue),
                    ),
                    Span::raw(" | "),
                    Span::raw(format!(
                        "{}",
                        story
                            .estimate
                            .map(|e| format!("{}h", e))
                            .unwrap_or_default()
                    )),
                ]))
            })
            .collect();

        let list = List::new(items)
            .block(Block::default().borders(Borders::ALL))
            .highlight_style(Style::default().bg(Color::Blue).fg(Color::White));

        f.render_widget(list, chunks[1]);

        // Footer with navigation hints
        let footer = Paragraph::new(Text::from(vec![Line::from(vec![
            Span::styled("↑↓", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw("/"),
            Span::styled("jk", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" nav  "),
            Span::styled("Enter", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" view  "),
            Span::styled("?", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" help  "),
            Span::styled("q", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw("/"),
            Span::styled("Esc", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" quit  |  "),
            Span::raw(format!("Selected: {} / {}", selected + 1, stories.len())),
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

    fn render_settings(
        f: &mut ratatui::Frame,
        area: Rect,
        multi_config: &crate::core::config::MultiAccountConfig,
        selected: usize,
        current_account: &str,
        app: &App,
    ) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Header
                Constraint::Min(0),    // Content
                Constraint::Length(3), // Footer
            ])
            .split(area);

        // Header
        let header = Paragraph::new(Text::from(vec![Line::from(vec![
            Span::styled("Settings", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" - Press q/ESC to close"),
        ])]))
        .block(Block::default().borders(Borders::ALL).title("ZenTao"));

        f.render_widget(header, chunks[0]);

        // Settings content
        let accounts: Vec<&String> = multi_config.list_account_names();
        let items: Vec<ListItem> = accounts
            .iter()
            .map(|name| {
                let is_current = *name == current_account;
                let suffix = if is_current { " (current)" } else { "" };
                ListItem::new(Line::from(vec![Span::styled(
                    format!("{}{}", name, suffix),
                    if is_current {
                        Style::default()
                            .add_modifier(Modifier::BOLD)
                            .fg(Color::Green)
                    } else {
                        Style::default()
                    },
                )]))
            })
            .collect();

        let settings_text = vec![
            Line::from(Span::raw("")),
            Line::from(vec![Span::styled(
                "Accounts",
                Style::default().add_modifier(Modifier::BOLD),
            )]),
            Line::from(Span::raw("")),
        ];

        let settings_para =
            Paragraph::new(Text::from(settings_text)).block(Block::default().borders(Borders::ALL));

        let list = if items.is_empty() {
            Paragraph::new(Text::from(vec![Line::from(Span::styled(
                "No accounts configured. Run 'zentao auth login' first.",
                Style::default().fg(Color::DarkGray),
            ))]))
        } else {
            Paragraph::new(Text::from(vec![]))
        };

        let content = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(chunks[1]);

        let left = Paragraph::new(Text::from(vec![
            Line::from(Span::raw("")),
            Line::from(vec![Span::styled(
                "Accounts:",
                Style::default().add_modifier(Modifier::BOLD),
            )]),
            Line::from(Span::raw("")),
        ]))
        .block(Block::default().borders(Borders::ALL));

        let account_items: Vec<ListItem> = accounts
            .iter()
            .map(|name| {
                let is_current = *name == current_account;
                ListItem::new(Line::from(vec![
                    if is_current {
                        Span::styled("> ", Style::default().fg(Color::Green))
                    } else {
                        Span::raw("  ")
                    },
                    Span::raw(*name),
                    if is_current {
                        Span::styled(" (current)", Style::default().fg(Color::DarkGray))
                    } else {
                        Span::raw("")
                    },
                ]))
            })
            .collect();

        let list = List::new(account_items)
            .block(Block::default().borders(Borders::ALL))
            .highlight_style(Style::default().bg(Color::Blue).fg(Color::White));

        f.render_widget(left, content[0]);
        f.render_widget(list, content[0]);

        let right = Paragraph::new(Text::from(vec![
            Line::from(Span::raw("")),
            Line::from(vec![Span::styled(
                "Current Config:",
                Style::default().add_modifier(Modifier::BOLD),
            )]),
            Line::from(Span::raw("")),
            Line::from(Span::raw(format!("URL: {}", app.config.url))),
            Line::from(Span::raw(format!("Account: {}", current_account))),
            Line::from(Span::raw(format!(
                "Product ID: {:?}",
                app.config.product_id
            ))),
        ]))
        .block(Block::default().borders(Borders::ALL));

        f.render_widget(right, content[1]);

        // Footer
        let footer = Paragraph::new(Text::from(vec![Line::from(vec![
            Span::styled("↑↓", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" select account  "),
            Span::styled("Enter", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" switch  "),
            Span::styled("q", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" close"),
        ])]));
        f.render_widget(footer, chunks[2]);
    }

    fn render_product_select(
        f: &mut ratatui::Frame,
        area: Rect,
        products: &[Product],
        selected: usize,
        loading: bool,
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
            Span::styled(
                "Select Product",
                Style::default().add_modifier(Modifier::BOLD),
            ),
            Span::raw(" - Press q/ESC to close"),
        ])]))
        .block(Block::default().borders(Borders::ALL).title("ZenTao"));

        f.render_widget(header, chunks[0]);

        if loading {
            let loading_text = Paragraph::new(Text::from(vec![Line::from(Span::styled(
                "Loading products...",
                Style::default().fg(Color::Yellow),
            ))]))
            .block(Block::default().borders(Borders::ALL));
            f.render_widget(loading_text, chunks[1]);
        } else if products.is_empty() {
            let empty_text = Paragraph::new(Text::from(vec![Line::from(Span::styled(
                "No products available",
                Style::default().fg(Color::DarkGray),
            ))]))
            .block(Block::default().borders(Borders::ALL));
            f.render_widget(empty_text, chunks[1]);
        } else {
            let items: Vec<ListItem> = products
                .iter()
                .map(|p| {
                    ListItem::new(Line::from(vec![
                        Span::raw(format!("{:6}", p.id)),
                        Span::raw(" "),
                        Span::styled(&p.name, Style::default()),
                        Span::raw(" ("),
                        Span::raw(&p.code),
                        Span::raw(")"),
                    ]))
                })
                .collect();

            let list = List::new(items)
                .block(Block::default().borders(Borders::ALL))
                .highlight_style(Style::default().bg(Color::Blue).fg(Color::White));

            f.render_widget(list, chunks[1]);
        }

        let footer = Paragraph::new(Text::from(vec![Line::from(vec![
            Span::styled("↑↓", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" select  "),
            Span::styled("Enter", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" confirm  "),
            Span::styled("q", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" close"),
        ])]));
        f.render_widget(footer, chunks[2]);
    }

    fn render_account_select(
        f: &mut ratatui::Frame,
        area: Rect,
        multi_config: &crate::core::config::MultiAccountConfig,
        selected: usize,
        app: &App,
    ) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(0),
                Constraint::Length(3),
            ])
            .split(area);

        let accounts: Vec<&String> = multi_config.list_account_names();

        let header = Paragraph::new(Text::from(vec![Line::from(vec![
            Span::styled(
                "Select Account",
                Style::default().add_modifier(Modifier::BOLD),
            ),
            Span::raw(" - Press q/ESC to close"),
        ])]))
        .block(Block::default().borders(Borders::ALL).title("ZenTao"));

        f.render_widget(header, chunks[0]);

        if accounts.is_empty() {
            let empty_text = Paragraph::new(Text::from(vec![Line::from(Span::styled(
                "No accounts configured. Run 'zentao auth login' first.",
                Style::default().fg(Color::DarkGray),
            ))]))
            .block(Block::default().borders(Borders::ALL));
            f.render_widget(empty_text, chunks[1]);
        } else {
            let items: Vec<ListItem> = accounts
                .iter()
                .map(|name| {
                    let is_default = multi_config.default_account.as_deref() == Some(*name);
                    ListItem::new(Line::from(vec![
                        if is_default {
                            Span::styled("* ", Style::default().fg(Color::Green))
                        } else {
                            Span::raw("  ")
                        },
                        Span::raw(*name),
                    ]))
                })
                .collect();

            let list = List::new(items)
                .block(Block::default().borders(Borders::ALL))
                .highlight_style(Style::default().bg(Color::Blue).fg(Color::White));

            f.render_widget(list, chunks[1]);
        }

        let footer = Paragraph::new(Text::from(vec![Line::from(vec![
            Span::styled("↑↓", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" select  "),
            Span::styled("Enter", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" switch  "),
            Span::styled("q", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" close"),
        ])]));
        f.render_widget(footer, chunks[2]);
    }
}
