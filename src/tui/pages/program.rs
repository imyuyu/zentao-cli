use ratatui::Frame;
use ratatui::layout::Rect;
use crate::api::Program;

pub fn render_program_list(
    f: &mut Frame,
    area: Rect,
    programs: &[Program],
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
            "Program List",
            Style::default().add_modifier(Modifier::BOLD),
        ),
        Span::raw(" ("),
        Span::raw(format!("{}", programs.len())),
        Span::raw(" items)"),
    ])]))
    .block(Block::default().borders(Borders::ALL).title("ZenTao"));

    f.render_widget(header, chunks[0]);

    let items: Vec<ListItem> = programs
        .iter()
        .map(|program| {
            ListItem::new(Line::from(vec![
                Span::raw(format!("{:6}", program.id)),
                Span::raw(" "),
                Span::styled(&program.name, Style::default()),
                Span::raw(" ("),
                Span::raw(&program.code),
                Span::raw(") | "),
                Span::styled(
                    format!("[{}]", program.status),
                    match program.status.as_str() {
                        "doing" => Style::default().fg(Color::Green),
                        "closed" => Style::default().fg(Color::Red),
                        "wait" => Style::default().fg(Color::Yellow),
                        _ => Style::default().fg(Color::Blue),
                    },
                ),
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
        Span::raw(format!("Selected: {} / {}", selected + 1, programs.len())),
    ])]));
    f.render_widget(footer, chunks[2]);
}

pub fn render_program_detail(f: &mut Frame, area: Rect, program: &Program) {
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
        Span::raw(format!("Program #{} - ", program.id)),
        Span::styled(&program.name, Style::default().add_modifier(Modifier::BOLD)),
    ])]))
    .block(
        Block::default()
            .borders(Borders::ALL)
            .title("Program Detail"),
    );

    f.render_widget(title, chunks[0]);

    let details = Paragraph::new(Text::from(vec![
        Line::from(vec![
            Span::raw("ID: "),
            Span::raw(format!("{}", program.id)),
        ]),
        Line::from(vec![Span::raw("Name: "), Span::raw(&program.name)]),
        Line::from(vec![Span::raw("Code: "), Span::raw(&program.code)]),
        Line::from(vec![Span::raw("Status: "), Span::raw(&program.status)]),
        Line::from(vec![
            Span::raw("Type: "),
            Span::raw(program.type_.as_deref().unwrap_or("N/A")),
        ]),
        Line::from(vec![
            Span::raw("Parent: "),
            Span::raw(format!(
                "{}",
                program
                    .parent
                    .map(|p| p.to_string())
                    .unwrap_or_else(|| "N/A".to_string())
            )),
        ]),
        Line::from(vec![
            Span::raw("Manager: "),
            Span::raw(program.manager.as_deref().unwrap_or("N/A")),
        ]),
        Line::from(vec![
            Span::raw("Begin: "),
            Span::raw(program.begin.as_deref().unwrap_or("N/A")),
        ]),
        Line::from(vec![
            Span::raw("End: "),
            Span::raw(program.end.as_deref().unwrap_or("N/A")),
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
