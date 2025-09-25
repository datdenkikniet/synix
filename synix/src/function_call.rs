use synix_lexer::Span;

use crate::Expr;

#[derive(Debug)]
pub struct ExprFunctionCall {
    pub head: Expr,
    pub tail: Expr,
    pub(crate) span: Span,
}

impl ExprFunctionCall {
    pub fn span(&self) -> Span {
        self.span.clone()
    }
}
