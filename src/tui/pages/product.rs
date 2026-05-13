use crate::api::Product;
use ratatui::layout::Rect;
use ratatui::Frame;

pub fn render_product_list(
    f: &mut Frame,
    area: Rect,
    products: &[Product],
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
            "Product List",
            Style::default().add_modifier(Modifier::BOLD),
        ),
        Span::raw(" ("),
        Span::raw(format!("{}", products.len())),
        Span::raw(" items)"),
    ])]))
    .block(Block::default().borders(Borders::ALL).title("ZenTao"));

    f.render_widget(header, chunks[0]);

    let items: Vec<ListItem> = products
        .iter()
        .map(|product| {
            ListItem::new(Line::from(vec![
                Span::raw(format!("{:6}", product.id)),
                Span::raw(" "),
                Span::styled(&product.name, Style::default()),
                Span::raw(" ("),
                Span::raw(&product.code),
                Span::raw(") | "),
                Span::styled(
                    format!("[{}]", product.status),
                    match product.status.as_str() {
                        "normal" => Style::default().fg(Color::Green),
                        "closed" => Style::default().fg(Color::Red),
                        _ => Style::default().fg(Color::Yellow),
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
        Span::raw(format!("Selected: {} / {}", selected + 1, products.len())),
    ])]));
    f.render_widget(footer, chunks[2]);
}

pub fn render_product_detail(f: &mut Frame, area: Rect, product: &Product) {
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
        Span::raw(format!("Product #{} - ", product.id)),
        Span::styled(&product.name, Style::default().add_modifier(Modifier::BOLD)),
    ])]))
    .block(
        Block::default()
            .borders(Borders::ALL)
            .title("Product Detail"),
    );

    f.render_widget(title, chunks[0]);

    let details = Paragraph::new(Text::from(vec![
        Line::from(vec![
            Span::raw("ID: "),
            Span::raw(format!("{}", product.id)),
        ]),
        Line::from(vec![Span::raw("Name: "), Span::raw(&product.name)]),
        Line::from(vec![Span::raw("Code: "), Span::raw(&product.code)]),
        Line::from(vec![Span::raw("Status: "), Span::raw(&product.status)]),
    ]))
    .block(Block::default().borders(Borders::ALL).title("Details"));

    f.render_widget(details, chunks[1]);

    let desc = Paragraph::new(Text::from(vec![Line::from(Span::raw(
        product
            .desc
            .as_deref()
            .unwrap_or("No description provided."),
    ))]))
    .block(Block::default().borders(Borders::ALL).title("Description"));

    f.render_widget(desc, chunks[2]);

    let footer = Paragraph::new(Text::from(vec![Line::from(vec![
        Span::styled("q", Style::default().add_modifier(Modifier::BOLD)),
        Span::raw(" back  "),
        Span::styled("o", Style::default().add_modifier(Modifier::BOLD)),
        Span::raw(" open"),
    ])]));

    f.render_widget(footer, chunks[3]);
}
