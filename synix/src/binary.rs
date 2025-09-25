use crate::{Expr, Parse, ParseBuffer, Result, *};

#[derive(Debug)]
pub struct ExprBinary {
    pub lhs: Box<Expr>,
    pub operator: Operator,
    pub rhs: Box<Expr>,
    pub(crate) span: Span,
}

impl ExprBinary {
    pub fn span(&self) -> Span {
        self.span.clone()
    }
}

#[derive(Debug)]
pub enum Operator {
    Add,
    Subtract,
    Divide,
    Multiply,
    Update,
    And,
    Or,
    Equals,
    Gt,
    Ge,
    Lt,
    Le,
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
        }

        Err(Error::new(buffer.span(), "Expected binary operator."))
    }
}

impl Peek for Operator {
    fn peek(input: &ParseBuffer) -> bool {
        Self::parse(&mut input.fork()).is_ok()
    }
}
