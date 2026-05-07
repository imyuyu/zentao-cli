//! TUI 配置引导向导模块
//!
//! 提供交互式配置初始化界面

use crate::core::config::{save_config_global, Config};
use crate::tui::config::{ConfigWizard, ConfigWizardState};
use anyhow::Result;
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame, Terminal,
};
use std::io::Stdout;
use std::sync::atomic::{AtomicBool, Ordering};

/// 静态变量用于信号
static SHOULD_QUIT: AtomicBool = AtomicBool::new(false);

/// 运行配置向导
pub fn run_wizard() -> Result<Option<Config>> {
    let stdout = std::io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // 进入 alternate screen
    crossterm::execute!(
        terminal.backend_mut(),
        crossterm::terminal::EnterAlternateScreen
    )?;
    crossterm::execute!(terminal.backend_mut(), crossterm::cursor::Hide)?;
    crossterm::execute!(
        terminal.backend_mut(),
        crossterm::terminal::Clear(crossterm::terminal::ClearType::All)
    )?;

    let mut wizard = ConfigWizard::new();

    loop {
        if SHOULD_QUIT.load(Ordering::SeqCst) {
            break;
        }

        terminal.draw(|f| {
            render_wizard_frame(f, f.size(), &wizard);
        })?;

        // 处理事件
        if crossterm::event::poll(std::time::Duration::from_millis(50))? {
            if let Ok(crossterm::event::Event::Key(key)) = crossterm::event::read() {
                if let KeyEventKind::Press = key.kind {
                    if handle_key_event(&key, &mut wizard, &mut terminal)? {
                        // 用户完成或退出
                        break;
                    }
                }
            }
        }
    }

    // 退出前清理
    let _ = crossterm::execute!(terminal.backend_mut(), crossterm::cursor::Show);
    let _ = crossterm::execute!(
        terminal.backend_mut(),
        crossterm::terminal::LeaveAlternateScreen
    );
    let _ = terminal.clear();

    // 检查是否保存成功
    if let ConfigWizardState::Saved { .. } = &wizard.state {
        let config = match &wizard.state {
            ConfigWizardState::Saved { url, .. } => Config {
                url: url.clone(),
                token: None,
                product_id: None,
                project_id: None,
                api_version: None,
                account: None,
            },
            _ => Config::default(),
        };
        return Ok(Some(config));
    }

    Ok(None)
}

/// 处理键盘事件
/// 返回 true 表示向导结束
fn handle_key_event(
    key: &KeyEvent,
    wizard: &mut ConfigWizard,
    _terminal: &mut Terminal<CrosstermBackend<Stdout>>,
) -> Result<bool> {
    match key.code {
        KeyCode::Esc | KeyCode::Char('q') => {
            SHOULD_QUIT.store(true, Ordering::SeqCst);
            return Ok(true);
        }
        _ => {}
    }

    // 根据当前状态处理按键
    let should_return_to_url = matches!(&wizard.state, ConfigWizardState::Error { .. })
        && matches!(key.code, KeyCode::Enter | KeyCode::Char('r'));

    if should_return_to_url {
        wizard.state = ConfigWizardState::Url;
        return Ok(false);
    }

    // 处理各个状态的按键
    match key.code {
        KeyCode::Enter => {
            match &wizard.state {
                ConfigWizardState::Success { url, token } => {
                    wizard.set_select_product(url.clone(), token.clone());
                }
                ConfigWizardState::SelectProduct { loading, .. } => {
                    if !*loading {
                        wizard.set_product_selected();
                    }
                }
                ConfigWizardState::SelectProject { loading, .. } => {
                    if !*loading {
                        // 保存配置
                        let (url, token, product_id, projects, selected) = match &wizard.state {
                            ConfigWizardState::SelectProject {
                                url,
                                token,
                                product_id,
                                projects,
                                selected,
                                ..
                            } => (
                                url.clone(),
                                token.clone(),
                                *product_id,
                                projects.clone(),
                                *selected,
                            ),
                            _ => return Ok(false),
                        };

                        let project_id = if !projects.is_empty() {
                            Some(projects[selected.min(projects.len() - 1)].id)
                        } else {
                            None
                        };

                        let config = Config {
                            url,
                            token: Some(token),
                            product_id,
                            project_id,
                            api_version: None,
                            account: None,
                        };

                        match save_config_global(&config) {
                            Ok(path) => {
                                let path_str = path.to_string_lossy().to_string();
                                wizard.set_saved(path_str.clone(), path_str);
                            }
                            Err(e) => {
                                wizard.set_error(format!("保存失败: {}", e));
                            }
                        }
                    }
                }
                _ => {}
            }
        }
        KeyCode::Up | KeyCode::Char('k') => {
            wizard.move_up();
        }
        KeyCode::Down | KeyCode::Char('j') => {
            wizard.move_down();
        }
        KeyCode::Esc => match &wizard.state {
            ConfigWizardState::Account { url, .. } => {
                wizard.state = ConfigWizardState::Url;
            }
            ConfigWizardState::Password { url, account, .. } => {
                wizard.state = ConfigWizardState::Account {
                    url: url.clone(),
                    account: account.clone(),
                };
            }
            ConfigWizardState::Success { url, .. } => {
                wizard.state = ConfigWizardState::Password {
                    url: url.clone(),
                    account: String::new(),
                    password: String::new(),
                };
            }
            ConfigWizardState::SelectProject { url, token, .. } => {
                wizard.set_select_product(url.clone(), token.clone());
            }
            _ => {}
        },
        _ => {}
    }

    Ok(false)
}

/// 渲染向导界面
fn render_wizard_frame(frame: &mut Frame, area: Rect, wizard: &ConfigWizard) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // 标题
            Constraint::Min(0),    // 内容
            Constraint::Length(3), // 底部提示
        ])
        .split(area);

    // 标题
    render_title(frame, chunks[0]);

    // 内容区
    match &wizard.state {
        ConfigWizardState::Url => {
            render_url_input(frame, chunks[1]);
        }
        ConfigWizardState::Account { url, account } => {
            render_account_input(frame, chunks[1], url, account);
        }
        ConfigWizardState::Password {
            url,
            account,
            password,
        } => {
            render_password_input(frame, chunks[1], url, account, password);
        }
        ConfigWizardState::Connecting { url, account, .. } => {
            render_connecting(frame, chunks[1], url, account);
        }
        ConfigWizardState::Success { url, token } => {
            render_success(frame, chunks[1], url, token);
        }
        ConfigWizardState::SelectProduct {
            url,
            token,
            products,
            selected,
            loading,
            error,
        } => {
            render_select_product(
                frame,
                chunks[1],
                url,
                token,
                products,
                *selected,
                *loading,
                error.as_deref(),
            );
        }
        ConfigWizardState::SelectProject {
            url,
            token,
            product_id,
            projects,
            selected,
            loading,
            error,
        } => {
            render_select_project(
                frame,
                chunks[1],
                url,
                token,
                *product_id,
                projects,
                *selected,
                *loading,
                error.as_deref(),
            );
        }
        ConfigWizardState::Saved { path, .. } => {
            render_saved(frame, chunks[1], path);
        }
        ConfigWizardState::Error { message } => {
            render_error_state(frame, chunks[1], message);
        }
    }

    // 底部提示
    render_footer(frame, chunks[2], &wizard.state);
}

/// 渲染标题
fn render_title(frame: &mut Frame, area: Rect) {
    let title = Paragraph::new(vec![Line::from(vec![
        Span::styled(" ZenTao CLI ", Style::default().fg(Color::Cyan).bold()),
        Span::styled("配置向导", Style::default().fg(Color::White)),
    ])])
    .block(
        Block::default()
            .borders(Borders::all())
            .border_style(Style::default().fg(Color::Cyan)),
    );
    frame.render_widget(title, area);
}

/// 渲染 URL 输入状态
fn render_url_input(frame: &mut Frame, area: Rect) {
    let content = Paragraph::new(vec![
        Line::from(""),
        Line::from(Span::raw("欢迎使用 ZenTao CLI!")),
        Line::from(""),
        Line::from(Span::raw("请输入 ZenTao 服务器地址:")),
        Line::from(""),
        Line::from(Span::styled(
            "例如: https://demo.zentao.site",
            Style::default().fg(Color::DarkGray),
        )),
        Line::from(""),
        Line::from(Span::raw("URL: ")),
    ])
    .block(
        Block::default()
            .title("步骤 1/4: 服务器地址")
            .borders(Borders::all()),
    );

    frame.render_widget(content, area);
}

/// 渲染账号输入状态
fn render_account_input(frame: &mut Frame, area: Rect, url: &str, account: &str) {
    let content = Paragraph::new(vec![
        Line::from(""),
        Line::from(Span::raw(format!("服务器: {}", url))),
        Line::from(""),
        Line::from(Span::raw("请输入 ZenTao 账号:")),
        Line::from(""),
        Line::from(Span::raw(format!("账号: {}", account))),
        Line::from(""),
        Line::from(Span::styled(
            "按 Enter 继续, Esc 返回上一步",
            Style::default().fg(Color::DarkGray),
        )),
    ])
    .block(
        Block::default()
            .title("步骤 2/4: 账号")
            .borders(Borders::all()),
    );

    frame.render_widget(content, area);
}

/// 渲染密码输入状态
fn render_password_input(frame: &mut Frame, area: Rect, url: &str, account: &str, password: &str) {
    let password_display = "*".repeat(password.len());
    let content = Paragraph::new(vec![
        Line::from(""),
        Line::from(Span::raw(format!("服务器: {}", url))),
        Line::from(Span::raw(format!("账号: {}", account))),
        Line::from(""),
        Line::from(Span::raw("请输入密码 (输入不回显):")),
        Line::from(""),
        Line::from(Span::raw(format!("密码: {}", password_display))),
        Line::from(""),
        Line::from(Span::styled(
            "按 Enter 登录, Esc 返回上一步",
            Style::default().fg(Color::DarkGray),
        )),
    ])
    .block(
        Block::default()
            .title("步骤 3/4: 密码")
            .borders(Borders::all()),
    );

    frame.render_widget(content, area);
}

/// 渲染连接中状态
fn render_connecting(frame: &mut Frame, area: Rect, url: &str, account: &str) {
    let content = Paragraph::new(vec![
        Line::from(""),
        Line::from(Span::raw(format!("正在连接 {} ...", url))),
        Line::from(Span::raw(format!("账号: {}", account))),
        Line::from(""),
        Line::from(Span::styled(
            "连接中...",
            Style::default().fg(Color::Yellow),
        )),
        Line::from(""),
        Line::from(Span::styled("请稍候", Style::default().fg(Color::DarkGray))),
    ])
    .block(
        Block::default()
            .title("步骤 3/4: 连接中")
            .borders(Borders::all()),
    );

    frame.render_widget(content, area);
}

/// 渲染登录成功状态
fn render_success(frame: &mut Frame, area: Rect, url: &str, token: &str) {
    let token_short = if token.len() > 20 {
        format!("{}...", &token[..20])
    } else {
        token.to_string()
    };

    let content = Paragraph::new(vec![
        Line::from(""),
        Line::from(Span::styled(
            "登录成功!",
            Style::default().fg(Color::Green).bold(),
        )),
        Line::from(""),
        Line::from(Span::raw(format!("服务器: {}", url))),
        Line::from(Span::raw(format!("Token: {}", token_short))),
        Line::from(""),
        Line::from(Span::raw("下一步: 选择产品")),
        Line::from(""),
        Line::from(Span::styled(
            "按 Enter 继续, Esc 重新输入",
            Style::default().fg(Color::DarkGray),
        )),
    ])
    .block(
        Block::default()
            .title("步骤 4/4: 登录成功")
            .borders(Borders::all()),
    );

    frame.render_widget(content, area);
}

/// 渲染产品选择状态
fn render_select_product(
    frame: &mut Frame,
    area: Rect,
    url: &str,
    token: &str,
    products: &[crate::api::Product],
    selected: usize,
    loading: bool,
    error: Option<&str>,
) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(6), // 标题信息
            Constraint::Min(0),    // 列表
            Constraint::Length(3), // 错误信息
        ])
        .split(area);

    let header = Paragraph::new(vec![
        Line::from(Span::raw(format!("服务器: {}", url))),
        Line::from(Span::raw("Token: ***")),
        Line::from(""),
        Line::from(Span::raw("请选择产品 (↑↓ 选择, Enter 确认):")),
    ])
    .block(Block::default().title("选择产品").borders(Borders::all()));

    frame.render_widget(header, chunks[0]);

    if loading {
        let loading_text = Paragraph::new(vec![Line::from(Span::styled(
            "加载中...",
            Style::default().fg(Color::Yellow),
        ))])
        .block(Block::default().borders(Borders::all()));

        frame.render_widget(loading_text, chunks[1]);
    } else if let Some(error_msg) = error {
        let error_text = Paragraph::new(vec![Line::from(Span::styled(
            format!("错误: {}", error_msg),
            Style::default().fg(Color::Red),
        ))])
        .block(Block::default().borders(Borders::all()));

        frame.render_widget(error_text, chunks[1]);
    } else if products.is_empty() {
        let empty_text = Paragraph::new(vec![Line::from(Span::styled(
            "没有可用的产品",
            Style::default().fg(Color::DarkGray),
        ))])
        .block(Block::default().borders(Borders::all()));

        frame.render_widget(empty_text, chunks[1]);
    } else {
        let items: Vec<ListItem> = products
            .iter()
            .enumerate()
            .map(|(i, product)| {
                let display = format!("[{}] {} - {}", product.id, product.name, product.status);
                let is_selected = i == selected;
                let line = if is_selected {
                    vec![
                        Span::styled("▶ ", Style::default().fg(Color::Cyan)),
                        Span::styled(display, Style::default().fg(Color::White).bold()),
                    ]
                } else {
                    vec![Span::raw(format!("  {}", display))]
                };
                ListItem::new(Line::from(line))
            })
            .collect();

        let list = List::new(items)
            .block(Block::default().borders(Borders::all()))
            .highlight_style(Style::default().bg(Color::Blue).fg(Color::White));

        frame.render_widget(list, chunks[1]);
    }

    let footer = Paragraph::new(vec![Line::from(Span::styled(
        "↑↓ 选择 | Enter 确认 | Esc 退出",
        Style::default().fg(Color::DarkGray),
    ))]);

    frame.render_widget(footer, chunks[2]);
}

/// 渲染项目选择状态
fn render_select_project(
    frame: &mut Frame,
    area: Rect,
    url: &str,
    token: &str,
    product_id: Option<u64>,
    projects: &[crate::api::Project],
    selected: usize,
    loading: bool,
    error: Option<&str>,
) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(6), // 标题信息
            Constraint::Min(0),    // 列表
            Constraint::Length(3), // 错误信息
        ])
        .split(area);

    let header = Paragraph::new(vec![
        Line::from(Span::raw(format!("服务器: {}", url))),
        Line::from(Span::raw(format!("产品ID: {:?}", product_id))),
        Line::from(""),
        Line::from(Span::raw("请选择项目 (↑↓ 选择, Enter 确认):")),
    ])
    .block(Block::default().title("选择项目").borders(Borders::all()));

    frame.render_widget(header, chunks[0]);

    if loading {
        let loading_text = Paragraph::new(vec![Line::from(Span::styled(
            "加载中...",
            Style::default().fg(Color::Yellow),
        ))])
        .block(Block::default().borders(Borders::all()));

        frame.render_widget(loading_text, chunks[1]);
    } else if let Some(error_msg) = error {
        let error_text = Paragraph::new(vec![Line::from(Span::styled(
            format!("错误: {}", error_msg),
            Style::default().fg(Color::Red),
        ))])
        .block(Block::default().borders(Borders::all()));

        frame.render_widget(error_text, chunks[1]);
    } else if projects.is_empty() {
        let empty_text = Paragraph::new(vec![
            Line::from(Span::styled(
                "没有可用的项目",
                Style::default().fg(Color::DarkGray),
            )),
            Line::from(Span::raw("")),
            Line::from(Span::raw("按 Enter 跳过项目选择，直接保存配置")),
        ])
        .block(Block::default().borders(Borders::all()));

        frame.render_widget(empty_text, chunks[1]);
    } else {
        let items: Vec<ListItem> = projects
            .iter()
            .enumerate()
            .map(|(i, project)| {
                let display = format!("[{}] {} - {}", project.id, project.name, project.status);
                let is_selected = i == selected;
                let line = if is_selected {
                    vec![
                        Span::styled("▶ ", Style::default().fg(Color::Cyan)),
                        Span::styled(display, Style::default().fg(Color::White).bold()),
                    ]
                } else {
                    vec![Span::raw(format!("  {}", display))]
                };
                ListItem::new(Line::from(line))
            })
            .collect();

        let list = List::new(items)
            .block(Block::default().borders(Borders::all()))
            .highlight_style(Style::default().bg(Color::Blue).fg(Color::White));

        frame.render_widget(list, chunks[1]);
    }

    let footer = Paragraph::new(vec![Line::from(Span::styled(
        "↑↓ 选择 | Enter 确认 | Esc 返回",
        Style::default().fg(Color::DarkGray),
    ))]);

    frame.render_widget(footer, chunks[2]);
}

/// 渲染保存成功状态
fn render_saved(frame: &mut Frame, area: Rect, path: &str) {
    let content = Paragraph::new(vec![
        Line::from(""),
        Line::from(Span::styled(
            "配置保存成功!",
            Style::default().fg(Color::Green).bold(),
        )),
        Line::from(""),
        Line::from(Span::raw(format!("保存路径: {}", path))),
        Line::from(""),
        Line::from(Span::raw("您现在可以使用 zentao-cli 访问 ZenTao 了")),
        Line::from(""),
        Line::from(Span::styled(
            "按任意键退出",
            Style::default().fg(Color::DarkGray),
        )),
    ])
    .block(Block::default().title("完成").borders(Borders::all()));

    frame.render_widget(content, area);
}

/// 渲染错误状态
fn render_error_state(frame: &mut Frame, area: Rect, message: &str) {
    let content = Paragraph::new(vec![
        Line::from(""),
        Line::from(Span::styled("错误", Style::default().fg(Color::Red).bold())),
        Line::from(""),
        Line::from(Span::raw(message)),
        Line::from(""),
        Line::from(Span::styled(
            "按 Enter 或 r 重试",
            Style::default().fg(Color::DarkGray),
        )),
    ])
    .block(Block::default().title("错误").borders(Borders::all()));

    frame.render_widget(content, area);
}

/// 渲染底部提示
fn render_footer(frame: &mut Frame, area: Rect, state: &ConfigWizardState) {
    let help_text = match state {
        ConfigWizardState::Url => "Tab 下一项 | Enter 确认 | Esc 退出",
        ConfigWizardState::Account { .. } => "Enter 继续 | Esc 返回",
        ConfigWizardState::Password { .. } => "Enter 登录 | Esc 返回",
        ConfigWizardState::Connecting { .. } => "请稍候...",
        ConfigWizardState::Success { .. } => "Enter 继续 | Esc 重新输入",
        ConfigWizardState::SelectProduct { .. } => "↑↓ 选择 | Enter 确认 | Esc 退出",
        ConfigWizardState::SelectProject { .. } => "↑↓ 选择 | Enter 确认 | Esc 返回",
        ConfigWizardState::Saved { .. } => "按任意键退出",
        ConfigWizardState::Error { .. } => "Enter/r 重试 | Esc 退出",
    };

    let footer = Paragraph::new(vec![Line::from(Span::styled(
        help_text,
        Style::default().fg(Color::DarkGray),
    ))])
    .block(Block::default().borders(Borders::all()));

    frame.render_widget(footer, area);
}
