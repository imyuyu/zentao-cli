use crate::api::Department;
use ratatui::layout::Rect;
use ratatui::Frame;

pub fn render_department_list(
    f: &mut Frame,
    area: Rect,
    departments: &[Department],
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
            "Department List",
            Style::default().add_modifier(Modifier::BOLD),
        ),
        Span::raw(" ("),
        Span::raw(format!("{}", departments.len())),
        Span::raw(" items)"),
    ])]))
    .block(Block::default().borders(Borders::ALL).title("ZenTao"));

    f.render_widget(header, chunks[0]);

    let items: Vec<ListItem> = departments
        .iter()
        .map(|dept| {
            ListItem::new(Line::from(vec![
                Span::raw(format!("{:6}", dept.id)),
                Span::raw(" "),
                Span::styled(&dept.name, Style::default()),
                Span::raw(" | "),
                Span::raw(format!(
                    "Parent: {}",
                    dept.parent
                        .map(|p| p.to_string())
                        .unwrap_or_else(|| "root".to_string())
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
        Span::raw(format!(
            "Selected: {} / {}",
            selected + 1,
            departments.len()
        )),
    ])]));
    f.render_widget(footer, chunks[2]);
}

pub fn render_department_detail(f: &mut Frame, area: Rect, department: &Department) {
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
            Constraint::Length(8),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(area);

    let title = Paragraph::new(Text::from(vec![Line::from(vec![
        Span::raw(format!("Department #{} - ", department.id)),
        Span::styled(
            &department.name,
            Style::default().add_modifier(Modifier::BOLD),
        ),
    ])]))
    .block(
        Block::default()
            .borders(Borders::ALL)
            .title("Department Detail"),
    );

    f.render_widget(title, chunks[0]);

    let details = Paragraph::new(Text::from(vec![
        Line::from(vec![
            Span::raw("ID: "),
            Span::raw(format!("{}", department.id)),
        ]),
        Line::from(vec![Span::raw("Name: "), Span::raw(&department.name)]),
        Line::from(vec![
            Span::raw("Parent: "),
            Span::raw(
                department
                    .parent
                    .map(|p| p.to_string())
                    .unwrap_or_else(|| "root".to_string())
                    .to_string(),
            ),
        ]),
        Line::from(vec![
            Span::raw("Order: "),
            Span::raw(
                department
                    .order
                    .map(|o| o.to_string())
                    .unwrap_or_else(|| "N/A".to_string())
                    .to_string(),
            ),
        ]),
        Line::from(vec![
            Span::raw("Path: "),
            Span::raw(department.path.as_deref().unwrap_or("N/A")),
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
