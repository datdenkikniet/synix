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
}
