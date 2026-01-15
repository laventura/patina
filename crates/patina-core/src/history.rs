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
    pub fn insert(position: usize, text: String, cursor_before: Selection, cursor_after: Selection) -> Self {
        Self {
            position,
            deleted: String::new(),
            inserted: text,
            cursor_before,
            cursor_after,
        }
    }

    /// Create a deletion edit
    pub fn delete(position: usize, text: String, cursor_before: Selection, cursor_after: Selection) -> Self {
        Self {
            position,
            deleted: text,
            inserted: String::new(),
            cursor_before,
            cursor_after,
        }
    }

    /// Create a replacement edit
    pub fn replace(position: usize, deleted: String, inserted: String, cursor_before: Selection, cursor_after: Selection) -> Self {
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
}
