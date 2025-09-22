use synix_lexer::{Span, TokenTree};

use crate::{Error, Parse, Peek};

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
