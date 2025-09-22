use std::fmt::Write;

use crate::{Error, Lex, LexBuffer, Span};

#[derive(Debug, Clone)]
pub struct Punct {
    pub spacing: Spacing,
    pub ch: Char,
    span: Span,
}

impl Punct {
    pub fn span(&self) -> Span {
        self.span.clone()
    }

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
            ($($char:literal => $value:ident)*) => {{
                match buffer.next() {
                    $(
                        Some($char) => Char::$value,
                    )*
                    v => return Err(err(v)),
                }
            }}
        }

        let start = buffer.current();
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
        let span = buffer.span_from(start);

        let spacing = if buffer.peek().is_none() || buffer.skip_ws() {
            Spacing::Alone
        } else {
            Spacing::Joint
        };

        Ok(Punct { spacing, ch, span })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Spacing {
    Alone,
    Joint,
}

impl Spacing {
    pub fn is_joint(&self) -> bool {
        self == &Self::Joint
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
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

impl core::fmt::Display for Char {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let v = match self {
            Char::Semicolon => ';',
            Char::Colon => ':',
            Char::Comma => ',',
            Char::Dot => '.',
            Char::At => '@',
            Char::Plus => '+',
            Char::Minus => '-',
            Char::Asterisk => '*',
            Char::Gt => '>',
            Char::Lt => '<',
            Char::Equals => '=',
            Char::Question => '?',
            Char::Ampersand => '&',
            Char::Pipe => '|',
            Char::Slash => '/',
        };

        f.write_char(v)
    }
}
