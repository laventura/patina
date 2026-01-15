//! Configuration management.

use anyhow::Result;
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

/// Application configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    /// Color theme name
    pub theme: String,

    /// Editor settings
    pub editor: EditorConfig,

    /// UI settings
    pub ui: UiConfig,

    /// Keybinding mode
    pub keybindings: KeybindingMode,

    /// Markdown settings
    pub markdown: MarkdownConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct EditorConfig {
    /// Tab size in spaces
    pub tab_size: usize,
    /// Use spaces instead of tabs
    pub use_spaces: bool,
    /// Soft wrap long lines
    pub soft_wrap: bool,
    /// Auto-save interval in seconds (0 to disable)
    pub auto_save: u64,
    /// Auto-close brackets
    pub auto_close_brackets: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct UiConfig {
    /// Show line numbers
    pub line_numbers: bool,
    /// Show minimap
    pub minimap: bool,
    /// Highlight current line
    pub highlight_line: bool,
    /// Show indent guides
    pub indent_guides: bool,
    /// Default view mode
    pub default_view: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct MarkdownConfig {
    /// Parser flavor (commonmark, gfm)
    pub flavor: String,
    /// Render emoji shortcodes
    pub render_emoji: bool,
    /// Enable LaTeX math
    pub enable_math: bool,
    /// Enable Mermaid diagrams
    pub enable_mermaid: bool,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum KeybindingMode {
    #[default]
    Vim,
    Emacs,
    Standard,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            theme: "Dracula".to_string(),
            editor: EditorConfig::default(),
            ui: UiConfig::default(),
            keybindings: KeybindingMode::default(),
            markdown: MarkdownConfig::default(),
        }
    }
}

impl Default for EditorConfig {
    fn default() -> Self {
        Self {
            tab_size: 4,
            use_spaces: true,
            soft_wrap: true,
            auto_save: 0,
            auto_close_brackets: true,
        }
    }
}

impl Default for UiConfig {
    fn default() -> Self {
        Self {
            line_numbers: true,
            minimap: false,
            highlight_line: true,
            indent_guides: true,
            default_view: "split".to_string(),
        }
    }
}

impl Default for MarkdownConfig {
    fn default() -> Self {
        Self {
            flavor: "gfm".to_string(),
            render_emoji: true,
            enable_math: true,
            enable_mermaid: true,
        }
    }
}

impl Config {
    /// Load configuration from file
    pub fn load(path: Option<&Path>) -> Result<Self> {
        let config_path = path.map(PathBuf::from).or_else(Self::default_path);

        if let Some(path) = config_path {
            if path.exists() {
                let content = std::fs::read_to_string(&path)?;
                let config: Config = toml::from_str(&content)?;
                return Ok(config);
            }
        }

        Ok(Self::default())
    }

    /// Save configuration to file (used in v0.2+ for settings UI)
    #[allow(dead_code)]
    pub fn save(&self, path: Option<&Path>) -> Result<()> {
        let config_path = path.map(PathBuf::from).or_else(Self::default_path);

        if let Some(path) = config_path {
            if let Some(parent) = path.parent() {
                std::fs::create_dir_all(parent)?;
            }
            let content = toml::to_string_pretty(self)?;
            std::fs::write(path, content)?;
        }

        Ok(())
    }

    /// Get the default config path
    fn default_path() -> Option<PathBuf> {
        ProjectDirs::from("com", "patina", "patina")
            .map(|dirs| dirs.config_dir().join("config.toml"))
    }
}
