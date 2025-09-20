use synix_lexer::{Lex, LexBuffer, literal::LitBool};

#[test]
pub fn r#true() {
    let mut buffer = LexBuffer::new("true");

    let bool = LitBool::lex(&mut buffer).unwrap();

    assert_eq!(bool.value, true);
}

#[test]
pub fn r#true2() {
    let mut buffer = LexBuffer::new("  true  ");

    let bool = LitBool::lex(&mut buffer).unwrap();

    assert_eq!(bool.value, true);
}

#[test]
pub fn r#false() {
    let mut buffer = LexBuffer::new("false");

    let bool = LitBool::lex(&mut buffer).unwrap();

    assert_eq!(bool.value, false);
}

#[test]
pub fn false2() {
    let mut buffer = LexBuffer::new("  false  ");

    let bool = LitBool::lex(&mut buffer).unwrap();

    assert_eq!(bool.value, false);
}

#[test]
pub fn not_true() {
    let mut buffer = LexBuffer::new("trueb");

    assert!(LitBool::lex(&mut buffer).is_err());
}

#[test]
pub fn not_false() {
    let mut buffer = LexBuffer::new("falsez");

    assert!(LitBool::lex(&mut buffer).is_err());
}
