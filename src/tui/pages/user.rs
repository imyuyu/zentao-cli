use crate::api::User;
use ratatui::layout::Rect;
use ratatui::Frame;

pub fn render_user_list(
    f: &mut Frame,
    area: Rect,
    users: &[User],
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
        Span::styled("User List", Style::default().add_modifier(Modifier::BOLD)),
        Span::raw(" ("),
        Span::raw(format!("{}", users.len())),
        Span::raw(" items)"),
    ])]))
    .block(Block::default().borders(Borders::ALL).title("ZenTao"));

    f.render_widget(header, chunks[0]);

    let items: Vec<ListItem> = users
        .iter()
        .map(|user| {
            ListItem::new(Line::from(vec![
                Span::raw(format!("{:6}", user.id)),
                Span::raw(" "),
                Span::styled(&user.account, Style::default()),
                Span::raw(" ("),
                Span::raw(&user.realname),
                Span::raw(")"),
                Span::raw(" | "),
                Span::raw(user.role.as_deref().unwrap_or("-")),
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
        Span::raw(format!("Selected: {} / {}", selected + 1, users.len())),
    ])]));
    f.render_widget(footer, chunks[2]);
}

pub fn render_user_detail(f: &mut Frame, area: Rect, user: &User) {
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
        Span::raw(format!("User #{} - ", user.id)),
        Span::styled(
            &user.realname,
            Style::default().add_modifier(Modifier::BOLD),
        ),
        Span::raw(" ("),
        Span::raw(&user.account),
        Span::raw(")"),
    ])]))
    .block(Block::default().borders(Borders::ALL).title("User Detail"));

    f.render_widget(title, chunks[0]);

    let details = Paragraph::new(Text::from(vec![
        Line::from(vec![Span::raw("Account: "), Span::raw(&user.account)]),
        Line::from(vec![Span::raw("Realname: "), Span::raw(&user.realname)]),
        Line::from(vec![
            Span::raw("Email: "),
            Span::raw(user.email.as_deref().unwrap_or("N/A")),
        ]),
        Line::from(vec![
            Span::raw("Dept: "),
            Span::raw(
                user.dept
                    .map(|d| d.to_string())
                    .unwrap_or_else(|| "N/A".to_string())
                    .to_string(),
            ),
        ]),
        Line::from(vec![
            Span::raw("Role: "),
            Span::raw(user.role.as_deref().unwrap_or("N/A")),
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
