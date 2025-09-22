use synix_lexer::{Lex, LexBuffer, group::Delimiter, group::Group};

macro_rules! test_empty {
    ($name:ident, $input:literal, $expected:ident) => {
        #[test]
        fn $name() {
            let mut buffer = LexBuffer::new($input);

            let group = Group::lex(&mut buffer).unwrap();

            assert!(group.inner.is_empty());
            assert_eq!(group.delimiter, Delimiter::$expected);
        }
    };
}

test_empty!(brace, "{}", Brace);
test_empty!(bracket, "[]", Bracket);
test_empty!(parent, "()", Paren);

#[test]
pub fn bruh() {
    let mut buffer = LexBuffer::new("{ a, b }");

    let group = Group::lex(&mut buffer);

    assert!(buffer.is_empty());
    assert!(group.is_ok());
}
