use crate::{Error, Lex, LexBuffer, Result, Span};

#[derive(Debug, Clone)]
pub enum Literal {
    Int(LitInt),
    Float(LitFloat),
    Str(LitStr),
}

impl Literal {
    pub fn starts(char: Option<char>) -> bool {
        let char = if let Some(next) = char {
            next
        } else {
            return false;
        };

        char == '"' || char == '\'' || char.is_numeric()
    }
}

impl Lex for Literal {
    fn lex(buffer: &mut LexBuffer) -> Result<Self> {
        let peeked = buffer.peek();

        if peeked == Some('"') || peeked == Some('\'') {
            let str = buffer.lex()?;
            Ok(Self::Str(str))
        } else if peeked.is_some_and(|v| v.is_numeric()) {
            let num: IntOrFloat = buffer.lex()?;

            match num.kind {
                Kind::Int => Ok(Self::Int(LitInt {
                    digits: num.digits,
                    span: num.span,
                })),
                Kind::Float => Ok(Self::Float(LitFloat {
                    digits: num.digits,
                    span: num.span,
                })),
            }
        } else {
            Err(Error::new(buffer.span(), "Expected literal."))
        }
    }
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

            // Nix does not require string literals to be whitespace-separated from
            // successive token trees.

            Ok(Self { value, span })
        } else if buffer.peek() == Some('\'') {
            let _ = buffer.next();
            todo!("Multiline strings")
        } else {
            Err(Error::new(buffer.span(), "Expected string literal"))
        }
    }
}

enum Kind {
    Int,
    #[expect(unused)]
    Float,
}

struct IntOrFloat {
    kind: Kind,
    digits: String,
    span: Span,
}

impl Lex for IntOrFloat {
    fn lex(buffer: &mut LexBuffer) -> Result<Self> {
        let start = buffer.current();
        let mut digits = if buffer.peek().is_some_and(|v| v.is_numeric()) {
            let digit = buffer.next().expect("There is a character.");
            String::from(digit)
        } else {
            let msg = if let Some(peeked) = buffer.peek() {
                format!(
                    "Invalid character `{}` integer (floats are not supported yet).",
                    peeked
                )
            } else {
                format!("Expected integer (floats are not supported yet), got end of input.")
            };

            return Err(Error::new(buffer.span(), msg));
        };

        while let Some(digit) = buffer.peek()
            && digit.is_numeric()
        {
            let _ = buffer.next();
            digits.push(digit);
        }

        // Nix does not require integer literals to be whitespace-separated from
        // successive token trees.

        Ok(IntOrFloat {
            digits,
            kind: Kind::Int,
            span: buffer.span_from(start),
        })
    }
}
