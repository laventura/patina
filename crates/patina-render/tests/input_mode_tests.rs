//! Tests for input mode functionality

use patina_render::tui::{App, InputMode};

#[test]
fn test_start_open_prompt() {
    let mut app = App::new();

    assert_eq!(app.input_mode, InputMode::Normal);
    assert!(app.input_prompt.is_none());

    app.start_open_prompt();

    assert_eq!(app.input_mode, InputMode::OpenFile);
    assert!(app.input_prompt.is_some());

    let prompt = app.input_prompt.as_ref().unwrap();
    assert_eq!(prompt.prompt, "Open file: ");
    assert_eq!(prompt.buffer, "");
    assert_eq!(prompt.cursor, 0);
}

#[test]
fn test_start_save_as_prompt() {
    let mut app = App::new();

    app.start_save_as_prompt();

    assert_eq!(app.input_mode, InputMode::SaveAs);
    assert!(app.input_prompt.is_some());

    let prompt = app.input_prompt.as_ref().unwrap();
    assert_eq!(prompt.prompt, "Save as: ");
}

#[test]
fn test_cancel_input() {
    let mut app = App::new();

    app.start_open_prompt();
    assert!(app.is_input_mode());

    app.cancel_input();

    assert_eq!(app.input_mode, InputMode::Normal);
    assert!(app.input_prompt.is_none());
    assert!(!app.is_input_mode());
}

#[test]
fn test_finish_input() {
    let mut app = App::new();

    app.start_open_prompt();

    // Simulate typing
    if let Some(prompt) = &mut app.input_prompt {
        prompt.buffer = "test.md".to_string();
        prompt.cursor = 7;
    }

    let result = app.finish_input();

    assert_eq!(result, Some("test.md".to_string()));
    assert_eq!(app.input_mode, InputMode::Normal);
    assert!(app.input_prompt.is_none());
}

#[test]
fn test_input_buffer_simulation() {
    let mut app = App::new();

    app.start_save_as_prompt();

    // Simulate typing "new_file.md"
    if let Some(prompt) = &mut app.input_prompt {
        for ch in "new_file.md".chars() {
            prompt.buffer.insert(prompt.cursor, ch);
            prompt.cursor += 1;
        }

        assert_eq!(prompt.buffer, "new_file.md");
        assert_eq!(prompt.cursor, 11);

        // Simulate backspace (delete 'd')
        if prompt.cursor > 0 {
            prompt.buffer.remove(prompt.cursor - 1);
            prompt.cursor -= 1;
        }

        assert_eq!(prompt.buffer, "new_file.m");
        assert_eq!(prompt.cursor, 10);

        // Simulate moving cursor left
        prompt.cursor = prompt.cursor.saturating_sub(2);
        assert_eq!(prompt.cursor, 8);

        // Insert at cursor position
        prompt.buffer.insert(prompt.cursor, 'x');
        prompt.cursor += 1;

        assert_eq!(prompt.buffer, "new_filex.m");
    }
}
