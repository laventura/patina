//! Cursor and selection handling.

/// A position in the document
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Position {
    /// Line number (0-indexed)
    pub line: usize,
    /// Column (0-indexed, in characters)
    pub col: usize,
}

impl Position {
    pub fn new(line: usize, col: usize) -> Self {
        Self { line, col }
    }
}

/// A selection or cursor position
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Selection {
    /// Anchor point (where selection started)
    pub anchor: Position,
    /// Head point (where cursor is)
    pub head: Position,
}

impl Selection {
    /// Create a cursor (zero-width selection)
    pub fn cursor(pos: Position) -> Self {
        Self {
            anchor: pos,
            head: pos,
        }
    }

    /// Create a selection from anchor to head
    pub fn new(anchor: Position, head: Position) -> Self {
        Self { anchor, head }
    }

    /// Check if this is a cursor (no selection)
    pub fn is_cursor(&self) -> bool {
        self.anchor == self.head
    }

    /// Get the start of the selection (min position)
    pub fn start(&self) -> Position {
        if self.anchor.line < self.head.line
            || (self.anchor.line == self.head.line && self.anchor.col <= self.head.col)
        {
            self.anchor
        } else {
            self.head
        }
    }

    /// Get the end of the selection (max position)
    pub fn end(&self) -> Position {
        if self.anchor.line > self.head.line
            || (self.anchor.line == self.head.line && self.anchor.col >= self.head.col)
        {
            self.anchor
        } else {
            self.head
        }
    }

    /// Move cursor/selection by lines
    pub fn move_lines(&mut self, delta: isize, extend: bool) {
        let new_line = if delta < 0 {
            self.head.line.saturating_sub(delta.unsigned_abs())
        } else {
            self.head.line.saturating_add(delta as usize)
        };

        self.head.line = new_line;

        if !extend {
            self.anchor = self.head;
        }
    }

    /// Move cursor/selection by columns
    pub fn move_cols(&mut self, delta: isize, extend: bool) {
        let new_col = if delta < 0 {
            self.head.col.saturating_sub(delta.unsigned_abs())
        } else {
            self.head.col.saturating_add(delta as usize)
        };

        self.head.col = new_col;

        if !extend {
            self.anchor = self.head;
        }
    }
}

impl Default for Selection {
    fn default() -> Self {
        Self::cursor(Position::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cursor() {
        let sel = Selection::cursor(Position::new(5, 10));
        assert!(sel.is_cursor());
        assert_eq!(sel.start(), sel.end());
    }

    #[test]
    fn test_selection_order() {
        // Selection going forward
        let sel = Selection::new(Position::new(1, 5), Position::new(3, 10));
        assert_eq!(sel.start().line, 1);
        assert_eq!(sel.end().line, 3);

        // Selection going backward
        let sel = Selection::new(Position::new(3, 10), Position::new(1, 5));
        assert_eq!(sel.start().line, 1);
        assert_eq!(sel.end().line, 3);
    }
}
