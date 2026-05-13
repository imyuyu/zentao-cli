use ratatui::Frame;
use ratatui::layout::Rect;
use crate::api::Bug;

pub fn render_bug_list(
    f: &mut Frame,
    area: Rect,
    bugs: &[Bug],
    selected: usize,
    app: &crate::tui::app::App,
) {
    use ratatui::{
        layout::{Constraint, Direction, Layout},
        style::{Color, Modifier, Style},
        text::{Line, Text},
        widgets::{Block, Borders, List, ListItem, Paragraph},
    };

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Header with search
            Constraint::Min(0),  // List
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
        ratatui::text::Span::styled("Bug List", Style::default().add_modifier(Modifier::BOLD)),
        ratatui::text::Span::raw(" ("),
        ratatui::text::Span::raw(format!("{}", bugs.len())),
        ratatui::text::Span::raw(" items)  |  "),
        ratatui::text::Span::styled(&search_hint, Style::default().fg(Color::Cyan)),
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
                ratatui::text::Span::raw(format!("{:6}", bug.id)),
                ratatui::text::Span::raw(" "),
                ratatui::text::Span::styled(&bug.title, Style::default()),
                ratatui::text::Span::raw(" | "),
                ratatui::text::Span::styled(
                    format!("[{}]", bug.status),
                    match bug.status.as_str() {
                        "active" => Style::default().fg(Color::Green),
                        "closed" => Style::default().fg(Color::Red),
                        _ => Style::default().fg(Color::Yellow),
                    },
                ),
                ratatui::text::Span::raw(" | "),
                ratatui::text::Span::styled(format!("Pri:{}", bug.pri), Style::default().fg(Color::Blue)),
                ratatui::text::Span::raw(" | "),
                ratatui::text::Span::styled(
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

    f.render_stateful_widget(list, chunks[1], &mut app.list_state.borrow_mut());

    // Footer with navigation hints
    let footer = Paragraph::new(Text::from(vec![Line::from(vec![
        ratatui::text::Span::styled("↑↓", Style::default().add_modifier(Modifier::BOLD)),
        ratatui::text::Span::raw("/"),
        ratatui::text::Span::styled("jk", Style::default().add_modifier(Modifier::BOLD)),
        ratatui::text::Span::raw(" nav  "),
        ratatui::text::Span::styled("Enter", Style::default().add_modifier(Modifier::BOLD)),
        ratatui::text::Span::raw(" view  "),
        ratatui::text::Span::styled("?", Style::default().add_modifier(Modifier::BOLD)),
        ratatui::text::Span::raw(" help  "),
        ratatui::text::Span::styled("o", Style::default().add_modifier(Modifier::BOLD)),
        ratatui::text::Span::raw(" open  "),
        ratatui::text::Span::styled("q", Style::default().add_modifier(Modifier::BOLD)),
        ratatui::text::Span::raw("/"),
        ratatui::text::Span::styled("Esc", Style::default().add_modifier(Modifier::BOLD)),
        ratatui::text::Span::raw(" quit  |  "),
        ratatui::text::Span::raw(format!("Selected: {} / {}", selected + 1, bugs.len())),
    ])]));
    f.render_widget(footer, chunks[2]);
}

pub fn render_bug_detail(f: &mut Frame, area: Rect, bug: &Bug) {
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

    // Strip HTML tags from steps
    let steps_text = bug
        .steps
        .as_deref()
        .unwrap_or("No reproduction steps provided.");
    let steps_text = crate::tui::browser::strip_html_tags(steps_text);
    // Split by newline to create multiple lines
    let steps_lines: Vec<Line> = steps_text
        .split('\n')
        .map(|s| Line::from(Span::raw(s)))
        .collect();
    let steps = Paragraph::new(Text::from(steps_lines))
        .block(Block::default().borders(Borders::ALL).title("Steps"));

    f.render_widget(steps, chunks[2]);

    let footer = Paragraph::new(Text::from(vec![Line::from(vec![
        Span::styled("q", Style::default().add_modifier(Modifier::BOLD)),
        Span::raw(" back  "),
        Span::styled("o", Style::default().add_modifier(Modifier::BOLD)),
        Span::raw(" open"),
    ])]));

    f.render_widget(footer, chunks[3]);
}
