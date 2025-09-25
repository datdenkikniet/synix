use std::str::FromStr;

pub mod attrset;
pub mod binary;
pub mod list;
pub mod lit;
pub mod token;

mod error;
mod function_call;
mod ident;
mod lambda;
mod r#let;
mod parenthesized;
mod with;

pub use error::Error;
pub use function_call::ExprFunctionCall;
pub use ident::Ident;
pub use lambda::ExprLambda;
pub use r#let::ExprLet;
pub use parenthesized::ExprParenthesized;
use synix_lexer::{
    Span, TokenStream, TokenTree,
    group::Delimiter,
    punct::{Char, Punct},
};
pub use with::ExprWith;

use crate::{
    attrset::ExprAttrSet,
    binary::{ExprBinary, Operator},
    list::ExprList,
    lit::ExprLit,
};
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
    AttrSet(ExprAttrSet),
    Parenthesized(ExprParenthesized),
    List(ExprList),
    With(ExprWith),
    FunctionCall(ExprFunctionCall),
    Binary(ExprBinary),
}

impl Expr {
    pub fn span(&self) -> Span {
        match self {
            Expr::Let(expr_let) => expr_let.span(),
            Expr::Lit(expr_lit) => expr_lit.span(),
            Expr::Lambda(expr_lambda) => expr_lambda.span(),
            Expr::Ident(ident) => ident.span(),
            Expr::AttrSet(attr_set) => attr_set.span(),
            Expr::Parenthesized(paren) => paren.span(),
            Expr::List(expr_list) => expr_list.span(),
            Expr::With(expr_with) => expr_with.span(),
            Expr::FunctionCall(expr_function_call) => expr_function_call.span(),
            Expr::Binary(expr_binary) => expr_binary.span(),
        }
    }
}

impl Parse for Expr {
    fn parse(input: &mut ParseBuffer) -> Result<Self> {
        let start = input.span();

        let output = if ExprLit::peek(input) {
            let lit = input.parse()?;
            Self::Lit(lit)
        } else if ExprLet::peek(input) {
            let let_ = input.parse()?;
            Self::Let(let_)
        } else if ExprLambda::peek(input) {
            let lambda = input.parse()?;
            Self::Lambda(lambda)
        } else if ExprAttrSet::peek(input) {
            let attrset = input.parse()?;
            Self::AttrSet(attrset)
        } else if ExprParenthesized::peek(input) {
            let parenthesized = input.parse()?;
            Self::Parenthesized(parenthesized)
        } else if ExprList::peek(input) {
            let list = input.parse()?;
            Self::List(list)
        } else if ExprWith::peek(input) {
            let with = input.parse()?;
            Self::With(with)
        } else if input.peek(Ident) {
            let ident = input.parse()?;
            Self::Ident(ident)
        } else {
            return Err(Error::new(input.span(), "Expected expr."));
        };

        let result = if ParseBuffer::is_empty(input) {
            output
        } else {
            if Operator::peek(input) {
                let operator = input.parse()?;

                let rhs = input.parse()?;
                let span = start.join(&input.span());

                Self::Binary(ExprBinary {
                    lhs: Box::new(output),
                    operator,
                    rhs: Box::new(rhs),
                    span,
                })
            } else {
                let body = input.parse()?;
                let span = start.join(&input.span());

                // Reorder to give function calls higher
                // precedence than binary expressions.
                if let Expr::Binary(ExprBinary {
                    lhs,
                    operator,
                    rhs,
                    span: _,
                }) = body
                {
                    let function_call = ExprFunctionCall {
                        head: Box::new(output),
                        tail: lhs,
                        // TODO: compute this
                        span: Default::default(),
                    };

                    Self::Binary(ExprBinary {
                        lhs: Box::new(Self::FunctionCall(function_call)),
                        operator,
                        rhs,
                        // TODO: compute this
                        span: Default::default(),
                    })
                } else {
                    Self::FunctionCall(ExprFunctionCall {
                        head: Box::new(output),
                        tail: Box::new(body),
                        span,
                    })
                }
            }
        };

        if let Expr::Binary(binary) = result {
            Ok(binary.fix_presedence())
        } else {
            Ok(result)
        }
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

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    // TODO: make this less scan-y
    pub fn until<'b, P: Peek>(&'b mut self) -> ParseBuffer<'b> {
        let forked = self.fork();
        let mut len = 0;

        while !P::peek(self) {
            len += 1;
            self.next();
        }

        Self {
            trees: &forked.trees[..len],
            last_span: None,
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

#[macro_export]
macro_rules! delimited {
    ($buffer:ident, $value:ident, $err:expr, $delim:ident) => {
        match $buffer.next() {
            Some(::synix_lexer::TokenTree::Group(group))
                if group.delimiter == synix_lexer::group::Delimiter::$delim =>
            {
                $value = crate::ParseBuffer::new(group.inner.as_ref());
                let _discard = $buffer;
            }
            v => {
                let span = v.map(|v| v.span()).unwrap_or($buffer.span());
                return Err(crate::Error::new(span, $err));
            }
        }
    };
}

#[macro_export]
macro_rules! bracketed {
    ($buffer:ident as $value:ident else $err:expr) => {
        $crate::delimited!($buffer, $value, $err, Bracket)
    };
}

#[macro_export]
macro_rules! braced {
    ($buffer:ident as $value:ident else $err:expr) => {
        $crate::delimited!($buffer, $value, $err, Brace)
    };
}

#[macro_export]
macro_rules! parenthesized {
    ($buffer:ident as $value:ident else $err:expr) => {
        $crate::delimited!($buffer, $value, $err, Paren)
    };
}
