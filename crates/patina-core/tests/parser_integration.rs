//! Integration tests for parser and document

use comrak::Arena;
use patina_core::Document;

#[test]
fn test_document_html_generation() {
    let mut doc = Document::from_content("# Hello\n\nThis is a **test**.");

    let html = doc.html();

    assert!(html.contains("<h1>Hello</h1>"));
    assert!(html.contains("<strong>test</strong>"));
}

#[test]
fn test_document_html_caching() {
    let mut doc = Document::from_content("# Test");

    // First call generates HTML
    let html1 = doc.html().to_string();
    assert!(html1.contains("<h1>Test</h1>"));

    // Second call should return cached version
    let html2 = doc.html().to_string();
    assert_eq!(html1, html2);
}

#[test]
fn test_document_html_invalidation() {
    let mut doc = Document::from_content("# Original");

    let html1 = doc.html();
    assert!(html1.contains("Original"));

    // Modify buffer and invalidate cache
    doc.buffer.insert(0, "Updated ");
    doc.invalidate_cache();

    let html2 = doc.html();
    assert!(html2.contains("Updated"));
}

#[test]
fn test_document_headings_extraction() {
    let doc = Document::from_content("# Title\n\n## Section 1\n\n## Section 2\n\n### Subsection");

    let headings = doc.headings();

    assert_eq!(headings.len(), 4);
    assert_eq!(headings[0].level, 1);
    assert_eq!(headings[0].text, "Title");
    assert_eq!(headings[1].level, 2);
    assert_eq!(headings[1].text, "Section 1");
    assert_eq!(headings[2].level, 2);
    assert_eq!(headings[2].text, "Section 2");
    assert_eq!(headings[3].level, 3);
    assert_eq!(headings[3].text, "Subsection");
}

#[test]
fn test_document_ast_parsing() {
    let doc = Document::from_content("# Heading\n\nParagraph with *emphasis*.");

    let arena = Arena::new();
    let ast = doc.parse(&arena);

    // Verify we get a valid AST root
    assert!(ast.children().count() > 0);
}

#[test]
fn test_empty_document_parsing() {
    let mut doc = Document::new();

    let html = doc.html();
    assert!(html.is_empty() || html.trim().is_empty());

    let headings = doc.headings();
    assert_eq!(headings.len(), 0);
}

#[test]
fn test_gfm_features_in_document() {
    let md = r#"
# Document with GFM

## Table

| Feature | Status |
|---------|--------|
| Tables  | ✓      |
| Tasks   | ✓      |

## Task List

- [x] Completed task
- [ ] Pending task

## Strikethrough

~~This is deleted~~

## Autolink

https://example.com
"#;

    let mut doc = Document::from_content(md);
    let html = doc.html();

    assert!(html.contains("<table>"));
    assert!(html.contains("checkbox"));
    assert!(html.contains("<del>This is deleted</del>"));
    assert!(html.contains("https://example.com"));
}

#[test]
fn test_code_blocks_in_document() {
    let md = r#"
# Code Examples

Inline `code` works.

```rust
fn main() {
    println!("Hello, world!");
}
```

```python
def greet():
    print("Hello")
```
"#;

    let mut doc = Document::from_content(md);
    let html = doc.html();

    assert!(html.contains("<code>code</code>"));
    assert!(html.contains("fn main()"));
    assert!(html.contains("def greet()"));
}

#[test]
fn test_multiline_content() {
    let md = "Line 1\n\nLine 2\n\nLine 3";
    let mut doc = Document::from_content(md);

    let html = doc.html();

    assert!(html.contains("Line 1"));
    assert!(html.contains("Line 2"));
    assert!(html.contains("Line 3"));
}

#[test]
fn test_heading_line_numbers_accuracy() {
    let md = "# First\n\nContent\n\n## Second\n\nMore content\n\n### Third";
    let doc = Document::from_content(md);

    let headings = doc.headings();

    assert_eq!(headings[0].line, 1); // "# First" is on line 1
    assert_eq!(headings[1].line, 5); // "## Second" is on line 5
    assert_eq!(headings[2].line, 9); // "### Third" is on line 9
}
