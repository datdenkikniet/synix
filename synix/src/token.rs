macro_rules! tokens {
    ($($token:ident),*$(,)?) => {
        $(
            #[derive(Debug, Default, Clone, Copy, PartialEq)]
            pub struct $token;

            impl Token for $token {}
        )*
    };
}

pub trait Token {}

tokens!(
    Semicolon, Colon, Comma, Dot, At, Plus, Minus, Asterisk, Gt, GtE, Lt, LtE, Equal, Question,
    Ampersand, Pipe, Slash
);
