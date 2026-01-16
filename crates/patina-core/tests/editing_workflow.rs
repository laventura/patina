//! Integration test for full editing workflow

use patina_core::selection::{Position, Selection};
use patina_core::{Document, Edit};

#[test]
fn test_complete_editing_workflow() {
    let mut doc = Document::new();

    // Start with empty document
    assert_eq!(doc.buffer.text(), "");
    assert_eq!(doc.cursor, (0, 0));

    // Insert some text
    let pos = doc.buffer.line_col_to_char(0, 0);
    doc.buffer.insert(pos, "Hello");
    doc.cursor = (0, 5);

    let edit1 = Edit::insert(
        pos,
        "Hello".to_string(),
        Selection::cursor(Position::new(0, 0)),
        Selection::cursor(Position::new(0, 5)),
    );
    doc.history.record(edit1);

    assert_eq!(doc.buffer.text(), "Hello");
    assert!(doc.is_modified());

    // Insert a space
    let pos = doc.buffer.line_col_to_char(0, 5);
    doc.buffer.insert(pos, " ");
    doc.cursor = (0, 6);

    let edit2 = Edit::insert(
        pos,
        " ".to_string(),
        Selection::cursor(Position::new(0, 5)),
        Selection::cursor(Position::new(0, 6)),
    );
    doc.history.record(edit2);

    // Insert more text
    let pos = doc.buffer.line_col_to_char(0, 6);
    doc.buffer.insert(pos, "World");
    doc.cursor = (0, 11);

    let edit3 = Edit::insert(
        pos,
        "World".to_string(),
        Selection::cursor(Position::new(0, 6)),
        Selection::cursor(Position::new(0, 11)),
    );
    doc.history.record(edit3);

    assert_eq!(doc.buffer.text(), "Hello World");

    // Insert a newline
    let pos = doc.buffer.line_col_to_char(0, 11);
    doc.buffer.insert(pos, "\n");
    doc.cursor = (1, 0);

    let edit4 = Edit::insert(
        pos,
        "\n".to_string(),
        Selection::cursor(Position::new(0, 11)),
        Selection::cursor(Position::new(1, 0)),
    );
    doc.history.record(edit4);

    // Insert text on second line
    let pos = doc.buffer.line_col_to_char(1, 0);
    doc.buffer.insert(pos, "Second line");
    doc.cursor = (1, 11);

    let edit5 = Edit::insert(
        pos,
        "Second line".to_string(),
        Selection::cursor(Position::new(1, 0)),
        Selection::cursor(Position::new(1, 11)),
    );
    doc.history.record(edit5);

    assert_eq!(doc.buffer.text(), "Hello World\nSecond line");
    assert_eq!(doc.buffer.len_lines(), 2);

    // Test undo
    assert!(doc.history.can_undo());
    let edit = doc.history.undo().unwrap();
    doc.buffer
        .delete(edit.position, edit.position + edit.inserted.len());
    doc.cursor = (edit.cursor_before.head.line, edit.cursor_before.head.col);

    assert_eq!(doc.buffer.text(), "Hello World\n");

    // Undo again
    let edit = doc.history.undo().unwrap();
    doc.buffer
        .delete(edit.position, edit.position + edit.inserted.len());
    doc.cursor = (edit.cursor_before.head.line, edit.cursor_before.head.col);

    assert_eq!(doc.buffer.text(), "Hello World");

    // Redo
    assert!(doc.history.can_redo());
    let edit = doc.history.redo().unwrap();
    doc.buffer.insert(edit.position, &edit.inserted);
    doc.cursor = (edit.cursor_after.head.line, edit.cursor_after.head.col);

    assert_eq!(doc.buffer.text(), "Hello World\n");
}

#[test]
fn test_delete_operations() {
    let mut doc = Document::from_content("Hello World");
    doc.cursor = (0, 5);

    // Delete forward (delete the space)
    let pos = doc.buffer.line_col_to_char(0, 5);
    let deleted = doc.buffer.slice(pos, pos + 1);
    doc.buffer.delete(pos, pos + 1);

    let edit = Edit::delete(
        pos,
        deleted,
        Selection::cursor(Position::new(0, 5)),
        Selection::cursor(Position::new(0, 5)),
    );
    doc.history.record(edit);

    assert_eq!(doc.buffer.text(), "HelloWorld");

    // Delete backward (backspace the 'o')
    let pos = doc.buffer.line_col_to_char(0, 5);
    let deleted = doc.buffer.slice(pos - 1, pos);
    doc.buffer.delete(pos - 1, pos);
    doc.cursor = (0, 4);

    let edit = Edit::delete(
        pos - 1,
        deleted,
        Selection::cursor(Position::new(0, 5)),
        Selection::cursor(Position::new(0, 4)),
    );
    doc.history.record(edit);

    assert_eq!(doc.buffer.text(), "HellWorld");

    // Undo delete backward
    let edit = doc.history.undo().unwrap();
    doc.buffer.insert(edit.position, &edit.deleted);
    doc.cursor = (edit.cursor_before.head.line, edit.cursor_before.head.col);

    assert_eq!(doc.buffer.text(), "HelloWorld");
}

#[test]
fn test_cursor_movement_simulation() {
    let doc = Document::from_content("Line 1\nLine 2\nLine 3");

    // Test line lengths
    assert_eq!(doc.buffer.len_lines(), 3);

    // Simulate cursor at start
    let mut cursor = (0, 0);
    assert_eq!(doc.buffer.line_col_to_char(cursor.0, cursor.1), 0);

    // Move to end of first line
    cursor = (0, 6);
    let pos = doc.buffer.line_col_to_char(cursor.0, cursor.1);
    assert_eq!(doc.buffer.slice(pos, pos), "");

    // Move to start of second line
    cursor = (1, 0);
    let pos = doc.buffer.line_col_to_char(cursor.0, cursor.1);
    assert_eq!(doc.buffer.slice(pos, pos + 1), "L");

    // Move to end of last line
    cursor = (2, 6);
    let pos = doc.buffer.line_col_to_char(cursor.0, cursor.1);
    assert_eq!(pos, doc.buffer.len_chars());
}

#[test]
fn test_multiline_editing() {
    let mut doc = Document::from_content("A\nB\nC");

    // Insert at start of line 1 (index 0 is "A")
    let pos = doc.buffer.line_col_to_char(1, 0);
    doc.buffer.insert(pos, "New ");

    assert_eq!(doc.buffer.text(), "A\nNew B\nC");

    // Delete the newline between A and New (join lines)
    let newline_pos = doc.buffer.line_col_to_char(0, 1);
    doc.buffer.delete(newline_pos, newline_pos + 1);

    assert_eq!(doc.buffer.text(), "ANew B\nC");
    assert_eq!(doc.buffer.len_lines(), 2);
}
