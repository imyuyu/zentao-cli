//! API 选择器 TUI 模块
//!
//! 提供交互式 API 端点选择界面

use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph},
    Frame, Terminal,
};
use std::io::stdout;

use crate::cmd::api_cmd::ApiEndpoint;

/// API 选择器状态
pub struct ApiSelector {
    endpoints: Vec<ApiEndpoint>,
    filtered: Vec<ApiEndpoint>,
    selected: usize,
    chars: Vec<char>, // 使用字符向量方便操作
    cursor_pos: usize,
}

impl ApiSelector {
    pub fn new(endpoints: Vec<ApiEndpoint>) -> Self {
        Self {
            filtered: endpoints.clone(),
            endpoints,
            selected: 0,
            chars: Vec::new(),
            cursor_pos: 0,
        }
    }

    /// 根据搜索词过滤端点
    fn filter(&mut self) {
        let query: String = self.chars.iter().collect::<String>().to_lowercase();
        if query.is_empty() {
            self.filtered = self.endpoints.clone();
        } else {
            self.filtered = self
                .endpoints
                .iter()
                .filter(|e| {
                    e.name.to_lowercase().contains(&query)
                        || e.path.to_lowercase().contains(&query)
                        || e.description.to_lowercase().contains(&query)
                })
                .cloned()
                .collect();
        }
        // 确保选中项在有效范围内
        if self.selected >= self.filtered.len() {
            self.selected = self.filtered.len().saturating_sub(1);
        }
    }
}

/// 运行 API 选择器 TUI
pub fn run_selector() -> Option<ApiEndpoint> {
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout())).ok()?;

    // 进入 alternate screen
    crossterm::execute!(
        terminal.backend_mut(),
        crossterm::terminal::EnterAlternateScreen
    )
    .ok()?;
    crossterm::execute!(terminal.backend_mut(), crossterm::cursor::Hide).ok()?;

    let endpoints = ApiEndpoint::all();
    let mut selector = ApiSelector::new(endpoints);

    // 渲染初始界面
    terminal
        .draw(|f| {
            let area = f.size();
            render_selector_frame(f, area, &selector);
        })
        .ok()?;

    loop {
        // 读取事件
        let event = crossterm::event::read().ok()?;

        if let crossterm::event::Event::Key(key) = event {
            if !matches!(key.kind, crossterm::event::KeyEventKind::Press) {
                continue;
            }
            use crossterm::event::KeyCode;
            match key.code {
                KeyCode::Esc => {
                    break;
                }
                KeyCode::Enter if !selector.filtered.is_empty() => {
                    let selected = selector.filtered[selector.selected].clone();
                    // 退出前清理
                    let _ = crossterm::execute!(terminal.backend_mut(), crossterm::cursor::Show);
                    let _ = terminal.clear();
                    let _ = crossterm::execute!(
                        terminal.backend_mut(),
                        crossterm::terminal::LeaveAlternateScreen
                    );
                    return Some(selected);
                }
                KeyCode::Up if selector.selected > 0 => {
                    selector.selected -= 1;
                }
                KeyCode::Down if selector.selected < selector.filtered.len().saturating_sub(1) => {
                    selector.selected += 1;
                }
                KeyCode::Left if selector.cursor_pos > 0 => {
                    selector.cursor_pos -= 1;
                }
                KeyCode::Right if selector.cursor_pos < selector.chars.len() => {
                    selector.cursor_pos += 1;
                }
                KeyCode::Home => {
                    selector.cursor_pos = 0;
                }
                KeyCode::End => {
                    selector.cursor_pos = selector.chars.len();
                }
                KeyCode::Backspace if selector.cursor_pos > 0 => {
                    selector.cursor_pos -= 1;
                    selector.chars.remove(selector.cursor_pos);
                    selector.filter();
                }
                KeyCode::Char(c) => {
                    selector.chars.insert(selector.cursor_pos, c);
                    selector.cursor_pos += 1;
                    selector.filter();
                }
                KeyCode::Delete if selector.cursor_pos < selector.chars.len() => {
                    selector.chars.remove(selector.cursor_pos);
                    selector.filter();
                }
                _ => {}
            }
        }

        // 更新显示
        terminal
            .draw(|f| {
                let area = f.size();
                render_selector_frame(f, area, &selector);
            })
            .ok()?;
    }

    // 退出前清理
    let _ = crossterm::execute!(terminal.backend_mut(), crossterm::cursor::Show);
    let _ = terminal.clear();
    let _ = crossterm::execute!(
        terminal.backend_mut(),
        crossterm::terminal::LeaveAlternateScreen
    );

    None
}

/// 渲染选择器界面
fn render_selector_frame(f: &mut Frame, area: Rect, selector: &ApiSelector) {
    // 清屏
    f.render_widget(Clear, area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // 标题栏
            Constraint::Length(3), // 搜索栏
            Constraint::Min(0),    // 列表
            Constraint::Length(3), // 底部提示
        ])
        .split(area);

    // 标题
    let title = Paragraph::new(vec![Line::from(Span::styled(
        " ZenTao API 选择器 ",
        Style::default().fg(Color::Cyan).bold(),
    ))])
    .block(
        Block::default()
            .borders(Borders::all())
            .title_style(Style::default().fg(Color::Cyan)),
    );
    f.render_widget(title, chunks[0]);

    // 搜索栏 - 使用 chars 向量正确处理 UTF-8
    let search_display: String = if selector.chars.is_empty() {
        "█".to_string()
    } else {
        let mut result = String::new();
        for (i, c) in selector.chars.iter().enumerate() {
            if i == selector.cursor_pos {
                result.push('█');
            }
            result.push(*c);
        }
        // 如果光标在末尾
        if selector.cursor_pos >= selector.chars.len() {
            result.push('█');
        }
        result
    };

    let search_content = Paragraph::new(vec![
        Line::from(""),
        Line::from(vec![Span::raw("搜索: "), Span::raw(search_display)]),
    ]);
    f.render_widget(search_content, chunks[1]);

    // 列表
    if selector.filtered.is_empty() {
        let no_result = Paragraph::new(vec![
            Line::from(""),
            Line::from(Span::styled(
                "没有找到匹配的 API 端点",
                Style::default().fg(Color::DarkGray),
            )),
        ]);
        f.render_widget(no_result, chunks[2]);
    } else {
        let items: Vec<ListItem> = selector
            .filtered
            .iter()
            .enumerate()
            .map(|(i, endpoint)| {
                let is_selected = i == selector.selected;
                let method_style = match endpoint.method {
                    "GET" => Style::default().fg(Color::Green),
                    "POST" => Style::default().fg(Color::Yellow),
                    "PUT" => Style::default().fg(Color::Blue),
                    "DELETE" => Style::default().fg(Color::Red),
                    _ => Style::default().fg(Color::White),
                };
                let line = if is_selected {
                    vec![
                        Span::styled("▶ ", Style::default().fg(Color::Cyan)),
                        Span::styled(endpoint.method, method_style.bold()),
                        Span::raw(" "),
                        Span::styled(endpoint.name, Style::default().fg(Color::White).bold()),
                        Span::raw(" "),
                        Span::styled(endpoint.path, Style::default().fg(Color::DarkGray)),
                    ]
                } else {
                    vec![
                        Span::raw("  "),
                        Span::styled(endpoint.method, method_style),
                        Span::raw(" "),
                        Span::raw(endpoint.name),
                        Span::raw(" "),
                        Span::styled(endpoint.path, Style::default().fg(Color::DarkGray)),
                    ]
                };
                ListItem::new(Line::from(line))
            })
            .collect();

        let list =
            List::new(items).block(Block::default().borders(Borders::all()).title("API 端点"));
        f.render_widget(list, chunks[2]);
    }

    // 底部提示
    let help_text = "↑↓ 选择 | Enter 确认 | 退格删除 | Esc 退出";
    let footer = Paragraph::new(vec![Line::from(Span::styled(
        help_text,
        Style::default().fg(Color::DarkGray),
    ))])
    .block(Block::default().borders(Borders::all()));
    f.render_widget(footer, chunks[3]);
}
