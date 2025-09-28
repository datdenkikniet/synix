use synix_lexer::{Span, TokenTree};

use crate::*;

// TODO: do all usages of this also support strings?
#[derive(Debug)]
pub enum LiteralOrInterpolatedIdent {
    Literal(Ident),
    Interpolated(InterpolatedIdent),
}

impl LiteralOrInterpolatedIdent {
    pub fn span(&self) -> Span {
        match self {
            LiteralOrInterpolatedIdent::Literal(ident) => ident.span(),
            LiteralOrInterpolatedIdent::Interpolated(interpolated_ident) => {
                interpolated_ident.span()
            }
        }
    }
}

impl Peek for LiteralOrInterpolatedIdent {
    fn peek(input: &ParseBuffer) -> bool {
        Ident::peek(input) || InterpolatedIdent::peek(input)
    }
}

impl Parse for LiteralOrInterpolatedIdent {
    fn parse(buffer: &mut ParseBuffer) -> Result<Self> {
        if Ident::peek(buffer) {
            Ok(Self::Literal(buffer.parse()?))
        } else if InterpolatedIdent::peek(buffer) {
            Ok(Self::Interpolated(buffer.parse()?))
        } else {
            Err(Error::new(
                buffer.span(),
                "Expected literal or interpolated ident",
            ))
        }
    }
}

#[derive(Debug)]
pub struct Ident {
    inner: synix_lexer::Ident,
}

impl Ident {
    pub fn ident(&self) -> &str {
        self.inner.ident()
    }

    pub fn span(&self) -> Span {
        self.inner.span()
    }
}

impl Parse for Ident {
    fn parse(buffer: &mut crate::ParseBuffer) -> crate::Result<Self> {
        // TODO: warn on un-handy idents?
        if let Some(TokenTree::Ident(ident)) = buffer.peek_tree() {
            let _ = buffer.next();
            Ok(Self {
                inner: ident.clone(),
            })
        } else {
            Err(Error::new(buffer.span(), "Expected ident"))
        }
    }
}

impl Peek for Ident {
    fn peek(input: &crate::ParseBuffer) -> bool {
        Ident::parse(&mut input.fork()).is_ok()
    }
}

#[derive(Debug)]
pub struct InterpolatedIdent {
    pub dollar: Token![$],
    pub value: Expr,
    span: Span,
}

impl InterpolatedIdent {
    pub fn span(&self) -> Span {
        self.span.clone()
    }
}

impl Peek for InterpolatedIdent {
    fn peek(input: &crate::ParseBuffer) -> bool {
        <Token![$]>::peek(input)
    }
}

impl Parse for InterpolatedIdent {
    fn parse(buffer: &mut ParseBuffer) -> Result<Self> {
        let span = buffer.span();
        let dollar = buffer.parse()?;

        let mut inner;
        braced!(buffer as inner else "Expected braced expression.");

        let value = inner.parse()?;

        if !inner.is_empty() {
            return Err(Error::new(inner.span(), "Leftover tokens."));
        }

        let span = inner.span().join(&span);

        Ok(Self {
            dollar,
            value,
            span,
        })
    }
}
