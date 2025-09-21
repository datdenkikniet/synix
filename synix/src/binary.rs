use crate::{Expr, Spanned, span::Span};

#[derive(Debug)]
pub struct BinaryExpr {
    pub lhs: Box<Expr>,
    pub operator: Operator,
    pub rhs: Box<Expr>,
    pub span: Span,
}

impl BinaryExpr {
    pub fn new(lhs: Expr, operator: Operator, rhs: Expr) -> Self {
        let span = lhs.span().join(&rhs.span());

        Self {
            lhs: Box::new(lhs),
            operator,
            rhs: Box::new(rhs),
            span,
        }
    }
}

impl Spanned for BinaryExpr {
    fn span(&self) -> Span {
        self.span.clone()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Operator {
    Addition,
    Subtraction,
}

impl Operator {
    pub fn peek_parse(input: syn::parse::ParseStream) -> Option<Self> {
        use syn::Token;

        macro_rules! map {
            ($token:tt => $value:ident) => {
                if input.peek(Token![$token]) {
                    let _: Token![$token] = input.parse().expect("Token we just peeked.");
                    return Some(Self::$value);
                }
            };
        }

        map!(+ => Addition);
        map!(- => Subtraction);

        None
    }
}
