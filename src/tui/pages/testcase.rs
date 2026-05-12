use ratatui::Frame;
use ratatui::layout::Rect;
use crate::api::Testcase;

pub fn render_testcase_list(
    f: &mut Frame,
    area: Rect,
    testcases: &[Testcase],
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
            "Testcase List",
            Style::default().add_modifier(Modifier::BOLD),
        ),
        Span::raw(" ("),
        Span::raw(format!("{}", testcases.len())),
        Span::raw(" items)"),
    ])]))
    .block(Block::default().borders(Borders::ALL).title("ZenTao"));

    f.render_widget(header, chunks[0]);

    let items: Vec<ListItem> = testcases
        .iter()
        .map(|tc| {
            ListItem::new(Line::from(vec![
                Span::raw(format!("{:6}", tc.id)),
                Span::raw(" "),
                Span::styled(&tc.title, Style::default()),
                Span::raw(" | "),
                Span::styled(
                    format!("[{}]", tc.status),
                    match tc.status.as_str() {
                        "normal" => Style::default().fg(Color::Green),
                        "blocked" => Style::default().fg(Color::Red),
                        "bypass" => Style::default().fg(Color::Yellow),
                        _ => Style::default().fg(Color::Blue),
                    },
                ),
                Span::raw(" | "),
                Span::styled(
                    format!(
                        "Sev:{}",
                        tc.severity
                            .map(|s| s.to_string())
                            .unwrap_or_else(|| "N/A".to_string())
                    ),
                    match tc.severity {
                        Some(1) => Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
                        Some(2) => Style::default().fg(Color::Yellow),
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
        Span::raw(format!("Selected: {} / {}", selected + 1, testcases.len())),
    ])]));
    f.render_widget(footer, chunks[2]);
}

pub fn render_testcase_detail(f: &mut Frame, area: Rect, testcase: &Testcase) {
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
        Span::raw(format!("Testcase #{} - ", testcase.id)),
        Span::styled(
            &testcase.title,
            Style::default().add_modifier(Modifier::BOLD),
        ),
    ])]))
    .block(
        Block::default()
            .borders(Borders::ALL)
            .title("Testcase Detail"),
    );

    f.render_widget(title, chunks[0]);

    let details = Paragraph::new(Text::from(vec![
        Line::from(vec![Span::raw("Status: "), Span::raw(&testcase.status)]),
        Line::from(vec![
            Span::raw("Type: "),
            Span::raw(testcase.type_.as_deref().unwrap_or("N/A")),
        ]),
        Line::from(vec![
            Span::raw("Severity: "),
            Span::raw(
                testcase
                    .severity
                    .map(|s| s.to_string())
                    .unwrap_or_else(|| "N/A".to_string()),
            ),
        ]),
        Line::from(vec![
            Span::raw("Priority: "),
            Span::raw(
                testcase
                    .pri
                    .map(|p| p.to_string())
                    .unwrap_or_else(|| "N/A".to_string()),
            ),
        ]),
        Line::from(vec![
            Span::raw("Product: "),
            Span::raw(format!("{}", testcase.product)),
        ]),
        Line::from(vec![
            Span::raw("Version: "),
            Span::raw(format!(
                "{}",
                testcase
                    .version
                    .map(|v| v.to_string())
                    .unwrap_or_else(|| "N/A".to_string())
            )),
        ]),
    ]))
    .block(Block::default().borders(Borders::ALL).title("Details"));

    f.render_widget(details, chunks[1]);

    // Strip HTML tags from steps
    let steps_text = testcase.steps.as_deref().unwrap_or("No steps provided.");
    let steps_text = crate::tui::browser::strip_html_tags(steps_text);
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
