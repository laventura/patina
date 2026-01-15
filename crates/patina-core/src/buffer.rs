//! Text buffer implementation using a rope data structure.
//!
//! The buffer provides O(log n) insertions and deletions, making it
//! suitable for large documents.

use ropey::Rope;

/// A text buffer backed by a rope data structure.
#[derive(Debug, Clone)]
pub struct Buffer {
    /// The underlying rope
    rope: Rope,
    /// Whether the buffer has been modified since last save
    modified: bool,
}

impl Buffer {
    /// Create a new empty buffer
    pub fn new() -> Self {
        Self {
            rope: Rope::new(),
            modified: false,
        }
    }

    /// Create a buffer from a string
    pub fn from_str(text: &str) -> Self {
        Self {
            rope: Rope::from_str(text),
            modified: false,
        }
    }

    /// Get the entire text as a String
    pub fn text(&self) -> String {
        self.rope.to_string()
    }

    /// Get the number of lines
    pub fn len_lines(&self) -> usize {
        self.rope.len_lines()
    }

    /// Get the total number of characters
    pub fn len_chars(&self) -> usize {
        self.rope.len_chars()
    }

    /// Check if the buffer is empty
    pub fn is_empty(&self) -> bool {
        self.rope.len_chars() == 0
    }

    /// Get a specific line (0-indexed)
    pub fn line(&self, line_idx: usize) -> Option<String> {
        if line_idx < self.rope.len_lines() {
            Some(self.rope.line(line_idx).to_string())
        } else {
            None
        }
    }

    /// Insert text at a character position
    pub fn insert(&mut self, char_idx: usize, text: &str) {
        self.rope.insert(char_idx, text);
        self.modified = true;
    }

    /// Delete a range of characters
    pub fn delete(&mut self, start: usize, end: usize) {
        self.rope.remove(start..end);
        self.modified = true;
    }

    /// Replace a range with new text
    pub fn replace(&mut self, start: usize, end: usize, text: &str) {
        self.rope.remove(start..end);
        self.rope.insert(start, text);
        self.modified = true;
    }

    /// Convert a line and column to a character index
    pub fn line_col_to_char(&self, line: usize, col: usize) -> usize {
        let line_start = self.rope.line_to_char(line);
        let line_len = self.rope.line(line).len_chars();
        line_start + col.min(line_len.saturating_sub(1))
    }

    /// Convert a character index to line and column
    pub fn char_to_line_col(&self, char_idx: usize) -> (usize, usize) {
        let line = self.rope.char_to_line(char_idx);
        let line_start = self.rope.line_to_char(line);
        let col = char_idx - line_start;
        (line, col)
    }

    /// Check if buffer has been modified
    pub fn is_modified(&self) -> bool {
        self.modified
    }

    /// Mark buffer as saved (not modified)
    pub fn mark_saved(&mut self) {
        self.modified = false;
    }

    /// Get a slice of text
    pub fn slice(&self, start: usize, end: usize) -> String {
        self.rope.slice(start..end).to_string()
    }
}

impl Default for Buffer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_buffer() {
        let buf = Buffer::new();
        assert!(buf.is_empty());
        assert!(!buf.is_modified());
    }

    #[test]
    fn test_insert() {
        let mut buf = Buffer::new();
        buf.insert(0, "Hello");
        assert_eq!(buf.text(), "Hello");
        assert!(buf.is_modified());
    }

    #[test]
    fn test_line_operations() {
        let buf = Buffer::from_str("line 1\nline 2\nline 3");
        assert_eq!(buf.len_lines(), 3);
        assert_eq!(buf.line(1), Some("line 2\n".to_string()));
    }
}
