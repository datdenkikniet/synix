use synix_lexer::Span;
use synix_lexer::TokenTree;
use synix_lexer::literal::Literal;
pub use synix_lexer::literal::{LitFloat, LitInt, LitStr};

use crate::Error;
use crate::Parse;
use crate::Peek;

#[derive(Debug)]
pub enum ExprLit {
    Int(LitInt),
    Float(LitFloat),
    Str(LitStr),
    Bool(LitBool),
}

impl Parse for ExprLit {
    fn parse(buffer: &mut crate::ParseBuffer) -> crate::Result<Self> {
        let next = if buffer.peek_tree().is_some() {
            buffer.next().expect("There's a tree")
        } else {
            return Err(Error::new(
                buffer.span(),
                "Expected literal, got end of input",
            ));
        };

        let output = match next {
            TokenTree::Literal(Literal::Int(int)) => Self::Int(int.clone()),
            TokenTree::Literal(Literal::Float(float)) => Self::Float(float.clone()),
            TokenTree::Literal(Literal::Str(str)) => Self::Str(str.clone()),
            TokenTree::Ident(ident) if ident.ident() == "true" || ident.ident() == "false" => {
                Self::Bool(LitBool {
                    span: ident.span.clone(),
                    value: ident.ident() == "true",
                })
            }
            v => return Err(Error::new(v.span(), "Expected literal.")),
        };

        Ok(output)
    }
}

impl Peek for ExprLit {
    fn peek(input: &crate::ParseBuffer) -> bool {
        Self::parse(&mut input.fork()).is_ok()
    }
}

#[derive(Debug)]
pub struct LitBool {
    pub span: Span,
    pub value: bool,
}
