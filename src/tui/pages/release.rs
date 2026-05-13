use ratatui::Frame;
use ratatui::layout::Rect;
use crate::api::Release;

pub fn render_release_list(
    f: &mut Frame,
    area: Rect,
    releases: &[Release],
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
            "Release List",
            Style::default().add_modifier(Modifier::BOLD),
        ),
        Span::raw(" ("),
        Span::raw(format!("{}", releases.len())),
        Span::raw(" items)"),
    ])]))
    .block(Block::default().borders(Borders::ALL).title("ZenTao"));

    f.render_widget(header, chunks[0]);

    let items: Vec<ListItem> = releases
        .iter()
        .map(|release| {
            ListItem::new(Line::from(vec![
                Span::raw(format!("{:6}", release.id)),
                Span::raw(" "),
                Span::styled(&release.name, Style::default()),
                Span::raw(" | "),
                Span::styled(
                    format!("[{}]", release.status),
                    match release.status.as_str() {
                        "normal" => Style::default().fg(Color::Green),
                        "closed" => Style::default().fg(Color::Red),
                        _ => Style::default().fg(Color::Yellow),
                    },
                ),
                Span::raw(" | "),
                Span::raw(release.marker.as_deref().unwrap_or("-")),
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
        Span::raw(format!("Selected: {} / {}", selected + 1, releases.len())),
    ])]));
    f.render_widget(footer, chunks[2]);
}

pub fn render_release_detail(f: &mut Frame, area: Rect, release: &Release) {
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
        Span::raw(format!("Release #{} - ", release.id)),
        Span::styled(&release.name, Style::default().add_modifier(Modifier::BOLD)),
    ])]))
    .block(
        Block::default()
            .borders(Borders::ALL)
            .title("Release Detail"),
    );

    f.render_widget(title, chunks[0]);

    let details = Paragraph::new(Text::from(vec![
        Line::from(vec![Span::raw("Status: "), Span::raw(&release.status)]),
        Line::from(vec![
            Span::raw("Product: "),
            Span::raw(
                release
                    .product
                    .map(|p| p.to_string())
                    .unwrap_or_else(|| "N/A".to_string()),
            ),
        ]),
        Line::from(vec![
            Span::raw("Build: "),
            Span::raw(release
                    .build
                    .map(|b| b.to_string())
                    .unwrap_or_else(|| "N/A".to_string()).to_string()),
        ]),
        Line::from(vec![
            Span::raw("Marker: "),
            Span::raw(release.marker.as_deref().unwrap_or("N/A")),
        ]),
        Line::from(vec![
            Span::raw("Date: "),
            Span::raw(release.date.as_deref().unwrap_or("N/A")),
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
