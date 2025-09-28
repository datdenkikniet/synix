use crate::{ident::LiteralOrInterpolatedIdent, *};

#[derive(Debug)]
pub enum Assignment {
    Inherit(AssignmentInherit),
    Named(AssignmentNamed),
}

impl Assignment {
    pub fn span(&self) -> Span {
        match self {
            Assignment::Inherit(v) => v.span(),
            Assignment::Named(v) => v.span(),
        }
    }
}

impl Parse for Assignment {
    fn parse(buffer: &mut ParseBuffer) -> Result<Self> {
        let result = if AssignmentInherit::peek(buffer) {
            let inherit = buffer.parse()?;
            Self::Inherit(inherit)
        } else {
            let assignment = buffer.parse()?;
            Self::Named(assignment)
        };

        Ok(result)
    }
}

#[derive(Debug)]
pub struct AssignmentInherit {
    pub inherit: Token![inherit],
    pub base: Option<Expr>,
    pub names: Vec<Ident>,
    pub semicolon: Token![;],
    span: Span,
}

impl AssignmentInherit {
    pub fn span(&self) -> Span {
        self.span.clone()
    }
}

impl Peek for AssignmentInherit {
    fn peek(input: &ParseBuffer) -> bool {
        <Token![inherit]>::peek(input)
    }
}

impl Parse for AssignmentInherit {
    fn parse(buffer: &mut ParseBuffer) -> Result<Self> {
        use LiteralOrInterpolatedIdent::*;

        let start = buffer.span();
        let inherit = buffer.parse()?;

        let inner = &mut buffer.until::<Token![;]>();

        let base = if inner.peek(Paren) {
            let mut expr;
            parenthesized!(inner as expr else "Expected parenthesized expression");
            Some(expr.parse()?)
        } else {
            None
        };

        let mut names = Vec::new();
        while !ParseBuffer::is_empty(inner) {
            let ident = match LiteralOrInterpolatedIdent::parse(inner)? {
                Literal(ident) => ident,
                Interpolated(_) => {
                    let msg = "Interpolated identifiers not allowed in this context";
                    return Err(Error::new(buffer.span(), msg));
                }
            };

            names.push(ident);
        }

        let semicolon = buffer.parse()?;

        let span = start.join(&buffer.span());

        Ok(Self {
            inherit,
            base,
            names,
            span,
            semicolon,
        })
    }
}

#[derive(Debug)]
pub struct AssignmentNamed {
    pub head: LiteralOrInterpolatedIdent,
    pub tail: Vec<LiteralOrInterpolatedIdent>,
    pub eq: Token![=],
    pub value: Expr,
    pub semicolon: Token![;],
    span: Span,
}

impl AssignmentNamed {
    pub fn span(&self) -> Span {
        self.span.clone()
    }
}

impl Parse for AssignmentNamed {
    fn parse(buffer: &mut ParseBuffer) -> Result<Self> {
        let start = buffer.span();
        let inner = &mut buffer.until::<Token![;]>();
        let head = inner.parse()?;

        let mut tail = Vec::new();
        while <Token![.]>::peek(inner) {
            let _ = <Token![.]>::parse(inner)?;
            tail.push(inner.parse()?);
        }

        let eq = inner.parse()?;
        let value = inner.parse()?;

        let semicolon = buffer.parse()?;

        let span = start.join(&buffer.span());

        Ok(Self {
            head,
            tail,
            eq,
            value,
            semicolon,
            span,
        })
    }
}
