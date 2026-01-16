//! # Patina Core
//!
//! Core library for Patina markdown editor.
//! Contains the text buffer, markdown parser, document model, and editing operations.

pub mod buffer;
pub mod document;
pub mod frontmatter;
pub mod history;
pub mod parser;
pub mod selection;
pub mod syntax;

// Re-exports for convenience
pub use buffer::Buffer;
pub use document::Document;
pub use frontmatter::Frontmatter;
pub use history::{Edit, History};
pub use parser::MarkdownParser;
pub use selection::Selection;

/// Core result type
pub type Result<T> = std::result::Result<T, Error>;

/// Core error type
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Buffer error: {0}")]
    Buffer(String),

    #[error("Parse error: {0}")]
    Parse(String),

    #[error("Frontmatter error: {0}")]
    Frontmatter(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}
