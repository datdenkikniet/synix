use synix_lexer::Span;

use crate::Expr;

#[derive(Debug)]
pub struct ExprFunctionCall {
    pub head: Box<Expr>,
    pub tail: Box<Expr>,
    pub(crate) span: Span,
}

impl ExprFunctionCall {
    pub fn span(&self) -> Span {
        self.span.clone()
    }
}
