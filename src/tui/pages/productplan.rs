use ratatui::Frame;
use ratatui::layout::Rect;
use crate::api::ProductPlan;

pub fn render_productplan_list(
    f: &mut Frame,
    area: Rect,
    plans: &[ProductPlan],
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
            "ProductPlan List",
            Style::default().add_modifier(Modifier::BOLD),
        ),
        Span::raw(" ("),
        Span::raw(format!("{}", plans.len())),
        Span::raw(" items)"),
    ])]))
    .block(Block::default().borders(Borders::ALL).title("ZenTao"));

    f.render_widget(header, chunks[0]);

    let items: Vec<ListItem> = plans
        .iter()
        .map(|plan| {
            ListItem::new(Line::from(vec![
                Span::raw(format!("{:6}", plan.id)),
                Span::raw(" "),
                Span::styled(plan.name.as_deref().unwrap_or("-"), Style::default()),
                Span::raw(" | "),
                Span::styled(
                    format!("[{}]", plan.status.as_deref().unwrap_or("-")),
                    match plan.status.as_deref() {
                        Some("done") => Style::default().fg(Color::Green),
                        Some("closed") => Style::default().fg(Color::Red),
                        Some("doing") => Style::default().fg(Color::Yellow),
                        _ => Style::default().fg(Color::Blue),
                    },
                ),
                Span::raw(" | "),
                Span::raw(plan.begin.clone().unwrap_or_else(|| "-".to_string())),
                Span::raw(" ~ "),
                Span::raw(plan.end.clone().unwrap_or_else(|| "-".to_string())),
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
        Span::raw(format!("Selected: {} / {}", selected + 1, plans.len())),
    ])]));
    f.render_widget(footer, chunks[2]);
}

pub fn render_productplan_detail(f: &mut Frame, area: Rect, plan: &ProductPlan) {
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
        Span::raw(format!("ProductPlan #{} - ", plan.id)),
        Span::styled(
            plan.name.as_deref().unwrap_or("-"),
            Style::default().add_modifier(Modifier::BOLD),
        ),
    ])]))
    .block(
        Block::default()
            .borders(Borders::ALL)
            .title("ProductPlan Detail"),
    );

    f.render_widget(title, chunks[0]);

    let details = Paragraph::new(Text::from(vec![
        Line::from(vec![Span::raw("ID: "), Span::raw(format!("{}", plan.id))]),
        Line::from(vec![
            Span::raw("Name: "),
            Span::raw(plan.name.as_deref().unwrap_or("-")),
        ]),
        Line::from(vec![
            Span::raw("Code: "),
            Span::raw(plan.code.as_deref().unwrap_or("-")),
        ]),
        Line::from(vec![
            Span::raw("Status: "),
            Span::raw(plan.status.as_deref().unwrap_or("-")),
        ]),
        Line::from(vec![
            Span::raw("Type: "),
            Span::raw(plan.type_.as_deref().unwrap_or("N/A")),
        ]),
        Line::from(vec![
            Span::raw("Product: "),
            Span::raw(format!("{}", plan.product)),
        ]),
        Line::from(vec![
            Span::raw("Begin: "),
            Span::raw(plan.begin.as_deref().unwrap_or("N/A")),
        ]),
        Line::from(vec![
            Span::raw("End: "),
            Span::raw(plan.end.as_deref().unwrap_or("N/A")),
        ]),
        Line::from(vec![
            Span::raw("Owner: "),
            Span::raw(plan.owner.as_deref().unwrap_or("N/A")),
        ]),
        Line::from(vec![
            Span::raw("Stories: "),
            Span::raw(format!(
                "{}",
                plan.story_count
                    .map(|s| s.to_string())
                    .unwrap_or_else(|| "N/A".to_string())
            )),
        ]),
        Line::from(vec![
            Span::raw("Bugs: "),
            Span::raw(format!(
                "{}",
                plan.bug_count
                    .map(|b| b.to_string())
                    .unwrap_or_else(|| "N/A".to_string())
            )),
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
