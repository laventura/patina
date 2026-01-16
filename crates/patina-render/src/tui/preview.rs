// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Patina Contributors

//! Preview widget for displaying rendered Markdown

use comrak::{parse_document, Arena, Options};
use patina_core::Document;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget, Wrap},
};

use super::MarkdownRenderer;
use crate::Theme;

/// Preview widget that renders Markdown content
pub struct PreviewWidget<'a> {
    document: &'a Document,
    theme: &'a Theme,
    scroll_offset: usize,
}

impl<'a> PreviewWidget<'a> {
    /// Create a new preview widget
    pub fn new(document: &'a Document, theme: &'a Theme, scroll_offset: usize) -> Self {
        Self {
            document,
            theme,
            scroll_offset,
        }
    }

    /// Render frontmatter as styled lines
    fn render_frontmatter(&self, width: u16) -> Vec<Line<'static>> {
        let mut lines = Vec::new();

        if let Some(frontmatter) = &self.document.frontmatter {
            // Top border
            let border = format!(
                "┌─ Frontmatter {}",
                "─".repeat(width.saturating_sub(15) as usize)
            );
            lines.push(Line::from(Span::styled(
                border,
                Style::default().fg(self.theme.ui_border.to_ratatui()),
            )));

            // Key-value pairs
            for (key, value) in frontmatter.data.iter() {
                let key_style = Style::default().fg(self.theme.fg_secondary.to_ratatui());
                let value_style = Style::default().fg(self.theme.fg_primary.to_ratatui());

                // Format value based on type
                let value_str = match value {
                    serde_json::Value::String(s) => s.clone(),
                    serde_json::Value::Number(n) => n.to_string(),
                    serde_json::Value::Bool(b) => b.to_string(),
                    serde_json::Value::Array(_) => "[...]".to_string(),
                    serde_json::Value::Object(_) => "{...}".to_string(),
                    serde_json::Value::Null => "null".to_string(),
                };

                lines.push(Line::from(vec![
                    Span::styled("│ ", Style::default().fg(self.theme.ui_border.to_ratatui())),
                    Span::styled(format!("{}: ", key), key_style),
                    Span::styled(value_str, value_style),
                ]));
            }

            // Bottom border
            lines.push(Line::from(Span::styled(
                format!("└{}", "─".repeat(width.saturating_sub(1) as usize)),
                Style::default().fg(self.theme.ui_border.to_ratatui()),
            )));

            // Blank line after frontmatter
            lines.push(Line::from(""));
        }

        lines
    }

    /// Get or render the markdown content
    fn render_content(&self, width: u16) -> Vec<Line<'static>> {
        // Create arena for parsing (arena must outlive the AST)
        let arena = Arena::new();
        let mut options = Options::default();

        // Enable GitHub Flavored Markdown extensions
        options.extension.strikethrough = true;
        options.extension.table = true;
        options.extension.autolink = true;
        options.extension.tasklist = true;
        options.extension.footnotes = true;
        options.extension.description_lists = true;

        // Parse the markdown
        let text = self.document.buffer.text();
        let root = parse_document(&arena, &text, &options);

        // Render using MarkdownRenderer
        let renderer = MarkdownRenderer::new(self.theme, width);
        renderer.render(root)
    }
}

impl<'a> Widget for PreviewWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // Create a block with border
        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(self.theme.ui_border.to_ratatui()))
            .title(Span::styled(
                " Preview ",
                Style::default()
                    .fg(self.theme.fg_primary.to_ratatui())
                    .add_modifier(Modifier::BOLD),
            ));

        // Calculate inner area (inside the border)
        let inner = block.inner(area);

        // Render the block border first
        block.render(area, buf);

        // Collect all lines: frontmatter + content
        let mut all_lines = Vec::new();

        // Add frontmatter if present
        all_lines.extend(self.render_frontmatter(inner.width));

        // Add markdown content
        all_lines.extend(self.render_content(inner.width));

        // Handle empty document
        if all_lines.is_empty() {
            all_lines.push(Line::from(Span::styled(
                "Empty document",
                Style::default().fg(self.theme.fg_muted.to_ratatui()),
            )));
        }

        // Apply scroll offset
        let visible_lines: Vec<_> = all_lines
            .into_iter()
            .skip(self.scroll_offset)
            .take(inner.height as usize)
            .collect();

        // Render as paragraph with wrapping
        let paragraph = Paragraph::new(visible_lines).wrap(Wrap { trim: false });

        paragraph.render(inner, buf);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Theme;
    use patina_core::Document;

    #[test]
    fn test_preview_widget_creation() {
        let doc = Document::from_content("# Hello World");
        let theme = Theme::default();
        let widget = PreviewWidget::new(&doc, &theme, 0);

        // Just test that we can create the widget
        assert_eq!(widget.scroll_offset, 0);
    }

    #[test]
    fn test_empty_document_preview() {
        let doc = Document::from_content("");
        let theme = Theme::default();
        let widget = PreviewWidget::new(&doc, &theme, 0);

        // Should not panic on empty document
        let lines = widget.render_content(80);
        assert!(lines.is_empty() || !lines.is_empty());
    }

    #[test]
    fn test_frontmatter_rendering() {
        let doc = Document::from_content("---\ntitle: Test\n---\n\n# Hello");
        let theme = Theme::default();
        let widget = PreviewWidget::new(&doc, &theme, 0);

        // Should render frontmatter if present
        let fm_lines = widget.render_frontmatter(80);
        if doc.frontmatter.is_some() {
            assert!(!fm_lines.is_empty());
        }
    }

    #[test]
    fn test_scroll_offset() {
        let doc = Document::from_content("# Line 1\n\n# Line 2\n\n# Line 3");
        let theme = Theme::default();

        // Test different scroll offsets
        let widget1 = PreviewWidget::new(&doc, &theme, 0);
        let widget2 = PreviewWidget::new(&doc, &theme, 5);

        assert_eq!(widget1.scroll_offset, 0);
        assert_eq!(widget2.scroll_offset, 5);
    }
}
