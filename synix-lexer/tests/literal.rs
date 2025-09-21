use synix_lexer::{
    Lex, LexBuffer,
    literal::{LitBool, LitStr},
};

#[test]
pub fn r#true() {
    let mut buffer = LexBuffer::new("true");

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
pub fn not_true() {
    let mut buffer = LexBuffer::new("trueb");

    assert!(LitBool::lex(&mut buffer).is_err());
}

#[test]
pub fn not_false() {
    let mut buffer = LexBuffer::new("falsez");

    assert!(LitBool::lex(&mut buffer).is_err());
}

#[test]
pub fn string() {
    let mut buffer = LexBuffer::new("\"\\\"A string\\\"\"");

    let output = LitStr::lex(&mut buffer).unwrap();

    assert_eq!(output.value, "\"A string\"");
}

#[test]
pub fn multiline_string() {
    let mut buffer = LexBuffer::new(
        r#""A
                string""#,
    );

    assert!(LitStr::lex(&mut buffer).is_err());
}
