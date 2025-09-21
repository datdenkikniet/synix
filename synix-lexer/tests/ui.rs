use std::str::FromStr;

use synix_lexer::{
    Ident, TokenStream, TokenTree,
    literal::{LitInt, LitStr, Literal},
};

#[test]
pub fn chained_idents_literals() {
    let input = "123asdf\"\"";

    let mut parse = TokenStream::from_str(input).unwrap().into_iter();

    assert!(matches!(
        parse.next(),
        Some(TokenTree::Literal(Literal::Int(LitInt { digits, .. }))) if digits == "123"
    ));

    assert!(matches!(
        parse.next(),
        Some(TokenTree::Ident(ident)) if ident.ident() == "asdf"
    ));

    assert!(matches!(
        parse.next(),
        Some(TokenTree::Literal(Literal::Str(LitStr { value, .. }))) if value == ""
    ));

    assert!(parse.next().is_none());
}
