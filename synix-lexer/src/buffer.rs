use std::{iter::Peekable, str::Chars};

use crate::{Lex, LineColumn, Span};

#[derive(Debug)]
pub struct LexBuffer<'a> {
    inner: Peekable<Chars<'a>>,
    current_offset: usize,
    current: LineColumn,
}

impl<'a> LexBuffer<'a> {
    pub fn new(str: &'a str) -> Self {
        Self {
            inner: str.chars().peekable(),
            current_offset: 0,
            current: Default::default(),
        }
    }

    pub fn current(&self) -> LineColumn {
        self.current
    }

    pub fn span(&self) -> Span {
        Span {
            start: self.current,
            end: self.current,
        }
    }

    pub fn span_from(&self, start: LineColumn) -> Span {
        Span {
            start,
            end: self.current,
        }
    }

    pub fn fork(&self) -> Self {
        LexBuffer {
            inner: self.inner.clone(),
            current_offset: self.current_offset,
            current: self.current,
        }
    }

    pub fn peek(&mut self) -> Option<char> {
        self.inner.peek().copied()
    }

    pub fn skip_ws(&mut self) -> bool {
        let mut fork = self.fork();

        let mut any = false;

        while fork.next().is_some_and(|v| v.is_whitespace()) {
            self.next();
            any = true;
        }

        any
    }

    pub fn next(&mut self) -> Option<char> {
        loop {
            let value = self.inner.next()?;

            self.current_offset += value.len_utf8();

            if value == '\n' {
                self.current = LineColumn {
                    line: self.current.line + 1,
                    column: 0,
                };
            }

            return Some(value);
        }
    }

    pub fn lex<T: Lex>(&mut self) -> crate::Result<T> {
        T::lex(self)
    }
}

impl Iterator for LexBuffer<'_> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        self.next()
    }
}
