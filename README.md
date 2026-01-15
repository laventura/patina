# Patina

> A fast, lightweight Markdown editor written in pure Rust

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)

## Overview

Patina is a terminal-based Markdown editor designed for speed and simplicity. It supports modern Markdown features including:

- **GitHub Flavored Markdown** (tables, task lists, strikethrough)
- **YAML/TOML frontmatter**
- **LaTeX math** (rendered as Unicode)
- **Mermaid diagrams** (rendered as ASCII art)
- **Emoji shortcodes** (`:rocket:` → 🚀)
- **Syntax highlighting** (50+ languages)

## Features

- ⚡ **Fast** — <50ms startup, instant response
- 🪶 **Lightweight** — Single binary, no dependencies
- 🖥️ **TUI** — Works in any terminal, over SSH
- 🎨 **Themes** — Dracula, One Dark, Solarized, and more
- 🌍 **i18n** — Multiple language support
- 🔧 **Configurable** — Vim/Emacs/Standard keybindings

## Installation

### From Source

```bash
# Clone the repository
git clone https://github.com/yourusername/patina.git
cd patina

# Build release version
cargo build --release

# Install to ~/.cargo/bin
cargo install --path crates/patina
```

### Pre-built Binaries

Coming soon!

## Usage

```bash
# Open a file
patina document.md

# Open multiple files
patina file1.md file2.md

# Open a workspace
patina --workspace ./my-project/

# Start in Zen mode
patina --zen document.md
```

## Keyboard Shortcuts

| Shortcut | Action |
|----------|--------|
| `Ctrl+S` | Save |
| `Ctrl+Q` | Quit |
| `Ctrl+N` | New file |
| `Ctrl+O` | Open file |
| `Ctrl+W` | Close tab |
| `Ctrl+Tab` | Next tab |
| `Ctrl+\` | Toggle split view |
| `Ctrl+Shift+Z` | Toggle Zen mode |

## Project Structure

```
patina/
├── crates/
│   ├── patina/           # TUI binary
│   ├── patina-gui/       # GUI binary (egui) - v0.5
│   ├── patina-core/      # Core library
│   ├── patina-render/    # Rendering backends
│   ├── patina-extensions/# LaTeX, Mermaid, Emoji
│   └── patina-i18n/      # Internationalization
├── Cargo.toml            # Workspace manifest
└── README.md
```

## Development

```bash
# Run in development mode
cargo run -- test.md

# Run tests
cargo test --workspace

# Check formatting
cargo fmt --check

# Run lints
cargo clippy --workspace
```

## Roadmap

- [x] MVP - Basic TUI editor with split preview
- [ ] v0.5 - GUI version (egui)
- [ ] v1.0 - Mermaid diagrams, full i18n
- [ ] v2.0 - Plugin system, LSP integration

## License

MIT License - see [LICENSE](LICENSE) for details.

## Acknowledgments

- [ratatui](https://github.com/ratatui-org/ratatui) - TUI framework
- [egui](https://github.com/emilk/egui) - GUI framework
- [comrak](https://github.com/kivikakk/comrak) - Markdown parser
- [syntect](https://github.com/trishume/syntect) - Syntax highlighting
- [Ferrite](https://github.com/OlaProeis/Ferrite) - Inspiration for features
