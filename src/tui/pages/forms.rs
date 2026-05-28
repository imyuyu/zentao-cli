//! 表单渲染模块
//!
//! 提供 Program/ProductPlan/Release 表单的 TUI 渲染

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::tui::app::AppState;
use crate::tui::forms::FormField;

/// 渲染 Program 创建表单
pub fn render_program_create(f: &mut Frame, area: Rect, app: &crate::tui::app::App) {
    if let AppState::ProgramCreate { ref fields } = app.state {
        render_form(
            f,
            area,
            "创建项目集",
            fields.get_fields(),
            app.selected_index,
        );
    }
}

/// 渲染 Program 更新表单
pub fn render_program_update(f: &mut Frame, area: Rect, app: &crate::tui::app::App) {
    if let AppState::ProgramUpdate { ref fields, .. } = app.state {
        render_form(
            f,
            area,
            "编辑项目集",
            fields.get_fields(),
            app.selected_index,
        );
    }
}

/// 渲染 ProductPlan 创建表单
pub fn render_productplan_create(f: &mut Frame, area: Rect, app: &crate::tui::app::App) {
    if let AppState::ProductPlanCreate { ref fields, .. } = app.state {
        render_form(
            f,
            area,
            "创建产品计划",
            fields.get_fields(),
            app.selected_index,
        );
    }
}

/// 渲染 ProductPlan 更新表单
pub fn render_productplan_update(f: &mut Frame, area: Rect, app: &crate::tui::app::App) {
    if let AppState::ProductPlanUpdate { ref fields, .. } = app.state {
        render_form(
            f,
            area,
            "编辑产品计划",
            fields.get_fields(),
            app.selected_index,
        );
    }
}

/// 渲染 Release 创建表单
pub fn render_release_create(f: &mut Frame, area: Rect, app: &crate::tui::app::App) {
    if let AppState::ReleaseCreate { ref fields, .. } = app.state {
        render_form(f, area, "创建发布", fields.get_fields(), app.selected_index);
    }
}

/// 渲染 Release 更新表单
pub fn render_release_update(f: &mut Frame, area: Rect, app: &crate::tui::app::App) {
    if let AppState::ReleaseUpdate { ref fields, .. } = app.state {
        render_form(f, area, "编辑发布", fields.get_fields(), app.selected_index);
    }
}

/// 通用表单渲染
fn render_form(
    f: &mut Frame,
    area: Rect,
    title: &str,
    fields: Vec<&FormField>,
    selected_field: usize,
) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Header
            Constraint::Min(0),    // Form fields
            Constraint::Length(3), // Footer
        ])
        .split(area);

    // Header
    let header = Paragraph::new(Line::from(vec![Span::raw("  "), Span::raw(title)]))
        .block(Block::default().borders(Borders::ALL).title(""));

    f.render_widget(header, chunks[0]);

    // Form fields
    let field_area = chunks[1];
    let field_height = std::cmp::max(1, field_area.height / fields.len() as u16);

    for (i, field) in fields.iter().enumerate() {
        let row_top = i as u16 * field_height;
        if row_top >= field_area.height {
            break;
        }

        let row_area = Rect::new(
            field_area.x,
            field_area.y + row_top,
            field_area.width,
            field_height,
        );

        let is_selected = i == selected_field;

        let label_style = if is_selected {
            ratatui::style::Style::default()
                .fg(ratatui::style::Color::Yellow)
                .bg(ratatui::style::Color::DarkGray)
        } else {
            ratatui::style::Style::default().fg(ratatui::style::Color::Cyan)
        };

        let value_style = if field.editable {
            if is_selected {
                ratatui::style::Style::default()
                    .fg(ratatui::style::Color::White)
                    .bg(ratatui::style::Color::Blue)
            } else {
                ratatui::style::Style::default().fg(ratatui::style::Color::White)
            }
        } else {
            ratatui::style::Style::default().fg(ratatui::style::Color::DarkGray)
        };

        let required_mark = if field.is_required { " *" } else { "" };
        let display_value = if field.value.is_empty() {
            field.placeholder.as_str()
        } else {
            field.value.as_str()
        };

        let line = Line::from(vec![
            Span::styled(format!("  {}:{}", field.label, required_mark), label_style),
            Span::raw(" "),
            Span::styled(display_value, value_style),
        ]);

        let field_widget = Paragraph::new(line)
            .block(Block::default().borders(Borders::LEFT | Borders::RIGHT | Borders::BOTTOM));

        f.render_widget(field_widget, row_area);
    }

    // Footer
    let footer = Paragraph::new(Line::from(vec![
        Span::raw("  "),
        Span::raw("Tab/Shift+Tab: 导航 | Enter: 提交 | Esc: 取消"),
    ]))
    .block(Block::default().borders(Borders::ALL).title(""));

    f.render_widget(footer, chunks[2]);
}

/// 渲染删除确认对话框
pub fn render_delete_dialog(f: &mut Frame, area: Rect, app: &crate::tui::app::App) {
    let (id, name, entity_type) = match &app.state {
        AppState::ProgramDelete { id, name } => (*id, name.clone(), "项目集"),
        AppState::ProductPlanDelete { id, name } => (*id, name.clone(), "产品计划"),
        AppState::ReleaseDelete { id, name } => (*id, name.clone(), "发布"),
        _ => return,
    };

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(5), // Title
            Constraint::Length(3), // Content
            Constraint::Length(3), // Footer
        ])
        .margin(1)
        .split(area);

    // Dialog box
    let dialog_area = chunks[0];

    let title = Paragraph::new(Line::from(vec![Span::raw("  确认删除")]))
        .block(Block::default().borders(Borders::ALL).title(""));

    let content = Paragraph::new(Line::from(vec![
        Span::raw("  "),
        Span::raw(format!(
            "确定要删除 {} \"{}\" (ID: {}) 吗？",
            entity_type, name, id
        )),
    ]));

    let footer = Paragraph::new(Line::from(vec![
        Span::raw("  "),
        Span::raw("Enter: 确认删除 | Esc: 取消"),
    ]));

    f.render_widget(title, dialog_area);
    f.render_widget(content, chunks[1]);
    f.render_widget(footer, chunks[2]);
}
