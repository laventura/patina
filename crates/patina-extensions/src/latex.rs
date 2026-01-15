//! LaTeX math rendering to Unicode.

use std::collections::HashMap;
use once_cell::sync::Lazy;

/// LaTeX to Unicode symbol mappings
static SYMBOLS: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut m = HashMap::new();
    
    // Greek letters
    m.insert("alpha", "α");
    m.insert("beta", "β");
    m.insert("gamma", "γ");
    m.insert("delta", "δ");
    m.insert("epsilon", "ε");
    m.insert("zeta", "ζ");
    m.insert("eta", "η");
    m.insert("theta", "θ");
    m.insert("iota", "ι");
    m.insert("kappa", "κ");
    m.insert("lambda", "λ");
    m.insert("mu", "μ");
    m.insert("nu", "ν");
    m.insert("xi", "ξ");
    m.insert("pi", "π");
    m.insert("rho", "ρ");
    m.insert("sigma", "σ");
    m.insert("tau", "τ");
    m.insert("upsilon", "υ");
    m.insert("phi", "φ");
    m.insert("chi", "χ");
    m.insert("psi", "ψ");
    m.insert("omega", "ω");
    
    // Capital Greek
    m.insert("Gamma", "Γ");
    m.insert("Delta", "Δ");
    m.insert("Theta", "Θ");
    m.insert("Lambda", "Λ");
    m.insert("Xi", "Ξ");
    m.insert("Pi", "Π");
    m.insert("Sigma", "Σ");
    m.insert("Phi", "Φ");
    m.insert("Psi", "Ψ");
    m.insert("Omega", "Ω");
    
    // Operators
    m.insert("sum", "Σ");
    m.insert("prod", "Π");
    m.insert("int", "∫");
    m.insert("oint", "∮");
    m.insert("partial", "∂");
    m.insert("nabla", "∇");
    m.insert("sqrt", "√");
    m.insert("infty", "∞");
    m.insert("pm", "±");
    m.insert("mp", "∓");
    m.insert("times", "×");
    m.insert("div", "÷");
    m.insert("cdot", "·");
    m.insert("leq", "≤");
    m.insert("geq", "≥");
    m.insert("neq", "≠");
    m.insert("approx", "≈");
    m.insert("equiv", "≡");
    m.insert("in", "∈");
    m.insert("notin", "∉");
    m.insert("subset", "⊂");
    m.insert("supset", "⊃");
    m.insert("cup", "∪");
    m.insert("cap", "∩");
    m.insert("emptyset", "∅");
    m.insert("forall", "∀");
    m.insert("exists", "∃");
    m.insert("neg", "¬");
    m.insert("land", "∧");
    m.insert("lor", "∨");
    m.insert("to", "→");
    m.insert("gets", "←");
    m.insert("leftrightarrow", "↔");
    m.insert("Rightarrow", "⇒");
    m.insert("Leftarrow", "⇐");
    m.insert("Leftrightarrow", "⇔");
    
    m
});

/// Superscript mappings
static SUPERSCRIPTS: Lazy<HashMap<char, char>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert('0', '⁰');
    m.insert('1', '¹');
    m.insert('2', '²');
    m.insert('3', '³');
    m.insert('4', '⁴');
    m.insert('5', '⁵');
    m.insert('6', '⁶');
    m.insert('7', '⁷');
    m.insert('8', '⁸');
    m.insert('9', '⁹');
    m.insert('+', '⁺');
    m.insert('-', '⁻');
    m.insert('=', '⁼');
    m.insert('(', '⁽');
    m.insert(')', '⁾');
    m.insert('n', 'ⁿ');
    m.insert('i', 'ⁱ');
    m
});

/// Subscript mappings
static SUBSCRIPTS: Lazy<HashMap<char, char>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert('0', '₀');
    m.insert('1', '₁');
    m.insert('2', '₂');
    m.insert('3', '₃');
    m.insert('4', '₄');
    m.insert('5', '₅');
    m.insert('6', '₆');
    m.insert('7', '₇');
    m.insert('8', '₈');
    m.insert('9', '₉');
    m.insert('+', '₊');
    m.insert('-', '₋');
    m.insert('=', '₌');
    m.insert('(', '₍');
    m.insert(')', '₎');
    m.insert('a', 'ₐ');
    m.insert('e', 'ₑ');
    m.insert('i', 'ᵢ');
    m.insert('n', 'ₙ');
    m.insert('o', 'ₒ');
    m.insert('x', 'ₓ');
    m
});

/// LaTeX renderer for terminal output
pub struct LatexRenderer;

impl LatexRenderer {
    /// Create a new renderer
    pub fn new() -> Self {
        Self
    }

    /// Render LaTeX to Unicode string
    pub fn render(&self, latex: &str) -> String {
        let mut result = String::new();
        let mut chars = latex.chars().peekable();

        while let Some(c) = chars.next() {
            match c {
                '\\' => {
                    // Parse command
                    let cmd: String = chars
                        .by_ref()
                        .take_while(|c| c.is_alphabetic())
                        .collect();
                    
                    if let Some(symbol) = SYMBOLS.get(cmd.as_str()) {
                        result.push_str(symbol);
                    } else {
                        result.push('\\');
                        result.push_str(&cmd);
                    }
                }
                '^' => {
                    // Superscript
                    if let Some(next) = chars.next() {
                        if next == '{' {
                            // Grouped superscript
                            let group: String = chars
                                .by_ref()
                                .take_while(|c| *c != '}')
                                .collect();
                            for gc in group.chars() {
                                if let Some(&sup) = SUPERSCRIPTS.get(&gc) {
                                    result.push(sup);
                                } else {
                                    result.push(gc);
                                }
                            }
                        } else if let Some(&sup) = SUPERSCRIPTS.get(&next) {
                            result.push(sup);
                        } else {
                            result.push('^');
                            result.push(next);
                        }
                    }
                }
                '_' => {
                    // Subscript
                    if let Some(next) = chars.next() {
                        if next == '{' {
                            let group: String = chars
                                .by_ref()
                                .take_while(|c| *c != '}')
                                .collect();
                            for gc in group.chars() {
                                if let Some(&sub) = SUBSCRIPTS.get(&gc) {
                                    result.push(sub);
                                } else {
                                    result.push(gc);
                                }
                            }
                        } else if let Some(&sub) = SUBSCRIPTS.get(&next) {
                            result.push(sub);
                        } else {
                            result.push('_');
                            result.push(next);
                        }
                    }
                }
                '{' | '}' => {
                    // Skip grouping braces
                }
                ' ' => {
                    // Preserve spaces
                    result.push(' ');
                }
                _ => {
                    result.push(c);
                }
            }
        }

        result
    }
}

impl Default for LatexRenderer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_symbols() {
        let renderer = LatexRenderer::new();
        assert_eq!(renderer.render("\\alpha"), "α");
        assert_eq!(renderer.render("\\sum"), "Σ");
        assert_eq!(renderer.render("\\infty"), "∞");
    }

    #[test]
    fn test_superscripts() {
        let renderer = LatexRenderer::new();
        assert_eq!(renderer.render("x^2"), "x²");
        assert_eq!(renderer.render("e^{ix}"), "eⁱˣ");
    }

    #[test]
    fn test_subscripts() {
        let renderer = LatexRenderer::new();
        assert_eq!(renderer.render("x_0"), "x₀");
        assert_eq!(renderer.render("a_{n}"), "aₙ");
    }

    #[test]
    fn test_complex() {
        let renderer = LatexRenderer::new();
        let result = renderer.render("E = mc^2");
        assert!(result.contains("²"));
    }
}
