//! TUI Application state and main loop.

use crate::Theme;
use patina_core::Document;

/// TUI Application state
pub struct App {
    /// Open documents
    pub documents: Vec<Document>,
    /// Active document index
    pub active_doc: usize,
    /// Current theme
    pub theme: Theme,
    /// Should quit
    pub should_quit: bool,
    /// View mode
    pub view_mode: ViewMode,
    /// Zen mode active
    pub zen_mode: bool,
    /// Status message (shown in status bar, cleared on next action)
    pub status_message: Option<String>,
    /// Current input mode
    pub input_mode: InputMode,
    /// Input prompt state (when in input mode)
    pub input_prompt: Option<InputPrompt>,
}

/// Editor view modes
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ViewMode {
    /// Raw markdown editing
    Raw,
    /// Rendered preview only
    Rendered,
    /// Split view (raw + rendered)
    Split,
}

/// Input mode for prompts
#[derive(Debug, Clone, PartialEq)]
pub enum InputMode {
    /// Normal editing mode
    Normal,
    /// Prompting for file path (Open)
    OpenFile,
    /// Prompting for save path (Save As)
    SaveAs,
}

/// Input prompt state
#[derive(Debug, Clone)]
pub struct InputPrompt {
    /// Prompt message to display
    pub prompt: String,
    /// User input buffer
    pub buffer: String,
    /// Cursor position in input buffer
    pub cursor: usize,
}

impl App {
    /// Create a new app
    pub fn new() -> Self {
        Self {
            documents: vec![Document::new()],
            active_doc: 0,
            theme: Theme::default(),
            should_quit: false,
            view_mode: ViewMode::Split,
            zen_mode: false,
            status_message: None,
            input_mode: InputMode::Normal,
            input_prompt: None,
        }
    }

    /// Check if any document has unsaved changes
    pub fn has_unsaved_changes(&self) -> bool {
        self.documents.iter().any(|doc| doc.is_modified())
    }

    /// Set a status message
    pub fn set_status(&mut self, msg: impl Into<String>) {
        self.status_message = Some(msg.into());
    }

    /// Clear the status message
    pub fn clear_status(&mut self) {
        self.status_message = None;
    }

    /// Get the active document
    pub fn active_document(&self) -> &Document {
        &self.documents[self.active_doc]
    }

    /// Get the active document mutably
    pub fn active_document_mut(&mut self) -> &mut Document {
        &mut self.documents[self.active_doc]
    }

    /// Open a new document
    pub fn open_document(&mut self, doc: Document) {
        self.documents.push(doc);
        self.active_doc = self.documents.len() - 1;
    }

    /// Close the active document
    pub fn close_active_document(&mut self) {
        if self.documents.len() > 1 {
            self.documents.remove(self.active_doc);
            if self.active_doc >= self.documents.len() {
                self.active_doc = self.documents.len() - 1;
            }
        }
    }

    /// Switch to next document
    pub fn next_document(&mut self) {
        if self.active_doc < self.documents.len() - 1 {
            self.active_doc += 1;
        } else {
            self.active_doc = 0;
        }
    }

    /// Switch to previous document
    pub fn prev_document(&mut self) {
        if self.active_doc > 0 {
            self.active_doc -= 1;
        } else {
            self.active_doc = self.documents.len() - 1;
        }
    }

    /// Toggle zen mode
    pub fn toggle_zen_mode(&mut self) {
        self.zen_mode = !self.zen_mode;
    }

    /// Cycle view mode
    pub fn cycle_view_mode(&mut self) {
        self.view_mode = match self.view_mode {
            ViewMode::Raw => ViewMode::Rendered,
            ViewMode::Rendered => ViewMode::Split,
            ViewMode::Split => ViewMode::Raw,
        };
    }

    /// Request quit
    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    /// Start prompting for file path (Open)
    pub fn start_open_prompt(&mut self) {
        self.input_mode = InputMode::OpenFile;
        self.input_prompt = Some(InputPrompt {
            prompt: "Open file: ".to_string(),
            buffer: String::new(),
            cursor: 0,
        });
    }

    /// Start prompting for save path (Save As)
    pub fn start_save_as_prompt(&mut self) {
        self.input_mode = InputMode::SaveAs;
        self.input_prompt = Some(InputPrompt {
            prompt: "Save as: ".to_string(),
            buffer: String::new(),
            cursor: 0,
        });
    }

    /// Cancel the current input prompt
    pub fn cancel_input(&mut self) {
        self.input_mode = InputMode::Normal;
        self.input_prompt = None;
    }

    /// Finish input and return the value
    pub fn finish_input(&mut self) -> Option<String> {
        if let Some(prompt) = self.input_prompt.take() {
            self.input_mode = InputMode::Normal;
            Some(prompt.buffer)
        } else {
            None
        }
    }

    /// Check if in input mode
    pub fn is_input_mode(&self) -> bool {
        self.input_mode != InputMode::Normal
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}
