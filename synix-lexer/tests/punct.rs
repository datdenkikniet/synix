use std::iter::once;
use synix_lexer::{
    Lex, LexBuffer,
    punct::{Char, Punct, Spacing},
};

macro_rules! punct {
    ($name:ident, $input:literal, $expected:ident) => {
        #[test]
        fn $name() {
            let str: String = once($input).collect();
            let mut buffer = LexBuffer::new(&str);

            let punct = Punct::lex(&mut buffer).unwrap();

            assert_eq!(punct.spacing, Spacing::Alone);
            assert_eq!(punct.ch, Char::$expected);
        }

        paste::paste! {
            #[test]
            fn [<$name "_spaced">]() {
                let str: String = once($input).chain(once(' ')).collect();
                let mut buffer = LexBuffer::new(&str);

                let punct = Punct::lex(&mut buffer).unwrap();

                assert_eq!(punct.spacing, Spacing::Alone);
                assert_eq!(punct.ch, Char::$expected);
            }

            #[test]
            fn [<$name "_joint">]() {
                let str: String = once($input).chain(once('+')).collect();
                let mut buffer = LexBuffer::new(&str);

                let punct = Punct::lex(&mut buffer).unwrap();

                assert_eq!(punct.spacing, Spacing::Joint);
                assert_eq!(punct.ch, Char::$expected);
            }
        }
    };
}

punct!(semicolon, ';', Semicolon);
punct!(colon, ':', Colon);
punct!(comma, ',', Comma);
punct!(dot, '.', Dot);
punct!(at, '@', At);
punct!(plus, '+', Plus);
punct!(minus, '-', Minus);
punct!(asterisk, '*', Asterisk);
punct!(gt, '>', Gt);
punct!(lt, '<', Lt);
punct!(equals, '=', Equals);
punct!(question, '?', Question);
punct!(ampersand, '&', Ampersand);
punct!(pipe, '|', Pipe);
punct!(slash, '/', Slash);
