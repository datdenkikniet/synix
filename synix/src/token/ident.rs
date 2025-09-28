use synix_lexer::{Span, TokenTree};

use crate::{Error, Parse, ParseBuffer, Peek, Result};

fn ident_helper(buffer: &mut ParseBuffer, name: &str) -> Result<Span> {
    let next = buffer.next();
    let ident = if let Some(TokenTree::Ident(ident)) = next {
        ident
    } else {
        let msg = format!("Expected `{name}`");
        let span = next.map(|v| v.span()).unwrap_or(buffer.span());
        return Err(Error::new(span, msg));
    };

    if ident.ident() == name {
        Ok(ident.span())
    } else {
        let msg = format!("Expected `{name}`, got {}", ident.ident());
        Err(Error::new(ident.span(), msg))
    }
}

macro_rules! keyword {
    ($($ty:ident = $keyword:literal)*) => {
        $(
            #[derive(Debug, Clone, Default)]
            pub struct $ty {
                pub span: Span,
            }

            impl $ty {
                pub const fn new() -> Self {
                    Self { span: Span::default() }
                }
            }

            impl Parse for $ty {
                fn parse(buffer: &mut ParseBuffer) -> Result<Self> {
                    let span = ident_helper(buffer, $keyword)?;
                    Ok(Self { span })
                }
            }

            impl Peek for $ty {
                fn peek(buffer: &ParseBuffer) -> bool {
                    Self::parse(&mut buffer.fork()).is_ok()
                }
            }
        )*
    };
}

keyword! {
    Let = "let"
    In = "in"
    Inherit = "inherit"
    With = "with"
    Rec = "rec"
}
