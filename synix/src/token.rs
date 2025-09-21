use crate::Peek;

macro_rules! tokens {
    ($($lit:tt => $token:ident),*$(,)?) => {
        $(
            #[derive(Debug, Default, Clone, Copy, PartialEq)]
            pub struct $token;

            impl Token for $token {}
        )*

        #[macro_export]
        macro_rules ! Token {
            $(
                ($lit) => {
                    $crate::token::$token
                };
            )*
            (let) => {
                $crate::token::Let
            }
        }
    };
}

pub trait Token {}

tokens!(
    ; => Semicolon, : => Colon, , => Comma, . => Dot, @ => At, + => Plus, - => Minus, * => Asterisk, > => Gt, >= => GtE, < => Lt, <= => LtE, = => Equal, ? => Question,
    & => Ampersand, | => Pipe, / => Slash
);

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Let;

impl Token for Let {}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Brace;

impl Token for Brace {}

impl Peek for Brace {
    fn peek(buffer: &crate::ParseBuffer) -> bool {
        todo!()
    }
}
