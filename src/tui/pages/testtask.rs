use ratatui::Frame;
use ratatui::layout::Rect;
use crate::api::Testtask;

pub fn render_testtask_list(
    f: &mut Frame,
    area: Rect,
    testtasks: &[Testtask],
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
            "Testtask List",
            Style::default().add_modifier(Modifier::BOLD),
        ),
        Span::raw(" ("),
        Span::raw(format!("{}", testtasks.len())),
        Span::raw(" items)"),
    ])]))
    .block(Block::default().borders(Borders::ALL).title("ZenTao"));

    f.render_widget(header, chunks[0]);

    let items: Vec<ListItem> = testtasks
        .iter()
        .map(|tt| {
            ListItem::new(Line::from(vec![
                Span::raw(format!("{:6}", tt.id)),
                Span::raw(" "),
                Span::styled(&tt.name, Style::default()),
                Span::raw(" | "),
                Span::styled(
                    format!("[{}]", tt.status),
                    match tt.status.as_str() {
                        "done" => Style::default().fg(Color::Green),
                        "closed" => Style::default().fg(Color::Red),
                        "doing" => Style::default().fg(Color::Yellow),
                        _ => Style::default().fg(Color::Blue),
                    },
                ),
                Span::raw(" | "),
                Span::raw(format!(
                    "Cases: {}",
                    tt.case_count.as_deref().unwrap_or("0")
                )),
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
        Span::raw(format!("Selected: {} / {}", selected + 1, testtasks.len())),
    ])]));
    f.render_widget(footer, chunks[2]);
}

pub fn render_testtask_detail(f: &mut Frame, area: Rect, testtask: &Testtask) {
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
        Span::raw(format!("Testtask #{} - ", testtask.id)),
        Span::styled(
            &testtask.name,
            Style::default().add_modifier(Modifier::BOLD),
        ),
    ])]))
    .block(
        Block::default()
            .borders(Borders::ALL)
            .title("Testtask Detail"),
    );

    f.render_widget(title, chunks[0]);

    let details = Paragraph::new(Text::from(vec![
        Line::from(vec![Span::raw("Status: "), Span::raw(&testtask.status)]),
        Line::from(vec![
            Span::raw("Type: "),
            Span::raw(testtask.type_.as_deref().unwrap_or("N/A")),
        ]),
        Line::from(vec![
            Span::raw("Project: "),
            Span::raw(format!("{}", testtask.project)),
        ]),
        Line::from(vec![
            Span::raw("Product: "),
            Span::raw(format!(
                "{}",
                testtask
                    .product
                    .map(|p| p.to_string())
                    .unwrap_or_else(|| "N/A".to_string())
            )),
        ]),
        Line::from(vec![
            Span::raw("Assigned: "),
            Span::raw(testtask.assigned_to.as_deref().unwrap_or("Unassigned")),
        ]),
        Line::from(vec![
            Span::raw("Begin: "),
            Span::raw(testtask.begin.as_deref().unwrap_or("N/A")),
        ]),
        Line::from(vec![
            Span::raw("End: "),
            Span::raw(testtask.end.as_deref().unwrap_or("N/A")),
        ]),
        Line::from(vec![
            Span::raw("Cases: "),
            Span::raw(testtask.case_count.as_deref().unwrap_or("0")),
        ]),
        Line::from(vec![
            Span::raw("Passed: "),
            Span::raw(testtask.passed_count.as_deref().unwrap_or("0")),
        ]),
        Line::from(vec![
            Span::raw("Failed: "),
            Span::raw(testtask.failed_count.as_deref().unwrap_or("0")),
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
