use std::str::FromStr;

mod error;
mod r#let;

pub mod ident;
mod lambda;
pub mod lit;
pub mod token;
pub use lambda::ExprLambda;
pub use r#let::ExprLet;

pub use error::Error;
use ident::Ident;
use synix_lexer::{
    Span, TokenStream, TokenTree,
    group::Delimiter,
    punct::{Char, Punct},
};

use crate::lit::ExprLit;
pub type Result<T> = core::result::Result<T, Error>;

#[expect(non_snake_case)]
pub fn Comma(tree: &TokenTree) -> bool {
    punct_peek_helper(tree, Char::Comma)
}

#[expect(non_snake_case)]
pub fn Brace(tree: &TokenTree) -> bool {
    group_peek_helper(tree, Delimiter::Brace)
}

#[expect(non_snake_case)]
pub fn Paren(tree: &TokenTree) -> bool {
    group_peek_helper(tree, Delimiter::Paren)
}

#[expect(non_snake_case)]
pub fn Bracket(tree: &TokenTree) -> bool {
    group_peek_helper(tree, Delimiter::Bracket)
}

fn group_peek_helper(tree: &TokenTree, check: Delimiter) -> bool {
    if let TokenTree::Group(synix_lexer::group::Group { delimiter, .. }) = tree {
        delimiter == &check
    } else {
        false
    }
}

fn punct_peek_helper(tree: &TokenTree, char: Char) -> bool {
    if let TokenTree::Punct(Punct { ch, .. }) = tree {
        ch == &char
    } else {
        false
    }
}

pub fn parse(input: &str) -> Result<Expr> {
    let lexed = TokenStream::from_str(input)?;
    let mut buffer = ParseBuffer::new(lexed.as_ref());
    buffer.parse()
}

#[expect(non_snake_case)]
pub const fn Ident(tree: &TokenTree) -> bool {
    matches!(tree, TokenTree::Ident(_))
}

#[derive(Debug)]
pub enum Expr {
    Let(ExprLet),
    Lit(ExprLit),
    Lambda(ExprLambda),
    Ident(Ident),
}

impl Expr {
    pub fn span(&self) -> Span {
        match self {
            Expr::Let(expr_let) => expr_let.span(),
            Expr::Lit(expr_lit) => expr_lit.span(),
            Expr::Lambda(expr_lambda) => expr_lambda.span(),
            Expr::Ident(ident) => ident.span(),
        }
    }
}

impl Parse for Expr {
    fn parse(input: &mut ParseBuffer) -> Result<Self> {
        let output = if ExprLit::peek(input) {
            let lit = input.parse()?;
            Self::Lit(lit)
        } else if ExprLet::peek(input) {
            let let_ = input.parse()?;
            Self::Let(let_)
        } else if ExprLambda::peek(input) {
            let lambda = input.parse()?;
            Self::Lambda(lambda)
        } else if input.peek(Ident) {
            let ident = input.parse()?;
            Self::Ident(ident)
        } else {
            return Err(Error::new(input.span(), "Expected expr."));
        };

        if input.len() != 0 {
            return Err(Error::new(input.span(), "Leftover tokens."));
        }

        Ok(output)
    }
}

#[derive(Debug, Clone)]
pub struct ParseBuffer<'a> {
    trees: &'a [TokenTree],
    last_span: Option<Span>,
}

impl<'a> ParseBuffer<'a> {
    pub fn new(trees: &'a [TokenTree]) -> Self {
        Self {
            trees,
            last_span: None,
        }
    }

    pub fn span(&self) -> Span {
        self.trees
            .get(0)
            .map(|t| t.span())
            .or(self.last_span.clone())
            .unwrap_or(Span::default())
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

    pub fn peek(&self, f: fn(&'a TokenTree) -> bool) -> bool {
        if let Some(tree) = self.peek_tree() {
            f(tree)
        } else {
            false
        }
    }
}

impl<'a> Iterator for ParseBuffer<'a> {
    type Item = &'a TokenTree;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(current) = self.trees.get(0) {
            self.trees = &self.trees[1..];
            self.last_span = Some(current.span());
            Some(current)
        } else {
            self.last_span = self.last_span.clone().map(|s| Span::new(s.end(), s.end()));
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
