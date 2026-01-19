//! Emoji shortcode expansion.

use once_cell::sync::Lazy;
use std::collections::HashMap;

/// Common emoji shortcodes (subset - full list would be ~1800)
static EMOJI: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut m = HashMap::new();

    // Smileys
    m.insert("smile", "😊");
    m.insert("grin", "😁");
    m.insert("joy", "😂");
    m.insert("rofl", "🤣");
    m.insert("wink", "😉");
    m.insert("heart_eyes", "😍");
    m.insert("thinking", "🤔");
    m.insert("sunglasses", "😎");

    // Gestures
    m.insert("+1", "👍");
    m.insert("thumbsup", "👍");
    m.insert("-1", "👎");
    m.insert("thumbsdown", "👎");
    m.insert("wave", "👋");
    m.insert("clap", "👏");
    m.insert("pray", "🙏");
    m.insert("muscle", "💪");

    // Hearts
    m.insert("heart", "❤️");
    m.insert("sparkling_heart", "💖");
    m.insert("broken_heart", "💔");

    // Objects
    m.insert("rocket", "🚀");
    m.insert("star", "⭐");
    m.insert("fire", "🔥");
    m.insert("100", "💯");
    m.insert("bulb", "💡");
    m.insert("books", "📚");
    m.insert("memo", "📝");
    m.insert("computer", "💻");
    m.insert("phone", "📱");

    // Nature
    m.insert("sun", "☀️");
    m.insert("moon", "🌙");
    m.insert("cloud", "☁️");
    m.insert("rainbow", "🌈");
    m.insert("tree", "🌳");
    m.insert("flower", "🌸");

    // Symbols
    m.insert("check", "✅");
    m.insert("x", "❌");
    m.insert("warning", "⚠️");
    m.insert("question", "❓");
    m.insert("exclamation", "❗");
    m.insert("heavy_check_mark", "✔️");
    m.insert("heavy_multiplication_x", "✖️");
    m.insert("arrow_right", "➡️");
    m.insert("arrow_left", "⬅️");
    m.insert("arrow_up", "⬆️");
    m.insert("arrow_down", "⬇️");

    // Programming related
    m.insert("bug", "🐛");
    m.insert("gear", "⚙️");
    m.insert("wrench", "🔧");
    m.insert("hammer", "🔨");
    m.insert("package", "📦");
    m.insert("link", "🔗");
    m.insert("lock", "🔒");
    m.insert("key", "🔑");
    m.insert("sparkles", "✨");
    m.insert("zap", "⚡");

    // Additional common emojis
    m.insert("tada", "🎉");
    m.insert("construction", "🚧");
    m.insert("white_check_mark", "✅");
    m.insert("round_pushpin", "📍");
    m.insert("pushpin", "📌");

    m
});

/// Emoji shortcode expander
pub struct EmojiExpander;

impl EmojiExpander {
    /// Create a new expander
    pub fn new() -> Self {
        Self
    }

    /// Expand a single shortcode (without colons)
    pub fn expand(&self, shortcode: &str) -> Option<&'static str> {
        EMOJI.get(shortcode).copied()
    }

    /// Expand all shortcodes in text (:shortcode: -> emoji)
    pub fn expand_all(&self, text: &str) -> String {
        let mut result = String::with_capacity(text.len());
        let mut chars = text.char_indices();

        while let Some((i, c)) = chars.next() {
            if c == ':' {
                // Collect characters until we hit ':', ' ', '\n', or end of string
                let mut shortcode = String::new();
                let mut found_closing = false;

                // Peek ahead and collect shortcode characters
                let remaining = &text[i + 1..];
                for ch in remaining.chars() {
                    if ch == ':' {
                        found_closing = true;
                        break;
                    } else if ch == ' ' || ch == '\n' {
                        break;
                    } else {
                        shortcode.push(ch);
                    }
                }

                // Try to expand if we found a valid shortcode
                if found_closing && !shortcode.is_empty() {
                    if let Some(emoji) = EMOJI.get(shortcode.as_str()) {
                        result.push_str(emoji);
                        // Skip the shortcode and closing colon
                        for _ in 0..shortcode.len() {
                            chars.next();
                        }
                        chars.next(); // Skip closing colon
                        continue;
                    }
                }

                // Not a valid shortcode, output the opening colon
                result.push(':');
            } else {
                result.push(c);
            }
        }

        result
    }

    /// Get all available shortcodes
    pub fn available_shortcodes() -> Vec<&'static str> {
        let mut codes: Vec<_> = EMOJI.keys().copied().collect();
        codes.sort();
        codes
    }

    /// Search shortcodes by prefix
    pub fn search(&self, prefix: &str) -> Vec<(&'static str, &'static str)> {
        let prefix_lower = prefix.to_lowercase();
        EMOJI
            .iter()
            .filter(|(k, _)| k.starts_with(&prefix_lower))
            .map(|(&k, &v)| (k, v))
            .collect()
    }
}

impl Default for EmojiExpander {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expand_single() {
        let expander = EmojiExpander::new();
        assert_eq!(expander.expand("rocket"), Some("🚀"));
        assert_eq!(expander.expand("unknown"), None);
    }

    #[test]
    fn test_search() {
        let expander = EmojiExpander::new();
        let results = expander.search("arrow");
        assert!(!results.is_empty());
        assert!(results.iter().any(|(k, _)| *k == "arrow_right"));
    }

    #[test]
    fn test_available() {
        let codes = EmojiExpander::available_shortcodes();
        assert!(!codes.is_empty());
        assert!(codes.contains(&"rocket"));
    }
}
