---
title: Patina Markdown Editor
author: Development Team
date: 2026-01-16
version: 0.5.0
tags: [markdown, editor, rust, tui]
---
# Welcome to Patina v0.5.0

This document demonstrates the new features implemented in **v0.4.0** and **v0.5.0**:

## Features

OK this is live editin. I'm enterin a pi π^2 


### v0.4.0: Split View & Rendering

- ✅ Live markdown preview
- ✅ Split view (editor + preview)
- ✅ Rendering of all markdown elements
- ✅ Headings, paragraphs, lists, blockquotes
- ✅ Links, images, inline code
- ✅ **Bold**, *italic*, ~~strikethrough~~

### v0.5.0: Syntax Highlighting & Frontmatter

- ✅ Syntax highlighting in code blocks
- ✅ Frontmatter display (YAML/TOML)
- ✅ 50+ programming languages supported
- ✅ Theme integration

## Code Examples

### Rust

```rust
fn main() {
    println!("Hello from Patina!");

    let numbers = vec![1, 2, 3, 4, 5];
    let sum: i32 = numbers.iter().sum();

    println!("Sum: {}", sum);
}
```

### Python

```python
def fibonacci(n):
    """Generate Fibonacci sequence up to n terms."""
    a, b = 0, 1
    for _ in range(n):
        yield a
        a, b = b, a + b

# Print first 10 Fibonacci numbers
for num in fibonacci(10):
    print(num, end=' ')
```

### JavaScript

```javascript
const fetchData = async (url) => {
    try {
        const response = await fetch(url);
        const data = await response.json();
        return data;
    } catch (error) {
        console.error('Error fetching data:', error);
        throw error;
    }
};

// Usage
fetchData('https://api.example.com/data')
    .then(data => console.log(data))
    .catch(err => console.error(err));
```

## Markdown Features

### Lists

**Unordered:**
- Item 1
- Item 2
  - Nested item
  - Another nested item
- Item 3

**Ordered:**
1. First step
2. Second step
3. Third step

### Blockquotes

> This is a blockquote.
> It can span multiple lines.
>
> And have multiple paragraphs.

### Links and Images

Check out [Patina on GitHub](https://github.com/anthropics/patina)

![Rust Logo](https://www.rust-lang.org/logos/rust-logo-512x512.png)

### Tables

| Feature | v0.4.0 | v0.5.0 |
|---------|--------|--------|
| Split View | ✅ | ✅ |
| Syntax Highlighting | ❌ | ✅ |
| Frontmatter | ❌ | ✅ |

### Inline Elements

Use `inline code` for commands like `cargo run`.

Text can be **bold**, *italic*, or ***both***.

You can also ~~strike through~~ text.

## Try It Out!

1. Open this file in Patina: `cargo run -- examples/frontmatter_sample.md`
2. Press `Ctrl+\` to toggle between Raw, Split, and Preview modes
3. Navigate with arrow keys
4. Scroll with PageUp/PageDown (or Fn+Up/Down on Mac)
5. Switch tabs with Alt+Left/Right
6. Save changes with Ctrl+S

## Performance

- Startup: <50ms
- Memory: <25MB
- Render: 60fps
- Supports files up to 100,000 lines

Enjoy coding with Patina! 🚀
