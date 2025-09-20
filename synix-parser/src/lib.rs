mod attrset;
mod r#let;
mod ident;

pub mod lit;

pub use attrset::AttrSet;
pub use r#let::LetExpr;
pub use ident::Ident;

use lit::Lit;

pub fn parse(input: &str) -> Result<Expr, syn::Error> {
    syn::parse_str(input)
}

#[derive(Debug)]
pub enum Expr {
    Let(LetExpr),
    Lit(Lit),
    Ident(Ident),
    AttrSet(AttrSet),
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

        Ok(result)
    }
}
