# Patina Implementation Plan

> Incremental development plan following the philosophy in CLAUDE.md: compile, test, commit at every stage.

**Last Updated:** 2026-01-14
**Status:** Active Development

---

## Overview

Patina will be built incrementally across multiple versions, with each version being fully functional, tested, and usable. This plan breaks down the MVP into 6 stages (v0.1 through v0.6), followed by the GUI addition (v0.7-v0.8), and advanced features (v0.9+).

**Core Principles:**
- Every version must compile and pass all tests
- Each version adds one cohesive feature set
- Architectural boundaries (core/render/extensions) enforced from the start
- Performance targets validated at each stage
- No "stub" implementations—build complete features incrementally

---

## Version Roadmap

```
v0.1.0 → v0.2.0 → v0.3.0 → v0.4.0 → v0.5.0 → v0.6.0 (MVP Complete)
                                                  ↓
                                              v0.7.0 → v0.8.0 (GUI)
                                                              ↓
                                                          v0.9.0+ (Advanced)
```

| Version | Focus | Key Deliverables | Validation |
|---------|-------|------------------|------------|
| **v0.1.0** | Foundation | Text buffer, basic TUI shell | Opens files, displays text |
| **v0.2.0** | Core Editing | Cursor movement, text insertion/deletion, undo/redo | Can edit and save files |
| **v0.3.0** | Markdown Parsing | ComraK integration, AST generation | Parses all CommonMark |
| **v0.4.0** | Split View | Edit + Preview panes, rendering pipeline | Live preview works |
| **v0.5.0** | Syntax & Frontmatter | Code highlighting, YAML/TOML parsing | Renders code blocks, extracts metadata |
| **v0.6.0** | LaTeX & Polish | Unicode math, config system, keybindings | MVP feature-complete |
| **v0.7.0** | GUI Foundation | egui backend, feature flags | GUI opens files |
| **v0.8.0** | GUI Parity | Full GUI feature set matching TUI | Both TUI and GUI work identically |
| **v0.9.0+** | Advanced | Mermaid, full i18n, export, workspace mode | v1.0 preparation |

---

## Stage-by-Stage Implementation

### v0.1.0: Foundation (Days 1-2)

**Goal:** Establish the workspace architecture and display text from files.

**Success Criteria:**
- [ ] Workspace structure with 6 crates compiles
- [ ] `patina` binary opens a file and displays it in TUI
- [ ] Text buffer uses `ropey::Rope`
- [ ] Basic cursor positioning works
- [ ] `/validate-build` passes

#### Tasks

**Stage 1.1: Workspace Setup**
- Create Cargo workspace with all 6 crates
- Set up `patina-core` with basic modules (buffer, document)
- Set up `patina-render` with feature flags (tui default)
- Set up `patina-extensions`, `patina-i18n` as stubs
- Set up `patina` TUI binary with clap CLI
- Set up `patina-gui` as minimal stub

**Stage 1.2: Text Buffer (`patina-core`)**
- Implement `Buffer` struct wrapping `ropey::Rope`
- Methods: `new()`, `from_str()`, `text()`, `len_lines()`, `len_chars()`
- Implement `modified` flag tracking
- Unit tests for buffer operations
- **Validation:** `cargo test -p patina-core`

**Stage 1.3: Document Model (`patina-core`)**
- Implement `Document` struct
- Fields: buffer, path, cursor, scroll_offset
- Methods: `new()`, `from_file()`, `from_str()`, `save()`
- Integration tests for file I/O
- **Validation:** Can open and save files programmatically

**Stage 1.4: Basic TUI Shell (`patina`)**
- Initialize ratatui terminal
- Render simple UI with status bar
- Display buffer text with cursor
- Handle quit command (Ctrl+Q)
- **Validation:** `cargo run -- test.md` opens and displays file

**Commit:** "feat: v0.1.0 - Foundation with text buffer and basic TUI"

---

### v0.2.0: Core Editing (Days 3-4)

**Goal:** Make the editor functional for basic text editing.

**Success Criteria:**
- [ ] Can insert and delete text
- [ ] Cursor moves correctly (arrow keys, Home/End, PgUp/PgDn)
- [ ] Undo/redo works with history stack
- [ ] Can save files (Ctrl+S)
- [ ] All tests pass

#### Tasks

**Stage 2.1: Cursor Movement**
- Implement cursor movement in `Document`
- Methods: `move_cursor()`, `move_to_line()`, `move_to_char()`
- Handle arrow keys, Home, End, PgUp, PgDn
- Keep cursor in valid positions
- **Validation:** Cursor moves correctly in all cases

**Stage 2.2: Text Insertion/Deletion**
- Implement insert/delete operations in `Buffer`
- Methods: `insert()`, `delete()`, `insert_char()`, `delete_char()`
- Update modified flag
- Handle newlines correctly
- **Validation:** Can type and delete text

**Stage 2.3: History System (`patina-core`)**
- Implement `History` struct with command pattern
- Define `Command` trait with `execute()`, `undo()`
- Implement `InsertCommand`, `DeleteCommand`
- Methods: `push()`, `undo()`, `redo()`
- **Validation:** Undo/redo works for all operations

**Stage 2.4: File Operations**
- Wire up save command (Ctrl+S)
- Handle unsaved changes warning
- Implement auto-save (configurable)
- **Validation:** Can edit, save, and reload files

**Stage 2.5: Input Handling (`patina`)**
- Implement keyboard event handling
- Map keys to editor commands
- Handle Ctrl combinations
- **Validation:** All basic keybindings work

**Commit:** "feat: v0.2.0 - Core editing with undo/redo"

---

### v0.3.0: Markdown Parsing (Day 5)

**Goal:** Parse Markdown into AST for later rendering.

**Success Criteria:**
- [ ] Integrates `comrak` for CommonMark parsing
- [ ] Generates AST from document text
- [ ] Parser module is in `patina-core`
- [ ] No rendering yet—just parsing
- [ ] Tests cover all CommonMark features

#### Tasks

**Stage 3.1: Parser Module (`patina-core`)**
- Create `parser.rs` module
- Implement `MarkdownParser` struct
- Method: `parse(text: &str) -> ComrakDocument`
- Configure comrak with GFM extensions
- **Validation:** Parses sample markdown correctly

**Stage 3.2: Parser Integration**
- Add `ast` field to `Document` (optional)
- Parse on file load and after edits (debounced)
- Cache parsed AST
- **Validation:** AST updates when text changes

**Stage 3.3: Parser Tests**
- Test cases for headers, lists, links, code blocks
- Test GFM features: tables, strikethrough, task lists
- Test edge cases: empty input, malformed markdown
- **Validation:** `cargo test -p patina-core` passes

**Commit:** "feat: v0.3.0 - Markdown parsing with comrak"

---

### v0.4.0: Split View & Rendering (Days 6-7)

**Goal:** Display live preview of rendered Markdown alongside editor.

**Success Criteria:**
- [ ] Split pane layout: edit on left, preview on right
- [ ] Preview renders basic Markdown (no syntax highlighting yet)
- [ ] Rendering is in `patina-render` crate
- [ ] Can toggle split view on/off
- [ ] Scrolling works in both panes

#### Tasks

**Stage 4.1: Rendering Architecture (`patina-render`)**
- Create `tui/mod.rs` with feature gate
- Implement `App` struct for TUI state
- Define `ViewMode` enum: EditOnly, SplitView, PreviewOnly
- **Validation:** Compiles with and without TUI feature

**Stage 4.2: Layout System (`patina-render`)**
- Implement split pane layout in ratatui
- Calculate pane sizes dynamically
- Handle window resize events
- **Validation:** Split view displays correctly

**Stage 4.3: Preview Rendering (`patina-render`)**
- Implement basic Markdown rendering to ratatui widgets
- Render: headers, paragraphs, lists, blockquotes
- Style: bold, italic, inline code
- **Validation:** Preview shows styled Markdown

**Stage 4.4: Editor Widget (`patina-render`)**
- Implement text editor widget
- Show line numbers (optional)
- Highlight current line
- Show cursor position
- **Validation:** Editor pane displays correctly

**Stage 4.5: Scrolling**
- Implement scroll offset tracking
- Synchronize scroll between edit and preview (basic)
- Handle PgUp/PgDn in both panes
- **Validation:** Scrolling works smoothly

**Stage 4.6: UI Integration (`patina`)**
- Wire up split view toggle (Ctrl+\)
- Update status bar with mode indicator
- Handle input routing to active pane
- **Validation:** Split view works end-to-end

**Commit:** "feat: v0.4.0 - Split view with live preview"

---

### v0.5.0: Syntax Highlighting & Frontmatter (Days 8-9)

**Goal:** Add code block syntax highlighting and frontmatter support.

**Success Criteria:**
- [ ] Code blocks have syntax highlighting (syntect)
- [ ] YAML and TOML frontmatter parsed
- [ ] Frontmatter displayed in preview
- [ ] 20+ languages supported
- [ ] Tests pass

#### Tasks

**Stage 5.1: Syntax Highlighting (`patina-core`)**
- Create `syntax.rs` module
- Integrate `syntect` with default theme set
- Implement `highlight_code(lang, code) -> Vec<StyledLine>`
- Support 50+ languages from product spec
- **Validation:** Code blocks are highlighted

**Stage 5.2: Syntax in Rendering (`patina-render`)**
- Update preview renderer to use syntax highlighting
- Render code blocks with colored text
- Add language label to code blocks
- **Validation:** Rust, Python, JS code renders correctly

**Stage 5.3: Frontmatter Parsing (`patina-core`)**
- Create `frontmatter.rs` module
- Implement `Frontmatter::extract(text) -> (Option<Frontmatter>, &str)`
- Support YAML (`---` delimiters) and TOML (`+++` delimiters)
- Parse into serde_json::Value for flexibility
- **Validation:** Extracts frontmatter correctly

**Stage 5.4: Frontmatter Integration**
- Update `Document::from_str()` to extract frontmatter
- Add `frontmatter` field to `Document`
- Display frontmatter in preview (formatted box)
- **Validation:** Frontmatter appears in preview

**Stage 5.5: Theme System (`patina-render`)**
- Create `theme.rs` and `style.rs` modules
- Implement `Theme` enum: Dracula, OneDark, Solarized
- Add color definitions for each theme
- Make theme configurable
- **Validation:** Can switch themes

**Commit:** "feat: v0.5.0 - Syntax highlighting and frontmatter"

---

### v0.6.0: LaTeX & MVP Polish (Days 10-11)

**Goal:** Complete MVP with LaTeX math rendering and polish.

**Success Criteria:**
- [ ] Inline math ($...$) renders to Unicode
- [ ] Display math ($$...$$) renders centered
- [ ] Config system works (~/.config/patina/config.toml)
- [ ] Emoji shortcodes expand
- [ ] i18n framework initialized (English only)
- [ ] All MVP keyboard shortcuts work
- [ ] Performance targets met (<50ms startup, <20MB memory)
- [ ] Documentation complete (README, CHANGELOG)

#### Tasks

**Stage 6.1: LaTeX Extension (`patina-extensions`)**
- Create `latex.rs` module
- Implement `render_latex_inline(expr: &str) -> String`
- Implement `render_latex_display(expr: &str) -> String`
- Handle common LaTeX: superscripts, subscripts, fractions, sqrt, sum
- Fallback to raw text for unsupported LaTeX
- **Validation:** Math expressions render to Unicode

**Stage 6.2: Emoji Extension (`patina-extensions`)**
- Create `emoji.rs` module
- Load emoji shortcode database (GitHub-compatible)
- Implement `expand_shortcodes(text: &str) -> String`
- Support ~1,800 emoji
- **Validation:** `:rocket:` → 🚀

**Stage 6.3: Extension Integration**
- Wire extensions into preview rendering pipeline
- Process LaTeX blocks during preview render
- Process emoji shortcodes during render
- **Validation:** Extensions only run during rendering, not on every keystroke

**Stage 6.4: Config System (`patina`)**
- Create `config.rs` module
- Define `Config` struct matching spec
- Load from `~/.config/patina/config.toml`
- Implement defaults
- **Validation:** Config loads and applies correctly

**Stage 6.5: i18n Framework (`patina-i18n`)**
- Integrate `fluent-rs`
- Create `en/main.ftl` with English strings
- Implement `t!(key)` macro for translations
- Use in UI strings
- **Validation:** UI displays English strings from Fluent

**Stage 6.6: Keybindings (`patina`)**
- Implement all MVP keyboard shortcuts from spec
- File operations: Ctrl+N, Ctrl+O, Ctrl+S, Ctrl+W
- Editing: Ctrl+Z, Ctrl+Y, Ctrl+F
- View: Ctrl+\, Ctrl+Shift+Z (zen mode stub)
- **Validation:** All shortcuts work

**Stage 6.7: Performance Validation**
- Run `/bench-perf` skill
- Optimize startup time to <50ms
- Check memory footprint <20MB
- Profile and fix any hot paths
- **Validation:** Meets all performance targets

**Stage 6.8: Documentation**
- Update README with installation and usage
- Create CHANGELOG.md with v0.1-v0.6 entries
- Add examples/ directory with sample markdown
- Write CONTRIBUTING.md
- **Validation:** Documentation is complete and accurate

**Commit:** "feat: v0.6.0 - MVP complete with LaTeX and polish"

**Release:** Tag v0.6.0 as MVP

---

## Validation at Each Stage

### Automated Checks
After each stage, run:
```bash
/validate-build      # Build, test, clippy, format
/check-arch          # Architectural boundaries
cargo run -- examples/test.md  # Manual smoke test
```

### Stage Sign-off Checklist
- [ ] Code compiles with zero warnings
- [ ] All tests pass (unit + integration)
- [ ] Clippy reports no issues
- [ ] Code is formatted (`cargo fmt`)
- [ ] Architectural boundaries maintained
- [ ] Performance acceptable for stage
- [ ] Feature works end-to-end in TUI
- [ ] Git commit created with clear message

---

## Post-MVP: GUI and Advanced Features

### v0.7.0: GUI Foundation (Week 4)

**Goal:** Add egui backend with basic file viewing.

**Tasks:**
- Implement `patina-render/src/gui/mod.rs` with `#[cfg(feature = "gui")]`
- Create `patina-gui` binary
- Implement basic egui layout: menu bar, editor pane
- Open and display files in GUI
- **Validation:** GUI and TUI both work, share same `patina-core`

### v0.8.0: GUI Parity (Week 5)

**Goal:** GUI matches TUI feature set.

**Tasks:**
- Implement all TUI features in GUI
- Split view in GUI
- Same keybindings
- Same config system
- **Validation:** Feature parity between TUI and GUI

### v0.9.0+: Advanced Features (Weeks 6-12)

Following features from product spec:
- v0.9.0: Mermaid diagram rendering (ASCII for TUI, SVG for GUI)
- v0.10.0: Full i18n with Hindi and 3+ languages
- v0.11.0: Export engine (HTML, PDF)
- v0.12.0: Workspace mode (file tree, multi-file, git status)
- v0.13.0: Advanced editing (multiple cursors, find/replace regex)
- v0.14.0: Zen mode, minimap, document outline
- v1.0.0: Polish and release

---

## Risk Mitigation

### If Behind Schedule
1. Push non-critical features to next version
2. Simplify feature scope (e.g., fewer LaTeX commands)
3. Defer GUI to post-MVP
4. Focus on TUI-only for MVP

### If Performance Issues
1. Profile with `cargo flamegraph`
2. Use benchmarking suite to identify bottlenecks
3. Optimize hot paths (likely: rendering, parsing)
4. Consider incremental parsing

### If Architectural Issues
1. Stop and refactor immediately—don't accumulate debt
2. Run `/check-arch` frequently
3. Code review before merging features
4. Keep `patina-core` UI-agnostic at all costs

---

## Development Guidelines

### Commit Messages
Follow conventional commits:
```
feat(core): add undo/redo history system
fix(render): correct cursor positioning in split view
docs(readme): update installation instructions
test(parser): add edge cases for frontmatter
```

### Branch Strategy
- `main` - stable, tagged versions
- `develop` - integration branch for next version
- `feature/v0.x-feature-name` - feature branches

### Code Review
- Every feature must pass `/validate-build` and `/check-arch`
- No PRs merged with failing tests or clippy warnings
- API changes in `patina-core` require extra scrutiny

---

## Success Metrics

### Technical
- [x] All stages compile and pass tests
- [ ] <50ms startup time maintained throughout
- [ ] <20MB memory footprint throughout
- [ ] Zero clippy warnings
- [ ] >80% test coverage by v0.6.0

### Process
- [ ] One commit per stage (atomic, working code)
- [ ] No broken builds on main
- [ ] No skipped tests
- [ ] All architectural boundaries respected

---

*This plan is a living document. Update after each stage based on learnings.*
