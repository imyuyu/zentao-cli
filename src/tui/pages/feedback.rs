use crate::api::Feedback;
use ratatui::layout::Rect;
use ratatui::Frame;

pub fn render_feedback_list(
    f: &mut Frame,
    area: Rect,
    feedbacks: &[Feedback],
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
            "Feedback List",
            Style::default().add_modifier(Modifier::BOLD),
        ),
        Span::raw(" ("),
        Span::raw(format!("{}", feedbacks.len())),
        Span::raw(" items)"),
    ])]))
    .block(Block::default().borders(Borders::ALL).title("ZenTao"));

    f.render_widget(header, chunks[0]);

    let items: Vec<ListItem> = feedbacks
        .iter()
        .map(|fb| {
            ListItem::new(Line::from(vec![
                Span::raw(format!("{:6}", fb.id)),
                Span::raw(" "),
                Span::styled(&fb.title, Style::default()),
                Span::raw(" | "),
                Span::styled(
                    format!("[{}]", fb.status),
                    match fb.status.as_str() {
                        "open" => Style::default().fg(Color::Green),
                        "assigned" => Style::default().fg(Color::Yellow),
                        "closed" => Style::default().fg(Color::Red),
                        _ => Style::default().fg(Color::Blue),
                    },
                ),
                Span::raw(" | "),
                Span::raw(&fb.type_),
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
        Span::raw(format!("Selected: {} / {}", selected + 1, feedbacks.len())),
    ])]));
    f.render_widget(footer, chunks[2]);
}

pub fn render_feedback_detail(f: &mut Frame, area: Rect, feedback: &Feedback) {
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
        Span::raw(format!("Feedback #{} - ", feedback.id)),
        Span::styled(
            &feedback.title,
            Style::default().add_modifier(Modifier::BOLD),
        ),
    ])]))
    .block(
        Block::default()
            .borders(Borders::ALL)
            .title("Feedback Detail"),
    );

    f.render_widget(title, chunks[0]);

    let details = Paragraph::new(Text::from(vec![
        Line::from(vec![Span::raw("Status: "), Span::raw(&feedback.status)]),
        Line::from(vec![
            Span::raw("Type: "),
            Span::raw(&feedback.type_),
        ]),
        Line::from(vec![
            Span::raw("Product: "),
            Span::raw(feedback.product.to_string()),
        ]),
        Line::from(vec![
            Span::raw("Assigned: "),
            Span::raw(feedback.assigned_to.as_deref().unwrap_or("Unassigned")),
        ]),
        Line::from(vec![
            Span::raw("Opened By: "),
            Span::raw(feedback.opened_by.realname.as_deref().unwrap_or(feedback.opened_by.account.as_deref().unwrap_or("N/A"))),
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
