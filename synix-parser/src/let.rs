use syn::Ident;

use crate::Expr;

#[derive(Debug)]
pub struct LetExpr {
    pub statements: Vec<(Ident, Expr)>,
    pub in_: Box<Expr>,
}

impl syn::parse::Parse for LetExpr {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
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

        Ok(Self { statements, in_ })
    }
}
