use comrak::nodes::{AstNode, NodeValue};
use comrak::{parse_document, Arena, Options};

fn walk<'a>(node: &'a AstNode<'a>, depth: usize) {
    let indent = "  ".repeat(depth);
    let ast = node.data.borrow();

    match &ast.value {
        NodeValue::List(_) => println!("{}List", indent),
        NodeValue::Item(_) => println!("{}Item", indent),
        NodeValue::Paragraph => println!("{}Paragraph", indent),
        NodeValue::TaskItem(symbol) => println!("{}TaskItem(symbol={:?})", indent, symbol),
        NodeValue::Text(t) => println!("{}Text(\"{}\")", indent, t),
        _ => {}
    }

    for child in node.children() {
        walk(child, depth + 1);
    }
}

fn main() {
    let md = r#"- [ ] Unchecked item
- [x] Checked lowercase
- [X] Checked uppercase"#;

    let arena = Arena::new();
    let mut options = Options::default();
    options.extension.tasklist = true;

    let root = parse_document(&arena, md, &options);

    println!("Input markdown:");
    println!("{}", md);
    println!("\nAST structure:");
    walk(root, 0);
}
