use crate::{Spanned, span::Span};

#[derive(Debug, Clone)]
pub enum Lit {
    Bool(LitBool),
    String(LitStr),
    Int(LitInt),
    Float(LitFloat),
}

impl Spanned for Lit {
    fn span(&self) -> Span {
        match self {
            Lit::Bool(lit_bool) => lit_bool.span(),
            Lit::String(lit_str) => lit_str.span(),
            Lit::Int(lit_int) => lit_int.span(),
            Lit::Float(lit_float) => lit_float.span(),
        }
    }
}

impl syn::parse::Parse for Lit {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let literal: syn::Lit = input.parse()?;
        let span = literal.span();

        let literal = match literal {
            syn::Lit::Str(lit_str) => Self::String(LitStr {
                value: lit_str.value(),
                span: span.into(),
                proc_macro_span: span,
            }),
            syn::Lit::Int(lit_int) => Self::Int(LitInt {
                digits: lit_int.base10_digits().to_string(),
                span: span.into(),
                proc_macro_span: span,
            }),
            syn::Lit::Float(lit_float) => Self::Float(LitFloat {
                digits: lit_float.base10_digits().to_string(),
                span: span.into(),
                proc_macro_span: span,
            }),
            syn::Lit::Bool(lit_bool) => Self::Bool(LitBool {
                value: lit_bool.value(),
                span: span.into(),
                proc_macro_span: span,
            }),
            _ => return Err(syn::Error::new(literal.span(), "Unsupported literal type.")),
        };

        Ok(literal)
    }
}

macro_rules! literal {
    ($($name:ident, $value_name:ident = $value:ty),*$(,)?) => {
        $(
            #[derive(Debug, Clone)]
            pub struct $name {
                pub $value_name: $value,
                pub span: Span,
                pub proc_macro_span: proc_macro2::Span,
            }

            impl $name {
                pub fn proc_macro_span(&self) -> proc_macro2::Span {
                    self.proc_macro_span.clone()
                }
            }

            impl Spanned for $name {
                fn span(&self) -> Span {
                    self.span.clone()
                }
            }
        )*
    };
}

literal! {
    LitBool, value = bool,
    LitStr, value = String,
    LitInt, digits = String,
    LitFloat, digits = String,
}
