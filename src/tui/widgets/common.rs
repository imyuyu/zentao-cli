use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph, List, ListItem},
};

pub fn render_bug_list(frame: &mut Frame, area: Rect, items: &[ListItem], selected: usize) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Header
            Constraint::Min(0),      // List
            Constraint::Length(3),  // Footer
        ])
        .split(area);

    // Header
    let header = Paragraph::new(Text::from(vec![
        Line::from(Span::raw("Bug List - ")),
        Line::from(Span::styled("↑↓ select | Enter view | q quit", Style::default().fg(Color::DarkGray))),
    ]))
    .block(Block::default().borders(Borders::ALL).title("ZenTao"));

    frame.render_widget(header, chunks[0]);

    // List
    let list = List::new(items.to_vec())
        .block(Block::default().borders(Borders::ALL))
        .highlight_style(Style::default().bg(Color::Blue).fg(Color::White));

    frame.render_widget(list, chunks[1]);

    // Footer with help
    let footer = Paragraph::new(Text::from(vec![
        Line::from(Span::raw(format!("Selected: {}", selected + 1))),
    ]));

    frame.render_widget(footer, chunks[2]);
}

pub fn render_story_list(frame: &mut Frame, area: Rect, items: &[ListItem], selected: usize) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Header
            Constraint::Min(0),      // List
            Constraint::Length(3),  // Footer
        ])
        .split(area);

    // Header
    let header = Paragraph::new(Text::from(vec![
        Line::from(Span::raw("Story List - ")),
        Line::from(Span::styled("↑↓ select | Enter view | q quit", Style::default().fg(Color::DarkGray))),
    ]))
    .block(Block::default().borders(Borders::ALL).title("ZenTao"));

    frame.render_widget(header, chunks[0]);

    // List
    let list = List::new(items.to_vec())
        .block(Block::default().borders(Borders::ALL))
        .highlight_style(Style::default().bg(Color::Blue).fg(Color::White));

    frame.render_widget(list, chunks[1]);

    // Footer
    let footer = Paragraph::new(Text::from(vec![
        Line::from(Span::raw(format!("Selected: {}", selected + 1))),
    ]));

    frame.render_widget(footer, chunks[2]);
}

pub fn render_bug_detail(frame: &mut Frame, area: Rect, bug: &crate::api::Bug) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),   // Title
            Constraint::Length(10),  // Details
            Constraint::Min(0),      // Steps
            Constraint::Length(3),  // Footer
        ])
        .split(area);

    let title = Paragraph::new(Text::from(vec![
        Line::from(vec![
            Span::raw(format!("Bug #{} - ", bug.id)),
            Span::styled(bug.title.as_str(), Style::default().add_modifier(Modifier::BOLD)),
        ]),
    ]))
    .block(Block::default().borders(Borders::ALL).title("Bug Detail"));

    frame.render_widget(title, chunks[0]);

    let details = Paragraph::new(Text::from(vec![
        Line::from(vec![
            Span::raw("Status: "),
            Span::raw(bug.status.as_str()),
        ]),
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
    ]))
    .block(Block::default().borders(Borders::ALL).title("Details"));

    frame.render_widget(details, chunks[1]);

    let steps = Paragraph::new(Text::from(vec![
        Line::from(Span::raw(bug.steps.as_deref().unwrap_or("No reproduction steps"))),
    ]))
    .block(Block::default().borders(Borders::ALL).title("Steps"));

    frame.render_widget(steps, chunks[2]);

    let footer = Paragraph::new(Text::from(vec![
        Line::from(Span::styled("q - quit", Style::default().fg(Color::DarkGray))),
    ]));

    frame.render_widget(footer, chunks[3]);
}

pub fn render_loading(frame: &mut Frame, area: Rect, message: &str) {
    let text = Paragraph::new(Text::from(vec![
        Line::from(Span::raw(message)),
        Line::from(Span::raw("")),
        Line::from(Span::raw("Loading...")),
    ]))
    .block(Block::default().borders(Borders::ALL).title("Please wait"))
    .style(Style::default().fg(Color::Yellow));

    frame.render_widget(text, area);
}

pub fn render_error(frame: &mut Frame, area: Rect, message: &str) {
    let text = Paragraph::new(Text::from(vec![
        Line::from(Span::styled("Error:", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD))),
        Line::from(Span::raw(message)),
    ]))
    .block(Block::default().borders(Borders::ALL).title("Error"));

    frame.render_widget(text, area);
}
