# Feature Flag Validator Skill

## Purpose
Ensures all feature flag combinations compile correctly to prevent feature-gating bugs.

## Why This Matters

`patina-render` uses feature flags for optional backends:
```toml
[features]
default = ["tui"]
tui = ["ratatui", "crossterm"]
gui = ["egui", "eframe"]
```

Feature flag bugs are easy to introduce:
- Code works with default features but breaks without them
- Features conflict with each other
- Conditional compilation (`#[cfg(feature = "...")]`) has typos
- Missing feature gates on platform-specific code

## Feature Combinations to Test

| Combination | Use Case | Command |
|-------------|----------|---------|
| Default (tui) | Normal TUI build | `cargo build -p patina-render` |
| TUI only | Explicit TUI | `cargo build -p patina-render --no-default-features --features tui` |
| GUI only | Desktop GUI | `cargo build -p patina-render --no-default-features --features gui` |
| Both | Development/testing | `cargo build -p patina-render --features tui,gui` |
| Neither | Should fail gracefully | `cargo build -p patina-render --no-default-features` |

## Steps to Execute

### 1. Test Default Features
```bash
cargo build -p patina-render
```

**Expected:** Builds successfully with TUI support.

### 2. Test TUI Feature Only
```bash
cargo build -p patina-render --no-default-features --features tui
```

**Expected:** Builds successfully with only TUI support.

**Check:**
- No egui code is compiled
- All TUI code is available

### 3. Test GUI Feature Only
```bash
cargo build -p patina-render --no-default-features --features gui
```

**Expected:** Builds successfully with only GUI support.

**Check:**
- No ratatui code is compiled
- All GUI code is available

### 4. Test Both Features
```bash
cargo build -p patina-render --features tui,gui
```

**Expected:** Builds successfully with both backends.

**Check:**
- Both TUI and GUI code compile together
- No feature conflicts
- Binary size is larger (includes both backends)

### 5. Test No Features (Edge Case)
```bash
cargo build -p patina-render --no-default-features 2>&1 | tee /tmp/no-features.log
```

**Expected:** Either:
- Builds successfully (with no rendering backends)
- Fails with helpful error message

**If it builds:** Check that it doesn't accidentally include backend code.

**If it fails:** Error message should be clear about missing features.

### 6. Test Downstream Dependents
```bash
# Test that patina TUI binary works with render features
cargo build -p patina --no-default-features
cargo build -p patina

# Test that patina-gui works (when implemented)
cargo build -p patina-gui --no-default-features 2>/dev/null || echo "GUI not yet implemented"
```

**Expected:** Binaries build with their required features.

### 7. Check Feature Documentation
```bash
# Ensure features are documented
cargo doc -p patina-render --open --no-deps
```

**Check in docs:**
- Features section lists `tui` and `gui`
- Each feature is documented with purpose
- Examples show how to use each feature

## Success Criteria

All builds succeed with appropriate feature combinations:
- ✅ Default features build
- ✅ TUI-only builds
- ✅ GUI-only builds
- ✅ Both features build together
- ✅ No-features handled gracefully
- ✅ Downstream crates build correctly
- ✅ Features are documented

## Common Issues and Fixes

### 1. Missing Feature Gate

**Symptom:** Build fails when feature is disabled.

**Example:**
```rust
// Missing #[cfg]
use ratatui::Terminal;  // ❌ Breaks when tui feature disabled
```

**Fix:**
```rust
#[cfg(feature = "tui")]
use ratatui::Terminal;  // ✅
```

### 2. Incorrect Feature Name

**Symptom:** Feature gate doesn't work.

**Example:**
```rust
#[cfg(feature = "terminal")]  // ❌ Typo!
use ratatui::Terminal;
```

**Fix:**
```rust
#[cfg(feature = "tui")]  // ✅ Matches Cargo.toml
use ratatui::Terminal;
```

### 3. Feature Conflict

**Symptom:** Can't enable both features simultaneously.

**Example:**
```rust
// Code assumes only one feature is active
#[cfg(feature = "tui")]
pub type Backend = TuiBackend;

#[cfg(feature = "gui")]
pub type Backend = GuiBackend;  // ❌ Conflicts if both enabled
```

**Fix:**
```rust
// Support both simultaneously
pub enum Backend {
    #[cfg(feature = "tui")]
    Tui(TuiBackend),

    #[cfg(feature = "gui")]
    Gui(GuiBackend),
}
```

### 4. Transitive Dependencies Not Gated

**Symptom:** Optional dependencies leak through.

**Fix in Cargo.toml:**
```toml
[dependencies]
# BAD - always included
ratatui = "0.28"

# GOOD - only included with feature
ratatui = { version = "0.28", optional = true }
```

### 5. Tests Fail Without Features

**Symptom:** `cargo test` fails when features disabled.

**Fix:**
```rust
#[cfg(test)]
mod tests {
    #[test]
    #[cfg(feature = "tui")]  // ✅ Only run test if feature enabled
    fn test_tui_rendering() {
        // ...
    }
}
```

## Advanced Checks

### Check for Unused Features
```bash
# Find code not behind any feature gate
cargo +nightly udeps -p patina-render
```

### Minimize Feature Combinations
Use `cargo-hack` to test all combinations:
```bash
cargo install cargo-hack
cargo hack check --feature-powerset -p patina-render
```

This tests: `[]`, `[tui]`, `[gui]`, `[tui, gui]`

## When to Run

- After modifying feature gates
- After adding new features
- Before releases
- When adding new dependencies
- Weekly in CI pipeline

## CI Integration

Add to `.github/workflows/ci.yml`:
```yaml
- name: Check feature combinations
  run: |
    cargo build -p patina-render --no-default-features --features tui
    cargo build -p patina-render --no-default-features --features gui
    cargo build -p patina-render --features tui,gui
```
