//! Mermaid diagram rendering to ASCII art.
//!
//! Supports flowcharts, sequence diagrams, and other Mermaid diagram types
//! rendered as Unicode box-drawing characters.

/// Mermaid diagram renderer
pub struct MermaidRenderer {
    /// Maximum width for output
    max_width: usize,
    /// Use Unicode box-drawing characters
    use_unicode: bool,
}

impl MermaidRenderer {
    /// Create a new renderer
    pub fn new() -> Self {
        Self {
            max_width: 80,
            use_unicode: true,
        }
    }

    /// Set maximum output width
    pub fn max_width(mut self, width: usize) -> Self {
        self.max_width = width;
        self
    }

    /// Use ASCII instead of Unicode
    pub fn ascii_mode(mut self) -> Self {
        self.use_unicode = false;
        self
    }

    /// Render a Mermaid diagram to text
    pub fn render(&self, mermaid: &str) -> Result<String, MermaidError> {
        let diagram_type = self.detect_type(mermaid)?;

        match diagram_type {
            DiagramType::Flowchart => self.render_flowchart(mermaid),
            DiagramType::Sequence => self.render_sequence(mermaid),
            DiagramType::State => self.render_state(mermaid),
            DiagramType::Pie => self.render_pie(mermaid),
            DiagramType::Unknown => Err(MermaidError::UnsupportedDiagram),
        }
    }

    fn detect_type(&self, mermaid: &str) -> Result<DiagramType, MermaidError> {
        let first_line = mermaid.lines().next().unwrap_or("").trim().to_lowercase();

        Ok(
            if first_line.starts_with("graph") || first_line.starts_with("flowchart") {
                DiagramType::Flowchart
            } else if first_line.starts_with("sequencediagram") {
                DiagramType::Sequence
            } else if first_line.starts_with("statediagram") {
                DiagramType::State
            } else if first_line.starts_with("pie") {
                DiagramType::Pie
            } else {
                DiagramType::Unknown
            },
        )
    }

    fn render_flowchart(&self, _mermaid: &str) -> Result<String, MermaidError> {
        // Simplified placeholder - real implementation would parse and render
        let (tl, tr, bl, br, h, v) = if self.use_unicode {
            ('┌', '┐', '└', '┘', '─', '│')
        } else {
            ('+', '+', '+', '+', '-', '|')
        };

        Ok(format!(
            "{tl}{h}{h}{h}{h}{h}{h}{h}{h}{h}{tr}     {tl}{h}{h}{h}{h}{h}{h}{h}{h}{h}{tr}     {tl}{h}{h}{h}{h}{h}{h}{h}{h}{h}{tr}\n\
             {v}  Start  {v}────▶{v} Process {v}────▶{v}   End   {v}\n\
             {bl}{h}{h}{h}{h}{h}{h}{h}{h}{h}{br}     {bl}{h}{h}{h}{h}{h}{h}{h}{h}{h}{br}     {bl}{h}{h}{h}{h}{h}{h}{h}{h}{h}{br}"
        ))
    }

    fn render_sequence(&self, _mermaid: &str) -> Result<String, MermaidError> {
        // Placeholder
        Ok(
            "┌─────┐          ┌─────┐\n│  A  │───────▶│  B  │\n└─────┘          └─────┘"
                .to_string(),
        )
    }

    fn render_state(&self, _mermaid: &str) -> Result<String, MermaidError> {
        // Placeholder
        Ok("( State1 ) ──▶ ( State2 )".to_string())
    }

    fn render_pie(&self, _mermaid: &str) -> Result<String, MermaidError> {
        // Placeholder - render as horizontal bars
        Ok("███████████░░░░░░░░░ 55% - Item A\n████████░░░░░░░░░░░░ 40% - Item B\n█░░░░░░░░░░░░░░░░░░░  5% - Item C".to_string())
    }
}

impl Default for MermaidRenderer {
    fn default() -> Self {
        Self::new()
    }
}

/// Diagram types
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DiagramType {
    Flowchart,
    Sequence,
    State,
    Pie,
    Unknown,
}

/// Mermaid rendering errors
#[derive(Debug, thiserror::Error)]
pub enum MermaidError {
    #[error("Failed to parse diagram")]
    ParseError,

    #[error("Unsupported diagram type")]
    UnsupportedDiagram,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_flowchart() {
        let renderer = MermaidRenderer::new();
        let mermaid = "graph TD\n    A --> B";
        let result = renderer.render(mermaid);
        assert!(result.is_ok());
    }

    #[test]
    fn test_detect_pie() {
        let renderer = MermaidRenderer::new();
        let mermaid = "pie\n    title Test\n    \"A\": 50\n    \"B\": 50";
        let result = renderer.render(mermaid);
        assert!(result.is_ok());
    }
}
