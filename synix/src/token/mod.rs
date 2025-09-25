mod delimiter;
mod ident;
mod punct;

pub use delimiter::*;
pub use ident::*;
pub use punct::*;

#[macro_export]
#[rustfmt::skip]
macro_rules ! Token {
    (;) => { $crate::token::Semicolon };
    (:) => { $crate::token::Colon };
    (,) => { $crate::token::Comma };
    (.) => { $crate::token::Dot };
    (@) => { $crate::token::At };
    (+) => { $crate::token::Plus };
    (-) => { $crate::token::Minus };
    (*) => { $crate::token::Asterisk };
    (>) => { $crate::token::Gt };
    (<) => { $crate::token::Lt };
    (=) => { $crate::token::Equals };
    (?) => { $crate::token::Question };
    (&) => { $crate::token::Ampersand };
    (|) => { $crate::token::Pipe };
    (/) => { $crate::token::Slash };
    (...) => { $crate::token::Ellipsis };
    (>=) => { $crate::token::Ge };
    (<=) => { $crate::token::Le };
    (==) => { $crate::token::EqualsEquals };
    (!=) => { $crate::token::NotEquals };
    (&&) => { $crate::token::And };
    (||) => { $crate::token::Or };
    (++) => { $crate::token::Concat };
    (let) => { $crate::token::Let };
    (in) => { $crate::token::In };
    (with) => { $crate::token::With };
    (inherit) => { $crate::token::Inherit };
}

#[cfg(test)]
#[expect(dead_code)]
mod test {
    use crate::token::{Brace, Bracket, Paren, Update};

    const SEMICOLON: Token![;] = <Token![;]>::new();
    const COLON: Token![:] = <Token![:]>::new();
    const COMMA: Token![,] = <Token![,]>::new();
    const DOT: Token![.] = <Token![.]>::new();
    const AT: Token![@] = <Token![@]>::new();
    const PLUS: Token![+] = <Token![+]>::new();
    const MINUS: Token![-] = <Token![-]>::new();
    const ASTERISK: Token![*] = <Token![*]>::new();
    const GT: Token![>] = <Token![>]>::new();
    const LT: Token![<] = <Token![<]>::new();
    const EQUALS: Token![=] = <Token![=]>::new();
    const QUESTION: Token![?] = <Token![?]>::new();
    const AMPERSAND: Token![&] = <Token![&]>::new();
    const PIPE: Token![|] = <Token![|]>::new();
    const SLASH: Token![/] = <Token![/]>::new();
    const ELLIPSIS: Token![...] = <Token![...]>::new();
    const GE: Token![>=] = <Token![>=]>::new();
    const LE: Token![<=] = <Token![<=]>::new();
    const EQ_EQ: Token![==] = <Token![==]>::new();
    const AND: Token![&&] = <Token![&&]>::new();
    const OR: Token![||] = <Token![||]>::new();
    const CONCAT: Token![++] = <Token![++]>::new();
    const NOT_EQUALS: Token![!=] = <Token![!=]>::new();
    const UPDATE: Update = Update::new();
    const BRACE: Brace = Brace;
    const PAREN: Paren = Paren;
    const BRACKET: Bracket = Bracket;
}
