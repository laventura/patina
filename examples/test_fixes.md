# Bug Fix Test

## Emoji Test

Testing emoji expansion:

- :rocket: rocket
- :fire: fire
- :star: star
- :tada: tada (was missing)
- :white_check_mark: white check mark (was missing)
- :construction: construction (was missing)
- :round_pushpin: round pushpin (was missing)

Mixed: Hello :wave: from :rocket: Patina! This is :fire: hot! :tada:

## Table Alignment Test

### Left, Center, Right Alignment

| Left Aligned | Center Aligned | Right Aligned |
|:-------------|:--------------:|--------------:|
| Left | Center | Right |
| Data 1 | Data 2 | Data 3 |
| Short | Medium Text | Long Text Here |

The first column should be left-aligned, the second centered, and the third right-aligned.

### Default (Left) Alignment

| Feature | Status | Version |
|---------|--------|---------|
| Emoji | Fixed | v0.6.0 |
| Tables | Fixed | v0.6.0 |
| LaTeX | Partial | v0.6.0 |

All columns should be left-aligned by default.
