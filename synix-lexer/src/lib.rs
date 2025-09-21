mod buffer;
mod error;
pub mod group;
mod ident;
pub mod literal;
pub mod punct;
mod span;

use std::str::FromStr;

pub use buffer::LexBuffer;
pub use error::Error;
use group::Group;
pub use ident::Ident;
use literal::Literal;
use punct::Punct;
pub use span::Span;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Default)]
pub struct TokenStream {
    pub trees: Vec<TokenTree>,
}

impl TokenStream {
    pub fn empty() -> Self {
        Self::default()
    }

    pub fn is_empty(&self) -> bool {
        self.trees.is_empty()
    }
}

impl FromStr for TokenStream {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut buffer = LexBuffer::new(s);
        buffer.lex()
    }
}

#[derive(Debug)]
pub enum TokenTree {
    Group(Group),
    Ident(Ident),
    Punct(Punct),
    Literal(Literal),
}

#[derive(Debug, Clone, Copy, Default)]
pub struct LineColumn {
    pub line: usize,
    pub column: usize,
}

pub trait Lex: Sized {
    fn lex(buffer: &mut LexBuffer) -> Result<Self>;
}

impl Lex for TokenStream {
    fn lex(input: &mut LexBuffer) -> Result<Self> {
        input.skip_ws();

        if input.peek().is_none() {
            Ok(Self::default())
        } else if Group::has_delimiter(input) {
            let group: Group = input.lex()?;
            Ok(Self {
                trees: vec![TokenTree::Group(group)],
            })
        } else {
            todo!()
        }
    }
}
