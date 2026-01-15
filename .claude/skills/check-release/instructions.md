# Release Readiness Skill

## Purpose
Validates that the release build (with LTO, panic=abort, strip) compiles and passes all tests.

## Why This Matters

The `Cargo.toml` profile uses aggressive optimizations:
```toml
[profile.release]
lto = true              # Link-time optimization
codegen-units = 1       # Better optimization, slower compile
panic = "abort"         # No unwinding
strip = true            # Remove debug symbols
```

These settings can expose bugs that don't appear in debug builds:
- **LTO** can change inlining behavior
- **panic=abort** breaks code that relies on unwinding
- **codegen-units=1** changes code generation

## Steps to Execute

### 1. Clean Previous Builds
```bash
cargo clean
```

This ensures a fresh build from scratch.

### 2. Build Release Workspace
```bash
cargo build --release --workspace
```

**Check for:**
- Zero compilation errors
- No unexpected warnings about LTO or codegen

**If fails:** Check error output. Common issues:
- LTO exposing undefined behavior
- Missing `Send`/`Sync` bounds
- Inline assembly issues

### 3. Run Release Tests
```bash
cargo test --release --workspace
```

**Why release tests?** Some bugs only appear with optimizations enabled.

**Check for:**
- All tests pass
- No new test failures vs debug mode
- No timing-dependent test failures

### 4. Verify Binary Sizes
```bash
ls -lh target/release/patina
ls -lh target/release/patina-gui 2>/dev/null || echo "GUI binary not built"
```

**Expected:**
- `patina` < 15MB
- Binaries should be stripped (no debug symbols)

### 5. Quick Smoke Test
```bash
# Test that binary actually runs
target/release/patina --version
target/release/patina --help

# Test basic file opening (if test file exists)
echo "# Test" > /tmp/test-patina.md
timeout 5s target/release/patina /tmp/test-patina.md --zen || echo "Manual test needed"
```

**Expected:**
- Binary launches without crashes
- Help text displays correctly
- File opens successfully

## Success Criteria

All steps succeed:
- ✅ Clean build completes
- ✅ Release build succeeds
- ✅ All release tests pass
- ✅ Binary sizes within limits
- ✅ Smoke tests pass

## Common Failure Modes

### 1. LTO Exposes Undefined Behavior

**Symptom:** Code works in debug but crashes in release.

**Fix:**
- Check for uninitialized memory
- Review unsafe blocks
- Look for race conditions
- Use `cargo miri` to detect UB

### 2. Panic=Abort Breaks Code

**Symptom:** Code that uses `catch_unwind` fails.

**Fix:**
- Remove `catch_unwind` usage (not recommended in Rust anyway)
- Change `panic!` to `Result<T, E>` returns
- Use explicit error handling

### 3. Test Timing Issues

**Symptom:** Tests pass in debug but fail in release (or vice versa).

**Fix:**
- Tests should not depend on timing
- Use proper synchronization primitives
- Add explicit waits instead of sleeps

### 4. Binary Too Large

**Symptom:** Binary > 15MB after strip.

**Fix:**
- Run `cargo bloat --release` to find large dependencies
- Check for accidentally bundled resources
- Review dependency tree: `cargo tree --duplicate`
- Consider feature flags to exclude unused code

## Platform-Specific Notes

### macOS
- Binary should be code-signed for distribution
- Test on both Intel and Apple Silicon if possible

### Linux
- Test on both glibc and musl targets if distributing statically
- Check dependencies: `ldd target/release/patina`

### Windows
- Ensure .exe works without runtime dependencies
- Test in Windows Terminal and cmd.exe

## When to Run

- Before tagging releases
- After major dependency updates
- After changing optimization settings
- Weekly in CI/CD pipeline
- When preparing release announcements
