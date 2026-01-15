# Patina Claude Code Skills

Custom skills for maintaining code quality and architectural integrity in the Patina markdown editor project.

## Available Skills

### `/validate-build` - Build Validator
**Priority:** ⭐⭐⭐ Must-run before every commit

Validates workspace integrity by running:
- `cargo build --workspace`
- `cargo test --workspace`
- `cargo clippy --workspace -- -D warnings`
- `cargo fmt --check`

**When to use:** After any code change, before commits, in pre-commit hooks.

---

### `/bench-perf` - Performance Benchmark
**Priority:** ⭐⭐⭐ Critical for maintaining performance targets

Measures performance against project targets:
- Cold start time: < 50ms
- Memory footprint: < 20MB
- Binary size: < 15MB

**When to use:** After optimization changes, before releases, weekly checks.

---

### `/check-arch` - Architecture Guard
**Priority:** ⭐⭐⭐ Must-run for architectural changes

Enforces architectural boundaries:
- `patina-core` stays UI-agnostic (no ratatui/egui)
- Feature flags used correctly in `patina-render`
- No circular dependencies
- Buffer modifications use History system
- Extensions only run at render time

**When to use:** Before committing architectural changes, during refactoring, in code reviews.

---

### `/check-release` - Release Readiness
**Priority:** ⭐⭐ Important before releases

Validates release build with aggressive optimizations:
- LTO enabled
- panic=abort
- strip=true
- All tests pass in release mode

**When to use:** Before tagging releases, after dependency updates, weekly CI checks.

---

### `/review-api` - API Reviewer
**Priority:** ⭐⭐ Important for library crates

Reviews public API changes in `patina-core` and `patina-render`:
- Documentation completeness
- Proper visibility (pub vs pub(crate))
- Breaking changes detection
- No type leakage across boundaries

**When to use:** After API changes, before releases, during code reviews, monthly maintenance.

---

### `/check-features` - Feature Flag Validator
**Priority:** ⭐ Nice-to-have, less frequent

Tests all feature flag combinations:
- TUI only
- GUI only
- Both TUI and GUI
- No features (edge case)

**When to use:** After modifying features, before releases, when adding dependencies.

---

## Recommended Workflow

### Daily Development
```bash
# After making changes
/validate-build

# If touching core or render APIs
/check-arch
```

### Pre-Commit
```bash
/validate-build
```

### Weekly Maintenance
```bash
/validate-build
/bench-perf
/check-arch
```

### Before Release
```bash
/validate-build
/check-arch
/check-release
/bench-perf
/review-api
/check-features
```

## Integration with CI/CD

These skills can be automated in GitHub Actions:

```yaml
# .github/workflows/ci.yml
name: CI

on: [push, pull_request]

jobs:
  validate:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable

      # Equivalent to /validate-build
      - name: Build
        run: cargo build --workspace
      - name: Test
        run: cargo test --workspace
      - name: Clippy
        run: cargo clippy --workspace -- -D warnings
      - name: Format
        run: cargo fmt --check

      # Equivalent to /check-release
      - name: Release build
        run: cargo build --release --workspace
      - name: Release tests
        run: cargo test --release --workspace
```

## Skill Maintenance

These skills are living documents. Update them when:
- Project structure changes
- New architectural rules are added
- Performance targets change
- New features are added to Cargo.toml

## Contributing

When modifying skills:
1. Update both `skill.json` (metadata) and `instructions.md` (detailed steps)
2. Test the skill manually before committing
3. Update this README if adding new skills
4. Keep instructions actionable and specific
