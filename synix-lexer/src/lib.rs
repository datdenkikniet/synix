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
        match self {
            TokenTree::Group(group) => group.span(),
            TokenTree::Ident(ident) => ident.span(),
            TokenTree::Punct(punct) => punct.span(),
            TokenTree::Literal(literal) => literal.span(),
        }
    }
}

impl Lex for TokenTree {
    fn lex(input: &mut LexBuffer) -> Result<Self> {
        // TODO: deal with multiline comments
        if input.peek() == Some('#') {
            while input.next() != Some('\n') {}
            input.skip_ws();
        }

        let tree = if Group::starts(input) {
            let group = input.lex()?;
            TokenTree::Group(group)
        } else if Literal::starts(input.peek()) {
            let lit = input.lex()?;
            TokenTree::Literal(lit)
        } else if Ident::starts(input.peek()) {
            let ident = input.lex()?;
            TokenTree::Ident(ident)
        } else if Punct::peek(input) {
            let punct = input.lex()?;
            TokenTree::Punct(punct)
        } else {
            return Err(Error::new(input.span(), "Unexpected input."));
        };

        Ok(tree)
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

            trees.push(input.lex()?);
        }

        Ok(Self::new(trees))
    }
}
