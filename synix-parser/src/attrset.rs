use std::collections::BTreeMap;

use crate::{Expr, Ident};

#[derive(Debug)]
pub struct AttrSet {
    pub key_values: BTreeMap<Ident, Expr>,
}

impl syn::parse::Parse for AttrSet {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        use syn::Token;

        let key_values;
        syn::braced!(key_values in input);

        let mut output = BTreeMap::new();
        while !key_values.is_empty() {
            let ident: Ident = key_values.parse()?;

            let (ident, expr) = if ident == "inherit" {
                let inherit_ident: Ident = key_values.parse()?;
                let _semi: Token![;] = key_values.parse()?;

                (inherit_ident.clone(), Expr::Ident(inherit_ident))
            } else {
                let _equals: Token![=] = key_values.parse()?;
                let expr: Expr = key_values.parse()?;
                let _semi: Token![;] = key_values.parse()?;

                (ident, expr)
            };

            if output.contains_key(&ident) {
                let message = format!("Duplicate key `{}` in attribute set.", ident);
                return Err(syn::Error::new(ident.span(), message));
            }

            assert!(output.insert(ident, expr).is_none());
        }

        Ok(Self { key_values: output })
    }
}
