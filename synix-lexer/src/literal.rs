use crate::{Error, Lex, LexBuffer, Span};

#[derive(Debug)]
pub enum Literal {
    LitInt(LitInt),
    LitFloat(LitFloat),
    LitStr(LitStr),
    LitBool(LitBool),
}

macro_rules! literal {
    ($($name:ident, $value_name:ident = $value:ty),*$(,)?) => {
        $(
            #[derive(Debug, Clone)]
            pub struct $name {
                pub $value_name: $value,
                pub span: Span,
            }
        )*
    };
}

literal! {
    LitBool, value = bool,
    LitStr, value = String,
    LitInt, digits = String,
    LitFloat, digits = String,
}

impl Lex for LitStr {
    fn lex(buffer: &mut LexBuffer) -> crate::Result<Self> {
        if buffer.peek() == Some('"') {
            let start = buffer.current();
            let _ = buffer.next();

            // TODO: handle escape sequences other than \"
            let mut escaped = false;
            let mut finished = false;
            let mut value = String::new();
            while let Some(char) = buffer.next() {
                if char == '\\' {
                    escaped = true;
                    continue;
                } else if char == '\r' || char == '\n' {
                    return Err(Error::new(
                        buffer.span_from(start),
                        "Unterminated string. For multiline strings, use `'''`",
                    ));
                } else if char == '"' {
                    if escaped {
                        escaped = false;
                    } else {
                        finished = true;
                        break;
                    }
                }

                value.push(char);
            }

            let span = buffer.span_from(start);

            if !finished {
                return Err(Error::new(span, "Unterminated string."));
            }

            Ok(Self { value, span })
        } else if buffer.peek() == Some('\'') {
            let _ = buffer.next();
            todo!()
        } else {
            Err(Error::new(buffer.span(), "Expected string literal"))
        }
    }
}

impl Lex for LitBool {
    fn lex(buffer: &mut LexBuffer) -> crate::Result<Self> {
        const TRUE: [char; 4] = ['t', 'r', 'u', 'e'];
        const FALSE: [char; 5] = ['f', 'a', 'l', 's', 'e'];

        let mut chars = [char::default(); 5];

        buffer.fork().take(5).enumerate().for_each(|(idx, v)| {
            chars[idx] = v;
        });

        let value = if &chars[..4] == &TRUE[..] {
            true
        } else if chars == FALSE {
            false
        } else {
            return Err(Error::new(buffer.span(), format!("Expected bool")));
        };

        let start = buffer.current();
        let len = if value { 4 } else { 5 };
        let _ = (0..len).for_each(|_| {
            buffer.next();
        });

        let span = buffer.span_from(start);

        if buffer.peek().is_none() || buffer.skip_ws() {
            Ok(Self { span, value })
        } else {
            return Err(Error::new(span, format!("Expected bool")));
        }
    }
}
