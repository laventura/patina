---
title: Complete Markdown Feature Showcase
author: Patina Team
date: 2026-01-19
tags: [markdown, demo, features]
---
# Patina - Complete Feature Showcase

This document demonstrates all the markdown features supported by Patina v0.6.0.

## Frontmatter

The frontmatter above (YAML between `---` markers) is parsed and displayed in a styled box at the top of the preview.

## Headings

# H1 Heading (Largest)
## H2 Heading
### H3 Heading
#### H4 Heading
##### H5 Heading
###### H6 Heading

---

## Text Formatting

**Bold text** for emphasis.

*Italic text* for subtle emphasis.

~~Strikethrough text~~ for corrections.

`Inline code` with monospace font.

Combine **_bold and italic_** for maximum impact!

---

## Lists

### Unordered List

- First item
- Second item
  - Nested item 1
  - Nested item 2
- Third item

### Ordered List

1. First step
2. Second step
   1. Sub-step A
   2. Sub-step B
3. Third step

### Task Lists

- [x] Completed task
- [ ] Pending task
- [ ] Another pending task
- [x] Another completed task

---

## Links and Images

[Visit Patina on GitHub](https://github.com/yourusername/patina)

[Link with title](https://example.com "Example Website")

Image (alt text shown): ![Rust Logo](https://www.rust-lang.org/logos/rust-logo-512x512.png)

---

## Code Blocks

### Inline Code

Use the `println!` macro in Rust.

### Code Block with Syntax Highlighting

```rust
fn main() {
    println!("Hello, Patina!");

    let x = 42;
    let y = x * 2;

    println!("The answer is: {}", y);
}
```

```python
def fibonacci(n):
    if n <= 1:
        return n
    return fibonacci(n-1) + fibonacci(n-2)

print(f"F(10) = {fibonacci(10)}")
```

```javascript
const greet = (name) => {
    return `Hello, ${name}!`;
};

console.log(greet("World"));
```

---

## LaTeX Math :fire:

### Inline Math

The Pythagorean theorem: $a^2 + b^2 = c^2$

Einstein's equation: $E = mc^2$

Quadratic roots: $x = \frac{-b \pm \sqrt{b^2 - 4ac}}{2a}$

### Greek Letters

Lowercase: $\alpha$, $\beta$, $\gamma$, $\delta$, $\epsilon$, $\theta$, $\lambda$, $\mu$, $\pi$, $\sigma$, $\omega$

Uppercase: $\Gamma$, $\Delta$, $\Theta$, $\Lambda$, $\Pi$, $\Sigma$, $\Phi$, $\Psi$, $\Omega$

### Mathematical Operators

- Sum: $\sum_{i=1}^{n} i$
- Product: $\prod_{i=1}^{n} i$
- Integral: $\int_0^\infty e^{-x}dx$
- Partial derivative: $\frac{\partial f}{\partial x}$
- Square root: $\sqrt{2}$, $\sqrt[3]{8}$
- Infinity: $\infty$

### Subscripts and Superscripts

- Subscripts: $x_0$, $a_n$, $v_{max}$
- Superscripts: $x^2$, $e^{ix}$, $2^{10}$
- Combined: $x_i^2 + y_j^2 = z_{ij}$

### Display Math (Centered)

$$\sum_{n=1}^{\infty} \frac{1}{n^2} = \frac{\pi^2}{6}$$

$$\int_{-\infty}^{\infty} e^{-x^2}dx = \sqrt{\pi}$$

$$e^{i\pi} + 1 = 0$$

---

## Emoji Support :rocket:

### Smileys and Emotions

:smile: :grin: :joy: :rofl: :wink: :heart_eyes: :thinking: :sunglasses:

### Gestures

:+1: :-1: :wave: :clap: :pray: :muscle:

### Objects and Symbols

:rocket: :fire: :star: :100: :bulb: :books: :memo: :computer: :phone:

### Programming Emojis

Fix the :bug: with your :wrench: and :hammer:!

Deploy the :package: to production :zap:

Lock it up: :lock: :key:

Add some :sparkles: to your code!

### Nature

:sun: :moon: :cloud: :rainbow: :tree: :flower:

### Status Symbols

:check: :x: :warning: :question: :exclamation:

---

## Blockquotes

> This is a blockquote.
> It can span multiple lines.
>
> And have multiple paragraphs.

> **Nested formatting** works too!
>
> Including `code` and :rocket: emoji!

---

## Tables

| Feature | Status | Version |
|---------|--------|---------|
| Markdown | ✓ | v0.3.0 |
| Syntax Highlighting | ✓ | v0.5.0 |
| LaTeX Math | ✓ | v0.6.0 |
| Emoji | ✓ | v0.6.0 |
| Mermaid | Planned | v0.9.0 |

| Left Aligned | Center Aligned | Right Aligned |
|:-------------|:--------------:|--------------:|
| Left | Center | Right |
| Data 1 | Data 2 | Data 3 |

---

## Mixed Content

You can combine all features together! For example:

**The area of a circle** :round_pushpin: with radius $r$ is given by:

$$A = \pi r^2$$

For $r = 5$, we calculate: :computer:

$$A = \pi \times 5^2 = 25\pi \approx 78.54$$

That's amazing! :star: :100: :rocket:

---

## Horizontal Rules

You can create horizontal rules with `---`:

---

Like that!

---

## Complex Example

Here's a **complex example** combining :sparkles: **multiple features** :sparkles::

> ### Euler's Identity :star:
>
> Euler's identity is considered the most beautiful equation in mathematics:
>
> $$e^{i\pi} + 1 = 0$$
>
> It connects five fundamental mathematical constants:
>
> - $e$ (Euler's number): The base of natural logarithms
> - $i$ (imaginary unit): Satisfies $i^2 = -1$
> - $\pi$ (pi): The ratio of a circle's circumference to diameter
> - $1$ (one): The multiplicative identity
> - $0$ (zero): The additive identity
>
> :bulb: **Fun fact**: This equation appears in many areas of mathematics and physics!

---

*Made with* :heart: by *Patina team*

:tada:

:love:
---

## Code with Math Comments

```python
import numpy as np

# Calculate π using Monte Carlo method
# Formula: π ≈ 4 * (points inside circle / total points)
def estimate_pi(n_samples=1000000):
    """
    Estimate π using random sampling.

    Based on: A = πr², where r=1, so A = π
    """
    inside = 0

    for _ in range(n_samples):
        x = np.random.random()  # x ∈ [0, 1]
        y = np.random.random()  # y ∈ [0, 1]

        if x**2 + y**2 <= 1:    # x² + y² ≤ 1
            inside += 1

    return 4.0 * inside / n_samples

print(f"π ≈ {estimate_pi()}")  # Should be ≈ 3.14159
```

In the preview, math like $\pi$ in comments is rendered! :rocket:

---

## Task List with Emojis

Development progress for v0.6.0:

- [x] :white_check_mark: Core editor functionality
- [x] :white_check_mark: Markdown parsing (CommonMark + GFM)
- [x] :white_check_mark: Split view with preview
- [x] :white_check_mark: Syntax highlighting
- [x] :white_check_mark: Frontmatter support
- [x] :white_check_mark: LaTeX math rendering
- [x] :white_check_mark: Emoji expansion
- [x] :white_check_mark: i18n framework
- [ ] :construction: Search and replace
- [ ] :construction: GUI version (v0.7.0)

---

## Conclusion

:tada: **Congratulations!** :tada:

You've seen all the features in Patina v0.6.0! :rocket:

From **basic formatting** to $\LaTeX$ math :heart:, from :fire: emoji :fire: to `code highlighting`, Patina has you covered!

**Try editing this file and watch the preview update in real-time!**

---

*Made with* :heart: *by the Patina team*
