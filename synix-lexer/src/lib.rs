mod buffer;
mod error;
pub mod group;
mod ident;
pub mod literal;
pub mod punct;
mod span;
mod token_stream;

pub use buffer::LexBuffer;
pub use error::Error;
use group::Group;
pub use ident::Ident;
use literal::Literal;
use punct::Punct;
pub use span::Span;
pub use token_stream::{IntoIter, TokenStream};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone)]
pub enum TokenTree {
    Group(Group),
    Ident(Ident),
    Punct(Punct),
    Literal(Literal),
}

impl TokenTree {
    pub fn span(&self) -> Span {
        let start = LineColumn { line: 0, column: 0 };
        Span { start, end: start }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct LineColumn {
    pub line: usize,
    pub column: usize,
}

impl LineColumn {
    pub const fn default() -> Self {
        Self::new(0, 0)
    }

    pub const fn new(line: usize, column: usize) -> Self {
        Self { line, column }
    }
}

pub trait Lex: Sized {
    fn lex(buffer: &mut LexBuffer) -> Result<Self>;
}

impl Lex for TokenStream {
    fn lex(input: &mut LexBuffer) -> Result<Self> {
        let mut trees = Vec::new();

        loop {
            input.skip_ws();

            if input.is_empty() {
                break;
            }

            if Group::starts(input) {
                let group = input.lex()?;
                trees.push(TokenTree::Group(group));
            } else if Literal::starts(input.peek()) {
                let lit = input.lex()?;
                trees.push(TokenTree::Literal(lit));
            } else if Ident::starts(input.peek()) {
                let ident = input.lex()?;
                trees.push(TokenTree::Ident(ident));
            } else if Punct::peek(input) {
                let punct = input.lex()?;
                trees.push(TokenTree::Punct(punct));
            } else {
                return Err(Error::new(input.span(), "Unexpected input."));
            }
        }

        Ok(Self::new(trees))
    }
}
