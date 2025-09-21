use crate::{Error, Lex, LexBuffer, Span};

#[derive(Debug, Clone)]
pub struct Ident {
    ident: String,
    pub span: Span,
}

impl Ident {
    pub fn ident(&self) -> &str {
        &self.ident
    }

    fn allowed(c: char) -> bool {
        c.is_alphanumeric() || c == '_' || c == '\''
    }

    pub fn starts(char: Option<char>) -> bool {
        let char = if let Some(char) = char {
            char
        } else {
            return false;
        };

        char.is_alphabetic() || char == '_'
    }
}

impl Lex for Ident {
    fn lex(buffer: &mut LexBuffer) -> crate::Result<Self> {
        let start = buffer.current();

        let mut ident = if Self::starts(buffer.peek()) {
            let char = buffer.next().expect("This cannot be empty");
            String::from(char)
        } else {
            return Err(Error::new(buffer.span(), "Expected ident"));
        };

        while let Some(char) = buffer.peek()
            && Self::allowed(char)
        {
            let _ = buffer.next();
            ident.push(char);
        }

        let span = buffer.span_from(start);
        Ok(Ident { span, ident })
    }
}
