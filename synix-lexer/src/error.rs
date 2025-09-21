use crate::Span;

#[derive(Debug, Clone)]
pub struct Error {
    span: Span,
    message: String,
}

impl Error {
    pub fn new(span: Span, message: impl Into<String>) -> Self {
        Self {
            span,
            message: message.into(),
        }
    }

    pub fn span(&self) -> Span {
        self.span.clone()
    }

    pub fn message(&self) -> &str {
        &self.message
    }
}
