//! # Patina Extensions
//!
//! Extension modules for Patina:
//! - LaTeX math rendering (to Unicode)
//! - Mermaid diagram rendering (to ASCII art)
//! - Emoji shortcode expansion

pub mod latex;
pub mod mermaid;
pub mod emoji;

pub use latex::LatexRenderer;
pub use mermaid::MermaidRenderer;
pub use emoji::EmojiExpander;
