//! Document model combining buffer, frontmatter, and file metadata.

use crate::{Buffer, Frontmatter, History, MarkdownParser};
use comrak::{nodes::AstNode, Arena};
use std::path::PathBuf;
use std::str::FromStr;

/// A document with its buffer, metadata, and editing history.
#[derive(Debug)]
pub struct Document {
    /// The text buffer
    pub buffer: Buffer,
    /// Parsed frontmatter (if any)
    pub frontmatter: Option<Frontmatter>,
    /// File path (None if untitled)
    pub path: Option<PathBuf>,
    /// Undo/redo history
    pub history: History,
    /// Cursor position (line, column)
    pub cursor: (usize, usize),
    /// Scroll offset (for restoring view)
    pub scroll_offset: usize,
    /// Markdown parser (shared instance)
    parser: MarkdownParser,
    /// Cached HTML render (updated lazily)
    cached_html: Option<String>,
    /// Whether the cached HTML is stale
    html_dirty: bool,
}

impl Document {
    /// Create a new empty document
    pub fn new() -> Self {
        Self {
            buffer: Buffer::new(),
            frontmatter: None,
            path: None,
            history: History::new(),
            cursor: (0, 0),
            scroll_offset: 0,
            parser: MarkdownParser::new(),
            cached_html: None,
            html_dirty: true,
        }
    }

    /// Create a document from a file path
    pub fn from_file(path: PathBuf) -> std::io::Result<Self> {
        let content = std::fs::read_to_string(&path)?;
        let mut doc = Self::from_content(&content);
        doc.path = Some(path);
        doc.buffer.mark_saved();
        Ok(doc)
    }

    /// Create a document from a string (convenience wrapper for FromStr)
    pub fn from_content(content: &str) -> Self {
        let (frontmatter, body) = Frontmatter::extract(content);
        Self {
            buffer: Buffer::from_text(body),
            frontmatter,
            path: None,
            history: History::new(),
            cursor: (0, 0),
            scroll_offset: 0,
            parser: MarkdownParser::new(),
            cached_html: None,
            html_dirty: true,
        }
    }
}

impl FromStr for Document {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from_content(s))
    }
}

impl Document {
    /// Save document to its path
    pub fn save(&mut self) -> std::io::Result<()> {
        if let Some(ref path) = self.path {
            let content = self.full_content();
            std::fs::write(path, content)?;
            self.buffer.mark_saved();
            Ok(())
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Document has no path",
            ))
        }
    }

    /// Save document to a new path
    pub fn save_as(&mut self, path: PathBuf) -> std::io::Result<()> {
        self.path = Some(path);
        self.save()
    }

    /// Get the full content including frontmatter
    pub fn full_content(&self) -> String {
        match &self.frontmatter {
            Some(fm) => format!("{}\n{}", fm, self.buffer.text()),
            None => self.buffer.text(),
        }
    }

    /// Get the document title (from frontmatter or filename)
    pub fn title(&self) -> String {
        // Try frontmatter title first
        if let Some(ref fm) = self.frontmatter {
            if let Some(title) = fm.get("title") {
                return title.to_string();
            }
        }

        // Fall back to filename
        if let Some(ref path) = self.path {
            if let Some(name) = path.file_name() {
                return name.to_string_lossy().to_string();
            }
        }

        "Untitled".to_string()
    }

    /// Check if document has unsaved changes
    pub fn is_modified(&self) -> bool {
        self.buffer.is_modified()
    }

    /// Mark the document as needing a re-parse
    pub fn invalidate_cache(&mut self) {
        self.html_dirty = true;
    }

    /// Get the rendered HTML (cached, updates if dirty)
    pub fn html(&mut self) -> &str {
        if self.html_dirty || self.cached_html.is_none() {
            let html = self.parser.to_html(&self.buffer.text());
            self.cached_html = Some(html);
            self.html_dirty = false;
        }
        self.cached_html.as_ref().unwrap()
    }

    /// Parse the document and return AST (for temporary analysis)
    /// Note: Arena must outlive the returned AstNode
    pub fn parse<'a>(&self, arena: &'a Arena<AstNode<'a>>) -> &'a AstNode<'a> {
        self.parser.parse(arena, &self.buffer.text())
    }

    /// Extract headings from the document for outline
    pub fn headings(&self) -> Vec<crate::parser::Heading> {
        self.parser.extract_headings(&self.buffer.text())
    }
}

impl Default for Document {
    fn default() -> Self {
        Self::new()
    }
}
