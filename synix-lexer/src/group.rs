use crate::{Error, Lex, LexBuffer, Span, TokenStream};

#[derive(Debug)]
pub struct Group {
    pub delimiter: Delimiter,
    pub inner: TokenStream,
    pub span: Span,
}

impl Group {
    pub fn has_delimiter(buf: &LexBuffer) -> bool {
        let mut fork = buf.fork();
        fork.skip_ws();

        match fork.peek() {
            Some('[') | Some('(') | Some('{') => true,
            _ => false,
        }
    }
}

impl Lex for Group {
    fn lex(buffer: &mut LexBuffer) -> crate::Result<Self> {
        use Delimiter::*;

        let start = buffer.current();
        let if_error = Span { start, end: start };

        buffer.skip_ws();

        let (delimiter, closing) = match buffer.peek() {
            Some('[') => (Bracket, ']'),
            Some('(') => (Paren, ')'),
            Some('{') => (Brace, '}'),
            Some(v) => return Err(Error::new(if_error, format!("Unexpected character {v}"))),
            None => return Err(Error::new(if_error, "Unexpected end of input.")),
        };

        // Consume character
        let _ = buffer.next();

        buffer.skip_ws();
        let next = buffer.peek();

        let inner = if next != Some(closing) {
            buffer.lex()?
        } else {
            Default::default()
        };

        let span = buffer.span_from(start);

        let me = Self {
            delimiter,
            inner,
            span: span.clone(),
        };

        let next = buffer.next();

        if next == Some(closing) {
            Ok(me)
        } else if let Some(next) = next {
            Err(Error::new(
                span,
                format!("Unclosed group. Expecting {}, got {}", closing, next),
            ))
        } else {
            let span = buffer.span_from(start);
            Err(Error::new(
                span,
                format!("Unclosed group. Expecting {}, got EOF", closing),
            ))
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Delimiter {
    Brace,
    Paren,
    Bracket,
}
