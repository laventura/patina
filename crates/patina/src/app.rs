//! Main application logic.

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;
use std::path::PathBuf;
use std::time::{Duration, Instant};

use patina_core::{selection::Position, Document, Edit, Selection};
use patina_render::tui::App as TuiApp;
use patina_render::Theme;

use crate::config::Config;
use crate::ui;

/// Main application
pub struct App {
    /// TUI app state
    tui: TuiApp,
    /// Terminal
    terminal: Terminal<CrosstermBackend<io::Stdout>>,
    /// Configuration
    config: Config,
    /// Whether quit was requested with unsaved changes (for confirmation)
    quit_pending: bool,
    /// Last auto-save check time
    last_auto_save: Instant,
    /// Terminal height (for page sizing)
    terminal_height: u16,
}

impl App {
    /// Create a new application
    pub fn new(config: Config) -> Result<Self> {
        let terminal = patina_render::tui::init_terminal()?;
        let mut tui = TuiApp::new();

        // Apply config
        tui.theme = Theme::by_name(&config.theme);

        // Get initial terminal size
        let terminal_height = terminal.size()?.height;

        Ok(Self {
            tui,
            terminal,
            config,
            quit_pending: false,
            last_auto_save: Instant::now(),
            terminal_height,
        })
    }

    /// Run the main event loop
    pub fn run(&mut self) -> Result<()> {
        loop {
            // Update terminal size
            self.terminal_height = self.terminal.size()?.height;

            // Draw UI
            self.terminal.draw(|frame| {
                ui::draw(frame, &self.tui);
            })?;

            // Handle events
            if event::poll(Duration::from_millis(100))? {
                if let Event::Key(key) = event::read()? {
                    self.handle_key(key)?;
                }
            }

            // Auto-save check
            self.check_auto_save()?;

            // Check for quit
            if self.tui.should_quit {
                break;
            }
        }

        // Cleanup
        patina_render::tui::restore_terminal(&mut self.terminal)?;
        Ok(())
    }

    /// Check and perform auto-save if needed
    fn check_auto_save(&mut self) -> Result<()> {
        let auto_save_secs = self.config.editor.auto_save;
        if auto_save_secs == 0 {
            return Ok(()); // Auto-save disabled
        }

        let elapsed = self.last_auto_save.elapsed().as_secs();
        if elapsed >= auto_save_secs {
            self.last_auto_save = Instant::now();

            // Save all modified documents that have a path
            for doc in &mut self.tui.documents {
                if doc.is_modified() && doc.path.is_some() {
                    if let Err(e) = doc.save() {
                        log::warn!("Auto-save failed: {}", e);
                    } else {
                        log::debug!("Auto-saved: {:?}", doc.path);
                    }
                }
            }
        }

        Ok(())
    }

    /// Handle a key event
    fn handle_key(&mut self, key: event::KeyEvent) -> Result<()> {
        // Handle input mode separately
        if self.tui.is_input_mode() {
            return self.handle_input_mode(key);
        }

        let ctrl = key.modifiers.contains(KeyModifiers::CONTROL);
        let shift = key.modifiers.contains(KeyModifiers::SHIFT);
        let alt = key.modifiers.contains(KeyModifiers::ALT);

        // Clear status message and quit_pending on any key except quit confirmation
        let is_quit_key = ctrl && matches!(key.code, KeyCode::Char('q'));
        if !is_quit_key {
            self.quit_pending = false;
            self.tui.clear_status();
        }

        match key.code {
            // === Application Commands ===

            // Quit
            KeyCode::Char('q') if ctrl => {
                if self.tui.has_unsaved_changes() {
                    if self.quit_pending {
                        // Second Ctrl+Q - force quit
                        self.tui.quit();
                    } else {
                        // First Ctrl+Q - show warning
                        self.quit_pending = true;
                        self.tui.set_status(
                            "Unsaved changes! Press Ctrl+Q again to quit without saving.",
                        );
                    }
                } else {
                    self.tui.quit();
                }
            }

            // Save
            KeyCode::Char('s') if ctrl => {
                self.save_document()?;
            }

            // Open
            KeyCode::Char('o') if ctrl => {
                self.tui.start_open_prompt();
            }

            // New
            KeyCode::Char('n') if ctrl => {
                self.new_document();
            }

            // Close tab
            KeyCode::Char('w') if ctrl => {
                self.tui.close_active_document();
            }

            // Next tab (Ctrl+Tab, might not work in all terminals)
            KeyCode::Tab if ctrl => {
                self.tui.next_document();
            }

            // Previous tab (Ctrl+Shift+Tab, might not work in all terminals)
            KeyCode::BackTab if ctrl => {
                self.tui.prev_document();
            }

            // Toggle Zen mode (must come before Ctrl+Z for undo)
            KeyCode::Char('z') if ctrl && shift => {
                self.toggle_zen_mode();
            }

            // Undo
            KeyCode::Char('z') if ctrl => {
                self.undo();
            }

            // Redo
            KeyCode::Char('y') if ctrl => {
                self.redo();
            }

            // Cycle view mode (Ctrl+\ or Ctrl+P)
            KeyCode::Char('\\') if ctrl => {
                self.tui.cycle_view_mode();
            }

            KeyCode::Char('p') if ctrl => {
                self.tui.cycle_view_mode();
            }

            // Next tab (Alt+Right - works in all terminals)
            KeyCode::Right if alt => {
                self.tui.next_document();
            }

            // Previous tab (Alt+Left - works in all terminals)
            KeyCode::Left if alt => {
                self.tui.prev_document();
            }

            // === Navigation ===
            KeyCode::Up => {
                let doc = self.tui.active_document_mut();
                if doc.cursor.0 > 0 {
                    doc.cursor.0 -= 1;
                    // Clamp column to new line length
                    doc.cursor.1 = doc.cursor.1.min(Self::line_length(doc, doc.cursor.0));
                }
            }

            KeyCode::Down => {
                let doc = self.tui.active_document_mut();
                let max_line = doc.buffer.len_lines().saturating_sub(1);
                if doc.cursor.0 < max_line {
                    doc.cursor.0 += 1;
                    // Clamp column to new line length
                    doc.cursor.1 = doc.cursor.1.min(Self::line_length(doc, doc.cursor.0));
                }
            }

            KeyCode::Left => {
                let doc = self.tui.active_document_mut();
                if doc.cursor.1 > 0 {
                    doc.cursor.1 -= 1;
                } else if doc.cursor.0 > 0 {
                    // Wrap to end of previous line
                    doc.cursor.0 -= 1;
                    doc.cursor.1 = Self::line_length(doc, doc.cursor.0);
                }
            }

            KeyCode::Right => {
                let doc = self.tui.active_document_mut();
                let line_len = Self::line_length(doc, doc.cursor.0);
                if doc.cursor.1 < line_len {
                    doc.cursor.1 += 1;
                } else if doc.cursor.0 < doc.buffer.len_lines().saturating_sub(1) {
                    // Wrap to start of next line
                    doc.cursor.0 += 1;
                    doc.cursor.1 = 0;
                }
            }

            KeyCode::Home => {
                let doc = self.tui.active_document_mut();
                doc.cursor.1 = 0;
            }

            KeyCode::End => {
                let doc = self.tui.active_document_mut();
                doc.cursor.1 = Self::line_length(doc, doc.cursor.0);
            }

            KeyCode::PageUp => {
                let doc = self.tui.active_document_mut();
                // Page size is terminal height minus UI elements (status bar, etc.)
                let page_size = (self.terminal_height.saturating_sub(3)) as usize;
                doc.cursor.0 = doc.cursor.0.saturating_sub(page_size);
                doc.scroll_offset = doc.scroll_offset.saturating_sub(page_size);
                // Clamp column to new line length
                doc.cursor.1 = doc.cursor.1.min(Self::line_length(doc, doc.cursor.0));
            }

            KeyCode::PageDown => {
                let doc = self.tui.active_document_mut();
                // Page size is terminal height minus UI elements (status bar, etc.)
                let page_size = (self.terminal_height.saturating_sub(3)) as usize;
                let max_line = doc.buffer.len_lines().saturating_sub(1);
                doc.cursor.0 = (doc.cursor.0 + page_size).min(max_line);
                doc.scroll_offset = (doc.scroll_offset + page_size).min(max_line);
                // Clamp column to new line length
                doc.cursor.1 = doc.cursor.1.min(Self::line_length(doc, doc.cursor.0));
            }

            // === Text Editing ===
            KeyCode::Char(c) if !ctrl && !alt => {
                // Only insert regular characters, not Ctrl/Alt combinations
                self.insert_char(c);
            }

            KeyCode::Enter => {
                self.insert_newline();
            }

            KeyCode::Tab => {
                // Insert tab character or spaces based on config
                if self.config.editor.use_spaces {
                    // Insert configured number of spaces
                    let spaces = " ".repeat(self.config.editor.tab_size);
                    for ch in spaces.chars() {
                        self.insert_char(ch);
                    }
                } else {
                    // Insert actual tab character
                    self.insert_char('\t');
                }
            }

            KeyCode::Backspace => {
                self.delete_backward();
            }

            KeyCode::Delete => {
                self.delete_forward();
            }

            _ => {}
        }

        // Ensure cursor is visible after any operation
        self.ensure_cursor_visible();

        Ok(())
    }

    /// Get the length of a line (excluding newline character)
    fn line_length(doc: &Document, line_idx: usize) -> usize {
        doc.buffer
            .line(line_idx)
            .map(|l| l.trim_end_matches('\n').len())
            .unwrap_or(0)
    }

    /// Get cursor as Selection
    fn cursor_selection(doc: &Document) -> Selection {
        Selection::cursor(Position::new(doc.cursor.0, doc.cursor.1))
    }

    /// Ensure cursor is visible by adjusting scroll offset
    fn ensure_cursor_visible(&mut self) {
        let doc = self.tui.active_document_mut();
        let cursor_line = doc.cursor.0;

        // Calculate visible area (terminal height minus UI elements)
        let visible_lines = (self.terminal_height.saturating_sub(3)) as usize;

        // If cursor is above visible area, scroll up
        if cursor_line < doc.scroll_offset {
            doc.scroll_offset = cursor_line;
        }

        // If cursor is below visible area, scroll down
        let bottom_visible_line = doc.scroll_offset + visible_lines.saturating_sub(1);
        if cursor_line > bottom_visible_line {
            doc.scroll_offset = cursor_line.saturating_sub(visible_lines.saturating_sub(1));
        }
    }

    /// Insert a character at cursor
    fn insert_char(&mut self, c: char) {
        let doc = self.tui.active_document_mut();
        let cursor_before = Self::cursor_selection(doc);
        let pos = doc.buffer.line_col_to_char(doc.cursor.0, doc.cursor.1);

        let text = c.to_string();
        doc.buffer.insert(pos, &text);
        doc.cursor.1 += 1;

        let cursor_after = Self::cursor_selection(doc);
        doc.history
            .record(Edit::insert(pos, text, cursor_before, cursor_after));
    }

    /// Insert a newline at cursor
    fn insert_newline(&mut self) {
        let doc = self.tui.active_document_mut();
        let cursor_before = Self::cursor_selection(doc);
        let pos = doc.buffer.line_col_to_char(doc.cursor.0, doc.cursor.1);

        doc.buffer.insert(pos, "\n");
        doc.cursor.0 += 1;
        doc.cursor.1 = 0;

        let cursor_after = Self::cursor_selection(doc);
        doc.history.record(Edit::insert(
            pos,
            "\n".to_string(),
            cursor_before,
            cursor_after,
        ));
    }

    /// Delete character before cursor (backspace)
    fn delete_backward(&mut self) {
        let doc = self.tui.active_document_mut();
        let pos = doc.buffer.line_col_to_char(doc.cursor.0, doc.cursor.1);

        if pos > 0 {
            let cursor_before = Self::cursor_selection(doc);
            let deleted = doc.buffer.slice(pos - 1, pos);

            doc.buffer.delete(pos - 1, pos);

            // Update cursor position
            if doc.cursor.1 > 0 {
                doc.cursor.1 -= 1;
            } else if doc.cursor.0 > 0 {
                // Joined with previous line
                doc.cursor.0 -= 1;
                doc.cursor.1 = Self::line_length(doc, doc.cursor.0);
            }

            let cursor_after = Self::cursor_selection(doc);
            doc.history
                .record(Edit::delete(pos - 1, deleted, cursor_before, cursor_after));
        }
    }

    /// Delete character at cursor (delete key)
    fn delete_forward(&mut self) {
        let doc = self.tui.active_document_mut();
        let pos = doc.buffer.line_col_to_char(doc.cursor.0, doc.cursor.1);
        let total_chars = doc.buffer.len_chars();

        if pos < total_chars {
            let cursor_before = Self::cursor_selection(doc);
            let deleted = doc.buffer.slice(pos, pos + 1);

            doc.buffer.delete(pos, pos + 1);

            // Cursor stays in same position
            let cursor_after = Self::cursor_selection(doc);
            doc.history
                .record(Edit::delete(pos, deleted, cursor_before, cursor_after));
        }
    }

    /// Undo the last edit
    fn undo(&mut self) {
        let doc = self.tui.active_document_mut();
        if let Some(edit) = doc.history.undo() {
            // Reverse the edit
            if !edit.inserted.is_empty() {
                // Was an insertion, so delete
                doc.buffer
                    .delete(edit.position, edit.position + edit.inserted.len());
            }
            if !edit.deleted.is_empty() {
                // Was a deletion, so insert
                doc.buffer.insert(edit.position, &edit.deleted);
            }
            // Restore cursor
            doc.cursor = (edit.cursor_before.head.line, edit.cursor_before.head.col);
        }
    }

    /// Redo the last undone edit
    fn redo(&mut self) {
        let doc = self.tui.active_document_mut();
        if let Some(edit) = doc.history.redo() {
            // Reapply the edit
            if !edit.deleted.is_empty() {
                // Was a deletion, so delete again
                doc.buffer
                    .delete(edit.position, edit.position + edit.deleted.len());
            }
            if !edit.inserted.is_empty() {
                // Was an insertion, so insert again
                doc.buffer.insert(edit.position, &edit.inserted);
            }
            // Restore cursor
            doc.cursor = (edit.cursor_after.head.line, edit.cursor_after.head.col);
        }
    }

    /// Open a file (or create new document with that path if file doesn't exist)
    pub fn open_file(&mut self, path: PathBuf) -> Result<()> {
        let doc = if path.exists() {
            Document::from_file(path)?
        } else {
            // Create new document with path set (will be created on save)
            let mut doc = Document::new();
            doc.path = Some(path);
            doc
        };
        self.tui.open_document(doc);
        Ok(())
    }

    /// Open a workspace
    pub fn open_workspace(&mut self, _path: PathBuf) -> Result<()> {
        // TODO: Implement workspace mode
        Ok(())
    }

    /// Create a new document
    pub fn new_document(&mut self) {
        self.tui.open_document(Document::new());
    }

    /// Save the active document
    fn save_document(&mut self) -> Result<()> {
        let doc = self.tui.active_document_mut();
        if let Some(path) = doc.path.clone() {
            doc.save()?;
            let filename = path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("file")
                .to_string();
            self.tui.set_status(format!("✓ Saved: {}", filename));
        } else {
            // Start Save As prompt
            self.tui.start_save_as_prompt();
        }
        Ok(())
    }

    /// Set the theme
    pub fn set_theme(&mut self, theme: &str) {
        self.tui.theme = Theme::by_name(theme);
    }

    /// Toggle Zen mode
    pub fn toggle_zen_mode(&mut self) {
        self.tui.toggle_zen_mode();
    }

    /// Handle key events when in input mode
    fn handle_input_mode(&mut self, key: event::KeyEvent) -> Result<()> {
        use patina_render::tui::InputMode;

        match key.code {
            KeyCode::Esc => {
                // Cancel input
                self.tui.cancel_input();
            }
            KeyCode::Enter => {
                // Finish input and process
                let mode = self.tui.input_mode.clone();
                if let Some(input) = self.tui.finish_input() {
                    match mode {
                        InputMode::OpenFile => {
                            let path = PathBuf::from(input);
                            if let Err(e) = self.open_file(path) {
                                self.tui.set_status(format!("Error opening file: {}", e));
                            }
                        }
                        InputMode::SaveAs => {
                            let path = PathBuf::from(input);
                            let filename = path
                                .file_name()
                                .and_then(|n| n.to_str())
                                .unwrap_or("file")
                                .to_string();
                            let doc = self.tui.active_document_mut();
                            doc.path = Some(path);
                            if let Err(e) = doc.save() {
                                self.tui.set_status(format!("✗ Error saving file: {}", e));
                            } else {
                                self.tui.set_status(format!("✓ Saved: {}", filename));
                            }
                        }
                        InputMode::Normal => {}
                    }
                }
            }
            KeyCode::Backspace => {
                // Delete character
                if let Some(prompt) = &mut self.tui.input_prompt {
                    if prompt.cursor > 0 {
                        prompt.buffer.remove(prompt.cursor - 1);
                        prompt.cursor -= 1;
                    }
                }
            }
            KeyCode::Delete => {
                // Delete character at cursor
                if let Some(prompt) = &mut self.tui.input_prompt {
                    if prompt.cursor < prompt.buffer.len() {
                        prompt.buffer.remove(prompt.cursor);
                    }
                }
            }
            KeyCode::Left => {
                // Move cursor left
                if let Some(prompt) = &mut self.tui.input_prompt {
                    if prompt.cursor > 0 {
                        prompt.cursor -= 1;
                    }
                }
            }
            KeyCode::Right => {
                // Move cursor right
                if let Some(prompt) = &mut self.tui.input_prompt {
                    if prompt.cursor < prompt.buffer.len() {
                        prompt.cursor += 1;
                    }
                }
            }
            KeyCode::Home => {
                // Move to start
                if let Some(prompt) = &mut self.tui.input_prompt {
                    prompt.cursor = 0;
                }
            }
            KeyCode::End => {
                // Move to end
                if let Some(prompt) = &mut self.tui.input_prompt {
                    prompt.cursor = prompt.buffer.len();
                }
            }
            KeyCode::Char(c) => {
                // Insert character at cursor
                if let Some(prompt) = &mut self.tui.input_prompt {
                    prompt.buffer.insert(prompt.cursor, c);
                    prompt.cursor += 1;
                }
            }
            _ => {}
        }
        Ok(())
    }
}
