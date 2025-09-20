use proc_macro2::Span;

#[derive(Debug, Clone)]
pub enum Lit {
    Bool(LitBool),
    String(LitStr),
    Int(LitInt),
    Float(LitFloat),
}

impl syn::parse::Parse for Lit {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let literal: syn::Lit = input.parse()?;
        let span = literal.span();

        let literal = match literal {
            syn::Lit::Str(lit_str) => Self::String(LitStr {
                value: lit_str.value(),
                span,
            }),
            syn::Lit::Int(lit_int) => Self::Int(LitInt {
                digits: lit_int.base10_digits().to_string(),
                span,
            }),
            syn::Lit::Float(lit_float) => Self::Float(LitFloat {
                digits: lit_float.base10_digits().to_string(),
                span,
            }),
            syn::Lit::Bool(lit_bool) => Self::Bool(LitBool {
                value: lit_bool.value(),
                span: span,
            }),
            _ => return Err(syn::Error::new(literal.span(), "Unsupported literal type.")),
        };

        Ok(literal)
    }
}

#[derive(Debug, Clone)]
pub struct LitBool {
    pub value: bool,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct LitStr {
    pub value: String,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct LitInt {
    pub digits: String,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct LitFloat {
    pub digits: String,
    pub span: Span,
}
