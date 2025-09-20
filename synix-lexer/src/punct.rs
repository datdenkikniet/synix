use crate::{Error, Lex, LexBuffer};

#[derive(Debug, Clone)]
pub struct Punct {
    pub spacing: Spacing,
    pub ch: Char,
}

impl Punct {
    pub fn peek(buf: &LexBuffer) -> bool {
        let mut buf = buf.fork();

        Self::lex(&mut buf).is_ok()
    }
}

impl Lex for Punct {
    fn lex(buffer: &mut LexBuffer) -> crate::Result<Self> {
        let span = buffer.span();

        let err = |v| Error::new(span, format!("Expected punct, got {v:?}"));

        macro_rules ! do_map {
            ($($char:literal => $value:ident)*) => {
                match buffer.next() {
                    $(
                        Some($char) => Char::$value,
                    )*
                    v => return Err(err(v)),
                }
            }
        }

        let ch = do_map!(
            ';' => Semicolon
            ':' => Colon
            ',' => Comma
            '.' => Dot
            '@' => At
            '+' => Plus
            '-' => Minus
            '*' => Asterisk
            '>' => Gt
            '<' => Lt
            '=' => Equals
            '?' => Question
            '&' => Ampersand
            '|' => Pipe
            '/' => Slash
        );

        let spacing = if buffer.peek().is_none() || buffer.skip_ws() {
            Spacing::Alone
        } else {
            Spacing::Joint
        };

        Ok(Punct { spacing, ch })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Spacing {
    Alone,
    Joint,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Char {
    Semicolon,
    Colon,
    Comma,
    Dot,
    At,
    Plus,
    Minus,
    Asterisk,
    Gt,
    Lt,
    Equals,
    Question,
    Ampersand,
    Pipe,
    Slash,
}
