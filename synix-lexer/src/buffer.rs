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

    /// Returns a span that spans characters from start
    /// until the previously-lexed character (inclusive).
    pub fn span_from(&self, start: LineColumn) -> Span {
        // TODO: do the inclusive thing.
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

    pub fn skip_ws_and_comments(&mut self) -> bool {
        let mut fork = self.fork();

        let mut any = false;

        loop {
            let next = fork.next();

            if next.is_some_and(|n| n.is_whitespace()) {
                self.next();
                any = true;
            }
            // TODO: deal with comments
            else if next.is_some_and(|n| n == '#') {
                self.next();

                while fork.next() != Some('\n') {
                    self.next();
                }
                any = true;
            } else {
                break;
            }
        }

        any
    }

    pub fn next(&mut self) -> Option<char> {
        loop {
            let value = self.inner.next()?;

            self.current_offset += value.len_utf8();
            self.current.column += 1;

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

    pub fn is_empty(&mut self) -> bool {
        self.inner.peek().is_none()
    }
}

impl Iterator for LexBuffer<'_> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        self.next()
    }
}
