use syn::Ident;

use crate::{Expr, Spanned, span::Span};

#[derive(Debug)]
pub struct LetExpr {
    pub assignments: Vec<(Ident, Expr)>,
    pub in_: Box<Expr>,
    pub span: Span,
}

impl syn::parse::Parse for LetExpr {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let span_start: Span = input.span().into();

        use syn::{Ident, Token};

        let _let: Token![let] = input.parse()?;

        let mut statements = Vec::new();
        while let Ok(ident) = input.parse::<Ident>() {
            let _equals: Token![=] = input.parse()?;
            let value: Expr = input.parse()?;
            let _semicolon: Token![;] = input.parse()?;

            statements.push((ident, value));
        }

        let _in: Token![in] = input.parse()?;

        let in_ = input.parse()?;

        let span_end: Span = input.span().into();

        let span = span_start.join(&span_end);

        Ok(Self {
            assignments: statements,
            in_,
            span,
        })
    }
}

impl Spanned for LetExpr {
    fn span(&self) -> crate::span::Span {
        self.span.clone()
    }
}
