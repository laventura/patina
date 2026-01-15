//! Editor widget for TUI.

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style, Modifier},
    widgets::Widget,
};
use patina_core::Document;
use crate::Theme;

/// Editor widget that renders a document
pub struct EditorWidget<'a> {
    document: &'a Document,
    theme: &'a Theme,
    show_line_numbers: bool,
}

impl<'a> EditorWidget<'a> {
    pub fn new(document: &'a Document, theme: &'a Theme) -> Self {
        Self {
            document,
            theme,
            show_line_numbers: true,
        }
    }

    pub fn line_numbers(mut self, show: bool) -> Self {
        self.show_line_numbers = show;
        self
    }
}

impl<'a> Widget for EditorWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let line_number_width = if self.show_line_numbers {
            // Calculate width needed for line numbers
            let max_line = self.document.buffer.len_lines();
            (max_line.to_string().len() + 2) as u16
        } else {
            0
        };

        let text_area = Rect {
            x: area.x + line_number_width,
            y: area.y,
            width: area.width.saturating_sub(line_number_width),
            height: area.height,
        };

        // Render line numbers
        if self.show_line_numbers {
            let line_num_style = Style::default()
                .fg(Color::Rgb(
                    self.theme.ui_line_number.r,
                    self.theme.ui_line_number.g,
                    self.theme.ui_line_number.b,
                ));

            for (i, y) in (0..area.height).enumerate() {
                let line_num = self.document.scroll_offset + i + 1;
                if line_num <= self.document.buffer.len_lines() {
                    let num_str = format!("{:>width$} ", line_num, width = line_number_width as usize - 2);
                    buf.set_string(area.x, area.y + y as u16, &num_str, line_num_style);
                }
            }
        }

        // Render text content
        let text_style = Style::default()
            .fg(Color::Rgb(
                self.theme.fg_primary.r,
                self.theme.fg_primary.g,
                self.theme.fg_primary.b,
            ));

        for (i, y) in (0..text_area.height).enumerate() {
            let line_idx = self.document.scroll_offset + i;
            if let Some(line) = self.document.buffer.line(line_idx) {
                let display_line: String = line.chars()
                    .take(text_area.width as usize)
                    .collect();
                buf.set_string(text_area.x, text_area.y + y as u16, &display_line, text_style);
            }
        }

        // Render cursor
        let (cursor_line, cursor_col) = self.document.cursor;
        if cursor_line >= self.document.scroll_offset 
            && cursor_line < self.document.scroll_offset + text_area.height as usize 
        {
            let cursor_y = (cursor_line - self.document.scroll_offset) as u16;
            let cursor_x = cursor_col as u16;
            
            if cursor_x < text_area.width && cursor_y < text_area.height {
                let cell = buf.get_mut(text_area.x + cursor_x, text_area.y + cursor_y);
                cell.set_style(Style::default().add_modifier(Modifier::REVERSED));
            }
        }
    }
}
