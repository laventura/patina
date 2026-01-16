//! Markdown parser using comrak.

use comrak::{nodes::AstNode, parse_document, Arena, Options};

/// Markdown parser configuration
#[derive(Debug)]
pub struct MarkdownParser {
    options: Options,
}

impl MarkdownParser {
    /// Create a new parser with default GFM options
    pub fn new() -> Self {
        let mut options = Options::default();

        // Enable GitHub Flavored Markdown extensions
        options.extension.strikethrough = true;
        options.extension.table = true;
        options.extension.autolink = true;
        options.extension.tasklist = true;
        options.extension.footnotes = true;
        options.extension.description_lists = true;

        // Parsing options
        options.parse.smart = true;

        // Render options
        options.render.github_pre_lang = true;
        options.render.unsafe_ = true; // Allow raw HTML

        Self { options }
    }

    /// Parse markdown and return HTML
    pub fn to_html(&self, markdown: &str) -> String {
        let arena = Arena::new();
        let root = parse_document(&arena, markdown, &self.options);

        let mut html = Vec::new();
        comrak::format_html(root, &self.options, &mut html).unwrap();

        String::from_utf8(html).unwrap_or_default()
    }

    /// Parse markdown and return the AST
    pub fn parse<'a>(&self, arena: &'a Arena<AstNode<'a>>, markdown: &str) -> &'a AstNode<'a> {
        parse_document(arena, markdown, &self.options)
    }

    /// Extract headings from markdown for document outline
    pub fn extract_headings(&self, markdown: &str) -> Vec<Heading> {
        let arena = Arena::new();
        let root = parse_document(&arena, markdown, &self.options);

        let mut headings = Vec::new();
        Self::walk_headings(root, &mut headings);
        headings
    }

    fn walk_headings<'a>(node: &'a AstNode<'a>, headings: &mut Vec<Heading>) {
        use comrak::nodes::NodeValue;

        if let NodeValue::Heading(heading) = &node.data.borrow().value {
            let text = Self::extract_text(node);
            let line = node.data.borrow().sourcepos.start.line;
            headings.push(Heading {
                level: heading.level,
                text,
                line,
            });
        }

        for child in node.children() {
            Self::walk_headings(child, headings);
        }
    }

    fn extract_text<'a>(node: &'a AstNode<'a>) -> String {
        use comrak::nodes::NodeValue;

        let mut text = String::new();
        for child in node.children() {
            match &child.data.borrow().value {
                NodeValue::Text(t) => text.push_str(t),
                NodeValue::Code(c) => text.push_str(&c.literal),
                _ => text.push_str(&Self::extract_text(child)),
            }
        }
        text
    }
}

impl Default for MarkdownParser {
    fn default() -> Self {
        Self::new()
    }
}

/// A heading extracted from the document
#[derive(Debug, Clone)]
pub struct Heading {
    /// Heading level (1-6)
    pub level: u8,
    /// Heading text
    pub text: String,
    /// Line number (1-indexed)
    pub line: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_html() {
        let parser = MarkdownParser::new();
        let html = parser.to_html("# Hello\n\nWorld");
        assert!(html.contains("<h1>Hello</h1>"));
        assert!(html.contains("<p>World</p>"));
    }

    #[test]
    fn test_headers() {
        let parser = MarkdownParser::new();
        let md = "# H1\n## H2\n### H3\n#### H4\n##### H5\n###### H6";
        let html = parser.to_html(md);

        assert!(html.contains("<h1>H1</h1>"));
        assert!(html.contains("<h2>H2</h2>"));
        assert!(html.contains("<h3>H3</h3>"));
        assert!(html.contains("<h4>H4</h4>"));
        assert!(html.contains("<h5>H5</h5>"));
        assert!(html.contains("<h6>H6</h6>"));
    }

    #[test]
    fn test_paragraphs() {
        let parser = MarkdownParser::new();
        let md = "First paragraph.\n\nSecond paragraph.\n\nThird paragraph.";
        let html = parser.to_html(md);

        assert!(html.contains("<p>First paragraph.</p>"));
        assert!(html.contains("<p>Second paragraph.</p>"));
        assert!(html.contains("<p>Third paragraph.</p>"));
    }

    #[test]
    fn test_lists_unordered() {
        let parser = MarkdownParser::new();
        let md = "- Item 1\n- Item 2\n- Item 3";
        let html = parser.to_html(md);

        assert!(html.contains("<ul>"));
        assert!(html.contains("<li>Item 1</li>"));
        assert!(html.contains("<li>Item 2</li>"));
        assert!(html.contains("<li>Item 3</li>"));
        assert!(html.contains("</ul>"));
    }

    #[test]
    fn test_lists_ordered() {
        let parser = MarkdownParser::new();
        let md = "1. First\n2. Second\n3. Third";
        let html = parser.to_html(md);

        assert!(html.contains("<ol>"));
        assert!(html.contains("<li>First</li>"));
        assert!(html.contains("<li>Second</li>"));
        assert!(html.contains("<li>Third</li>"));
        assert!(html.contains("</ol>"));
    }

    #[test]
    fn test_links() {
        let parser = MarkdownParser::new();
        let md = "[Link text](https://example.com)";
        let html = parser.to_html(md);

        assert!(html.contains("<a href=\"https://example.com\">Link text</a>"));
    }

    #[test]
    fn test_inline_code() {
        let parser = MarkdownParser::new();
        let md = "This is `inline code` in text.";
        let html = parser.to_html(md);

        assert!(html.contains("<code>inline code</code>"));
    }

    #[test]
    fn test_code_blocks() {
        let parser = MarkdownParser::new();
        let md = "```rust\nfn main() {\n    println!(\"Hello\");\n}\n```";
        let html = parser.to_html(md);

        // comrak may wrap code differently, check for code presence
        assert!(html.contains("fn main()"));
        assert!(html.contains("println"));
    }

    #[test]
    fn test_code_blocks_with_language() {
        let parser = MarkdownParser::new();
        let md = "```python\ndef hello():\n    print('world')\n```";
        let html = parser.to_html(md);

        assert!(html.contains("python"));
        assert!(html.contains("def hello()"));
    }

    #[test]
    fn test_blockquotes() {
        let parser = MarkdownParser::new();
        let md = "> This is a quote\n> Second line";
        let html = parser.to_html(md);

        assert!(html.contains("<blockquote>"));
        assert!(html.contains("This is a quote"));
        assert!(html.contains("</blockquote>"));
    }

    #[test]
    fn test_emphasis() {
        let parser = MarkdownParser::new();
        let md = "*italic* **bold** ***both***";
        let html = parser.to_html(md);

        assert!(html.contains("<em>italic</em>"));
        assert!(html.contains("<strong>bold</strong>"));
    }

    // GFM Extension Tests

    #[test]
    fn test_gfm_tables() {
        let parser = MarkdownParser::new();
        let html = parser.to_html("| A | B |\n|---|---|\n| 1 | 2 |");
        assert!(html.contains("<table>"));
        assert!(html.contains("<th>A</th>"));
        assert!(html.contains("<th>B</th>"));
        assert!(html.contains("<td>1</td>"));
        assert!(html.contains("<td>2</td>"));
    }

    #[test]
    fn test_gfm_strikethrough() {
        let parser = MarkdownParser::new();
        let md = "~~strikethrough text~~";
        let html = parser.to_html(md);

        assert!(html.contains("<del>strikethrough text</del>"));
    }

    #[test]
    fn test_gfm_task_lists() {
        let parser = MarkdownParser::new();
        let md = "- [ ] Unchecked\n- [x] Checked";
        let html = parser.to_html(md);

        assert!(html.contains("checkbox"));
        assert!(html.contains("Unchecked"));
        assert!(html.contains("Checked"));
    }

    #[test]
    fn test_gfm_autolink() {
        let parser = MarkdownParser::new();
        let md = "https://www.example.com";
        let html = parser.to_html(md);

        assert!(html.contains("<a href=\"https://www.example.com\">"));
    }

    // Heading extraction tests

    #[test]
    fn test_extract_headings() {
        let parser = MarkdownParser::new();
        let headings = parser.extract_headings("# One\n## Two\n### Three");

        assert_eq!(headings.len(), 3);
        assert_eq!(headings[0].level, 1);
        assert_eq!(headings[0].text, "One");
        assert_eq!(headings[1].level, 2);
        assert_eq!(headings[1].text, "Two");
        assert_eq!(headings[2].level, 3);
        assert_eq!(headings[2].text, "Three");
    }

    #[test]
    fn test_extract_headings_with_inline_formatting() {
        let parser = MarkdownParser::new();
        let headings = parser.extract_headings("# **Bold** heading\n## Heading with `code`");

        assert_eq!(headings.len(), 2);
        assert_eq!(headings[0].text, "Bold heading");
        assert_eq!(headings[1].text, "Heading with code");
    }

    #[test]
    fn test_extract_headings_line_numbers() {
        let parser = MarkdownParser::new();
        let md = "# First\n\nSome text\n\n## Second\n\nMore text\n\n### Third";
        let headings = parser.extract_headings(md);

        assert_eq!(headings.len(), 3);
        assert_eq!(headings[0].line, 1);
        assert_eq!(headings[1].line, 5);
        assert_eq!(headings[2].line, 9);
    }

    // Edge case tests

    #[test]
    fn test_empty_input() {
        let parser = MarkdownParser::new();
        let html = parser.to_html("");

        assert_eq!(html.trim(), "");
    }

    #[test]
    fn test_whitespace_only() {
        let parser = MarkdownParser::new();
        let html = parser.to_html("   \n\n   \n");

        // Should produce minimal/empty output
        assert!(html.len() < 50);
    }

    #[test]
    fn test_malformed_table() {
        let parser = MarkdownParser::new();
        // Table with mismatched columns
        let md = "| A | B |\n|---|\n| 1 | 2 | 3 |";
        let html = parser.to_html(md);

        // Should still parse without crashing
        assert!(!html.is_empty());
    }

    #[test]
    fn test_nested_lists() {
        let parser = MarkdownParser::new();
        let md = "- Item 1\n  - Nested 1\n  - Nested 2\n- Item 2";
        let html = parser.to_html(md);

        assert!(html.contains("<ul>"));
        assert!(html.contains("Item 1"));
        assert!(html.contains("Nested 1"));
    }

    #[test]
    fn test_mixed_content() {
        let parser = MarkdownParser::new();
        let md = r#"# Title

This is a paragraph with **bold** and *italic*.

## Code Example

```rust
fn main() {}
```

## List

- Item 1
- Item 2

> Quote

| Table | Header |
|-------|--------|
| Cell  | Data   |
"#;
        let html = parser.to_html(md);

        assert!(html.contains("<h1>Title</h1>"));
        assert!(html.contains("<strong>bold</strong>"));
        assert!(html.contains("<em>italic</em>"));
        assert!(html.contains("fn main()"));
        assert!(html.contains("<ul>"));
        assert!(html.contains("<blockquote>"));
        assert!(html.contains("<table>"));
    }

    #[test]
    fn test_html_escaping() {
        let parser = MarkdownParser::new();
        let md = "Text with <script>alert('xss')</script>";
        let html = parser.to_html(md);

        // With unsafe_ = true, HTML should be preserved
        assert!(html.contains("<script>"));
    }

    #[test]
    fn test_unicode_content() {
        let parser = MarkdownParser::new();
        let md = "# Hello 世界\n\nEmoji: 🚀 ✨ 💻";
        let html = parser.to_html(md);

        assert!(html.contains("世界"));
        assert!(html.contains("🚀"));
        assert!(html.contains("✨"));
        assert!(html.contains("💻"));
    }
}
