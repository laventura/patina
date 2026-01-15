# Cross-Crate API Reviewer Skill

## Purpose
Reviews public API changes in library crates (`patina-core`, `patina-render`) to maintain quality and prevent breaking changes.

## Why This Matters

`patina-core` and `patina-render` are the API boundaries:
- **patina-core** is used by both TUI and GUI binaries
- **patina-render** is used by both binaries with different features
- Breaking changes cascade through all dependents

## What This Skill Reviews

### 1. Public Item Documentation
All public items must have doc comments:
```rust
/// Creates a new buffer from text.
///
/// # Arguments
/// * `text` - The initial text content
///
/// # Example
/// ```
/// let buffer = Buffer::from_str("Hello");
/// ```
pub fn from_str(text: &str) -> Self { ... }
```

### 2. Visibility Levels
Items should use appropriate visibility:
- `pub` - Part of public API
- `pub(crate)` - Used across modules in same crate
- `pub(super)` - Used in parent module only
- (no modifier) - Private to module

### 3. Breaking Changes
Changes that break compatibility:
- Removing public items
- Changing function signatures
- Changing struct fields from public to private
- Renaming public items
- Changing trait bounds

### 4. API Separation
- Core types in `patina-core`
- Rendering types in `patina-render`
- No leakage between boundaries

## Steps to Execute

### 1. Review Recent Changes
```bash
# Show recent commits to core/render
git log --oneline -10 crates/patina-core/ crates/patina-render/
```

### 2. Check for Undocumented Public APIs
```bash
# Build with warnings on missing docs
RUSTDOCFLAGS="-D warnings" cargo doc -p patina-core --no-deps
RUSTDOCFLAGS="-D warnings" cargo doc -p patina-render --no-deps
```

**Expected:** No warnings about missing documentation.

**If warnings:** Add doc comments to all public items.

### 3. Review Public API Surface
```bash
# List all public items in patina-core
cargo doc -p patina-core --open
```

**Manually review:**
- Does each public item need to be public?
- Could some items be `pub(crate)` instead?
- Are implementation details leaking?

### 4. Check for Breaking Changes
```bash
# Compare API with previous version
cargo install cargo-public-api
cargo public-api -p patina-core diff
cargo public-api -p patina-render diff
```

**If breaking changes found:**
- Document them in CHANGELOG.md
- Consider deprecation instead of removal
- Update version number (semver: major bump)

### 5. Verify Examples Compile
```bash
# Run doc tests
cargo test --doc -p patina-core
cargo test --doc -p patina-render
```

**Expected:** All examples in documentation compile and run.

### 6. Check Type Leakage
```bash
# Search for ratatui/egui types in patina-core public API
grep -r "pub.*ratatui" crates/patina-core/src/
grep -r "pub.*egui" crates/patina-core/src/
grep -r "pub fn.*Terminal" crates/patina-core/src/
```

**Expected:** No rendering types in core's public API.

**If found:** Wrap in abstraction or move to `patina-render`.

## Review Checklist

For each changed public item:

- [ ] Has documentation comment (`///`)
- [ ] Documentation includes purpose and examples
- [ ] Uses appropriate visibility (`pub`, `pub(crate)`, etc.)
- [ ] No implementation details exposed
- [ ] Breaking changes are documented
- [ ] Examples in docs compile
- [ ] No rendering types in `patina-core`
- [ ] Feature flags are properly used

## Common Issues and Fixes

### 1. Missing Documentation

**Bad:**
```rust
pub struct Buffer {
    rope: Rope,
}
```

**Good:**
```rust
/// A text buffer backed by a rope data structure.
///
/// Provides efficient text editing operations with O(log n) complexity
/// for insertions and deletions.
pub struct Buffer {
    rope: Rope,
}
```

### 2. Wrong Visibility

**Bad:**
```rust
// Used only within patina-core but marked pub
pub struct InternalCache { ... }
```

**Good:**
```rust
// Internal implementation detail
pub(crate) struct InternalCache { ... }
```

### 3. Leaking Implementation Details

**Bad:**
```rust
// Exposes ropey::Rope in public API
pub fn get_rope(&self) -> &Rope { ... }
```

**Good:**
```rust
// Returns generic text representation
pub fn text(&self) -> String { ... }
```

### 4. Type Leakage Across Boundaries

**Bad (in patina-core):**
```rust
// ratatui type in core!
pub fn to_color(&self) -> ratatui::style::Color { ... }
```

**Good:**
```rust
// Core defines own Color type
pub fn to_rgb(&self) -> (u8, u8, u8) { ... }

// Render layer does conversion
impl From<CoreColor> for ratatui::style::Color { ... }
```

## When to Run

- After adding new public APIs
- Before version releases
- During code review process
- When refactoring core types
- Monthly as maintenance

## Automation Notes

Consider adding to CI:
```bash
# In .github/workflows/ci.yml
- name: Check API docs
  run: RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps
```
