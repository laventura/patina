# Architecture Guard Skill

## Purpose
Enforces the architectural boundaries documented in CLAUDE.md to prevent technical debt.

## Architecture Rules

### Rule 1: Core Must Be UI-Agnostic
`patina-core` must have ZERO dependencies on:
- `ratatui` or `crossterm` (TUI)
- `egui` or `eframe` (GUI)
- Any rendering-specific code

**Rationale:** Core powers both TUI and GUI. Rendering dependencies break this separation.

### Rule 2: Proper Feature Flag Usage
`patina-render` must:
- Use `#[cfg(feature = "tui")]` for ratatui code
- Use `#[cfg(feature = "gui")]` for egui code
- Not mix TUI/GUI-specific code in non-feature-gated modules

### Rule 3: No Circular Dependencies
The dependency graph must flow one direction:
```
patina (TUI binary)
    ↓
patina-render [features="tui"]
    ↓
patina-core ← patina-extensions
    ↓           ↓
patina-i18n
```

### Rule 4: Buffer Modifications Use History
All buffer modifications that should be undoable must:
- Go through `History::execute()` or similar
- Implement the Command pattern
- Not directly mutate `buffer.rope`

### Rule 5: Extensions Run at Render Time
Extensions (LaTeX, Mermaid, Emoji) should:
- Only run during preview rendering
- NOT run on every keystroke
- Be opt-in via configuration

## Steps to Execute

### 1. Check Core Dependencies
```bash
# Check patina-core Cargo.toml
grep -E "(ratatui|crossterm|egui|eframe)" crates/patina-core/Cargo.toml
```

**Expected:** No matches found.

**If found:** Remove the dependency. Core must stay UI-agnostic.

### 2. Check Core Source Code
```bash
# Search for rendering imports in core
grep -r "use ratatui" crates/patina-core/src/
grep -r "use egui" crates/patina-core/src/
grep -r "use crossterm" crates/patina-core/src/
```

**Expected:** No matches found.

**If found:** Remove the import. Move rendering logic to `patina-render`.

### 3. Verify Feature Flags in patina-render
```bash
# Check that TUI/GUI code is properly feature-gated
grep -A5 "use ratatui" crates/patina-render/src/ | grep -v "#\[cfg(feature"
grep -A5 "use egui" crates/patina-render/src/ | grep -v "#\[cfg(feature"
```

**Expected:** All ratatui/egui usage should be after `#[cfg(feature = "...")]`.

**If found:** Add proper feature gates.

### 4. Check Dependency Graph
```bash
# Visualize dependency tree
cargo tree -p patina-core
```

**Check for:**
- No rendering crates in the tree
- No circular dependencies (crate appearing multiple times in different branches)

### 5. Review Buffer Modifications
```bash
# Find direct rope mutations
grep -r "rope\." crates/patina-core/src/buffer.rs
```

**Expected:** All mutations should be in `buffer.rs` methods, not scattered throughout codebase.

**Manual check:** Review recent git diff for changes to buffer operations. Ensure they:
1. Are methods on `Buffer` struct
2. Update `modified` flag
3. Create `History::Command` if undoable

### 6. Check Extension Call Sites
```bash
# Find where extensions are called
grep -r "latex::" crates/patina/src/ crates/patina-render/src/
grep -r "mermaid::" crates/patina/src/ crates/patina-render/src/
grep -r "emoji::" crates/patina/src/ crates/patina-render/src/
```

**Expected:** Calls should be in:
- `patina-render/src/tui/widgets.rs` (preview rendering)
- `patina-render/src/gui/mod.rs` (preview rendering)
- NOT in `patina/src/input.rs` (event handling)

## Success Criteria

All checks pass:
- ✅ No rendering deps in `patina-core`
- ✅ Feature flags properly used in `patina-render`
- ✅ Dependency graph is acyclic and follows design
- ✅ Buffer modifications follow Command pattern
- ✅ Extensions only called during rendering

## On Failure

### Core Has Rendering Dependencies
**Fix:** Move the code to `patina-render` and expose a trait in `patina-core` if needed.

Example:
```rust
// In patina-core (BAD)
use ratatui::style::Color;  // ❌

// In patina-core (GOOD)
pub trait Colorable {
    fn to_rgb(&self) -> (u8, u8, u8);
}

// In patina-render (GOOD)
impl Colorable for Color { ... }  // ✅
```

### Feature Flags Missing
**Fix:** Add feature gate:
```rust
#[cfg(feature = "tui")]
use ratatui::Terminal;
```

### Circular Dependency
**Fix:** Refactor to extract common code into a lower-level crate.

### Direct Buffer Mutations
**Fix:** Create a method on `Buffer` and use the History system:
```rust
// BAD
doc.buffer.rope.insert(pos, text);  // ❌

// GOOD
doc.buffer.insert(pos, text);  // ✅
doc.history.push(InsertCommand { ... });
```

### Extensions Called Too Often
**Fix:** Move extension calls from input handlers to render functions.

## When to Run

- Before committing architectural changes
- When adding new dependencies
- When modifying core APIs
- During code reviews
- Weekly as part of maintenance
