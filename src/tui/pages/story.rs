use crate::api::Story;
use ratatui::layout::Rect;
use ratatui::Frame;

pub fn render_story_list(
    f: &mut Frame,
    area: Rect,
    stories: &[Story],
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
                Span::raw(
                    story
                        .estimate
                        .map(|e| format!("{}h", e))
                        .unwrap_or_default()
                        .to_string(),
                ),
            ]))
        })
        .collect();

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL))
        .highlight_style(Style::default().bg(Color::Blue).fg(Color::White));

    f.render_stateful_widget(list, chunks[1], &mut app.list_state.borrow_mut());

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
        Span::styled("o", Style::default().add_modifier(Modifier::BOLD)),
        Span::raw(" open  "),
        Span::styled("q", Style::default().add_modifier(Modifier::BOLD)),
        Span::raw("/"),
        Span::styled("Esc", Style::default().add_modifier(Modifier::BOLD)),
        Span::raw(" quit  |  "),
        Span::raw(format!("Selected: {} / {}", selected + 1, stories.len())),
    ])]));
    f.render_widget(footer, chunks[2]);
}

pub fn render_story_detail(f: &mut Frame, area: Rect, story: &Story) {
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

    let footer = Paragraph::new(Text::from(vec![Line::from(vec![
        Span::styled("q", Style::default().add_modifier(Modifier::BOLD)),
        Span::raw(" back  "),
        Span::styled("o", Style::default().add_modifier(Modifier::BOLD)),
        Span::raw(" open"),
    ])]));

    f.render_widget(footer, chunks[3]);
}
