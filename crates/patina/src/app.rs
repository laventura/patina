//! Main application logic.

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;
use std::path::PathBuf;
use std::time::Duration;

use patina_core::Document;
use patina_render::tui::{App as TuiApp, ViewMode};
use patina_render::Theme;
use patina_i18n::t;

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
}

impl App {
    /// Create a new application
    pub fn new(config: Config) -> Result<Self> {
        let terminal = patina_render::tui::init_terminal()?;
        let mut tui = TuiApp::new();
        
        // Apply config
        tui.theme = Theme::by_name(&config.theme);
        
        Ok(Self {
            tui,
            terminal,
            config,
        })
    }

    /// Run the main event loop
    pub fn run(&mut self) -> Result<()> {
        loop {
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

            // Check for quit
            if self.tui.should_quit {
                break;
            }
        }

        // Cleanup
        patina_render::tui::restore_terminal(&mut self.terminal)?;
        Ok(())
    }

    /// Handle a key event
    fn handle_key(&mut self, key: event::KeyEvent) -> Result<()> {
        let ctrl = key.modifiers.contains(KeyModifiers::CONTROL);
        let shift = key.modifiers.contains(KeyModifiers::SHIFT);

        match key.code {
            // Quit
            KeyCode::Char('q') if ctrl => {
                self.tui.quit();
            }
            
            // Save
            KeyCode::Char('s') if ctrl => {
                self.save_document()?;
            }
            
            // Open
            KeyCode::Char('o') if ctrl => {
                // TODO: Open file dialog
            }
            
            // New
            KeyCode::Char('n') if ctrl => {
                self.new_document();
            }
            
            // Close tab
            KeyCode::Char('w') if ctrl => {
                self.tui.close_active_document();
            }
            
            // Next tab
            KeyCode::Tab if ctrl => {
                self.tui.next_document();
            }
            
            // Previous tab
            KeyCode::BackTab if ctrl => {
                self.tui.prev_document();
            }
            
            // Toggle Zen mode
            KeyCode::Char('z') if ctrl && shift => {
                self.toggle_zen_mode();
            }
            
            // Cycle view mode
            KeyCode::Char('\\') if ctrl => {
                self.tui.cycle_view_mode();
            }
            
            // Navigation
            KeyCode::Up => {
                let doc = self.tui.active_document_mut();
                if doc.cursor.0 > 0 {
                    doc.cursor.0 -= 1;
                }
            }
            KeyCode::Down => {
                let doc = self.tui.active_document_mut();
                let max_line = doc.buffer.len_lines().saturating_sub(1);
                if doc.cursor.0 < max_line {
                    doc.cursor.0 += 1;
                }
            }
            KeyCode::Left => {
                let doc = self.tui.active_document_mut();
                if doc.cursor.1 > 0 {
                    doc.cursor.1 -= 1;
                }
            }
            KeyCode::Right => {
                let doc = self.tui.active_document_mut();
                doc.cursor.1 += 1;
            }
            
            // Text input
            KeyCode::Char(c) => {
                let doc = self.tui.active_document_mut();
                let pos = doc.buffer.line_col_to_char(doc.cursor.0, doc.cursor.1);
                doc.buffer.insert(pos, &c.to_string());
                doc.cursor.1 += 1;
            }
            
            KeyCode::Enter => {
                let doc = self.tui.active_document_mut();
                let pos = doc.buffer.line_col_to_char(doc.cursor.0, doc.cursor.1);
                doc.buffer.insert(pos, "\n");
                doc.cursor.0 += 1;
                doc.cursor.1 = 0;
            }
            
            KeyCode::Backspace => {
                let doc = self.tui.active_document_mut();
                let pos = doc.buffer.line_col_to_char(doc.cursor.0, doc.cursor.1);
                if pos > 0 {
                    doc.buffer.delete(pos - 1, pos);
                    if doc.cursor.1 > 0 {
                        doc.cursor.1 -= 1;
                    } else if doc.cursor.0 > 0 {
                        doc.cursor.0 -= 1;
                        if let Some(line) = doc.buffer.line(doc.cursor.0) {
                            doc.cursor.1 = line.len().saturating_sub(1);
                        }
                    }
                }
            }

            _ => {}
        }

        Ok(())
    }

    /// Open a file
    pub fn open_file(&mut self, path: PathBuf) -> Result<()> {
        let doc = Document::from_file(path)?;
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
        if doc.path.is_some() {
            doc.save()?;
        } else {
            // TODO: Save As dialog
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
}
