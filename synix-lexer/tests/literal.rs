use synix_lexer::{
    Lex, LexBuffer,
    literal::{LitStr, Literal},
};

#[test]
pub fn string() {
    let mut buffer = LexBuffer::new("\"\\\"A string\\\"\"");

    let output = LitStr::lex(&mut buffer).unwrap();

    assert_eq!(output.value, "\"A string\"");
}

#[test]
pub fn multiline_string_wrong_delimiter() {
    let mut buffer = LexBuffer::new(
        r#""A
                string""#,
    );

    assert!(LitStr::lex(&mut buffer).is_err());
}

#[test]
pub fn long_integer() {
    let digits = "1049812093810948019283091823091804918203";
    let mut buffer = LexBuffer::new(digits);

    let output = match Literal::lex(&mut buffer) {
        Ok(Literal::Int(int)) => int,
        v => panic!("Expected literal int, got {v:?}"),
    };

    assert_eq!(output.digits, digits);
}
