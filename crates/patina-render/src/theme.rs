//! Theme definitions for the editor.

use crate::Color;

/// Editor color theme
#[derive(Debug, Clone)]
pub struct Theme {
    pub name: String,
    pub is_dark: bool,

    // Background colors
    pub bg_primary: Color,
    pub bg_secondary: Color,
    pub bg_selection: Color,
    pub bg_line_highlight: Color,

    // Text colors
    pub fg_primary: Color,
    pub fg_secondary: Color,
    pub fg_muted: Color,

    // Syntax colors
    pub syntax_keyword: Color,
    pub syntax_string: Color,
    pub syntax_number: Color,
    pub syntax_comment: Color,
    pub syntax_function: Color,
    pub syntax_type: Color,
    pub syntax_operator: Color,

    // Markdown specific
    pub md_heading: Color,
    pub md_bold: Color,
    pub md_italic: Color,
    pub md_link: Color,
    pub md_code: Color,
    pub md_blockquote: Color,

    // UI elements
    pub ui_border: Color,
    pub ui_cursor: Color,
    pub ui_line_number: Color,
    pub ui_status_bar: Color,
}

impl Theme {
    /// Dracula theme (dark)
    pub fn dracula() -> Self {
        Self {
            name: "Dracula".to_string(),
            is_dark: true,

            bg_primary: Color::rgb(40, 42, 54),
            bg_secondary: Color::rgb(68, 71, 90),
            bg_selection: Color::rgba(68, 71, 90, 180),
            bg_line_highlight: Color::rgb(49, 51, 65),

            fg_primary: Color::rgb(248, 248, 242),
            fg_secondary: Color::rgb(189, 147, 249),
            fg_muted: Color::rgb(98, 114, 164),

            syntax_keyword: Color::rgb(255, 121, 198),
            syntax_string: Color::rgb(241, 250, 140),
            syntax_number: Color::rgb(189, 147, 249),
            syntax_comment: Color::rgb(98, 114, 164),
            syntax_function: Color::rgb(80, 250, 123),
            syntax_type: Color::rgb(139, 233, 253),
            syntax_operator: Color::rgb(255, 121, 198),

            md_heading: Color::rgb(189, 147, 249),
            md_bold: Color::rgb(255, 184, 108),
            md_italic: Color::rgb(241, 250, 140),
            md_link: Color::rgb(139, 233, 253),
            md_code: Color::rgb(80, 250, 123),
            md_blockquote: Color::rgb(98, 114, 164),

            ui_border: Color::rgb(68, 71, 90),
            ui_cursor: Color::rgb(248, 248, 242),
            ui_line_number: Color::rgb(98, 114, 164),
            ui_status_bar: Color::rgb(68, 71, 90),
        }
    }

    /// One Dark theme
    pub fn one_dark() -> Self {
        Self {
            name: "One Dark".to_string(),
            is_dark: true,

            bg_primary: Color::rgb(40, 44, 52),
            bg_secondary: Color::rgb(33, 37, 43),
            bg_selection: Color::rgba(62, 68, 81, 180),
            bg_line_highlight: Color::rgb(44, 49, 58),

            fg_primary: Color::rgb(171, 178, 191),
            fg_secondary: Color::rgb(97, 175, 239),
            fg_muted: Color::rgb(92, 99, 112),

            syntax_keyword: Color::rgb(198, 120, 221),
            syntax_string: Color::rgb(152, 195, 121),
            syntax_number: Color::rgb(209, 154, 102),
            syntax_comment: Color::rgb(92, 99, 112),
            syntax_function: Color::rgb(97, 175, 239),
            syntax_type: Color::rgb(229, 192, 123),
            syntax_operator: Color::rgb(86, 182, 194),

            md_heading: Color::rgb(224, 108, 117),
            md_bold: Color::rgb(209, 154, 102),
            md_italic: Color::rgb(152, 195, 121),
            md_link: Color::rgb(97, 175, 239),
            md_code: Color::rgb(152, 195, 121),
            md_blockquote: Color::rgb(92, 99, 112),

            ui_border: Color::rgb(62, 68, 81),
            ui_cursor: Color::rgb(171, 178, 191),
            ui_line_number: Color::rgb(76, 82, 99),
            ui_status_bar: Color::rgb(33, 37, 43),
        }
    }

    /// Solarized Light theme
    pub fn solarized_light() -> Self {
        Self {
            name: "Solarized Light".to_string(),
            is_dark: false,

            bg_primary: Color::rgb(253, 246, 227),
            bg_secondary: Color::rgb(238, 232, 213),
            bg_selection: Color::rgba(7, 54, 66, 40),
            bg_line_highlight: Color::rgb(238, 232, 213),

            fg_primary: Color::rgb(101, 123, 131),
            fg_secondary: Color::rgb(38, 139, 210),
            fg_muted: Color::rgb(147, 161, 161),

            syntax_keyword: Color::rgb(133, 153, 0),
            syntax_string: Color::rgb(42, 161, 152),
            syntax_number: Color::rgb(211, 54, 130),
            syntax_comment: Color::rgb(147, 161, 161),
            syntax_function: Color::rgb(38, 139, 210),
            syntax_type: Color::rgb(181, 137, 0),
            syntax_operator: Color::rgb(133, 153, 0),

            md_heading: Color::rgb(203, 75, 22),
            md_bold: Color::rgb(181, 137, 0),
            md_italic: Color::rgb(42, 161, 152),
            md_link: Color::rgb(38, 139, 210),
            md_code: Color::rgb(133, 153, 0),
            md_blockquote: Color::rgb(147, 161, 161),

            ui_border: Color::rgb(147, 161, 161),
            ui_cursor: Color::rgb(101, 123, 131),
            ui_line_number: Color::rgb(147, 161, 161),
            ui_status_bar: Color::rgb(238, 232, 213),
        }
    }

    /// Get theme by name
    pub fn by_name(name: &str) -> Self {
        match name.to_lowercase().as_str() {
            "dracula" => Self::dracula(),
            "one dark" | "one_dark" | "onedark" => Self::one_dark(),
            "solarized light" | "solarized_light" => Self::solarized_light(),
            _ => Self::dracula(), // Default
        }
    }

    /// List available themes
    pub fn available() -> Vec<&'static str> {
        vec!["Dracula", "One Dark", "Solarized Light"]
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self::dracula()
    }
}
