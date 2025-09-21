use crate::{Parse, ParseBuffer, Result, Spanned, lit::LitStr, span::Span};

#[derive(Debug, Clone)]
pub enum Ident {
    Ident(synix_lexer::Ident),
    Stringy(LitStr),
}

impl Spanned for Ident {
    fn span(&self) -> Span {
        match self {
            Ident::Ident(ident) => ident.span().into(),
            Ident::Stringy(lit_str) => lit_str.span(),
        }
    }
}

impl std::fmt::Display for Ident {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Ident::Ident(ident) => write!(f, "{}", ident.to_string()),
            Ident::Stringy(lit_str) => write!(f, "{}", lit_str.value),
        }
    }
}

impl PartialEq for Ident {
    fn eq(&self, other: &Self) -> bool {
        self.to_string().eq(&other.to_string())
    }
}

impl Eq for Ident {}

impl PartialOrd for Ident {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.to_string().cmp(&other.to_string()))
    }
}

impl Ord for Ident {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.to_string().cmp(&other.to_string())
    }
}

impl PartialEq<&str> for Ident {
    fn eq(&self, other: &&str) -> bool {
        match self {
            Ident::Ident(ident) => ident == other,
            Ident::Stringy(ident) => &ident.value == other,
        }
    }
}

impl Parse for Ident {
    fn parse(input: &mut ParseBuffer) -> Result<Self> {
        let result = if input.peek(syn::Ident) {
            let ident: syn::Ident = input.parse()?;
            Self::Ident(ident)
        } else if input.peek(syn::Lit) {
            let lit_str: syn::LitStr = if let Ok(lit) = input.parse() {
                lit
            } else {
                return Err(syn::Error::new(input.span(), "Expected ident"));
            };

            Self::Stringy(LitStr {
                value: lit_str.value(),
                span: lit_str.span().into(),
                proc_macro_span: lit_str.span(),
            })
        } else {
            return Err(syn::Error::new(input.span(), "Expected ident"));
        };

        Ok(result)
    }
}
