use synix_lexer::{Ident, Lex, LexBuffer};

#[test]
pub fn ident_false() {
    let mut buffer = LexBuffer::new("false");

    let ident = Ident::lex(&mut buffer).unwrap();

    assert_eq!(ident.ident(), "false");
}

#[test]
pub fn start_with_underscore() {
    let mut buffer = LexBuffer::new("_underscore");

    let ident = Ident::lex(&mut buffer).unwrap();

    assert_eq!(ident.ident(), "_underscore");
}
