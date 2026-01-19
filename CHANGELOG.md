# Changelog

All notable changes to Patina will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.6.0] - 2026-01-19

### Added
- **LaTeX math rendering**: Inline (`$...$`) and display (`$$...$$`) math rendered to Unicode
  - Greek letters (α, β, γ, δ, θ, π, ω, Γ, Δ, Σ, Ω, etc.)
  - Mathematical operators (∑, ∏, ∫, ∂, √, ∞, ±, ×, ÷, ≤, ≥, ≠, ≈, etc.)
  - Subscripts and superscripts (x₀, a²,  e^{ix}, etc.)
  - Set theory symbols (∈, ∉, ⊂, ⊃, ∪, ∩, ∅, ∀, ∃)
  - Logic symbols (¬, ∧, ∨, →, ⇒, ⇔)
- **Emoji expansion**: Shortcodes (`:rocket:`, `:fire:`, `:star:`, etc.) expand to emoji
  - 80+ common emoji shortcodes supported
  - GitHub-compatible shortcode syntax
  - Works inline with text and LaTeX
- **i18n framework**: Internationalization support with English translations
  - Translation system for UI strings
  - Extensible for future multi-language support
  - Simple key-value lookup with argument substitution
- **Example files**: Comprehensive markdown examples demonstrating all features
  - `examples/latex_and_emoji.md`: LaTeX and emoji showcase
  - `examples/full_features.md`: Complete feature demonstration
  - `examples/frontmatter_sample.md`: Frontmatter examples

### Changed
- Preview rendering now processes LaTeX and emoji in real-time
- Extension system integrated into markdown rendering pipeline
- Improved text processing to handle mixed content (text + math + emoji)

### Technical
- Added `patina-extensions` dependency to `patina-render`
- LaTeX renderer uses Unicode mathematical symbols
- Emoji expander with ~80 shortcodes (expandable to 1800+)
- i18n system using HashMap for fast lookups

## [0.5.0] - 2026-01-16

### Added
- **Syntax highlighting** for 50+ programming languages using syntect
  - Code blocks with language detection
  - Multiple color schemes matching editor themes
  - Fallback to plain text for unsupported languages
- **Frontmatter parsing** for YAML and TOML
  - Extract metadata from markdown documents
  - Display frontmatter in preview pane
  - Support for title, author, date, tags, etc.
- **Theme system** with multiple built-in themes
  - Dracula (default)
  - One Dark
  - Solarized Light/Dark
  - Gruvbox
  - Nord
- **Configuration system** with TOML-based config files
  - `~/.config/patina/config.toml` for user preferences
  - Editor settings (tab size, soft wrap, auto-save)
  - UI settings (line numbers, minimap, highlight line)
  - Markdown settings (flavor, enable extensions)
  - Keybinding modes (Vim, Emacs, Standard)

### Changed
- Code blocks now display with syntax-aware colors
- Preview pane shows frontmatter in styled box
- Themes apply to both editor and preview panes

### Technical
- Integrated `syntect` for syntax highlighting
- YAML/TOML parsing with `serde_yaml` and `toml` crates
- Theme color conversion between syntect and ratatui

## [0.4.0] - 2026-01-14

### Added
- **Split view** with live markdown preview
  - Horizontal split (editor | preview)
  - Toggle between Raw, Preview, and Split modes (Ctrl+\)
  - Synchronized scrolling between panes
  - Resizable panes
- **Markdown rendering** for preview pane
  - Headers with visual hierarchy (H1-H6)
  - Bold, italic, strikethrough
  - Inline code and code blocks
  - Lists (ordered, unordered, nested)
  - Task lists with checkboxes
  - Blockquotes with visual border
  - Tables with proper formatting
  - Links (with URLs shown)
  - Images (with alt text)
  - Horizontal rules

### Changed
- UI layout now supports multiple view modes
- Preview updates on text changes (debounced)
- Border styles and visual indicators improved

### Technical
- Comrak AST traversal for markdown rendering
- Ratatui widgets for preview display
- Split pane layout management

## [0.3.0] - 2026-01-13

### Added
- **Markdown parsing** using comrak
  - CommonMark support
  - GitHub Flavored Markdown (GFM) extensions
  - Strikethrough, tables, autolinks, task lists
  - Footnotes and description lists
- **Document AST** generation and caching
  - Parse on file load and after edits
  - Cache parsed AST for performance

### Technical
- Integrated `comrak` crate with GFM options
- AST stored in `Document` struct
- Parser module in `patina-core`

## [0.2.0] - 2026-01-12

### Added
- **Core editing functionality**
  - Text insertion and deletion
  - Cursor movement (arrows, Home, End, PgUp, PgDn)
  - Undo/redo with history stack
  - File save (Ctrl+S)
  - Auto-save support (configurable interval)
- **Keyboard shortcuts**
  - Ctrl+Z: Undo
  - Ctrl+Y: Redo
  - Ctrl+S: Save
  - Ctrl+N: New file
  - Ctrl+O: Open file
  - Ctrl+W: Close tab
  - Ctrl+Q: Quit
- **Input handling** for all editing operations
- **Modified flag** tracking for unsaved changes
- **Quit confirmation** when unsaved changes exist

### Changed
- Buffer operations now go through history system
- Cursor stays at valid positions
- Status bar shows unsaved indicator

### Technical
- Implemented `History` struct with command pattern
- `InsertCommand` and `DeleteCommand` for undo/redo
- Event handling in `app.rs`

## [0.1.0] - 2026-01-11

### Added
- **Initial release** of Patina TUI editor
- **Workspace architecture** with 6 crates
  - `patina`: TUI binary
  - `patina-gui`: GUI binary (stub)
  - `patina-core`: Core library (buffer, document, parser)
  - `patina-render`: Rendering backends (TUI + GUI)
  - `patina-extensions`: LaTeX, Mermaid, Emoji (stubs)
  - `patina-i18n`: Internationalization (stub)
- **Text buffer** using ropey for efficient text operations
  - O(log n) insertions and deletions
  - Line-based access
  - Modified flag tracking
- **Document model** combining buffer, metadata, and state
  - File I/O (open, save)
  - Cursor position tracking
  - Scroll offset management
- **Basic TUI** with ratatui and crossterm
  - Terminal initialization and cleanup
  - Status bar with file info
  - Border and layout
  - Quit command (Ctrl+Q)

### Technical
- Cargo workspace with incremental build optimization
- Release profile with LTO, strip, and codegen-units=1
- Platform support: macOS, Linux, Windows (via crossterm)
- Performance targets: <50ms startup, <20MB memory

---

## Version Naming

- **v0.1-0.6**: TUI MVP development
- **v0.7-0.8**: GUI support with egui
- **v0.9+**: Advanced features (Mermaid, full i18n, export)
- **v1.0**: Stable release

## Links

- Repository: https://github.com/yourusername/patina
- Issues: https://github.com/yourusername/patina/issues
- Documentation: https://github.com/yourusername/patina/blob/main/README.md
