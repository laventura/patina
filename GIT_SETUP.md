# Git Repository Setup

This guide walks through initializing the Patina repository with git.

## Initial Setup

```bash
# Initialize git repository
git init

# Configure git for this project (if not already global)
git config user.name "Atul Acharya"
git config user.email "your-email@example.com"

# Add remote (replace with your actual repo URL)
git remote add origin https://github.com/laventura/patina.git
```

## What Gets Committed

### ✅ Should be committed (tracked by git):

**Documentation**
- `README.md`
- `CLAUDE.md` ← **Yes, commit this!**
- `IMPLEMENTATION_PLAN.md`
- `PROJECT_STATUS.md`
- `CHANGELOG.md`
- `CONTRIBUTING.md`
- All files in `specs/`

**Code & Configuration**
- All `*.rs` source files
- All `Cargo.toml` files
- `Cargo.lock` ← **Yes, for binary projects**
- `.gitignore` itself

**Project Infrastructure**
- `.claude/skills/` ← **All skills are project-specific**
- GitHub workflows (when added)
- CI/CD configuration

**Examples & Tests**
- `examples/*.md`
- Test files in `tests/`

### ❌ Should NOT be committed (in .gitignore):

**Build Artifacts**
- `/target/`
- `debug/`, `release/`
- Compiled binaries

**Personal Settings**
- `.claude/settings.local.json` ← Personal Claude Code preferences
- `.vscode/` ← Personal IDE settings
- `.idea/` ← Personal IDE settings

**OS Files**
- `.DS_Store` (macOS)
- `Thumbs.db` (Windows)

**Secrets**
- `.env` files
- API keys, credentials

**Generated Files**
- Profiling output
- Coverage reports
- Generated documentation

## Initial Commit Steps

```bash
# 1. Check what will be added
git status

# 2. Add planning documents
git add CLAUDE.md IMPLEMENTATION_PLAN.md PROJECT_STATUS.md
git add README.md CHANGELOG.md Cargo.toml .gitignore
git add specs/ .claude/skills/

# 3. Create initial commit
git commit -m "docs: Initial project planning and architecture

- Add CLAUDE.md with architecture and development guidelines
- Add IMPLEMENTATION_PLAN.md with incremental versioning strategy
- Add detailed specs for v0.1.0 through v0.6.0 (MVP)
- Add 6 custom Claude Code skills for quality assurance
- Add comprehensive .gitignore for Rust project

Planning phase complete, ready for v0.1.0 implementation."

# 4. Create main branch (if needed)
git branch -M main

# 5. Push to remote (when ready)
git push -u origin main
```

## Branching Strategy

### Branch Names

```
main              # Stable, tagged releases
develop           # Integration branch for next version
feature/v0.1-foundation       # Feature branches
feature/v0.2-editing
fix/buffer-overflow
docs/update-readme
```

### Workflow

1. **Create feature branch from develop:**
   ```bash
   git checkout develop
   git checkout -b feature/v0.1-foundation
   ```

2. **Work on feature with atomic commits:**
   ```bash
   git add patina-core/src/buffer.rs
   git commit -m "feat(core): implement rope-based text buffer"

   git add patina-core/src/document.rs
   git commit -m "feat(core): add document model with file I/O"
   ```

3. **Merge to develop when complete:**
   ```bash
   git checkout develop
   git merge --no-ff feature/v0.1-foundation
   git branch -d feature/v0.1-foundation
   ```

4. **Merge to main and tag release:**
   ```bash
   git checkout main
   git merge --no-ff develop
   git tag -a v0.1.0 -m "Release v0.1.0: Foundation"
   git push --tags
   ```

## Commit Message Convention

Follow [Conventional Commits](https://www.conventionalcommits.org/):

```
<type>(<scope>): <subject>

<body>

<footer>
```

### Types

- `feat` - New feature
- `fix` - Bug fix
- `docs` - Documentation only
- `style` - Formatting, no code change
- `refactor` - Code restructuring
- `perf` - Performance improvement
- `test` - Adding tests
- `chore` - Maintenance tasks

### Scopes

- `core` - patina-core crate
- `render` - patina-render crate
- `extensions` - patina-extensions crate
- `i18n` - patina-i18n crate
- `tui` - TUI binary
- `gui` - GUI binary
- `deps` - Dependencies

### Examples

```bash
git commit -m "feat(core): add undo/redo history system"

git commit -m "fix(render): correct cursor positioning in split view"

git commit -m "docs(readme): update installation instructions"

git commit -m "perf(core): optimize rope insertion performance

Replaced linear scan with binary search for insertion point.
Reduces insertion time from O(n) to O(log n) for large buffers.

Closes #42"
```

## Pre-commit Hooks (Optional)

Create `.git/hooks/pre-commit`:

```bash
#!/bin/bash

# Run validation before commit
echo "Running pre-commit checks..."

# Format check
cargo fmt --check || {
    echo "❌ Code not formatted. Run: cargo fmt"
    exit 1
}

# Clippy check
cargo clippy --workspace -- -D warnings || {
    echo "❌ Clippy errors found"
    exit 1
}

# Tests
cargo test --workspace || {
    echo "❌ Tests failed"
    exit 1
}

echo "✅ All pre-commit checks passed"
```

Make it executable:
```bash
chmod +x .git/hooks/pre-commit
```

## Tagging Releases

When completing a version:

```bash
# Tag with annotation
git tag -a v0.1.0 -m "Release v0.1.0: Foundation

Implemented:
- Rope-based text buffer
- Basic TUI shell with ratatui
- File opening and display
- Event loop with Ctrl+Q to quit

Performance:
- Startup time: 18ms
- Memory footprint: 8MB
- Binary size: 4.2MB"

# Push tag
git push origin v0.1.0

# Or push all tags
git push --tags
```

## Checking What's Tracked

```bash
# See what's tracked by git
git ls-files

# See what's ignored
git status --ignored

# Check if a specific file would be ignored
git check-ignore -v .DS_Store
```

## Cleaning Up

If you accidentally committed something that should be ignored:

```bash
# Remove from git but keep locally
git rm --cached .DS_Store
git rm --cached -r target/

# Commit the removal
git commit -m "chore: remove ignored files from tracking"
```

## First Push to GitHub

```bash
# Create repo on GitHub first, then:
git remote add origin https://github.com/laventura/patina.git
git branch -M main
git push -u origin main
git push --tags
```

---

## Quick Reference

```bash
# Status check
git status

# What would be committed
git diff --staged

# Commit with validation
cargo test && cargo clippy && git commit

# Push current branch
git push

# Create and switch to branch
git checkout -b feature/new-feature

# Merge feature to develop
git checkout develop && git merge feature/new-feature

# Tag release
git tag -a v0.1.0 -m "Release v0.1.0"
```

---

**Ready to initialize git!** Run the commands in "Initial Setup" to begin.
