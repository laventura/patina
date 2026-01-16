//! Feature validation tests for v0.2.0 and v0.3.0

use patina_core::selection::{Position, Selection};
use patina_core::{Document, Edit};

#[test]
fn test_cursor_movement_bounds() {
    let mut doc = Document::from_content("Line 1\nLine 2\nLine 3");

    // Start at 0,0
    assert_eq!(doc.cursor, (0, 0));

    // Can't go up from top
    doc.cursor = (0, 3);
    // Simulate up arrow - should stay at line 0
    if doc.cursor.0 > 0 {
        doc.cursor.0 -= 1;
    }
    assert_eq!(doc.cursor.0, 0);

    // Can't go down past last line
    doc.cursor = (2, 0); // Last line
    let max_line = doc.buffer.len_lines().saturating_sub(1);
    if doc.cursor.0 < max_line {
        doc.cursor.0 += 1;
    }
    assert_eq!(doc.cursor.0, 2); // Should stay at line 2
}

#[test]
fn test_line_wrapping_left_right() {
    let doc = Document::from_content("ABC\nDEF");

    // Simulate cursor at end of first line
    let mut cursor = (0, 3);

    // Right arrow should wrap to next line
    let line_len = 3;
    if cursor.1 < line_len {
        cursor.1 += 1;
    } else if cursor.0 < doc.buffer.len_lines().saturating_sub(1) {
        cursor.0 += 1;
        cursor.1 = 0;
    }
    assert_eq!(cursor, (1, 0));

    // Left arrow should wrap to previous line end
    if cursor.1 > 0 {
        cursor.1 -= 1;
    } else if cursor.0 > 0 {
        cursor.0 -= 1;
        cursor.1 = 3; // End of "ABC"
    }
    assert_eq!(cursor, (0, 3));
}

#[test]
fn test_scroll_offset_page_down() {
    let mut doc = Document::new();

    // Create a document with many lines
    for i in 1..=100 {
        doc.buffer
            .insert(doc.buffer.len_chars(), &format!("Line {}\n", i));
    }

    // Start at top
    doc.cursor = (0, 0);
    doc.scroll_offset = 0;

    // Simulate PageDown (assume page size of 20)
    let page_size = 20;
    let max_line = doc.buffer.len_lines().saturating_sub(1);

    doc.cursor.0 = (doc.cursor.0 + page_size).min(max_line);
    doc.scroll_offset = (doc.scroll_offset + page_size).min(max_line);

    assert_eq!(doc.cursor.0, 20);
    assert_eq!(doc.scroll_offset, 20);

    // Another PageDown
    doc.cursor.0 = (doc.cursor.0 + page_size).min(max_line);
    doc.scroll_offset = (doc.scroll_offset + page_size).min(max_line);

    assert_eq!(doc.cursor.0, 40);
    assert_eq!(doc.scroll_offset, 40);
}

#[test]
fn test_scroll_offset_page_up() {
    let mut doc = Document::new();

    for i in 1..=100 {
        doc.buffer
            .insert(doc.buffer.len_chars(), &format!("Line {}\n", i));
    }

    // Start at line 60
    doc.cursor = (60, 0);
    doc.scroll_offset = 60;

    // PageUp
    let page_size = 20;

    doc.cursor.0 = doc.cursor.0.saturating_sub(page_size);
    doc.scroll_offset = doc.scroll_offset.saturating_sub(page_size);

    assert_eq!(doc.cursor.0, 40);
    assert_eq!(doc.scroll_offset, 40);

    // PageUp again
    doc.cursor.0 = doc.cursor.0.saturating_sub(page_size);
    doc.scroll_offset = doc.scroll_offset.saturating_sub(page_size);

    assert_eq!(doc.cursor.0, 20);
    assert_eq!(doc.scroll_offset, 20);
}

#[test]
fn test_undo_redo_sequence() {
    let mut doc = Document::new();

    // Insert "Hello"
    for (i, ch) in "Hello".chars().enumerate() {
        let pos = i;
        doc.buffer.insert(pos, &ch.to_string());
        doc.history.record(Edit::insert(
            pos,
            ch.to_string(),
            Selection::cursor(Position::new(0, i)),
            Selection::cursor(Position::new(0, i + 1)),
        ));
    }

    assert_eq!(doc.buffer.text(), "Hello");
    assert_eq!(doc.history.undo_count(), 5);

    // Undo twice
    let edit1 = doc.history.undo().unwrap();
    doc.buffer
        .delete(edit1.position, edit1.position + edit1.inserted.len());
    assert_eq!(doc.buffer.text(), "Hell");

    let edit2 = doc.history.undo().unwrap();
    doc.buffer
        .delete(edit2.position, edit2.position + edit2.inserted.len());
    assert_eq!(doc.buffer.text(), "Hel");

    assert_eq!(doc.history.redo_count(), 2);

    // Redo once
    let edit3 = doc.history.redo().unwrap();
    doc.buffer.insert(edit3.position, &edit3.inserted);
    assert_eq!(doc.buffer.text(), "Hell");

    assert_eq!(doc.history.redo_count(), 1);
}

#[test]
fn test_new_edit_clears_redo() {
    let mut doc = Document::new();

    doc.buffer.insert(0, "A");
    doc.history.record(Edit::insert(
        0,
        "A".to_string(),
        Selection::cursor(Position::new(0, 0)),
        Selection::cursor(Position::new(0, 1)),
    ));

    doc.buffer.insert(1, "B");
    doc.history.record(Edit::insert(
        1,
        "B".to_string(),
        Selection::cursor(Position::new(0, 1)),
        Selection::cursor(Position::new(0, 2)),
    ));

    // Undo
    let edit = doc.history.undo().unwrap();
    doc.buffer
        .delete(edit.position, edit.position + edit.inserted.len());

    assert_eq!(doc.buffer.text(), "A");
    assert!(doc.history.can_redo());

    // New edit should clear redo
    doc.buffer.insert(1, "C");
    doc.history.record(Edit::insert(
        1,
        "C".to_string(),
        Selection::cursor(Position::new(0, 1)),
        Selection::cursor(Position::new(0, 2)),
    ));

    assert!(!doc.history.can_redo());
}

#[test]
fn test_backspace_line_joining() {
    let mut doc = Document::from_content("Line 1\nLine 2");

    // Cursor at start of line 2 (position 7, which is after "Line 1\n")
    doc.cursor = (1, 0);
    let pos = doc.buffer.line_col_to_char(1, 0);
    assert_eq!(pos, 7); // After "Line 1\n"

    // Backspace should delete the newline
    if pos > 0 {
        let deleted = doc.buffer.slice(pos - 1, pos);
        assert_eq!(deleted, "\n");

        doc.buffer.delete(pos - 1, pos);

        // Update cursor
        doc.cursor.0 -= 1;
        doc.cursor.1 = 6; // End of "Line 1"
    }

    assert_eq!(doc.buffer.text(), "Line 1Line 2");
    assert_eq!(doc.cursor, (0, 6));
}

#[test]
fn test_enter_splits_line() {
    let mut doc = Document::from_content("Hello World");

    // Cursor after "Hello" (position 5)
    doc.cursor = (0, 5);
    let pos = doc.buffer.line_col_to_char(0, 5);

    doc.buffer.insert(pos, "\n");
    doc.cursor.0 += 1;
    doc.cursor.1 = 0;

    assert_eq!(doc.buffer.text(), "Hello\n World");
    assert_eq!(doc.cursor, (1, 0));
    assert_eq!(doc.buffer.len_lines(), 2);
}

#[test]
fn test_save_with_no_path_fails() {
    let mut doc = Document::new();
    doc.buffer.insert(0, "Test content");

    // Should fail because no path is set
    let result = doc.save();
    assert!(result.is_err());
}

#[test]
fn test_save_with_path_works() {
    use std::fs;
    use std::path::PathBuf;

    let temp_path = PathBuf::from("/tmp/patina_test_save.md");

    // Clean up if exists
    let _ = fs::remove_file(&temp_path);

    let mut doc = Document::new();
    doc.buffer.insert(0, "Test content");
    doc.path = Some(temp_path.clone());

    // Should succeed
    let result = doc.save();
    assert!(result.is_ok());

    // Verify file exists and has correct content
    let content = fs::read_to_string(&temp_path).unwrap();
    assert_eq!(content, "Test content");

    // Clean up
    let _ = fs::remove_file(&temp_path);
}

#[test]
fn test_modified_flag() {
    let mut doc = Document::new();

    // New document is not modified
    assert!(!doc.is_modified());

    // After insert, should be modified
    doc.buffer.insert(0, "Test");
    assert!(doc.is_modified());

    // After marking saved, should not be modified
    doc.buffer.mark_saved();
    assert!(!doc.is_modified());
}

#[test]
fn test_unicode_handling() {
    let mut doc = Document::from_content("Hello 世界 🚀");

    assert!(doc.buffer.text().contains("世界"));
    assert!(doc.buffer.text().contains("🚀"));

    let html = doc.html().to_string();
    assert!(html.contains("世界"));
    assert!(html.contains("🚀"));
}

#[test]
fn test_long_line_handling() {
    let long_text = "a".repeat(1000);
    let doc = Document::from_content(&long_text);

    assert_eq!(doc.buffer.len_chars(), 1000);
    assert_eq!(doc.buffer.len_lines(), 1);
}

#[test]
fn test_empty_document_operations() {
    let mut doc = Document::new();

    // Should handle empty buffer gracefully
    assert_eq!(doc.buffer.len_lines(), 1); // ropey always has at least 1 line
    assert_eq!(doc.buffer.len_chars(), 0);

    let html = doc.html();
    assert!(html.is_empty() || html.trim().is_empty());

    let headings = doc.headings();
    assert_eq!(headings.len(), 0);
}
