use crate::*;
use crate::{Brace, Expr, Ident, Parse, Peek, braced};

#[derive(Debug)]
pub struct ExprAttrSet {
    pub entries: Vec<Assignment>,
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
pub struct AttributeAccess {
    pub set: Expr,
    pub accessors: Vec<(Token![.], Ident)>,
}

impl AttributeAccess {
    pub fn span(&self) -> Span {
        let mut span = self.set.span();

        for (dot, ident) in &self.accessors {
            span = span.join(&dot.span).join(&ident.span());
        }

        span
    }

    pub fn peek(buffer: &ParseBuffer) -> bool {
        <Token![.]>::peek(buffer)
    }

    pub fn parse_rest(set: Expr, parser: &mut ParseBuffer) -> Result<Self> {
        if !Self::peek(&parser) {
            return Err(Error::new(parser.span(), "Expected `.`"));
        }

        let mut accessors = Vec::new();

        while <Token![.]>::peek(parser) {
            let dot = parser.parse()?;
            let field = parser.parse()?;

            accessors.push((dot, field));
        }

        Ok(Self { set, accessors })
    }
}
