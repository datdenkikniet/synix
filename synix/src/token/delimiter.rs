use crate::{ParseBuffer, Peek};

use synix_lexer::{
    TokenTree,
    group::{Delimiter, Group},
};

macro_rules ! delimiter {
    ($($name:ident),*$(,)?) => {
        $(
            #[derive(Debug, Default, Clone, Copy, PartialEq)]
            pub struct $name;

            impl Peek for $name {
                fn peek<'a>(input: &'a ParseBuffer<'a>) -> bool {
                    matches!(
                        input.peek_tree(),
                        Some(TokenTree::Group(Group {
                            delimiter: Delimiter::$name,
                            ..
                        }))
                    )
                }
            }
        )*
    }
}

delimiter!(Brace, Paren, Bracket);
