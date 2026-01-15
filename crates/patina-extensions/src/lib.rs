//! # Patina Extensions
//!
//! Extension modules for Patina:
//! - LaTeX math rendering (to Unicode)
//! - Mermaid diagram rendering (to ASCII art)
//! - Emoji shortcode expansion

pub mod emoji;
pub mod latex;
pub mod mermaid;

pub use emoji::EmojiExpander;
pub use latex::LatexRenderer;
pub use mermaid::MermaidRenderer;
