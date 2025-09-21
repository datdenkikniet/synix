use std::str::FromStr;

mod error;
mod r#let;

pub mod ident;
pub mod lit;
pub mod token;
pub use r#let::ExprLet;

pub use error::Error;
pub use ident::Ident;
use synix_lexer::{LineColumn, Span, TokenStream, TokenTree};

use crate::lit::ExprLit;
pub type Result<T> = core::result::Result<T, Error>;

pub fn parse(input: &str) -> Result<Expr> {
    let lexed = TokenStream::from_str(input)?;
    let mut buffer = ParseBuffer::new(lexed.as_ref());
    buffer.parse()
}

#[derive(Debug)]
pub enum Expr {
    Let(ExprLet),
    Lit(ExprLit),
}

impl Parse for Expr {
    fn parse(input: &mut ParseBuffer) -> Result<Self> {
        let output = if ExprLit::peek(input) {
            let lit = input.parse()?;
            Self::Lit(lit)
        } else if ExprLet::peek(input) {
            let let_ = input.parse()?;
            Self::Let(let_)
        } else {
            return Err(Error::new(input.span(), "Expected expr."));
        };

        Ok(output)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ParseBuffer<'a> {
    trees: &'a [TokenTree],
}

impl<'a> ParseBuffer<'a> {
    pub fn new(trees: &'a [TokenTree]) -> Self {
        Self { trees }
    }

    pub fn span(&self) -> Span {
        let start = LineColumn { line: 0, column: 0 };
        Span::new(start, start)
    }

    pub fn parse<T: Parse>(&mut self) -> Result<T> {
        T::parse(self)
    }

    pub fn fork(&self) -> Self {
        self.clone()
    }

    pub(crate) fn peek_tree(&self) -> Option<&'a TokenTree> {
        self.trees.get(0)
    }
}

impl<'a> Iterator for ParseBuffer<'a> {
    type Item = &'a TokenTree;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(current) = self.trees.get(0) {
            self.trees = &self.trees[1..];
            Some(current)
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.trees.len(), Some(self.trees.len()))
    }
}

impl ExactSizeIterator for ParseBuffer<'_> {
    fn len(&self) -> usize {
        self.trees.len()
    }
}

pub trait Parse: Sized {
    fn parse(buffer: &mut ParseBuffer) -> Result<Self>;
}

pub trait Peek {
    fn peek(input: &ParseBuffer) -> bool;
}
