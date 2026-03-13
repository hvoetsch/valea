#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Diagnostic {
    pub code: &'static str,
    pub message: String,
    pub span: Span,
}

impl Diagnostic {
    pub fn new(code: &'static str, message: impl Into<String>, start: usize, end: usize) -> Self {
        Self {
            code,
            message: message.into(),
            span: Span { start, end },
        }
    }

    /// Basic rendering using raw byte offsets. Prefer `render_human_with_source` when the
    /// source text is available, as it shows human-readable line:col positions.
    pub fn render_human(&self) -> String {
        format!(
            "{}: {} ({}..{})",
            self.code, self.message, self.span.start, self.span.end
        )
    }

    /// Renders the diagnostic with a `line:col` position derived from the source text.
    pub fn render_human_with_source(&self, source: &str) -> String {
        let (line, col) = offset_to_line_col(source, self.span.start);
        format!("{}:{}: {}: {}", line, col, self.code, self.message)
    }
}

/// Converts a byte offset into a (line, col) pair (both 1-based).
pub fn offset_to_line_col(source: &str, offset: usize) -> (usize, usize) {
    let clamped = offset.min(source.len());
    let before = &source[..clamped];
    let line = before.chars().filter(|&c| c == '\n').count() + 1;
    let col = before.rfind('\n').map(|p| clamped - p - 1).unwrap_or(clamped) + 1;
    (line, col)
}
