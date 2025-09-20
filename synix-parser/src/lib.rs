mod attrset;
mod ident;
mod r#let;
mod span;

pub mod binary;
pub mod lit;

pub use attrset::AttrSet;
pub use ident::Ident;
pub use r#let::LetExpr;

use binary::BinaryExpr;
use lit::Lit;
use span::Span;

use crate::binary::Operator;

pub fn parse(input: &str) -> Result<Expr, syn::Error> {
    syn::parse_str(input)
}

pub trait Spanned {
    fn span(&self) -> Span;
}

#[derive(Debug)]
pub enum Expr {
    Let(LetExpr),
    Lit(Lit),
    Ident(Ident),
    AttrSet(AttrSet),
    Binary(BinaryExpr),
}

impl Spanned for Expr {
    fn span(&self) -> Span {
        match self {
            Expr::Let(let_expr) => let_expr.span(),
            Expr::Lit(lit) => lit.span(),
            Expr::Ident(ident) => ident.span(),
            Expr::AttrSet(attr_set) => attr_set.span(),
            Expr::Binary(binary_expr) => binary_expr.span(),
        }
    }
}

impl syn::parse::Parse for Expr {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        use syn::{Token, token};

        let result = if input.peek(Token![let]) {
            let r#let: LetExpr = input.parse()?;
            Self::Let(r#let)
        } else if input.peek(token::Brace) {
            let attrset: AttrSet = input.parse()?;
            Self::AttrSet(attrset)
        } else if input.peek(syn::Ident) {
            let ident: Ident = input.parse()?;
            Self::Ident(ident)
        } else if input.peek(syn::Lit) {
            let literal: Lit = input.parse()?;
            Self::Lit(literal)
        } else {
            return Err(syn::Error::new(
                input.span(),
                "Unexpected tokens in input. Expected expression",
            ));
        };

        let after_operation = if let Some(operator) = Operator::peek_parse(input) {
            let lhs = result;
            let rhs: Expr = input.parse()?;

            Self::Binary(BinaryExpr::new(lhs, operator, rhs))
        } else {
            result
        };

        Ok(after_operation)
    }
}
