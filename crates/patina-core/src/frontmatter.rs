//! Frontmatter parsing for YAML and TOML.

use serde_json::Value;
use std::collections::HashMap;
use std::fmt;

/// Frontmatter format
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FrontmatterFormat {
    Yaml,
    Toml,
}

/// Parsed frontmatter data
#[derive(Debug, Clone)]
pub struct Frontmatter {
    /// The format used
    pub format: FrontmatterFormat,
    /// Raw frontmatter string (for preservation)
    pub raw: String,
    /// Parsed key-value data
    pub data: HashMap<String, Value>,
}

impl Frontmatter {
    /// Extract frontmatter from document content.
    /// Returns (Option<Frontmatter>, body_content)
    pub fn extract(content: &str) -> (Option<Self>, &str) {
        let trimmed = content.trim_start();

        // Check for YAML frontmatter (---)
        if let Some(rest) = trimmed.strip_prefix("---") {
            if let Some(end) = rest.find("\n---") {
                let raw = rest[..end].trim();
                let body = rest[end + 4..].trim_start_matches('\n');

                if let Ok(data) = Self::parse_yaml(raw) {
                    return (
                        Some(Self {
                            format: FrontmatterFormat::Yaml,
                            raw: raw.to_string(),
                            data,
                        }),
                        body,
                    );
                }
            }
        }

        // Check for TOML frontmatter (+++)
        if let Some(rest) = trimmed.strip_prefix("+++") {
            if let Some(end) = rest.find("\n+++") {
                let raw = rest[..end].trim();
                let body = rest[end + 4..].trim_start_matches('\n');

                if let Ok(data) = Self::parse_toml(raw) {
                    return (
                        Some(Self {
                            format: FrontmatterFormat::Toml,
                            raw: raw.to_string(),
                            data,
                        }),
                        body,
                    );
                }
            }
        }

        (None, content)
    }

    /// Parse YAML frontmatter
    fn parse_yaml(raw: &str) -> Result<HashMap<String, Value>, String> {
        let yaml_value: serde_yaml::Value = serde_yaml::from_str(raw).map_err(|e| e.to_string())?;

        let json_value: Value = serde_json::to_value(yaml_value).map_err(|e| e.to_string())?;

        Self::value_to_hashmap(json_value)
    }

    /// Parse TOML frontmatter
    fn parse_toml(raw: &str) -> Result<HashMap<String, Value>, String> {
        let toml_value: toml::Value = toml::from_str(raw).map_err(|e| e.to_string())?;

        let json_value: Value = serde_json::to_value(toml_value).map_err(|e| e.to_string())?;

        Self::value_to_hashmap(json_value)
    }

    /// Convert a JSON value to a HashMap (if it's an object)
    fn value_to_hashmap(value: Value) -> Result<HashMap<String, Value>, String> {
        match value {
            Value::Object(map) => Ok(map.into_iter().collect()),
            _ => Err("Frontmatter must be an object".to_string()),
        }
    }

    /// Get a value from frontmatter
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.data.get(key)
    }

    /// Get a string value from frontmatter
    pub fn get_str(&self, key: &str) -> Option<&str> {
        self.data.get(key).and_then(|v| v.as_str())
    }
}

impl fmt::Display for Frontmatter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.format {
            FrontmatterFormat::Yaml => write!(f, "---\n{}\n---", self.raw),
            FrontmatterFormat::Toml => write!(f, "+++\n{}\n+++", self.raw),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_yaml_frontmatter() {
        let content = "---\ntitle: Test\nauthor: Me\n---\n\n# Hello";
        let (fm, body) = Frontmatter::extract(content);

        assert!(fm.is_some());
        let fm = fm.unwrap();
        assert_eq!(fm.format, FrontmatterFormat::Yaml);
        assert_eq!(fm.get_str("title"), Some("Test"));
        assert!(body.starts_with("# Hello"));
    }

    #[test]
    fn test_toml_frontmatter() {
        let content = "+++\ntitle = \"Test\"\nauthor = \"Me\"\n+++\n\n# Hello";
        let (fm, body) = Frontmatter::extract(content);

        assert!(fm.is_some());
        let fm = fm.unwrap();
        assert_eq!(fm.format, FrontmatterFormat::Toml);
        assert_eq!(fm.get_str("title"), Some("Test"));
        assert!(body.starts_with("# Hello"));
    }

    #[test]
    fn test_no_frontmatter() {
        let content = "# Just a heading\n\nSome content.";
        let (fm, body) = Frontmatter::extract(content);

        assert!(fm.is_none());
        assert_eq!(body, content);
    }
}
