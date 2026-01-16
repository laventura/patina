# Testing Guide for Patina

Quick guide for testing all implemented features.

## Testing Scrolling

### Line-by-Line Scrolling (Arrow Keys)
```bash
cargo run --bin patina test-scrolling.md
```
- Use **Down arrow** repeatedly to scroll down through the 103 lines
- View should scroll automatically to keep cursor visible
- Use **Up arrow** to scroll back up
- Cursor should always be visible on screen

### Page Scrolling (PageUp/PageDown)

**On Mac keyboards without dedicated PageUp/PageDown:**
- **PageDown:** `Fn + Down Arrow`
- **PageUp:** `Fn + Up Arrow`

**On keyboards with dedicated keys:**
- Just press **PageUp** or **PageDown**

**Expected behavior:**
- PageDown: Jump down by ~terminal height lines
- PageUp: Jump up by ~terminal height lines
- View scrolls along with cursor
- Dynamic page size based on terminal height

```bash
# Test with the scrolling document
cargo run --bin patina test-scrolling.md

# Once open:
# 1. Press Fn+Down (or PageDown) - should jump ~20-30 lines down
# 2. Press Fn+Up (or PageUp) - should jump back up
# 3. Repeat to navigate through entire document
```

## Testing Tab Switching

### With Multiple Files
```bash
cargo run --bin patina test.md test-scrolling.md VALIDATION.md
```

### Keybindings to Test

| Keys | Action | Should Work |
|------|--------|-------------|
| **Alt + Right** | Next tab | ✅ Works in all terminals (RECOMMENDED) |
| **Alt + Left** | Previous tab | ✅ Works in all terminals (RECOMMENDED) |
| Ctrl + Tab | Next tab | ⚠️ May not work (terminal captures) |
| Ctrl + Shift + Tab | Previous tab | ⚠️ May not work (terminal captures) |

**What to test:**
1. Open 3 files as shown above
2. Try **Alt + Right** - should switch to next tab
3. Try **Alt + Left** - should switch to previous tab
4. Verify you can cycle through all open files
5. Modified indicator (●) should show correctly per file
6. Verify no extra "Untitled" buffer is present when opening files

## Testing Save Messages

### Save Existing File
```bash
cargo run --bin patina test.md
```
1. Make some changes to the text
2. Press **Ctrl + S**
3. Should see **green** message: "✓ Saved: test.md"
4. Message should show the actual filename

### Save Untitled Document
```bash
cargo run --bin patina
```
1. Type some text
2. Press **Ctrl + S**
3. Should prompt: "Save as: "
4. Type a filename (e.g., "my-doc.md")
5. Press **Enter**
6. Should see **green** message: "✓ Saved: my-doc.md"

### Error Messages
Try saving to a read-only location to see **red** error messages:
```bash
# Make a test file read-only
touch readonly.md
chmod 444 readonly.md
cargo run --bin patina readonly.md

# Make changes and try to save
# Should see red "✗ Error saving file: ..." message
```

## Testing Cursor Movement

### Basic Navigation
```bash
cargo run --bin patina test.md
```

**Arrow Keys:**
- Up/Down: Move line by line
- Left/Right: Move character by character
- Left at start of line: Wrap to end of previous line
- Right at end of line: Wrap to start of next line

**Home/End:**
- **Home**: Jump to start of current line
- **End**: Jump to end of current line

### Column Clamping
1. Navigate to a long line (use End to go to end)
2. Press Down to a shorter line
3. Column should clamp to the shorter line's length
4. Press Down again to a longer line
5. Original column position should be restored

## Testing Undo/Redo

```bash
cargo run --bin patina test.md
```

1. Type "Hello World"
2. Press **Ctrl + Z** (Undo) - should remove "d"
3. Press **Ctrl + Z** again - should remove "l"
4. Press **Ctrl + Y** (Redo) - should restore "l"
5. Press **Ctrl + Y** again - should restore "d"
6. Type new text - redo history should clear
7. Cursor position should restore correctly with undo/redo

## Testing File Operations

### Open File (Ctrl+O)
```bash
cargo run --bin patina
```
1. Press **Ctrl + O**
2. Type a filename in the prompt (e.g., "test.md")
3. Press **Enter**
4. File should open in new tab

### New Document (Ctrl+N)
1. Press **Ctrl + N**
2. New untitled document should open

### Close Tab (Ctrl+W)
1. Open multiple files
2. Press **Ctrl + W**
3. Current tab should close
4. Should switch to next tab

### Quit (Ctrl+Q)
1. Make changes to a file
2. Press **Ctrl + Q**
3. Should warn: "Unsaved changes! Press Ctrl+Q again to quit without saving."
4. Press **Ctrl + Q** again to force quit
5. OR Press **Ctrl + S** to save first, then **Ctrl + Q** to quit

## Features NOT Yet Implemented

### Mouse Scrolling ❌
**Status:** Not implemented in v0.2.0 or v0.3.0
**Planned for:** Future version (TBD)
**Workaround:** Use keyboard navigation (arrows, PageUp/PageDown)

### Preview Pane ❌
**Status:** Placeholder in v0.3.0
**Planned for:** v0.4.0 (Split View & Rendering)
**Current:** Can parse markdown but not display preview yet

### Syntax Highlighting in Editor ❌
**Status:** Not implemented
**Planned for:** v0.5.0
**Current:** Code blocks in markdown are parsed but not syntax highlighted

### Workspace Mode ❌
**Status:** Stub implementation
**Planned for:** v0.12.0
**Current:** Can open multiple individual files but no file tree

## Performance Testing

### Startup Time
```bash
time cargo run --bin patina --release test.md
```
**Target:** < 50ms cold start

### Memory Usage
```bash
cargo run --bin patina --release test-scrolling.md &
ps aux | grep patina
```
**Target:** < 20MB memory footprint

### Large File Handling
Create a large test file:
```bash
for i in {1..1000}; do echo "Line $i with some content to make it longer" >> large.md; done
cargo run --bin patina large.md
```
- Should open quickly
- Scrolling should be smooth
- No lag in editing

## Validation Checklist

Before considering features complete:

- [ ] Scrolling works smoothly (line-by-line and page)
- [ ] Tab switching works with Alt+Left/Right
- [ ] Tab switching works with Ctrl+[/]
- [ ] Ctrl+[/] do NOT insert characters
- [ ] Save messages are green with filename
- [ ] Error messages are red
- [ ] Undo/Redo works correctly
- [ ] Cursor always visible when navigating
- [ ] All 89 tests pass
- [ ] Zero compiler warnings
- [ ] Zero clippy warnings

## Reporting Issues

If you find bugs:
1. Note the exact key sequence to reproduce
2. Check your terminal emulator (iTerm2, Terminal.app, etc.)
3. Try alternative keybindings
4. File issue at: https://github.com/anthropics/patina/issues
