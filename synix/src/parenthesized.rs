use synix_lexer::Span;

use crate::{Expr, Paren, Parse, ParseBuffer, Peek, Result, parenthesized};

#[derive(Debug)]
pub struct ExprParenthesized {
    pub inner: Box<Expr>,
    span: Span,
}

impl ExprParenthesized {
    pub fn span(&self) -> Span {
        self.span.clone()
    }

    pub fn into_inner(self) -> Expr {
        *self.inner
    }
}

impl From<ExprParenthesized> for Expr {
    fn from(value: ExprParenthesized) -> Self {
        value.into_inner()
    }
}

impl Peek for ExprParenthesized {
    fn peek(input: &ParseBuffer) -> bool {
        input.peek(Paren)
    }
}

impl Parse for ExprParenthesized {
    fn parse(buffer: &mut ParseBuffer) -> Result<Self> {
        let mut paren;
        parenthesized!(buffer as paren else "Expected parenthesized expression.");

        let span = paren.span();
        let inner = Box::new(paren.parse()?);

        Ok(Self { inner, span })
    }
}
