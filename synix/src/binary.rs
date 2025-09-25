use std::cmp::Ordering;

use crate::{Expr, Parse, ParseBuffer, Result, *};

#[derive(Debug)]
pub struct ExprBinary {
    pub lhs: Box<Expr>,
    pub operator: Operator,
    pub rhs: Box<Expr>,
    pub(crate) span: Span,
}

impl ExprBinary {
    pub(crate) fn new(lhs: Expr, operator: Operator, rhs: Expr, span: Span) -> Self {
        Self {
            lhs: Box::new(lhs),
            operator,
            rhs: Box::new(rhs),
            span,
        }
    }

    pub fn span(&self) -> Span {
        self.span.clone()
    }

    pub fn parse_rest(lhs: Expr, operator: Operator, rest: &mut ParseBuffer) -> Result<Self> {
        let rhs: Expr = rest.parse()?;

        let out = if let Expr::Binary(rhs) = rhs {
            if operator.presedence(&rhs.operator) == Ordering::Greater {
                // TODO: compute span
                let lhs = ExprBinary::new(lhs, operator, *rhs.lhs, Default::default());
                let lhs = Expr::Binary(lhs);

                // TODO: compute span.
                ExprBinary::new(lhs, rhs.operator, *rhs.rhs, Default::default())
            } else {
                let rhs = Expr::Binary(rhs);
                // TODO: compute span.
                ExprBinary::new(lhs, operator, rhs, Default::default())
            }
        } else {
            // TODO: compute span.
            ExprBinary::new(lhs, operator, rhs, Default::default())
        };

        Ok(out)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Operator {
    Add,
    Subtract,
    Divide,
    Multiply,
    Update,
    And,
    Or,
    Equals,
    NotEquals,
    Gt,
    Ge,
    Lt,
    Le,
    Concat,
}

impl Operator {
    fn presedence_val(&self) -> usize {
        match self {
            // Mathematical operators
            Operator::Multiply => 4,
            Operator::Divide => 4,
            Operator::Add => 3,
            Operator::Subtract => 3,
            // Equality operators
            Operator::Equals => 2,
            Operator::NotEquals => 2,
            Operator::Gt => 2,
            Operator::Ge => 2,
            Operator::Lt => 2,
            Operator::Le => 2,
            // Binary operators
            Operator::And => 1,
            Operator::Or => 1,
            // List & attribute set operators
            Operator::Update => 0,
            Operator::Concat => 0,
        }
    }

    pub fn presedence(&self, other: &Self) -> Ordering {
        self.presedence_val().cmp(&other.presedence_val())
    }
}

impl Parse for Operator {
    fn parse(buffer: &mut ParseBuffer) -> Result<Self> {
        macro_rules! do_map {
            ($($token:ty => $value:ident),*$(,)?) => {
                $(
                    if <$token>::peek(&buffer) {
                        let _: $token = buffer.parse()?;
                        return Ok(Self::$value);
                    }
                )*
            };
        }

        do_map! {
            Token![+] => Add,
            Token![-] => Subtract,
            Token![/] => Divide,
            Token![*] => Multiply,
            crate::token::Update => Update,
            Token![&&] => And,
            Token![||] => Or,
            Token![==] => Equals,
            Token![>] => Gt,
            Token![>=] => Ge,
            Token![<] => Lt,
            Token![<=] => Le,
            crate::token::Concat => Concat,
            Token![!=] => NotEquals,
        }

        Err(Error::new(buffer.span(), "Expected binary operator."))
    }
}

impl Peek for Operator {
    fn peek(input: &ParseBuffer) -> bool {
        Self::parse(&mut input.fork()).is_ok()
    }
}
