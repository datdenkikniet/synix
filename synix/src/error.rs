use synix_lexer::Span;

#[derive(Debug)]
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

impl From<synix_lexer::Error> for Error {
    fn from(value: synix_lexer::Error) -> Self {
        Self::new(value.span(), format!("Lexer error: {}", value.message()))
    }
}
