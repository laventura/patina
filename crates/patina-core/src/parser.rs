//! Markdown parser using comrak.

use comrak::{
    nodes::AstNode,
    parse_document, Arena, Options,
    plugins::syntect::SyntectAdapter,
};

/// Markdown parser configuration
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
    fn test_gfm_tables() {
        let parser = MarkdownParser::new();
        let html = parser.to_html("| A | B |\n|---|---|\n| 1 | 2 |");
        assert!(html.contains("<table>"));
    }

    #[test]
    fn test_extract_headings() {
        let parser = MarkdownParser::new();
        let headings = parser.extract_headings("# One\n## Two\n### Three");
        
        assert_eq!(headings.len(), 3);
        assert_eq!(headings[0].level, 1);
        assert_eq!(headings[0].text, "One");
        assert_eq!(headings[1].level, 2);
    }
}
