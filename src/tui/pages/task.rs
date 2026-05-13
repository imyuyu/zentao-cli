use ratatui::Frame;
use ratatui::layout::Rect;
use crate::api::Task;

pub fn render_task_list(
    f: &mut Frame,
    area: Rect,
    tasks: &[Task],
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
        Span::styled("Task List", Style::default().add_modifier(Modifier::BOLD)),
        Span::raw(" ("),
        Span::raw(format!("{}", tasks.len())),
        Span::raw(" items)"),
    ])]))
    .block(Block::default().borders(Borders::ALL).title("ZenTao"));

    f.render_widget(header, chunks[0]);

    let items: Vec<ListItem> = tasks
        .iter()
        .map(|task| {
            ListItem::new(Line::from(vec![
                Span::raw(format!("{:6}", task.id)),
                Span::raw(" "),
                Span::styled(&task.name, Style::default()),
                Span::raw(" | "),
                Span::styled(
                    format!("[{}]", task.status),
                    match task.status.as_str() {
                        "done" => Style::default().fg(Color::Green),
                        "closed" => Style::default().fg(Color::Red),
                        "in progress" => Style::default().fg(Color::Yellow),
                        _ => Style::default().fg(Color::Blue),
                    },
                ),
                Span::raw(" | Pri:"),
                Span::raw(format!("{}", task.pri)),
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
        Span::raw(format!("Selected: {} / {}", selected + 1, tasks.len())),
    ])]));
    f.render_widget(footer, chunks[2]);
}

pub fn render_task_detail(f: &mut Frame, area: Rect, task: &Task) {
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
        Span::raw(format!("Task #{} - ", task.id)),
        Span::styled(&task.name, Style::default().add_modifier(Modifier::BOLD)),
    ])]))
    .block(Block::default().borders(Borders::ALL).title("Task Detail"));

    f.render_widget(title, chunks[0]);

    let details = Paragraph::new(Text::from(vec![
        Line::from(vec![Span::raw("Status: "), Span::raw(&task.status)]),
        Line::from(vec![
            Span::raw("Priority: "),
            Span::raw(format!("{}", task.pri)),
        ]),
        Line::from(vec![
            Span::raw("Project: "),
            Span::raw(format!("{}", task.project)),
        ]),
        Line::from(vec![
            Span::raw("Assigned: "),
            Span::raw(task.assigned_to.as_deref().unwrap_or("Unassigned")),
        ]),
        Line::from(vec![
            Span::raw("Estimate: "),
            Span::raw(task.estimate
                    .map(|e| format!("{}h", e))
                    .unwrap_or_else(|| "N/A".to_string()).to_string()),
        ]),
        Line::from(vec![
            Span::raw("Consumed: "),
            Span::raw(task.consumed
                    .map(|c| format!("{}h", c))
                    .unwrap_or_else(|| "N/A".to_string()).to_string()),
        ]),
        Line::from(vec![
            Span::raw("Left: "),
            Span::raw(task.left
                    .map(|l| format!("{}h", l))
                    .unwrap_or_else(|| "N/A".to_string()).to_string()),
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
