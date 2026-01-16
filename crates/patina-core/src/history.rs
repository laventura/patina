//! Undo/redo history.

use crate::Selection;

/// An edit operation that can be undone/redone
#[derive(Debug, Clone)]
pub struct Edit {
    /// Character position where edit started
    pub position: usize,
    /// Text that was deleted (empty for pure insertions)
    pub deleted: String,
    /// Text that was inserted (empty for pure deletions)
    pub inserted: String,
    /// Cursor state before the edit
    pub cursor_before: Selection,
    /// Cursor state after the edit
    pub cursor_after: Selection,
}

impl Edit {
    /// Create an insertion edit
    pub fn insert(
        position: usize,
        text: String,
        cursor_before: Selection,
        cursor_after: Selection,
    ) -> Self {
        Self {
            position,
            deleted: String::new(),
            inserted: text,
            cursor_before,
            cursor_after,
        }
    }

    /// Create a deletion edit
    pub fn delete(
        position: usize,
        text: String,
        cursor_before: Selection,
        cursor_after: Selection,
    ) -> Self {
        Self {
            position,
            deleted: text,
            inserted: String::new(),
            cursor_before,
            cursor_after,
        }
    }

    /// Create a replacement edit
    pub fn replace(
        position: usize,
        deleted: String,
        inserted: String,
        cursor_before: Selection,
        cursor_after: Selection,
    ) -> Self {
        Self {
            position,
            deleted,
            inserted,
            cursor_before,
            cursor_after,
        }
    }
}

/// Undo/redo history manager
#[derive(Debug, Default)]
pub struct History {
    /// Stack of undoable edits
    undo_stack: Vec<Edit>,
    /// Stack of redoable edits
    redo_stack: Vec<Edit>,
    /// Maximum history size
    max_size: usize,
}

impl History {
    /// Create a new history with default capacity
    pub fn new() -> Self {
        Self {
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            max_size: 1000,
        }
    }

    /// Create a history with custom max size
    pub fn with_max_size(max_size: usize) -> Self {
        Self {
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            max_size,
        }
    }

    /// Record an edit
    pub fn record(&mut self, edit: Edit) {
        // Clear redo stack on new edit
        self.redo_stack.clear();

        // Add to undo stack
        self.undo_stack.push(edit);

        // Trim if too large
        if self.undo_stack.len() > self.max_size {
            self.undo_stack.remove(0);
        }
    }

    /// Undo the last edit, returning it if available
    pub fn undo(&mut self) -> Option<Edit> {
        if let Some(edit) = self.undo_stack.pop() {
            self.redo_stack.push(edit.clone());
            Some(edit)
        } else {
            None
        }
    }

    /// Redo the last undone edit, returning it if available
    pub fn redo(&mut self) -> Option<Edit> {
        if let Some(edit) = self.redo_stack.pop() {
            self.undo_stack.push(edit.clone());
            Some(edit)
        } else {
            None
        }
    }

    /// Check if undo is available
    pub fn can_undo(&self) -> bool {
        !self.undo_stack.is_empty()
    }

    /// Check if redo is available
    pub fn can_redo(&self) -> bool {
        !self.redo_stack.is_empty()
    }

    /// Clear all history
    pub fn clear(&mut self) {
        self.undo_stack.clear();
        self.redo_stack.clear();
    }

    /// Get the number of undo steps available
    pub fn undo_count(&self) -> usize {
        self.undo_stack.len()
    }

    /// Get the number of redo steps available
    pub fn redo_count(&self) -> usize {
        self.redo_stack.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::selection::Position;

    fn dummy_cursor() -> Selection {
        Selection::cursor(Position::new(0, 0))
    }

    #[test]
    fn test_undo_redo() {
        let mut history = History::new();

        let edit = Edit::insert(0, "hello".to_string(), dummy_cursor(), dummy_cursor());
        history.record(edit);

        assert!(history.can_undo());
        assert!(!history.can_redo());

        let undone = history.undo();
        assert!(undone.is_some());
        assert!(!history.can_undo());
        assert!(history.can_redo());

        let redone = history.redo();
        assert!(redone.is_some());
    }

    #[test]
    fn test_multiple_edits() {
        let mut history = History::new();

        // Record multiple edits
        history.record(Edit::insert(
            0,
            "a".to_string(),
            dummy_cursor(),
            dummy_cursor(),
        ));
        history.record(Edit::insert(
            1,
            "b".to_string(),
            dummy_cursor(),
            dummy_cursor(),
        ));
        history.record(Edit::insert(
            2,
            "c".to_string(),
            dummy_cursor(),
            dummy_cursor(),
        ));

        assert_eq!(history.undo_count(), 3);

        // Undo all
        assert!(history.undo().is_some());
        assert!(history.undo().is_some());
        assert!(history.undo().is_some());
        assert!(history.undo().is_none());

        assert_eq!(history.redo_count(), 3);
    }

    #[test]
    fn test_new_edit_clears_redo() {
        let mut history = History::new();

        history.record(Edit::insert(
            0,
            "a".to_string(),
            dummy_cursor(),
            dummy_cursor(),
        ));
        history.record(Edit::insert(
            1,
            "b".to_string(),
            dummy_cursor(),
            dummy_cursor(),
        ));

        // Undo one edit
        history.undo();
        assert!(history.can_redo());

        // New edit should clear redo stack
        history.record(Edit::insert(
            1,
            "x".to_string(),
            dummy_cursor(),
            dummy_cursor(),
        ));
        assert!(!history.can_redo());
    }

    #[test]
    fn test_edit_with_cursor_positions() {
        let cursor_before = Selection::cursor(Position::new(0, 0));
        let cursor_after = Selection::cursor(Position::new(0, 5));

        let edit = Edit::insert(0, "hello".to_string(), cursor_before, cursor_after);

        assert_eq!(edit.cursor_before.head.col, 0);
        assert_eq!(edit.cursor_after.head.col, 5);
    }

    #[test]
    fn test_delete_edit() {
        let cursor_before = Selection::cursor(Position::new(0, 5));
        let cursor_after = Selection::cursor(Position::new(0, 0));

        let edit = Edit::delete(0, "hello".to_string(), cursor_before, cursor_after);

        assert_eq!(edit.deleted, "hello");
        assert!(edit.inserted.is_empty());
    }
}
