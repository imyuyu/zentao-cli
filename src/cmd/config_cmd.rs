//! ZenTao CLI 配置命令模块
//!
//! 提供交互式配置向导和配置管理功能

use anyhow::Result;
use clap::Subcommand;
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, ListItem, Paragraph},
    Frame, Terminal,
};
use std::io::stdout;

use crate::api::{ApiClient, Auth, ProductApi, ProjectApi};
use crate::core::{
    global_config_path, load_config, project_config_path, unset_config, update_config, Config,
    GlobalConfig,
};
use crate::safe_println;
use crate::tui::config::{ConfigWizard, ConfigWizardState};

#[derive(Subcommand, Clone, Debug)]
pub enum ConfigSubcommand {
    /// 交互式初始化配置向导
    #[command(name = "init")]
    Init {
        #[arg(long, short = 'g')]
        global: bool,
    },
    /// 显示当前配置
    #[command(name = "show")]
    Show,
    /// 设置配置项
    #[command(name = "set")]
    Set {
        key: String,
        value: String,
        /// 保存到全局配置（不加参数默认保存到项目配置）
        #[arg(long, short = 'g')]
        global: bool,
    },
    /// 获取配置项
    #[command(name = "get")]
    Get { key: String },
    /// 取消设置配置项
    #[command(name = "unset")]
    Unset {
        key: String,
        /// 从全局配置取消设置
        #[arg(long, short = 'g')]
        global: bool,
    },
}

/// 运行 TUI 配置向导
pub async fn run_tui_wizard(global: bool) -> Result<()> {
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    // 进入alternate screen
    crossterm::execute!(
        terminal.backend_mut(),
        crossterm::terminal::EnterAlternateScreen
    )?;
    crossterm::execute!(terminal.backend_mut(), crossterm::cursor::Hide)?;

    let mut wizard = ConfigWizard::new();
    let mut input_buffer = String::new();
    let mut cursor_pos: usize = 0; // 光标位置

    // 渲染初始界面
    terminal.draw(|f| {
        let area = f.size();
        render_wizard_frame(f, area, &wizard.state, &input_buffer, cursor_pos);
    })?;

    loop {
        // 读取事件
        let event = crossterm::event::read()?;

        if let crossterm::event::Event::Key(key) = event {
            // 只处理press事件
            if !matches!(key.kind, crossterm::event::KeyEventKind::Press) {
                continue;
            }
            use crossterm::event::KeyCode;
            match key.code {
                KeyCode::Esc => {
                    // 在选择状态下，Esc 跳过选择
                    if matches!(wizard.state, ConfigWizardState::SelectProduct { .. }) {
                        // 跳过产品选择，transition to SelectProject
                        if let ConfigWizardState::SelectProduct { url, token, .. } = &wizard.state {
                            wizard.state = ConfigWizardState::SelectProject {
                                url: url.clone(),
                                token: token.clone(),
                                product_id: None,
                                projects: Vec::new(),
                                selected: 0,
                                loading: false,
                                error: None,
                            };
                        }
                    } else if matches!(wizard.state, ConfigWizardState::SelectProject { .. }) {
                        // 跳过项目选择，直接保存
                        if let ConfigWizardState::SelectProject {
                            url,
                            token,
                            product_id,
                            projects: _,
                            ..
                        } = &wizard.state
                        {
                            let path = save_config(url, token, *product_id, None, global)
                                .await
                                .unwrap_or_default();
                            wizard.set_saved(url.clone(), path);
                        }
                    } else {
                        break;
                    }
                }
                KeyCode::Enter => {
                    handle_enter(&mut wizard, &input_buffer, global).await?;
                    input_buffer.clear();
                    cursor_pos = 0;
                    // 只有 Saved 状态才退出（按 Enter 确认后）
                    if matches!(wizard.state, ConfigWizardState::Saved { .. }) {
                        break;
                    }
                }
                KeyCode::Left if cursor_pos > 0 => {
                    cursor_pos -= 1;
                }
                KeyCode::Right if cursor_pos < input_buffer.len() => {
                    cursor_pos += 1;
                }
                KeyCode::Home => {
                    cursor_pos = 0;
                }
                KeyCode::End => {
                    cursor_pos = input_buffer.len();
                }
                KeyCode::Backspace if cursor_pos > 0 => {
                    input_buffer.remove(cursor_pos - 1);
                    cursor_pos -= 1;
                }
                KeyCode::Char(c) => {
                    input_buffer.insert(cursor_pos, c);
                    cursor_pos += 1;
                }
                KeyCode::Delete if cursor_pos < input_buffer.len() => {
                    input_buffer.remove(cursor_pos);
                }
                KeyCode::Up => {
                    wizard.move_up();
                }
                KeyCode::Down => {
                    wizard.move_down();
                }
                _ => {}
            }
        }

        // 更新显示
        terminal.draw(|f| {
            let area = f.size();
            render_wizard_frame(f, area, &wizard.state, &input_buffer, cursor_pos);
        })?;
    }

    // 退出前清理：显示光标、清除屏幕、离开alternate screen
    let _ = crossterm::execute!(terminal.backend_mut(), crossterm::cursor::Show);
    let _ = terminal.clear();
    let _ = crossterm::execute!(
        terminal.backend_mut(),
        crossterm::terminal::LeaveAlternateScreen
    );

    Ok(())
}

/// 处理回车键
async fn handle_enter(wizard: &mut ConfigWizard, input_buffer: &str, global: bool) -> Result<()> {
    match &wizard.state {
        ConfigWizardState::Url => {
            if input_buffer.is_empty() {
                return Ok(());
            }
            wizard.set_url(input_buffer.to_string());
        }
        ConfigWizardState::Account { .. } => {
            if input_buffer.is_empty() {
                return Ok(());
            }
            wizard.set_account(input_buffer.to_string());
        }
        ConfigWizardState::Password { .. } => {
            if input_buffer.is_empty() {
                return Ok(());
            }
            wizard.set_password(input_buffer.to_string());

            // 开始登录
            if let ConfigWizardState::Connecting {
                url,
                account,
                password,
            } = &wizard.state
            {
                let auth = Auth::new(url);
                match auth.login(account, password).await {
                    Ok(token) => {
                        wizard.set_success(url.clone(), token);
                    }
                    Err(e) => {
                        wizard.set_error(e.to_string());
                    }
                }
            }
        }
        ConfigWizardState::Success { url, token } => {
            // 登录成功后，进入产品选择
            let url = url.clone();
            let token = token.clone();
            wizard.set_select_product(url.clone(), token.clone());
            // 同步加载产品列表
            let client = ApiClient::new(&url, Some(token));
            match ProductApi::list(&client).await {
                Ok(products) => wizard.set_products(products),
                Err(e) => wizard.set_load_error(e.to_string()),
            }
        }
        ConfigWizardState::SelectProduct { .. } => {
            // 用户选择产品后，进入项目选择
            wizard.set_product_selected();
            // 同步加载项目列表
            if let ConfigWizardState::SelectProject { url, token, .. } = &wizard.state {
                let url = url.clone();
                let token = token.clone();
                let client = ApiClient::new(&url, Some(token));
                match ProjectApi::list(&client).await {
                    Ok(projects) => wizard.set_projects(projects),
                    Err(e) => wizard.set_load_error(e.to_string()),
                }
            }
        }
        ConfigWizardState::SelectProject {
            url,
            token,
            product_id,
            projects,
            ..
        } => {
            // 用户选择项目后，保存配置
            let selected_project = if !projects.is_empty() {
                if let ConfigWizardState::SelectProject { selected, .. } = &wizard.state {
                    Some(projects[*selected].id)
                } else {
                    None
                }
            } else {
                None
            };
            let path = save_config(url, token, *product_id, selected_project, global).await?;
            wizard.set_saved(url.clone(), path);
        }
        ConfigWizardState::Error { .. } => {
            // 重置
            *wizard = ConfigWizard::new();
        }
        _ => {}
    }
    Ok(())
}

/// 保存配置到文件
async fn save_config(
    url: &str,
    token: &str,
    product_id: Option<u64>,
    project_id: Option<u64>,
    global: bool,
) -> Result<String> {
    let scope = if global {
        if let Some(parent) = global_config_path().parent() {
            std::fs::create_dir_all(parent)?;
        }
        global_config_path()
    } else {
        std::fs::create_dir_all(project_config_path().parent().unwrap())?;
        project_config_path()
    };

    let config = Config {
        url: url.to_string(),
        token: Some(token.to_string()),
        product_id,
        project_id,
        api_version: None,
    };

    let global = GlobalConfig { default: config };

    let content = toml::to_string_pretty(&global)?;
    std::fs::write(&scope, &content)?;

    Ok(scope.display().to_string())
}

/// 渲染向导界面
fn render_wizard_frame(
    f: &mut Frame,
    area: Rect,
    state: &ConfigWizardState,
    input_buffer: &str,
    cursor_pos: usize,
) {
    // 清屏
    f.render_widget(Clear, area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // 标题栏
            Constraint::Min(0),    // 内容
            Constraint::Length(3), // 底部提示
        ])
        .split(area);

    // 标题
    let title = Paragraph::new(vec![Line::from(Span::styled(
        " ZenTao CLI 配置向导 ",
        Style::default().fg(Color::Cyan).bold(),
    ))])
    .block(
        Block::default()
            .borders(Borders::all())
            .title_style(Style::default().fg(Color::Cyan)),
    );

    f.render_widget(title, chunks[0]);

    // 内容区
    match state {
        ConfigWizardState::Url
        | ConfigWizardState::Account { .. }
        | ConfigWizardState::Password { .. } => {
            let (buffer, prompt, is_password) = match state {
                ConfigWizardState::Url => (input_buffer, "请输入 ZenTao 服务器地址：", false),
                ConfigWizardState::Account { .. } => (input_buffer, "请输入账号：", false),
                ConfigWizardState::Password { .. } => (input_buffer, "请输入密码：", true),
                _ => unreachable!(),
            };

            let display: String = if is_password {
                "*".repeat(buffer.len())
            } else {
                buffer.to_string()
            };

            // 构建输入行：before + [光标位置字符] + after
            let mut spans = vec![Span::raw("> ")];
            spans.push(Span::raw(
                display.chars().take(cursor_pos).collect::<String>(),
            ));

            // 光标位置的字符（下划线样式）
            if let Some(c) = display.chars().nth(cursor_pos) {
                spans.push(Span::styled(
                    c.to_string(),
                    Style::default().fg(Color::Black).on_yellow().bold(),
                ));
            } else {
                spans.push(Span::styled(
                    " ",
                    Style::default().fg(Color::Black).on_yellow(),
                ));
            }

            spans.push(Span::raw(
                display.chars().skip(cursor_pos + 1).collect::<String>(),
            ));

            let content = Paragraph::new(vec![
                Line::from(""),
                Line::from(""),
                Line::from(prompt),
                Line::from(""),
                Line::from(spans),
            ]);
            f.render_widget(content, chunks[1]);
        }
        ConfigWizardState::Connecting { .. } => {
            let content = Paragraph::new(vec![
                Line::from(""),
                Line::from(""),
                Line::from(Span::styled(
                    "正在连接...",
                    Style::default().fg(Color::Yellow),
                )),
                Line::from("请稍候..."),
            ]);
            f.render_widget(content, chunks[1]);
        }
        ConfigWizardState::Success { url, token } => {
            let masked = if token.len() > 8 {
                format!("{}...", &token[..8])
            } else {
                token.clone()
            };
            let content = Paragraph::new(vec![
                Line::from(""),
                Line::from(""),
                Line::from(Span::styled(
                    "✓ 登录成功！",
                    Style::default().fg(Color::Green),
                )),
                Line::from(""),
                Line::from(format!("URL: {}", url)),
                Line::from(format!("Token: {}", masked)),
            ]);
            f.render_widget(content, chunks[1]);
        }
        ConfigWizardState::SelectProduct {
            products,
            selected,
            loading,
            error,
            ..
        } => {
            let content = if *loading {
                Paragraph::new(vec![
                    Line::from(""),
                    Line::from(""),
                    Line::from(Span::styled(
                        "正在加载产品列表...",
                        Style::default().fg(Color::Yellow),
                    )),
                ])
            } else if let Some(err) = error {
                Paragraph::new(vec![
                    Line::from(""),
                    Line::from(Span::styled("✗ 加载失败", Style::default().fg(Color::Red))),
                    Line::from(Span::raw(err.clone())),
                    Line::from(""),
                    Line::from("按 Enter 跳过，或 Esc 退出"),
                ])
            } else if products.is_empty() {
                Paragraph::new(vec![
                    Line::from(""),
                    Line::from("未找到产品"),
                    Line::from(""),
                    Line::from("按 Enter 跳过，或 Esc 退出"),
                ])
            } else {
                let items: Vec<ListItem> = products
                    .iter()
                    .enumerate()
                    .map(|(i, p)| {
                        let style = if i == *selected {
                            Style::default().bg(Color::Blue).fg(Color::White)
                        } else {
                            Style::default()
                        };
                        ListItem::new(Line::from(vec![
                            Span::raw(format!("{:3}", p.id)),
                            Span::raw(" "),
                            Span::raw(&p.name),
                        ]))
                        .style(style)
                    })
                    .collect();
                let list = ratatui::widgets::List::new(items)
                    .block(Block::default().borders(Borders::ALL).title("选择产品"))
                    .highlight_style(Style::default().bg(Color::Blue).fg(Color::White));
                f.render_widget(list, chunks[1]);
                f.render_widget(
                    Paragraph::new(vec![Line::from("↑/↓ 选择 | Enter 确认 | Esc 跳过")]),
                    chunks[2],
                );
                return;
            };
            f.render_widget(content, chunks[1]);
        }
        ConfigWizardState::SelectProject {
            projects,
            selected,
            loading,
            error,
            product_id,
            ..
        } => {
            let content = if *loading {
                Paragraph::new(vec![
                    Line::from(""),
                    Line::from(""),
                    Line::from(Span::styled(
                        "正在加载项目列表...",
                        Style::default().fg(Color::Yellow),
                    )),
                ])
            } else if let Some(err) = error {
                Paragraph::new(vec![
                    Line::from(""),
                    Line::from(Span::styled("✗ 加载失败", Style::default().fg(Color::Red))),
                    Line::from(Span::raw(err.clone())),
                    Line::from(""),
                    Line::from("按 Enter 跳过，或 Esc 退出"),
                ])
            } else if projects.is_empty() {
                Paragraph::new(vec![
                    Line::from(""),
                    Line::from("未找到项目"),
                    Line::from(format!("产品ID: {:?}", product_id)),
                    Line::from(""),
                    Line::from("按 Enter 完成配置，或 Esc 退出"),
                ])
            } else {
                let items: Vec<ListItem> = projects
                    .iter()
                    .enumerate()
                    .map(|(i, p)| {
                        let style = if i == *selected {
                            Style::default().bg(Color::Blue).fg(Color::White)
                        } else {
                            Style::default()
                        };
                        ListItem::new(Line::from(vec![
                            Span::raw(format!("{:3}", p.id)),
                            Span::raw(" "),
                            Span::raw(&p.name),
                        ]))
                        .style(style)
                    })
                    .collect();
                let list = ratatui::widgets::List::new(items)
                    .block(Block::default().borders(Borders::ALL).title("选择项目"))
                    .highlight_style(Style::default().bg(Color::Blue).fg(Color::White));
                f.render_widget(list, chunks[1]);
                f.render_widget(
                    Paragraph::new(vec![
                        Line::from(""),
                        Line::from("↑/↓ 选择 | Enter 确认 | Esc 跳过"),
                    ]),
                    chunks[2],
                );
                return;
            };
            f.render_widget(content, chunks[1]);
        }
        ConfigWizardState::Saved { url, path } => {
            let content = Paragraph::new(vec![
                Line::from(""),
                Line::from(""),
                Line::from(Span::styled(
                    "✓ 配置已保存！",
                    Style::default().fg(Color::Green),
                )),
                Line::from(""),
                Line::from(format!("URL: {}", url)),
                Line::from(format!("保存路径: {}", path)),
                Line::from(""),
                Line::from("运行 'zentao-cli auth status' 验证配置"),
            ]);
            f.render_widget(content, chunks[1]);
        }
        ConfigWizardState::Error { message } => {
            let content = Paragraph::new(vec![
                Line::from(""),
                Line::from(""),
                Line::from(Span::styled("✗ 配置失败", Style::default().fg(Color::Red))),
                Line::from(""),
                Line::from(Span::raw(message.clone())),
            ]);
            f.render_widget(content, chunks[1]);
        }
    }

    // 底部提示
    let help_text = match state {
        ConfigWizardState::Url
        | ConfigWizardState::Account { .. }
        | ConfigWizardState::Password { .. } => "Enter 确认 | Esc 退出 | 退格删除",
        ConfigWizardState::Connecting { .. } => "请稍候...",
        ConfigWizardState::Success { .. } => "按 Enter 选择产品",
        ConfigWizardState::SelectProduct { .. } => "↑/↓ 选择 | Enter 确认 | Esc 跳过",
        ConfigWizardState::SelectProject { .. } => "↑/↓ 选择 | Enter 确认 | Esc 跳过",
        ConfigWizardState::Saved { .. } => "按 Enter 退出",
        ConfigWizardState::Error { .. } => "按任意键重试",
    };

    let footer = Paragraph::new(vec![Line::from(Span::styled(
        help_text,
        Style::default().fg(Color::DarkGray),
    ))])
    .block(Block::default().borders(Borders::all()));

    f.render_widget(footer, chunks[2]);
}

/// 执行配置命令
pub async fn run(config_cmd: &ConfigSubcommand) -> Result<()> {
    match config_cmd {
        ConfigSubcommand::Init { global } => {
            // 启用原始模式
            crossterm::terminal::enable_raw_mode()?;
            let result = run_tui_wizard(*global).await;
            let _ = crossterm::terminal::disable_raw_mode();
            result
        }

        ConfigSubcommand::Show => {
            let config = load_config()?;

            safe_println("ZenTao CLI Configuration");
            safe_println("======================");
            println!("Global config: {}", global_config_path().display());
            println!("Project config: {}", project_config_path().display());
            println!();
            safe_println("Current values:");
            println!("  url: {}", config.url);
            println!(
                "  token: {}",
                config
                    .token
                    .as_ref()
                    .map(|t| if t.is_empty() {
                        "(empty)".to_string()
                    } else {
                        format!("{}...", &t[..t.len().min(8)])
                    })
                    .unwrap_or_else(|| "(not set)".to_string())
            );
            println!("  product_id: {:?}", config.product_id);
            println!("  project_id: {:?}", config.project_id);
            Ok(())
        }

        ConfigSubcommand::Set { key, value, global } => {
            let path = update_config(key, value, *global)?;
            let scope = if *global { "global" } else { "project" };
            println!("✓ {} set to {} in {} config", key, value, scope);
            if key == "token" {
                println!("  Token saved to: {}", path.display());
            }
            Ok(())
        }

        ConfigSubcommand::Get { key } => {
            let config = load_config()?;
            match key.as_str() {
                "url" => println!("{}", config.url),
                "token" => println!(
                    "{}",
                    config
                        .token
                        .as_ref()
                        .map(|t| if t.is_empty() {
                            "(empty)".to_string()
                        } else {
                            format!("{}...", &t[..t.len().min(8)])
                        })
                        .unwrap_or_else(|| "(not set)".to_string())
                ),
                "product_id" => println!("{:?}", config.product_id),
                "project_id" => println!("{:?}", config.project_id),
                _ => println!("Unknown key: {}", key),
            }
            Ok(())
        }

        ConfigSubcommand::Unset { key, global } => {
            let path = unset_config(key, *global)?;
            let scope = if *global { "global" } else { "project" };
            println!("✓ {} unset in {} config", key, scope);
            println!("  Removed from: {}", path.display());
            Ok(())
        }
    }
}
