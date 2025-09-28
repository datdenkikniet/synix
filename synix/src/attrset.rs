use crate::ident::LiteralOrInterpolatedIdent;
use crate::*;
use crate::{Brace, Expr, Parse, Peek, braced};

#[derive(Debug)]
pub struct ExprAttrSet {
    pub rec: Option<Token![rec]>,
    pub assignments: Vec<Assignment>,
    span: Span,
}

impl ExprAttrSet {
    pub fn span(&self) -> Span {
        self.span.clone()
    }
}

impl Peek for ExprAttrSet {
    fn peek(input: &crate::ParseBuffer) -> bool {
        input.peek(Brace) || <Token![rec]>::peek(input)
    }
}

impl Parse for ExprAttrSet {
    fn parse(buffer: &mut crate::ParseBuffer) -> crate::Result<Self> {
        let rec = if <Token![rec]>::peek(buffer) {
            Some(buffer.parse()?)
        } else {
            None
        };

        let mut braced;
        braced!(buffer as braced else "Expected attribute set.");

        let span = braced.span();
        let mut assignments = Vec::new();
        while !braced.is_empty() {
            assignments.push(braced.parse()?);
        }

        Ok(Self {
            rec,
            assignments,
            span,
        })
    }
}

#[derive(Debug)]
pub struct AttributeAccess {
    pub set: Expr,
    pub accessors: Vec<LiteralOrInterpolatedIdent>,
}

impl AttributeAccess {
    pub fn span(&self) -> Span {
        let mut span = self.set.span();

        for v in &self.accessors {
            span = span.join(&v.span());
        }

        span
    }

    pub fn peek(buffer: &ParseBuffer) -> bool {
        <Token![.]>::peek(buffer)
    }

    pub fn parse_rest(set: Expr, parser: &mut ParseBuffer) -> Result<Self> {
        if !Self::peek(&parser) {
            return Err(Error::new(
                parser.span(),
                "Expected `.` while parsing attribute access",
            ));
        }

        let mut accessors = Vec::new();

        while <Token![.]>::peek(parser) {
            let _dot: Token![.] = parser.parse()?;
            let accessor = parser.parse()?;

            accessors.push(accessor);
        }

        Ok(Self { set, accessors })
    }
}
