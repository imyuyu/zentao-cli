use ratatui::Frame;
use ratatui::layout::Rect;
use crate::api::Ticket;

pub fn render_ticket_list(
    f: &mut Frame,
    area: Rect,
    tickets: &[Ticket],
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
        Span::styled("Ticket List", Style::default().add_modifier(Modifier::BOLD)),
        Span::raw(" ("),
        Span::raw(format!("{}", tickets.len())),
        Span::raw(" items)"),
    ])]))
    .block(Block::default().borders(Borders::ALL).title("ZenTao"));

    f.render_widget(header, chunks[0]);

    let items: Vec<ListItem> = tickets
        .iter()
        .map(|ticket| {
            ListItem::new(Line::from(vec![
                Span::raw(format!("{:6}", ticket.id)),
                Span::raw(" "),
                Span::styled(&ticket.title, Style::default()),
                Span::raw(" | "),
                Span::styled(
                    format!("[{}]", ticket.status),
                    match ticket.status.as_str() {
                        "open" => Style::default().fg(Color::Green),
                        "closed" => Style::default().fg(Color::Red),
                        _ => Style::default().fg(Color::Yellow),
                    },
                ),
                Span::raw(" | "),
                Span::raw(ticket.type_.as_deref().unwrap_or("-")),
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
        Span::raw(format!("Selected: {} / {}", selected + 1, tickets.len())),
    ])]));
    f.render_widget(footer, chunks[2]);
}

pub fn render_ticket_detail(f: &mut Frame, area: Rect, ticket: &Ticket) {
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
            Constraint::Length(14),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(area);

    let title = Paragraph::new(Text::from(vec![Line::from(vec![
        Span::raw(format!("Ticket #{} - ", ticket.id)),
        Span::styled(&ticket.title, Style::default().add_modifier(Modifier::BOLD)),
    ])]))
    .block(
        Block::default()
            .borders(Borders::ALL)
            .title("Ticket Detail"),
    );

    f.render_widget(title, chunks[0]);

    let details = Paragraph::new(Text::from(vec![
        Line::from(vec![Span::raw("Status: "), Span::raw(&ticket.status)]),
        Line::from(vec![
            Span::raw("Type: "),
            Span::raw(ticket.type_.as_deref().unwrap_or("N/A")),
        ]),
        Line::from(vec![
            Span::raw("Priority: "),
            Span::raw(format!(
                "{}",
                ticket
                    .pri
                    .map(|p| p.to_string())
                    .unwrap_or_else(|| "N/A".to_string())
            )),
        ]),
        Line::from(vec![
            Span::raw("Severity: "),
            Span::raw(format!(
                "{}",
                ticket
                    .severity
                    .map(|s| s.to_string())
                    .unwrap_or_else(|| "N/A".to_string())
            )),
        ]),
        Line::from(vec![
            Span::raw("Product: "),
            Span::raw(format!(
                "{}",
                ticket
                    .product
                    .map(|p| p.to_string())
                    .unwrap_or_else(|| "N/A".to_string())
            )),
        ]),
        Line::from(vec![
            Span::raw("Assigned: "),
            Span::raw(ticket.assigned_to.as_deref().unwrap_or("Unassigned")),
        ]),
        Line::from(vec![
            Span::raw("Opened By: "),
            Span::raw(ticket.opened_by.as_deref().unwrap_or("N/A")),
        ]),
        Line::from(vec![
            Span::raw("Resolution: "),
            Span::raw(ticket.resolution.as_deref().unwrap_or("N/A")),
        ]),
        Line::from(vec![
            Span::raw("Ticket Code: "),
            Span::raw(ticket.ticket_code.as_deref().unwrap_or("N/A")),
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
