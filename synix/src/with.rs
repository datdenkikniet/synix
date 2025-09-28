use synix_lexer::Span;

use crate::*;

#[derive(Debug)]
pub struct ExprWith {
    pub with: Token![with],
    pub expr: Expr,
    pub semicolon: Token![;],
    pub body: Expr,
}

impl ExprWith {
    pub fn span(&self) -> Span {
        self.with
            .span
            .join(&self.expr.span())
            .join(&self.semicolon.span)
            .join(&self.body.span())
    }
}

impl Peek for ExprWith {
    fn peek(input: &ParseBuffer) -> bool {
        <Token![with]>::peek(input)
    }
}

impl Parse for ExprWith {
    fn parse(buffer: &mut ParseBuffer) -> Result<Self> {
        let with = buffer.parse()?;

        // TODO: probably delimit by ; and paren or something
        let expr = buffer.parse()?;

        let semicolon = buffer.parse()?;
        let body = buffer.parse()?;

        Ok(Self {
            with,
            expr,
            semicolon,
            body,
        })
    }
}
