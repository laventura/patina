// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Patina Contributors

//! Markdown rendering to terminal UI
//!
//! Converts comrak AST to styled ratatui text that can be displayed in the preview pane.

use comrak::nodes::{AstNode, ListType, NodeValue};
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};

use crate::Theme;

/// A styled line for rendering (using owned data)
pub type StyledLine = Line<'static>;

/// Markdown renderer that converts AST to styled terminal text
pub struct MarkdownRenderer<'a> {
    theme: &'a Theme,
    width: u16,
}

/// Rendering context for tracking state during AST walk
struct RenderContext {
    /// Current list depth for indentation
    list_depth: usize,
    /// Current list item number (for ordered lists)
    list_number: usize,
    /// Whether we're inside a list
    in_list: bool,
}

impl<'a> MarkdownRenderer<'a> {
    /// Create a new renderer with the given theme and width
    pub fn new(theme: &'a Theme, width: u16) -> Self {
        Self { theme, width }
    }

    /// Render a markdown AST to styled lines
    pub fn render(&self, root: &'a AstNode<'a>) -> Vec<StyledLine> {
        let mut lines = Vec::new();
        let mut context = RenderContext {
            list_depth: 0,
            list_number: 0,
            in_list: false,
        };

        self.render_node(root, &mut lines, &mut context);
        lines
    }

    /// Render a single AST node and its children
    fn render_node(
        &self,
        node: &'a AstNode<'a>,
        lines: &mut Vec<StyledLine>,
        context: &mut RenderContext,
    ) {
        let ast = node.data.borrow();

        match &ast.value {
            NodeValue::Document => {
                // Document is the root, just render children
                for child in node.children() {
                    self.render_node(child, lines, context);
                }
            }

            NodeValue::Heading(heading) => {
                self.render_heading(node, heading.level, lines);
            }

            NodeValue::Paragraph => {
                self.render_paragraph(node, lines, context);
            }

            NodeValue::CodeBlock(code_block) => {
                self.render_code_block(&code_block.info, &code_block.literal, lines);
            }

            NodeValue::BlockQuote => {
                self.render_blockquote(node, lines, context);
            }

            NodeValue::List(list) => {
                self.render_list(node, list, lines, context);
            }

            NodeValue::Item(_) => {
                // Items are rendered as part of list handling
                self.render_list_item(node, lines, context);
            }

            NodeValue::ThematicBreak => {
                // Horizontal rule
                let hr = "─".repeat(self.width as usize);
                lines.push(Line::from(Span::styled(
                    hr,
                    Style::default().fg(self.theme.ui_border.to_ratatui()),
                )));
                lines.push(Line::from(""));
            }

            NodeValue::Table(_) => {
                // Tables are complex - for now render as plaintext
                // TODO: Implement proper table rendering
                self.render_table(node, lines, context);
            }

            _ => {
                // For other node types, recurse to children
                for child in node.children() {
                    self.render_node(child, lines, context);
                }
            }
        }
    }

    /// Render a heading
    fn render_heading(&self, node: &'a AstNode<'a>, level: u8, lines: &mut Vec<StyledLine>) {
        let text = self.extract_text(node);

        // Different styles for different heading levels
        let style = Style::default()
            .fg(self.theme.md_heading.to_ratatui())
            .add_modifier(Modifier::BOLD);

        // Add visual hierarchy with distinct Unicode block markers
        let prefix = match level {
            1 => "█ ", // H1 - full block (largest)
            2 => "▓ ", // H2 - dark shade
            3 => "▒ ", // H3 - medium shade
            4 => "░ ", // H4 - light shade
            5 => "▪ ", // H5 - small square
            _ => "▫ ", // H6 - white small square
        };

        lines.push(Line::from("")); // Blank line before
        lines.push(Line::from(Span::styled(
            format!("{}{}", prefix, text),
            style,
        )));
        lines.push(Line::from("")); // Blank line after
    }

    /// Render a paragraph
    fn render_paragraph(
        &self,
        node: &'a AstNode<'a>,
        lines: &mut Vec<StyledLine>,
        context: &RenderContext,
    ) {
        let spans = self.render_inline_content(node);

        // Apply list indentation if in a list
        let indent = if context.in_list {
            "  ".repeat(context.list_depth)
        } else {
            String::new()
        };

        if !indent.is_empty() {
            let mut indented_spans = vec![Span::raw(indent)];
            indented_spans.extend(spans);
            lines.push(Line::from(indented_spans));
        } else {
            lines.push(Line::from(spans));
        }

        if !context.in_list {
            lines.push(Line::from("")); // Blank line after paragraph
        }
    }

    /// Render inline content (text with formatting)
    fn render_inline_content(&self, node: &'a AstNode<'a>) -> Vec<Span<'static>> {
        let mut spans = Vec::new();

        for child in node.children() {
            self.collect_inline_spans(child, &mut spans, Style::default());
        }

        spans
    }

    /// Recursively collect inline spans
    fn collect_inline_spans(
        &self,
        node: &'a AstNode<'a>,
        spans: &mut Vec<Span<'static>>,
        inherited_style: Style,
    ) {
        let ast = node.data.borrow();

        match &ast.value {
            NodeValue::Text(text) => {
                spans.push(Span::styled(
                    text.to_string(),
                    inherited_style.fg(self.theme.fg_primary.to_ratatui()),
                ));
            }

            NodeValue::Code(code) => {
                spans.push(Span::styled(
                    format!(" {} ", code.literal),
                    Style::default()
                        .fg(self.theme.md_code.to_ratatui())
                        .bg(self.theme.bg_secondary.to_ratatui()),
                ));
            }

            NodeValue::Strong => {
                let new_style = inherited_style
                    .fg(self.theme.md_bold.to_ratatui())
                    .add_modifier(Modifier::BOLD);
                for child in node.children() {
                    self.collect_inline_spans(child, spans, new_style);
                }
            }

            NodeValue::Emph => {
                let new_style = inherited_style
                    .fg(self.theme.md_italic.to_ratatui())
                    .add_modifier(Modifier::ITALIC);
                for child in node.children() {
                    self.collect_inline_spans(child, spans, new_style);
                }
            }

            NodeValue::Strikethrough => {
                let new_style = inherited_style.add_modifier(Modifier::CROSSED_OUT);
                for child in node.children() {
                    self.collect_inline_spans(child, spans, new_style);
                }
            }

            NodeValue::Link(link) => {
                // Render link text in blue and underlined
                let link_style = Style::default()
                    .fg(self.theme.md_link.to_ratatui())
                    .add_modifier(Modifier::UNDERLINED);

                for child in node.children() {
                    self.collect_inline_spans(child, spans, link_style);
                }

                // Show URL in muted color
                spans.push(Span::styled(
                    format!(" ({})", link.url),
                    Style::default().fg(self.theme.fg_muted.to_ratatui()),
                ));
            }

            NodeValue::Image(image) => {
                // Show image alt text with icon
                spans.push(Span::styled(
                    "🖼 ",
                    Style::default().fg(self.theme.fg_secondary.to_ratatui()),
                ));
                for child in node.children() {
                    self.collect_inline_spans(child, spans, inherited_style);
                }
                spans.push(Span::styled(
                    format!(" [{}]", image.url),
                    Style::default().fg(self.theme.fg_muted.to_ratatui()),
                ));
            }

            NodeValue::SoftBreak | NodeValue::LineBreak => {
                spans.push(Span::raw(" "));
            }

            _ => {
                // Recurse for other inline elements
                for child in node.children() {
                    self.collect_inline_spans(child, spans, inherited_style);
                }
            }
        }
    }

    /// Render a code block (with syntax highlighting)
    fn render_code_block(&self, info: &str, literal: &str, lines: &mut Vec<StyledLine>) {
        use patina_core::Highlighter;
        use ratatui::style::Color;

        // Language label
        let lang = info.split_whitespace().next().unwrap_or("");
        if !lang.is_empty() {
            lines.push(Line::from(Span::styled(
                format!("▸ {}", lang),
                Style::default()
                    .fg(self.theme.fg_muted.to_ratatui())
                    .add_modifier(Modifier::ITALIC),
            )));
        }

        // Try syntax highlighting
        let highlighter = Highlighter::new("base16-ocean.dark");
        if let Some(syntax) = highlighter.syntax_for_language(lang) {
            // Collect lines as string slices
            let code_lines: Vec<&str> = literal.lines().collect();

            // Highlight all lines
            let highlighted_lines = highlighter.highlight_lines(&code_lines, syntax);

            // Convert syntect highlighted lines to ratatui styled lines
            for hl_line in highlighted_lines {
                let mut spans = vec![Span::raw("  ")]; // Indent

                for (style, text) in hl_line {
                    // Convert syntect RGB to ratatui Color
                    let fg = Color::Rgb(style.foreground.r, style.foreground.g, style.foreground.b);
                    let bg = Color::Rgb(style.background.r, style.background.g, style.background.b);

                    spans.push(Span::styled(
                        text.to_string(),
                        Style::default().fg(fg).bg(bg),
                    ));
                }

                lines.push(Line::from(spans));
            }
        } else {
            // Fallback to plain code block
            let code_style = Style::default()
                .fg(self.theme.md_code.to_ratatui())
                .bg(self.theme.bg_secondary.to_ratatui());

            for line in literal.lines() {
                lines.push(Line::from(Span::styled(format!("  {}", line), code_style)));
            }
        }

        lines.push(Line::from(""));
    }

    /// Render a blockquote
    fn render_blockquote(
        &self,
        node: &'a AstNode<'a>,
        lines: &mut Vec<StyledLine>,
        context: &mut RenderContext,
    ) {
        let border_style = Style::default().fg(self.theme.md_blockquote.to_ratatui());

        // Render children with a border
        let start_idx = lines.len();
        for child in node.children() {
            self.render_node(child, lines, context);
        }

        // Add border to all lines in the blockquote
        for line in lines.iter_mut().skip(start_idx) {
            let mut new_spans = vec![Span::styled("│ ", border_style)];
            new_spans.extend(line.spans.iter().cloned());
            *line = Line::from(new_spans);
        }

        lines.push(Line::from(""));
    }

    /// Render a list
    fn render_list(
        &self,
        node: &'a AstNode<'a>,
        list: &comrak::nodes::NodeList,
        lines: &mut Vec<StyledLine>,
        context: &mut RenderContext,
    ) {
        context.in_list = true;
        context.list_depth += 1;
        context.list_number = list.start;

        for child in node.children() {
            self.render_node(child, lines, context);
            if matches!(list.list_type, ListType::Ordered) {
                context.list_number += 1;
            }
        }

        context.list_depth -= 1;
        if context.list_depth == 0 {
            context.in_list = false;
            lines.push(Line::from(""));
        }
    }

    /// Render a list item
    fn render_list_item(
        &self,
        node: &'a AstNode<'a>,
        lines: &mut Vec<StyledLine>,
        context: &mut RenderContext,
    ) {
        let indent = "  ".repeat(context.list_depth.saturating_sub(1));

        // Check for task list marker (checkbox)
        let mut is_task_list = false;
        let mut is_checked = false;
        for child in node.children() {
            if let NodeValue::TaskItem(symbol) = &child.data.borrow().value {
                is_task_list = true;
                is_checked = symbol.is_some() && *symbol != Some(' ');
                break;
            }
        }

        // Determine bullet/number/checkbox
        let marker = if is_task_list {
            if is_checked {
                "[✓] ".to_string()
            } else {
                "[ ] ".to_string()
            }
        } else if context.in_list {
            // Check parent list type
            if let Some(parent) = node.parent() {
                if let NodeValue::List(list) = &parent.data.borrow().value {
                    match list.list_type {
                        ListType::Ordered => format!("{}. ", context.list_number),
                        ListType::Bullet => "• ".to_string(),
                    }
                } else {
                    "• ".to_string()
                }
            } else {
                "• ".to_string()
            }
        } else {
            "• ".to_string()
        };

        // Collect content spans for the first paragraph
        let start_line_idx = lines.len();

        // Render item content
        for child in node.children() {
            self.render_node(child, lines, context);
        }

        // Prepend marker to the first line of content
        if lines.len() > start_line_idx {
            let first_content_line = &lines[start_line_idx];
            let mut new_spans = vec![Span::styled(
                format!("{}{}", indent, marker),
                Style::default().fg(self.theme.fg_secondary.to_ratatui()),
            )];
            new_spans.extend(first_content_line.spans.iter().cloned());
            lines[start_line_idx] = Line::from(new_spans);
        }
    }

    /// Render a table (simplified for now)
    fn render_table(
        &self,
        node: &'a AstNode<'a>,
        lines: &mut Vec<StyledLine>,
        context: &mut RenderContext,
    ) {
        // Simple table rendering - just show the content
        // TODO: Implement proper table layout with borders
        for child in node.children() {
            self.render_node(child, lines, context);
        }
        lines.push(Line::from(""));
    }

    /// Extract plain text from a node (recursive)
    fn extract_text(&self, node: &'a AstNode<'a>) -> String {
        let mut text = String::new();
        for child in node.children() {
            let ast = child.data.borrow();
            match &ast.value {
                NodeValue::Text(t) => text.push_str(t),
                NodeValue::Code(c) => text.push_str(&c.literal),
                _ => text.push_str(&self.extract_text(child)),
            }
        }
        text
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Theme;
    use comrak::{parse_document, Arena, Options};

    fn render_markdown(md: &str) -> Vec<StyledLine> {
        let arena = Arena::new();
        let options = Options::default();
        let root = parse_document(&arena, md, &options);

        let theme = Theme::default();
        let renderer = MarkdownRenderer::new(&theme, 80);
        renderer.render(root)
    }

    #[test]
    fn test_heading_renders() {
        let lines = render_markdown("# Hello World");
        assert!(!lines.is_empty());
        // Should have blank line, heading, blank line
        assert!(lines.len() >= 3);
    }

    #[test]
    fn test_paragraph_renders() {
        let lines = render_markdown("This is a paragraph.");
        assert!(!lines.is_empty());
    }

    #[test]
    fn test_code_block_renders() {
        let lines = render_markdown("```rust\nfn main() {}\n```");
        assert!(!lines.is_empty());
    }

    #[test]
    fn test_list_renders() {
        let lines = render_markdown("- Item 1\n- Item 2\n- Item 3");
        assert!(!lines.is_empty());
    }

    #[test]
    fn test_bold_and_italic() {
        let lines = render_markdown("**bold** and *italic* text");
        assert!(!lines.is_empty());
    }

    #[test]
    fn test_link_renders() {
        let lines = render_markdown("[Example](https://example.com)");
        assert!(!lines.is_empty());
    }

    #[test]
    fn test_empty_document() {
        let lines = render_markdown("");
        // Empty document should not panic
        assert!(lines.is_empty() || lines.len() == 1);
    }
}
