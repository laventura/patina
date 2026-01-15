//! Syntax highlighting using syntect.

use once_cell::sync::Lazy;
use syntect::easy::HighlightLines;
use syntect::highlighting::{Style, Theme, ThemeSet};
use syntect::parsing::{SyntaxReference, SyntaxSet};

/// Global syntax set (loaded once)
static SYNTAX_SET: Lazy<SyntaxSet> = Lazy::new(|| SyntaxSet::load_defaults_newlines());

/// Global theme set
static THEME_SET: Lazy<ThemeSet> = Lazy::new(|| ThemeSet::load_defaults());

/// A syntax highlighter
pub struct Highlighter {
    theme_name: String,
}

impl Highlighter {
    /// Create a highlighter with a theme
    pub fn new(theme_name: &str) -> Self {
        Self {
            theme_name: theme_name.to_string(),
        }
    }

    /// Get the current theme
    pub fn theme(&self) -> &Theme {
        THEME_SET
            .themes
            .get(&self.theme_name)
            .unwrap_or_else(|| THEME_SET.themes.get("base16-ocean.dark").unwrap())
    }

    /// Get syntax for a language
    pub fn syntax_for_language(&self, lang: &str) -> Option<&SyntaxReference> {
        SYNTAX_SET
            .find_syntax_by_token(lang)
            .or_else(|| SYNTAX_SET.find_syntax_by_extension(lang))
    }

    /// Get syntax for markdown
    pub fn markdown_syntax(&self) -> &SyntaxReference {
        SYNTAX_SET.find_syntax_by_extension("md").unwrap()
    }

    /// Highlight a line of code
    pub fn highlight_line<'a>(
        &self,
        line: &'a str,
        syntax: &SyntaxReference,
    ) -> Vec<(Style, &'a str)> {
        let mut highlighter = HighlightLines::new(syntax, self.theme());
        highlighter
            .highlight_line(line, &SYNTAX_SET)
            .unwrap_or_else(|_| vec![(Style::default(), line)])
    }

    /// Highlight multiple lines
    pub fn highlight_lines<'a>(
        &self,
        lines: &'a [&'a str],
        syntax: &SyntaxReference,
    ) -> Vec<Vec<(Style, &'a str)>> {
        let mut highlighter = HighlightLines::new(syntax, self.theme());
        lines
            .iter()
            .map(|line| {
                highlighter
                    .highlight_line(line, &SYNTAX_SET)
                    .unwrap_or_else(|_| vec![(Style::default(), *line)])
            })
            .collect()
    }

    /// List available themes
    pub fn available_themes() -> Vec<&'static str> {
        THEME_SET
            .themes
            .keys()
            .map(|s: &String| s.as_str())
            .collect()
    }

    /// List available syntaxes
    pub fn available_syntaxes() -> Vec<&'static str> {
        SYNTAX_SET
            .syntaxes()
            .iter()
            .map(|s| s.name.as_str())
            .collect()
    }

    /// Set theme
    pub fn set_theme(&mut self, theme_name: &str) {
        if THEME_SET.themes.contains_key(theme_name) {
            self.theme_name = theme_name.to_string();
        }
    }
}

impl Default for Highlighter {
    fn default() -> Self {
        Self::new("base16-ocean.dark")
    }
}

/// Convert syntect Style to RGB tuple
pub fn style_to_rgb(style: &Style) -> (u8, u8, u8) {
    (style.foreground.r, style.foreground.g, style.foreground.b)
}

/// Convert syntect Style to RGBA tuple
pub fn style_to_rgba(style: &Style) -> (u8, u8, u8, u8) {
    (
        style.foreground.r,
        style.foreground.g,
        style.foreground.b,
        style.foreground.a,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_themes_available() {
        let themes = Highlighter::available_themes();
        assert!(!themes.is_empty());
    }

    #[test]
    fn test_markdown_highlighting() {
        let highlighter = Highlighter::default();
        let syntax = highlighter.markdown_syntax();
        let result = highlighter.highlight_line("# Hello **world**", syntax);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_rust_highlighting() {
        let highlighter = Highlighter::default();
        if let Some(syntax) = highlighter.syntax_for_language("rust") {
            let result = highlighter.highlight_line("fn main() {}", syntax);
            assert!(!result.is_empty());
        }
    }
}
