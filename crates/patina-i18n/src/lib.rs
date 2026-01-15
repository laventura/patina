//! # Patina i18n
//!
//! Internationalization support using Fluent.
//! This is a minimal stub implementation for v0.1.0.
//! Full i18n support will be implemented in v0.6.0.

/// Get a translation (stub - returns key as-is for v0.1.0)
pub fn t(id: &str) -> String {
    // For v0.1.0, just return the key
    // Full Fluent integration in v0.6.0
    id.to_string()
}

/// Get a translation with arguments (stub for v0.1.0)
pub fn t_args(id: &str, _args: &[(&str, &str)]) -> String {
    // For v0.1.0, just return the key
    id.to_string()
}

/// Set the global locale (stub for v0.1.0)
pub fn set_locale(_locale: &str) {
    // No-op for v0.1.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stub_translation() {
        assert_eq!(t("app-title"), "app-title");
    }
}
