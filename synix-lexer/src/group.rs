use crate::{Error, Lex, LexBuffer, Span, TokenStream};

#[derive(Debug, Clone)]
pub struct Group {
    pub delimiter: Delimiter,
    pub inner: TokenStream,
    span: Span,
}

impl Group {
    pub fn span(&self) -> Span {
        self.span.clone()
    }

    pub fn starts(buf: &mut LexBuffer) -> bool {
        match buf.peek() {
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

        let (delimiter, closing) = match buffer.next() {
            Some('[') => (Bracket, ']'),
            Some('(') => (Paren, ')'),
            Some('{') => (Brace, '}'),
            Some(v) => return Err(Error::new(if_error, format!("Unexpected character {v}"))),
            None => return Err(Error::new(if_error, "Unexpected end of input.")),
        };

        buffer.skip_ws();
        let next = buffer.peek();

        let inner = if next != Some(closing) {
            let mut trees = Vec::new();
            loop {
                let next_tree = buffer.lex()?;
                trees.push(next_tree);

                buffer.skip_ws();

                if buffer.peek() == Some(closing) {
                    break;
                }
            }

            TokenStream::new(trees)
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Delimiter {
    Brace,
    Paren,
    Bracket,
}
