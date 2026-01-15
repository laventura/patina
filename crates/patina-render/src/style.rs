//! Editor styling configuration.

/// Editor style configuration
#[derive(Debug, Clone)]
pub struct EditorStyle {
    /// Show line numbers
    pub line_numbers: bool,
    /// Show minimap
    pub minimap: bool,
    /// Highlight current line
    pub highlight_line: bool,
    /// Show indent guides
    pub indent_guides: bool,
    /// Tab size in spaces
    pub tab_size: usize,
    /// Soft wrap long lines
    pub soft_wrap: bool,
    /// Maximum line width for Zen mode
    pub zen_width: usize,
    /// Font size (for GUI)
    pub font_size: f32,
}

impl Default for EditorStyle {
    fn default() -> Self {
        Self {
            line_numbers: true,
            minimap: false,
            highlight_line: true,
            indent_guides: true,
            tab_size: 4,
            soft_wrap: true,
            zen_width: 80,
            font_size: 14.0,
        }
    }
}
