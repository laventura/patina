//! UI rendering.

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Tabs},
    Frame,
};

use patina_render::tui::{App, EditorWidget, ViewMode};

/// Draw the entire UI
pub fn draw(frame: &mut Frame, app: &App) {
    if app.zen_mode {
        draw_zen_mode(frame, app);
    } else {
        draw_normal_mode(frame, app);
    }
}

/// Draw normal mode UI
fn draw_normal_mode(frame: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),  // Tab bar
            Constraint::Min(1),     // Editor area
            Constraint::Length(1),  // Status bar
        ])
        .split(frame.area());

    draw_tab_bar(frame, chunks[0], app);
    draw_editor_area(frame, chunks[1], app);
    draw_status_bar(frame, chunks[2], app);
}

/// Draw Zen mode UI (minimal, centered)
fn draw_zen_mode(frame: &mut Frame, app: &App) {
    let area = frame.area();
    
    // Center the content
    let zen_width = 80.min(area.width.saturating_sub(4));
    let x_offset = (area.width.saturating_sub(zen_width)) / 2;
    
    let zen_area = Rect {
        x: x_offset,
        y: 1,
        width: zen_width,
        height: area.height.saturating_sub(2),
    };

    let doc = app.active_document();
    let editor = EditorWidget::new(doc, &app.theme)
        .line_numbers(false);
    
    frame.render_widget(editor, zen_area);
}

/// Draw the tab bar
fn draw_tab_bar(frame: &mut Frame, area: Rect, app: &App) {
    let titles: Vec<Line> = app.documents
        .iter()
        .map(|doc| {
            let title = doc.title();
            let modified = if doc.is_modified() { " •" } else { "" };
            Line::from(format!(" {}{} ", title, modified))
        })
        .collect();

    let tabs = Tabs::new(titles)
        .select(app.active_doc)
        .style(Style::default().fg(Color::Rgb(
            app.theme.fg_muted.r,
            app.theme.fg_muted.g,
            app.theme.fg_muted.b,
        )))
        .highlight_style(Style::default()
            .fg(Color::Rgb(
                app.theme.fg_primary.r,
                app.theme.fg_primary.g,
                app.theme.fg_primary.b,
            ))
            .add_modifier(Modifier::BOLD)
        )
        .divider("|");

    frame.render_widget(tabs, area);
}

/// Draw the editor area
fn draw_editor_area(frame: &mut Frame, area: Rect, app: &App) {
    let doc = app.active_document();
    
    match app.view_mode {
        ViewMode::Raw => {
            let editor = EditorWidget::new(doc, &app.theme);
            frame.render_widget(editor, area);
        }
        ViewMode::Rendered => {
            // TODO: Rendered view
            let block = Block::default()
                .title("Preview")
                .borders(Borders::ALL);
            frame.render_widget(block, area);
        }
        ViewMode::Split => {
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Percentage(50),
                    Constraint::Percentage(50),
                ])
                .split(area);

            // Left: Raw editor
            let editor = EditorWidget::new(doc, &app.theme);
            frame.render_widget(editor, chunks[0]);

            // Right: Preview
            let preview = Block::default()
                .title("Preview")
                .borders(Borders::LEFT);
            frame.render_widget(preview, chunks[1]);
        }
    }
}

/// Draw the status bar
fn draw_status_bar(frame: &mut Frame, area: Rect, app: &App) {
    let doc = app.active_document();
    
    let mode = match app.view_mode {
        ViewMode::Raw => "RAW",
        ViewMode::Rendered => "PREVIEW",
        ViewMode::Split => "SPLIT",
    };
    
    let status = format!(
        " {} │ Ln {}, Col {} │ {} │ {} ",
        if doc.is_modified() { "●" } else { "○" },
        doc.cursor.0 + 1,
        doc.cursor.1 + 1,
        mode,
        app.theme.name,
    );

    let status_style = Style::default()
        .bg(Color::Rgb(
            app.theme.ui_status_bar.r,
            app.theme.ui_status_bar.g,
            app.theme.ui_status_bar.b,
        ))
        .fg(Color::Rgb(
            app.theme.fg_primary.r,
            app.theme.fg_primary.g,
            app.theme.fg_primary.b,
        ));

    let paragraph = Paragraph::new(status).style(status_style);
    frame.render_widget(paragraph, area);
}
