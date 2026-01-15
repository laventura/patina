//! # Patina i18n
//!
//! Internationalization support using Fluent.

use fluent::{FluentBundle, FluentResource, FluentArgs, FluentValue};
use unic_langid::LanguageIdentifier;
use std::collections::HashMap;
use once_cell::sync::Lazy;

/// Embedded English translations
const EN_FTL: &str = r#"
# Patina - English translations

app-title = Patina
app-description = A fast, lightweight Markdown editor

# File menu
file-new = New
file-open = Open
file-save = Save
file-save-as = Save As
file-close = Close
file-quit = Quit

# Edit menu
edit-undo = Undo
edit-redo = Redo
edit-cut = Cut
edit-copy = Copy
edit-paste = Paste
edit-find = Find
edit-replace = Replace
edit-goto-line = Go to Line

# View menu
view-raw = Raw
view-rendered = Rendered
view-split = Split
view-zen = Zen Mode
view-minimap = Minimap
view-outline = Outline

# Status messages
status-saved = File saved: { $filename }
status-modified = Modified
status-line-col = Ln { $line }, Col { $col }
status-unsaved-changes = You have { $count ->
    [one] one unsaved change
   *[other] { $count } unsaved changes
}

# Dialogs
dialog-save-changes = Save changes to { $filename }?
dialog-yes = Yes
dialog-no = No
dialog-cancel = Cancel

# Settings
settings-title = Settings
settings-theme = Theme
settings-font-size = Font Size
settings-line-numbers = Line Numbers
settings-word-wrap = Word Wrap
"#;

/// Translator instance
pub struct Translator {
    bundles: HashMap<String, FluentBundle<FluentResource>>,
    current_locale: String,
}

impl Translator {
    /// Create a new translator with default locale
    pub fn new() -> Self {
        let mut translator = Self {
            bundles: HashMap::new(),
            current_locale: "en".to_string(),
        };
        
        // Load embedded English
        translator.load_ftl("en", EN_FTL);
        
        translator
    }

    /// Load a Fluent translation file
    pub fn load_ftl(&mut self, locale: &str, ftl_content: &str) {
        let lang_id: LanguageIdentifier = locale.parse()
            .unwrap_or_else(|_| "en".parse().unwrap());
        
        let resource = FluentResource::try_new(ftl_content.to_string())
            .expect("Failed to parse FTL");
        
        let mut bundle = FluentBundle::new(vec![lang_id]);
        bundle.add_resource(resource)
            .expect("Failed to add resource");
        
        self.bundles.insert(locale.to_string(), bundle);
    }

    /// Set the current locale
    pub fn set_locale(&mut self, locale: &str) {
        if self.bundles.contains_key(locale) {
            self.current_locale = locale.to_string();
        } else {
            log::warn!("Locale {} not found, using English", locale);
            self.current_locale = "en".to_string();
        }
    }

    /// Get a translated message
    pub fn get(&self, id: &str) -> String {
        self.get_with_args(id, None)
    }

    /// Get a translated message with arguments
    pub fn get_with_args(&self, id: &str, args: Option<&FluentArgs>) -> String {
        let bundle = self.bundles.get(&self.current_locale)
            .or_else(|| self.bundles.get("en"))
            .expect("No bundles loaded");
        
        let msg = bundle.get_message(id);
        
        if let Some(msg) = msg {
            if let Some(pattern) = msg.value() {
                let mut errors = vec![];
                let result = bundle.format_pattern(pattern, args, &mut errors);
                
                if !errors.is_empty() {
                    log::warn!("Translation errors for {}: {:?}", id, errors);
                }
                
                return result.to_string();
            }
        }
        
        // Fallback to message ID
        log::warn!("Missing translation: {}", id);
        id.to_string()
    }

    /// Get current locale
    pub fn current_locale(&self) -> &str {
        &self.current_locale
    }

    /// List available locales
    pub fn available_locales(&self) -> Vec<&str> {
        self.bundles.keys().map(|s| s.as_str()).collect()
    }
}

impl Default for Translator {
    fn default() -> Self {
        Self::new()
    }
}

/// Global translator instance
static TRANSLATOR: Lazy<std::sync::RwLock<Translator>> = Lazy::new(|| {
    std::sync::RwLock::new(Translator::new())
});

/// Get a translation using the global translator
pub fn t(id: &str) -> String {
    TRANSLATOR.read().unwrap().get(id)
}

/// Get a translation with arguments
pub fn t_args(id: &str, args: &[(&str, &str)]) -> String {
    let mut fluent_args = FluentArgs::new();
    for (key, value) in args {
        fluent_args.set(*key, FluentValue::from(*value));
    }
    TRANSLATOR.read().unwrap().get_with_args(id, Some(&fluent_args))
}

/// Set the global locale
pub fn set_locale(locale: &str) {
    TRANSLATOR.write().unwrap().set_locale(locale);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_translation() {
        let translator = Translator::new();
        assert_eq!(translator.get("app-title"), "Patina");
    }

    #[test]
    fn test_missing_translation() {
        let translator = Translator::new();
        let result = translator.get("nonexistent-key");
        assert_eq!(result, "nonexistent-key");
    }

    #[test]
    fn test_global_translator() {
        assert_eq!(t("app-title"), "Patina");
    }
}
