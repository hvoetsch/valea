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

    pub fn render_human(&self) -> String {
        format!(
            "{}: {} ({}..{})",
            self.code, self.message, self.span.start, self.span.end
        )
    }
}
