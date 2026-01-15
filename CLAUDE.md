# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Patina is a fast, lightweight Markdown editor written in pure Rust. It provides both a terminal UI (TUI) via ratatui and a GUI version (planned) via egui. The project uses a Cargo workspace architecture with multiple crates that separate concerns cleanly.

**Performance targets:**
- Cold start: <50ms
- Memory footprint: <20MB idle
- Binary size: <15MB with all features

## Build Commands

```bash
# Build the entire workspace
cargo build --workspace

# Build release version (optimized with LTO, stripped)
cargo build --release

# Run the TUI editor in development
cargo run -- test.md

# Install TUI binary to ~/.cargo/bin
cargo install --path crates/patina

# Run tests across all crates
cargo test --workspace

# Check code formatting
cargo fmt --check

# Run clippy lints
cargo clippy --workspace

# Run specific crate tests
cargo test -p patina-core
cargo test -p patina-render
```

## Workspace Architecture

The project uses a **Cargo workspace** with 6 crates organized by responsibility:

### Core Library (`patina-core`)
**Purpose:** Platform-agnostic text editing logic and document model.

Key components:
- `Buffer` - Rope-based text buffer (ropey) for O(log n) edits
- `Document` - Combines buffer + frontmatter + file metadata + history
- `MarkdownParser` - Markdown parsing via comrak (GFM support)
- `Frontmatter` - YAML/TOML frontmatter extraction
- `Selection` - Text selection handling
- `History` - Undo/redo with command pattern
- `Syntax` - Syntax highlighting via syntect

**Dependencies:** No rendering dependencies. This crate should remain UI-agnostic.

### Rendering Backend (`patina-render`)
**Purpose:** Render documents for both TUI and GUI.

Features:
- `tui` feature (default) - ratatui-based terminal rendering
- `gui` feature (optional) - egui-based desktop rendering
- `Theme` - Theme system (Dracula, One Dark, Solarized, etc.)
- `EditorStyle` - Style configuration
- `Color` - Platform-agnostic color with conversions to ratatui/egui

**Architecture note:** Uses feature flags to conditionally compile TUI or GUI backends. Both backends share the same core `patina-core` types.

### Extensions (`patina-extensions`)
**Purpose:** Advanced Markdown features (LaTeX, Mermaid, Emoji).

Modules:
- `latex` - LaTeX math rendering to Unicode
- `mermaid` - Mermaid diagram rendering to ASCII art
- `emoji` - Emoji shortcode expansion (`:rocket:` → 🚀)

**Design:** Extensions operate on parsed Markdown and are decoupled from both core and rendering.

### TUI Binary (`patina`)
**Purpose:** Terminal-based editor binary.

Key files:
- `main.rs` - CLI argument parsing (clap), initialization
- `app.rs` - Main application loop, event handling
- `ui.rs` - UI layout and rendering logic
- `input.rs` - Keyboard input handling
- `config.rs` - Configuration loading/saving

**Features:**
- Git integration (git2)
- File watching (notify)
- Fuzzy file finding (fuzzy-matcher)

### GUI Binary (`patina-gui`)
**Purpose:** Desktop GUI version (v0.5 roadmap).

**Status:** Stub implementation. Uses egui/eframe when `gui` feature is enabled.

### i18n (`patina-i18n`)
**Purpose:** Internationalization using Fluent.

**Dependencies:** fluent, fluent-bundle, unic-langid

## Development Workflow

### Text Buffer Architecture
- **Never use String for large text** - The `Buffer` type uses `ropey::Rope` for efficient edits
- Cursor positions are `(line, column)` tuples (0-indexed)
- Buffer modifications go through the `History` system for undo/redo
- The buffer tracks `modified` state for save indicators

### Document Model
The `Document` struct is the single source of truth:
```rust
pub struct Document {
    pub buffer: Buffer,           // Text content
    pub frontmatter: Option<Frontmatter>,  // Parsed YAML/TOML
    pub path: Option<PathBuf>,    // File path (None = untitled)
    pub history: History,         // Undo/redo stack
    pub cursor: (usize, usize),   // (line, col)
    pub scroll_offset: usize,     // For view restoration
}
```

When modifying document logic, ensure all state changes maintain consistency across these fields.

### Rendering Pipeline
1. **Input** → `app.rs` event loop captures keyboard/mouse events
2. **Update** → Event handlers modify `Document` state via `Buffer` methods
3. **Render** → `ui.rs` calls `patina-render` to convert `Document` to terminal output
4. **Extensions** → Applied during render phase (not on every keystroke)

**Performance consideration:** Extensions (LaTeX, Mermaid) should only run when rendering the preview pane, not during active editing.

### Feature Flags
- Use `#[cfg(feature = "tui")]` for TUI-specific code in `patina-render`
- Use `#[cfg(feature = "gui")]` for GUI-specific code
- Default features: `["tui"]`

### Testing Strategy
- `patina-core` has comprehensive unit tests (buffer operations, parsing)
- `patina-render` tests rendering logic against snapshot fixtures
- Integration tests in `crates/patina/tests/` test full editor workflows
- Use `cargo test --workspace` to run all tests

## Codebase Conventions

### Error Handling
- `patina-core` defines `Result<T>` and `Error` enum
- Use `thiserror` for error types, `anyhow` for application-level errors
- Propagate errors with `?` operator, handle at application boundary

### Logging
- Use `log` crate macros: `trace!`, `debug!`, `info!`, `warn!`, `error!`
- Initialize with `env_logger` in binaries
- Set `RUST_LOG=debug` for verbose logging

### Imports
Follow Rust convention:
```rust
// std
use std::path::PathBuf;

// External crates
use ropey::Rope;
use ratatui::Terminal;

// Internal workspace crates
use patina_core::{Buffer, Document};

// Local modules
use crate::config::Config;
```

## Key Architectural Decisions

### Why Rope for Text Buffer?
- O(log n) insertions/deletions vs O(n) for String
- Efficient line-based access (critical for editor operations)
- Low memory overhead for large files

### Why Separate `patina-core` from `patina-render`?
- **Testability** - Core logic can be tested without UI dependencies
- **Reusability** - Same core powers both TUI and GUI
- **Platform independence** - Core works on any platform

### Why Feature Flags for TUI/GUI?
- Avoids pulling in both ratatui and egui dependencies when only one is needed
- Keeps binary size small for single-target builds
- Allows workspace members to depend on `patina-render` with specific features

### Why Workspace Instead of Single Crate?
- **Compile times** - Change in `patina-extensions` doesn't rebuild `patina-core`
- **Dependency isolation** - GUI dependencies don't leak into TUI binary
- **Clear boundaries** - Forces good API design at crate boundaries

## Common Patterns

### Adding a New Document Operation
1. Add method to `Buffer` in `patina-core/src/buffer.rs`
2. Create `History::Command` variant if operation should be undoable
3. Wire up keyboard shortcut in `patina/src/input.rs`
4. Add UI feedback in `patina/src/ui.rs`
5. Write unit test in `buffer.rs`

### Adding a New Theme
1. Add theme definition in `patina-render/src/theme.rs`
2. Implement `Theme` methods for color mappings
3. Update `Theme::by_name()` to recognize new theme
4. Update config schema to allow theme selection

### Adding a New Extension
1. Create module in `patina-extensions/src/`
2. Implement processor that takes Markdown AST and returns modified AST
3. Register in extension pipeline
4. Add integration test with sample Markdown

## Release Profile Notes

The workspace uses aggressive optimization for release builds:
- **LTO enabled** - Link-time optimization for smaller binary
- **codegen-units = 1** - Better optimization at cost of compile time
- **panic = "abort"** - No unwinding, smaller binary
- **strip = true** - Remove debug symbols

For development, dependencies are optimized (`opt-level = 3`) even in dev mode to keep TUI responsive.

## Platform Considerations

- **macOS/Linux:** Primary development targets, full feature support
- **Windows:** Supported via crossterm, test on Windows before releases
- **SSH/tmux:** TUI must work over SSH and within terminal multiplexers
