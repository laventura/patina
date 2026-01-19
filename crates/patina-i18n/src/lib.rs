//! # Patina i18n
//!
//! Internationalization support for Patina.
//! For v0.6.0, this is a simplified implementation using embedded English translations.

use once_cell::sync::Lazy;
use std::collections::HashMap;

/// English translations loaded at compile time
static TRANSLATIONS: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut map = HashMap::new();

    // Application
    map.insert("app-title", "Patina");
    map.insert("app-description", "A fast, lightweight Markdown editor");

    // File Operations
    map.insert("file-new", "New File");
    map.insert("file-open", "Open...");
    map.insert("file-save", "Save");
    map.insert("file-save-as", "Save As...");
    map.insert("file-close", "Close");
    map.insert("file-quit", "Quit");

    // Edit Operations
    map.insert("edit-undo", "Undo");
    map.insert("edit-redo", "Redo");
    map.insert("edit-cut", "Cut");
    map.insert("edit-copy", "Copy");
    map.insert("edit-paste", "Paste");
    map.insert("edit-select-all", "Select All");
    map.insert("edit-find", "Find...");
    map.insert("edit-replace", "Replace...");

    // View Operations
    map.insert("view-toggle-split", "Toggle Split View");
    map.insert("view-toggle-zen", "Toggle Zen Mode");
    map.insert("view-next-tab", "Next Tab");
    map.insert("view-prev-tab", "Previous Tab");

    // Status Messages
    map.insert("status-saved", "Saved");
    map.insert("status-saved-as", "Saved as {path}");
    map.insert("status-file-opened", "Opened {path}");
    map.insert("status-file-created", "Created new file");
    map.insert("status-unsaved-changes", "Unsaved changes! Press {key} again to quit without saving.");
    map.insert("status-no-file-path", "No file path specified");
    map.insert("status-error-saving", "Error saving file: {error}");
    map.insert("status-error-opening", "Error opening file: {error}");

    // Prompts
    map.insert("prompt-open-file", "Open file:");
    map.insert("prompt-save-as", "Save as:");
    map.insert("prompt-search", "Find:");

    // View Modes
    map.insert("view-mode-raw", "Raw");
    map.insert("view-mode-preview", "Preview");
    map.insert("view-mode-split", "Split");

    // Themes
    map.insert("theme-dracula", "Dracula");
    map.insert("theme-one-dark", "One Dark");
    map.insert("theme-solarized-light", "Solarized Light");
    map.insert("theme-solarized-dark", "Solarized Dark");
    map.insert("theme-gruvbox", "Gruvbox");
    map.insert("theme-nord", "Nord");

    // Errors
    map.insert("error-file-not-found", "File not found: {path}");
    map.insert("error-permission-denied", "Permission denied: {path}");
    map.insert("error-invalid-utf8", "Invalid UTF-8 in file");
    map.insert("error-io", "I/O error: {error}");

    // Help
    map.insert("help-keyboard-shortcuts", "Keyboard Shortcuts");
    map.insert("help-about", "About Patina");
    map.insert("help-version", "Version {version}");

    map
});

/// Get a translation by ID
///
/// # Example
/// ```
/// use patina_i18n::t;
/// let title = t("app-title");
/// assert_eq!(title, "Patina");
/// ```
pub fn t(id: &str) -> String {
    TRANSLATIONS
        .get(id)
        .map(|s| s.to_string())
        .unwrap_or_else(|| id.to_string())
}

/// Get a translation with arguments
///
/// Simple string replacement for v0.6.0.
///
/// # Example
/// ```
/// use patina_i18n::t_args;
/// let msg = t_args("status-saved-as", &[("path", "/tmp/test.md")]);
/// assert!(msg.contains("/tmp/test.md"));
/// ```
pub fn t_args(id: &str, args: &[(&str, &str)]) -> String {
    let template = t(id);

    // Simple string replacement
    let mut result = template;
    for (key, value) in args {
        let placeholder = format!("{{{}}}", key);
        result = result.replace(&placeholder, value);
    }

    result
}

/// Set the global locale (for future multi-language support)
///
/// Currently only English is supported. This is a stub for v0.6.0.
pub fn set_locale(_locale: &str) {
    // No-op for v0.6.0
}

/// Get available locales
pub fn available_locales() -> Vec<&'static str> {
    vec!["en"]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_translation() {
        assert_eq!(t("app-title"), "Patina");
        assert_eq!(t("file-save"), "Save");
        assert_eq!(t("edit-undo"), "Undo");
    }

    #[test]
    fn test_missing_translation() {
        assert_eq!(t("nonexistent-key"), "nonexistent-key");
    }

    #[test]
    fn test_translation_with_args() {
        let msg = t_args("status-saved-as", &[("path", "/tmp/test.md")]);
        assert!(msg.contains("/tmp/test.md"));
    }

    #[test]
    fn test_available_locales() {
        let locales = available_locales();
        assert!(locales.contains(&"en"));
    }
}
