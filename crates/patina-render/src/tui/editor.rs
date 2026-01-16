//! Editor widget for TUI.

use crate::Theme;
use patina_core::Document;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    widgets::Widget,
};

/// Editor widget that renders a document
pub struct EditorWidget<'a> {
    document: &'a Document,
    theme: &'a Theme,
    show_line_numbers: bool,
    soft_wrap: bool,
}

impl<'a> EditorWidget<'a> {
    pub fn new(document: &'a Document, theme: &'a Theme) -> Self {
        Self {
            document,
            theme,
            show_line_numbers: true,
            soft_wrap: true,
        }
    }

    pub fn line_numbers(mut self, show: bool) -> Self {
        self.show_line_numbers = show;
        self
    }

    pub fn soft_wrap(mut self, wrap: bool) -> Self {
        self.soft_wrap = wrap;
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

        let text_width = text_area.width as usize;
        if text_width == 0 {
            return;
        }

        let text_style = Style::default().fg(Color::Rgb(
            self.theme.fg_primary.r,
            self.theme.fg_primary.g,
            self.theme.fg_primary.b,
        ));

        let line_num_style = Style::default().fg(Color::Rgb(
            self.theme.ui_line_number.r,
            self.theme.ui_line_number.g,
            self.theme.ui_line_number.b,
        ));

        let (cursor_line, cursor_col) = self.document.cursor;
        let mut screen_row: u16 = 0;
        let mut doc_line = self.document.scroll_offset;

        // Track where cursor should be rendered
        let mut cursor_screen_pos: Option<(u16, u16)> = None;

        while screen_row < text_area.height && doc_line < self.document.buffer.len_lines() {
            let line_content = self.document.buffer.line(doc_line).unwrap_or_default();
            let line_chars: Vec<char> = line_content.trim_end_matches('\n').chars().collect();

            if self.soft_wrap && !line_chars.is_empty() {
                // Soft wrap: split line into chunks
                let chunks: Vec<&[char]> = line_chars.chunks(text_width).collect();
                let num_chunks = chunks.len().max(1);

                for (chunk_idx, chunk) in chunks.iter().enumerate() {
                    if screen_row >= text_area.height {
                        break;
                    }

                    // Line number only on first chunk
                    if self.show_line_numbers && chunk_idx == 0 {
                        let num_str = format!(
                            "{:>width$} ",
                            doc_line + 1,
                            width = line_number_width as usize - 2
                        );
                        buf.set_string(area.x, area.y + screen_row, &num_str, line_num_style);
                    }

                    // Render text chunk
                    let display: String = chunk.iter().collect();
                    buf.set_string(text_area.x, text_area.y + screen_row, &display, text_style);

                    // Check if cursor is in this chunk
                    if doc_line == cursor_line {
                        let chunk_start = chunk_idx * text_width;
                        let chunk_end = chunk_start + chunk.len();
                        if cursor_col >= chunk_start && cursor_col <= chunk_end {
                            let cursor_x = (cursor_col - chunk_start) as u16;
                            cursor_screen_pos = Some((cursor_x, screen_row));
                        }
                    }

                    screen_row += 1;
                }

                // Handle cursor at end of line (past last char)
                if doc_line == cursor_line && cursor_col >= line_chars.len() {
                    let last_chunk_idx = num_chunks.saturating_sub(1);
                    let chunk_start = last_chunk_idx * text_width;
                    let cursor_x = (cursor_col - chunk_start) as u16;
                    if cursor_x < text_width as u16 {
                        cursor_screen_pos =
                            Some((cursor_x, (screen_row - 1).min(text_area.height - 1)));
                    }
                }
            } else {
                // No wrap or empty line
                if self.show_line_numbers {
                    let num_str = format!(
                        "{:>width$} ",
                        doc_line + 1,
                        width = line_number_width as usize - 2
                    );
                    buf.set_string(area.x, area.y + screen_row, &num_str, line_num_style);
                }

                let display: String = line_chars.iter().take(text_width).collect();
                buf.set_string(text_area.x, text_area.y + screen_row, &display, text_style);

                // Cursor position for non-wrapped line
                if doc_line == cursor_line && cursor_col < text_width {
                    cursor_screen_pos = Some((cursor_col as u16, screen_row));
                }

                screen_row += 1;
            }

            doc_line += 1;
        }

        // Render cursor
        if let Some((cx, cy)) = cursor_screen_pos {
            if cx < text_area.width && cy < text_area.height {
                buf[(text_area.x + cx, text_area.y + cy)]
                    .set_style(Style::default().add_modifier(Modifier::REVERSED));
            }
        }
    }
}
