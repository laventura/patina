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
        }
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
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}
