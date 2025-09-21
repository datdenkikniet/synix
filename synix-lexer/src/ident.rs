use crate::{Error, Lex, LexBuffer, Span};

#[derive(Debug)]
pub struct Ident {
    ident: String,
    pub span: Span,
}

impl Ident {
    pub fn ident(&self) -> &str {
        &self.ident
    }

    fn allowed(c: char) -> bool {
        c.is_alphanumeric() || c == '_'
    }

    pub fn starts(char: Option<char>) -> bool {
        let char = if let Some(char) = char {
            char
        } else {
            return false;
        };

        char.is_alphabetic()
    }
}

impl Lex for Ident {
    fn lex(buffer: &mut LexBuffer) -> crate::Result<Self> {
        let mut ident = if let Some(char) = buffer.peek()
            && char.is_alphabetic()
        {
            String::from(char)
        } else {
            return Err(Error::new(buffer.span(), "Expected ident"));
        };

        let start = buffer.current();
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
