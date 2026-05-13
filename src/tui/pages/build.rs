use crate::api::Build;
use ratatui::layout::Rect;
use ratatui::Frame;

pub fn render_build_list(
    f: &mut Frame,
    area: Rect,
    builds: &[Build],
    selected: usize,
    app: &crate::tui::app::App,
) {
    use ratatui::{
        layout::{Constraint, Direction, Layout},
        style::{Color, Modifier, Style},
        text::{Line, Span, Text},
        widgets::{Block, Borders, List, ListItem, Paragraph},
    };

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(area);

    let header = Paragraph::new(Text::from(vec![Line::from(vec![
        Span::styled("Build List", Style::default().add_modifier(Modifier::BOLD)),
        Span::raw(" ("),
        Span::raw(format!("{}", builds.len())),
        Span::raw(" items)"),
    ])]))
    .block(Block::default().borders(Borders::ALL).title("ZenTao"));

    f.render_widget(header, chunks[0]);

    let items: Vec<ListItem> = builds
        .iter()
        .map(|build| {
            ListItem::new(Line::from(vec![
                Span::raw(format!("{:6}", build.id)),
                Span::raw(" "),
                Span::styled(&build.name, Style::default()),
                Span::raw(" | "),
                Span::raw(format!(
                    "Stories: {}",
                    build.stories.as_deref().unwrap_or("0")
                )),
                Span::raw(" | "),
                Span::raw(format!("Bugs: {}", build.bugs.as_deref().unwrap_or("0"))),
            ]))
        })
        .collect();

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL))
        .highlight_style(Style::default().bg(Color::Blue).fg(Color::White));

    f.render_stateful_widget(list, chunks[1], &mut app.list_state.borrow_mut());

    let footer = Paragraph::new(Text::from(vec![Line::from(vec![
        Span::styled("↑↓", Style::default().add_modifier(Modifier::BOLD)),
        Span::raw("/"),
        Span::styled("jk", Style::default().add_modifier(Modifier::BOLD)),
        Span::raw(" nav  "),
        Span::styled("Enter", Style::default().add_modifier(Modifier::BOLD)),
        Span::raw(" view  "),
        Span::styled("q", Style::default().add_modifier(Modifier::BOLD)),
        Span::raw("/"),
        Span::styled("Esc", Style::default().add_modifier(Modifier::BOLD)),
        Span::raw(" back  |  "),
        Span::raw(format!("Selected: {} / {}", selected + 1, builds.len())),
    ])]));
    f.render_widget(footer, chunks[2]);
}

pub fn render_build_detail(f: &mut Frame, area: Rect, build: &Build) {
    use ratatui::{
        layout::{Constraint, Direction, Layout},
        style::{Modifier, Style},
        text::{Line, Span, Text},
        widgets::{Block, Borders, Paragraph},
    };

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(12),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(area);

    let title = Paragraph::new(Text::from(vec![Line::from(vec![
        Span::raw(format!("Build #{} - ", build.id)),
        Span::styled(&build.name, Style::default().add_modifier(Modifier::BOLD)),
    ])]))
    .block(Block::default().borders(Borders::ALL).title("Build Detail"));

    f.render_widget(title, chunks[0]);

    let details = Paragraph::new(Text::from(vec![
        Line::from(vec![
            Span::raw("Product: "),
            Span::raw(format!("{}", build.product)),
        ]),
        Line::from(vec![
            Span::raw("Project: "),
            Span::raw(format!("{}", build.project)),
        ]),
        Line::from(vec![
            Span::raw("Branch: "),
            Span::raw(
                build
                    .branch
                    .map(|b| b.to_string())
                    .unwrap_or_else(|| "N/A".to_string())
                    .to_string(),
            ),
        ]),
        Line::from(vec![
            Span::raw("SCM Path: "),
            Span::raw(build.scm_path.as_deref().unwrap_or("N/A")),
        ]),
        Line::from(vec![
            Span::raw("CI: "),
            Span::raw(build.ci.as_deref().unwrap_or("N/A")),
        ]),
        Line::from(vec![
            Span::raw("Package: "),
            Span::raw(build.pkg.as_deref().unwrap_or("N/A")),
        ]),
        Line::from(vec![
            Span::raw("File Size: "),
            Span::raw(build.file_size.as_deref().unwrap_or("N/A")),
        ]),
        Line::from(vec![
            Span::raw("Generated: "),
            Span::raw(build.generated_at.as_deref().unwrap_or("N/A")),
        ]),
        Line::from(vec![
            Span::raw("Stories: "),
            Span::raw(build.stories.as_deref().unwrap_or("0")),
        ]),
        Line::from(vec![
            Span::raw("Bugs: "),
            Span::raw(build.bugs.as_deref().unwrap_or("0")),
        ]),
    ]))
    .block(Block::default().borders(Borders::ALL).title("Details"));

    f.render_widget(details, chunks[1]);

    let footer = Paragraph::new(Text::from(vec![Line::from(vec![
        Span::styled("q", Style::default().add_modifier(Modifier::BOLD)),
        Span::raw(" back  "),
        Span::styled("o", Style::default().add_modifier(Modifier::BOLD)),
        Span::raw(" open"),
    ])]));

    f.render_widget(footer, chunks[3]);
}
