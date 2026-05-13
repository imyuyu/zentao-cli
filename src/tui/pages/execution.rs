use ratatui::Frame;
use ratatui::layout::Rect;
use crate::api::Execution;

pub fn render_execution_list(
    f: &mut Frame,
    area: Rect,
    executions: &[Execution],
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
        Span::styled(
            "Execution List",
            Style::default().add_modifier(Modifier::BOLD),
        ),
        Span::raw(" ("),
        Span::raw(format!("{}", executions.len())),
        Span::raw(" items)"),
    ])]))
    .block(Block::default().borders(Borders::ALL).title("ZenTao"));

    f.render_widget(header, chunks[0]);

    let items: Vec<ListItem> = executions
        .iter()
        .map(|exec| {
            ListItem::new(Line::from(vec![
                Span::raw(format!("{:6}", exec.id)),
                Span::raw(" "),
                Span::styled(&exec.name, Style::default()),
                Span::raw(" | "),
                Span::styled(
                    format!("[{}]", exec.status),
                    match exec.status.as_str() {
                        "doing" => Style::default().fg(Color::Green),
                        "closed" => Style::default().fg(Color::Red),
                        "suspended" => Style::default().fg(Color::Yellow),
                        _ => Style::default().fg(Color::Blue),
                    },
                ),
                Span::raw(" | "),
                Span::raw(exec.begin.clone().unwrap_or_else(|| "-".to_string())),
                Span::raw(" ~ "),
                Span::raw(exec.end.clone().unwrap_or_else(|| "-".to_string())),
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
        Span::raw(format!("Selected: {} / {}", selected + 1, executions.len())),
    ])]));
    f.render_widget(footer, chunks[2]);
}

pub fn render_execution_detail(
    f: &mut Frame,
    area: Rect,
    execution: &Execution,
) {
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
        Span::raw(format!("Execution #{} - ", execution.id)),
        Span::styled(
            &execution.name,
            Style::default().add_modifier(Modifier::BOLD),
        ),
    ])]))
    .block(
        Block::default()
            .borders(Borders::ALL)
            .title("Execution Detail"),
    );

    f.render_widget(title, chunks[0]);

    let details = Paragraph::new(Text::from(vec![
        Line::from(vec![Span::raw("Status: "), Span::raw(&execution.status)]),
        Line::from(vec![
            Span::raw("Type: "),
            Span::raw(execution.type_.as_deref().unwrap_or("N/A")),
        ]),
        Line::from(vec![
            Span::raw("Project: "),
            Span::raw(format!("{}", execution.project)),
        ]),
        Line::from(vec![
            Span::raw("Begin: "),
            Span::raw(execution.begin.as_deref().unwrap_or("N/A")),
        ]),
        Line::from(vec![
            Span::raw("End: "),
            Span::raw(execution.end.as_deref().unwrap_or("N/A")),
        ]),
        Line::from(vec![
            Span::raw("Days: "),
            Span::raw(execution
                    .days
                    .map(|d| d.to_string())
                    .unwrap_or_else(|| "N/A".to_string()).to_string()),
        ]),
    ]))
    .block(Block::default().borders(Borders::ALL).title("Details"));

    f.render_widget(details, chunks[1]);

    let desc = Paragraph::new(Text::from(vec![Line::from(Span::raw(
        execution
            .desc
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
