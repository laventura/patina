# Patina Specifications

This directory contains all specification documents for Patina development.

## Documents

### Product Specification
- **[patina-product-spec.md](patina-product-spec.md)** - Complete product specification with vision, features, architecture, and roadmap

### Implementation Planning
- **[../IMPLEMENTATION_PLAN.md](../IMPLEMENTATION_PLAN.md)** - Stage-by-stage implementation plan with validation criteria

### Version Specifications
Detailed technical specifications for each version:

- **[v0.1.0-foundation.md](versions/v0.1.0-foundation.md)** - Workspace setup, text buffer, basic TUI
- **[v0.2.0-editing.md](versions/v0.2.0-editing.md)** - Cursor movement, text editing, undo/redo
- **[v0.3.0-parsing.md](versions/v0.3.0-parsing.md)** - Markdown parsing with comrak
- **[v0.4.0-preview.md](versions/v0.4.0-preview.md)** - Split view with live preview
- **[v0.5.0-syntax.md](versions/v0.5.0-syntax.md)** - Syntax highlighting and frontmatter
- **[v0.6.0-mvp.md](versions/v0.6.0-mvp.md)** - LaTeX, emoji, config, i18n, MVP complete

## Development Flow

```
Read Product Spec
    ↓
Review Implementation Plan
    ↓
Start with v0.1.0 Spec
    ↓
Implement → Test → Validate
    ↓
Commit and tag release
    ↓
Move to next version
```

## Validation Tools

Use the custom Claude Code skills in `/.claude/skills/`:
- `/validate-build` - Build, test, clippy, format checks
- `/check-arch` - Verify architectural boundaries
- `/bench-perf` - Performance benchmarking
- `/check-release` - Release build validation
- `/review-api` - API quality review
- `/check-features` - Feature flag testing

## Version Status

| Version | Status | Target |
|---------|--------|--------|
| v0.1.0 | 📋 Planned | Days 1-2 |
| v0.2.0 | 📋 Planned | Days 3-4 |
| v0.3.0 | 📋 Planned | Day 5 |
| v0.4.0 | 📋 Planned | Days 6-7 |
| v0.5.0 | 📋 Planned | Days 8-9 |
| v0.6.0 | 📋 Planned | Days 10-11 |

Legend:
- 📋 Planned
- 🚧 In Progress
- ✅ Complete

## Contributing

When updating specifications:

1. Keep product spec as the source of truth for features
2. Update implementation plan when changing approach
3. Keep version specs in sync with implementation
4. Document all architectural decisions
5. Update this README with status changes

## Questions?

See [CLAUDE.md](../CLAUDE.md) for development guidelines and architecture overview.
