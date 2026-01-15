# Build Validator Skill

## Purpose
Validates that all code changes maintain workspace integrity by running the complete build pipeline.

## What This Skill Does

Runs the following checks in sequence:

1. **Build** - Ensures all crates compile
2. **Test** - Runs all workspace tests
3. **Clippy** - Checks for common mistakes and anti-patterns
4. **Format** - Verifies code formatting consistency

## Usage

Run this skill:
- After making any code changes
- Before creating commits
- As part of pre-commit validation

## Steps to Execute

1. Run full workspace build:
   ```bash
   cargo build --workspace
   ```

2. Run all tests:
   ```bash
   cargo test --workspace
   ```

3. Run clippy with deny warnings:
   ```bash
   cargo clippy --workspace -- -D warnings
   ```

4. Check code formatting:
   ```bash
   cargo fmt --check
   ```

## Success Criteria

All four commands must succeed with:
- Zero compilation errors
- All tests passing
- No clippy warnings
- No formatting issues

## On Failure

If any step fails:
1. Report which step failed
2. Show the error output
3. Suggest fixes based on the error type:
   - Build errors: Check for missing dependencies or type mismatches
   - Test failures: Run `cargo test -- --nocapture` for details
   - Clippy warnings: Address each warning or add `#[allow(...)]` with justification
   - Format issues: Run `cargo fmt` to fix automatically

## Performance Note

This skill may take 30-60 seconds on first run. Subsequent runs with warm cache are faster.
