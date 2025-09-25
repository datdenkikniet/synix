use crate::*;
use crate::{Brace, Expr, Ident, Parse, Peek, braced};

#[derive(Debug)]
pub struct ExprAttrSet {
    pub entries: Vec<AttrSetEntry>,
    span: Span,
}

impl ExprAttrSet {
    pub fn span(&self) -> Span {
        self.span.clone()
    }
}

impl Parse for ExprAttrSet {
    fn parse(buffer: &mut crate::ParseBuffer) -> crate::Result<Self> {
        let mut braced;
        braced!(buffer as braced else "Expected attribute set.");

        let span = braced.span();
        let mut entries = Vec::new();
        while !braced.is_empty() {
            entries.push(braced.parse()?);
        }

        Ok(Self { entries, span })
    }
}

impl Peek for ExprAttrSet {
    fn peek(input: &crate::ParseBuffer) -> bool {
        input.peek(Brace)
    }
}

#[derive(Debug)]
pub struct AttrSetEntry {
    pub name: Ident,
    pub eq: Token![=],
    pub value: Expr,
    pub semicolon: Token![;],
}

impl Parse for AttrSetEntry {
    fn parse(buffer: &mut ParseBuffer) -> Result<Self> {
        let name = buffer.parse()?;
        let eq = buffer.parse()?;
        let mut inner = buffer.until::<Token![;]>();
        let value = inner.parse()?;
        let semicolon = buffer.parse()?;

        Ok(Self {
            name,
            eq,
            value,
            semicolon,
        })
    }
}
